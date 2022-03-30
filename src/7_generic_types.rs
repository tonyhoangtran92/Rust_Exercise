struct point <T,U> {
    x: T,
    y: U
}

impl <T,U> point<T,U> {
    fn mixed<V,W> (self, other:point<V, W>) -> point<T,W> {
        point {
            x: self.x, 
            y: other.y }
    }
}
fn main () {
    let p1 = point{x: 5, y: 10};
    let p2 = point {x:10, y: 20.4};
    println!("{} {}", p2.x, p2.y);

    let p3 = p1.mixed(p2);
    println!("{} {}", p3.x, p3.y)
}