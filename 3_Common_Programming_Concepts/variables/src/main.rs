fn main() {
    println!("Hello World!");
    another_function();
    let x  = fib(30);
    println!("fib(30): {x}")
}
fn another_function(){
    println!("Hello THere!");
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    } else {
        return fib(n-1) + fib(n-2)
    }
}