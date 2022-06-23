fn main() {
    for i in 1..101 {
        println!("{}", fizz_buzz(i));
    }
}

fn fizz_buzz(i: u32) -> String {
    if i % 15 == 0 {
        "FizzBuzz".to_string()
    } else if i % 3 == 0 {
        "Fizz".to_string()
    } else if i % 5 == 0 {
        "Buzz".to_string()
    } else {
        i.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::fizz_buzz;

    #[test]
    fn test_fizz_buzz(){
        assert_eq!("Fizz", fizz_buzz(6));
        assert_eq!("Buzz", fizz_buzz(10));
        assert_eq!("FizzBuzz", fizz_buzz(30));
        assert_eq!("13", fizz_buzz(13));
    }
}