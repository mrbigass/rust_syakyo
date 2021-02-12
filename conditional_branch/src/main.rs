fn main() {
    'outer: loop {
        println!("Enterd the outer loop");

        'inner: loop {
            println!("Enterd the inner loop");
            // break; // inner loopを中断

            break 'outer; // outer loopを中断
        }
    }
}
