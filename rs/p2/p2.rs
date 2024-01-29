fn main() {
    let ceiling: u32 = 4000000;
    let sum: u32 = fibonacci_sequence_of_evens(1, 2, ceiling);
    
    println!("{}", sum);
}

fn fibonacci_sequence_of_evens(b: u32, a: u32, ceiling: u32) -> u32 {
    let mut w: u32;
    let mut z: u32 = 0;
    let mut y: u32 = b;
    let mut x: u32 = a;

    loop {
        w = y + x;
        y = x;
        x = w;
        if y >= ceiling {
            break;
        }
        z += is_even(y);
    }

    return z;
}

fn is_even(x: u32) -> u32 {
    if x % 2 == 0 {
        return x;
    }
    return 0;
}

