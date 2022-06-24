fn encrypt(plain: &str) -> String {
    let mut encrypted = String::new();
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    for ch in plain.chars() {
        let code = ch as i16;
        if code_a <= code && code <= code_z {
            let shifted = (code - code_a + 3 + 26) % 26 + code_a;
            encrypted.push((shifted as u8) as char);
        } else {
            encrypted.push(ch);
        }
    }
    encrypted
}

fn main(){
    println!("{}", encrypt("HOGE HOGE"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encrypt() {
        assert_eq!("L ORYH BRX.", super::encrypt("I LOVE YOU."));
    }
}
