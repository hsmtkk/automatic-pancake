use anyhow::{bail, Result};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    println!("Hello, world!");
}

struct Translator {
    dict: HashMap<String, String>,
}

impl Translator {
    fn new() -> Translator {
        let mut dict: HashMap<String, String> = HashMap::new();
        let file = File::open("ejdict-hand-utf8.txt").expect("failed to open file");
        let reader = BufReader::new(file);
        for line in reader.lines(){
            let line = line.unwrap();
            let elems: Vec<&str> = line.split_whitespace().collect();
            dict.insert(elems[0].to_string(), elems[1].to_string());
        }
        Translator{dict}
    }

    pub fn translate(&self, en:&str) -> Result<String> {
        match self.dict.get(en){
            Some(ja) => Ok(ja.clone()),
            _ => bail!("no such word {}", en),
        }
    }    
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_translate(){
        let t = super::Translator::new();
        assert_eq!("チューリッヒ(スイス北部にある同国最大の都市)", t.translate("Zurich").unwrap());
    }
}