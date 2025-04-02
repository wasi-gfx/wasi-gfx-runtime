use std::borrow::Cow;

use wasmtime::component::ResourceTable;

use crate::wasi::webgpu::webgpu;

pub trait ToCore<T> {
    fn to_core(self, table: &ResourceTable) -> T;
}

impl<T> ToCore<T> for wasmtime::component::Resource<T>
where
    T: Copy + 'static,
{
    fn to_core(self, table: &ResourceTable) -> T {
        *table.get(&self).unwrap()
    }
}

impl ToCore<wgpu_types::RequestAdapterOptions<wgpu_core::id::SurfaceId>>
    for webgpu::GpuRequestAdapterOptions
{
    fn to_core(
        self,
        _table: &ResourceTable,
    ) -> wgpu_types::RequestAdapterOptions<wgpu_core::id::SurfaceId> {
        wgpu_types::RequestAdapterOptions {
            power_preference: self
                .power_preference
                .map(|pp| pp.into())
                .unwrap_or(wgpu_types::PowerPreference::None),
            // https://www.w3.org/TR/webgpu/#adapter-selection
            force_fallback_adapter: self.force_fallback_adapter.unwrap_or(false),
            compatible_surface: None,
        }
    }
}

impl ToCore<wgpu_types::Extent3d> for webgpu::GpuExtent3D {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::Extent3d {
        // https://www.w3.org/TR/webgpu/#dictdef-gpuextent3ddict
        wgpu_types::Extent3d {
            width: self.width,
            height: self.height.unwrap_or(1),
            depth_or_array_layers: self.depth_or_array_layers.unwrap_or(1),
        }
    }
}

impl ToCore<wgpu_core::binding_model::BindGroupDescriptor<'static>>
    for webgpu::GpuBindGroupDescriptor
{
    fn to_core(
        self,
        table: &ResourceTable,
    ) -> wgpu_core::binding_model::BindGroupDescriptor<'static> {
        wgpu_core::binding_model::BindGroupDescriptor {
            label: self.label.map(|l| l.into()),
            layout: self.layout.to_core(table),
            entries: self.entries.into_iter().map(|e| e.to_core(table)).collect(),
        }
    }
}

impl<'a> ToCore<wgpu_core::binding_model::BindGroupEntry<'a>> for webgpu::GpuBindGroupEntry {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::binding_model::BindGroupEntry<'a> {
        wgpu_core::binding_model::BindGroupEntry {
            binding: self.binding,
            resource: self.resource.to_core(table),
        }
    }
}

impl<'a> ToCore<wgpu_core::binding_model::BindingResource<'a>> for webgpu::GpuBindingResource {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::binding_model::BindingResource<'a> {
        match self {
            webgpu::GpuBindingResource::GpuBufferBinding(buffer) => {
                wgpu_core::binding_model::BindingResource::Buffer(buffer.to_core(table))
            }
            webgpu::GpuBindingResource::GpuSampler(sampler) => {
                wgpu_core::binding_model::BindingResource::Sampler(sampler.to_core(table))
            }
            webgpu::GpuBindingResource::GpuTextureView(texture_view) => {
                wgpu_core::binding_model::BindingResource::TextureView(texture_view.to_core(table))
            }
        }
    }
}

impl<'a> ToCore<wgpu_core::binding_model::BufferBinding> for webgpu::GpuBufferBinding {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::binding_model::BufferBinding {
        let buffer = table.get(&self.buffer).unwrap();
        // https://www.w3.org/TR/webgpu/#dictdef-gpubufferbinding
        wgpu_core::binding_model::BufferBinding {
            buffer_id: buffer.buffer_id,
            offset: self.offset.unwrap_or(0),
            size: self.size.map(|s| s.try_into().unwrap()),
        }
    }
}

impl<'a> ToCore<wgpu_types::CommandEncoderDescriptor<wgpu_core::Label<'a>>>
    for webgpu::GpuCommandEncoderDescriptor
{
    fn to_core(
        self,
        _table: &ResourceTable,
    ) -> wgpu_types::CommandEncoderDescriptor<wgpu_core::Label<'a>> {
        wgpu_types::CommandEncoderDescriptor {
            label: self.label.map(|l| l.into()),
        }
    }
}

impl<'a> ToCore<wgpu_core::pipeline::ShaderModuleDescriptor<'a>>
    for webgpu::GpuShaderModuleDescriptor
{
    fn to_core(self, _table: &ResourceTable) -> wgpu_core::pipeline::ShaderModuleDescriptor<'a> {
        wgpu_core::pipeline::ShaderModuleDescriptor {
            label: self.label.map(|label| label.into()),
            shader_bound_checks: wgpu_types::ShaderBoundChecks::new(),
        }
    }
}

