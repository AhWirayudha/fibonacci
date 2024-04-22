use std::io;

fn main() {
    println!("Wellcome to Fibonacci app!");
    println!("Please enter a number n of fibonacci to generate:");
    
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");
    println!("You entered: {}", n);

    print_fibonacci(n);
    
}

fn fibonacci(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// generate fibonacci sequence and print it
// 0, 1, 1, 2, 3, 5, 8
fn print_fibonacci(n: i32) {
    for i in 0..n {
        println!("{}", fibonacci(i));
    }
}