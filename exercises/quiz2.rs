// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command {
                Command::Uppercase => {
                    output.push(string.to_uppercase());
                }
                Command::Trim => {
                    let new_str = string.trim();
                    output.push(new_str.to_string());
                }
                // bug: https://stackoverflow.com/questions/57437256/cannot-borrow-as-mutable-as-it-is-behind-a-reference
                // bug: https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable
                Command::Append(length) => {
                    let mut x = 0; // x is usize
                    let mut y = string.clone(); // string is not mutable, so cann't declared as &mut string or &string to borrow the value of string
                    while x < *length {
                        // length is &usize
                        y.push_str("bar");
                        x += 1;
                    }
                    output.push(y.to_string());
                }
                _ => {}
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we have to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        // println!("output={:#?}", output.clone());
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
