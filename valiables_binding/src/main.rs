fn main() {
    let long_lived_binding = 1;

    // ここから下がmainより小さいスコープを持つ
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        // スコープ外の同名の変数をシャドーイングする
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    } // 小スコープの終わり

    // error occured
    // println!("outer short: {}", short_lived_binding);

    // innerで書かれたことは持ち出されない
    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}
