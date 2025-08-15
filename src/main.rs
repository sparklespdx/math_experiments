fn main() {

    // let n = 7901;
    // let foo = is_prime(n);

    // if foo {
    //     println!("{n} is prime");
    // } else {
    //     println!("{n} is not prime");
    // }

    println!("starting test.....");

    for n in 0..100 {

        println!(">> testing {n}");

        let foo = is_prime_wheel(n);

        if foo {
            println!("**** {n} is prime *****");
        } else {
            println!("^^^^ {n} is not prime");
        }

    }
}

fn is_prime(n: u32) -> bool {

    match n {
        0 => return false, // zero is not prime
        1 => return false, // one is also not prime
        2 => return true, // two is the first prime
        _ => {},
    }


    // // all even numbers except 2 are not prime
    if n % 2 == 0 {
        println!("even number, bailed early");
        return false;
    } else {
        // compute up to sqrt(n) rounded down, from 3.
        // should we go down from sqrt(n) instead?
        let sqrt_n = n.isqrt();

        // we only need to test odd numbers in the search space,
        // so we can cut our time in half per number by adjusting
        // the step of the iterator.
        for i in (3..sqrt_n).step_by(2) {
            if n % i == 0 {
                println!("searched up to {i}");
                return false;
            }
        }
        println!("PRIME: searched up to {sqrt_n}");
        return true;
    }
}

fn is_prime_wheel(n: u32) -> bool {
    match n {
        0 => return false, // zero is not prime
        1 => return false, // one is also not prime
        2 => return true, // two is the first prime
        3 => return true, // three is also prime and not covered below
        5 => return true, // also prime and should bail early
        _ => {},
    }

    // wheel factorization: all primes are 6k±1, this excludes
    // all multiples of 3 and 5 that are larger than 5.
    // TODO: i lied it still hits 25 it's wrong
    // 
    // >> testing 25
    // >>>> - testing up to sqrt(25)
    // **** 25 is prime *****
    if n % 6 != 1 && n % 6 != 5 {
        println!(">>>> - number is not 6k±1, early bail");
        return false;
    } else {
        // test all remaining known valid factors up to sqrt(n)
        println!(">>>> - testing up to sqrt({n})");
        let sqrt_n = n.isqrt();

        for i in (7..=sqrt_n).step_by(2) {
            if i % 3 != 0 && i % 5 != 0 {
                println!(">>>> ---- testing {i} as a factor");
                if n % i == 0 {
                    return false
                }
            }
        }

        return true;
    }
}