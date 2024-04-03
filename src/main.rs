fn main() {
    println!("{}", greet());
}

fn greet() -> &'static str {
    "Hello, world!"
}

fn foo_bar_qix(number: usize) -> String {
    if number % 3 == 0 {
        return "Foo".to_string()
    }
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

    #[test]
    fn it_returns_foo_if_the_number_is_divisible_by_3() {
        let result = foo_bar_qix(3);
        assert_eq!(result, "Foo")
    }
}
