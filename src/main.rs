extern crate pixels;
extern crate winit;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// Julia set parameters
const DEFAULT_C_RE: f64 = -0.7;
const DEFAULT_C_IM: f64 = 0.27015;

const MAXITER: u32 = 10;
const COLOR_FACTOR: u32 = 100;

fn main() {
    // Launch parameters with correct aspect ratio and proportions
    let mut xs: f64 = -2.0; // x fractal space coordinates start
    let xe: f64 =  2.0; // x fractal space coordinates end
    let mut ys: f64 =  1.0;
    let ye: f64 = -1.0;
    let mut xr: f64 = xe - xs; // x fractal space coordinates range
    let mut yr: f64 = -(ye - ys);

    let ar = WIDTH as f64 / HEIGHT as f64;
    if ar > (xr / yr) {
        xr = ar * yr;
    } else {
        yr = xr / ar;
    }
    xs = -(xr / 2.0);
    // xe = xr / 2.0;
    ys = yr / 2.0;
    // ye = -(yr / 2.0);
    let xi: f64 = xr / WIDTH as f64; // x fractal space coordinates increment step
    let yi: f64 = yr / HEIGHT as f64;

    // Create an event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Julia Set Viewer")
        .with_inner_size(winit::dpi::PhysicalSize::new(WIDTH as f64, HEIGHT as f64))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    // Create a new instance of the Pixels object
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    // Julia set parameter C
    let mut c_re = DEFAULT_C_RE;
    let mut c_im = DEFAULT_C_IM;

    // Main loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {event: WindowEvent::CloseRequested,..} => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },..
            } => {
                //println!("Cursor position: {:?}", position);
                c_re = xs + (position.x as f64 * xr) / WIDTH as f64;
                c_im = ys - (position.y as f64 * (ys - ye)) / HEIGHT as f64;
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let frame = pixels.get_frame();
                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % WIDTH as usize) as f64;
                    let y = (i / WIDTH as usize) as f64;

                    let zx = xs + (x as f64 * xi);
                    let zy = ys - (y as f64 * yi);

                    let mut px = zx;
                    let mut py = zy;
                    let mut i: u32 = 0;

                    while px * px + py * py <= 4.0 && i < MAXITER {
                        let tmp = px * px - py * py + c_re;
                        py = 2.0 * px * py + c_im;
                        px = tmp;
                        i += 1;
                    }

                    let color = get_color(i);
                    pixel.copy_from_slice(&color);
                }

                // Send the frame to the window for display
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => {}
        }
    });
}

fn get_color(i: u32) -> [u8; 4] {
    if i == MAXITER {
        [0, 0, 0, 255] // Fully opaque black
    } else {
        let hue = ((COLOR_FACTOR * i) as f64 / MAXITER as f64) as u64 % 360;
        let rgb = hsb_to_rgb(hue as f64, 1.0, 1.0);
        [rgb.0, rgb.1, rgb.2, 255] // Fully opaque grayscale
    }
}

// h = [0, 360], s = [0, 1], b = [0, 1]
fn hsb_to_rgb(h: f64, s: f64, b: f64) -> (u8, u8, u8) {
    let c = b * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = b - c;

    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
