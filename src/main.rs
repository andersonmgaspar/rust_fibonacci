
fn main() {
    println!("Hello, Fibonacci!");
    // Recursive fibonacci implementation
    for int in 0..10 {
        print!("{} ", fibonacci_re(int));
    }
    println!("");
    //Non recursive fibonacci implementation
    fibonacci(10);
}

pub fn fibonacci_re(n: u64) -> u64 {
    if n <= 0 {
        return 0;
    } else if n == 1{
        return 1;
    } else {
        return fibonacci_re(n-1) + fibonacci_re(n-2);
    }
}

pub fn fibonacci(count: i64) {
    let (mut n1, mut n2, mut n3): (i64, i64, i64) =(0,1,0);
    if count < 1 {
        print!("0");
    }else if count == 1 {
        println!("{}", n1);
    }else {
        print!("{0} {1}",n1, n2);
        for i in 2..count  {
            n3 = n1+n2;
            print!(" {n3}");
            n1 = n2;
            n2 = n3;
        }
    }
    println!("");
}