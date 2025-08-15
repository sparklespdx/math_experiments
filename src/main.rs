fn main() {

    let foo = is_prime(7867);

    if foo {
        println!("foo is prime");
    } else {
        println!("foo is not prime");
    }
}

fn is_prime(n: u32) -> bool {
    match n {
        0 => return false, // zero is not prime
        1 => return false, // one is also not prime
        2 => return true, // two is the first prime
        _ => {},
    }

    // all even numbers except 2 are not prime
    if n % 2 == 0 {
        return false;
    } else {
        // compute up to sqrt(n) rounded down, from 3.
        // should we go down from sqrt(n) instead?
        let sqrt_n = n.isqrt();

        for i in 3..sqrt_n {
            if n % i == 0 {
                return false;
            }
        }
        return true;
    }
}