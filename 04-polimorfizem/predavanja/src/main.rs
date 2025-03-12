use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::Display;

fn prestej_i32(v: &Vec<i32>) -> i32 {
    let mut stevilo = 0;
    for _ in v {
        stevilo += 1;
    }
    stevilo
}

fn prestej_f64(v: &Vec<f64>) -> i32 {
    let mut stevilo = 0;
    for _ in v {
        stevilo += 1;
    }
    stevilo
}

fn prestej<T>(v: &Vec<T>) -> i32 {
    let mut stevilo = 0;
    for _ in v {
        stevilo += 1;
    }
    stevilo
}

fn najvecji<T: PartialOrd>(v: &Vec<T>) -> Option<&T> {
    let mut najvecji = None;
    for x in v {
        match najvecji {
            None => najvecji = Some(x),
            Some(m) => {
                if m < x {
                    najvecji = Some(x)
                }
            }
        }
    }
    najvecji
}

// enum Option<T> {
//     None,
//     Some(T)
// }

enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(PartialEq)]
struct Tocka<T, O> {
    x: T,
    y: T,
    oznaka: O,
}

impl<O, T> Tocka<T, O> {
    fn abscisa(&self) -> &T {
        &self.x
    }

    fn naredi_novo_tocko_z_oznako_drugega_tipa<P>(self, oznaka: P) -> Tocka<T, P> {
        Tocka {
            x: self.x,
            y: self.y,
            oznaka: oznaka,
        }
    }
}

impl<O> Tocka<f64, O> {
    fn absolutna_vrednost(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // fn abscisa(self) -> f64 {
    //     self.x
    // }
}

impl<O> Tocka<i32, O> {
    fn absolutna_vrednost(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

impl Tocka<f64, String> {
    fn to_string(&self) -> String {
        String::from("točka")
    }
}

impl<T: PartialEq, O: PartialEq> PartialOrd for Tocka<T, O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        None
    }
}

impl<T: ToString> ToString for Option<T> {
    fn to_string(&self) -> String {
        match self {
            None => String::from("None"),
            Some(x) => format!("Some({})", x.to_string()),
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let n1 = prestej(&v1);
    let n10 = prestej(&v1);
    println!("Dolžina prvega vektorja je {n1}");
    let v2 = vec![1., 2., 3., 4., 5., 6.];
    let n2 = prestej(&v2);
    println!("Dolžina drugega vektorja je {n2}");
    let p1 = Tocka {
        x: 3.0,
        y: 4.0,
        oznaka: String::from("A"),
    };
    let p2 = Tocka {
        x: 3.0,
        y: 4.0,
        oznaka: String::from("A"),
    };
    println!(
        "Absolutna vrednost točke {} je {}",
        p1.to_string(),
        p2.absolutna_vrednost()
    );
    let p = najvecji(&vec![p1, p2]);
    println!("Največja točka je {}", p.to_string());
}
