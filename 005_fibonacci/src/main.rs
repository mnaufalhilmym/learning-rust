use std::io;

fn main() {
    println!("Fibo: ");

    let mut fib = String::new();

    io::stdin()
        .read_line(&mut fib)
        .expect("Failed to read fibo");

    let fib: u32 = match fib.trim().parse() {
        Ok(val) => val,
        Err(_) => panic!(),
    };

    println!("{}", fibo(fib));
}

fn fibo(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return fibo(n - 1) + fibo(n - 2);
}
