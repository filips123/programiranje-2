use std::fmt::Display;
use std::ops::Add;
use std::ops::Mul;

trait Zaporedje<T> {
    fn name(&self) -> String;
    fn start(&self) -> T;
    fn n_th(&self, n: usize) -> T;
    fn contains(&self, an: T) -> bool;
}

#[derive(Debug)]
struct KonstantnoZaporedje<T> {
    c: T,
}

impl<T> KonstantnoZaporedje<T> {
    fn new(c: T) -> Self {
        Self {
            c: c,
        }
    }
}

impl<T: Display + Copy + Eq> Zaporedje<T> for KonstantnoZaporedje<T> {
    fn name(&self) -> String {
        format!("KZ({})", self.c)
    }

    fn start(&self) -> T {
        self.c
    }

    fn n_th(&self, _: usize) -> T {
        self.c
    }

    fn contains(&self, an: T) -> bool {
        an == self.c
    }
}

#[derive(Debug)]
struct ArtimeticnoZaporedje<T> {
    a0: T,
    d: T,
    n: usize,
}

impl<T> ArtimeticnoZaporedje<T> {
    fn new(a0: T, d: T) -> Self {
        Self {
            a0: a0,
            d: d,
            n: 0,
        }
    }
}

impl<T: Display + Copy + Add<Output = T>> Zaporedje<T> for ArtimeticnoZaporedje<T> {
    fn name(&self) -> String {
        format!("AK({}, {})", self.a0, self.d)
    }

    fn start(&self) -> T {
        self.a0
    }

    fn n_th(&self, n: usize) -> T {
        let mut an = self.a0;

        for _ in 0..n {
            an = an + self.d;
        }

        an
    }

    fn contains(&self, an: T) -> bool {
        todo!()
    }
}

impl<T> ArtimeticnoZaporedje<T> {
    fn current(&self) -> T where T: Display + Copy + Add<Output = T> {
        self.n_th(self.n)
    }

    fn next(&mut self) -> T where T: Display + Copy + Add<Output = T> {
        let current = self.current();
        self.n += 1;
        current
    }

    fn reset(&mut self) {
        self.n = 0;
    }

    fn sum(&self, n: usize) -> T where T: Display + Copy + Add<Output = T> {
        let mut sn = self.n_th(0);

        for i in 1..n {
            sn = sn + self.n_th(i);
        }

        sn
    }

    fn vsota(&self, b: &ArtimeticnoZaporedje<T>) -> ArtimeticnoZaporedje<T> where T: Copy + Add<Output = T> {
        ArtimeticnoZaporedje::new(self.a0 + b.a0, self.d + b.d)
    }

    fn produkt(&self, b: &ArtimeticnoZaporedje<T>) -> ArtimeticnoZaporedje<T> where T: Copy + Mul<Output = T> {
        ArtimeticnoZaporedje::new(self.a0 * b.a0, self.d * b.d)
    }
}

fn main() {
    println!("Hello, world!");
}
