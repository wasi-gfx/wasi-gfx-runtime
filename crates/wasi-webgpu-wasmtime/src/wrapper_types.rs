// Wrappers around `wgpu_*` types
// Every type here should have an explanation as to why we can't use the type directly.

use std::{collections::HashMap, sync::Arc};

use crate::wasi::webgpu::webgpu;

// can't pass generics to `wasmtime::component::bindgen`
pub type RecordGpuPipelineConstantValue = HashMap<String, webgpu::GpuPipelineConstantValue>;

// RenderPassEncoder, ComputePassEncoder, and RenderBundleEncoder need to be dropped when calling `.end`/`.finish` on them, but we can't guarantee that they'll be dropped in time by GC languages. Takeable lets you take the value and leaves None in place, so that RenderPass/ComputePass get dropped from Rust's point of view, but the wasm module can keep it's reference.
// this is caused by the same underlying issue as this one https://github.com/gfx-rs/wgpu-native/issues/412
pub type RenderPassEncoder = Takeable<wgpu_core::command::RenderPass>;
pub type ComputePassEncoder = Takeable<wgpu_core::command::ComputePass>;
pub type RenderBundleEncoder = Takeable<wgpu_core::command::RenderBundleEncoder>;

#[derive(Clone, Debug)]
pub struct Takeable<T: std::fmt::Debug>(Arc<std::sync::Mutex<Option<T>>>);
impl<T> Takeable<T>
where
    T: std::fmt::Debug,
{
    pub fn new(id: T) -> Self {
        Takeable(Arc::new(std::sync::Mutex::new(Some(id))))
    }
    pub fn lock<'a>(&'a self) -> std::sync::MutexGuard<'a, Option<T>> {
        self.0.lock().unwrap()
    }
    pub fn take(&self) -> Option<T> {
        self.0.lock().unwrap().take()
    }
}

// // needed just to group the pointer and length together
// pub struct BufferPtr {
//     // See https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general/topic/Should.20wasi.20resources.20be.20stored.20behind.20a.20mutex.3F
//     pub(crate) ptr: NonNull<u8>,
//     pub(crate) len: u64,
// }
// impl BufferPtr {
//     pub fn slice(&self) -> &[u8] {
//         unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len as usize) }
//     }
//     pub fn slice_mut(&mut self) -> &mut [u8] {
//         unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len as usize) }
//     }
// }
// unsafe impl Send for BufferPtr {}
// unsafe impl Sync for BufferPtr {}

// size needed in `GpuBuffer.size`, `RenderPass.set_index_buffer`, `RenderPass.set_vertex_buffer`.
// usage needed in `GpuBuffer.usage`
pub struct Buffer {
    pub(crate) buffer_id: wgpu_core::id::BufferId,
    pub(crate) size: u64,
    pub(crate) usage: wgpu_types::BufferUsages,
    pub(crate) map_state: webgpu::GpuBufferMapState,
}

// queue needed for Device.queue
// adapter needed for surface_get_capabilities in connect_graphics_context
#[derive(Clone, Copy)]
pub struct Device {
    pub(crate) device: wgpu_core::id::DeviceId,
    pub(crate) queue: wgpu_core::id::QueueId,
    pub(crate) adapter: wgpu_core::id::AdapterId,
}
