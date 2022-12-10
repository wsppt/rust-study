// use num::complex::Complex;

// cargo install cargo-edit
// cargo add num

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // let  a = Complex{ re: (2.1), im: (-1.2), };
    // let b = Complex::new(11.1, 22.2);
    //
    // let result = a + b;
    //
    // println!("{} + {}i", result.re, result.im);
    // let collection = [1, 2, 3, 4, 5];
    // for i in 0..collection.len() {
    //     let item = collection[i];
    //     println!("{}", item);
    // }

    let a = [10, 20, 30, 40, 50];
    for elemnt in a.iter() {
        println!("The value is: {}", elemnt);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }

    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let ss = "dddddd";
    print_type_of(&ss);
}
