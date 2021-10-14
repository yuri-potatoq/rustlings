// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

// I AM DONE

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice: &[i32] = &a[1..(a.len() - 1)];
    // not the same as `let nice_vec: vec<i32> = &[x..y];`

    assert_eq!([2, 3, 4], nice_slice)
}
