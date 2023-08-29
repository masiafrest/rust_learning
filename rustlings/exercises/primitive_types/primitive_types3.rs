// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

fn main() {
    // let a: [char; 100] = ['a'; 100];
    let a: [i8; 100] = [-128; 100];
    // let a: [u8; 100] = [255; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
