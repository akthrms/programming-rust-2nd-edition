use std::ops::{Add, AddAssign, Neg};

#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<L, R> Add<Complex<R>> for Complex<L>
where
    L: Add<R>,
{
    type Output = Complex<L::Output>;

    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}
