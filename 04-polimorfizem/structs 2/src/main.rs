use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
struct ArtimeticnoZaporedje<T> {
    zacetni_clen: T, // ar zaporedje lahko podamo s zacetnim clenom in diferenco
    diferenca: T,
    trenutni_clen: T,
}

impl<T> ArtimeticnoZaporedje<T>
where
    T: Add<Output = T> + Copy,
{
    fn new(zacetni: T, dif: T) -> Self {
        ArtimeticnoZaporedje {
            zacetni_clen: zacetni,
            diferenca: dif,
            trenutni_clen: zacetni,
        }
    }
    fn next(&mut self) -> T {
        let clen = self.trenutni_clen;
        self.trenutni_clen = self.trenutni_clen + self.diferenca;
        clen
    }
    fn n_th(&mut self, n: usize) -> T
    where
        T: Add<Output = T> + Mul<usize, Output = T>,
    {
        self.zacetni_clen + self.diferenca * (n - 1)
    }
    fn reset(&mut self) {
        self.trenutni_clen = self.zacetni_clen;
    }
    fn current(&self) -> T {
        self.trenutni_clen
    }
    fn sum(&self, n: i32) -> T
    where
        T: Mul<usize, Output = T> + Add<Output = T> + From<u8>,
    {
        let mut vsota = T::from(0);
        let mut pomozna = self.zacetni_clen;
        for _ in 0..n {
            vsota = vsota + pomozna;
            pomozna = pomozna + self.diferenca;
        }
        vsota
    }
    fn vsota(
        zap1: ArtimeticnoZaporedje<T>,
        zap2: ArtimeticnoZaporedje<T>,
    ) -> ArtimeticnoZaporedje<T> {
        ArtimeticnoZaporedje {
            zacetni_clen: zap1.zacetni_clen + zap2.zacetni_clen,
            diferenca: zap1.diferenca + zap2.diferenca,
            trenutni_clen: zap1.zacetni_clen + zap2.zacetni_clen,
        }
    }
    fn produkt(a: &ArtimeticnoZaporedje<T>, b: &ArtimeticnoZaporedje<T>) -> ArtimeticnoZaporedje<T>
    where
        T: Mul<Output = T>,
    {
        ArtimeticnoZaporedje {
            trenutni_clen: a.zacetni_clen * b.zacetni_clen,
            zacetni_clen: a.zacetni_clen * b.zacetni_clen,
            diferenca: a.diferenca * b.diferenca,
        }
    }
}

impl<T: PartialEq> PartialEq for ArtimeticnoZaporedje<T> {
    fn eq(&self, other: &Self) -> bool {
        self.zacetni_clen == other.zacetni_clen && self.diferenca == other.diferenca
    }
}
use std::fmt;
#[derive(Debug, Clone)]
enum BinOperacija {
    Plus,
    Minus,
    Times,
}
#[derive(Debug, Clone)]
enum Izraz<T> {
    Konstanta(T),
    Operacija(Box<Izraz<T>>, BinOperacija, Box<Izraz<T>>),
}

impl<T> Izraz<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone,
{
    fn eval(&self) -> T {
        match self {
            Izraz::Konstanta(vrednost) => vrednost.clone(),
            Izraz::Operacija(konst1, oper, konst2) => {
                let leva = konst1.eval();
                let desna = konst2.eval();
                match oper {
                    BinOperacija::Plus => leva + desna,
                    BinOperacija::Minus => leva - desna,
                    BinOperacija::Times => leva * desna,
                }
            }
        }
    }
    fn collect(&self) -> u32 {
        match self {
            Izraz::Konstanta(_) => 1,
            Izraz::Operacija(k1, _, k2) => k1.collect() + k2.collect(),
        }
    }
    
}

impl<T: fmt::Display> fmt::Display for Izraz<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Izraz::Konstanta(x) => write!(f, "{}", x),
            Izraz::Operacija(left, op, right) => {
                let op_str = match op {
                    BinOperacija::Plus => "+",
                    BinOperacija::Minus => "-",
                    BinOperacija::Times => "*",
                };
                write!(f, "({} {} {})", left, op_str, right)
            }
        }
    }
}

fn main() {
    let racun1 = Izraz::Operacija(
        Box::new(Izraz::Konstanta(1)),
        BinOperacija::Plus,
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(2)),
            BinOperacija::Times,
            Box::new(Izraz::Konstanta(3)),
        )),
    );
    let racun2 = Izraz::Operacija(
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(1)),
            BinOperacija::Plus,
            Box::new(Izraz::Konstanta(2)),
        )),
        BinOperacija::Times,
        Box::new(Izraz::Konstanta(3)),
    );
    let racun3 = Izraz::Operacija(
        Box::new(Izraz::Konstanta(1)),
        BinOperacija::Plus,
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(2)),
            BinOperacija::Plus,
            Box::new(Izraz::Konstanta(3)),
        )),
    );

    println!("{}", racun1.eval());
    println!("{}", racun2.eval());
    println!("{}", racun3.eval());

}
