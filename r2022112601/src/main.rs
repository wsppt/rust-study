// use std::convert::TryInto;

fn main() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);

    assert_eq!(abc.0 + abc.1, abc.2);

    println!("{:?}", abc);
}
