// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)


// I AM DONE
#[warn(unused_imports)]


mod sausage_factory {

    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        use super::*;

        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
