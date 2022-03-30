fn main() {

    //ownership
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3=takes_and_gives_back(s2);

    println!("{}, {}", s1, s3);

    //tham chieu reference
    let mut s4 = String::from("hello");
    let len = calculate_length(&mut s4);
    println!("{} - {}", s4, len);
}

fn gives_ownership() -> String {
    let some_string= String::from("Hello");
    some_string
}

fn takes_and_gives_back(some_string : String) -> String {
    some_string
}

fn calculate_length(some_string: &mut String) -> usize {
    some_string.push_str(" World");
    let length = some_string.len();
    length
}