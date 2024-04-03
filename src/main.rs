fn main() {
    println!("{}", greet());
}

fn greet() -> &'static str {
    "Hello, world!"
}

fn foo_bar_qix(number: usize) -> String {
    number.to_string()
}

#[cfg(test)]
mod tests {

    use crate::{foo_bar_qix, greet};

    #[test]
    fn it_returns_a_greeting_message() {
        let result = greet();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn it_returns_1_if_input_is_1() {
        let result = foo_bar_qix(1);
        assert_eq!(result, "1")
    }

    #[test]
    fn it_returns_2_if_input_is_2() {
        let result = foo_bar_qix(2);
        assert_eq!(result, "2")
    }
}
