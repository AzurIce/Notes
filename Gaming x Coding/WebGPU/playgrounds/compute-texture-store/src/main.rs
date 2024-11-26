use std::time::Instant;

use image::{ImageBuffer, Rgba};
use wgpu::{TextureSampleType, TextureViewDimension};

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

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::StorageTexture {
                access: wgpu::StorageTextureAccess::WriteOnly,
                format: wgpu::TextureFormat::Rgba8Unorm,
                view_dimension: wgpu::TextureViewDimension::D2,
            },
            count: None,
        }],
    });
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    let module = &device.create_shader_module(wgpu::include_wgsl!("../shader/shader.wgsl"));
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&compute_pipeline_layout),
        module,
        entry_point: Some("main"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Texture"),
        size: wgpu::Extent3d {
            width: 1024,
            height: 768,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::STORAGE_BINDING,
        view_formats: &[],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(
                &texture.create_view(&wgpu::TextureViewDescriptor::default()),
            ),
        }],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Encoder"),
    });

    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&compute_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(32, 32, 32);
    }
    queue.submit(Some(encoder.finish()));
    save_texture(&device, &queue, &texture).await;
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
