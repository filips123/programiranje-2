use core::panic;

/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(a0: u32, a1: u32, n: u32) -> u32 {
    match n {
        0 => a0,
        1 => a1,
        n => fib(a1, a0 + a1, n - 1)
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

fn je_prestopno(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);

fn je_veljaven_datum((day, month, year): Date) -> bool {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 13 => day >= 1 && day <= 31,
        4 | 6 | 9 | 11 => day >= 1 && day <= 30,
        2 => day >= 1 && day <= if je_prestopno(year) { 29 } else { 28 },
        _ => false
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

fn iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    match cond(start) {
        true => start,
        false => iteracija(fun(start), fun, cond)
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
    let c = (a + b) / 2.;

    if fun(c).abs() < prec || (b - a) < prec {
        return c;
    }

    if fun(a) * fun(c) < 0. {
        bisekcija(a, c, fun, prec)
    } else {
        bisekcija(c, b, fun, prec)
    }
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.

fn guessing_game() {
    let mut first = None;
    let mut second = None;
    let mut diff = None;

    loop {
        let mut number = String::new();

        std::io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match first {
            Some(first) => match second {
                Some(second) => {
                    if guess - second != diff {
                        println!("Ni več aritmetično zaporedje");
                        return;
                    } else {
                        first = second;
                        second = Some(number);
                        diff = Some(number - first);
                    }

                },
                None => {
                    second = Some(number);
                    diff = Some(number - first);
                },
            },
            None => first = Some(number),
        }
    }
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    panic!("Not implemented");
}

fn vsebuje<T : PartialEq>(v: &Vec<T>, x : &T) -> bool {
    for y in v {
      if x == y {
        return true
      }
    }
    return false
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(arr: &mut [u32]) {}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn pyramid(n: u32) {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahkom predpostavite, da bo n največ 26.
///      A
///    A B A
///   A B C B A
/// A B C D C B A
/// Napišite funkcijo `fn selection_sort(mut arr: [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn main() {
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
    fn test_fib() {
        assert_eq!(fib(1, 2, 0), 1);
        assert_eq!(fib(1, 2, 1), 2);
        assert_eq!(fib(1, 2, 2), 3);
        assert_eq!(fib(1, 2, 3), 5);
        assert_eq!(fib(1, 2, 4), 8);
    }

    #[test]
    fn test_je_prestopno() {
        assert_eq!(je_prestopno(2000), true);
        assert_eq!(je_prestopno(2004), true);
        assert_eq!(je_prestopno(2008), true);
        assert_eq!(je_prestopno(2100), false);
        assert_eq!(je_prestopno(2200), false);
        assert_eq!(je_prestopno(2400), true);
    }

    #[test]
    fn test_je_veljaven_datum() {
        assert_eq!(je_veljaven_datum((1, 0, 2000)), false);
        assert_eq!(je_veljaven_datum((0, 1, 2000)), false);
        assert_eq!(je_veljaven_datum((1, 1, 2000)), true);
        assert_eq!(je_veljaven_datum((31, 1, 2000)), true);
        assert_eq!(je_veljaven_datum((32, 1, 2000)), false);
        assert_eq!(je_veljaven_datum((29, 2, 2000)), true);
        assert_eq!(je_veljaven_datum((29, 2, 2001)), false);
    }
}
