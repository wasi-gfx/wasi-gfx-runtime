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

use component::webgpu::{pointer_events, request_animation_frame, webgpu};

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

    let displayable_entity = webgpu::get_displayable_entity(&adapter, &device);

    let pointer_up = pointer_events::up();
    let pointer_up_pollable = pointer_up.subscribe();
    let frame = request_animation_frame::get_frame();
    let frame_pollable = frame.subscribe();
    let pollables = vec![&pointer_up_pollable, &frame_pollable];
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
            let event = pointer_up.get();
            print(&format!("pointer_up: {:?}", event));
            green = !green;
        }

        if pollables_res.contains(&1) {
            frame.get();
            print(&format!("frame event"));
            // print(&format!("{:?}", g));
            // let pointer_up_instance = pointer_up.get();
            // let frame_instance = frame.get();

            // print(&format!("pointer_up: {:?}", pointer_up_instance));
            // print(&format!("frame: {:?}", frame_instance));
            // let g = pollable.block();

            // on frame:

            // let frame = surface
            //     .get_current_texture()
            //     .expect("Failed to acquire next swap chain texture");
            // let view = frame
            //     .texture
            //     .create_view(&wgpu::TextureViewDescriptor::default());

            let view = displayable_entity.create_view();
            // let encoder = device.create_command_encoder();
            // {
            //     print("xx");

            //     let rpass = encoder.begin_render_pass(GpuRenderPassDescriptor {
            //         label: String::from("fdsa"),
            //         color_attachments: vec![GpuColorAttachment {
            //             view,
            //         }],
            //     });
            //     print("xxx");

            //     rpass.set_pipeline(render_pipeline);
            //     rpass.draw(2);
            // }
            let encoder = device.do_all(
                webgpu::GpuRenderPassDescriptor {
                    label: String::from("fdsa"),
                    color_attachments: vec![webgpu::GpuColorAttachment { view }],
                },
                render_pipeline,
                4,
            );

            device.queue().submit(vec![encoder.finish()])
            // queue.submit(Some(encoder.finish()));
            // frame.present();
        }
    }
}
