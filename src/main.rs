

#[derive(Debug, Clone)]
struct Node<T> {
    parent: Option<*mut Node<T>>,
    left : Option<Box<Node<T>>>,
    right : Option<Box<Node<T>>>,
    value : T,
}

impl <T> Node<T> {

    ///
    /// We must always use a node via Box<Node>
    ///
    pub fn new(value : T) -> Box<Self> {
        Box::new(Node {
            parent: None,
            left: None,
            right: None,
            value,
        })
    }

    ///
    /// Set the left node and define the parent of added node
    ///
    pub fn set_left(&mut self, mut left : Box<Node<T>>) -> &mut Box<Node<T>> {
        let self_ptr = self as *mut Node<T>;
        left.parent = Some(self_ptr);
        self.left = Some(left);
        let opt = self.left.as_mut().unwrap();
        opt
    }

    ///
    /// Set the right node and define the parent of added node
    ///
    pub fn set_right(&mut self, mut right : Box<Node<T>>) -> &mut Box<Node<T>> {
        let self_ptr = self as *mut Node<T>;
        right.parent = Some(self_ptr);
        self.right = Some(right);
        let opt = self.right.as_mut().unwrap();
        opt
    }


    ///
    /// Retrieve the parent of the node
    ///
    pub fn get_parent(&self) -> Option<Box<&mut Node<T>>> {
        let my_parent = match self.parent {
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

    ///
    ///
    ///
    pub fn read_value(&self) -> &T {
        &self.value
    }

    ///
    ///
    ///
    pub fn read_left_value(&self) -> Option<&T> {
        let ret = match &self.left {
            None => {None}
            Some(l) => {
                Some(&l.value)
            }
        };
        ret
    }

}

fn main() {

}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::ops::{Deref, DerefMut};
    use crate::Node;


    ///
    /// Create a binary tree with pure integers
    ///
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
        // println!("Node parent :  [{:?}]", &parent.unwrap());
        parent.unwrap().value = 999;

        {
            let node_2a = Node::new(400);
            let node_2a_ref = left_node.as_mut().set_left(node_2a);
            node_2a_ref.value = 401;
            println!("Node 2 a ref :  [{:?}]", node_2a_ref);
        }

        let parent_2 = left_node.get_parent();
        // println!("Node parent 2 :  [{:?}]", &parent_2.unwrap());

        let pp = parent_2.unwrap();

        let p_value = pp.read_value(); // &pp.value;
        assert_eq!(999, *p_value);

        let l_value = pp.read_left_value().unwrap(); // &pp.left.as_ref().unwrap().value;
        assert_eq!(200, *l_value);

    }


    ///
    /// Create a binary tree with references to an integer.
    /// We can separately change the value of nodes
    ///
    #[test]
    fn test_tree_of_references() {
        let amount = 34_000_000;
        let mut ref_node = Node::new(&amount);
        ref_node.as_mut().set_left(Node::new(&amount));
        let amt = ref_node.as_mut().set_right(Node::new(&amount));
        amt.value = &1000;
        //println!("Ref node : {:?}", &ref_node);
        let value = ref_node.read_value().deref();
        assert_eq!(&34_000_000, value);
    }

    ///
    /// Create a binary tree of RefCell, so
    /// we can place the same struct on each node.
    /// We modify the reference's value, it changes for all the nodes.
    ///
    #[test]
    fn test_tree_of_same_structure() {
        #[derive(Clone)]
        struct A {
            a : i64,
        }

        // Create a structure on the heap
        let val = Box::new(A { a : 34_000_000_i64 });
        // Enclose it into a RefCell to allow borrowing and inner mutability
        let amount = RefCell::new(val);
        // Print the initial value
        let a = amount.borrow().deref().a;
        println!("Initial root node value : {:?}", &a);

        // Create the root node, pointing at the RefCell
        let mut root_node = Node::new(&amount);
        // Setting the left node with the same RefCell
        root_node.as_mut().set_left(Node::new(&amount));
        // Setting the right node with the same RefCell and get the node reference back in return
        let amt = root_node.as_mut().set_right(Node::new(&amount));
        // Change the value on the right node
        amt.read_value().borrow_mut().deref_mut().a = 1000_i64;
        // amt.as_mut().value.borrow_mut().deref_mut().a = 1000_i64;

        // The value has changed also on the root node !!! :))
        let root_a = root_node.read_value().borrow().a;
        // let root_a = root_node.as_ref().value.borrow().a; // It's actually ".deref().deref().a";

        println!("Root node value : {:?}", root_a);

        assert_eq!(1000_i64, root_a);
    }

}