fn main() {
    println!("{}", greet());
}

fn greet() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use crate::greet;

    #[test]
    fn it_returns_a_greeting_message() {
        let result = greet();
        assert_eq!(result, "Hello, world!");
    }
}