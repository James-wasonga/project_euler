//printing the largest palindrome with 3 digits 

//palindrome is a number that can be read from both ways e.g
//9009 -> it is the same when read both ways from right to left and left to right 


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
    println!("The largest Palindrome is {}",result);


}