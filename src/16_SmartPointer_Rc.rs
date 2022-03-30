//smart pointer
use std::rc::Rc;

#[derive(Debug)]
struct Car {
    number: String,
}

#[derive(Debug)]
struct Door {
    vehicle: Rc<Car>,
}

fn main () {
    let car = Rc::new(Car {
        number: "43A-44551".to_owned(),
    });
    let left_door = Door {
        vehicle: Rc::clone(&car),
    };

    let right_door = Door {
        vehicle: Rc::clone(&car),
    };
    drop(car);
    println!("{:?}", left_door.vehicle);
    println!("{:?}", right_door.vehicle);
}