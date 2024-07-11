//printing the largest palindrome with 3 digits 

//palindrome is a number that can be read from both ways e.g
//9009 -> it is the same when read both ways from right to left and left to right 
mod fibonacci;
use fibonacci::fibo;

mod euler_5;
use euler_5::smallest;


mod problem_6;
use problem_6::{sum_square,sum};

//function to check for the palindromee

fn my_palindrome(n: u32) -> bool{
    let my_string = n.to_string();
    let my_reverse: String = my_string.chars().rev().collect();
    my_string == my_reverse
}
fn largest_palindrome() -> u32{
    let mut max_palindrome = 0;
  

for i in 100..1000{
    for j in 100..1000{
        let product = i * j;
        if my_palindrome(product){
            if product > max_palindrome{
                max_palindrome = product
            }
        }
    }
}

max_palindrome
}

fn main(){
    let result = largest_palindrome();
    println!("The largest Palindrome is {}", result);
    

    //calling fibonacci function

     fibo(12);


     
        let n = 20;
        let result = smallest(n);
        println!("The smallest number that is evenly divisible by all of the numbers from 1 to {} is: {}", n, result);

        let number = 100;
        let result = sum_square(number);
        println!("The sum of first {} is {}", number,result );

        let res = sum(number);
        println!("The square of the sum of first {} are {}",number,res);

        let diff = res - result;
        println!("The difference is {}",diff);


}