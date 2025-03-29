fn fibonacci(n: u64) -> u64 {
    if (n == 0) | (n == 1) {
        n
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

fn main() {
    for n in 0..10 {
        let fib = fibonacci(n);
        println!("Fibonacci({n}) = {fib}");
    }
}
