fn main() {
    println!("{}", multiples_of_three_or_five(3, 1000, 0))
}

fn multiples_of_three_or_five(start: i32, end: i32, mut sum: i32) -> i32 {
    for x in start..end {
        sum += three(x) | five(x)
    }
    sum
}

fn three(multiple: i32) -> i32 {
    if multiple % 3 == 0 {
        multiple
    } else {
        0
    }
}

fn five(multiple: i32) -> i32 {
    if multiple % 5 == 0 {
        multiple
    } else {
        0
    }
}
