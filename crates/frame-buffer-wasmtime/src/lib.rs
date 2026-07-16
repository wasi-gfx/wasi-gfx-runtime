//! This crate is mostly just a shell. It exports `HasBuffer` and binds it to wasmtime.
//!
//! The actual implementation lives in the `surface-wasmtime` crate (see `surface_frame_buffer.rs`).

use crate::wasi_gfx::frame_buffer::frame_buffer;
use wasmtime::component::{HasData, Resource};

wasmtime::component::bindgen!({
    world: "wasi-gfx:frame-buffer/imports",
    imports: {
        default: trappable,
    },
    with: {
        "wasi-gfx:frame-buffer/frame-buffer.buffer": GfxBuffer,
    },
});

// types

// trait that providers of the frame-buffer should implement
pub trait HasBuffer: Send {
    fn get_buffer(&self) -> wasmtime::Result<Vec<u8>>;
    fn set_buffer(&mut self, value: &[u8]) -> wasmtime::Result<()>;
}
pub struct GfxBuffer {
    pub buffer: Box<dyn HasBuffer>,
}

// linker connection
pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: FrameBufferCtxView,
{
    wasi_gfx::frame_buffer::frame_buffer::add_to_linker::<_, HasFrameBufferCtx>(
        l,
        T::frame_buffer_ctx,
    )?;
    Ok(())
}

pub trait FrameBufferCtxView {
    fn frame_buffer_ctx<'a>(&'a mut self) -> FrameBufferCtx<'a>;
}

pub struct FrameBufferCtx<'a> {
    pub table: &'a mut wasmtime_wasi::ResourceTable,
}

struct HasFrameBufferCtx;

impl HasData for HasFrameBufferCtx {
    type Data<'a> = FrameBufferCtx<'a>;
}

// wasmtime trait impls
impl<'a> frame_buffer::Host for FrameBufferCtx<'a> {}

impl<'a> frame_buffer::HostBuffer for FrameBufferCtx<'a> {
    fn get_with_copy(&mut self, buffer: Resource<GfxBuffer>) -> wasmtime::Result<Vec<u8>> {
        let buffer = self.table.get(&buffer)?;
        let buffer = buffer.buffer.get_buffer()?;
        Ok(buffer)
    }

    fn set_with_copy(&mut self, buffer: Resource<GfxBuffer>, val: Vec<u8>) -> wasmtime::Result<()> {
        let buffer = self.table.get_mut(&buffer)?;
        buffer.buffer.set_buffer(&val).unwrap();
        Ok(())
    }

    fn drop(&mut self, frame_buffer: Resource<GfxBuffer>) -> wasmtime::Result<()> {
        let _frame_buffer = self.table.delete(frame_buffer)?;
        Ok(())
    }
}
