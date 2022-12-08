use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap, rc::Rc, cell::RefCell};

struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    children: HashMap<String, Rc<RefCell<Directory>>>,
    files: Vec<u32>,
}

enum LineType {
    List,
    ChangeDir,
    Output,
}

impl LineType {
    fn from(line: &str) -> LineType {
        if line.starts_with("$ ls") {
            LineType::List
        } else if line.starts_with("$ cd") {
            LineType::ChangeDir
        } else {
            LineType::Output
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("can't find input.txt");
    let mut reader = BufReader::new(file).lines().filter_map(|i| i.ok());

    // We know the first line is cd /. skip it
    let root = Rc::new(RefCell::new(Directory { name: String::from("/"), parent: None, children: HashMap::new(), files: Vec::new()}));
    reader.next();
    let mut curdir = root.clone();
    while let Some(line) = reader.next() {
        println!("{}", line);
        match LineType::from(&line) {
            LineType::List => (),
            LineType::ChangeDir => {
                let dest = line.strip_prefix("$ cd ").unwrap();
                if dest == ".." {
                    let foo = curdir.borrow().parent.clone().unwrap();
                    curdir = foo;
                } else {
                    println!("{}", dest);
                    let bar = curdir.borrow().children.get(dest).unwrap().clone();
                    curdir = bar;
                }
            },
            LineType::Output => {
                let (first, second) = line.split_once(' ').unwrap();
                    let name = second.to_owned();
                    if first == "dir" {
                        curdir.borrow_mut().children.insert(name, Rc::new(RefCell::new(Directory{ 
                            name: second.to_owned(),
                            parent: curdir.clone().into(),
                            children: HashMap::new(),
                            files: Vec::new(),
                        })));
                    }
            },
        }
    }
}

