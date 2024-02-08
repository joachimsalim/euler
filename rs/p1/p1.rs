fn main() {
    println!("{}", multiples_of_three_or_five(3, 1000, 0))
}

fn multiples_of_three_or_five(start: i32, end: i32, mut sum: i32) -> i32 {
    for x in start..end {
        sum += multiple_of_factor(x, 3) | multiple_of_factor(x, 5)
    }
    sum
}

fn multiple_of_factor(multiple: i32, factor: i32) -> i32 {
    if is_evenly_divisible(multiple, factor) {
        multiple
    } else {
        0
    }
}

fn is_evenly_divisible(multiple: i32, factor: i32) -> bool {
    multiple % factor == 0
}
