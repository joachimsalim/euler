fn main() {
    println!("{}", multiples_of_three_or_five(0, 3, 1000))
}

fn multiples_of_three_or_five(mut sum: i32, start: i32, end: i32) -> i32 {
    for x in start..end {
        sum += multiple(x, 3) | multiple(x, 5)
    }
    sum
}

fn multiple(multiplier: i32, multiplicand: i32) -> i32 {
    if evenly_divisible(multiplier, multiplicand) {
        multiplier
    } else {
        0
    }
}

fn evenly_divisible(dividend: i32, divisor: i32) -> bool {
    dividend % divisor == 0
}

