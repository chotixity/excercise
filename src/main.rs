fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    let mut x = 10;
    println!("Hello World {x}");
    x = 11;
    while x <= 20 {
        println!("this is {}", x);
        x +=1;
    }
    println!("this is the new x {x}");
    println!("The result is: {}", interproduct(120, 200, 248));
    let n = 20;
    println!("fibonacci({n}) = {}", fibonacci(n));
}
