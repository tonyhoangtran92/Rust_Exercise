struct HINHCHUNHAT {
    dai: u32,
    rong: u32
}

impl HINHCHUNHAT {
    fn dien_tich(&self) -> u32 {
        self.dai * self.rong
    }
}

fn main () {
    let kichthuoc = HINHCHUNHAT {dai: 30, rong: 20};
    println!("dien tich hinh chu nhat la {}", kichthuoc.dien_tich());
}