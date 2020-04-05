use crate::linked_list::MyList::{Empty, Last, Node};

#[derive(Debug)]
pub enum MyList {
    Empty,
    Node {
        value: i32,
        next: Box<MyList>,
    },
    Last,
}

impl MyList {
    pub fn create() -> self::MyList {
        Empty
    }

    pub fn add(&mut self, value: i32) {
        match self {
            MyList::Empty => { *self = Node { value, next: Box::from(Last) } }
            MyList::Node { value: _, next: node } => { node.add(value) }
            MyList::Last => { *self = Node { value, next: Box::from(Last) } }
        }
    }
}
