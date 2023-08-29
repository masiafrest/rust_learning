// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0 = Vec::new();

    // 1. https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-clone
    // vec is in the heap and stack, we want to deep copy
    let vec_clone = vec0.clone();
    let mut vec1 = fill_vec(vec_clone);

    // let mut vec1 = fill_vec(vec0); // let mut vec1 = fill_vec(vec0.clone())

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

//2.
// fn main() {
//     let vec0 = Vec::new();

//     // 1. https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-clone
//     // vec is in the heap and stack, we want to deep copy
//     // let vec_clone = vec0.clone();
//     // let mut vec1 = fill_vec(vec_clone);

//     //2.
//     let mut vec1 = fill_vec_borrow(&vec0);

//
//     // Do not change the following line!
// println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
//
// vec1.push(88);
//
//   println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

// fn fill_vec_borrow(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// -----------

//3.
// fn main() {
//   let mut vec0 = Vec::new();

//   // let mut vec1 = fill_vec_borrow(&vec0);

//   fill_vec(&mut vec0);

//   // Do not change the following line!
//   println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//   vec0.push(88);

//   println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
// }

// fn fill_vec(vec: &mut Vec<i32>) {
//   let mut vec = vec;

//   vec.push(22);
//   vec.push(44);
//   vec.push(66);

//   // vec
// }
