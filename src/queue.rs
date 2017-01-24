use std::collections::VecDeque;
use std::ptr;

fn main() {

    //using an implementation from the std lib
    let mut stack = VecDeque::new();
    stack.push_back("E1");
    stack.push_back("E2");
    assert_eq!(Some(&"E1"), stack.front());
    assert_eq!(Some("E1"), stack.pop_front());
    assert_eq!(Some("E2"), stack.pop_front());
    assert_eq!(None, stack.pop_front());
}

pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Item<T>
}

type Link<T> = Option<Box<Item<T>>>;

struct Item<T> {
    elem: T,
    next: Link<T>,
}


impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None, tail: ptr::null_mut() }
    }

    pub fn enqueue(&mut self, elem: T) {
        let mut new_tail = Box::new(Item {
            elem: elem,
            next: None,
        });
        let raw_tail: *mut _ = &mut *new_tail;
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }
        else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail //?
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let h = *head;
            self.head = h.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            h.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        return self.head.as_ref().map(|item| {
            &item.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        return self.head.as_mut().map(|item| {
            &mut item.elem
        })
    }
}