impl<'a> ToCore<wgpu_core::resource::TextureViewDescriptor<'a>>
    for webgpu::GpuTextureViewDescriptor
{
    fn to_core(self, _table: &ResourceTable) -> wgpu_core::resource::TextureViewDescriptor<'a> {
        wgpu_core::resource::TextureViewDescriptor {
            label: self.label.map(|l| l.into()),
            format: self.format.map(|f| f.into()),
            dimension: self.dimension.map(|d| d.into()),
            // https://www.w3.org/TR/webgpu/#texture-view-creation
            range: wgpu_types::ImageSubresourceRange {
                aspect: self
                    .aspect
                    .map(|a| a.into())
                    .unwrap_or(wgpu_types::TextureAspect::All),
                base_mip_level: self.base_mip_level.unwrap_or(0),
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer.unwrap_or(0),
                array_layer_count: self.array_layer_count,
            },
        }
    }
}

impl<'a> ToCore<wgpu_core::binding_model::PipelineLayoutDescriptor<'a>>
    for webgpu::GpuPipelineLayoutDescriptor
{
    fn to_core(
        self,
        table: &ResourceTable,
    ) -> wgpu_core::binding_model::PipelineLayoutDescriptor<'a> {
        wgpu_core::binding_model::PipelineLayoutDescriptor {
            label: self.label.map(|l| l.into()),
            bind_group_layouts: self
                .bind_group_layouts
                .into_iter()
                .map(|bind_group_layout| {
                    *table
                        .get(&bind_group_layout.expect("TODO: handle null"))
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .into(),
            push_constant_ranges: vec![].into(),
        }
    }
}

impl<'a> ToCore<wgpu_core::pipeline::RenderPipelineDescriptor<'a>>
    for webgpu::GpuRenderPipelineDescriptor
{
    fn to_core(self, table: &ResourceTable) -> wgpu_core::pipeline::RenderPipelineDescriptor<'a> {
        wgpu_core::pipeline::RenderPipelineDescriptor {
            label: self.label.map(|l| l.into()),
            layout: match self.layout {
                webgpu::GpuLayoutMode::Specific(layout) => Some(layout.to_core(table)),
                webgpu::GpuLayoutMode::Auto => None,
            },
            vertex: self.vertex.to_core(table),
            primitive: self.primitive.map(|p| p.to_core(table)).unwrap_or_default(),
            depth_stencil: self.depth_stencil.map(|ds| ds.to_core(table)),
            multisample: self
                .multisample
                .map(|ms| ms.to_core(table))
                .unwrap_or_default(),
            fragment: self.fragment.map(|f| f.to_core(table)),
            // multiview and cache are not present in WebGPU
            multiview: None,
            cache: None,
        }
    }
}

impl ToCore<wgpu_types::MultisampleState> for webgpu::GpuMultisampleState {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::MultisampleState {
        // https://www.w3.org/TR/webgpu/#dictdef-gpumultisamplestate
        wgpu_types::MultisampleState {
            count: self.count.unwrap_or(1),
            mask: self.mask.unwrap_or(0xFFFFFFFF).into(),
            alpha_to_coverage_enabled: self.alpha_to_coverage_enabled.unwrap_or(false),
        }
    }
}

impl ToCore<wgpu_types::DepthStencilState> for webgpu::GpuDepthStencilState {
    fn to_core(self, table: &ResourceTable) -> wgpu_types::DepthStencilState {
        // https://www.w3.org/TR/webgpu/#depth-stencil-state
        wgpu_types::DepthStencilState {
            format: self.format.into(),
            depth_write_enabled: self.depth_write_enabled.expect("TODO: handle null").into(),
            depth_compare: self.depth_compare.expect("TODO: handle null").into(),
            stencil: wgpu_types::StencilState {
                front: self
                    .stencil_front
                    .map(|f| f.to_core(table))
                    .unwrap_or_default(),
                back: self
                    .stencil_back
                    .map(|b| b.to_core(table))
                    .unwrap_or_default(),
                read_mask: self.stencil_read_mask.unwrap_or(0xFFFFFFFF),
                write_mask: self.stencil_write_mask.unwrap_or(0xFFFFFFFF),
            },
            bias: wgpu_types::DepthBiasState {
                constant: self.depth_bias.unwrap_or(0),
                slope_scale: self.depth_bias_slope_scale.unwrap_or(0.0),
                clamp: self.depth_bias_clamp.unwrap_or(0.0),
            },
        }
    }
}

