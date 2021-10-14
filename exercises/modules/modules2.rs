// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)

// I AM DONE

mod delicious_snacks {

    // TODO: Fix these use statements
    pub use self::{
        fruits::PEAR as pear,
        veggies::CUCUMBER as cucumber,
    };

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    use delicious_snacks::{
        cucumber, pear
    };

    println!(
        "favorite snacks: {} and {}",
        pear, cucumber
    );
}
