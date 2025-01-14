// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// Execute `rustlings hint primitive_types6` for hints!

// I AM DONE

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let (_, second, _) = numbers;

    let second = match numbers {
        (_, second, _) => second,
        _ => 0
    };

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
