//lifetime
fn main () {
    
}


fn test<'a, 'b: 'a> (param1: i32, param2: i32, param3: &'a str, param4: &'a str) -> &'a str {
    if param1>param2 {
        param3
    } else {
        param4
    }
}