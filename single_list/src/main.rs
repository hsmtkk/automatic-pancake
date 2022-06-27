mod slist;

fn main() {
    let mut list = slist::List::new();
    list.push(100);
    list.push(200);
    list.unshift(10);
    list.unshift(20);
    
}
