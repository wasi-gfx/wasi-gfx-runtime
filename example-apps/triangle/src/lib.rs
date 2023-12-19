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
    let adapter = webgpu::request_adapter();
    let device = adapter.request_device();

    let canvas = mini_canvas::MiniCanvas::create(mini_canvas::CreateDesc {
        height: 100,
        width: 100,
        offscreen: false,
    });
    let graphics_context = graphics_context::GraphicsContext::create();
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
        let render_pipeline = device.create_render_pipeline(webgpu::GpuRenderPipelineDescriptor {
            vertex: webgpu::GpuVertexState {
                module: device.create_shader_module(&webgpu::GpuShaderModuleDescriptor {
                    code: SHADER_CODE.to_string(),
                    label: None,
                }),
                entry_point: "vs_main".to_string(),
            },
            fragment: webgpu::GpuFragmentState {
                module: device.create_shader_module(&webgpu::GpuShaderModuleDescriptor {
                    code: SHADER_CODE.to_string(),
                    label: None,
                }),
                entry_point: {
                    if green {
                        "fs_green"
                    } else {
                        "fs_main"
                    }
                }
                .to_string(),
                targets: vec![webgpu::GpuTextureFormat::Bgra8UnormSrgb],
            },
            primitive: webgpu::GpuPrimitiveState {
                topology: webgpu::GpuPrimitiveTopology::PointList,
            },
        });

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
            let view = texture.create_view();
            let encoder = device.create_command_encoder();
            {
                let rpass = encoder.begin_render_pass(webgpu::GpuRenderPassDescriptor {
                    label: String::from("fdsa"),
                    color_attachments: vec![webgpu::GpuColorAttachment {
                        view: &view,
                    }],
                });

                rpass.set_pipeline(render_pipeline);
                rpass.draw(3);
            }

            device
                .queue()
                .submit(vec![webgpu::GpuCommandEncoder::finish(encoder)]);
            webgpu::GpuTexture::non_standard_present(texture);
            // queue.submit(Some(encoder.finish()));
            // frame.present();
        }
    }
}
