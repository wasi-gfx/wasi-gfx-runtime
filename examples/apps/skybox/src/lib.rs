// This example is shamelessly stolen from https://github.com/gfx-rs/wgpu/tree/trunk/examples/src/skybox

wit_bindgen::generate!({
    path: "../../../wit",
    world: "example:example/example",
    generate_all,
});

export!(ExampleSkybox);

struct ExampleSkybox;

use wasi::{graphics_context::graphics_context, surface::surface, webgpu::webgpu};

impl Guest for ExampleSkybox {
    fn start() {
        let mut example = Example::init();
        example.render();
        let pointer_up_pollable = example.canvas.subscribe_pointer_up();
        let pointer_down_pollable = example.canvas.subscribe_pointer_down();
        let pointer_move_pollable = example.canvas.subscribe_pointer_move();
        let key_up_pollable = example.canvas.subscribe_key_up();
        let key_down_pollable = example.canvas.subscribe_key_down();
        let resize_pollable = example.canvas.subscribe_resize();
        let frame_pollable = example.canvas.subscribe_frame();
        let pollables = vec![
            &pointer_up_pollable,
            &pointer_down_pollable,
            &pointer_move_pollable,
            &key_up_pollable,
            &key_down_pollable,
            &resize_pollable,
            &frame_pollable,
        ];
        loop {
            let pollables_res = wasi::io::poll::poll(&pollables);

            if pollables_res.contains(&0) {
                let event = example.canvas.get_pointer_up();
                print(&format!("pointer_up: {:?}", event));
            }
            if pollables_res.contains(&1) {
                let event = example.canvas.get_pointer_down();
                print(&format!("pointer_down: {:?}", event));
            }
            if pollables_res.contains(&2) {
                let event = example.canvas.get_pointer_move();
                print(&format!("pointer_move: {:?}", event));
                let event = event.unwrap();
                example.update(event.x as i32, event.y as i32);
            }
            if pollables_res.contains(&3) {
                let event = example.canvas.get_key_up();
                print(&format!("key_up: {:?}", event));
            }
            if pollables_res.contains(&4) {
                let event = example.canvas.get_key_down();
                print(&format!("key_down: {:?}", event));
            }
            if pollables_res.contains(&5) {
                let event = example.canvas.get_resize();
                print(&format!("resize: {:?}", event));
                my_run();
            }

            if pollables_res.contains(&5) {
                example.canvas.get_frame();
                print(&format!("frame event"));
                example.render();
            }
        }
    }
}

mod flags;
use flags::{BufferUsages, ColorWrites, ShaderStages, TextureUsages};

use bytemuck::{Pod, Zeroable};
use std::f32::consts;

const IMAGE_SIZE: u32 = 256;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct Vertex {
    pos: [f32; 3],
    normal: [f32; 3],
}

struct Entity {
    vertex_count: u32,
    vertex_buf: MyBuffer,
}

// Note: we use the Y=up coordinate space in this example.
struct Camera {
    screen_size: (u32, u32),
    angle_y: f32,
    angle_xz: f32,
    dist: f32,
}

const MODEL_CENTER_Y: f32 = 2.0;

impl Camera {
    fn to_uniform_data(&self) -> [f32; 16 * 3 + 4] {
        let aspect = self.screen_size.0 as f32 / self.screen_size.1 as f32;
        let proj = glam::Mat4::perspective_rh(consts::FRAC_PI_4, aspect, 1.0, 50.0);
        let cam_pos = glam::Vec3::new(
            self.angle_xz.cos() * self.angle_y.sin() * self.dist,
            self.angle_xz.sin() * self.dist + MODEL_CENTER_Y,
            self.angle_xz.cos() * self.angle_y.cos() * self.dist,
        );
        let view = glam::Mat4::look_at_rh(
            cam_pos,
            glam::Vec3::new(0f32, MODEL_CENTER_Y, 0.0),
            glam::Vec3::Y,
        );
        let proj_inv = proj.inverse();

        let mut raw = [0f32; 16 * 3 + 4];
        raw[..16].copy_from_slice(&AsRef::<[f32; 16]>::as_ref(&proj)[..]);
        raw[16..32].copy_from_slice(&AsRef::<[f32; 16]>::as_ref(&proj_inv)[..]);
        raw[32..48].copy_from_slice(&AsRef::<[f32; 16]>::as_ref(&view)[..]);
        raw[48..51].copy_from_slice(AsRef::<[f32; 3]>::as_ref(&cam_pos));
        raw[51] = 1.0;
        raw
    }
}

