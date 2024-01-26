fn main() {
    println!("{}", fibonacci_sequence(1, 2, 4000000));
}

fn fibonacci_sequence(a: u32, b: u32, limit: u32) -> u32 {
   let mut sum: u32 = 0;
    let mut x: u32 = a;
    let mut y: u32 = b;
    let mut z: u32;

    for _i in 0u32..limit {
        z = x + y;
        x = y;
        y = z;
        if x >= limit {
            break;
        }
        sum += evaluate(x);
    }

    return sum;
}

fn evaluate(x: u32) -> u32 {
    if x % 2 == 0 {
        return x;
    }
    return 0;
}
