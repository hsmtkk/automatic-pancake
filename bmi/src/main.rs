fn main() {
    let args: Vec<String> = std::env::args().collect();
    let height: f64 = args[1].parse().expect("height in meter");
    let weight: f64 = args[2].parse().expect("weight in kilogram");
    let b = bmi(height, weight);
    println!("height {}m", height);
    println!("weight {}kg", weight);
    println!("BMI {} {}", b, judge_bmi(b));
}

fn bmi(height: f64, weight: f64) -> f64 {
    weight / height / height
}

fn judge_bmi(bmi: f64) -> String {
    if bmi < 18.5 {
        "thin"
    } else if 18.5 <= bmi && bmi < 25.0 {
        "normal"
    } else {
        "fat"
    }
    .to_string()
}
