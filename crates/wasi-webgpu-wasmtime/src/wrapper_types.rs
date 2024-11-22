// Wrappers around `wgpu_*` types
// Every type here should have an explanation as to why we can't use the type directly.

use std::{collections::HashMap, ptr::NonNull, slice};

use crate::wasi::webgpu::webgpu;

// can't pass generics to `wasmtime::component::bindgen`
pub type RenderPass = wgpu_core::command::RenderPass<crate::Backend>;
pub type ComputePass = wgpu_core::command::ComputePass<crate::Backend>;
pub type RecordGpuPipelineConstantValue = HashMap<String, webgpu::GpuPipelineConstantValue>;

// needed just to group the pointer and length together
pub struct BufferPtr {
    // See https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general/topic/Should.20wasi.20resources.20be.20stored.20behind.20a.20mutex.3F
    pub(crate) ptr: NonNull<u8>,
    pub(crate) len: u64,
}
impl BufferPtr {
    pub fn slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len as usize) }
    }
    pub fn slice_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len as usize) }
    }
}
unsafe impl Send for BufferPtr {}
unsafe impl Sync for BufferPtr {}

// size needed in `GpuBuffer.size`, `RenderPass.set_index_buffer`, `RenderPass.set_vertex_buffer`
pub struct Buffer {
    pub(crate) buffer_id: wgpu_core::id::BufferId,
    pub(crate) size: u64,
}

// queue needed for Device.queue
// adapter needed for surface_get_capabilities in connect_graphics_context
#[derive(Clone, Copy)]
pub struct Device {
    pub(crate) device: wgpu_core::id::DeviceId,
    pub(crate) queue: wgpu_core::id::QueueId,
    pub(crate) adapter: wgpu_core::id::AdapterId,
}
