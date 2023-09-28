use std::cell::RefCell;
use std::rc::Rc;

type NodeListState = Option<Vec<Node>>;

#[derive(Debug)]
struct Container {
    nodes: RefCell<NodeListState>
}

impl Container {
    fn new() -> Self {
        Self { nodes: RefCell::new(None) }
    }

    fn init(&self, v: Vec<Node>) -> () {
        self.nodes.replace(Some(v));
    }

    fn add_nodes(&self, vadd: &mut Vec<Node>) -> () {
        let tmp = self.nodes.take();
        if let Some(mut vb) = tmp {
            vb.append(vadd);
            self.nodes.replace(Some(vb));
        }
    }
}

#[derive(Debug)]
struct Node {
    id: i32,
    ref_parent: Rc<Container>
}

impl Node {
    fn new(id: i32, ref_parent: Rc<Container>) -> Self {
        Self { id, ref_parent }
    }
}

fn main() {
    let head = Rc::new(Container::new());
    let a = Node::new(1, Rc::clone(&head));
    let b = Node::new(2, Rc::clone(&head));
    let vec = vec![a, b];
    head.init(vec);

    println!("addr of head: {:p}", head);

    {
        let v = head.nodes.borrow();
        let v = v.as_ref().unwrap();
        println!("space 0. id: {}, points at: {:p}", v[0].id, v[0].ref_parent);
        println!("space 1. id: {}, points at: {:p}", v[1].id, v[1].ref_parent)
    }

    let c = Node::new(3, Rc::clone(&head));
    let d = Node::new(4, Rc::clone(&head));
    let mut vec = vec![c, d];
    head.add_nodes(&mut vec);

    {
        let v = head.nodes.borrow();
        let v = v.as_ref().unwrap();
        println!("space 2. id: {}, points at: {:p}", v[2].id, v[2].ref_parent);
        println!("space 3. id: {}, points at: {:p}", v[3].id, v[3].ref_parent)
    }
    
}
