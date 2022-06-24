fn main() {
    for i in 1..10 {
        let mut nums = Vec::new();
        for j in 1..10 {
            nums.push(format!("{:>3}", i * j));
        }
        println!("{}", nums.join(","));
    }
}
