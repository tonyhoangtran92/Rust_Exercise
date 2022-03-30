#[derive(Debug)]
enum IpaddressKind {
    V4(u8,u8,u8,u8),
    V6(String)
}

#[derive(Debug)]    //debug để in ra giá trị chung của enum và struct
struct _Ipaddress {
    kind: IpaddressKind,
    address: String
}

impl _Ipaddress {
}


fn main () {
    let localhost = IpaddressKind::V4 (127,0,0,1);

    println!("Local Host = {:#?}", localhost);

    //Option Enumm
    let _number = Some(5);
    let _string = Some("a String");

    let x = 6;
    let y = Some(5);
    let sum = x + y.unwrap_or(0);
    println!("Sum = {}", sum);

}