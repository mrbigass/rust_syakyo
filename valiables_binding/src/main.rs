fn main() {
    let mut _mutable_integer = 7_i32;

    {
        // immutableにシャドーイング
        let _mutable_integer = _mutable_integer;

        // このスコープではフリーズしている
        // _mutable_integer += 50;
    } // _mutable_integerがスコープを抜ける

    _mutable_integer = 3;

    println!("mutable in base scope: {}", _mutable_integer);
}
