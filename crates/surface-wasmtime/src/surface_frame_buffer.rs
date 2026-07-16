use crate::surface::{MainThreadSpawner, Surface};
use frame_buffer_wasmtime::{GfxBuffer, HasBuffer};
use std::{
    marker::PhantomData,
    num::NonZeroU32,
    sync::{Arc, Mutex},
};
use wasi_gfx::surface::surface_frame_buffer;
use wasmtime::component::{HasData, Resource};

wasmtime::component::bindgen!({
    world: "wasi-gfx:surface/frame-buffer-imports",
    require_store_data_send: true,
    imports: {
        default: trappable,
    },
    with: {
        "wasi-gfx:surface/surface": crate::surface::wasi_gfx::surface::surface,
        "wasi-gfx:frame-buffer/frame-buffer": frame_buffer_wasmtime::wasi_gfx::frame_buffer::frame_buffer,
        "wasi-gfx:surface/surface-frame-buffer.context": GfxContext,
    },
});

// types
pub struct GfxContext {
    fb_surface: FBSurfaceArc,
}

struct FBSurfaceArc {
    surface: Arc<Mutex<softbuffer::Surface<Surface, Surface>>>,
    /// softbuffer only presents pixels written into the current `buffer_mut()`, so
    /// `set_buffer` stashes the frame here and `present` copies + presents it.
    staged: Arc<Mutex<Vec<u32>>>,
}

impl FBSurfaceArc {
    pub fn new(surface: softbuffer::Surface<Surface, Surface>) -> Self {
        Self {
            surface: Arc::new(Mutex::new(surface)),
            staged: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn arc_clone(&self) -> Self {
        Self {
            surface: Arc::clone(&self.surface),
            staged: Arc::clone(&self.staged),
        }
    }
}

impl HasBuffer for FBSurfaceArc {
    fn get_buffer(&self) -> wasmtime::Result<Vec<u8>> {
        let staged = self.staged.lock().unwrap();
        Ok(bytemuck::cast_slice(staged.as_slice()).to_vec())
    }

    fn set_buffer(&mut self, value: &[u8]) -> wasmtime::Result<()> {
        let value: &[u32] = bytemuck::try_cast_slice(value)?;
        let mut staged = self.staged.lock().unwrap();
        staged.clear();
        staged.extend_from_slice(value);
        Ok(())
    }
}

// linker connection
pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: SurfaceFrameBufferCtxView,
{
    wasi_gfx::surface::surface_frame_buffer::add_to_linker::<
        _,
        HasSurfaceFrameBufferCtx<T::Spawner>,
    >(l, T::surface_frame_buffer_ctx)?;
    Ok(())
}

pub trait SurfaceFrameBufferCtxView: Send {
    /// Spawner used to run main-thread-only framebuffer operations.
    type Spawner: MainThreadSpawner;
    /// returns a struct of references.
    /// Returning all references in a struct allows us to use multiple mutable references at the same time.
    fn surface_frame_buffer_ctx(&mut self) -> SurfaceFrameBufferCtx<'_, Self::Spawner>;
}

pub struct SurfaceFrameBufferCtx<'a, S: MainThreadSpawner> {
    pub table: &'a mut wasmtime_wasi::ResourceTable,
    pub instance: &'a Arc<wasi_webgpu_wasmtime::reexports::wgpu_core::global::Global>,
    pub main_thread_spawner: &'a S,
}

struct HasSurfaceFrameBufferCtx<S>(PhantomData<S>);

impl<S: MainThreadSpawner> HasData for HasSurfaceFrameBufferCtx<S> {
    type Data<'a> = SurfaceFrameBufferCtx<'a, S>;
}

// wasmtime trait impls
impl<'a, S: MainThreadSpawner> surface_frame_buffer::Host for SurfaceFrameBufferCtx<'a, S> {}

impl<'a, S: MainThreadSpawner> surface_frame_buffer::HostContext for SurfaceFrameBufferCtx<'a, S> {
    fn new(
        &mut self,
        gfx_surface: Resource<surface_frame_buffer::Surface>,
    ) -> wasmtime::Result<Resource<surface_frame_buffer::Context>> {
        let gfx_surface = self.table.get(&gfx_surface)?.arc_clone();

        let fb_surface = futures::executor::block_on(self.main_thread_spawner.spawn(move || {
            let fb_context = softbuffer::Context::new(gfx_surface.arc_clone()).unwrap();
            let mut fb_surface =
                softbuffer::Surface::new(&fb_context, gfx_surface.arc_clone()).unwrap();

            fb_surface
                .resize(
                    gfx_surface
                        .width()
                        .try_into()
                        .unwrap_or(NonZeroU32::new(1).unwrap()),
                    gfx_surface
                        .height()
                        .try_into()
                        .unwrap_or(NonZeroU32::new(1).unwrap()),
                )
                .unwrap();

            FBSurfaceArc::new(fb_surface)
        }));

        let gfx_context = GfxContext { fb_surface };
        Ok(self.table.push(gfx_context)?)
    }

    fn get_current_buffer(
        &mut self,
        gfx_context: Resource<surface_frame_buffer::Context>,
    ) -> wasmtime::Result<Resource<surface_frame_buffer::Buffer>> {
        let gfx_context = self.table.get_mut(&gfx_context)?;
        let gfx_buffer = GfxBuffer {
            buffer: Box::new(gfx_context.fb_surface.arc_clone()),
        };
        Ok(self.table.push(gfx_buffer)?)
    }

    fn present(
        &mut self,
        gfx_context: Resource<surface_frame_buffer::Context>,
    ) -> wasmtime::Result<()> {
        let gfx_context = self.table.get_mut(&gfx_context)?;
        let staged = gfx_context.fb_surface.staged.lock().unwrap();
        let mut surface = gfx_context.fb_surface.surface.lock().unwrap();
        let mut buffer = surface.buffer_mut().unwrap();
        buffer.copy_from_slice(&staged);
        buffer.present().unwrap();
        Ok(())
    }

    fn drop(&mut self, context: Resource<surface_frame_buffer::Context>) -> wasmtime::Result<()> {
        self.table.delete(context)?;
        Ok(())
    }
}
