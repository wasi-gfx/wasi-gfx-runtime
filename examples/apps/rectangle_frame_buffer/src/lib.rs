use crate::surface::PointerEvent;

wit_bindgen::generate!({
    path: "../../../wit",
    world: "example:example/example",
    generate_all,
});

export!(Example);

struct Example;

impl Guest for Example {
    fn start() {
        draw_rectangle();
    }
}

use std::cmp::min;
use wasi::{frame_buffer::frame_buffer, graphics_context::graphics_context, surface::surface};

fn draw_rectangle() {
    let canvas = surface::Surface::new(surface::CreateDesc {
        height: None,
        width: None,
    });
    canvas.request_set_size(Some(1024), Some(768));
    let graphics_context = graphics_context::Context::new();
    canvas.connect_graphics_context(&graphics_context);

    let surface = frame_buffer::Device::new();
    surface.connect_graphics_context(&graphics_context);

    let pointer_up_pollable = canvas.subscribe_pointer_up();
    let resize_pollable = canvas.subscribe_resize();
    let frame_pollable = canvas.subscribe_frame();
    let pollables = vec![&pointer_up_pollable, &resize_pollable, &frame_pollable];
    let mut green = false;
    let mut height = canvas.height();
    let mut width = canvas.width();
    //let mut height = 800;
    //let mut width = 600;
    loop {
        let pollables_res = wasi::io::poll::poll(&pollables);

        if pollables_res.contains(&0) {

            print("Drawing rectangle example started 1");
            let event = canvas.get_pointer_up();
            match event {
                Some(PointerEvent { x, y }) => {
                    print(&format!("pointer up: {:?}", event));
                    print(&format!("x: {}, y: {}", x, y));
                }
                None => {
                    print("pointer up event not found");
                }
            }
            print(&format!("up: ___ {:?}", event));
            green = !green;
        }

        if pollables_res.contains(&1) {
            if let event = canvas.get_resize() {
                print(&format!("resize: {:?}", event));
                //canvas.request_set_size(Some(event.unwrap().width), Some(event.unwrap().height));
                //height = event.height;
                //width = event.width;
            } else {
                print("resize event not found");
            }
        }

        if pollables_res.contains(&2) {
            let event = canvas.get_frame();
            print(&format!("frame event {:?}", event));

            let graphics_buffer = graphics_context.get_current_buffer();
            let buffer = frame_buffer::Buffer::from_graphics_buffer(graphics_buffer);

            const RED: u32 = 0b_00000000_11111111_00000000_00000000;
            const GREEN: u32 = 0b_00000000_00000000_11111111_00000000;
            const GRAY: u32 = 0b_00000000_10000000_10000000_10000000;

            let local_width = min(width, 100);
            let local_height = min(height, 100);
            //let mut buf = vec![0; (width * height) as usize];
            let mut buf = vec![0; 3145728];
            for y in 0..local_height {
                for x in 0..local_width {
                    if (x < 1 || y < 1) {
                        continue;
                    }
                    let color = if green { GREEN } else { RED };
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

            print("before buffer");
            print(&format!("buffer size {}", buf.len()));
            // increase buffer size to 399920004
            let frame_data: &[u8] = bytemuck::cast_slice(&buf);
            print(&format!("frame data len {}", frame_data.len()));

            //let frame_data = bytemuck::cast_slice(&buf);
            //print(format!("frame data {:?}", frame_data).as_str());
            print(format!("frame data len {}", frame_data.len()).as_str());
            buffer.set(frame_data);;
            print("after buffer");

            print("draw before");
            graphics_context.present();
            print("draw after");
        }
    }
}

fn is_on_rect(width: u32, height: u32, x: u32, y: u32) -> bool {
    y == 1 || y == height - 2 || x == 1 || x == width - 2
}
