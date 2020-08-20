fn fib(x: u32) -> u32 {
    if x < 2 {
        1
    } else {
        fib(x - 1) + fib(x - 2)
    }
}

fn main() {
    println!("{}", fib(40));
}
