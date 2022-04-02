pub struct DisplayDriver {
  texture: nannou::wgpu::Texture,
  texture_capturer: nannou::wgpu::TextureCapturer,
  texture_reshaper: nannou::wgpu::TextureReshaper,
  draw: nannou::Draw,
  renderer: nannou::draw::Renderer,
}

impl DisplayDriver {
  pub fn new(window: &nannou::window::Window, texture_size: [u32; 2]) -> Self {
    // Retrieve the wgpu device.
    let device = window.device();
    // Create the custom texture.
    let sample_count = window.msaa_samples();
    let texture = nannou::wgpu::TextureBuilder::new()
      .size(texture_size)
      // The texture will be used as the RENDER_ATTACHMENT for the `Draw` render pass.
      // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
      .usage(
        nannou::wgpu::TextureUsages::RENDER_ATTACHMENT
          | nannou::wgpu::TextureUsages::TEXTURE_BINDING,
      )
      // Use nannou's default multisampling sample count.
      .sample_count(sample_count)
      // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
      .format(nannou::wgpu::TextureFormat::Rgba16Float)
      .build(device);
    // Create `Draw` instance and a renderer for it.
    let draw = nannou::Draw::new();
    let descriptor = texture.descriptor();
    let renderer =
      nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);
    // Create the texture capturer.
    let texture_capturer = nannou::wgpu::TextureCapturer::default();
    // Create the texture reshaper to display on screen
    let texture_view = texture.view().build();
    let texture_sample_type = texture.sample_type();
    let dst_format = nannou::Frame::TEXTURE_FORMAT;
    let texture_reshaper = nannou::wgpu::TextureReshaper::new(
      device,
      &texture_view,
      sample_count,
      texture_sample_type,
      sample_count,
      dst_format,
    );

    Self {
      texture,
      draw,
      renderer,
      texture_capturer,
      texture_reshaper,
    }
  }

  pub fn draw(&self) -> &nannou::draw::Draw {
    &self.draw
  }

  fn create_snapshot(&mut self, window: &nannou::window::Window) -> nannou::wgpu::TextueSnapshot {
    let device = window.device();
    let ce_desc = nannou::wgpu::CommandEncoderDescriptor {
      label: Some("texture renderer"),
    };
    let mut encoder = device.create_command_encoder(&ce_desc);
    self
      .renderer
      .render_to_texture(device, &mut encoder, &self.draw, &self.texture);
    // Take a snapshot of the texture. The capturer will do the following
    // 1. Resolve the texture to a non-multisampled texture if necessary.
    // 2. Convert the format to non-linear 8-bit sRGBA ready for image storage.
    // 3. Copy the result to a buffer ready to be mapped for reading.
    let snapshot = self
      .texture_capturer
      .capture(device, &mut encoder, &self.texture);
    // Submit the commands for our drawing and texture capture to the GPU.
    window.queue().submit(Some(encoder.finish()));
    snapshot
  }

  pub fn save(&mut self, window: &nannou::window::Window, path: std::path::PathBuf) {
    // Submit a function for writing our snapshot to a PNG.
    // NOTE: It is essential that the commands for capturing the snapshot are `submit`ted before we
    // attempt to read the snapshot - otherwise we will read a blank texture!
    let path = path.with_extension("png");
    println!("Saving {}", path.display());
    let snapshot = self.create_snapshot(window);
    snapshot
      .read(move |result| {
        let image = result.expect("failed to map texture memory").to_owned();
        image
          .save(&path)
          .expect("failed to save texture to png image");
      })
      .unwrap();
  }

  pub fn wait(&self, window: &nannou::window::Window) {
    println!("Waiting for PNG writing to complete");
    let device = window.device();
    self
      .texture_capturer
      .await_active_snapshots(&device)
      .unwrap();
    println!("PNG writing completed");
  }

  pub fn render(&self, frame: nannou::frame::Frame) {
    let mut encoder = frame.command_encoder();
    self
      .texture_reshaper
      .encode_render_pass(frame.texture_view(), &mut *encoder);
  }
}
