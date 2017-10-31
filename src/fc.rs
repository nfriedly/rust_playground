pub fn main() {
    println!("75f is {}c", ftoc(75.0));
    println!("20c is {}f", ctof(21.0));
    for n in 1..8 {
        println!("fib number {} is {}", n, fib(n));
    }
}
fn ftoc(f: f32) -> f32 {
    f - 32.0 * 5.0/9.0
}

fn ctof(c: f32) -> f32 {
    c * 9.0/5.0 + 32.0
}

fn fib(n: u32) -> u32 {
    if n <=1 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}
