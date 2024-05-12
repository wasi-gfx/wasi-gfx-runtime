// TODO: all bitflags should come from wit.

bitflags::bitflags! {
    /// Different ways that you can use a buffer.
    ///
    /// The usages determine what kind of memory the buffer is allocated from and what
    /// actions the buffer can partake in.
    ///
    /// Corresponds to [WebGPU `GPUBufferUsageFlags`](
    /// https://gpuweb.github.io/gpuweb/#typedefdef-gpubufferusageflags).
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct BufferUsages: u32 {
        /// Allow a buffer to be mapped for reading using [`Buffer::map_async`] + [`Buffer::get_mapped_range`].
        /// This does not include creating a buffer with [`BufferDescriptor::mapped_at_creation`] set.
        ///
        /// If [`Features::MAPPABLE_PRIMARY_BUFFERS`] isn't enabled, the only other usage a buffer
        /// may have is COPY_DST.
        const MAP_READ = 1 << 0;
        /// Allow a buffer to be mapped for writing using [`Buffer::map_async`] + [`Buffer::get_mapped_range_mut`].
        /// This does not include creating a buffer with `mapped_at_creation` set.
        ///
        /// If [`Features::MAPPABLE_PRIMARY_BUFFERS`] feature isn't enabled, the only other usage a buffer
        /// may have is COPY_SRC.
        const MAP_WRITE = 1 << 1;
        /// Allow a buffer to be the source buffer for a [`CommandEncoder::copy_buffer_to_buffer`] or [`CommandEncoder::copy_buffer_to_texture`]
        /// operation.
        const COPY_SRC = 1 << 2;
        /// Allow a buffer to be the destination buffer for a [`CommandEncoder::copy_buffer_to_buffer`], [`CommandEncoder::copy_texture_to_buffer`],
        /// [`CommandEncoder::clear_buffer`] or [`Queue::write_buffer`] operation.
        const COPY_DST = 1 << 3;
        /// Allow a buffer to be the index buffer in a draw operation.
        const INDEX = 1 << 4;
        /// Allow a buffer to be the vertex buffer in a draw operation.
        const VERTEX = 1 << 5;
        /// Allow a buffer to be a [`BufferBindingType::Uniform`] inside a bind group.
        const UNIFORM = 1 << 6;
        /// Allow a buffer to be a [`BufferBindingType::Storage`] inside a bind group.
        const STORAGE = 1 << 7;
        /// Allow a buffer to be the indirect buffer in an indirect draw call.
        const INDIRECT = 1 << 8;
        /// Allow a buffer to be the destination buffer for a [`CommandEncoder::resolve_query_set`] operation.
        const QUERY_RESOLVE = 1 << 9;
    }
}
bitflags::bitflags! {
    /// Describes the shader stages that a binding will be visible from.
    ///
    /// These can be combined so something that is visible from both vertex and fragment shaders can be defined as:
    ///
    /// `ShaderStages::VERTEX | ShaderStages::FRAGMENT`
    ///
    /// Corresponds to [WebGPU `GPUShaderStageFlags`](
    /// https://gpuweb.github.io/gpuweb/#typedefdef-gpushaderstageflags).
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ShaderStages: u32 {
        /// Binding is not visible from any shader stage.
        const NONE = 0;
        /// Binding is visible from the vertex shader of a render pipeline.
        const VERTEX = 1 << 0;
        /// Binding is visible from the fragment shader of a render pipeline.
        const FRAGMENT = 1 << 1;
        /// Binding is visible from the compute shader of a compute pipeline.
        const COMPUTE = 1 << 2;
        /// Binding is visible from the vertex and fragment shaders of a render pipeline.
        const VERTEX_FRAGMENT = Self::VERTEX.bits() | Self::FRAGMENT.bits();
    }
}
bitflags::bitflags! {
    /// Features that are not guaranteed to be supported.
    ///
    /// These are either part of the webgpu standard, or are extension features supported by
    /// wgpu when targeting native.
    ///
    /// If you want to use a feature, you need to first verify that the adapter supports
    /// the feature. If the adapter does not support the feature, requesting a device with it enabled
    /// will panic.
    ///
    /// Corresponds to [WebGPU `GPUFeatureName`](
    /// https://gpuweb.github.io/gpuweb/#enumdef-gpufeaturename).
    #[repr(transparent)]
    #[derive(Default)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Features: u64 {
        //
        // ---- Start numbering at 1 << 0 ----
        //
        // WebGPU features:
        //

        // API:

        /// By default, polygon depth is clipped to 0-1 range before/during rasterization.
        /// Anything outside of that range is rejected, and respective fragments are not touched.
        ///
        /// With this extension, we can disabling clipping. That allows
        /// shadow map occluders to be rendered into a tighter depth range.
        ///
        /// Supported platforms:
        /// - desktops
        /// - some mobile chips
        ///
        /// This is a web and native feature.
        const DEPTH_CLIP_CONTROL = 1 << 0;
        /// Enables use of Timestamp Queries. These queries tell the current gpu timestamp when
        /// all work before the query is finished.
        ///
        /// This feature allows the use of
        /// - [`CommandEncoder::write_timestamp`]
        /// - [`RenderPassDescriptor::timestamp_writes`]
        /// - [`ComputePassDescriptor::timestamp_writes`]
        /// to write out timestamps.
        /// For timestamps within passes refer to [`Features::TIMESTAMP_QUERY_INSIDE_PASSES`]
        ///
        /// They must be resolved using [`CommandEncoder::resolve_query_sets`] into a buffer,
        /// then the result must be multiplied by the timestamp period [`Queue::get_timestamp_period`]
        /// to get the timestamp in nanoseconds. Multiple timestamps can then be diffed to get the
        /// time for operations between them to finish.
        ///
        /// Supported Platforms:
        /// - Vulkan
        /// - DX12
        /// - Metal
        ///
        /// This is a web and native feature.
        const TIMESTAMP_QUERY = 1 << 1;
        /// Allows non-zero value for the `first_instance` member in indirect draw calls.
        ///
        /// If this feature is not enabled, and the `first_instance` member is non-zero, the behavior may be:
        /// - The draw call is ignored.
        /// - The draw call is executed as if the `first_instance` is zero.
        /// - The draw call is executed with the correct `first_instance` value.
        ///
        /// Supported Platforms:
        /// - Vulkan (mostly)
        /// - DX12
        /// - Metal on Apple3+ or Mac1+
        /// - OpenGL (Desktop 4.2+ with ARB_shader_draw_parameters only)
        ///
        /// Not Supported:
        /// - OpenGL ES / WebGL
        ///
        /// This is a web and native feature.
        const INDIRECT_FIRST_INSTANCE = 1 << 2;

        // 3..8 available

        // Shader:

        /// Allows shaders to acquire the FP16 ability
        ///
        /// Note: this is not supported in `naga` yet，only through `spirv-passthrough` right now.
        ///
        /// Supported Platforms:
        /// - Vulkan
        /// - Metal
        ///
        /// This is a web and native feature.
        const SHADER_F16 = 1 << 8;

        // 9..14 available

        // Texture Formats:

        // The features starting with a ? are features that might become part of the spec or
        // at the very least we can implement as native features; since they should cover all
        // possible formats and capabilities across backends.
        //
        // ? const FORMATS_TIER_1 = 1 << 14; (https://github.com/gpuweb/gpuweb/issues/3837)
        // ? const RW_STORAGE_TEXTURE_TIER_1 = 1 << 15; (https://github.com/gpuweb/gpuweb/issues/3838)

        /// Allows the [`wgpu::TextureUsages::STORAGE_BINDING`] usage on textures with format [`TextureFormat::Bgra8unorm`]
        ///
        /// Supported Platforms:
        /// - Vulkan
        /// - DX12
        /// - Metal
        ///
        /// This is a web and native feature.
        const BGRA8UNORM_STORAGE = 1 << 16;

        // ? const NORM16_FILTERABLE = 1 << 17; (https://github.com/gpuweb/gpuweb/issues/3839)
        // ? const NORM16_RESOLVE = 1 << 18; (https://github.com/gpuweb/gpuweb/issues/3839)

        /// Allows textures with formats "r32float", "rg32float", and "rgba32float" to be filterable.
        ///
        /// Supported Platforms:
        /// - Vulkan (mainly on Desktop GPUs)
        /// - DX12
        /// - Metal on macOS or Apple9+ GPUs, optional on iOS/iPadOS with Apple7/8 GPUs
        /// - GL with one of `GL_ARB_color_buffer_float`/`GL_EXT_color_buffer_float`/`OES_texture_float_linear`
        ///
        /// This is a web and native feature.
        const FLOAT32_FILTERABLE = 1 << 19;

        // ? const FLOAT32_BLENDABLE = 1 << 20; (https://github.com/gpuweb/gpuweb/issues/3556)
        // ? const 32BIT_FORMAT_MULTISAMPLE = 1 << 21; (https://github.com/gpuweb/gpuweb/issues/3844)
        // ? const 32BIT_FORMAT_RESOLVE = 1 << 22; (https://github.com/gpuweb/gpuweb/issues/3844)

        /// Allows for usage of textures of format [`TextureFormat::Rg11b10Float`] as a render target
        ///
        /// Supported platforms:
        /// - Vulkan
        /// - DX12
        /// - Metal
        ///
        /// This is a web and native feature.
        const RG11B10UFLOAT_RENDERABLE = 1 << 23;

        /// Allows for explicit creation of textures of format [`TextureFormat::Depth32FloatStencil8`]
        ///
        /// Supported platforms:
        /// - Vulkan (mostly)
        /// - DX12
        /// - Metal
        ///
        /// This is a web and native feature.
        const DEPTH32FLOAT_STENCIL8 = 1 << 24;
        /// Enables BCn family of compressed textures. All BCn textures use 4x4 pixel blocks
        /// with 8 or 16 bytes per block.
        ///
        /// Compressed textures sacrifice some quality in exchange for significantly reduced
        /// bandwidth usage.
        ///
        /// Support for this feature guarantees availability of [`TextureUsages::COPY_SRC | TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING`] for BCn formats.
        /// [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] may enable additional usages.
        ///
        /// Supported Platforms:
        /// - desktops
        ///
        /// This is a web and native feature.
        const TEXTURE_COMPRESSION_BC = 1 << 25;
        /// Enables ETC family of compressed textures. All ETC textures use 4x4 pixel blocks.
        /// ETC2 RGB and RGBA1 are 8 bytes per block. RTC2 RGBA8 and EAC are 16 bytes per block.
        ///
        /// Compressed textures sacrifice some quality in exchange for significantly reduced
        /// bandwidth usage.
        ///
        /// Support for this feature guarantees availability of [`TextureUsages::COPY_SRC | TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING`] for ETC2 formats.
        /// [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] may enable additional usages.
        ///
        /// Supported Platforms:
        /// - Vulkan on Intel
        /// - Mobile (some)
        ///
        /// This is a web and native feature.
        const TEXTURE_COMPRESSION_ETC2 = 1 << 26;
        /// Enables ASTC family of compressed textures. ASTC textures use pixel blocks varying from 4x4 to 12x12.
        /// Blocks are always 16 bytes.
        ///
        /// Compressed textures sacrifice some quality in exchange for significantly reduced
        /// bandwidth usage.
        ///
        /// Support for this feature guarantees availability of [`TextureUsages::COPY_SRC | TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING`] for ASTC formats with Unorm/UnormSrgb channel type.
        /// [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] may enable additional usages.
        ///
        /// Supported Platforms:
        /// - Vulkan on Intel
        /// - Mobile (some)
        ///
        /// This is a web and native feature.
        const TEXTURE_COMPRESSION_ASTC = 1 << 27;

        // ? const TEXTURE_COMPRESSION_ASTC_HDR = 1 << 28; (https://github.com/gpuweb/gpuweb/issues/3856)

        // 29..32 should be available but are for now occupied by native only texture related features
        // TEXTURE_FORMAT_16BIT_NORM & TEXTURE_COMPRESSION_ASTC_HDR will most likely become web features as well
        // TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES might not be necessary if we have all the texture features implemented

        //
        // ---- Restart Numbering for Native Features ---
        //
        // Native Features:
        //

        // Texture Formats:

        /// Enables normalized `16-bit` texture formats.
        ///
        /// Supported platforms:
        /// - Vulkan
        /// - DX12
        /// - Metal
        ///
        /// This is a native only feature.
        const TEXTURE_FORMAT_16BIT_NORM = 1 << 29;
        /// Enables ASTC HDR family of compressed textures.
        ///
        /// Compressed textures sacrifice some quality in exchange for significantly reduced
        /// bandwidth usage.
        ///
        /// Support for this feature guarantees availability of [`TextureUsages::COPY_SRC | TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING`] for ASTC formats with the HDR channel type.
        /// [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] may enable additional usages.
        ///
        /// Supported Platforms:
        /// - Metal
        /// - Vulkan
        /// - OpenGL
        ///
        /// This is a native only feature.
        const TEXTURE_COMPRESSION_ASTC_HDR = 1 << 30;
        /// Enables device specific texture format features.
        ///
        /// See `TextureFormatFeatures` for a listing of the features in question.
        ///
        /// By default only texture format properties as defined by the WebGPU specification are allowed.
        /// Enabling this feature flag extends the features of each format to the ones supported by the current device.
        /// Note that without this flag, read/write storage access is not allowed at all.
        ///
        /// This extension does not enable additional formats.
        ///
        /// This is a native only feature.
        const TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES = 1 << 31;

        // API:

        /// Enables use of Pipeline Statistics Queries. These queries tell the count of various operations
        /// performed between the start and stop call. Call [`RenderPassEncoder::begin_pipeline_statistics_query`] to start
        /// a query, then call [`RenderPassEncoder::end_pipeline_statistics_query`] to stop one.
        ///
        /// They must be resolved using [`CommandEncoder::resolve_query_sets`] into a buffer.
        /// The rules on how these resolve into buffers are detailed in the documentation for [`PipelineStatisticsTypes`].
        ///
        /// Supported Platforms:
        /// - Vulkan
        /// - DX12
        ///
        /// This is a native only feature with a [proposal](https://github.com/gpuweb/gpuweb/blob/0008bd30da2366af88180b511a5d0d0c1dffbc36/proposals/pipeline-statistics-query.md) for the web.
        const PIPELINE_STATISTICS_QUERY = 1 << 32;
        /// Allows for timestamp queries inside render passes.
        ///
        /// Implies [`Features::TIMESTAMP_QUERY`] is supported.
        ///
        /// Additionally allows for timestamp queries to be used inside render & compute passes using:
        /// - [`RenderPassEncoder::write_timestamp`]
        /// - [`ComputePassEncoder::write_timestamp`]
        ///
        /// Supported platforms:
        /// - Vulkan
        /// - DX12
        /// - Metal (AMD & Intel, not Apple GPUs)
        ///
        /// This is generally not available on tile-based rasterization GPUs.
        ///
        /// This is a native only feature with a [proposal](https://github.com/gpuweb/gpuweb/blob/0008bd30da2366af88180b511a5d0d0c1dffbc36/proposals/timestamp-query-inside-passes.md) for the web.
        const TIMESTAMP_QUERY_INSIDE_PASSES = 1 << 33;
        /// Webgpu only allows the MAP_READ and MAP_WRITE buffer usage to be matched with
        /// COPY_DST and COPY_SRC respectively. This removes this requirement.
        ///
        /// This is only beneficial on systems that share memory between CPU and GPU. If enabled
        /// on a system that doesn't, this can severely hinder performance. Only use if you understand
        /// the consequences.
        ///
        /// Supported platforms:
        /// - Vulkan
        /// - DX12
        /// - Metal
        ///
        /// This is a native only feature.
        const MAPPABLE_PRIMARY_BUFFERS = 1 << 34;
        /// Allows the user to create uniform arrays of textures in shaders:
        ///
        /// ex.
        /// - `var textures: binding_array<texture_2d<f32>, 10>` (WGSL)
        /// - `uniform texture2D textures[10]` (GLSL)
        ///
        /// If [`Features::STORAGE_RESOURCE_BINDING_ARRAY`] is supported as well as this, the user
        /// may also create uniform arrays of storage textures.
        ///
        /// ex.
        /// - `var textures: array<texture_storage_2d<f32, write>, 10>` (WGSL)
        /// - `uniform image2D textures[10]` (GLSL)
        ///
        /// This capability allows them to exist and to be indexed by dynamically uniform
        /// values.
        ///
        /// Supported platforms:
        /// - DX12
        /// - Metal (with MSL 2.0+ on macOS 10.13+)
        /// - Vulkan
        ///
        /// This is a native only feature.
        const TEXTURE_BINDING_ARRAY = 1 << 35;
        /// Allows the user to create arrays of buffers in shaders:
        ///
        /// ex.
        /// - `var<uniform> buffer_array: array<MyBuffer, 10>` (WGSL)
        /// - `uniform myBuffer { ... } buffer_array[10]` (GLSL)
        ///
        /// This capability allows them to exist and to be indexed by dynamically uniform
        /// values.
        ///
        /// If [`Features::STORAGE_RESOURCE_BINDING_ARRAY`] is supported as well as this, the user
        /// may also create arrays of storage buffers.
        ///
        /// ex.
        /// - `var<storage> buffer_array: array<MyBuffer, 10>` (WGSL)
        /// - `buffer myBuffer { ... } buffer_array[10]` (GLSL)
        ///
        /// Supported platforms:
        /// - DX12
        /// - Vulkan
        ///
        /// This is a native only feature.
        const BUFFER_BINDING_ARRAY = 1 << 36;
        /// Allows the user to create uniform arrays of storage buffers or textures in shaders,
        /// if resp. [`Features::BUFFER_BINDING_ARRAY`] or [`Features::TEXTURE_BINDING_ARRAY`]
        /// is supported.
        ///
        /// This capability allows them to exist and to be indexed by dynamically uniform
        /// values.
        ///
        /// Supported platforms:
        /// - Metal (with MSL 2.2+ on macOS 10.13+)
        /// - Vulkan
        ///
        /// This is a native only feature.
        const STORAGE_RESOURCE_BINDING_ARRAY = 1 << 37;
        /// Allows shaders to index sampled texture and storage buffer resource arrays with dynamically non-uniform values:
        ///
        /// ex. `texture_array[vertex_data]`
        ///
        /// In order to use this capability, the corresponding GLSL extension must be enabled like so:
        ///
        /// `#extension GL_EXT_nonuniform_qualifier : require`
        ///
        /// and then used either as `nonuniformEXT` qualifier in variable declaration:
        ///
        /// ex. `layout(location = 0) nonuniformEXT flat in int vertex_data;`
        ///
        /// or as `nonuniformEXT` constructor:
        ///
        /// ex. `texture_array[nonuniformEXT(vertex_data)]`
        ///
        /// WGSL and HLSL do not need any extension.
        ///
        /// Supported platforms:
        /// - DX12
        /// - Metal (with MSL 2.0+ on macOS 10.13+)
        /// - Vulkan 1.2+ (or VK_EXT_descriptor_indexing)'s shaderSampledImageArrayNonUniformIndexing & shaderStorageBufferArrayNonUniformIndexing feature)
        ///
        /// This is a native only feature.
        const SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING = 1 << 38;
        /// Allows shaders to index uniform buffer and storage texture resource arrays with dynamically non-uniform values:
        ///
        /// ex. `texture_array[vertex_data]`
        ///
        /// In order to use this capability, the corresponding GLSL extension must be enabled like so:
        ///
        /// `#extension GL_EXT_nonuniform_qualifier : require`
        ///
        /// and then used either as `nonuniformEXT` qualifier in variable declaration:
        ///
        /// ex. `layout(location = 0) nonuniformEXT flat in int vertex_data;`
        ///
        /// or as `nonuniformEXT` constructor:
        ///
        /// ex. `texture_array[nonuniformEXT(vertex_data)]`
        ///
        /// WGSL and HLSL do not need any extension.
        ///
        /// Supported platforms:
        /// - DX12
        /// - Metal (with MSL 2.0+ on macOS 10.13+)
        /// - Vulkan 1.2+ (or VK_EXT_descriptor_indexing)'s shaderUniformBufferArrayNonUniformIndexing & shaderStorageTextureArrayNonUniformIndexing feature)
        ///
        /// This is a native only feature.
        const UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING = 1 << 39;
        /// Allows the user to create bind groups containing arrays with less bindings than the BindGroupLayout.
        ///
        /// This is a native only feature.
        const PARTIALLY_BOUND_BINDING_ARRAY = 1 << 40;
        /// Allows the user to call [`RenderPass::multi_draw_indirect`] and [`RenderPass::multi_draw_indexed_indirect`].
        ///
        /// Allows multiple indirect calls to be dispatched from a single buffer.
        ///
        /// Supported platforms:
        /// - DX12
        /// - Vulkan
        /// - Metal on Apple3+ or Mac1+ (Emulated on top of `draw_indirect` and `draw_indexed_indirect`)
        ///
        /// This is a native only feature.
        ///
        /// [`RenderPass::multi_draw_indirect`]: ../wgpu/struct.RenderPass.html#method.multi_draw_indirect
        /// [`RenderPass::multi_draw_indexed_indirect`]: ../wgpu/struct.RenderPass.html#method.multi_draw_indexed_indirect
        const MULTI_DRAW_INDIRECT = 1 << 41;
        /// Allows the user to call [`RenderPass::multi_draw_indirect_count`] and [`RenderPass::multi_draw_indexed_indirect_count`].
        ///
        /// This allows the use of a buffer containing the actual number of draw calls.
        ///
        /// Supported platforms:
        /// - DX12
        /// - Vulkan 1.2+ (or VK_KHR_draw_indirect_count)
        ///
        /// This is a native only feature.
        ///
        /// [`RenderPass::multi_draw_indirect_count`]: ../wgpu/struct.RenderPass.html#method.multi_draw_indirect_count
        /// [`RenderPass::multi_draw_indexed_indirect_count`]: ../wgpu/struct.RenderPass.html#method.multi_draw_indexed_indirect_count
        const MULTI_DRAW_INDIRECT_COUNT = 1 << 42;
        /// Allows the use of push constants: small, fast bits of memory that can be updated
        /// inside a [`RenderPass`].
        ///
        /// Allows the user to call [`RenderPass::set_push_constants`], provide a non-empty array
        /// to [`PipelineLayoutDescriptor`], and provide a non-zero limit to [`Limits::max_push_constant_size`].
        ///
        /// A block of push constants can be declared with `layout(push_constant) uniform Name {..}` in shaders.
        ///
        /// Supported platforms:
        /// - DX12
        /// - Vulkan
        /// - Metal
        /// - OpenGL (emulated with uniforms)
        ///
        /// This is a native only feature.
        ///
        /// [`RenderPass`]: ../wgpu/struct.RenderPass.html
        /// [`PipelineLayoutDescriptor`]: ../wgpu/struct.PipelineLayoutDescriptor.html
        /// [`RenderPass::set_push_constants`]: ../wgpu/struct.RenderPass.html#method.set_push_constants
        const PUSH_CONSTANTS = 1 << 43;
        /// Allows the use of [`AddressMode::ClampToBorder`] with a border color
        /// of [`SamplerBorderColor::Zero`].
        ///
        /// Supported platforms:
        /// - DX12
        /// - Vulkan
        /// - Metal
        /// - OpenGL
        ///
        /// This is a native only feature.
        const ADDRESS_MODE_CLAMP_TO_ZERO = 1 << 44;
        /// Allows the use of [`AddressMode::ClampToBorder`] with a border color
        /// other than [`SamplerBorderColor::Zero`].
        ///
        /// Supported platforms:
        /// - DX12
        /// - Vulkan
        /// - Metal (macOS 10.12+ only)
        /// - OpenGL
        ///
        /// This is a native only feature.
        const ADDRESS_MODE_CLAMP_TO_BORDER = 1 << 45;
        /// Allows the user to set [`PolygonMode::Line`] in [`PrimitiveState::polygon_mode`]
        ///
        /// This allows drawing polygons/triangles as lines (wireframe) instead of filled
        ///
        /// Supported platforms:
        /// - DX12
        /// - Vulkan
        /// - Metal
        ///
        /// This is a native only feature.
        const POLYGON_MODE_LINE = 1 << 46;
        /// Allows the user to set [`PolygonMode::Point`] in [`PrimitiveState::polygon_mode`]
        ///
        /// This allows only drawing the vertices of polygons/triangles instead of filled
        ///
        /// Supported platforms:
        /// - Vulkan
        ///
        /// This is a native only feature.
        const POLYGON_MODE_POINT = 1 << 47;
        /// Allows the user to set a overestimation-conservative-rasterization in [`PrimitiveState::conservative`]
        ///
        /// Processing of degenerate triangles/lines is hardware specific.
        /// Only triangles are supported.
        ///
        /// Supported platforms:
        /// - Vulkan
        ///
        /// This is a native only feature.
        const CONSERVATIVE_RASTERIZATION = 1 << 48;
        /// Enables bindings of writable storage buffers and textures visible to vertex shaders.
        ///
        /// Note: some (tiled-based) platforms do not support vertex shaders with any side-effects.
        ///
        /// Supported Platforms:
        /// - All
        ///
        /// This is a native only feature.
        const VERTEX_WRITABLE_STORAGE = 1 << 49;
        /// Enables clear to zero for textures.
        ///
        /// Supported platforms:
        /// - All
        ///
        /// This is a native only feature.
        const CLEAR_TEXTURE = 1 << 50;
        /// Enables creating shader modules from SPIR-V binary data (unsafe).
        ///
        /// SPIR-V data is not parsed or interpreted in any way; you can use
        /// [`wgpu::make_spirv_raw!`] to check for alignment and magic number when converting from
        /// raw bytes.
        ///
        /// Supported platforms:
        /// - Vulkan, in case shader's requested capabilities and extensions agree with
        /// Vulkan implementation.
        ///
        /// This is a native only feature.
        const SPIRV_SHADER_PASSTHROUGH = 1 << 51;
        /// Enables multiview render passes and `builtin(view_index)` in vertex shaders.
        ///
        /// Supported platforms:
        /// - Vulkan
        /// - OpenGL (web only)
        ///
        /// This is a native only feature.
        const MULTIVIEW = 1 << 52;
        /// Enables using 64-bit types for vertex attributes.
        ///
        /// Requires SHADER_FLOAT64.
        ///
        /// Supported Platforms: N/A
        ///
        /// This is a native only feature.
        const VERTEX_ATTRIBUTE_64BIT = 1 << 53;
        /// Allows vertex shaders to have outputs which are not consumed
        /// by the fragment shader.
        ///
        /// Supported platforms:
        /// - Vulkan
        /// - Metal
        /// - OpenGL
        const SHADER_UNUSED_VERTEX_OUTPUT = 1 << 54;
        /// Allows for creation of textures of format [`TextureFormat::NV12`]
        ///
        /// Supported platforms:
        /// - DX12
        /// - Vulkan
        ///
        /// This is a native only feature.
        const TEXTURE_FORMAT_NV12 = 1 << 55;
        /// Allows for the creation of ray-tracing acceleration structures.
        ///
        /// Supported platforms:
        /// - Vulkan
        ///
        /// This is a native-only feature.
        const RAY_TRACING_ACCELERATION_STRUCTURE = 1 << 56;

        // 57 available

        // Shader:

        /// Allows for the creation of ray-tracing queries within shaders.
        ///
        /// Supported platforms:
        /// - Vulkan
        ///
        /// This is a native-only feature.
        const RAY_QUERY = 1 << 58;
        /// Enables 64-bit floating point types in SPIR-V shaders.
        ///
        /// Note: even when supported by GPU hardware, 64-bit floating point operations are
        /// frequently between 16 and 64 _times_ slower than equivalent operations on 32-bit floats.
        ///
        /// Supported Platforms:
        /// - Vulkan
        ///
        /// This is a native only feature.
        const SHADER_F64 = 1 << 59;
        /// Allows shaders to use i16. Not currently supported in `naga`, only available through `spirv-passthrough`.
        ///
        /// Supported platforms:
        /// - Vulkan
        ///
        /// This is a native only feature.
        const SHADER_I16 = 1 << 60;
        /// Enables `builtin(primitive_index)` in fragment shaders.
        ///
        /// Note: enables geometry processing for pipelines using the builtin.
        /// This may come with a significant performance impact on some hardware.
        /// Other pipelines are not affected.
        ///
        /// Supported platforms:
        /// - Vulkan
        /// - DX12
        /// - Metal (some)
        /// - OpenGL (some)
        ///
        /// This is a native only feature.
        const SHADER_PRIMITIVE_INDEX = 1 << 61;
        /// Allows shaders to use the `early_depth_test` attribute.
        ///
        /// Supported platforms:
        /// - GLES 3.1+
        ///
        /// This is a native only feature.
        const SHADER_EARLY_DEPTH_TEST = 1 << 62;
        /// Allows two outputs from a shader to be used for blending.
        /// Note that dual-source blending doesn't support multiple render targets.
        ///
        /// For more info see the OpenGL ES extension GL_EXT_blend_func_extended.
        ///
        /// Supported platforms:
        /// - OpenGL ES (with GL_EXT_blend_func_extended)
        /// - Metal (with MSL 1.2+)
        /// - Vulkan (with dualSrcBlend)
        /// - DX12
        const DUAL_SOURCE_BLENDING = 1 << 63;
    }
}
bitflags::bitflags! {
    /// Different ways that you can use a texture.
    ///
    /// The usages determine what kind of memory the texture is allocated from and what
    /// actions the texture can partake in.
    ///
    /// Corresponds to [WebGPU `GPUTextureUsageFlags`](
    /// https://gpuweb.github.io/gpuweb/#typedefdef-gputextureusageflags).
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct TextureUsages: u32 {
        /// Allows a texture to be the source in a [`CommandEncoder::copy_texture_to_buffer`] or
        /// [`CommandEncoder::copy_texture_to_texture`] operation.
        const COPY_SRC = 1 << 0;
        /// Allows a texture to be the destination in a  [`CommandEncoder::copy_buffer_to_texture`],
        /// [`CommandEncoder::copy_texture_to_texture`], or [`Queue::write_texture`] operation.
        const COPY_DST = 1 << 1;
        /// Allows a texture to be a [`BindingType::Texture`] in a bind group.
        const TEXTURE_BINDING = 1 << 2;
        /// Allows a texture to be a [`BindingType::StorageTexture`] in a bind group.
        const STORAGE_BINDING = 1 << 3;
        /// Allows a texture to be an output attachment of a render pass.
        const RENDER_ATTACHMENT = 1 << 4;
    }
}
bitflags::bitflags! {
    /// Color write mask. Disabled color channels will not be written to.
    ///
    /// Corresponds to [WebGPU `GPUColorWriteFlags`](
    /// https://gpuweb.github.io/gpuweb/#typedefdef-gpucolorwriteflags).
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ColorWrites: u32 {
        /// Enable red channel writes
        const RED = 1 << 0;
        /// Enable green channel writes
        const GREEN = 1 << 1;
        /// Enable blue channel writes
        const BLUE = 1 << 2;
        /// Enable alpha channel writes
        const ALPHA = 1 << 3;
        /// Enable red, green, and blue channel writes
        const COLOR = Self::RED.bits() | Self::GREEN.bits() | Self::BLUE.bits();
        /// Enable writes to all channels.
        const ALL = Self::RED.bits() | Self::GREEN.bits() | Self::BLUE.bits() | Self::ALPHA.bits();
    }
}