pub struct Example {
    device: webgpu::GpuDevice,
    canvas: surface::Surface,
    graphics_context: graphics_context::Context,
    camera: Camera,
    sky_pipeline: webgpu::GpuRenderPipeline,
    entity_pipeline: webgpu::GpuRenderPipeline,
    bind_group: webgpu::GpuBindGroup,
    uniform_buf: MyBuffer,
    entities: Vec<Entity>,
    depth_view: webgpu::GpuTextureView,
}

impl Example {
    const DEPTH_FORMAT: webgpu::GpuTextureFormat = webgpu::GpuTextureFormat::Depth24plus;

    fn create_depth_texture(
        // config: &webgpu::GpuSurfaceConfiguration,
        // config: &webgpu::GpuCanvasConfiguration,
        device: &webgpu::GpuDevice,
        width: u32,
        height: u32,
    ) -> webgpu::GpuTextureView {
        let depth_texture = device.create_texture(&webgpu::GpuTextureDescriptor {
            size: webgpu::GpuExtent3D {
                width,
                height: Some(height),
                depth_or_array_layers: Some(1),
            },
            mip_level_count: Some(1),
            sample_count: Some(1),
            dimension: Some(webgpu::GpuTextureDimension::D2),
            format: Self::DEPTH_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT.bits(),
            label: None,
            view_formats: Some(vec![]),
        });

        depth_texture.create_view(Some(&webgpu::GpuTextureViewDescriptor {
            format: None,
            dimension: None,
            aspect: None,
            base_mip_level: None,
            mip_level_count: None,
            base_array_layer: None,
            array_layer_count: None,
            label: None,
            usage: None,
        }))
    }

