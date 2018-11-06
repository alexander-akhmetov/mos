use alloc::boxed::Box;
use alloc::fmt;
use alloc::string::String;
use core::marker::PhantomData;

type NodeLink<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    next: NodeLink<T>,
}

impl<T> Node<T> {
    pub fn new(element: T, next: NodeLink<T>) -> Self {
        Node {
            element: element,
            next: next,
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    first: NodeLink<T>,
    phantom: PhantomData<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            first: None,
            phantom: PhantomData,
        }
    }

    pub fn push_front(&mut self, element: T) {
        let new_node = Node::new(element, self.first.take());
        self.first = Some(Box::new(new_node));
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.first.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.element
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
