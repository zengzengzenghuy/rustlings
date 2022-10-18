// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

// The order of defining macro should be in front of where you use it

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