    fn init() -> Self {
        let device = webgpu::get_gpu()
            .request_adapter(None)
            .unwrap()
            .request_device(None)
            .unwrap();
        let canvas = surface::Surface::new(surface::CreateDesc {
            height: None,
            width: None,
        });
        let graphics_context = graphics_context::Context::new();
        canvas.connect_graphics_context(&graphics_context);
        device.connect_graphics_context(&graphics_context);

        let height = canvas.height();
        let width = canvas.width();

        let mut entities = Vec::new();
        {
            let source = include_bytes!("models/teslacyberv3.0.obj");
            let data = obj::ObjData::load_buf(&source[..]).unwrap();
            let mut vertices = Vec::new();
            for object in data.objects {
                for group in object.groups {
                    vertices.clear();
                    for poly in group.polys {
                        for end_index in 2..poly.0.len() {
                            for &index in &[0, end_index - 1, end_index] {
                                let obj::IndexTuple(position_id, _texture_id, normal_id) =
                                    poly.0[index];
                                vertices.push(Vertex {
                                    pos: data.position[position_id],
                                    normal: data.normal[normal_id.unwrap()],
                                })
                            }
                        }
                    }
                    let vertex_buf = device_create_buffer_init(
                        &device,
                        &BufferInitDescriptor {
                            label: Some("Vertex"),
                            contents: bytemuck::cast_slice(&vertices),
                            usage: BufferUsages::VERTEX,
                        },
                    );
                    entities.push(Entity {
                        vertex_count: vertices.len() as u32,
                        vertex_buf,
                    });
                }
            }
        }

        let bind_group_layout =
            device.create_bind_group_layout(&webgpu::GpuBindGroupLayoutDescriptor {
                label: None,
                entries: vec![
                    webgpu::GpuBindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX.bits() | ShaderStages::FRAGMENT.bits(),
                        buffer: Some(webgpu::GpuBufferBindingLayout {
                            type_: Some(webgpu::GpuBufferBindingType::Uniform),
                            has_dynamic_offset: Some(false),
                            min_binding_size: None,
                        }),
                        sampler: None,
                        texture: None,
                        storage_texture: None,
                    },
                    webgpu::GpuBindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT.bits(),
                        buffer: None,
                        sampler: None,
                        texture: Some(webgpu::GpuTextureBindingLayout {
                            sample_type: Some(webgpu::GpuTextureSampleType::Float),
                            multisampled: Some(false),
                            view_dimension: Some(webgpu::GpuTextureViewDimension::Cube),
                        }),
                        storage_texture: None,
                    },
                    webgpu::GpuBindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::FRAGMENT.bits(),
                        buffer: None,
                        sampler: Some(webgpu::GpuSamplerBindingLayout {
                            type_: Some(webgpu::GpuSamplerBindingType::Filtering),
                        }),
                        texture: None,
                        storage_texture: None,
                    },
                ],
            });

        // Create the render pipeline
        let shader = device.create_shader_module(&webgpu::GpuShaderModuleDescriptor {
            label: None,
            code: String::from(include_str!("shader.wgsl")),
            compilation_hints: None,
        });

        let camera = Camera {
            screen_size: (width, height),
            // screen_size: (config.width, config.height),
            angle_xz: 0.2,
            angle_y: 0.2,
            dist: 20.0,
        };
        let raw_uniforms = camera.to_uniform_data();
        let uniform_buf = device_create_buffer_init(
            &device,
            &BufferInitDescriptor {
                label: Some("Buffer"),
                contents: bytemuck::cast_slice(&raw_uniforms),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            },
        );

        let pipeline_layout = device.create_pipeline_layout(&webgpu::GpuPipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: vec![Some(&bind_group_layout)],
        });

        let sky_pipeline = device.create_render_pipeline(webgpu::GpuRenderPipelineDescriptor {
            label: None,
            layout: webgpu::GpuLayoutMode::Specific(&pipeline_layout),
            vertex: webgpu::GpuVertexState {
                module: &shader,
                entry_point: Some("vs_sky".into()),
                // buffers: None,
                buffers: Some(vec![]),
                constants: None,
            },
            fragment: Some(webgpu::GpuFragmentState {
                module: &shader,
                entry_point: Some("fs_sky".into()),
                constants: None,
                targets: vec![Some(webgpu::GpuColorTargetState {
                    format: webgpu::get_gpu().get_preferred_canvas_format(),
                    blend: None,
                    write_mask: Some(ColorWrites::ALL.bits()),
                })],
            }),
            primitive: Some(webgpu::GpuPrimitiveState {
                front_face: Some(webgpu::GpuFrontFace::Cw),
                topology: None,
                strip_index_format: None,
                cull_mode: None,
                unclipped_depth: None,
            }),
            depth_stencil: Some(webgpu::GpuDepthStencilState {
                format: Self::DEPTH_FORMAT,
                depth_write_enabled: Some(false),
                depth_compare: Some(webgpu::GpuCompareFunction::LessEqual),
                stencil_front: None,
                stencil_back: None,
                stencil_read_mask: None,
                stencil_write_mask: None,
                depth_bias: None,
                depth_bias_slope_scale: None,
                depth_bias_clamp: None,
            }),
            multisample: Some(webgpu::GpuMultisampleState {
                count: Some(1),
                mask: Some(!0),
                alpha_to_coverage_enabled: Some(false),
            }),
        });

        let entity_pipeline = device.create_render_pipeline(webgpu::GpuRenderPipelineDescriptor {
            label: Some("Entity".into()),
            // layout: None,
            layout: webgpu::GpuLayoutMode::Specific(&pipeline_layout),
            vertex: webgpu::GpuVertexState {
                module: &shader,
                entry_point: Some("vs_entity".into()),
                constants: None,
                buffers: Some(vec![Some(webgpu::GpuVertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as u64,
                    step_mode: Some(webgpu::GpuVertexStepMode::Vertex),
                    // attributes: &webgpu::Gpuvertex_attr_array![0 => Float32x3, 1 => Float32x3],
                    attributes: vec![
                        webgpu::GpuVertexAttribute {
                            format: webgpu::GpuVertexFormat::Float32x3,
                            offset: 0,          // GpuSize64,
                            shader_location: 0, // GpuIndex32,
                        },
                        webgpu::GpuVertexAttribute {
                            format: webgpu::GpuVertexFormat::Float32x3,
                            offset: 12, // ::wgpu::VertexFormat::Float32x3.size(), // (32 * 3) / 8
                            shader_location: 1, // GpuIndex32,
                        },
                    ],
                })]),
            },
            fragment: Some(webgpu::GpuFragmentState {
                module: &shader,
                entry_point: Some("fs_entity".into()),
                constants: None,
                targets: vec![Some(webgpu::GpuColorTargetState {
                    format: webgpu::get_gpu().get_preferred_canvas_format(),
                    blend: None,
                    write_mask: Some(ColorWrites::ALL.bits()),
                })],
                // targets: &[Some(config.view_formats[0].into())],
            }),
            primitive: Some(webgpu::GpuPrimitiveState {
                front_face: Some(webgpu::GpuFrontFace::Cw),
                topology: None,
                strip_index_format: None,
                cull_mode: None,
                unclipped_depth: None,
            }),
            depth_stencil: Some(webgpu::GpuDepthStencilState {
                format: Self::DEPTH_FORMAT,
                // format: webgpu::GpuTextureFormat::Depth24plus,
                depth_write_enabled: Some(true),
                depth_compare: Some(webgpu::GpuCompareFunction::LessEqual),
                stencil_front: None,
                stencil_back: None,
                stencil_read_mask: None,
                stencil_write_mask: None,
                depth_bias: None,
                depth_bias_slope_scale: None,
                depth_bias_clamp: None,
            }),
            multisample: Some(webgpu::GpuMultisampleState {
                count: Some(1),
                mask: Some(!0),
                alpha_to_coverage_enabled: Some(false),
            }),
            // multiview: None,
        });

        let sampler = device.create_sampler(Some(&webgpu::GpuSamplerDescriptor {
            label: None,
            address_mode_u: Some(webgpu::GpuAddressMode::ClampToEdge),
            address_mode_v: Some(webgpu::GpuAddressMode::ClampToEdge),
            address_mode_w: Some(webgpu::GpuAddressMode::ClampToEdge),
            mag_filter: Some(webgpu::GpuFilterMode::Linear),
            min_filter: Some(webgpu::GpuFilterMode::Linear),
            mipmap_filter: Some(webgpu::GpuMipmapFilterMode::Linear),
            lod_min_clamp: Some(0.0),
            lod_max_clamp: Some(32.0),
            compare: None,
            // TODO:: this right?
            // max_anisotropy: Some(1),
            max_anisotropy: None,
        }));

        let device_features = device.features();

        let skybox_format = if device_features.has("texture-compression-astc") {
            log::info!("Using astc");
            webgpu::GpuTextureFormat::Astc4x4UnormSrgb
        } else if device_features.has("texture-compression-etc2") {
            log::info!("Using etc2");
            webgpu::GpuTextureFormat::Etc2Rgb8a1unormSrgb
        } else if device_features.has("texture-compression-bc") {
            log::info!("Using bc7");
            webgpu::GpuTextureFormat::Bc7RgbaUnormSrgb
        } else {
            log::info!("Using rgba8");
            webgpu::GpuTextureFormat::Rgba8unormSrgb
        };
        // let skybox_format = webgpu::GpuTextureFormat::Rgba8unormSrgb;

        let size = webgpu::GpuExtent3D {
            width: IMAGE_SIZE,
            height: Some(IMAGE_SIZE),
            depth_or_array_layers: Some(6),
        };

        let bytes = match skybox_format {
            webgpu::GpuTextureFormat::Astc4x4UnormSrgb => &include_bytes!("images/astc.ktx2")[..],
            webgpu::GpuTextureFormat::Etc2Rgb8a1unormSrgb => {
                &include_bytes!("images/etc2.ktx2")[..]
            }
            webgpu::GpuTextureFormat::Bc7RgbaUnormSrgb => &include_bytes!("images/bc7.ktx2")[..],
            webgpu::GpuTextureFormat::Rgba8unormSrgb => &include_bytes!("images/rgba8.ktx2")[..],
            _ => unreachable!(),
        };

        let reader = ktx2::Reader::new(bytes).unwrap();
        let header = reader.header();

        let mut image = Vec::with_capacity(reader.data().len());
        for level in reader.levels() {
            image.extend_from_slice(level);
        }

        let texture = device_create_texture_with_data(
            &device,
            &webgpu::GpuTextureDescriptor {
                size,
                mip_level_count: Some(header.level_count),
                sample_count: Some(1),
                dimension: Some(webgpu::GpuTextureDimension::D2),
                format: skybox_format,
                usage: TextureUsages::TEXTURE_BINDING.bits() | TextureUsages::COPY_DST.bits(),
                label: None,
                view_formats: vec![].into(),
            },
            // KTX2 stores mip levels in mip major order.
            // wgpu::util::TextureDataOrder::MipMajor,
            &image,
        );

        let texture_view = texture.create_view(Some(&webgpu::GpuTextureViewDescriptor {
            label: None,
            dimension: Some(webgpu::GpuTextureViewDimension::Cube),
            format: None,
            aspect: None,
            base_mip_level: None,
            mip_level_count: None,
            base_array_layer: None,
            array_layer_count: None,
            usage: None,
        }));
        let bind_group = device.create_bind_group(&webgpu::GpuBindGroupDescriptor {
            layout: &bind_group_layout,
            entries: vec![
                webgpu::GpuBindGroupEntry {
                    binding: 0,
                    // resource: uniform_buf.as_entire_binding(),
                    resource: webgpu::GpuBindingResource::GpuBufferBinding(
                        webgpu::GpuBufferBinding {
                            buffer: &uniform_buf.buffer,
                            // offset: None,
                            offset: Some(0),
                            size: Some(uniform_buf.size),
                        },
                    ),
                },
                webgpu::GpuBindGroupEntry {
                    binding: 1,
                    resource: webgpu::GpuBindingResource::GpuTextureView(&texture_view),
                },
                webgpu::GpuBindGroupEntry {
                    binding: 2,
                    resource: webgpu::GpuBindingResource::GpuSampler(&sampler),
                },
            ],
            label: None,
        });

        let depth_view = Self::create_depth_texture(&device, width, height);

        Example {
            device,
            canvas,
            graphics_context,
            camera,
            sky_pipeline,
            entity_pipeline,
            bind_group,
            uniform_buf,
            entities,
            depth_view,
        }
    }

    fn update(&mut self, x: i32, y: i32) {
        let norm_x = x as f32 / self.camera.screen_size.0 as f32 - 0.5;
        let norm_y = y as f32 / self.camera.screen_size.1 as f32 - 0.5;
        self.camera.angle_y = norm_x * 5.0;
        self.camera.angle_xz = norm_y;
    }

    fn render(&mut self) {
        let graphics_buffer = self.graphics_context.get_current_buffer();
        let texture = webgpu::GpuTexture::from_graphics_buffer(graphics_buffer);

        let view = texture.create_view(Some(&webgpu::GpuTextureViewDescriptor {
            format: None,
            dimension: None,
            aspect: None,
            base_mip_level: None,
            mip_level_count: None,
            base_array_layer: None,
            array_layer_count: None,
            label: None,
            usage: None,
        }));

        let encoder = self
            .device
            .create_command_encoder(Some(&webgpu::GpuCommandEncoderDescriptor { label: None }));

        // update rotation
        let raw_uniforms = self.camera.to_uniform_data();

        self.device
            .queue()
            .write_buffer_with_copy(
                &self.uniform_buf.buffer,
                0,
                bytemuck::cast_slice(&raw_uniforms),
                None,
                Some(raw_uniforms.len() as u64 * 4),
            )
            .unwrap();

        {
            let rpass = encoder.begin_render_pass(&webgpu::GpuRenderPassDescriptor {
                label: None,
                color_attachments: vec![Some(webgpu::GpuRenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    load_op: webgpu::GpuLoadOp::Clear,
                    store_op: webgpu::GpuStoreOp::Store,
                    depth_slice: None,
                    clear_value: Some(webgpu::GpuColor {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                })],
                depth_stencil_attachment: Some(webgpu::GpuRenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_load_op: Some(webgpu::GpuLoadOp::Clear),
                    depth_store_op: Some(webgpu::GpuStoreOp::Store),
                    depth_clear_value: Some(1.0),
                    depth_read_only: Some(false),
                    stencil_load_op: Some(webgpu::GpuLoadOp::Load),
                    stencil_store_op: Some(webgpu::GpuStoreOp::Store),
                    stencil_clear_value: Some(0),
                    stencil_read_only: Some(true),
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                max_draw_count: None,
            });

            rpass
                .set_bind_group(0, Some(&self.bind_group), None, None, None)
                .unwrap();
            rpass.set_pipeline(&self.entity_pipeline);

            for entity in self.entities.iter() {
                rpass.set_vertex_buffer(0, Some(&entity.vertex_buf.buffer), None, None);
                rpass.draw(entity.vertex_count, None, None, None);
            }

            rpass.set_pipeline(&self.sky_pipeline);
            rpass.draw(3, None, None, None);
            rpass.end();
        }

        self.device.queue().submit(&[&encoder.finish(None)]);

        self.graphics_context.present();
    }
}

fn device_create_texture_with_data(
    device: &webgpu::GpuDevice,
    desc: &webgpu::GpuTextureDescriptor,
    data: &[u8],
) -> webgpu::GpuTexture {
    // Implicitly add the COPY_DST usage
    let mut desc = desc.to_owned();
    desc.usage |= TextureUsages::COPY_DST.bits();
    // let texture = device.create_texture(&desc);
    let texture = device.create_texture(&desc);

    // Will return None only if it's a combined depth-stencil format
    // If so, default to 4, validation will fail later anyway since the depth or stencil
    // aspect needs to be written to individually
    // let block_size = desc.format.Rgba8unormSrgb(None).unwrap_or(4);
    let block_size = 4;
    // let (block_width, block_height) = desc.format.Rgba8unormSrgb();
    let (block_width, block_height) = (1, 1);
    // let layer_iterations = desc.array_layer_count();

    let layer_iterations = match desc.dimension {
        Some(webgpu::GpuTextureDimension::D1) | Some(webgpu::GpuTextureDimension::D3) => 1,
        Some(webgpu::GpuTextureDimension::D2) | None => desc.size.depth_or_array_layers.unwrap(),
    };

    let outer_iteration = desc.mip_level_count;
    let inner_iteration = layer_iterations;

    let mut binary_offset = 0;
    for outer in 0..outer_iteration.unwrap() {
        for inner in 0..inner_iteration {
            // let (layer, mip) = match order {
            //     TextureDataOrder::LayerMajor => (outer, inner),
            //     TextureDataOrder::MipMajor => (inner, outer),
            // };
            let (layer, mip) = (inner, outer);

            // let mut mip_size = desc.mip_level_size(mip).unwrap();
            // pub fn mip_level_size(&self, level: u32) -> Option<Extent3d> {
            //     if level >= self.mip_level_count {
            //         return None;
            //     }

            //     Some(self.size.mip_level_size(level, self.dimension))
            // }
            // let mut mip_size = webgpu::GpuExtent3DDict {
            //     width: desc.mip_level_count.unwrap() >> mip,
            //     height: Some(size.height.unwrap() >> mip),
            //     depth_or_array_layers: size.depth_or_array_layers,
            // };
            let mut mip_size = mip_level_size(
                desc.size,
                mip,
                desc.dimension.unwrap_or(webgpu::GpuTextureDimension::D2),
            ); // desc.mip_level_size(mip).unwrap();

            // let mut mip_size = Self {
            //     width: u32::max(1, self.width >> level),
            //     height: match dim {
            //         TextureDimension::D1 => 1,
            //         _ => u32::max(1, self.height >> level),
            //     },
            //     depth_or_array_layers: match dim {
            //         TextureDimension::D1 => 1,
            //         TextureDimension::D2 => self.depth_or_array_layers,
            //         TextureDimension::D3 => u32::max(1, self.depth_or_array_layers >> level),
            //     },
            // }

            // copying layers separately
            if desc.dimension != Some(webgpu::GpuTextureDimension::D3) {
                mip_size.depth_or_array_layers = Some(1);
            }

            // When uploading mips of compressed textures and the mip is supposed to be
            // a size that isn't a multiple of the block size, the mip needs to be uploaded
            // as its "physical size" which is the size rounded up to the nearest block size.
            let mip_physical = physical_size(&mip_size);

            // All these calculations are performed on the physical size as that's the
            // data that exists in the buffer.
            let width_blocks = mip_physical.width / block_width;
            let height_blocks = mip_physical.height.unwrap() / block_height;

            let bytes_per_row = width_blocks * block_size;
            let data_size = bytes_per_row * height_blocks * mip_size.depth_or_array_layers.unwrap();

            let end_offset = binary_offset + data_size as usize;

            device.queue().write_texture_with_copy(
                &webgpu::GpuTexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: Some(mip),
                    origin: Some(webgpu::GpuOrigin3D {
                        x: Some(0),
                        y: Some(0),
                        z: Some(layer),
                    }),
                    aspect: Some(webgpu::GpuTextureAspect::All),
                },
                &data[binary_offset..end_offset],
                webgpu::GpuTexelCopyBufferLayout {
                    offset: Some(0),
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(height_blocks),
                },
                mip_physical,
            );

            binary_offset = end_offset;
        }
    }

    texture
}

