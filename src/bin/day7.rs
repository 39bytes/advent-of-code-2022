use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
enum Node {
    File(u64),
    Dir {
        children: HashMap<String, Rc<RefCell<Node>>>,
        parent: Option<Rc<RefCell<Node>>>,
    },
}

impl Node {
    // fn add_child(&mut self, name: String, child: Rc<RefCell<Node>>) {
    //     match self {
    //         Node::Dir {
    //             ref mut children,
    //             parent,
    //         } => {
    //             children.insert(name, child);
    //             println!("{:?}", children);
    //         }
    //         Node::File(_) => {
    //             panic!("Cannot call add_child on a file")
    //         }
    //     }
    // }
    fn get_size(&self) -> u64 {
        match self {
            Node::File(size) => *size,
            Node::Dir { children, .. } => {
                children.values().map(|node| node.borrow().get_size()).sum()
            }
        }
    }
    fn get_total_at_most_100k(&self) -> u64 {
        match self {
            Node::File(_) => 0,
            Node::Dir { children, .. } => {
                let size = self.get_size();
                return if size < 100000 { size } else { 0 }
                    + children
                        .values()
                        .map(|node| node.borrow().get_total_at_most_100k())
                        .sum::<u64>();
            }
        }
    }

    fn get_least_needed(&self, need: u64) -> Option<u64> {
        match self {
            Node::File(_) => None,
            Node::Dir { children, .. } => {
                let min_needed_size = children
                    .values()
                    .filter_map(|node| node.borrow().get_least_needed(need))
                    .min();

                if min_needed_size.is_some() {
                    return min_needed_size;
                }

                let size = self.get_size();
                if size >= need {
                    return Some(size);
                }

                None
            }
        }
    }
}

fn main() {
    let file = File::open("inputs/day7.txt").unwrap();
    let reader = BufReader::new(file);

    let root = Rc::new(RefCell::new(Node::Dir {
        children: HashMap::new(),
        parent: None,
    }));

    let mut cur_node = Rc::clone(&root);

    for line in reader.lines().skip(1) {
        if let Ok(line) = line {
            let line: Vec<&str> = line.split_whitespace().collect();

            match line.as_slice() {
                ["$", "ls"] => {}
                ["$", "cd", to] => match *to {
                    ".." => {
                        let parent = match cur_node.borrow().clone() {
                            Node::Dir { parent, .. } => parent,
                            _ => panic!("a"),
                        };
                        cur_node = parent.unwrap();
                    }
                    dir => {
                        let child = match cur_node.borrow().clone() {
                            Node::Dir { children, .. } => Rc::clone(children.get(dir).unwrap()),
                            _ => panic!("a"),
                        };
                        cur_node = child;
                    }
                },
                [s, filename] => match *s {
                    "dir" => {
                        if let Node::Dir {
                            ref mut children, ..
                        } = *cur_node.borrow_mut()
                        {
                            children.insert(
                                String::from(*filename),
                                Rc::new(RefCell::new(Node::Dir {
                                    children: HashMap::new(),
                                    parent: Some(Rc::clone(&cur_node)),
                                })),
                            );
                        }
                    }
                    size => {
                        if let Node::Dir {
                            ref mut children, ..
                        } = *cur_node.borrow_mut()
                        {
                            children.insert(
                                String::from(*filename),
                                Rc::new(RefCell::new(Node::File(size.parse::<u64>().unwrap()))),
                            );
                        }
                    }
                },
                _ => {}
            }
        }
    }
    println!(
        "Solution to part 1: {}",
        root.borrow().get_total_at_most_100k()
    );

    let needed = 30000000 - (70000000 - root.borrow().get_size());

    println!(
        "Solution to part 2: {}",
        root.borrow().get_least_needed(needed).unwrap()
    );
}
