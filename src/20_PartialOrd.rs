use std::cmp::{Ordering, PartialOrd};

#[derive(PartialEq,)]

struct User {
    id: i32, 
    name: String,
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &User) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

fn main() {
    let a = User {
        id: 1,
        name: "Tony".to_owned(),
    };
    let b = User {
        id: 2,
        name: "Corn".to_owned(),
    };

    let c = a.partial_cmp(&b);
    println!("{:?}", c);
}