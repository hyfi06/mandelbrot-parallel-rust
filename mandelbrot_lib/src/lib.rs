use pyo3::prelude::*;
use rayon::prelude::*;

pub mod complex;

pub fn par_mandelbrot(
    x_range: &(f64, f64),
    y_range: &(f64, f64),
    resolution: u32,
) -> (u32, u32, Vec<u8>) {
    let (width, height, step) = get_step(x_range, y_range, resolution);

    let res = (0..height)
        .into_par_iter()
        .map(|row| {
            let im = y_range.1 - step * row as f64;
            (0..width)
                .map(|col| {
                    let re = x_range.0 + step * col as f64;
                    mandelbrot_seq(complex::Complex::new(re, im))
                })
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    (width, height, res)
}

pub fn serial_mandelbrot(
    x_range: &(f64, f64),
    y_range: &(f64, f64),
    resolution: u32,
) -> (u32, u32, Vec<u8>) {
    let (width, height, step) = get_step(x_range, y_range, resolution);

    let res: Vec<u8> = (0..height)
        .map(|row| {
            let im = y_range.1 - step * row as f64;
            (0..width)
                .map(move |col: u32| {
                    let re = x_range.0 + step * col as f64;
                    mandelbrot_seq(complex::Complex::new(re, im))
                })
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    (width, height, res)
}

fn mandelbrot_seq(c: complex::Complex<f64>) -> u8 {
    let mut count: u8 = 0;
    let mut z: complex::Complex<f64> = complex::Complex::new(0f64, 0f64);
    while z.norm_sqrt() < 4.0 && count < u8::MAX {
        z = z * z + c;
        count += 1;
    }
    count
}

fn get_step(x_range: &(f64, f64), y_range: &(f64, f64), resolution: u32) -> (u32, u32, f64) {
    let width: u32;
    let height: u32;
    let step: f64;
    assert!(x_range.0 < x_range.1);
    assert!(y_range.0 < y_range.1);
    if x_range.1 - x_range.0 > y_range.1 - y_range.0 {
        width = resolution;
        height =
            (resolution as f64 * (y_range.1 - y_range.0) / (x_range.1 - x_range.0)).floor() as u32;
        step = (x_range.1 - x_range.0) / resolution as f64;
    } else {
        height = resolution;
        width =
            (resolution as f64 * (x_range.1 - x_range.0) / (y_range.1 - y_range.0)).floor() as u32;
        step = (y_range.1 - y_range.0) / resolution as f64;
    }
    (width, height, step)
}

#[pyfunction]
fn py_mandelbrot(x_range: (f64, f64), y_range: (f64, f64), resolution: u32) -> (u32, u32, Vec<u8>) {
    let (width, height, step) = get_step(&x_range, &y_range, resolution);

    let res: Vec<u8> = (0..height)
        .map(|row| {
            let im = y_range.1 - step * row as f64;
            (0..width)
                .map(move |col: u32| {
                    let re = x_range.0 + step * col as f64;
                    mandelbrot_seq(complex::Complex::new(re, im))
                })
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    (width, height, res)
}

#[pymodule]
fn mandelbrot_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_mandelbrot, m)?)?;
    Ok(())
}