impl ToCore<wgpu_types::StencilFaceState> for webgpu::GpuStencilFaceState {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::StencilFaceState {
        wgpu_types::StencilFaceState {
            compare: self
                .compare
                .map(|c| c.into())
                .unwrap_or(wgpu_types::CompareFunction::Always),
            fail_op: self
                .fail_op
                .map(|fo| fo.into())
                .unwrap_or(wgpu_types::StencilOperation::Keep),
            depth_fail_op: self
                .depth_fail_op
                .map(|dfo| dfo.into())
                .unwrap_or(wgpu_types::StencilOperation::Keep),
            pass_op: self
                .pass_op
                .map(|po| po.into())
                .unwrap_or(wgpu_types::StencilOperation::Keep),
        }
    }
}

impl ToCore<wgpu_types::PrimitiveState> for webgpu::GpuPrimitiveState {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::PrimitiveState {
        // https://www.w3.org/TR/webgpu/#dictdef-gpuprimitivestate
        wgpu_types::PrimitiveState {
            topology: self
                .topology
                .map(|t| t.into())
                .unwrap_or(wgpu_types::PrimitiveTopology::TriangleList),
            strip_index_format: self.strip_index_format.map(|f| f.into()),
            front_face: self
                .front_face
                .map(|x| x.into())
                .unwrap_or(wgpu_types::FrontFace::Ccw),
            cull_mode: self.cull_mode.map(|cm| cm.into()),
            unclipped_depth: self.unclipped_depth.unwrap_or(false),
            // polygon_mode and conservative are not present in WebGPU
            polygon_mode: wgpu_types::PolygonMode::Fill,
            conservative: false,
        }
    }
}

