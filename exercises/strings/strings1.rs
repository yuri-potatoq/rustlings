// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

// I AM DONE

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    let ans: &str = "blue";
    // ans.to_owned()
    // ans.to_string()
    String::from(ans)
}
