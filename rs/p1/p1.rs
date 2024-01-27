fn main() {
    let bound: i32 = 1000;
    let sum: i32 = multiples_of_threes_or_fives(bound);
    
    println!("{}", sum);
}

fn multiples_of_threes_or_fives(bound: i32) -> i32 {
    let mut sum: i32 = 0;

    for x in 0i32..bound {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }
    
    return sum;
}
