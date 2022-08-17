use std::{cmp::Ordering, io};

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
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_re(n - 1) + fibonacci_re(n - 2)
    }
}

pub fn fibonacci(count: u64) {
    let (mut n1, mut n2): (u64, u64) = (0, 1);
    match count.cmp(&1) {
        Ordering::Less => print!("0"),
        Ordering::Equal => println!("{}", n1),
        Ordering::Greater => {
            print!("{0} {1}", n1, n2);
            for _i in 2..count {
                let n3 = n1 + n2;
                print!(" {n3}");
                n1 = n2;
                n2 = n3;
            }
        }
    }
    println!();
}
