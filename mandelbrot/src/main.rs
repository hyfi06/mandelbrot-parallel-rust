use std::time::Instant;

use mandelbrot_lib::*;
use save_image;

fn main() {
    let x_range = (-0.24, -0.23);
    let y_range = (0.82, 0.83);
    let resolution = 10000;

    let start_time = Instant::now();
    let par_res = par_mandelbrot(&x_range, &y_range, resolution);
    println!("Parallel Mandelbrot {:?}", start_time.elapsed());

    save_image::pgm_p6(par_res.0, par_res.1, &par_res.2, "mandelbrot_par").unwrap();

    let start_time = Instant::now();
    let serial_res = serial_mandelbrot(&x_range, &y_range, resolution);
    println!("Serial Mandelbrot {:?}", start_time.elapsed());

    save_image::pgm_p6(serial_res.0, serial_res.1, &serial_res.2, "mandelbrot_s").unwrap();
}
