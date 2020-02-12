/// Right Justified:
/// |     foo     bar|
///
/// Left Justified:
/// |foo     bar     |
fn main() {
    println!(
        "Right Justified:\n|{:>width$}{:>width$}|",
        "foo",
        "bar",
        width = 8
    );
    println!("");
    println!(
        "Left Justified:\n|{:<width$}{:<width$}|",
        "foo",
        "bar",
        width = 8
    );
}
