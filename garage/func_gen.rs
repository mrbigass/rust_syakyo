struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {} // Aという型を与えられたSGen<T> 関数名の直後に`<a>`という型パラメータがジェネリックあることを宣言していないので、この関数はAをジェネリックとして取らない
fn gen_spec_i32(_s: SGen<i32>) {} // こいつも上と同様
fn generic<T>(_s: SGen<T>) {} // <T>がSGen<T>に先行しているためTに対してジェネリック

fn main() {
    reg_fn(S(A)); // 具象型
    gen_spec_t(SGen(A)); // 型パラメータを暗黙のうちに受け取る
    gen_spec_i32(SGen(6)); // 型パラメータを暗黙のうちに受け取る
    generic::<char>(SGen('a')); // 明確にgeneric()へ型を宣言している
    generic(SGen('c')); // 暗黙的に決定
}