impl<'a> ToCore<wgpu_core::pipeline::FragmentState<'a>> for webgpu::GpuFragmentState {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::pipeline::FragmentState<'a> {
        wgpu_core::pipeline::FragmentState {
            stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                module: self.module.to_core(table),
                entry_point: self.entry_point.map(|ep| ep.into()),
                constants: self
                    .constants
                    .map(|constants| {
                        // TODO: can we get rid of the clone here?
                        let constants = table.get(&constants).unwrap().clone();
                        Cow::Owned(constants)
                    })
                    .unwrap_or_default(),
                zero_initialize_workgroup_memory: true,
                vertex_pulling_transform: false,
            },
            targets: self
                .targets
                .into_iter()
                .map(|t| t.map(|t| t.to_core(table)))
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

impl ToCore<wgpu_types::ColorTargetState> for webgpu::GpuColorTargetState {
    fn to_core(self, table: &ResourceTable) -> wgpu_types::ColorTargetState {
        // https://www.w3.org/TR/webgpu/#color-target-state
        wgpu_types::ColorTargetState {
            format: self.format.into(),
            blend: self.blend.map(|b| b.to_core(table)),
            write_mask: wgpu_types::ColorWrites::from_bits(self.write_mask.unwrap_or(0xF)).unwrap(),
        }
    }
}

impl ToCore<wgpu_types::BlendState> for webgpu::GpuBlendState {
    fn to_core(self, table: &ResourceTable) -> wgpu_types::BlendState {
        wgpu_types::BlendState {
            color: self.color.to_core(table),
            alpha: self.alpha.to_core(table),
        }
    }
}

impl ToCore<wgpu_types::BlendComponent> for webgpu::GpuBlendComponent {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::BlendComponent {
        wgpu_types::BlendComponent {
            // https://www.w3.org/TR/webgpu/#dictdef-gpublendcomponent
            operation: self
                .operation
                .map(|x| x.into())
                .unwrap_or(wgpu_types::BlendOperation::Add),
            src_factor: self
                .src_factor
                .map(|x| x.into())
                .unwrap_or(wgpu_types::BlendFactor::One),
            dst_factor: self
                .dst_factor
                .map(|x| x.into())
                .unwrap_or(wgpu_types::BlendFactor::Zero),
        }
    }
}

impl<'a> ToCore<wgpu_core::pipeline::VertexState<'a>> for webgpu::GpuVertexState {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::pipeline::VertexState<'a> {
        // https://www.w3.org/TR/webgpu/#dictdef-gpuvertexstate
        wgpu_core::pipeline::VertexState {
            stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                module: self.module.to_core(table),
                entry_point: self.entry_point.map(|e| e.into()),
                constants: self
                    .constants
                    .map(|constants| {
                        // TODO: can we get rid of the clone here?
                        let constants = table.get(&constants).unwrap().clone();
                        Cow::Owned(constants)
                    })
                    .unwrap_or_default(),
                zero_initialize_workgroup_memory: true,
                vertex_pulling_transform: false,
            },
            buffers: self
                .buffers
                .map(|buffer| {
                    buffer
                        .into_iter()
                        .map(|b| {
                            b.map(|b| b.to_core(table))
                                .expect("TODO: deal with `none` values")
                        })
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

impl<'a> ToCore<wgpu_core::pipeline::VertexBufferLayout<'a>> for webgpu::GpuVertexBufferLayout {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::pipeline::VertexBufferLayout<'a> {
        // https://www.w3.org/TR/webgpu/#dictdef-gpuvertexbufferlayout
        wgpu_core::pipeline::VertexBufferLayout {
            array_stride: self.array_stride,
            step_mode: self
                .step_mode
                .map(|sm| sm.into())
                .unwrap_or(wgpu_types::VertexStepMode::Vertex),
            attributes: self
                .attributes
                .into_iter()
                .map(|a| a.to_core(table))
                .collect(),
        }
    }
}

impl ToCore<wgpu_types::VertexAttribute> for webgpu::GpuVertexAttribute {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::VertexAttribute {
        wgpu_types::VertexAttribute {
            format: self.format.into(),
            offset: self.offset,
            shader_location: self.shader_location,
        }
    }
}

impl<'a> ToCore<wgpu_types::TextureDescriptor<wgpu_core::Label<'a>, Vec<wgpu_types::TextureFormat>>>
    for webgpu::GpuTextureDescriptor
{
    fn to_core(
        self,
        table: &ResourceTable,
    ) -> wgpu_types::TextureDescriptor<wgpu_core::Label<'a>, Vec<wgpu_types::TextureFormat>> {
        // https://www.w3.org/TR/webgpu/#gputexturedescriptor
        wgpu_types::TextureDescriptor {
            label: self.label.map(|l| l.into()),
            size: self.size.to_core(table),
            mip_level_count: self.mip_level_count.unwrap_or(1),
            sample_count: self.sample_count.unwrap_or(1),
            dimension: self
                .dimension
                .unwrap_or(webgpu::GpuTextureDimension::D2)
                .into(),
            format: self.format.into(),
            usage: wgpu_types::TextureUsages::from_bits(self.usage).unwrap(),
            view_formats: self
                .view_formats
                .map(|view_formats| {
                    view_formats
                        .into_iter()
                        .map(|view_format| view_format.into())
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

impl<'a> ToCore<wgpu_core::resource::SamplerDescriptor<'a>> for webgpu::GpuSamplerDescriptor {
    fn to_core(self, _table: &ResourceTable) -> wgpu_core::resource::SamplerDescriptor<'a> {
        // https://www.w3.org/TR/webgpu/#GPUSamplerDescriptor
        wgpu_core::resource::SamplerDescriptor {
            label: self.label.map(|l| l.into()),
            address_modes: [
                self.address_mode_u
                    .map(|am| am.into())
                    .unwrap_or(wgpu_types::AddressMode::ClampToEdge),
                self.address_mode_v
                    .map(|am| am.into())
                    .unwrap_or(wgpu_types::AddressMode::ClampToEdge),
                self.address_mode_w
                    .map(|am| am.into())
                    .unwrap_or(wgpu_types::AddressMode::ClampToEdge),
            ],
            mag_filter: self
                .mag_filter
                .map(|mf| mf.into())
                .unwrap_or(wgpu_types::FilterMode::Nearest),
            min_filter: self
                .min_filter
                .map(|mf| mf.into())
                .unwrap_or(wgpu_types::FilterMode::Nearest),
            mipmap_filter: self
                .mipmap_filter
                .map(|mf| mf.into())
                .unwrap_or(wgpu_types::FilterMode::Nearest),
            lod_min_clamp: self.lod_min_clamp.unwrap_or(0.0),
            lod_max_clamp: self.lod_max_clamp.unwrap_or(32.0),
            compare: self.compare.map(|compare| compare.into()),
            // TODO: make sure that anisotropy_clamp actually corresponds to maxAnisotropy
            anisotropy_clamp: self.max_anisotropy.unwrap_or(1),
            // border_color is not present in WebGPU
            border_color: None,
        }
    }
}

impl<'a> ToCore<wgpu_types::CommandBufferDescriptor<wgpu_core::Label<'a>>>
    for webgpu::GpuCommandBufferDescriptor
{
    fn to_core(
        self,
        _table: &ResourceTable,
    ) -> wgpu_types::CommandBufferDescriptor<wgpu_core::Label<'a>> {
        wgpu_types::CommandBufferDescriptor {
            label: self.label.map(|l| l.into()),
        }
    }
}

impl<'a> ToCore<wgpu_core::binding_model::BindGroupLayoutDescriptor<'a>>
    for webgpu::GpuBindGroupLayoutDescriptor
{
    fn to_core(
        self,
        table: &ResourceTable,
    ) -> wgpu_core::binding_model::BindGroupLayoutDescriptor<'a> {
        wgpu_core::binding_model::BindGroupLayoutDescriptor {
            label: self.label.map(|l| l.into()),
            entries: self
                .entries
                .into_iter()
                .map(|entry| entry.to_core(table))
                .collect(),
        }
    }
}

impl ToCore<wgpu_types::BindGroupLayoutEntry> for webgpu::GpuBindGroupLayoutEntry {
    fn to_core(self, table: &ResourceTable) -> wgpu_types::BindGroupLayoutEntry {
        wgpu_types::BindGroupLayoutEntry {
            binding: self.binding.into(),
            visibility: wgpu_types::ShaderStages::from_bits(self.visibility).unwrap(),
            ty: match (
                self.buffer,
                self.sampler,
                self.texture,
                self.storage_texture,
            ) {
                (Some(buffer), None, None, None) => buffer.to_core(table),
                (None, Some(sampler), None, None) => sampler.to_core(table),
                (None, None, Some(texture), None) => texture.to_core(table),
                (None, None, None, Some(storage_texture)) => storage_texture.to_core(table),
                (None, None, None, None) => todo!(),
                _ => panic!("Can't have multiple ..."),
            },
            // count is not present in WebGPU
            count: None,
        }
    }
}

impl ToCore<wgpu_types::BindingType> for webgpu::GpuBufferBindingLayout {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::BindingType {
        // https://www.w3.org/TR/webgpu/#dictdef-gpubufferbindinglayout
        wgpu_types::BindingType::Buffer {
            ty: self
                .type_
                .map(|t| t.into())
                .unwrap_or(wgpu_types::BufferBindingType::Uniform),
            has_dynamic_offset: self.has_dynamic_offset.unwrap_or(false),
            min_binding_size: self.min_binding_size.map(|x| x.try_into().unwrap()),
        }
    }
}

impl ToCore<wgpu_types::BindingType> for webgpu::GpuSamplerBindingLayout {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::BindingType {
        // https://www.w3.org/TR/webgpu/#dictdef-gpusamplerbindinglayout
        wgpu_types::BindingType::Sampler(
            self.type_
                .map(|t| t.into())
                .unwrap_or(wgpu_types::SamplerBindingType::Filtering),
        )
    }
}

impl ToCore<wgpu_types::BindingType> for webgpu::GpuTextureBindingLayout {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::BindingType {
        // https://www.w3.org/TR/webgpu/#enumdef-gputexturesampletype
        wgpu_types::BindingType::Texture {
            sample_type: self
                .sample_type
                .map(|st| st.into())
                .unwrap_or(wgpu_types::TextureSampleType::Float { filterable: true }),
            view_dimension: self
                .view_dimension
                .unwrap_or(webgpu::GpuTextureViewDimension::D2)
                .into(),
            multisampled: self.multisampled.unwrap_or(false),
        }
    }
}

impl ToCore<wgpu_types::BindingType> for webgpu::GpuStorageTextureBindingLayout {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::BindingType {
        todo!()
    }
}

// see begin_render_pass
// impl<'a> ToCore<wgpu_core::command::RenderPassDescriptor<'a>> for webgpu::GpuRenderPassDescriptor {
//     fn to_core(self, table: &ResourceTable) -> wgpu_core::command::RenderPassDescriptor<'a> {
//         wgpu_core::command::RenderPassDescriptor {
//             label: self.label.map(|l| l.into()),
//             color_attachments: self
//                 .color_attachments
//                 .into_iter()
//                 .map(|c| Some(c.to_core(table)))
//                 .collect::<Vec<_>>()
//                 .into(),
//             // depth_stencil_attachment: self.depth_stencil_attachment.map(|d| d.to_core(table)),
//             // timestamp_writes: self.timestamp_writes,
//             // occlusion_query_set: self.occlusion_query_set,
//             // TODO: self.max_draw_count not used
//         }
//     }
// }

impl ToCore<wgpu_core::command::RenderPassDepthStencilAttachment>
    for webgpu::GpuRenderPassDepthStencilAttachment
{
    fn to_core(
        self,
        table: &ResourceTable,
    ) -> wgpu_core::command::RenderPassDepthStencilAttachment {
        // https://www.w3.org/TR/webgpu/#dictdef-gpurenderpassdepthstencilattachment
        wgpu_core::command::RenderPassDepthStencilAttachment {
            view: self.view.to_core(table),
            depth: pass_channel_from_options(
                self.depth_load_op.map(|x| x.into()),
                self.depth_store_op.map(|x| x.into()),
                self.depth_clear_value.map(|x| x.into()),
                self.depth_read_only.unwrap_or(false),
            ),
            stencil: pass_channel_from_options(
                self.stencil_load_op.map(|x| x.into()),
                self.stencil_store_op.map(|x| x.into()),
                self.stencil_clear_value.map(|x| x.into()),
                self.stencil_read_only.unwrap_or(false),
            ),
        }
    }
}

fn pass_channel_from_options<V: Default + Copy>(
    load_op: Option<wgpu_core::command::LoadOp>,
    store_op: Option<wgpu_core::command::StoreOp>,
    clear_value: Option<V>,
    read_only: bool,
) -> wgpu_core::command::PassChannel<V> {
    match (load_op, store_op, clear_value) {
        (Some(load_op), Some(store_op), Some(clear_value)) => wgpu_core::command::PassChannel {
            load_op,
            store_op,
            clear_value,
            read_only,
        },
        (Some(load_op), Some(store_op), None) => wgpu_core::command::PassChannel {
            load_op,
            store_op,
            clear_value: V::default(),
            read_only,
        },
        (None, None, None) => wgpu_core::command::PassChannel {
            load_op: wgpu_core::command::LoadOp::Load,
            store_op: wgpu_core::command::StoreOp::Store,
            clear_value: V::default(),
            read_only,
        },
        _ => todo!(),
    }
}

impl ToCore<wgpu_core::command::RenderPassColorAttachment>
    for webgpu::GpuRenderPassColorAttachment
{
    fn to_core(self, table: &ResourceTable) -> wgpu_core::command::RenderPassColorAttachment {
        wgpu_core::command::RenderPassColorAttachment {
            view: self.view.to_core(table),
            resolve_target: self
                .resolve_target
                .map(|resolve_target| resolve_target.to_core(table)),
            channel: pass_channel_from_options(
                Some(self.load_op.into()),
                Some(self.store_op.into()),
                self.clear_value.map(|c| c.into()),
                // TODO: why default to false?
                false,
            ),
            // TODO: didn't use self.depth_slice
        }
    }
}

impl<'a> ToCore<wgpu_types::BufferDescriptor<wgpu_core::Label<'a>>>
    for webgpu::GpuBufferDescriptor
{
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::BufferDescriptor<wgpu_core::Label<'a>> {
        // https://www.w3.org/TR/webgpu/#gpubufferdescriptor
        wgpu_types::BufferDescriptor {
            label: self.label.map(|l| l.into()),
            size: self.size,
            usage: wgpu_types::BufferUsages::from_bits(self.usage).unwrap(),
            mapped_at_creation: self.mapped_at_creation.unwrap_or(false),
        }
    }
}

impl<'a> ToCore<wgpu_types::DeviceDescriptor<wgpu_core::Label<'a>>>
    for webgpu::GpuDeviceDescriptor
{
    fn to_core(self, table: &ResourceTable) -> wgpu_types::DeviceDescriptor<wgpu_core::Label<'a>> {
        wgpu_types::DeviceDescriptor {
            label: self.label.map(|l| l.into()),
            required_features: self
                .required_features
                .map(|f| f.to_core(table))
                .unwrap_or_default(),
            // TODO: take from self.required_limits once it's a record type in wit
            required_limits: Default::default(),
            // TODO: use self.default_queue?
            // memory_hints is not present in WebGPU
            memory_hints: wgpu_types::MemoryHints::default(),
        }
    }
}

impl<'a> ToCore<wgpu_types::Features> for Vec<webgpu::GpuFeatureName> {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::Features {
        self.into_iter()
            .map(|feature| match feature {
                webgpu::GpuFeatureName::DepthClipControl => {
                    wgpu_types::Features::DEPTH_CLIP_CONTROL
                }
                webgpu::GpuFeatureName::Depth32floatStencil8 => {
                    wgpu_types::Features::DEPTH32FLOAT_STENCIL8
                }
                webgpu::GpuFeatureName::TextureCompressionBc => {
                    wgpu_types::Features::TEXTURE_COMPRESSION_BC
                }
                webgpu::GpuFeatureName::TextureCompressionBcSliced3d => todo!(), // wgpu_types::Features::TEXTURE_COMPRESSION_BC_SLICED_3D,
                webgpu::GpuFeatureName::TextureCompressionEtc2 => {
                    wgpu_types::Features::TEXTURE_COMPRESSION_ETC2
                }
                webgpu::GpuFeatureName::TextureCompressionAstc => {
                    wgpu_types::Features::TEXTURE_COMPRESSION_ASTC
                }
                webgpu::GpuFeatureName::TextureCompressionAstcSliced3d => todo!(),
                webgpu::GpuFeatureName::TimestampQuery => wgpu_types::Features::TIMESTAMP_QUERY,
                webgpu::GpuFeatureName::IndirectFirstInstance => {
                    wgpu_types::Features::INDIRECT_FIRST_INSTANCE
                }
                webgpu::GpuFeatureName::ShaderF16 => wgpu_types::Features::SHADER_F16,
                webgpu::GpuFeatureName::Rg11b10ufloatRenderable => {
                    wgpu_types::Features::RG11B10UFLOAT_RENDERABLE
                }
                webgpu::GpuFeatureName::Bgra8unormStorage => {
                    wgpu_types::Features::BGRA8UNORM_STORAGE
                }
                webgpu::GpuFeatureName::Float32Filterable => {
                    wgpu_types::Features::FLOAT32_FILTERABLE
                }
                webgpu::GpuFeatureName::Float32Blendable => todo!(),
                webgpu::GpuFeatureName::ClipDistances => todo!(), // wgpu_types::Features::CLIP_DISTANCES,
                webgpu::GpuFeatureName::DualSourceBlending => {
                    wgpu_types::Features::DUAL_SOURCE_BLENDING
                }
                webgpu::GpuFeatureName::Subgroups => todo!(),
            })
            .collect()
    }
}

