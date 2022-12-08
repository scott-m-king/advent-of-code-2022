// https://adventofcode.com/2022/day/7

use std::{cell::RefCell, fs, rc::Rc};

type NodeLink = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    name: String,
    size: i64,
    children: Vec<NodeLink>,
}

impl Node {
    fn equals(&self, name: &str) -> bool {
        self.name == name
    }

    fn new_child(&self, name: &str, size: i64) -> NodeLink {
        Rc::new(RefCell::new(Node {
            name: String::from(name),
            size,
            children: Vec::new(),
        }))
    }
}

fn main() {
    let data = fs::read_to_string("test.txt").unwrap();
    let commands = data.lines();

    let root = Rc::new(RefCell::new(Node {
        name: String::from("/"),
        size: 0,
        children: Vec::new(),
    }));

    // maaaybe try using a stack to keep track of where we are
    let mut parent_stack: Vec<&NodeLink> = Vec::new();
    parent_stack.push(&root);

    for command in commands {
        match command.split(" ").collect::<Vec<&str>>().as_slice() {
            ["$", "cd", ".."] => {
                // todo
                if !parent_stack.is_empty() {
                    parent_stack.pop();
                }
            }
            ["$", "cd", dir_path] => {
                let clone = parent_stack.last().unwrap().clone().as_ref().borrow();

                let child = clone
                    .children
                    .iter()
                    .find(|node| node.as_ref().borrow().equals(&dir_path));

                match child {
                    Some(node) => {
                        // let another = node.clone().borrow();
                        // parent_stack.push(another);
                        // parent_stack.push(&Rc::clone(&node));
                        // parent_stack.push(node);
                    }
                    None => (),
                }
            }
            ["$", "ls"] => (),
            ["dir", name] => {
                let new_dir = parent_stack
                    .last()
                    .unwrap()
                    .as_ref()
                    .borrow()
                    .new_child(*name, 0);
                parent_stack
                    .last()
                    .unwrap()
                    .as_ref()
                    .borrow_mut()
                    .children
                    .push(new_dir);
            }
            // files
            [size, filename] => {
                let new_file = parent_stack
                    .last()
                    .unwrap()
                    .as_ref()
                    .borrow()
                    .new_child(filename, size.parse::<i64>().unwrap());
                parent_stack
                    .last()
                    .unwrap()
                    .as_ref()
                    .borrow_mut()
                    .children
                    .push(new_file);
            }
            _ => (),
        };
    }
    println!("{:?}", root);
}
