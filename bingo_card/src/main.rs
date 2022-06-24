use rand::prelude::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<u32> = (1..=75).collect();
    nums.shuffle(&mut rng);
    for row in 0..5 {
        for column in 0..5{
            if row==2 && column==2 {
                print!(" ** ");
            } else {
                print!(" {:>2} ", nums[5*row+column]);
            }
        }
        println!();
    }

}
