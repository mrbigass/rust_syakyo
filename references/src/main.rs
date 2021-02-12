fn main() {
    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    match mut_value {
        ref mut m => {
            println!("memory address of 'mut_value': {}", m);
            *m += 10;
            println!("We added 10 . 'mut_value': {}", mut_value);
        },
    }
}
