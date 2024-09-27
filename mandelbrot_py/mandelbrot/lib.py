from typing import Tuple, List
import math
from functools import reduce


def mandelbrot(x_range: Tuple[float, float], y_range: Tuple[float, float], resolution: int) -> Tuple[int, int, List[int]]:
    (width, height, step) = get_step(x_range, y_range, resolution)
    res = [[mandelbrot_seq(complex(
        x_range[0] + step * float(col),
        y_range[1] - step * float(row)
    )) for col in range(0, width)] for row in range(0, height)]
    return (width, height, list(reduce(lambda acc, cur: acc+cur, res, [])))


def mandelbrot_seq(c: complex):
    count: int = 0
    z: complex = 0.0 + 0.0j
    while abs(z) < 2 and count < 256:
        z = pow(z, 2) + c
        count += 1
    return count


def get_step(x_range: Tuple[float, float], y_range: Tuple[float, float], resolution: int) -> Tuple[int, int, float]:
    assert x_range[0] < x_range[1]
    assert y_range[0] < y_range[1]
    width: int
    height: int
    step: float

    if x_range[1] - x_range[0] > y_range[1] - y_range[0]:
        width = resolution
        height = int(math.floor(
            (float(resolution) * (y_range[1] - y_range[0]) / (x_range[1] - x_range[0]))))
        step = (x_range[1] - x_range[0]) / float(resolution)
    else:
        height = resolution
        width = int(math.floor(float(resolution) *
                    (x_range[1] - x_range[0]) / (y_range[1] - y_range[0])))

        step = (y_range[1] - y_range[0]) / float(resolution)
    return (width, height, step)
