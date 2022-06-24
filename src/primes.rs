pub fn primes(count:usize) -> Vec<u32> {
    let mut ps = Vec::new();
    for n in 2..{
        if is_prime(n){
            ps.push(n);
        }
        if ps.len()>=count{
            break;
        }
    }
    ps
}

fn is_prime(n:u32) -> bool {
    for m in 2..n {
        if n%m == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_primes(){
        assert_eq!(vec![2, 3, 5, 7, 11], super::primes(5));
    }
}