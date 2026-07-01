use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Stream, StreamExt};
use wasmtime::component::{
    Access, Destination, HasData, Lift, Lower, StreamProducer, StreamReader, StreamResult,
};
use wasmtime::StoreContextMut;

// TODO: explore moving these to wasmtime

/// Adapts any [`Stream`] into a wasmtime [`StreamProducer`], delivering one item
/// per read. Mirrors the `PipeProducer` used in wasmtime's own tests, but using
/// only public API so it can live here.
pub struct StreamPipe<S>(pub S);

impl<D, T, S> StreamProducer<D> for StreamPipe<S>
where
    T: Lower + Send + Sync + 'static,
    S: Stream<Item = T> + Send + Unpin + 'static,
{
    type Item = T;
    type Buffer = Option<T>;

    fn poll_produce<'a>(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        _: StoreContextMut<'a, D>,
        mut destination: Destination<'a, Self::Item, Self::Buffer>,
        finish: bool,
    ) -> Poll<wasmtime::Result<StreamResult>> {
        // `S: Unpin`, so we can poll the stream without a pin-projection.
        match self.0.poll_next_unpin(cx) {
            Poll::Pending if finish => Poll::Ready(Ok(StreamResult::Cancelled)),
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(item)) => {
                destination.set_buffer(Some(item));
                Poll::Ready(Ok(StreamResult::Completed))
            }
            Poll::Ready(None) => Poll::Ready(Ok(StreamResult::Dropped)),
        }
    }
}

/// Like [`StreamPipe`], but maps each item with access to the store data `D`
/// before lowering. The closure runs inside `poll_produce`, so it gets fresh
/// store access per item (e.g. to push the item into the resource table).
pub struct StreamPipeMap<S, F>(pub S, pub F);

impl<D, In, Out, S, F> StreamProducer<D> for StreamPipeMap<S, F>
where
    Out: Lower + Send + Sync + 'static,
    S: Stream<Item = In> + Send + Unpin + 'static,
    F: FnMut(&mut D, In) -> wasmtime::Result<Out> + Send + Unpin + 'static,
{
    type Item = Out;
    type Buffer = Option<Out>;

    fn poll_produce<'a>(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut store: StoreContextMut<'a, D>,
        mut destination: Destination<'a, Self::Item, Self::Buffer>,
        finish: bool,
    ) -> Poll<wasmtime::Result<StreamResult>> {
        // `S: Unpin` and `F: Unpin`, so `Self: Unpin` and we can project freely.
        let this = self.get_mut();
        match this.0.poll_next_unpin(cx) {
            Poll::Pending if finish => Poll::Ready(Ok(StreamResult::Cancelled)),
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(item)) => {
                let out = (this.1)(store.data_mut(), item)?;
                destination.set_buffer(Some(out));
                Poll::Ready(Ok(StreamResult::Completed))
            }
            Poll::Ready(None) => Poll::Ready(Ok(StreamResult::Dropped)),
        }
    }
}

/// Turns a broadcast receiver into a component-model stream. Needs the store
/// (via `access`) because `StreamReader::new` registers the producer with it.
pub fn channel_to_stream<T, D, A>(
    access: Access<'_, D, A>,
    receiver: async_broadcast::Receiver<T>,
) -> StreamReader<T>
where
    T: Lower + Lift + Clone + Send + Sync + 'static,
    D: 'static,
    A: HasData + ?Sized,
{
    StreamReader::new(access, StreamPipe(receiver)).unwrap()
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
