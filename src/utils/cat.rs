pub struct Cat {
    pub name: String,
    pub id: i32,
    pub age: i32
}

impl Cat {
    pub fn new(name: &str, id: i32, age: i32) -> Self {
        println!("Котик под именем {} ID({}) и возрастом {} создался!😸🦀", name, id, age);
        Self {
            name: name.to_string(),
            id: id,
            age: age
            }
    }
}

