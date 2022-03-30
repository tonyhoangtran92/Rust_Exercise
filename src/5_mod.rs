use rand:: {RngCore, Rng, SeedableRng};

mod back_house {
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String
    }

    pub enum Salad {
        Soup, Salad,
    }
    impl Breakfast {
        pub fn monday(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), fruit: String::from ("Banana") }
        }
    }
}

mod front_house {
    pub mod hosting{
        fn add_to_waitlist() {
            
        }
    }
}
fn eat_at_restaurant() {
    let mut order = back_house::Breakfast::monday("Fish");
    order.toast = String::from("Chicken");

    let order2 = back_house::Breakfast {
        toast: String::from("Wheat"),
        fruit: String:: from("apple")
    };

    let order3 = back_house::Salad::Salad;
}
fn main () {

}