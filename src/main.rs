use nannou::prelude::*;
use nannou::image;

struct Model {
    _window: WindowId,
    texture: wgpu::Texture
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    let window = app.main_window();
    let win = window.rect();

    // Initialize a texture
    let texture = wgpu::TextureBuilder::new()
        .size([win.w() as u32, win.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED)
        .build(window.swap_chain_device());

    Model { _window, texture }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let win = app.window_rect();

    // Update the texture using perlin
    let image = image::ImageBuffer::from_fn(win.w() as u32, win.h() as u32, |_x, _y| {
        nannou::image::Rgba([0, 100, 255, std::u8::MAX])
    });

    let flat_samples = image.as_flat_samples();
    model.texture.upload_data(
        app.main_window().swap_chain_device(),
        &mut *frame.command_encoder(),
        &flat_samples.as_slice(),
    );

    let draw = app.draw();
    draw.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
}
