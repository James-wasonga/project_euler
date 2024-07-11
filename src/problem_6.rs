pub fn sum_square(n: u64) -> u64{
    (1..=n).map(|x| x * x).sum()
}

pub fn sum(n: u64) -> u64{
    let sum: u64 = (1..=n).sum();
    sum * sum
    
} 