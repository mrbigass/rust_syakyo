struct Rect {
    height: u32,
    width: u32,
}

trait Printable {
    fn printable(&self);
}

impl Printable for Rect {
    fn printable(&self) {
        println!("width: {}, height: {}", self.width, self.height);
    }
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

    square.printable();
    println!("{}", square.area());
}
