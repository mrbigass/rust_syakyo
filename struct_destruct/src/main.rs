fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("first of x is 1, b = {}, y = {}", b, y),
        Foo { y: 2, x: i } => println!("{:?}", i),
        Foo { y, .. } => println!("we don't care about x, just for y: {}", y),
    }
}
