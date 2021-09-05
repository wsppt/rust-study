fn other_fun(){
    println!("This is a function");
}

fn other_fun1(a: i32, b: u32) {
    println!("a = {}, b = {}", a, b);
}

fn other_fun2(a: i32, b: i32) -> i32{
    let result = a + b;
    return result;
}

fn other_fun3(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    other_fun();
    let a: i32 = -1;
    let b: u32 = 2;
    other_fun1(a, b);
    let c: i32 =9;
    let r:i32 = other_fun2(a, c);
    println!("result={}", r);
    let r1:i32 = other_fun3(a, c);
    println!("r1={}", r1);
    println!("Hello, world!");
}
