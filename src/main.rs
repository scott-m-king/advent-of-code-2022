// https://adventofcode.com/2022/day/7

use std::{
    borrow::Borrow,
    cell::RefCell,
    fs,
    rc::{Rc, Weak},
};

type NodeLink = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    name: String,
    size: i64,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<NodeLink>,
}

impl Node {
    fn equals(&self, name: &str) -> bool {
        self.name == name
    }

    fn new_child(&self, parent: &NodeLink, name: &str, size: i64) -> NodeLink {
        Rc::new(RefCell::new(Node {
            name: String::from(name),
            size,
            parent: Some(Rc::<RefCell<Node>>::downgrade(parent)),
            children: Vec::new(),
        }))
    }
}

fn main() {
    let data = fs::read_to_string("test.txt").unwrap();
    let commands = data.lines();

    let root = Rc::new(RefCell::new(Node {
        name: String::from("/"),
        parent: None,
        size: 0,
        children: Vec::new(),
    }));

    // maaaybe try using a stack to keep track of where we are

    let mut root_ref = &Rc::clone(&root);

    for command in commands {
        match command.split(" ").collect::<Vec<&str>>().as_slice() {
            ["$", "cd", ".."] => {
                // todo
            }
            ["$", "cd", path] => {
                let parent = root_ref.as_ref().borrow();

                let child = parent
                    .children
                    .iter()
                    .find(|node| node.as_ref().borrow().equals(path));

                match child {
                    Some(node) => {
                    }
                    None => (),
                }
            }
            ["$", "ls"] => (),
            ["dir", name] => {
                let new_dir = root_ref
                    .as_ref()
                    .borrow()
                    .new_child(root_ref.borrow(), *name, 0);
                root_ref.as_ref().borrow_mut().children.push(new_dir);
            }
            // files
            [size, filename] => {
                let new_file = root_ref.as_ref().borrow().new_child(
                    root_ref.borrow(),
                    filename,
                    size.parse::<i64>().unwrap(),
                );
                root_ref.as_ref().borrow_mut().children.push(new_file);
            }
            _ => (),
        };
    }
    println!("{:?}", root);
}
