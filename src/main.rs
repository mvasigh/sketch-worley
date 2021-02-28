use nannou::image;
use nannou::prelude::*;

const MAX_DIST: f64 = 200.0;
const NUM_POINTS: usize = 20;

fn distance(a: &Vector2<f64>, b: &Vector2<f64>) -> f64 {
    let x = b.x - a.x;
    let y = b.y - a.y;
    (x.powi(2) + y.powi(2)).sqrt()
}

fn closer(origin: Vector2<f64>, pt1: Vector2<f64>, pt2: Vector2<f64>) -> Vector2<f64> {
    let dist1 = distance(&origin, &pt1);
    let dist2 = distance(&origin, &pt2);
    if dist2 > dist1 {
        pt1.to_owned()
    } else {
        pt2.to_owned()
    }
}

struct Model {
    _window: WindowId,
    texture: wgpu::Texture,
    points: Vec<Vector2<f64>>,
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

    let points = (0..NUM_POINTS)
        .map(|_i| {
            Vector2::new(
                random_range::<f64>(0.0, 800.0),
                random_range::<f64>(0.0, 800.0),
            )
        })
        .collect::<Vec<Vector2<f64>>>();

    Model {
        _window,
        texture,
        points,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let win = app.window_rect();

    // Update the texture using perlin
    let image = image::ImageBuffer::from_fn(win.w() as u32, win.h() as u32, |x, y| {
        let pt = vec2(x as f64, y as f64);
        let closest = model
            .points
            .to_owned()
            .into_iter()
            .fold(None, |acc, curr| match acc {
                None => Some(curr),
                Some(acc) => Some(closer(pt, acc, curr)),
            })
            .expect("Could not get closest point");
        let dist = clamp(distance(&pt, &closest), 0.0, MAX_DIST);
        let alpha = map_range(dist, 0.0, MAX_DIST, 0, std::u8::MAX);

        nannou::image::Rgba([0, 0, 0, alpha])
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
