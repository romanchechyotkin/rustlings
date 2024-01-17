// primitive_types2.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)

fn main() {
    // Characters (`char`)

    let my_first_initial = 'C';
    println!("{}", check_char(my_first_initial));
    println!("{}", check_char('1'));
}

fn check_char(ch: char) -> &'static str {
    if ch.is_alphabetic() {
        "Alphabetical!"
    } else if ch.is_numeric() {
        "Numerical!"
    } else {
        "Neither alphabetic nor numeric!"
    }
}
