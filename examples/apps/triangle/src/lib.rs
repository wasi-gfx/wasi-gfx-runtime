use futures::stream::StreamExt;
use std::cell::Cell;
use wasi::webgpu::webgpu;
use wasi_gfx::surface::{surface, surface_webgpu};

wit_bindgen::generate!({
    path: "../../../wit",
    world: "example:example/example",
    generate_all,
});

export!(ExampleTriangle);

struct ExampleTriangle;

impl Guest for ExampleTriangle {
    async fn start() {
        draw_triangle().await;
    }
}

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

async fn draw_triangle() {
    let gpu = webgpu::get_gpu();
    let adapter = gpu.request_adapter(None).await.unwrap();
    let device = adapter.request_device(None).await.unwrap();

    let surface = surface::Surface::new(surface::CreateDesc {
        height: None,
        width: None,
    });

    let context = surface_webgpu::Context::new(&surface);
    context.configure(&surface_webgpu::ContextConfiguration {
        device: &device,
        format: gpu.get_preferred_canvas_format(),
        usage: None,
        view_formats: None,
        color_space: None,
        tone_mapping: None,
        alpha_mode: None,
    });
    let green = Cell::new(false);

    let pointer_down_stream = surface.on_pointer_down().into_stream().for_each(|event| {
        print(&format!("pointer_down: {:?}", event));
        async {}
    });

    let pointer_up_stream = surface.on_pointer_up().into_stream().for_each(|event| {
        print(&format!("pointer_up: {:?}", event));
        green.set(!green.get());
        async {}
    });

    let pointer_move_stream = surface.on_pointer_move().into_stream().for_each(|event| {
        print(&format!("pointer_move: {:?}", event));
        async {}
    });

    let key_up_stream = surface.on_key_up().into_stream().for_each(|event| {
        print(&format!("key_up: {:?}", event));
        async {}
    });

    let key_down_stream = surface.on_key_down().into_stream().for_each(|event| {
        print(&format!("key_down: {:?}", event));
        async {}
    });

    let resize_stream = surface.on_resize().into_stream().for_each(|event| {
        print(&format!("resize: {:?}", event));
        async {}
    });

    let frame_stream = surface.on_frame().into_stream().for_each(|_event| {
        print("frame event");
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
                    if green.get() {
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
            immediate_size: None,
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

        let texture = context.get_current_texture();
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
        context.present();

        async {
            // give a chance for other events to come through
            futures_lite::future::yield_now().await
        }
    });

    futures::join!(
        pointer_down_stream,
        pointer_move_stream,
        key_up_stream,
        pointer_up_stream,
        key_down_stream,
        resize_stream,
        frame_stream,
    );
}
