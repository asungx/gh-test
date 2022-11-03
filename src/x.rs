pub fn say_hello(name: &str) -> () {
    println!("Hello, {}!", name);
}

pub fn meaning_of_life() -> u32 {
    42
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_meaning_of_life() {
        assert_eq!(42, meaning_of_life());
    }
}