// impl<'a> ToCore<wgpu_types::Limits> for Vec<webgpu::RecordGpuSize64> {
//     fn to_core(self, _table: &ResourceTable) -> wgpu_types::Limits {
//         let defaults = wgpu_types::Limits::default();
//         wgpu_types::Limits {
//             max_texture_dimension_1d: (),
//             max_texture_dimension_2d: (),
//             max_texture_dimension_3d: (),
//             max_texture_array_layers: (),
//             max_bind_groups: (),
//             max_bindings_per_bind_group: (),
//             max_dynamic_uniform_buffers_per_pipeline_layout: (),
//             max_dynamic_storage_buffers_per_pipeline_layout: (),
//             max_sampled_textures_per_shader_stage: (),
//             max_samplers_per_shader_stage: (),
//             max_storage_buffers_per_shader_stage: (),
//             max_storage_textures_per_shader_stage: (),
//             max_uniform_buffers_per_shader_stage: (),
//             max_uniform_buffer_binding_size: (),
//             max_storage_buffer_binding_size: (),
//             max_vertex_buffers: (),
//             max_buffer_size: (),
//             max_vertex_attributes: (),
//             max_vertex_buffer_array_stride: (),
//             min_uniform_buffer_offset_alignment: (),
//             min_storage_buffer_offset_alignment: (),
//             max_inter_stage_shader_components: (),
//             max_color_attachments: (),
//             max_color_attachment_bytes_per_sample: (),
//             max_compute_workgroup_storage_size: (),
//             max_compute_invocations_per_workgroup: (),
//             max_compute_workgroup_size_x: (),
//             max_compute_workgroup_size_y: (),
//             max_compute_workgroup_size_z: (),
//             max_compute_workgroups_per_dimension: (),
//             min_subgroup_size: (),
//             max_subgroup_size: (),
//             max_push_constant_size: (),
//             max_non_sampler_bindings: (),
//         }
//     }
// }

