fn main() {

    let is_true = true;
    let is_false = false;

    println!("is false={},{}", is_false, is_true);

    let a = 'a';
    println!("a={}", a);

    let b = '你';
    println!("b={}", b);
    // i8, i16, i32, i64, u8, u16, u32, u64
    let c: i8 = -111;
    println!("c={}", c);

    let d: f32 = 0.009;
    println!("d={}", d);

    // 自适应类型isize, usize
    println!("max = {}", usize::max_value());


}
