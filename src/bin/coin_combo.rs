#[derive(Debug)]
struct Coins {
    five_hundred: u32,
    hundred: u32,
    fifty: u32,
}

fn all_combos() -> Vec<Coins> {
    let mut results = Vec::new();
    for five_hundred in 0..=10 {
        for hundred in 0..=3 {
            for fifty in 0..=10 {
                let sum = 500 * five_hundred + 100 * hundred + 50 * fifty;
                if sum == 3950 {
                    results.push(Coins {
                        five_hundred,
                        hundred,
                        fifty,
                    });
                }
            }
        }
    }
    results
}

fn main() {
    for c in all_combos() {
        println!("{:?}", c);
    }
}