impl ToCore<wgpu_types::ImageCopyTexture<wgpu_core::id::TextureId>>
    for webgpu::GpuTexelCopyTextureInfo
{
    fn to_core(
        self,
        table: &ResourceTable,
    ) -> wgpu_types::ImageCopyTexture<wgpu_core::id::TextureId> {
        // https://www.w3.org/TR/webgpu/#gputexelcopytextureinfo
        wgpu_types::ImageCopyTexture {
            texture: self.texture.to_core(table),
            mip_level: self.mip_level.unwrap_or(0),
            origin: self.origin.map(|o| o.to_core(table)).unwrap_or_default(),
            aspect: self
                .aspect
                .map(|a| a.into())
                .unwrap_or(wgpu_types::TextureAspect::All),
        }
    }
}

impl ToCore<wgpu_types::Origin3d> for webgpu::GpuOrigin3D {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::Origin3d {
        // https://www.w3.org/TR/webgpu/#dictdef-gpuorigin3ddict
        wgpu_types::Origin3d {
            x: self.x.unwrap_or(0),
            y: self.y.unwrap_or(0),
            z: self.z.unwrap_or(0),
        }
    }
}

impl ToCore<wgpu_types::ImageDataLayout> for webgpu::GpuTexelCopyBufferLayout {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::ImageDataLayout {
        // https://www.w3.org/TR/webgpu/#gputexelcopybufferlayout
        wgpu_types::ImageDataLayout {
            offset: self.offset.unwrap_or(0),
            bytes_per_row: self.bytes_per_row,
            rows_per_image: self.rows_per_image,
        }
    }
}

