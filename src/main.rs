use std::ops::Index;

enum Vinfast {
    Vf6,
    Vf7,
    Vf8,
    Vf9,
}

struct Price {
    price_Vf6: i64,
    price_Vf7: i64,
    price_Vf8: i64,
    price_Vf9: i64,
}

impl Index<Vinfast> for Price {
    type Output = i64;

    fn index(&self, brand:Vinfast) -> &Self::Output {
        match brand {
            Vinfast::Vf6 => &self.price_Vf6,
            Vinfast::Vf7 => &self.price_Vf7,
            Vinfast::Vf8 => &self.price_Vf8,
            Vinfast::Vf9 => &self.price_Vf9,
        }
    }
}

fn main() {
    let price = Price{
        price_Vf6: 40000,
        price_Vf7: 45000,
        price_Vf8: 50000,
        price_Vf9: 55000,
    };
    let find = price[Vinfast::Vf8];
    println!("VF8 price is: {}$",find);
}