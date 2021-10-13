// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

// I AM DONE

fn main() {
    let mut vec0 = Vec::new();
    let vec1 = fill_vec(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    // cannot be used because that value was borred
    // println!("{} has length {} content `{:?}`", "vec0", vec.len(), vec0);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
