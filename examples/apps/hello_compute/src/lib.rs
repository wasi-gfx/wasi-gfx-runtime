use wasi::webgpu::webgpu;

wit_bindgen::generate!({
    path: "../../../wit",
    world: "example:example/example",
    generate_all,
});

export!(Example);

struct Example;

impl Guest for Example {
    fn start() {
        pollster::block_on(run_compute());
    }
}

// Indicates a u32 overflow in an intermediate Collatz value
const OVERFLOW: u32 = 0xffffffff;

#[cfg_attr(test, allow(dead_code))]
async fn run_compute() {
    let numbers = vec![1, 2, 3, 4];

    let steps = execute_gpu(&numbers).await.unwrap();

    let disp_steps: Vec<String> = steps
        .iter()
        .map(|&n| match n {
            OVERFLOW => "OVERFLOW".to_string(),
            _ => n.to_string(),
        })
        .collect();

    print(&format!("Steps: [{}]", disp_steps.join(", ")));
}

#[cfg_attr(test, allow(dead_code))]
async fn execute_gpu(numbers: &[u32]) -> Option<Vec<u32>> {
    let device = webgpu::get_gpu()
        .request_adapter(None)
        .unwrap()
        .request_device(None);

    // Loads the shader from WGSL
    let cs_module = device.create_shader_module(&webgpu::GpuShaderModuleDescriptor {
        label: None,
        // source: webgpu::GpuShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        code: include_str!("shader.wgsl").to_string(),
        compilation_hints: None,
    });

    // Gets the size in bytes of the buffer.
    let size = std::mem::size_of_val(numbers) as webgpu::GpuSize64;

    // Instantiates buffer without data.
    // `usage` of buffer specifies how it can be used:
    //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
    //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
    let staging_buffer = device.create_buffer(&webgpu::GpuBufferDescriptor {
        label: None,
        size,
        usage: webgpu::GpuBufferUsage::map_read() | webgpu::GpuBufferUsage::copy_dst(),
        mapped_at_creation: None,
    });

    // Instantiates buffer with data (`numbers`).
    // Usage allowing the buffer to be:
    //   A storage buffer (can be bound within a bind group and thus available to a shader).
    //   The destination of a copy.
    //   The source of a copy.
    let contents = bytemuck::cast_slice(numbers);
    let storage_buffer = device.create_buffer(&webgpu::GpuBufferDescriptor {
        label: Some("Storage Buffer".to_string()),
        size: contents.len() as webgpu::GpuSize64,
        // usage: webgpu::GpuBufferUsages::STORAGE
        //     | webgpu::GpuBufferUsages::COPY_DST
        //     | webgpu::GpuBufferUsages::COPY_SRC,
        usage: (1 << 7) | (1 << 3) | (1 << 2),
        mapped_at_creation: Some(true),
    });

    let data = storage_buffer.get_mapped_range(None, None);
    data.set(contents);
    storage_buffer.unmap();

    // A bind group defines how buffers are accessed by shaders.
    // It is to WebGPU what a descriptor set is to Vulkan.
    // `binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).

    // A pipeline specifies the operation of a shader

    // Instantiates the pipeline.
    let compute_pipeline = device.create_compute_pipeline(webgpu::GpuComputePipelineDescriptor {
        label: None,
        layout: webgpu::GpuLayout::GpuAutoLayoutMode(webgpu::GpuAutoLayoutMode::Auto),
        compute: webgpu::GpuProgrammableStage {
            module: &cs_module,
            entry_point: Some("main".to_string()),
            constants: None,
        },
    });

    // Instantiates the bind group, once again specifying the binding of buffers.
    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&webgpu::GpuBindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: vec![webgpu::GpuBindGroupEntry {
            binding: 0,
            resource: webgpu::GpuBindingResource::GpuBufferBinding(webgpu::GpuBufferBinding {
                buffer: &storage_buffer,
                offset: Some(0),
                size: None,
            }),
        }],
    });

    // A command encoder executes one or many pipelines.
    // It is to WebGPU what a command buffer is to Vulkan.
    let encoder =
        device.create_command_encoder(Some(&webgpu::GpuCommandEncoderDescriptor { label: None }));
    {
        let cpass = encoder.begin_compute_pass(None);
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, Some(&bind_group), Some(&[]));
        cpass.insert_debug_marker("compute collatz iterations");
        cpass.dispatch_workgroups(numbers.len() as u32, Some(1), Some(1)); // Number of cells to run, the (x,y,z) size of item being processed
        cpass.end();
    }
    // Sets adds copy operation to command encoder.
    // Will copy data from storage buffer on GPU to staging buffer on CPU.
    encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer, 0, size);

    // Submits command encoder for processing
    device.queue().submit(&[&encoder.finish(None)]);

    staging_buffer.map_async(webgpu::GpuMapMode::read(), Some(0), None);

    // Gets contents of buffer
    let data = staging_buffer.get_mapped_range(None, None);
    // Since contents are got in bytes, this converts these bytes back to u32
    let result = bytemuck::cast_slice(&data.get()).to_vec();

    // With the current interface, we have to make sure all mapped views are
    // dropped before we unmap the buffer.
    drop(data);
    staging_buffer.unmap(); // Unmaps buffer from memory
                            // If you are familiar with C++ these 2 lines can be thought of similarly to:
                            //   delete myPointer;
                            //   myPointer = NULL;
                            // It effectively frees the memory

    // Returns data from buffer
    Some(result)
}
