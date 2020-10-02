#[derive(Debug)]
struct MaxValues {
    pub a: isize,
    pub b: isize,
    pub n: isize,
}

#[derive(Clone, Copy)]
enum IsPrime {
    Prime,
    NotPrime,
}

impl IsPrime {
    pub fn is_prime(&self) -> bool {
        match self {
            Self::Prime => true,

            Self::NotPrime => false,
        }
    }
}

fn is_number_prime(i: isize, sieve: &Vec<IsPrime>) -> bool {
    if i < 0 {
        false
    }
    else {
        sieve[i as usize].is_prime()
    }
}

fn main() {
    let mut sieve = vec![IsPrime::Prime; 1_000_000];

    for i in 2..999_999_usize {
        if is_number_prime(i as isize, &sieve) {
            let mut i_m = i * 2;

            while i_m <= 999_999 {
                sieve[i_m] = IsPrime::NotPrime;

                i_m += i;
            }
        }
    }

    let mut maxes = MaxValues { a: 0, b: 0, n: 0 };

    for a in -999..999 {
        for b in -1000..1000 {
            let mut n: isize = 0;

            while is_number_prime(n.pow(2) + a*n + b, &sieve) {
                n += 1;
            }

            if n > maxes.n {
                maxes.n = n;
                maxes.a = a;
                maxes.b = b;
            }
        }
    }

    println!("{:?}", maxes);
}
