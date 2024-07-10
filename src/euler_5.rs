//2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder. 
//What is the smallest positive number that is evenly divisible by all of the numbers from 1 to n

pub fn gcd(a: u64, b: u64) -> u64{
   if b == 0 {
        a
   } else {
    gcd(b, a % b)
   }
}

pub fn lcm(a: u64, b: u64) -> u64{
    (a * b) / gcd(a,b) 
}

pub fn smallest(n: u64) -> u64{
    (1..=n).fold(1, |acc, x| lcm(acc, x))
}