impl<'a> ToCore<wgpu_core::pipeline::ComputePipelineDescriptor<'a>>
    for webgpu::GpuComputePipelineDescriptor
{
    fn to_core(self, table: &ResourceTable) -> wgpu_core::pipeline::ComputePipelineDescriptor<'a> {
        wgpu_core::pipeline::ComputePipelineDescriptor {
            label: self.label.map(|l| l.into()),
            layout: match self.layout {
                webgpu::GpuLayoutMode::Specific(layout) => Some(layout.to_core(table)),
                webgpu::GpuLayoutMode::Auto => None,
            },
            stage: self.compute.to_core(table),
            cache: None,
        }
    }
}

impl<'a> ToCore<wgpu_core::pipeline::ProgrammableStageDescriptor<'a>>
    for webgpu::GpuProgrammableStage
{
    fn to_core(
        self,
        table: &ResourceTable,
    ) -> wgpu_core::pipeline::ProgrammableStageDescriptor<'a> {
        wgpu_core::pipeline::ProgrammableStageDescriptor {
            module: self.module.to_core(table),
            entry_point: self.entry_point.map(|ep| ep.into()),
            constants: self
                .constants
                .map(|constants| {
                    // TODO: can we get rid of the clone here?
                    let constants = table.get(&constants).unwrap().clone();
                    Cow::Owned(constants)
                })
                .unwrap_or_default(),
            zero_initialize_workgroup_memory: true,
            vertex_pulling_transform: false,
        }
    }
}

