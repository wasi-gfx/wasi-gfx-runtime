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

use component::webgpu::{webgpu, request_animation_frame, pointer_events};

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
"#;

fn draw_triangle() {
    let adapter = webgpu::request_adapter();
    let device = adapter.request_device();

    let displayable_entity = webgpu::get_displayable_entity(adapter.handle(), device.handle());

    let frame = request_animation_frame::get_frame();
    let pollable_a = frame.subscribe();
    let pointer_up = pointer_events::up();
    let pollable_b = pointer_up.subscribe();
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
                entry_point: "fs_main".to_string(),
                targets: vec![webgpu::GpuTextureFormat::Bgra8UnormSrgb],
            },
            primitive: webgpu::GpuPrimitiveState {
                topology: webgpu::GpuPrimitiveTopology::PointList,
            },
        });

        let g = wasi::io::poll::poll(&[
            &pollable_b,
            &pollable_a,
        ]);
        print(&format!("{:?}", g));
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
