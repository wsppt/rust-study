fn main() {
    let a:i32 = 10;
    let b:u16 = 100;

    if a < (b as i32) {  // 小转化大的类型是安全的
        println!("Ten is less than one hundred.")
    }
}
