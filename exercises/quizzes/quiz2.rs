enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    // 输入是 (String, Command) 元组的 Vector，输出是 String 的 Vector
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut result = Vec::new();

        for (mut string, command) in input {
            let processed = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => {
                    // 使用 push_str 或 repeat 均可
                    string.push_str(&"bar".repeat(n));
                    string
                }
            };
            result.push(processed);
        }

        result
    }
}

fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // 使用 use super::my_module::transformer; 或者如下：
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}