struct FibonacciIterator {
    a: usize,
    b: usize,
}

impl FibonacciIterator {
    fn new() -> FibonacciIterator {
        FibonacciIterator{a:1, b:1}
    }
}

impl Iterator for FibonacciIterator {
    type Item = usize;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> { 
        let tmp = self.a;
        self.a = self.b;
        self.b += tmp;
        Some(self.a)
    }
}

fn main(){
    let fib_iter = FibonacciIterator::new();
    fib_iter.take(10).for_each(|f| print!("{} ", f));
    println!();
}