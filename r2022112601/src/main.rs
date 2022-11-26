use std::thread;

fn main() {
    let mut data = 100;
    // 线程的操作由操作系统决定，而不是由应用程序决定
    thread::spawn(|| {data = 500; });
    thread::spawn(|| {data = 1000;});

    println!("{}", data);
}