fn mip_level_size(
    extent_3d: webgpu::GpuExtent3D,
    level: u32,
    dim: webgpu::GpuTextureDimension,
) -> webgpu::GpuExtent3D {
    webgpu::GpuExtent3D {
        width: u32::max(1, extent_3d.width >> level),
        height: Some(match dim {
            webgpu::GpuTextureDimension::D1 => 1,
            _ => u32::max(1, extent_3d.height.unwrap() >> level),
        }),
        depth_or_array_layers: Some(match dim {
            webgpu::GpuTextureDimension::D1 => 1,
            webgpu::GpuTextureDimension::D2 => extent_3d.depth_or_array_layers.unwrap(),
            webgpu::GpuTextureDimension::D3 => {
                u32::max(1, extent_3d.depth_or_array_layers.unwrap() >> level)
            }
        }),
    }
}

fn physical_size(extend: &webgpu::GpuExtent3D) -> webgpu::GpuExtent3D {
    // let (block_width, block_height) = format.block_dimensions();
    let (block_width, block_height) = (1, 1);

    let width = ((extend.width + block_width - 1) / block_width) * block_width;
    let height = ((extend.height.unwrap() + block_height - 1) / block_height) * block_height;

    webgpu::GpuExtent3D {
        width,
        height: Some(height),
        depth_or_array_layers: extend.depth_or_array_layers,
    }
}

