#[derive(Debug)]
struct ArtimeticnoZaporedje {
    a0: i32,
    d: i32,
    n: i32,
}

impl ArtimeticnoZaporedje {
    pub fn new(a0: i32, d: i32) -> Self {
        Self {
            a0: a0,
            d: d,
            n: 0,
        }
    }

    pub fn current(&self) -> i32 {
        self.n_th(self.n)
    }

    pub fn next(&mut self) -> i32 {
        let current = self.current();
        self.n += 1;
        current
    }

    pub fn reset(&mut self) {
        self.n = 0;
    }

    pub fn n_th(&self, n: i32) -> i32 {
        self.a0 + n * self.d
    }

    pub fn sum(&self, n: i32) -> i32 {
        (0..n).map(|i| self.n_th(i)).sum()
    }

    pub fn vsota(&self, b: &ArtimeticnoZaporedje) -> ArtimeticnoZaporedje {
        ArtimeticnoZaporedje::new(self.a0 + b.a0, self.d + b.d)
    }
}

struct GeometrijskoZaporedje {
    a0: i32,
    q: i32,
    n: u32,
}

impl GeometrijskoZaporedje {
    pub fn new(a0: i32, q: i32) -> Self {
        Self {
            a0: a0,
            q: q,
            n: 0,
        }
    }

    pub fn current(&self) -> i32 {
        self.n_th(self.n)
    }

    pub fn next(&mut self) -> i32 {
        let current = self.current();
        self.n += 1;
        current
    }

    pub fn reset(&mut self) {
        self.n = 0;
    }

    pub fn n_th(&self, n: u32) -> i32 {
        self.a0 * i32::pow(self.q, n)
    }

    pub fn sum(&self, n: u32) -> i32 {
        (0..n).map(|i| self.n_th(i)).sum()
    }

    pub fn produkt(&self, b: &GeometrijskoZaporedje) -> GeometrijskoZaporedje {
        GeometrijskoZaporedje::new(self.a0 * b.a0, self.q * b.q)
    }
}

enum BinOperacija {
    Plus,
    Minus,
    Times,
    Power,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}

impl Izraz {
    pub fn eval(&self) -> u32 {
        match self {
            Self::Konstanta(n) => *n,
            Self::Operacija(izraz1, operacija, izraz2) => {
                match operacija {
                    BinOperacija::Plus => izraz1.eval() + izraz2.eval(),
                    BinOperacija::Minus => izraz1.eval() - izraz2.eval(),
                    BinOperacija::Times => izraz1.eval() * izraz2.eval(),
                    BinOperacija::Power => u32::pow(izraz1.eval(), izraz2.eval()),
                }
            }
        }
    }

    pub fn collect(&self) -> u32 {
        match self {
            Self::Konstanta(n) => 1,
            Self::Operacija(izraz1, _, izraz2) => izraz1.collect() + izraz2.collect()
        }
    }

    pub fn izpis(&self) -> String {
        match self {
            Self::Konstanta(n) => n.to_string(),
            Self::Operacija(izraz1, operacija, izraz2) => {
                let op = match operacija {
                    BinOperacija::Plus => "+",
                    BinOperacija::Minus => "-",
                    BinOperacija::Times => "*",
                    BinOperacija::Power => "^",
                };
                format!("({} {} {})", izraz1.izpis(), op, izraz2.izpis())
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
