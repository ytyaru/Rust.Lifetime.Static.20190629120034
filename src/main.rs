/*
 * Rustのライフタイム（'static）
 * CreatedAt: 2019-06-29
 */
fn main() {
    let s: &'static str;
    {
        s = "A";
    }
    println!("{}", s);
}

