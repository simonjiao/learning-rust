use error_chain::error_chain;
use image::ImageBuffer;
use num::complex::Complex;
use std::sync::mpsc::{self, RecvError};
use threadpool::ThreadPool;

use super::julia::julia;
use super::julia::wavelength_to_rgb;

error_chain! {
    foreign_links {
        MpsRecv(RecvError);
        Io(std::io::Error);
        ImageError(image::ImageError);
    }
}

pub fn draw_fractal() -> Result<()> {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = mpsc::channel();

    let (width, height) = (1920, 1080);
    let mut img = ImageBuffer::new(width, height);
    let iterations = 300;

    let c = Complex::new(-0.8, 0.156);

    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || {
            for x in 0..width {
                let i = julia(c, x, y, width, height, iterations);
                let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                tx.send((x, y, pixel)).expect("Could not send data");
            }
        });
    }
    drop(tx);

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x, y, pixel);
    }

    let _ = img.save("output.png")?;
    Ok(())
}
