from timeit import default_timer as timer
from datetime import timedelta
from typing import Tuple, List
from mandelbrot.lib import mandelbrot
import mandelbrot_lib as mlib


def main():
    x_range: Tuple[float, float] = (-0.24, -0.23)
    y_range: Tuple[float, float] = (0.82, 0.83)
    resolution: int = 1000
    start = timer()
    (_, _, py_res) = mandelbrot(x_range, y_range, resolution)
    end = timer()
    print(len(py_res))
    print(f"Python {timedelta(seconds=end-start)}")

    start = timer()
    (_, _, rust_res) = mlib.py_mandelbrot(x_range, y_range, resolution)
    end = timer()
    print(len(rust_res))
    print(f"Rust {timedelta(seconds=end-start)}")


if __name__ == "__main__":
    main()
