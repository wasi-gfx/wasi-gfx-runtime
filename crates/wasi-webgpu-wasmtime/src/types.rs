// Wrappers around `wgpu_*` types
// Every type here should have an explanation as to why we can't use the type directly.

use std::{
    collections::HashMap,
    fmt::{self, Debug},
    sync::{Arc, Mutex},
};

use crate::{owned::Owned, wasi::webgpu::webgpu};

pub struct WgslLanguageFeatures;
impl WgslLanguageFeatures {
    pub fn new() -> Self {
        Self
    }
    pub fn has(&self, feature: &str) -> bool {
        use wgpu_core::naga::front::wgsl::LanguageExtension;
        matches!(
            LanguageExtension::from_ident(feature),
            Some(LanguageExtension::Implemented(_))
        )
    }
}

// wgpu-core doesn't keep shader compilation errors, so `getCompilationInfo` needs them stored with the shader module
pub struct ShaderModule {
    pub(crate) shader_module_id: Owned<wgpu_core::id::ShaderModuleId>,
    pub(crate) compilation_messages: Arc<[CompilationMessage]>,
}

pub struct CompilationInfo {
    pub(crate) messages: Arc<[CompilationMessage]>,
}

#[derive(Clone, Debug)]
pub struct CompilationMessage {
    pub(crate) message: String,
    pub(crate) line_num: u64,
    pub(crate) line_pos: u64,
    pub(crate) offset: u64,
    pub(crate) length: u64,
}

impl CompilationMessage {
    pub(crate) fn from_error(error: &wgpu_core::pipeline::CreateShaderModuleError) -> Self {
        use wgpu_core::pipeline::CreateShaderModuleError;
        let location = match error {
            CreateShaderModuleError::Parsing(e) => e.inner.location(&e.source),
            CreateShaderModuleError::Validation(e) => e.inner.location(&e.source),
            _ => None,
        };
        Self {
            message: error.to_string(),
            line_num: location.map(|l| l.line_number.into()).unwrap_or(0),
            line_pos: location.map(|l| l.line_position.into()).unwrap_or(0),
            offset: location.map(|l| l.offset.into()).unwrap_or(0),
            length: location.map(|l| l.length.into()).unwrap_or(0),
        }
    }
}

// can't pass generics to `wasmtime::component::bindgen`
// TODO: these should be unit-structs instead of `types` so that the internals are private to the crate
pub type RecordGpuPipelineConstantValue = HashMap<String, webgpu::GpuPipelineConstantValue>;
pub type RecordOptionGpuSize64 = HashMap<String, Option<webgpu::GpuSize64>>;

// RenderPassEncoder, ComputePassEncoder, and RenderBundleEncoder need to be dropped when calling `.end`/`.finish` on them, but we can't guarantee that they'll be dropped in time by GC languages. Takeable lets you take the value and leaves None in place, so that RenderPass/ComputePass get dropped from Rust's point of view, but the wasm module can keep it's reference.
// this is caused by the same underlying issue as this one https://github.com/gfx-rs/wgpu-native/issues/412
// TODO: these should be unit-structs instead of `types` so that the internals are private to the crate
pub type RenderPassEncoder = Takeable<wgpu_core::command::RenderPass>;
pub type ComputePassEncoder = Takeable<wgpu_core::command::ComputePass>;
pub type RenderBundleEncoder = Takeable<RenderBundleEncoderInner>;

#[derive(Clone, Debug)]
pub struct Takeable<T: std::fmt::Debug>(Arc<std::sync::Mutex<Option<T>>>);
impl<T> Takeable<T>
where
    T: std::fmt::Debug,
{
    pub(crate) fn new(id: T) -> Self {
        Takeable(Arc::new(std::sync::Mutex::new(Some(id))))
    }
    pub(crate) fn lock<'a>(&'a self) -> std::sync::MutexGuard<'a, Option<T>> {
        self.0.lock().unwrap()
    }
    // pub(crate) fn take(&self) -> Option<T> {
    //     self.0.lock().unwrap().take()
    // }
}

// size needed in `GpuBuffer.size`, `RenderPass.set_index_buffer`, `RenderPass.set_vertex_buffer`.
// usage needed in `GpuBuffer.usage`
pub struct Buffer {
    pub(crate) buffer_id: Owned<wgpu_core::id::BufferId>,
    pub(crate) size: u64,
    pub(crate) usage: wgpu_types::BufferUsages,
    pub(crate) map_state: webgpu::GpuBufferMapState,
    pub(crate) error_handler: Arc<ErrorHandler>,
}

// references to adapter are also saved in device.
// TODO: this should be unit-structs instead of `type` so that the internals are private to the crate
pub type Adapter = Arc<Owned<wgpu_core::id::AdapterId>>;

