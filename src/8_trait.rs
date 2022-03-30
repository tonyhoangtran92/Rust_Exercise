//trait
struct Data {
    num1: u32,
    num2: u32,
    str1: String,
    optional: Option<i32>,
}

impl Data {
    fn new() -> Self {
        Data {
            num1: 20,
            num2: 30,
            str1: "Some string now".to_string(),
            optional: None,
        }
    }
}

trait Transform {
    fn revert(&self) -> String;
}

impl Transform for Data {
    fn revert(&self) -> String {
        self.str1.chars().rev().collect::<String>()
    }
}
fn main () {
    let a = Data::new();
    println!("{}", a.revert());
}