pub struct BufferInitDescriptor<'a> {
    /// Debug label of a buffer. This will show up in graphics debuggers for easy identification.
    pub label: Option<&'a str>,
    /// Contents of a buffer on creation.
    pub contents: &'a [u8],
    /// Usages of a buffer. If the buffer is used in any way that isn't specified here, the operation
    /// will panic.
    pub usage: BufferUsages,
}
struct MyBuffer {
    buffer: webgpu::GpuBuffer,
    size: u64,
}
fn device_create_buffer_init(
    device: &webgpu::GpuDevice,
    descriptor: &BufferInitDescriptor<'_>,
) -> MyBuffer {
    // Skip mapping if the buffer is zero sized
    if descriptor.contents.is_empty() {
        let buffer = device.create_buffer(&webgpu::GpuBufferDescriptor {
            label: descriptor.label.map(|l| l.into()),
            size: 0,
            usage: descriptor.usage.bits(),
            mapped_at_creation: Some(false),
        });
        MyBuffer { buffer, size: 0 }
    } else {
        const COPY_BUFFER_ALIGNMENT: u64 = 4;
        let unpadded_size = descriptor.contents.len() as u64;
        // Valid vulkan usage is
        // 1. buffer size must be a multiple of COPY_BUFFER_ALIGNMENT.
        // 2. buffer size must be greater than 0.
        // Therefore we round the value up to the nearest multiple, and ensure it's at least COPY_BUFFER_ALIGNMENT.
        let align_mask = COPY_BUFFER_ALIGNMENT - 1;
        let padded_size = ((unpadded_size + align_mask) & !align_mask).max(COPY_BUFFER_ALIGNMENT);

        let buffer = device.create_buffer(&webgpu::GpuBufferDescriptor {
            label: descriptor.label.map(|l| l.into()),
            size: padded_size,
            usage: descriptor.usage.bits(),
            mapped_at_creation: Some(true),
        });

        // let remote_buffer = buffer.get_mapped_range(None, None);
        // remote_buffer.set(descriptor.contents);
        buffer
            .get_mapped_range_set_with_copy(&descriptor.contents, None, None)
            .unwrap();

        buffer.unmap().unwrap();
        MyBuffer {
            buffer,
            size: padded_size,
        }
    }
}
