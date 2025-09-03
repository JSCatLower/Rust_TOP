mod utils;
use utils::Cat;

fn main() {
    println!("Привет, котики!😸");
    let Barsik = Cat::new("Барсик", 1, 2);
    println!("Имя Барсика: {}", Barsik.name);
}