impl ToCore<wgpu_core::command::PassTimestampWrites> for webgpu::GpuComputePassTimestampWrites {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::command::PassTimestampWrites {
        wgpu_core::command::PassTimestampWrites {
            query_set: self.query_set.to_core(table),
            beginning_of_pass_write_index: self.beginning_of_pass_write_index,
            end_of_pass_write_index: self.end_of_pass_write_index,
        }
    }
}

impl ToCore<wgpu_core::command::PassTimestampWrites> for webgpu::GpuRenderPassTimestampWrites {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::command::PassTimestampWrites {
        wgpu_core::command::PassTimestampWrites {
            query_set: self.query_set.to_core(table),
            beginning_of_pass_write_index: self.beginning_of_pass_write_index,
            end_of_pass_write_index: self.end_of_pass_write_index,
        }
    }
}

impl ToCore<wgpu_types::ImageCopyBuffer<wgpu_core::id::BufferId>>
    for webgpu::GpuTexelCopyBufferInfo
{
    fn to_core(
        self,
        table: &ResourceTable,
    ) -> wgpu_types::ImageCopyBuffer<wgpu_core::id::BufferId> {
        // https://www.w3.org/TR/webgpu/#gputexelcopybufferlayout
        wgpu_types::ImageCopyBuffer {
            buffer: table.get(&self.buffer).unwrap().buffer_id,
            layout: wgpu_types::ImageDataLayout {
                offset: self.offset.unwrap_or(0),
                bytes_per_row: self.bytes_per_row,
                rows_per_image: self.rows_per_image,
            },
        }
    }
}

impl<'a> ToCore<wgpu_core::command::RenderBundleEncoderDescriptor<'a>>
    for webgpu::GpuRenderBundleEncoderDescriptor
{
    fn to_core(
        self,
        _table: &ResourceTable,
    ) -> wgpu_core::command::RenderBundleEncoderDescriptor<'a> {
        wgpu_core::command::RenderBundleEncoderDescriptor {
            label: self.label.map(|l| l.into()),
            color_formats: self
                .color_formats
                .iter()
                .map(|f| f.map(|f| f.into()))
                .collect::<Vec<_>>()
                .into(),
            depth_stencil: self.depth_stencil_format.map(|f| {
                wgpu_types::RenderBundleDepthStencil {
                    format: f.into(),
                    depth_read_only: self.depth_read_only.unwrap_or(false),
                    stencil_read_only: self.stencil_read_only.unwrap_or(false),
                }
            }),
            sample_count: self.sample_count.unwrap_or(1),
            multiview: None,
        }
    }
}

impl<'a> ToCore<wgpu_types::QuerySetDescriptor<wgpu_core::Label<'a>>>
    for webgpu::GpuQuerySetDescriptor
{
    fn to_core(
        self,
        _table: &ResourceTable,
    ) -> wgpu_types::QuerySetDescriptor<wgpu_core::Label<'a>> {
        wgpu_types::QuerySetDescriptor::<wgpu_core::Label<'a>> {
            label: self.label.map(|l| l.into()),
            ty: match self.type_ {
                webgpu::GpuQueryType::Occlusion => wgpu_types::QueryType::Occlusion,
                webgpu::GpuQueryType::Timestamp => wgpu_types::QueryType::Timestamp,
                // TODO: Why is PipelineStatistics missing?
            },
            count: self.count,
        }
    }
}

impl<'a> ToCore<wgpu_core::command::RenderBundleDescriptor<'a>>
    for webgpu::GpuRenderBundleDescriptor
{
    fn to_core(self, _table: &ResourceTable) -> wgpu_core::command::RenderBundleDescriptor<'a> {
        wgpu_core::command::RenderBundleDescriptor {
            label: self.label.map(|l| l.into()),
        }
    }
}
