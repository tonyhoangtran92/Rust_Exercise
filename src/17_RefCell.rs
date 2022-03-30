//RefCell

use std::cell::RefCell;

struct Channel {
    name: RefCell<String>,
}
fn main() {
    let mychannel = Channel {
        name: RefCell::new("Tony channel".to_owned()),
    };

    {
        let mut a = mychannel.name.borrow_mut();
        *a = "Tony Poker Channel".to_owned();
    }

    {
        mychannel.name.replace("Tony VPS channel".to_owned());
    }

    println!("My channel: {:?}", mychannel.name);
}