fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("AAA!!!")
    }
    println!("I Love{}s!!!!", gift);
}
fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}
