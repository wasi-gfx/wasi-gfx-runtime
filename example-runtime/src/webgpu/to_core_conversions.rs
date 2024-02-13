use wasmtime::component::ResourceTable;

use crate::component::webgpu::webgpu;

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
        match self {
            webgpu::GpuExtent3D::GpuExtent3DDict(extent_dict) => wgpu_types::Extent3d {
                width: extent_dict.width,
                height: extent_dict.height.unwrap(),
                depth_or_array_layers: extent_dict.depth_or_array_layers.unwrap(),
            },
            webgpu::GpuExtent3D::ListGpuIntegerCoordinate(_coordinates) => todo!(),
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
            webgpu::GpuBindingResource::GpuExternalTexture(_external_texture) => todo!(),
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
        wgpu_core::binding_model::BufferBinding {
            buffer_id: self.buffer.to_core(table),
            // TODO: Not sure we can default here.
            offset: self.offset.unwrap_or_default(),
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
        // TODO:
        Default::default()
        // wgpu_core::resource::TextureViewDescriptor {
        //     label: self.label.map(|l| l.into()),
        //     format: self.format.into(),
        //     dimension: self.dimension.into(),
        //     range: self.range.into(),
        // }
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
            // TODO: remove defaults
            label: Default::default(),
            layout: Default::default(),
            vertex: self.vertex.to_core(table),
            primitive: Default::default(),
            depth_stencil: Default::default(),
            multisample: Default::default(),
            fragment: self.fragment.map(|f| f.to_core(table)),
            multiview: Default::default(),
        }
    }
}

impl<'a> ToCore<wgpu_core::pipeline::FragmentState<'a>> for webgpu::GpuFragmentState {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::pipeline::FragmentState<'a> {
        wgpu_core::pipeline::FragmentState {
            stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                module: self.module.to_core(table),
                entry_point: self.entry_point.into(),
            },
            // TODO: Remove Default?
            // targets: Default::default(),
            targets: vec![Some(wgpu_types::ColorTargetState {
                format: wgpu_types::TextureFormat::Bgra8UnormSrgb,
                blend: None,
                write_mask: Default::default(),
            })]
            .into(),
        }
    }
}

impl<'a> ToCore<wgpu_core::pipeline::VertexState<'a>> for webgpu::GpuVertexState {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::pipeline::VertexState<'a> {
        wgpu_core::pipeline::VertexState {
            stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                module: self.module.to_core(table),
                entry_point: self.entry_point.into(),
            },
            // TODO: Remove Default?
            buffers: Default::default(),
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
            dimension: self.dimension.into(),
            format: self.format.into(),
            // TODO: Remove from_bits?
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
            // TODO:
            anisotropy_clamp: 1,
            // TODO:
            border_color: Default::default(),
            // border_color: descriptor.border_color,
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
                self.external_texture,
            ) {
                (Some(buffer), None, None, None, None) => buffer.to_core(table),
                (None, Some(sampler), None, None, None) => sampler.to_core(table),
                (None, None, Some(texture), None, None) => texture.to_core(table),
                (None, None, None, Some(storage_texture), None) => storage_texture.to_core(table),
                (None, None, None, None, Some(external_texture)) => external_texture.to_core(table),
                (None, None, None, None, None) => todo!(),
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
            view_dimension: self.view_dimension.into(),
            multisampled: self.multisampled.unwrap_or_default(),
        }
    }
}

impl ToCore<wgpu_types::BindingType> for webgpu::GpuStorageTextureBindingLayout {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::BindingType {
        todo!()
    }
}

impl ToCore<wgpu_types::BindingType> for webgpu::GpuExternalTextureBindingLayout {
    fn to_core(self, _table: &ResourceTable) -> wgpu_types::BindingType {
        todo!()
    }
}

impl<'a> ToCore<wgpu_core::command::RenderPassDescriptor<'a>> for webgpu::GpuRenderPassDescriptor {
    fn to_core(self, table: &ResourceTable) -> wgpu_core::command::RenderPassDescriptor<'a> {
        wgpu_core::command::RenderPassDescriptor {
            label: self.label.map(|l| l.into()),
            color_attachments: self
                .color_attachments
                .into_iter()
                .map(|c| Some(c.to_core(table)))
                .collect::<Vec<_>>()
                .into(),
            // TODO: remove default
            ..Default::default() // depth_stencil_attachment: self.depth_stencil_attachment,
                                 // timestamp_writes: self.timestamp_writes,
                                 // occlusion_query_set: self.occlusion_query_set,
                                 // TODO: self.max_draw_count not used
        }
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
            channel: wgpu_core::command::PassChannel {
                load_op: self.load_op.into(),
                store_op: self.store_op.into(),
                clear_value: self.clear_value.map(|c| c.into()).unwrap_or_default(),
                // TODO:
                read_only: false,
            },
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
            // TODO:
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
