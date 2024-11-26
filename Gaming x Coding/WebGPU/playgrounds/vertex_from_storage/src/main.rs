use std::time::Instant;

use image::{ImageBuffer, Rgba};

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    _padding: f32,
    color: [f32; 4],
}

async fn run() {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Vec Buffer"),
        size: (std::mem::size_of::<Vertex>() * 1024) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let compute_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Compute Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[&compute_bind_group_layout],
        push_constant_ranges: &[],
    });
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&compute_pipeline_layout),
        module: &device.create_shader_module(wgpu::include_wgsl!("../shader/compute.wgsl")),
        entry_point: Some("cp_main"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });

    let render_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Render Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&render_bind_group_layout],
        push_constant_ranges: &[],
    });
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &device.create_shader_module(wgpu::include_wgsl!("../shader/shader.wgsl")),
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::PointList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &device.create_shader_module(wgpu::include_wgsl!("../shader/shader.wgsl")),
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8Unorm,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        multiview: None,
        cache: None,
    });

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Texture"),
        size: wgpu::Extent3d {
            width: 128,
            height: 128,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Compute Bind Group"),
        layout: &compute_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: vertex_buffer.as_entire_binding(),
        }],
    });

    let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Render Bind Group"),
        layout: &render_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: vertex_buffer.as_entire_binding(),
        }],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Encoder"),
    });

    // write to storage using compute shader
    compute(&mut encoder, &compute_pipeline, &compute_bind_group);

    // write to storage directly
    // let data = bytemuck::cast_slice(&[
    //     Vertex {
    //         position: [0.0, 0.0, 0.0],
    //         _padding: 0.0,
    //         color: [1.0, 0.0, 0.0, 1.0],
    //     },
    //     Vertex {
    //         position: [1.0, 0.0, 0.0],
    //         _padding: 0.0,
    //         color: [0.0, 1.0, 0.0, 1.0],
    //     },
    //     Vertex {
    //         position: [0.0, 1.0, 0.0],
    //         _padding: 0.0,
    //         color: [0.0, 0.0, 1.0, 1.0],
    //     },
    // ]);
    // println!("{:?}", data);
    // queue.write_buffer(&vertex_buffer, 0, data);
    // queue.submit([]);

    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            timestamp_writes: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture.create_view(&wgpu::TextureViewDescriptor::default()),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&render_pipeline);
        pass.set_bind_group(0, &render_bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
    queue.submit(Some(encoder.finish()));

    save_texture(&device, &queue, &texture).await;
}

fn compute(
    encoder: &mut wgpu::CommandEncoder,
    compute_pipeline: &wgpu::ComputePipeline,
    compute_bind_group: &wgpu::BindGroup,
) {
    let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        label: Some("Compute Pass"),
        timestamp_writes: None,
    });
    pass.set_pipeline(compute_pipeline);
    pass.set_bind_group(0, compute_bind_group, &[]);
    pass.dispatch_workgroups(3, 1, 1);
}

async fn save_texture(device: &wgpu::Device, queue: &wgpu::Queue, texture: &wgpu::Texture) {
    let size = texture.size().width as usize * texture.size().height as usize * 4;
    let mut texture_data = vec![0u8; size];
    let output_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Staging Buffer"),
        size: size as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_staging_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(texture.size().width as u32 * 4),
                rows_per_image: Some(texture.size().height as u32),
            },
        },
        texture.size(),
    );

    queue.submit(Some(encoder.finish()));
    let buffer_slice = output_staging_buffer.slice(..);

    // NOTE: We have to create the mapping THEN device.poll() before await
    // the future. Otherwise the application will freeze.
    let (tx, rx) = async_channel::bounded(1);
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        tx.send_blocking(result).unwrap()
    });
    device.poll(wgpu::Maintain::Wait).panic_on_timeout();
    rx.recv().await.unwrap().unwrap();

    {
        let view = buffer_slice.get_mapped_range();
        texture_data.copy_from_slice(&view);
    }
    output_staging_buffer.unmap();

    let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
        texture.size().width as u32,
        texture.size().height as u32,
        texture_data,
    )
    .unwrap();
    buffer.save("./output.png").unwrap();
}

fn main() {
    // @workgroup_size(1, 1): 430 ~ 450ms
    // @workgroup_size(16, 16): 450 ~ 480ms
    let t = Instant::now();
    pollster::block_on(run());
    println!("Time: {:?}", t.elapsed());
}