// Resources that are only a wgpu-core id. Dropping the table entry releases it.
pub type BindGroup = Owned<wgpu_core::id::BindGroupId>;
pub type BindGroupLayout = Owned<wgpu_core::id::BindGroupLayoutId>;
pub type CommandBuffer = Owned<wgpu_core::id::CommandBufferId>;
pub type PipelineLayout = Owned<wgpu_core::id::PipelineLayoutId>;
pub type QuerySet = Owned<wgpu_core::id::QuerySetId>;
pub type RenderBundle = Owned<wgpu_core::id::RenderBundleId>;
pub type Sampler = Owned<wgpu_core::id::SamplerId>;
pub type TextureView = Owned<wgpu_core::id::TextureViewId>;

// queue needed for Device.queue
// adapter needed for surface_get_capabilities in connect_graphics_context
// keeping queue and adapter as Arc for reference counting while dropping.
#[derive(Clone)]
pub struct Device {
    pub(crate) device: Arc<Owned<wgpu_core::id::DeviceId>>,
    pub(crate) queue: Arc<Owned<wgpu_core::id::QueueId>>,
    pub(crate) adapter: Adapter,
    pub(crate) error_handler: Arc<ErrorHandler>,
    pub(crate) lost: DeviceLost,
}

impl Device {
    pub fn device_id(&self) -> &wgpu_core::id::DeviceId {
        &self.device
    }

    /// Create a `Texture` from a `TextureId` that is connected to this device.
    /// Useful in cases where an external crate get a texture through get_current_texture
    /// and needs to connect it to a device.
    ///
    /// # Safety
    ///
    /// - `texture_id` must be connected to this device.
    /// - Ownership of `texture_id` moves to the returned `Texture`, which releases it when dropped. The caller must not release it, or pass it to `connect_texture` again. wgpu-core panics when an id is released twice.
    #[doc(hidden)]
    pub unsafe fn connect_texture(&self, texture_id: wgpu_core::id::TextureId) -> Texture {
        Texture {
            texture_id: Owned::new(self.device.instance(), texture_id),
            error_handler: Arc::clone(&self.error_handler),
        }
    }
}

pub struct Queue {
    // references to queue are also saved in device.
    pub(crate) queue: Arc<Owned<wgpu_core::id::QueueId>>,
    pub(crate) error_handler: Arc<ErrorHandler>,
}

pub struct CommandEncoder {
    pub(crate) command_encoder_id: Owned<wgpu_core::id::CommandEncoderId>,
    pub(crate) error_handler: Arc<ErrorHandler>,
}

pub struct Texture {
    pub(crate) texture_id: Owned<wgpu_core::id::TextureId>,
    pub(crate) error_handler: Arc<ErrorHandler>,
}
pub struct RenderPipeline {
    pub(crate) render_pipeline_id: Owned<wgpu_core::id::RenderPipelineId>,
    pub(crate) error_handler: Arc<ErrorHandler>,
}

#[derive(Debug)]
pub struct RenderBundleEncoderInner {
    pub(crate) render_bundle_encoder: wgpu_core::command::RenderBundleEncoder,
    pub(crate) error_handler: Arc<ErrorHandler>,
}
pub struct ComputePipeline {
    pub(crate) compute_pipeline_id: Owned<wgpu_core::id::ComputePipelineId>,
    pub(crate) error_handler: Arc<ErrorHandler>,
}

/// Why a device was lost.
#[derive(Clone, Debug)]
pub struct DeviceLostInfo {
    pub(crate) reason: webgpu::GpuDeviceLostReason,
    pub(crate) message: String,
}

/// Whether wgpu has reported the device lost, and why.
#[derive(Clone)]
pub(crate) struct DeviceLost(
    futures::future::Shared<futures::channel::oneshot::Receiver<DeviceLostInfo>>,
);

impl DeviceLost {
    /// Returns the closure to give wgpu for this device, and the loss it reports.
    pub(crate) fn new() -> (wgpu_core::device::DeviceLostClosure, Self) {
        let (sender, receiver) = futures::channel::oneshot::channel();
        let closure: wgpu_core::device::DeviceLostClosure = Box::new(move |reason, message| {
            let _ = sender.send(DeviceLostInfo {
                reason: reason.into(),
                message,
            });
        });
        (closure, Self(futures::FutureExt::shared(receiver)))
    }

