wit_bindgen::generate!({
    path: "../../../wit",
    world: "example:example/example",
    generate_all,
});

export!(ExampleTriangle);

struct ExampleTriangle;

impl Guest for ExampleTriangle {
    fn start() {
        draw_triangle();
    }
}

use wasi::{graphics_context::graphics_context, surface::surface, webgpu::webgpu};

const SHADER_CODE: &str = r#"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
@fragment
fn fs_green() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 1.0, 0.0, 1.0);
}
"#;

fn draw_triangle() {
    let gpu = webgpu::get_gpu();
    let adapter = gpu.request_adapter(None).unwrap();
    let device = adapter.request_device(None).unwrap();

    let canvas = surface::Surface::new(surface::CreateDesc {
        height: None,
        width: None,
    });
    let graphics_context = graphics_context::Context::new();
    canvas.connect_graphics_context(&graphics_context);
    device.connect_graphics_context(&graphics_context);

    let pointer_up_pollable = canvas.subscribe_pointer_up();
    let pointer_down_pollable = canvas.subscribe_pointer_down();
    let pointer_move_pollable = canvas.subscribe_pointer_move();
    let key_up_pollable = canvas.subscribe_key_up();
    let key_down_pollable = canvas.subscribe_key_down();
    let resize_pollable = canvas.subscribe_resize();
    let frame_pollable = canvas.subscribe_frame();
    let pollables = vec![
        &pointer_up_pollable,
        &pointer_down_pollable,
        &pointer_move_pollable,
        &key_up_pollable,
        &key_down_pollable,
        &resize_pollable,
        &frame_pollable,
    ];
    let mut green = false;
    loop {
        let vertex = webgpu::GpuVertexState {
            module: &device.create_shader_module(&webgpu::GpuShaderModuleDescriptor {
                code: SHADER_CODE.to_string(),
                label: None,
                compilation_hints: None,
            }),
            entry_point: Some("vs_main".to_string()),
            buffers: None,
            constants: None,
        };
        let fragment = webgpu::GpuFragmentState {
            module: &device.create_shader_module(&webgpu::GpuShaderModuleDescriptor {
                code: SHADER_CODE.to_string(),
                label: None,
                compilation_hints: None,
            }),
            entry_point: Some(
                {
                    if green {
                        "fs_green"
                    } else {
                        "fs_main"
                    }
                }
                .to_string(),
            ),
            targets: vec![Some(webgpu::GpuColorTargetState {
                format: gpu.get_preferred_canvas_format(),
                blend: None,
                write_mask: None,
            })],
            constants: None,
        };
        let pipeline_layout = device.create_pipeline_layout(&webgpu::GpuPipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: vec![],
        });
        let pipeline_description = webgpu::GpuRenderPipelineDescriptor {
            label: None,
            vertex,
            fragment: Some(fragment),
            primitive: Some(webgpu::GpuPrimitiveState {
                topology: Some(webgpu::GpuPrimitiveTopology::TriangleList),
                strip_index_format: None,
                front_face: None,
                cull_mode: None,
                unclipped_depth: None,
            }),
            depth_stencil: None,
            multisample: None,
            layout: webgpu::GpuLayoutMode::Specific(&pipeline_layout),
        };
        let render_pipeline = device.create_render_pipeline(pipeline_description);
        let pollables_res = wasi::io::poll::poll(&pollables);

        if pollables_res.contains(&0) {
            let event = canvas.get_pointer_up();
            print(&format!("pointer_up: {:?}", event));
            green = !green;
        }
        if pollables_res.contains(&1) {
            let event = canvas.get_pointer_down();
            print(&format!("pointer_down: {:?}", event));
        }
        if pollables_res.contains(&2) {
            let event = canvas.get_pointer_move();
            print(&format!("pointer_move: {:?}", event));
        }
        if pollables_res.contains(&3) {
            let event = canvas.get_key_up();
            print(&format!("key_up: {:?}", event));
        }
        if pollables_res.contains(&4) {
            let event = canvas.get_key_down();
            print(&format!("key_down: {:?}", event));
        }
        if pollables_res.contains(&5) {
            let event = canvas.get_resize();
            print(&format!("resize: {:?}", event));
        }

        if pollables_res.contains(&6) {
            canvas.get_frame();
            print(&format!("frame event"));

            let graphics_buffer = graphics_context.get_current_buffer();
            let texture = webgpu::GpuTexture::from_graphics_buffer(graphics_buffer);
            let view = texture.create_view(None);
            let encoder = device.create_command_encoder(None);

            {
                let render_pass_description = webgpu::GpuRenderPassDescriptor {
                    label: Some(String::from("fdsa")),
                    color_attachments: vec![Some(webgpu::GpuRenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        clear_value: Some(webgpu::GpuColor {
                            r: 0.0,
                            g: 0.0,
                            b: 0.1,
                            a: 0.0,
                        }),
                        load_op: webgpu::GpuLoadOp::Clear,
                        store_op: webgpu::GpuStoreOp::Store,
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    max_draw_count: None,
                };
                let render_pass = encoder.begin_render_pass(&render_pass_description);

                render_pass.set_pipeline(&render_pipeline);
                render_pass.draw(3, None, None, None);
                render_pass.end();
            }

            device.queue().submit(&[&encoder.finish(None)]);
            graphics_context.present();
        }
    }
}
