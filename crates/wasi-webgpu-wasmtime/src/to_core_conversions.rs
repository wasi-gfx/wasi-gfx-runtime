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

impl ToCore<wgpu_types::Extent3d> for webgpu::GpuExtent3D {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::Extent3d {
        wgpu_types::Extent3d {
            width: self.width,
            height: self.height.unwrap(),
            depth_or_array_layers: self.depth_or_array_layers.unwrap(),
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
        wgpu_core::binding_model::BufferBinding {
            buffer_id: buffer.buffer,
            offset: self.offset.unwrap(),
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
            // TODO: Don't default
            range: Default::default(),
            // range: wgpu_types::ImageSubresourceRange {
            //     aspect: self.aspect,
            //     base_mip_level: self.base_mip_level,
            //     mip_level_count: self.mip_level_count,
            //     base_array_layer: self.base_array_layer,
            //     array_layer_count: self.array_layer_count,
            // }
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
                .map(|bind_group_layout| *table.get(&bind_group_layout).unwrap())
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
                webgpu::GpuLayout::GpuPipelineLayout(layout) => Some(layout.to_core(table)),
                webgpu::GpuLayout::GpuAutoLayoutMode(mode) => match mode {
                    webgpu::GpuAutoLayoutMode::Auto => None,
                },
            },
            vertex: self.vertex.to_core(table),
            primitive: self.primitive.map(|p| p.to_core(table)).unwrap(),
            depth_stencil: self.depth_stencil.map(|ds| ds.to_core(table)),
            multisample: self
                .multisample
                .map(|ms| ms.to_core(table))
                .unwrap_or_default(),
            fragment: self.fragment.map(|f| f.to_core(table)),
            // TODO: remove default
            multiview: Default::default(),
            cache: None,
        }
    }
}

impl ToCore<wgpu_types::MultisampleState> for webgpu::GpuMultisampleState {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::MultisampleState {
        wgpu_types::MultisampleState {
            count: self.count.unwrap(),
            mask: self.mask.unwrap().into(),
            alpha_to_coverage_enabled: self.alpha_to_coverage_enabled.unwrap(),
        }
    }
}

