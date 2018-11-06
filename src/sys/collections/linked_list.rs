use alloc::rc::Rc;
use core::marker::PhantomData;


#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Node {
            element: element,
            next: None,
        }
    }
}


#[derive(Debug)]
pub struct LinkedList<T> {
    first: Option<Rc<Node<T>>>,
    phantom: PhantomData<T>,
}


impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            first: None,
            phantom: PhantomData,
        }
    }


    pub fn push_back(&mut self, _element: T) {

    }

    pub fn push_front(&mut self, element: T) {
        let mut new_node: Node<T> = Node::new(element);
        // new_node.next = self.first;
        self.first = Some(Rc::new(new_node));
    }
}
