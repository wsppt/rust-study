#[derive(Debug)]

enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    println!("{:?}", grains);
    drop(grains);
}
