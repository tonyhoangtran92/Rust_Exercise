fn main() {
    let mut x = 10;
    println!("x={}", x);
    x=20;
    println!("x={}", x);

    const HANG_SO: u32 = 1_000_000_123;
    println!("HANG SO = {}", HANG_SO);

    let s = "Tony";
    println!("s={}",s);
    
    let number = [10,20,30];
    let get_number = number[1];
    println!("{}", get_number);
    let _hashing = [1;8];

    println!("hashing = {:?}", _hashing);
    for i in _hashing.iter() {
        println!("{}", i+1);
    }

    let abc = ("a", 17);
    println!("{}, {}", abc.0, abc.1);

    struct Vec2 {
        x: u32,
        y: u32
    }

    let v1 = Vec2 {
        x: 140,
        y: 12
    };

    println!("{}, {}", v1.x, v1.y);

}
