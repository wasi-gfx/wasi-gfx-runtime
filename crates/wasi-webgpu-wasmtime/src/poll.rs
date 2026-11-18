//! Polling the wgpu instance.
//!
//! wgpu only fires callbacks when the instance is polled, but this crate never polls by itself. The embedder must poll, or guest calls that wait on those callbacks never complete.

use std::{
    io,
    sync::{
        Arc,
        mpsc::{self, Receiver, RecvTimeoutError},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use wgpu_core::global::Global;

/// unit-newtype passed to [`PollNotifier::notify`] when runtime asks embedder to poll
#[derive(Debug)]
pub struct PollDevicesRequest(pub(crate) ());

/// Notified each time the instance needs polling. Called from guest host calls, so it must not block.
pub trait PollNotifier: Send + Sync {
    fn notify(&self, request: PollDevicesRequest);
}

impl<F: Fn(PollDevicesRequest) + Send + Sync> PollNotifier for F {
    fn notify(&self, request: PollDevicesRequest) {
        self(request)
    }
}

/// A [`PollNotifier`] that sends requests on a [`std::sync::mpsc`] channel.
#[derive(Clone, Debug)]
pub struct MpscPollNotifier(pub std::sync::mpsc::Sender<PollDevicesRequest>);

impl PollNotifier for MpscPollNotifier {
    fn notify(&self, request: PollDevicesRequest) {
        // A dropped receiver means the embedder polls without being told.
        let _ = self.0.send(request);
    }
}

/// Settings for device polling
#[derive(Clone, Debug)]
pub struct PollDevicesConfig {
    /// How often to poll while GPU work is in flight.
    pub interval: Duration,
}

impl Default for PollDevicesConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_millis(1),
        }
    }
}

/// Polls the instance on the current thread until every sender is dropped.
///
/// NOTE: This function blocks the thread.
pub fn poll_devices_loop(
    instance: &Global,
    requests: Receiver<PollDevicesRequest>,
    config: PollDevicesConfig,
) {
    let mut idle = true;
    let mut failing = false;
    loop {
        // Idle: block until a request. Busy: wait `interval`
        let received = if idle {
            requests.recv().map_err(|_| RecvTimeoutError::Disconnected)
        } else {
            requests.recv_timeout(config.interval)
        };
        match received {
            Ok(_) | Err(RecvTimeoutError::Timeout) => {}
            // Every notifier is gone, so no guest can be waiting on a callback.
            Err(RecvTimeoutError::Disconnected) => return,
        }
        // One poll covers every request that arrived meanwhile.
        requests.try_iter().for_each(drop);
        idle = match instance.poll_all_devices(false) {
            Ok(all_queues_empty) => {
                failing = false;
                all_queues_empty
            }
            // Work may still be in flight, so keep polling. Log once per run of errors.
            Err(err) => {
                if !failing {
                    log::warn!("polling devices failed: {err}");
                }
                failing = true;
                false
            }
        };
    }
}

/// Creates a new thread with [`std::thread`] and runs [`poll_devices_loop`] on it.
pub fn spawn_poll_devices_thread(
    instance: Arc<Global>,
    config: PollDevicesConfig,
) -> io::Result<(MpscPollNotifier, JoinHandle<()>)> {
    let (sender, requests) = mpsc::channel();
    let thread = thread::Builder::new()
        .name("wasi-webgpu-poll".into())
        .spawn(move || poll_devices_loop(&instance, requests, config))?;
    Ok((MpscPollNotifier(sender), thread))
}

/// Like [`spawn_poll_devices_thread`], but runs the loop on tokio's blocking thread pool. Panics if called outside a tokio runtime.
#[cfg(feature = "tokio")]
pub fn spawn_poll_devices_tokio_thread(
    instance: Arc<Global>,
    config: PollDevicesConfig,
) -> (MpscPollNotifier, tokio::task::JoinHandle<()>) {
    let (sender, requests) = mpsc::channel();
    let task = tokio::task::spawn_blocking(move || poll_devices_loop(&instance, requests, config));
    (MpscPollNotifier(sender), task)
}