    /// Resolves once the device is lost. Never resolves if wgpu drops the device without losing it.
    pub(crate) async fn wait(&self) -> DeviceLostInfo {
        match self.0.clone().await {
            Ok(info) => info,
            Err(_) => std::future::pending().await,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GpuError {
    pub(crate) message: String,
    pub(crate) kind: webgpu::GpuErrorKind,
}

// Device level error handler
#[derive(Debug)]
pub(crate) struct ErrorHandler(Mutex<ErrorHandlerInner>);

#[derive(Debug)]
pub(crate) struct ErrorHandlerInner {
    scopes: Vec<ErrorScope>,
    uncaptured_error_sender: async_broadcast::Sender<webgpu::GpuError>,
    // Keeping inactive receiver to keep channel open.
    // See https://docs.rs/async-broadcast/0.7.1/async_broadcast/struct.InactiveReceiver.html
    _uncaptured_error_receiver: async_broadcast::InactiveReceiver<webgpu::GpuError>,
}

impl Default for ErrorHandler {
    fn default() -> Self {
        let (sender, receiver) = async_broadcast::broadcast(5);
        let receiver = receiver.deactivate();
        let inner = ErrorHandlerInner {
            scopes: Default::default(),
            uncaptured_error_sender: sender,
            _uncaptured_error_receiver: receiver,
        };
        Self(Mutex::new(inner))
    }
}

#[derive(Debug)]
struct ErrorScope {
    error: Option<webgpu::GpuError>,
    filter: webgpu::GpuErrorFilter,
}

impl ErrorHandler {
    pub fn push_scope(&self, filter: webgpu::GpuErrorFilter) {
        self.0.lock().unwrap().scopes.push(ErrorScope {
            filter,
            error: None,
        });
    }

    pub fn pop_scope(&self) -> Result<Option<webgpu::GpuError>, webgpu::PopErrorScopeError> {
        let scopes = self.0.lock().unwrap().scopes.pop();
        match scopes {
            Some(scope) => Ok(scope.error),
            None => {
                // From the spec:
                // > If any of the following requirements are unmet:
                // >  - this.[[errorScopeStack]].size must be > 0.
                // > Then issue the following steps on contentTimeline and return:
                // >  1. Reject promise with an OperationError.
                // https://www.w3.org/TR/webgpu/#dom-gpudevice-poperrorscope
                Err(webgpu::PopErrorScopeError {
                    kind: webgpu::PopErrorScopeErrorKind::OperationError,
                    message: "pop-error-scope on empty stack".to_string(),
                })
            }
        }
    }

    pub fn handle_possible_error<E: wgpu_types::error::WebGpuError + fmt::Display>(
        &self,
        error: Option<E>,
    ) {
        if let Some(error) = error {
            let error_kind = match error.webgpu_error_type() {
                wgpu_types::error::ErrorType::Internal => webgpu::GpuErrorKind::InternalError,
                wgpu_types::error::ErrorType::OutOfMemory => webgpu::GpuErrorKind::OutOfMemoryError,
                wgpu_types::error::ErrorType::Validation => webgpu::GpuErrorKind::ValidationError,
                wgpu_types::error::ErrorType::DeviceLost => {
                    // `DeviceLost` errors are not reported.
                    // From the spec:
                    // > Note: No errors are generated from a device which is lost. If this algorithm is called while device is lost, it will not be observable to the application.
                    // https://www.w3.org/TR/webgpu/#abstract-opdef-dispatch-error
                    return;
                }
            };

            let error = GpuError {
                message: error.to_string(),
                kind: error_kind,
            };

            let error_filter = error_kind.into();
            let mut inner = self.0.lock().unwrap();
            match &mut inner
                .scopes
                .iter_mut()
                .rev()
                .find(|scope| scope.filter == error_filter)
            {
                Some(scope) => {
                    // Only return one error per scope.
                    // From the spec:
                    // > 4. Let error be any one of the items in scope.[[errors]], or null if there are none.
                    // >   For any two errors E1 and E2 in the list, if E2 was caused by E1, E2 should not be the one selected.
                    // https://www.w3.org/TR/webgpu/#dom-gpudevice-poperrorscope
                    // Here we're assuming that the first error is the one that caused the others, so only set the first error.
                    if scope.error.is_none() {
                        scope.error = Some(error);
                    }
                }
                None => {
                    shared::unwrap_unless_inactive_or_full(
                        inner.uncaptured_error_sender.try_broadcast(error),
                    );
                }
            }
        }
    }

    pub(crate) fn new_error_receiver(&self) -> async_broadcast::Receiver<webgpu::GpuError> {
        self.0
            .lock()
            .unwrap()
            .uncaptured_error_sender
            .new_receiver()
    }
}

// For now, GpuErrorFilter and GpuErrorKind are effectively the same, just with different names.
impl From<webgpu::GpuErrorFilter> for webgpu::GpuErrorKind {
    fn from(filter: webgpu::GpuErrorFilter) -> Self {
        match filter {
            webgpu::GpuErrorFilter::Validation => webgpu::GpuErrorKind::ValidationError,
            webgpu::GpuErrorFilter::OutOfMemory => webgpu::GpuErrorKind::OutOfMemoryError,
            webgpu::GpuErrorFilter::Internal => webgpu::GpuErrorKind::InternalError,
        }
    }
}
impl From<webgpu::GpuErrorKind> for webgpu::GpuErrorFilter {
    fn from(kind: webgpu::GpuErrorKind) -> Self {
        match kind {
            webgpu::GpuErrorKind::ValidationError => webgpu::GpuErrorFilter::Validation,
            webgpu::GpuErrorKind::OutOfMemoryError => webgpu::GpuErrorFilter::OutOfMemory,
            webgpu::GpuErrorKind::InternalError => webgpu::GpuErrorFilter::Internal,
        }
    }
}
