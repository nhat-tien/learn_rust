pub struct LinkedList {
    head: Link
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn add_first(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take()
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32>{
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
    
}


