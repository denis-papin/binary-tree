//use std::ops::{Deref, DerefMut};


use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
struct Node<T> {
    pub parent: Option<*mut Node<T>>,
    pub left : Option<Box<Node<T>>>,
    pub right : Option<Box<Node<T>>>,
    pub value : T,
}

impl <T> Node<T> {
    pub fn new(value : T) -> Box<Self> {
        Box::new(Node {
            parent: None,
            left: None,
            right: None,
            value,
        })
    }

    pub fn set_left<'a>( &'a mut self, mut left : Box<Node<T>>) -> &'a mut Box<Node<T>> {
        let self_ptr = self as *mut Node<T>;
        left.parent = Some(self_ptr);
        self.left = Some(left);
        let opt = self.left.as_mut().unwrap();
        opt
    }

    pub fn set_right<'a>( &'a mut self, mut right : Box<Node<T>>) -> &'a mut Box<Node<T>> {
        let self_ptr = self as *mut Node<T>;
        right.parent = Some(self_ptr);
        self.right = Some(right);
        let opt = self.right.as_mut().unwrap();
        opt
    }

    pub fn get_parent<'a>(&'a self) -> Option<Box<&'a mut Node<T>>> {
        let my_parent = match  self.parent {
            None => {None}
            Some(raw_ptr) => {
                let parent;
                println!("Node p :  [{:?}]", &raw_ptr);
                unsafe {
                    let box_ref = &mut *raw_ptr;
                    parent = Some(Box::new(box_ref));
                }
                parent
            }
        };
        my_parent
    }

}


fn main() {

}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::ops::{Deref, DerefMut};
    use crate::Node;

    #[test]
    fn test_integer_tree() {
        let node_a = Node::new(200);
        let node_b = Node::new(333);
        let mut root = Node::new(100);

        let cc = &mut *root;
        println!("Address of root raw : {:p}", cc);

        let _ = root.as_mut().set_left(node_a);
        let _ = root.as_mut().set_right(node_b);

        println!("Root : [{:?}]", &root);
        println!("Node a :  [{:?}]", &root.left);

        let mut left_node = root.left.unwrap();
        let parent = left_node.as_mut().get_parent();
        println!("Node parent :  [{:?}]", &parent.unwrap());

        let pure_parent = left_node.as_mut().get_parent();
        pure_parent.unwrap().value = 999;

        {
            let node_2a = Node::new(400);
            let node_2a_ref = left_node.as_mut().set_left(node_2a);
            node_2a_ref.value = 401;
            println!("Node 2 a ref :  [{:?}]", node_2a_ref);
        }

        let pure_parent_2 = left_node.get_parent();
        println!("Node pure parent 2 :  [{:?}]", &pure_parent_2.unwrap());
    }


    #[test]
    fn test_tree_of_references() {
        let amount = 34_000_000;
        let mut ref_node = Node::new(&amount);
        ref_node.as_mut().set_left(Node::new(&amount));
        let amt = ref_node.as_mut().set_right(Node::new(&amount));
        amt.value = &1000;
        println!("Ref node : {:?}", &ref_node);
    }

    #[test]
    fn test_tree_of_same_structure() {
        #[derive(Clone)]
        struct A {
            a : i64,
        }

        let mut val = Box::new(A { a : 34_000_000_i64 });
        let mut amount = RefCell::new(val);
        let a = amount.borrow().deref().a;
        println!("Ref a : {:?}", &a);

        let mut root_node = Node::new(&amount);
        root_node.as_mut().set_left(Node::new(&amount));

        let mut amt = root_node.as_mut().set_right(Node::new(&amount));
        // We change the value on the right node
        root_node.as_mut().value.borrow_mut().deref_mut().a = 1000_i64;

        // The value has changed also on the root node !!! :))
        let a = root_node.as_ref().value.borrow().a; // It's actually ".deref().deref().a";
        println!("Ref aaa : {:?}", a);
    }

}