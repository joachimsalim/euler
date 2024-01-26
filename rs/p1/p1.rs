fn main() {
    println!("{}", sum_of_threes_or_fives());
}

fn sum_of_threes_or_fives() -> i32 {
    let mut sum: i32 = 0;

    for x in 0i32..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }
    
    return sum;
}
