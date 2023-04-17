// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0); // ownership from vec0 is given to fill_vec

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec; // we assing ownership from vec0 to vec

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec // we return an ownership
}
