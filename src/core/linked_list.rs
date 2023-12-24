#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
}

impl<T> LinkedList<T>
where
    T: Copy,
{
    pub fn new() -> LinkedList<T> {
        return LinkedList { head: None };
    }

    pub fn empty(&self) -> bool {
        return self.head.is_none();
    }

    pub fn push(&mut self, data: T) -> &Self {
        match self.head {
            Some(ref mut head) => {
                head.push(data);
            }
            None => {
                self.head = Some(Box::new(ListNode::new(data)));
            }
        }
        return self;
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();
        match old_head {
            Some(node) => {
                self.head = node.next;
                return Some(node.data);
            }
            None => return None,
        }
    }

    pub fn peek(&self) -> Option<T> {
        match self.head.as_ref() {
            Some(node) => {
                return Some(node.data);
            }
            None => return None,
        }
    }
}

#[test]
fn test_linked_list_constructor() {
    let expected: LinkedList<i32> = LinkedList { head: None };
    let result = LinkedList::new();

    assert_eq!(expected, result);
}

#[test]
fn test_linked_list_empty() {
    let expected = true;
    let list: LinkedList<i32> = LinkedList { head: None };

    assert_eq!(expected, list.empty());
}

#[test]
fn test_linked_list_push() {
    let expected: LinkedList<i32> = LinkedList {
        head: Some(Box::new(ListNode {
            data: 1,
            next: None,
        })),
    };
    let mut result: LinkedList<i32> = LinkedList::new();
    result.push(1);

    assert_eq!(expected, result);
}

#[test]
fn test_linked_list_pop() {
    let expected = LinkedList {
        head: Some(Box::new(ListNode {
            data: 2,
            next: None,
        })),
    };

    let mut result = LinkedList {
        head: Some(Box::new(ListNode {
            data: 1,
            next: Some(Box::new(ListNode {
                data: 2,
                next: None,
            })),
        })),
    };
    result.pop();

    assert_eq!(expected, result);
}

#[test]
fn peek() {
    let list = LinkedList {
        head: Some(Box::new(ListNode {
            data: 1,
            next: None,
        })),
    };

    assert_eq!(1, list.peek().unwrap());
}

#[derive(Debug, PartialEq)]
struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(data: T) -> ListNode<T> {
        return ListNode { data, next: None };
    }

    fn push(&mut self, data: T) -> &mut Self {
        match self.next {
            Some(ref mut next) => {
                next.push(data);
            }
            None => {
                self.next = Some(Box::new(ListNode::new(data)));
            }
        }

        return self;
    }
}

#[test]
fn test_list_node_constructor() {
    let expected: ListNode<i32> = ListNode {
        data: 1,
        next: None,
    };
    let actual = ListNode::new(1);
    assert_eq!(expected, actual);
}

#[test]
fn test_list_node_push() {
    let expected = ListNode {
        data: 1,
        next: Some(Box::new(ListNode {
            data: 2,
            next: Some(Box::new(ListNode {
                data: 3,
                next: None,
            })),
        })),
    };

    let mut list_node = ListNode::new(1);
    list_node.push(2).push(3);

    assert_eq!(expected, list_node);
}
