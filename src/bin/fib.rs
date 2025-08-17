fn main() {
    let thing = fib_r(3);
    println!("result is {thing}");
}

// iterative implementation using simple loop
fn fib(n: u64) -> u64 {

    if n <= 1 {return n;}

    let mut x: u64 = 0;
    let mut y: u64 = 1;
    let mut z: u64;

    let mut i: u64 = 0;
    loop {
        z = x + y;

        if i >= n - 2 {return z;}
        i += 1;

        x = y;
        y = z;
    }

}

// recursive implementation
// I think it's really interesting how when you map this out by hand, it looks like a tree.
fn fib_r(n: u64) -> u64 {
    match n {
        0 => n,
        1 => n,
        _ => fib_r(n - 1) + fib_r(n - 2),
    }
}
