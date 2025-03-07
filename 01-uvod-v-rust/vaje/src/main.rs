use std::io;
/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(a0: u32, a1: u32, n: u32) -> u32 {
    let mut prvi = a0;
    let mut drugi = a1;
    let mut counter = 0;
    loop {
        if counter >= n {
            return prvi;
        }
        let vsota = prvi + drugi;
        prvi = drugi;
        drugi = vsota;
        counter += 1;
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno
fn je_prestopno(leto: u32) -> bool {
    let leto = leto;
    (leto % 4 == 0 && leto % 100 != 0) || leto % 400 != 0
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);

fn je_veljaven_datum(datum: Date) -> bool {
    let (dan, mesec, leto) = datum;

    match mesec {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => dan <= 31,
        4 | 6 | 9 | 11 => dan <= 30,
        2 => {
            if je_prestopno(leto) {
                dan <= 29
            } else {
                dan <= 28
            }
        }
        _ => false,
    }
}
/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

fn iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    loop {
        if cond(start) {
            return start;
        }
        start = fun(start)
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b] glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

fn bisekcija(mut a: f64, mut b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    loop {
        let c = (a + b) / 2.0;

        let f_c = fun(c);

        if f_c.abs() < prec {
            return c;
        }
        if fun(a) * f_c < 0.0 {
            b = c
        } else {
            a = c
        }
    }
}
/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.

fn guessing_game() {
    println!("Vnesi številko!");

    let mut prejsnji_vnos = None;
    let mut razlika = None;

    loop {
        let mut vnos = String::new();

        println!("Vnesi število.");

        io::stdin()
            .read_line(&mut vnos)
            .expect("Failed to read line");

        match vnos.trim().parse::<i32>() {
            Ok(stevilo) => {
                if prejsnji_vnos.is_none() {
                    prejsnji_vnos = Some(stevilo);
                    continue;
                }

                let trenutna_razlika = stevilo - prejsnji_vnos.unwrap();

                if razlika.is_none() {
                    razlika = Some(trenutna_razlika);
                } else if razlika.unwrap() != trenutna_razlika {
                    println!("Prekinjeno zaporedje. Vnesel si {}", stevilo);
                    break;
                }

                prejsnji_vnos = Some(stevilo);
            }

            Err(_) => {
                println!("neveljaven vnos");
                continue;
            }
        }
    }
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    let mut rezultat = [[0; 2]; 2];
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    loop {
        rezultat[i][j] = a[i][j] * b[j][k];

        k += 1;

        if k == 2 {
            k = 0;
            j += 1;
        }

        if j == 2 {
            j = 0;
            i += 1
        }

        if i == 2 {
            break;
        }
    }

    rezultat
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    if arr.len() <= 2 {
        return true;
    }

    let narascajoce = arr[0] <= arr[1];
    let mut i = 0;

    loop {
        if i == arr.len() - 2 {
            return true;
        }

        if narascajoce {
            if arr[i] > arr[i + 1] {
                return false;
            }
        } else {
            if arr[i] < arr[i + 1] {
                return false;
            }
        }
        i += 1;
    }
}

fn vsebuje<T: PartialEq>(v: &Vec<T>, x: &T) -> bool {
    for y in v {
        if x == y {
            return true;
        }
    }
    return false;
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`
/// ------------------------------------------------------------------------------------------------
fn pow(mut x: u32, mut n: u32) -> u32 {
    if n == 0 {
        1;
    }
    let polovica = pow(x, n / 2);
    if n % 2 == 1 {
        x * polovica * polovica
    } else {
        polovica * polovica
    }
}

/// Prepišite hitro potenciranje v iterativno obliko

fn pow_it(mut x: u32, mut n: u32) -> u32 {
    let mut rezultat = 1;
    loop {
        if n % 2 == 1 {
            rezultat *= x;
        }
        x *= x;
        n /= 2;
        if n == 0 {
            break;
        }
    }
    rezultat
}
/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`
fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32 {
    let mut rezultat = 1;
    loop {
        if n % 2 == 1 {
            rezultat *= x % m;
        }
        x *= x % m;
        n /= 2;
        if n == 0 {
            break;
        }
    }
    rezultat
}
/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(arr: &mut [u32]) {
    let dolzina = arr.len();
    let mut i = 0;
    loop {
        if i >= dolzina - 1 {
            break;
        }
        let mut indeks_najmansega = i;
        let mut j = i + 1;
        loop {
            if j >= dolzina {
                break;
            }
            if arr[i] < arr[indeks_najmansega] {
                indeks_najmansega = j;
            }
            j += 1
        }
        arr.swap(i, indeks_najmansega);

        i += 1;
    }
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn pyramid(n: u32) {
    let mut stevec = 0;
    loop {
        if stevec > n {
            break;
        }

        let mut j = 0;
        loop {
            if j > n - stevec - 1 {
                break;
            }
            print!("  ");
            j += 1;
        }

        let mut k = 0;
        loop {
            if k > 2 * stevec + 1 {
                break;
            }
            print!("*");
            k += 1;
        }
        println!();
        stevec += 1
    }
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahkom predpostavite, da bo n največ 26.
///      A
///    A B A
///   A B C B A
/// A B C D C B A

fn becedna_piramida(n: u32) {
    let mut stevec = 0;
    loop {
        if stevec > n {
            break;
        }

        let mut presledki = 0;
        loop {
            if presledki >= (n - stevec - 1) {
                break;
            }
            print!("  ");
            presledki += 1;
        }

        let mut j = 0;
        loop {
            if j > stevec {
                break;
            }
            let crka = (b'A' + j as u8) as char;
            print!("{} ", crka);
            j += 1;
        }

        let mut j = stevec as i32 - 1;

        loop {
            if j < 0 {
                break;
            }
            let letter = (b'A' + j as u8) as char;
            print!("{} ", letter);
            j -= 1;
        }

        println!();
        stevec += 1
    }
}

///
///
///
///
///
///
fn naslednji(n: u32) -> u32 {
    n + 1
}

fn je_sodo(n: u32) -> bool {
    n % 16 == 0
}

fn main() {
    println!("{}", fib(0, 1, 15));
    println!("{}", je_veljaven_datum((31, 5, 5)));
    let rezultat = iteracija(3, naslednji, je_sodo);
    println!("Prvo sodo število: {}", rezultat);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        assert_eq!(result, ());
    }

    #[test]
    fn test_fib() {}
}
