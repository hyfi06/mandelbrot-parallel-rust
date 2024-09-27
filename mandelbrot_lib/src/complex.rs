use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Complex<T> {
    pub(crate) re: T,
    pub(crate) im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Complex<T> {
        Complex { re, im }
    }
}

impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Complex::<T>::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<T> Mul for Complex<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Complex<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex::<T>::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Complex<T> {
    pub fn norm_sqrt(&self) -> T {
        self.re * self.re + self.im * self.im
    }
}
