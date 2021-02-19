struct Rect {
    height: u32,
    width: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let square = Rect {
        height: 2,
        width: 4,
    };

    println!("area of square: {}", square.area());
}
