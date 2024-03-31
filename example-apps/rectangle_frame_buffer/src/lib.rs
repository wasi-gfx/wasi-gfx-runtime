wit_bindgen::generate!({
    path: "../../wit",
    world: "example:example/example",
    exports: {
        world: Example,
    },
});

struct Example;

impl Guest for Example {
    fn start() {
        draw_rectangle();
    }
}

use wasi::webgpu::{
    animation_frame, frame_buffer, graphics_context, mini_canvas, pointer_events,
};
use std::cmp::min;

fn draw_rectangle() {
    let canvas = mini_canvas::MiniCanvas::new(mini_canvas::CreateDesc {
        height: 100,
        width: 100,
        offscreen: false,
    });
    let graphics_context = graphics_context::GraphicsContext::new();
    canvas.connect_graphics_context(&graphics_context);

    let surface = frame_buffer::Surface::new();
    surface.connect_graphics_context(&graphics_context);

    let pointer_up_listener = pointer_events::up_listener(&canvas);
    let pointer_up_pollable = pointer_up_listener.subscribe();
    let resize_listener = canvas.resize_listener();
    let resize_pollable = resize_listener.subscribe();
    let frame_listener = animation_frame::listener(&canvas);
    let frame_pollable = frame_listener.subscribe();
    let pollables = vec![&pointer_up_pollable, &resize_pollable, &frame_pollable];
    let mut green = false;
    let mut height = canvas.height();
    let mut width = canvas.width();
    loop {
        let pollables_res = wasi::io::poll::poll(&pollables);

        if pollables_res.contains(&0) {
            let event = pointer_up_listener.get();
            print(&format!("up: {:?}", event));
            green = !green;
        }

        if pollables_res.contains(&1) {
            let event = resize_listener.get().unwrap();
            print(&format!("resize: {:?}", event));
            height = event.height;
            width = event.width;
        }

        if pollables_res.contains(&2) {
            frame_listener.get();
            print(&format!("frame event"));

            let graphics_buffer = graphics_context.get_current_buffer();

            let buffer = frame_buffer::FrameBuffer::from_graphics_buffer(graphics_buffer);

            const RED: u32 = 0b_00000000_11111111_00000000_00000000;
            const GREEN: u32 = 0b_00000000_00000000_11111111_00000000;
            const GRAY: u32 = 0b_00000000_10000000_10000000_10000000;

            let local_width = min(width, 100);
            let local_height = min(height, 100);
            for y in 0..local_height {
                for x in 0..local_width {
                    let color = if green { GREEN } else { RED };
                    let v = if is_on_rect(local_width, local_height, x, y) {
                        color
                    } else {
                        GRAY
                    };
                    let index = (y * width) + x;
                    if index < buffer.length() {
                        buffer.set(index, v);
                    }
                }
            }

            graphics_context.present();
        }
    }
}

fn is_on_rect(width: u32, height: u32, x: u32, y: u32) -> bool {
    y == 1 || y == height - 2 || x == 1 || x == width - 2
}
