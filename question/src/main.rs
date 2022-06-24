use std::collections::HashMap;

fn main() {
    println!("{:?}", collect_question("C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C"));
}

fn collect_question(question:&str) -> HashMap<&str, usize> {
    let mut result: HashMap<&str, usize> = HashMap::new();
    for ch in question.split(","){
        let counter = result.entry(ch).or_insert(0);
        *counter += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_collect_question(){
        let mut want = HashMap::new();
        want.insert("A", 4);
        want.insert("B", 7);
        want.insert("C", 11);
        let got = super::collect_question("C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C");
        assert_eq!(want, got);
    }
}