impl ToCore<wgpu_types::DepthStencilState> for webgpu::GpuDepthStencilState {
    fn to_core(self, table: &ResourceTable) -> wgpu_types::DepthStencilState {
        wgpu_types::DepthStencilState {
            format: self.format.into(),
            depth_write_enabled: self.depth_write_enabled.unwrap().into(),
            depth_compare: self.depth_compare.unwrap().into(),
            stencil: wgpu_types::StencilState {
                front: self
                    .stencil_front
                    .map(|f| f.to_core(table))
                    .unwrap_or_default(),
                back: self
                    .stencil_back
                    .map(|b| b.to_core(table))
                    .unwrap_or_default(),
                read_mask: self.stencil_read_mask.unwrap_or_default(),
                write_mask: self.stencil_write_mask.unwrap_or_default(),
            },
            bias: wgpu_types::DepthBiasState {
                constant: self.depth_bias.unwrap_or_default(),
                slope_scale: self.depth_bias_slope_scale.unwrap_or_default(),
                clamp: self.depth_bias_clamp.unwrap_or_default(),
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
        wgpu_types::PrimitiveState {
            topology: self.topology.map(|t| t.into()).unwrap_or_default(),
            strip_index_format: self.strip_index_format.map(|f| f.into()),
            front_face: self.front_face.map(|x| x.into()).unwrap_or_default(),
            cull_mode: self.cull_mode.map(|cm| cm.into()),
            unclipped_depth: self.unclipped_depth.unwrap_or_default(),
            // TODO: remove defaults
            polygon_mode: Default::default(),
            conservative: Default::default(),
        }
    }
}

impl<'a> ToCore<wgpu_core::pipeline::FragmentState<'a>> for webgpu::GpuFragmentState {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::pipeline::FragmentState<'a> {
        wgpu_core::pipeline::FragmentState {
            stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                module: self.module.to_core(table),
                entry_point: Some(self.entry_point.unwrap_or_default().into()),
                constants: Default::default(),
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
        wgpu_types::ColorTargetState {
            format: self.format.into(),
            blend: self.blend.map(|b| b.to_core(table)),
            write_mask: self
                .write_mask
                .map(|wm| wgpu_types::ColorWrites::from_bits(wm).unwrap())
                .unwrap_or_default(),
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
            // TODO: link to spec for defaults.
            src_factor: self
                .src_factor
                .map(|x| x.into())
                .unwrap_or(wgpu_types::BlendFactor::One),
            dst_factor: self
                .dst_factor
                .map(|x| x.into())
                .unwrap_or(wgpu_types::BlendFactor::Zero),
            operation: self
                .operation
                .map(|x| x.into())
                .unwrap_or(wgpu_types::BlendOperation::Add),
        }
    }
}

impl<'a> ToCore<wgpu_core::pipeline::VertexState<'a>> for webgpu::GpuVertexState {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::pipeline::VertexState<'a> {
        wgpu_core::pipeline::VertexState {
            stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                module: self.module.to_core(table),
                entry_point: self.entry_point.map(|e| e.into()),
                constants: Default::default(),
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
        wgpu_core::pipeline::VertexBufferLayout {
            array_stride: self.array_stride,
            step_mode: self.step_mode.unwrap().into(),
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
        wgpu_types::TextureDescriptor {
            label: self.label.map(|l| l.into()),
            size: self.size.to_core(table),
            mip_level_count: self.mip_level_count.unwrap(),
            sample_count: self.sample_count.unwrap(),
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
        wgpu_core::resource::SamplerDescriptor {
            label: self.label.map(|l| l.into()),
            address_modes: [
                self.address_mode_u.unwrap().into(),
                self.address_mode_v.unwrap().into(),
                self.address_mode_w.unwrap().into(),
            ],
            mag_filter: self.mag_filter.unwrap().into(),
            min_filter: self.min_filter.unwrap().into(),
            mipmap_filter: self.mipmap_filter.unwrap().into(),
            lod_min_clamp: self.lod_min_clamp.unwrap(),
            lod_max_clamp: self.lod_max_clamp.unwrap(),
            compare: self.compare.map(|compare| compare.into()),
            // TODO: should this be coming from self.anisotropy_clamp?
            anisotropy_clamp: 1,
            border_color: Default::default(),
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
        // TODO:
        Default::default()
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
            // TODO:
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
            // TODO:
            count: Default::default(),
        }
    }
}

impl ToCore<wgpu_types::BindingType> for webgpu::GpuBufferBindingLayout {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::BindingType {
        wgpu_types::BindingType::Buffer {
            ty: self.type_.map(|t| t.into()).unwrap_or_default(),
            has_dynamic_offset: self.has_dynamic_offset.unwrap_or_default(),
            min_binding_size: self.min_binding_size.map(|x| x.try_into().unwrap()),
        }
    }
}

impl ToCore<wgpu_types::BindingType> for webgpu::GpuSamplerBindingLayout {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::BindingType {
        wgpu_types::BindingType::Sampler(self.type_.map(|t| t.into()).unwrap())
    }
}

impl ToCore<wgpu_types::BindingType> for webgpu::GpuTextureBindingLayout {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::BindingType {
        wgpu_types::BindingType::Texture {
            sample_type: self.sample_type.unwrap().into(),
            view_dimension: self
                .view_dimension
                .unwrap_or(webgpu::GpuTextureViewDimension::D2)
                .into(),
            multisampled: self.multisampled.unwrap_or_default(),
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
//             ..Default::default()
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
        wgpu_core::command::RenderPassDepthStencilAttachment {
            view: self.view.to_core(table),
            depth: pass_channel_from_options(
                self.depth_load_op.map(|x| x.into()),
                self.depth_store_op.map(|x| x.into()),
                self.depth_clear_value.map(|x| x.into()),
                self.depth_read_only.map(|x| x.into()),
            ),
            stencil: pass_channel_from_options(
                self.stencil_load_op.map(|x| x.into()),
                self.stencil_store_op.map(|x| x.into()),
                self.stencil_clear_value.map(|x| x.into()),
                self.stencil_read_only.map(|x| x.into()),
            ),
        }
    }
}

fn pass_channel_from_options<V: Default + Copy>(
    load_op: Option<wgpu_core::command::LoadOp>,
    store_op: Option<wgpu_core::command::StoreOp>,
    clear_value: Option<V>,
    read_only: Option<bool>,
) -> wgpu_core::command::PassChannel<V> {
    match (load_op, store_op, clear_value) {
        (Some(load_op), Some(store_op), Some(clear_value)) => wgpu_core::command::PassChannel {
            load_op,
            store_op,
            clear_value,
            // TODO: why default to false?
            read_only: read_only.unwrap_or(false),
        },
        (Some(load_op), Some(store_op), None) => wgpu_core::command::PassChannel {
            load_op,
            store_op,
            clear_value: V::default(),
            // TODO: why default to false?
            read_only: read_only.unwrap_or(false),
        },
        (None, None, None) => wgpu_core::command::PassChannel {
            load_op: wgpu_core::command::LoadOp::Load,
            store_op: wgpu_core::command::StoreOp::Store,
            clear_value: V::default(),
            // TODO: why default to false?
            read_only: read_only.unwrap_or(false),
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
                Some(false),
            ),
            // TODO: didn't use self.depth_slice
        }
    }
}

impl<'a> ToCore<wgpu_types::BufferDescriptor<wgpu_core::Label<'a>>>
    for webgpu::GpuBufferDescriptor
{
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::BufferDescriptor<wgpu_core::Label<'a>> {
        wgpu_types::BufferDescriptor {
            label: self.label.map(|l| l.into()),
            size: self.size,
            usage: wgpu_types::BufferUsages::from_bits(self.usage).unwrap(),
            mapped_at_creation: self.mapped_at_creation.unwrap_or_default(),
        }
    }
}

impl<'a> ToCore<wgpu_types::DeviceDescriptor<wgpu_core::Label<'a>>>
    for webgpu::GpuDeviceDescriptor
{
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::DeviceDescriptor<wgpu_core::Label<'a>> {
        wgpu_types::DeviceDescriptor {
            label: self.label.map(|l| l.into()),
            // TODO: Don't default
            ..Default::default()
        }
    }
}

impl ToCore<wgpu_types::ImageCopyTexture<wgpu_core::id::TextureId>>
    for webgpu::GpuImageCopyTexture
{
    fn to_core(
        self,
        table: &ResourceTable,
    ) -> wgpu_types::ImageCopyTexture<wgpu_core::id::TextureId> {
        wgpu_types::ImageCopyTexture {
            texture: self.texture.to_core(table),
            mip_level: self.mip_level.unwrap(),
            origin: self.origin.unwrap().to_core(table),
            aspect: self.aspect.unwrap().into(),
        }
    }
}

impl ToCore<wgpu_types::Origin3d> for webgpu::GpuOrigin3D {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::Origin3d {
        wgpu_types::Origin3d {
            x: self.x.unwrap(),
            y: self.y.unwrap(),
            z: self.z.unwrap(),
        }
    }
}

impl ToCore<wgpu_types::ImageDataLayout> for webgpu::GpuImageDataLayout {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::ImageDataLayout {
        wgpu_types::ImageDataLayout {
            offset: self.offset.unwrap(),
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
                webgpu::GpuLayout::GpuPipelineLayout(layout) => Some(layout.to_core(table)),
                webgpu::GpuLayout::GpuAutoLayoutMode(mode) => match mode {
                    webgpu::GpuAutoLayoutMode::Auto => None,
                },
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
            constants: Default::default(),
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

impl ToCore<wgpu_types::ImageCopyBuffer<wgpu_core::id::BufferId>> for webgpu::GpuImageCopyBuffer {
    fn to_core(
        self,
        table: &ResourceTable,
    ) -> wgpu_types::ImageCopyBuffer<wgpu_core::id::BufferId> {
        wgpu_types::ImageCopyBuffer {
            buffer: table.get(&self.buffer).unwrap().buffer,
            layout: wgpu_types::ImageDataLayout {
                offset: self.offset.unwrap(),
                bytes_per_row: self.bytes_per_row,
                rows_per_image: self.rows_per_image,
            },
        }
    }
}


impl<'a> ToCore<wgpu_types::QuerySetDescriptor<wgpu_core::Label<'a>>> for webgpu::GpuQuerySetDescriptor {
    fn to_core(
        self,
        _table: &ResourceTable,
    ) -> wgpu_types::QuerySetDescriptor<wgpu_core::Label<'a>> {
        wgpu_types::QuerySetDescriptor::<wgpu_core::Label<'a>> {
            label: self.label.map(|l| l.into()),
            ty: match self.type_ {
                webgpu::GpuQueryType::Occlusion => wgpu_types::QueryType::Occlusion,
                webgpu::GpuQueryType::Timestamp => wgpu_types::QueryType::Timestamp,
                // Why is not PipelineStatistics in here?
            },
            count: self.count,
        }
    }
}
