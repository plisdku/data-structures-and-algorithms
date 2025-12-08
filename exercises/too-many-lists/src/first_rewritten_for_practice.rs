// List hack.
// Try to remember how to do this.

// Singly-linked list. First, use a head pointer?
//
// head -> [Elem, ptr] -> [Elem, ptr] -> [Elem, none]
//
// Second, can use a dummy element.
//
// [Elem, ptr] -> [Elem, ptr] -> [Elem, none]
//
// We were encouraged to think about heap and stack considerations.
// If your first node is on the heap, you can't easily insert in front
// of it. You want the heap to only have a dummy node / head pointer.

// Wait a minute. We don't have null pointers. The book says
//
// [Elem, ptr] -> [Elem, ptr] -> [Empty, junk]
//
// using enums.

#[derive(Debug)]
pub struct ListNode {
    data: i32,
    next: ListPtr,
}

#[derive(Debug)]
pub enum ListPtr {
    Empty,
    Full(Box<ListNode>),
}

pub struct List {
    head: ListPtr,
}

impl List {
    pub fn new() -> Self {
        List {
            head: ListPtr::Empty,
        }
    }

    pub fn push(&mut self, elem: i32) {
        // Make a new node and have the head pointer point to it.
        //
        // Original state:
        // head -> ListNode
        //
        // New state:
        // head -> ListNode(new data, old head)

        // We do a little dance. First, move head pointer out and swap in an empty pointer.
        // Second, set head to a new pointer.
        // Presumably this can optimize out.

        let new_node = ListNode {
            data: elem,
            next: std::mem::replace(&mut self.head, ListPtr::Empty),
        };
        self.head = ListPtr::Full(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match &mut self.head {
            ListPtr::Empty => None,
            ListPtr::Full(node_box) => {
                // How to pop?
                //
                // head = head->next

                let elem = node_box.data; // copy

                match &node_box.next {
                    ListPtr::Empty => {
                        self.head = ListPtr::Empty;
                    }
                    ListPtr::Full(_next_node_box) => {
                        self.head = std::mem::replace(&mut node_box.next, ListPtr::Empty);
                    }
                }
                Some(elem)
            }
        }
    }

    pub fn peek(&self) -> Option<i32> {
        match &self.head {
            ListPtr::Empty => None,
            ListPtr::Full(node_box) => Some(node_box.data),
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_your_mom() {
        use super::{ListNode, ListPtr};

        // make a head pointer and one object
        let full = ListNode {
            data: 99,
            next: ListPtr::Empty,
        };
    }

    #[test]
    fn test_new_list() {
        use super::List;

        let mut list = List::new();

        assert_eq!(list.peek(), None);
        list.push(44);
        assert_eq!(list.peek(), Some(44));
        list.push(55);
        assert_eq!(list.peek(), Some(55));

        assert_eq!(list.pop(), Some(55));
        assert_eq!(list.peek(), Some(44));
        assert_eq!(list.pop(), Some(44));
        assert_eq!(list.pop(), None);
    }
}
