use clap::{Command, Arg};

fn double(num: i32) -> i32 {
    num * 2
}

pub fn run() -> () {
    let matches = Command::new("gh_test")
        .author("Me")
        .version("0.1.0")
        .about("Give me a number and I'll double it!")
        .arg(
            Arg::new("number")
                .required(true)
                .value_parser(clap::value_parser!(i32))
        )
        .get_matches();

    if let Some(&num) = matches.get_one::<i32>("number") {
        println!("{}", double(num));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_double() {
        assert_eq!(42, double(21));
    }
}
