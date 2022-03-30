trait Clicky {
    fn click(&self) -> String;
}

struct Keyboard;
impl Clicky for Keyboard {
    fn click(&self) -> String {
        "Keyboard input".to_owned()
    }
}

struct Mouse;
impl Clicky for Mouse {
    fn click(&self) -> String {
        "Mouse click".to_owned()
    }
}

fn main () {
    let x:Box<dyn Clicky> = Box::new(Keyboard);
    let y:Box<dyn Clicky> = Box::new(Mouse);

    let clickers = vec![x,y];
    for i in clickers {
        if i.click() =="Mouse click".to_owned() {
            println!("This is mouse");
        } else {
            println!("This is not mouse");
        }
    }
}