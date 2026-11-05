//! A future that resolves when a callback is called.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, PoisonError};
use std::task::{Context, Poll, Waker};

pub(crate) type Resolve<T> = Box<dyn FnOnce(T) + Send>;
type Loader<T> = Box<dyn FnOnce(Resolve<T>) + Send>;

struct Slot<T> {
    value: Option<T>,
    waker: Option<Waker>,
}

/// Runs `loader` on the first poll, handing it a `resolve` callback.
///
/// Every poll stores its waker, so `resolve` wakes the task that polled last.
/// If `resolve` runs inside `loader`, the first poll is already ready.
pub(crate) struct CallbackFuture<T> {
    loader: Option<Loader<T>>,
    slot: Arc<Mutex<Slot<T>>>,
}

impl<T: Send + 'static> CallbackFuture<T> {
    pub(crate) fn new(loader: Loader<T>) -> Self {
        Self {
            loader: Some(loader),
            slot: Arc::new(Mutex::new(Slot {
                value: None,
                waker: None,
            })),
        }
    }

    fn take_value(&self) -> Option<T> {
        self.slot
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .value
            .take()
    }
}

impl<T: Send + 'static> Future for CallbackFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let this = self.get_mut();
        {
            let mut slot = this.slot.lock().unwrap_or_else(PoisonError::into_inner);
            if let Some(value) = slot.value.take() {
                return Poll::Ready(value);
            }
            slot.waker = Some(cx.waker().clone());
        }
        if let Some(loader) = this.loader.take() {
            let slot = Arc::clone(&this.slot);
            loader(Box::new(move |value| {
                let waker = {
                    let mut slot = slot.lock().unwrap_or_else(PoisonError::into_inner);
                    slot.value = Some(value);
                    slot.waker.take()
                };
                if let Some(waker) = waker {
                    waker.wake();
                }
            }));
            if let Some(value) = this.take_value() {
                return Poll::Ready(value);
            }
        }
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::Wake;

    #[derive(Default)]
    struct CountingWaker(AtomicUsize);

    impl Wake for CountingWaker {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn counting_waker() -> (Arc<CountingWaker>, Waker) {
        let counter = Arc::new(CountingWaker::default());
        (Arc::clone(&counter), Waker::from(counter))
    }

    #[test]
    fn resolves_on_first_poll_when_called_inline() {
        let mut future = CallbackFuture::new(Box::new(|resolve: Resolve<u32>| resolve(7)));
        let (_, waker) = counting_waker();
        let mut cx = Context::from_waker(&waker);
        assert_eq!(Pin::new(&mut future).poll(&mut cx), Poll::Ready(7));
    }

    #[test]
    fn wakes_the_latest_waker() {
        let stash: Arc<Mutex<Option<Resolve<u32>>>> = Arc::default();
        let loader_stash = Arc::clone(&stash);
        let mut future = CallbackFuture::new(Box::new(move |resolve: Resolve<u32>| {
            *loader_stash.lock().unwrap() = Some(resolve);
        }));

        let (first, first_waker) = counting_waker();
        let (second, second_waker) = counting_waker();
        assert!(Pin::new(&mut future)
            .poll(&mut Context::from_waker(&first_waker))
            .is_pending());
        assert!(Pin::new(&mut future)
            .poll(&mut Context::from_waker(&second_waker))
            .is_pending());

        let resolve = stash.lock().unwrap().take().unwrap();
        resolve(7);
        assert_eq!(first.0.load(Ordering::SeqCst), 0);
        assert_eq!(second.0.load(Ordering::SeqCst), 1);
        assert_eq!(
            Pin::new(&mut future).poll(&mut Context::from_waker(&second_waker)),
            Poll::Ready(7)
        );
    }
}
