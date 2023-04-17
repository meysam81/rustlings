// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut vec0 = Vec::new();

    let mut vec1_copy = fill_vec_copy(vec0.clone());

    // 1. Make another, separate version of the data that's in `vec0` and pass that
    // to `fill_vec` instead.

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    vec1_copy.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1_copy.len(), vec1_copy);

    // 2. Make `fill_vec` borrow its argument instead of taking ownership of it,
    // and then copy the data within the function in order to return an owned
    // `Vec<i32>`

    let mut vec1_borrow = fill_vec_borrow(&vec0);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    vec1_borrow.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1_copy.len(), vec1_copy);

    // 3. Make `fill_vec` *mutably* borrow a reference to its argument (which will need to be
    // mutable), modify it directly, then not return anything. Then you can get rid
    // of `vec1` entirely -- note that this will change what gets printed by the
    // first `println!`

    fill_vec_borrow_mut(&mut vec0);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_vec_copy(mut vec: Vec<i32>) -> Vec<i32> {

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_borrow(vec0: &Vec<i32>) -> Vec<i32> {

    let mut vec = vec0.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_borrow_mut(vec0: &mut Vec<i32>) {

    vec0.push(22);
    vec0.push(44);
    vec0.push(66);
}