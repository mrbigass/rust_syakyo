fn main() {
    fn age() -> u32 {
        12
    }

    match age() {
        n @ 0..=12 => println!("You are at {}", n),
        n @ 13..=19 => println!("You are teen age: {}", n),
        n => println!("{}", n),
    }
}
