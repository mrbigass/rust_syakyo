fn main() {
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    // 外で変数の存在を宣言したので小スコープで代入をしても引き継がれる
    println!("a binding: {}", a_binding);

    let another_binding;

    // println!("Another binding: {}", another_binding);

    another_binding = 1;

    println!("another bingind: {}", another_binding);
}
