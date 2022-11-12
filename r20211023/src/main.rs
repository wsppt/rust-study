use std::io;

fn main() {
    // println!("猜数!");
    // println!("猜测一个数");
    // // let mut foo =1;
    // // let bar = foo;
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("无法读取行");
    // println!("你猜测的数是：{}", guess)
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner        scope is:{}", x);
    }


    println!("The value of x is:{}", x);

    println!("This is test :{}", x + 5);

}
