use std::fmt::Debug;

#[derive(Debug)]
pub struct Listener<T, F>
where
    T: Debug + Clone + Send + Sync + 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    receiver: async_broadcast::Receiver<T>,
    on_data: F,
}

impl<T, F> Listener<T, F>
where
    T: Debug + Clone + Send + Sync + 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    pub fn new(receiver: async_broadcast::Receiver<T>, on_data: F) -> Self {
        Self { receiver, on_data }
    }
}

#[async_trait::async_trait] // TODO: remove async_trait crate once wasmtime drops it
impl<T, F> wasmtime_wasi_io::poll::Pollable for Listener<T, F>
where
    T: Debug + Clone + Send + Sync + 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        (self.on_data)(event);
    }
}

// Helper functions to ignore messages when async_broadcast channels are inactive or full.
use async_broadcast::TrySendError;

pub fn unwrap_unless_inactive<T>(res: Result<Option<T>, TrySendError<T>>) {
    if let Err(TrySendError::Inactive(_)) = &res {
        return;
    }
    res.unwrap();
}

pub fn unwrap_unless_inactive_or_full<T>(res: Result<Option<T>, TrySendError<T>>) {
    if let Err(e) = &res {
        if matches!(e, TrySendError::Inactive(_) | TrySendError::Full(_)) {
            return;
        }
    }
    res.unwrap();
}
