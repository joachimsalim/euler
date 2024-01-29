fn main() {
    let ceiling: i32 = 1000;
    let sum: i32 = multiples_of_three_or_five(ceiling);
    
    println!("{}", sum);
}

fn multiples_of_three_or_five(ceiling: i32) -> i32 {
    let mut y: i32 = 0;

    for x in 3i32..ceiling {
        if x % 3 == 0 || x % 5 == 0 {
            y += x;
        }
    }
    
    return y;
}

