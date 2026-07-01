wit_bindgen::generate!({
    path: "../../../wit",
    world: "example:example/example",
    generate_all,
});

export!(Example);

struct Example;

impl Guest for Example {
    async fn start() {
        draw_rectangle().await;
    }
}

use futures::stream::StreamExt;
use std::cell::Cell;
use std::cmp::min;
use wasi_gfx::surface::{surface, surface_frame_buffer};

async fn draw_rectangle() {
    let surface = surface::Surface::new(surface::CreateDesc {
        height: None,
        width: None,
    });
    let context = surface_frame_buffer::Context::new(&surface);
    let green = Cell::new(false);
    let height = Cell::new(surface.height());
    let width = Cell::new(surface.width());

    let pointer_up_stream = surface.on_pointer_up().into_stream().for_each(|event| {
        print(&format!("up: {:?}", event));
        green.set(!green.get());
        async {}
    });

    let resize_stream = surface.on_resize().into_stream().for_each(|event| {
        print(&format!("resize: {:?}", event));
        height.set(event.height);
        width.set(event.width);
        async {}
    });

    let frame_stream = surface.on_frame().into_stream().for_each(|_event| {
        print("frame event");

        let buffer = context.get_current_buffer();

        const RED: u32 = 0b_00000000_11111111_00000000_00000000;
        const GREEN: u32 = 0b_00000000_00000000_11111111_00000000;
        const GRAY: u32 = 0b_00000000_10000000_10000000_10000000;

        let width = width.get();
        let height = height.get();

        let local_width = min(width, 100);
        let local_height = min(height, 100);
        let mut buf = vec![0; (width * height) as usize];
        for y in 0..local_height {
            for x in 0..local_width {
                let color = if green.get() { GREEN } else { RED };
                let v = if is_on_rect(local_width, local_height, x, y) {
                    color
                } else {
                    GRAY
                };
                let index = (y * width) + x;
                if index < buf.len() as u32 {
                    buf[index as usize] = v;
                }
            }
        }

        buffer.set_with_copy(bytemuck::cast_slice(&buf));

        context.present();

        async {
            // give a chance for other events to come through
            futures_lite::future::yield_now().await
        }
    });

    futures::join!(pointer_up_stream, resize_stream, frame_stream,);
}

fn is_on_rect(width: u32, height: u32, x: u32, y: u32) -> bool {
    y == 1 || y == height - 2 || x == 1 || x == width - 2
}
