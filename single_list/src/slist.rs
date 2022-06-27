pub struct Node {
    data: isize,
    link: Option<Box<Node>>,
}

impl Node {
    fn new(data: isize, link: Option<Box<Node>>) -> Self {
        Self { data, link }
    }
}

pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }

    // 先頭に追加
    pub fn unshift(&mut self, v: isize) {
        let new_node = Node::new(v, self.head.take());
        self.head = Some(Box::new(new_node));
    }

    // 末尾に追加
    pub fn push(&mut self, v: isize) {
        let new_node = Node::new(v, None);
        match self.head {
            None => {
                self.head = Some(Box::new(new_node));
            }
            Some(ref mut head) => {
                let mut p = head;
                loop {
                    match p.link {
                        None => {
                            p.link = Some(Box::new(new_node));
                            break;
                        }
                        Some(ref mut next) => {
                            p = next;
                        }
                    }
                }
            }
        }
    }

    pub fn get(&self, index: isize) -> Option<isize> {
        match self.head {
            None => None,
            Some(ref top) => {
                let mut p = top;
                let mut i = 0;
                loop {
                    if i == index {
                        return Some(p.data);
                    }
                    match p.link {
                        None => return None,
                        Some(ref link) => p = link,
                    }
                    i += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_slist(){
        let mut list = super::List::new();
        list.push(100);
        list.push(200);
        list.unshift(10);
        list.unshift(20);
        assert_eq!(20, list.get(0).unwrap());
        assert_eq!(10, list.get(1).unwrap());
        assert_eq!(100, list.get(2).unwrap());
        assert_eq!(200, list.get(3).unwrap());
    }
}