wit_bindgen::generate!({
    path: "../../wit",
    world: "component:webgpu/example",
    exports: {
        world: ExampleTriangle,
    },
});

struct ExampleTriangle;

impl Guest for ExampleTriangle {
    fn start() {
        draw_triangle();
    }
}

use component::webgpu::{
    animation_frame, graphics_context, key_events, mini_canvas, pointer_events, webgpu,
};

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
    let adapter = gpu.request_adapter(None);
    let device = adapter.request_device(None);

    let canvas = mini_canvas::MiniCanvas::new(mini_canvas::CreateDesc {
        height: 100,
        width: 100,
        offscreen: false,
    });
    let graphics_context = graphics_context::GraphicsContext::new();
    canvas.connect_graphics_context(&graphics_context);
    device.connect_graphics_context(&graphics_context);

    let pointer_up_listener = pointer_events::up_listener();
    let pointer_up_pollable = pointer_up_listener.subscribe();
    let pointer_down_listener = pointer_events::down_listener();
    let pointer_down_pollable = pointer_down_listener.subscribe();
    let pointer_move_listener = pointer_events::move_listener();
    let pointer_move_pollable = pointer_move_listener.subscribe();
    let key_up_listener = key_events::up_listener();
    let key_up_pollable = key_up_listener.subscribe();
    let key_down_listener = key_events::down_listener();
    let key_down_pollable = key_down_listener.subscribe();
    let resize_listener = canvas.resize_listener();
    let resize_pollable = resize_listener.subscribe();
    let frame_listener = animation_frame::listener();
    let frame_pollable = frame_listener.subscribe();
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
            module: device.create_shader_module(webgpu::GpuShaderModuleDescriptor {
                code: SHADER_CODE.to_string(),
                label: None,
                compilation_hints: None,
            }),
            entry_point: "vs_main".to_string(),
            buffers: None,
        };
        let fragment = webgpu::GpuFragmentState {
            module: device.create_shader_module(webgpu::GpuShaderModuleDescriptor {
                code: SHADER_CODE.to_string(),
                label: None,
                compilation_hints: None,
            }),
            entry_point: {
                if green {
                    "fs_green"
                } else {
                    "fs_main"
                }
            }
            .to_string(),
            // targets: vec![webgpu::GpuColorTargetState {
            //     format: webgpu::GpuTextureFormat::Bgra8unormSrgb,
            //     blend: None,
            //     write_mask: None,
            // }],
        };
        let pipeline_description = webgpu::GpuRenderPipelineDescriptor {
            vertex,
            fragment: Some(fragment),
            primitive: Some(webgpu::GpuPrimitiveState {
                topology: Some(webgpu::GpuPrimitiveTopology::PointList),
                strip_index_format: None,
                front_face: None,
                cull_mode: None,
                unclipped_depth: None,
            }),
            depth_stencil: None,
            multisample: None,
        };
        let render_pipeline = device.create_render_pipeline(pipeline_description);
        // let render_pipeline = device.create_render_pipeline();
        let pollables_res = wasi::io::poll::poll(&pollables);

        if pollables_res.contains(&0) {
            let event = pointer_up_listener.get();
            print(&format!("pointer_up: {:?}", event));
            green = !green;
        }
        if pollables_res.contains(&1) {
            let event = pointer_down_listener.get();
            print(&format!("pointer_down: {:?}", event));
        }
        if pollables_res.contains(&2) {
            let event = pointer_move_listener.get();
            print(&format!("pointer_move: {:?}", event));
        }
        if pollables_res.contains(&3) {
            let event = key_up_listener.get();
            print(&format!("key_up: {:?}", event));
        }
        if pollables_res.contains(&4) {
            let event = key_down_listener.get();
            print(&format!("key_down: {:?}", event));
        }
        if pollables_res.contains(&5) {
            let event = resize_listener.get();
            print(&format!("resize: {:?}", event));
        }

        if pollables_res.contains(&6) {
            frame_listener.get();
            print(&format!("frame event"));

            let graphics_buffer = graphics_context.get_current_buffer();
            let texture = webgpu::GpuTexture::from_graphics_buffer(graphics_buffer);
            let view = texture.create_view(None);
            let encoder = device.create_command_encoder(None);
            let render_pass_description = webgpu::GpuRenderPassDescriptor {
                label: Some(String::from("fdsa")),
                color_attachments: vec![webgpu::GpuRenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    clear_value: None,
                    load_op: webgpu::GpuLoadOp::Load,
                    store_op: webgpu::GpuStoreOp::Store,
                }],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                max_draw_count: None,
            };
            let render_pass = encoder.begin_render_pass(render_pass_description);

            render_pass.set_pipeline(render_pipeline);
            render_pass.draw(3, 1, 0, 0);
            webgpu::GpuRenderPassEncoder::end(render_pass, &encoder);

            device
                .queue()
                .submit(vec![webgpu::GpuCommandEncoder::finish(encoder, None)]);
            graphics_context.present();
        }
    }
}
