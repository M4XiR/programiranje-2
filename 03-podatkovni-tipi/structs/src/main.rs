struct ArtimeticnoZaporedje {
    zacetni_clen: i32, // ar zaporedje lahko podamo s zacetnim clenom in diferenco
    diferenca: i32,
    trenutni_clen: i32,
}
struct NekoZaporedje {
    zacetni_clen: i32,
    diferenca: i32,
}

impl ArtimeticnoZaporedje {
    fn new(zacetni: i32, dif: i32) -> ArtimeticnoZaporedje {
        ArtimeticnoZaporedje {
            zacetni_clen: zacetni,
            diferenca: dif,
            trenutni_clen: zacetni,
        }
    }
    fn next(&mut self) -> i32 {
        let clen = self.trenutni_clen;
        self.trenutni_clen += self.diferenca;
        clen
    }
    fn n_th(&mut self, n: i32) -> i32 {
        self.zacetni_clen + (n - 1) * self.diferenca
    }
    fn reset(&mut self) {
        self.trenutni_clen = self.zacetni_clen;
    }
    fn current(&self) -> i32 {
        self.trenutni_clen
    }
    fn sum(&self, n: i32) -> i32 {
        let mut vsota = 0;
        let mut pomozna = self.zacetni_clen;
        for _ in 0..n {
            vsota += pomozna;
            pomozna += self.diferenca;
        }
        vsota
    }
    fn vsota(zap1: ArtimeticnoZaporedje, zap2: ArtimeticnoZaporedje) -> ArtimeticnoZaporedje {
        ArtimeticnoZaporedje {
            zacetni_clen: zap1.zacetni_clen + zap2.zacetni_clen,
            diferenca: zap1.diferenca + zap2.diferenca,
            trenutni_clen: zap1.zacetni_clen + zap2.zacetni_clen,
        }
    }
    fn produkt(a: &ArtimeticnoZaporedje, b: &ArtimeticnoZaporedje) -> NekoZaporedje {
        NekoZaporedje {
            zacetni_clen: a.zacetni_clen * b.zacetni_clen,
            diferenca: a.zacetni_clen * b.diferenca
                + b.zacetni_clen * a.diferenca
                + a.diferenca * b.diferenca,
        }
    }
}

struct GeometrijskoZaporedje {
    zacetni_clen: i32, // ar zaporedje lahko podamo s zacetnim clenom in diferenco
    kvocient: i32,
    trenutni_clen: i32,
}
struct Pomozno {
    zacetni_clen: i32,
    diferenca: i32,
}

impl GeometrijskoZaporedje {
    fn new(zacetni: i32, dif: i32) -> GeometrijskoZaporedje {
        GeometrijskoZaporedje {
            zacetni_clen: zacetni,
            kvocient: dif,
            trenutni_clen: zacetni,
        }
    }
    fn next(&mut self) -> i32 {
        let clen = self.trenutni_clen;
        self.trenutni_clen *= self.kvocient;
        clen
    }
    fn n_th(&mut self, n: u32) -> i32 {
        self.zacetni_clen * i32::pow(self.kvocient, n - 1)
    }
    fn reset(&mut self) {
        self.trenutni_clen = self.zacetni_clen;
    }
    fn current(&self) -> i32 {
        self.trenutni_clen
    }
    fn sum(&self, n: i32) -> i32 {
        let mut vsota = 0;
        let mut pomozna = self.zacetni_clen;
        for _ in 0..n {
            vsota += pomozna;
            pomozna *= self.kvocient;
        }
        vsota
    }
    fn produkt(a: &GeometrijskoZaporedje, b: &GeometrijskoZaporedje) -> GeometrijskoZaporedje {
        GeometrijskoZaporedje {
            zacetni_clen: a.zacetni_clen * b.zacetni_clen,
            kvocient: a.kvocient * b.kvocient,
            trenutni_clen: a.zacetni_clen * b.zacetni_clen,
        }
    }
}

enum BinOperacija {
    Plus,
    Minus,
    Times,
    Pow,
}
enum UnOperacija {
    UnMinus,
}

enum Izraz {
    Konstanta(i32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
    UnarnaOperacija(UnOperacija, Box<Izraz>),
}

impl Izraz {
    fn eval(&self) -> i32 {
        match self {
            Izraz::Konstanta(vrednost) => *vrednost,
            Izraz::Operacija(konst1, oper, konst2) => {
                let leva = konst1.eval();
                let desna = konst2.eval();
                match oper {
                    BinOperacija::Plus => leva + desna,
                    BinOperacija::Minus => leva - desna,
                    BinOperacija::Pow => leva.pow( desna as u32),
                    BinOperacija::Times => leva * desna,
                }
            },
            Izraz::UnarnaOperacija(UnOperacija::UnMinus, izraz)=> -izraz.eval(),
        }
    }
    fn collect(&self) -> i32 {
        match self {
            Izraz::Konstanta(_) => 1,
            Izraz::Operacija(k1, _, k2) => k1.collect() + k2.collect(),
            Izraz::UnarnaOperacija(UnOperacija::UnMinus, izraz) =>izraz.collect(),
        }
    }
    fn izpis(&self) -> String {
        match self {
            Izraz::Konstanta(vrednost) => vrednost.to_string(),
            Izraz::Operacija(konst1, oper, konst2) => {
                let leva = konst1.izpis();
                let desna = konst2.izpis();
                let op = match oper {
                    BinOperacija::Plus => "+",
                    BinOperacija::Minus => "-",
                    BinOperacija::Pow => "**",
                    BinOperacija::Times => "*",
                };
                format!("({} {} {})", leva, op, desna)
            },
            Izraz::UnarnaOperacija(UnOperacija::UnMinus, izraz)=> format!("-{}",izraz.izpis()),
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
    let racun4 = Izraz::Operacija(
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(5)),
            BinOperacija::Pow,
            Box::new(Izraz::Konstanta(2)),
        )),
        BinOperacija::Plus,
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(3)),
            BinOperacija::Pow,
            Box::new(Izraz::Konstanta(2)),
        )),
    );
    let racun5 = Izraz::Operacija(
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(5)),
            BinOperacija::Times,
            Box::new(Izraz::Konstanta(2)),
        )),
        BinOperacija::Plus,
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(4)),
            BinOperacija::Pow,
            Box::new(Izraz::Konstanta(2)),
        )),
    );
    println!("{}", racun1.izpis());
    println!("{}", racun2.izpis());
    println!("{}", racun3.izpis());
    println!("{}", racun4.izpis());
    println!("{}", racun5.izpis());
}
