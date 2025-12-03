use std::mem;

#[derive(Debug)]
pub enum DumbList {
    Empty,
    Elem(i32, Box<DumbList>),
}

// Why must recursive structures be boxed?
// Because the compiler apparently needs to be able to figure out how big
// things are in advance.
//
// DumbList has some funny behaviors.
// First, letting [] denote stack-allocated objects and () denote heap-allocated
// objects, our list looks like this:
//
// [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
//
// where I guess "junk" means there is wasted space because every type in
// DumbList has the same size.
//
// The book recommends we might want something more lightweight:
//
// [ptr] -> (Elem A, ptr) -> (Elem B, null)
//
// to save space.
//

// After fits and starts we end up with this:

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    /// Add element to the front of the list
    pub fn push(&mut self, elem: i32) {
        let node = Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        // once we OWN the old head link, everything becomes very simple.
        let old_head = mem::replace(&mut self.head, Link::Empty);

        if let Link::More(head_node) = old_head {
            self.head = head_node.next;
            println!("popping and we have more");
            Some(head_node.elem)
        } else {
            println!("popping and we don't have more");
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn test_push_pop() {
        let mut l = List::new();

        assert_eq!(l.pop(), None);

        l.push(0);
        l.push(1);
        l.push(2);

        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(1));

        l.push(3);

        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), Some(0));
    }
}
