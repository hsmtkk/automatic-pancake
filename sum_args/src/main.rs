fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nums: Vec<u32> = args[1..].iter().map(|s| s.parse::<u32>().unwrap()).collect();
    println!("{}", sum_nums(&nums));
}

fn sum_nums(nums:&[u32]) -> u32 {
    nums.iter().sum()
}