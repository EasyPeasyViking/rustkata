pub fn foo_bar_qix(number: usize) -> String {
    let mut result= "".to_string();
    if number == 3 {
        result.push_str("Foo");
    }

    if number % 3 == 0 {
        result.push_str("Foo");
    }

    if number % 5 == 0 {
        result.push_str("Bar");
    }

    if result.is_empty() {
        return number.to_string()
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{foo_bar_qix};

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
        assert_eq!(result, "FooFoo")
    }

    #[test]
    fn it_returns_bar_if_the_number_is_divisible_by_5() {
        let result = foo_bar_qix(10);
        assert_eq!(result, "Bar")
    }
}
