fn main() {
    let n: u128 = 5;
    let x: u128 = fibonacci(12);
    println!("{n}th fibonacci number is {x}");
}

fn fibonacci(n: u128) -> u128{
    if n <= 1{
        n
    }else{
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
