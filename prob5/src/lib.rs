use std::iter::FromIterator;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>//C 구조체 선언과 같다. 재귀적인 타입 정의이다. 값이 있으면 안되고 포인터가 있어야함
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut cur = &self.head;
        loop {
            match cur {
                None => break,
                Some(item) => {
                    cur = &item.next;
                    count += 1;
                }
            }
        }
        count
    }

    pub fn push(&mut self, _element: T) {
        let new_item = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }));
        self.head = new_item;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(head_node) => {
                self.head = head_node.next;
                Some(head_node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(head_node) => {
                Some(&head_node.data)
            }
        }
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        while !self.is_empty() {
            rev_list.push(self.pop().unwrap());
        }
        rev_list
    }
}

// 컨테이너에서 list로 변환하기 위한 트레잇
impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for it in _iter {
            list.push(it);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

// list에서 vector로 변환하기 위한 트레잇
impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();
        while !_linked_list.is_empty() {
            vec.insert(0, _linked_list.pop().unwrap());
        }
        vec
    }
}

#[test]
fn test_new_list_is_empty() {
    let list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert_eq!(list.len(), 0, "list's length must be 0");
}

#[test]
fn test_push_increments_length() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    assert_eq!(list.len(), 1, "list's length must be 1");
    list.push(2);
    assert_eq!(list.len(), 2, "list's length must be 2");
}

#[test]
fn test_pop_decrements_length() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.pop();
    assert_eq!(list.len(), 1, "list's length must be 1");
    list.pop();
    assert_eq!(list.len(), 0, "list's length must be 0");
}

#[test]
fn test_is_empty() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert!(list.is_empty(), "List wasn't empty on creation");
    for inserts in 0..100 {
        for i in 0..inserts {
            list.push(i);
            assert!(
                !list.is_empty(),
                "List was empty after having inserted {}/{} elements",
                i,
                inserts
            );
        }
        for i in 0..inserts {
            assert!(
                !list.is_empty(),
                "List was empty before removing {}/{} elements",
                i,
                inserts
            );
            list.pop();
        }
        assert!(
            list.is_empty(),
            "List wasn't empty after having removed {} elements",
            inserts
        );
    }
}

#[test]
fn test_pop_returns_head_element_and_removes_it() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    assert_eq!(list.pop(), Some(2), "Element must be 2");
    assert_eq!(list.pop(), Some(1), "Element must be 1");
    assert_eq!(list.pop(), None, "No element should be contained in list");
}

#[test]
fn test_peek_returns_reference_to_head_element_but_does_not_remove_it() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert_eq!(list.peek(), None, "No element should be contained in list");
    list.push(2);
    assert_eq!(list.peek(), Some(&2), "Element must be 2");
    assert_eq!(list.peek(), Some(&2), "Element must be still 2");
    list.push(3);
    assert_eq!(list.peek(), Some(&3), "Head element is now 3");
    assert_eq!(list.pop(), Some(3), "Element must be 3");
    assert_eq!(list.peek(), Some(&2), "Head element is now 2");
    assert_eq!(list.pop(), Some(2), "Element must be 2");
    assert_eq!(list.peek(), None, "No element should be contained in list");
}

#[test]
fn test_from_slice() {
    let mut array = vec!["1", "2", "3", "4"];
    let mut list: SimpleLinkedList<_> = array.drain(..).collect();
    assert_eq!(list.pop(), Some("4"));
    assert_eq!(list.pop(), Some("3"));
    assert_eq!(list.pop(), Some("2"));
    assert_eq!(list.pop(), Some("1"));
}

#[test]
fn test_reverse() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut rev_list = list.rev();
    assert_eq!(rev_list.pop(), Some(1));
    assert_eq!(rev_list.pop(), Some(2));
    assert_eq!(rev_list.pop(), Some(3));
    assert_eq!(rev_list.pop(), None);
}

#[test]
fn test_into_vector() {
    let mut v = Vec::new();
    let mut s = SimpleLinkedList::new();
    for i in 1..4 {
        v.push(i);
        s.push(i);
    }
    let s_as_vec: Vec<i32> = s.into();
    assert_eq!(v, s_as_vec);
}