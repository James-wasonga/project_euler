pub fn fibo(n: u32){
    let mut fibonacci_sequence = Vec::new();
    let mut sum = 0; 

    if n == 0 {
        fibonacci_sequence.push(0);
    }else if n == 1 {
        fibonacci_sequence.push(1);
    }else{
        let mut a = 0;
        let mut b = 1;

        fibonacci_sequence.push(a);
        fibonacci_sequence.push(b);

        for _ in 2..=n{
            let  c = a + b;
            fibonacci_sequence.push(c);
            a = b;
            b = c;
        }
    }

    for i in fibonacci_sequence.iter(){
        if i % 2 == 0{
            sum += i;
        }
        println!("{}",i);
    }
    println!("The sum of fibonnacci is {}",sum)
}