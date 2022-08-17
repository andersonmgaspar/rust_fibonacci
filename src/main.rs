use std::{io, ptr::null};

fn main() {
    println!("Hello, Fibonacci! (Exit with ctrl+c)");
    loop {
        println!("Please input your fibonacci range:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your input");
        let input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Recursive Fibonacci Implementation: ");
        for int in 0..input {
            print!("{} ", fibonacci_re(int));
        }
        println!();
        println!("Non Recursive Fibonacci Implementation: ");
        fibonacci(input);
    }
}

pub fn fibonacci_re(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci_re(n - 1) + fibonacci_re(n - 2);
    }
}

pub fn fibonacci(count: u64) {
    let (mut n1, mut n2, mut n3): (u64, u64, u64) = (0, 1, 0);
    if count < 1 {
        print!("0");
    } else if count == 1 {
        println!("{}", n1);
    } else {
        print!("{0} {1}", n1, n2);
        for _i in 2..count {
            n3 = n1 + n2;
            print!(" {n3}");
            n1 = n2;
            n2 = n3;
        }
    }
    println!();
}
