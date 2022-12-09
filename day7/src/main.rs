use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

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
    let mut lines = BufReader::new(file).lines().filter_map(|i| i.ok());

    let tree = build_tree(&mut lines);

    let mut total_res: u32 = 0;
    total_size(&tree.borrow(), &mut total_res);
    println!("{}", total_res);
}

fn total_size(tree: &Directory, total_res: &mut u32) -> u32 {
    let mut total: u32 = tree.files.iter().sum();
    total = total
        + tree
            .children
            .values()
            .map(|c| total_size(&c.borrow(), total_res))
            .sum::<u32>();
    if total < 100000 {
        *total_res = *total_res + total;
        println!("{} - {}", tree.name, total);
    }
    return total;
}

fn build_tree(lines: &mut impl Iterator<Item = String>) -> Rc<RefCell<Directory>> {
    // We know the first line is cd /. skip it
    let root = Rc::new(RefCell::new(Directory {
        name: String::from("/"),
        parent: None,
        children: HashMap::new(),
        files: Vec::new(),
    }));
    lines.next();
    let mut curdir = root.clone();
    while let Some(line) = lines.next() {
        println!("{}", line);
        match LineType::from(&line) {
            LineType::List => (),
            LineType::ChangeDir => {
                let dest = line.strip_prefix("$ cd ").unwrap();
                if dest == ".." {
                    let foo = curdir.borrow().parent.clone().unwrap();
                    curdir = foo;
                } else {
                    // println!("{}", dest);
                    let bar = curdir.borrow().children.get(dest).unwrap().clone();
                    curdir = bar;
                }
            }
            LineType::Output => {
                let (first, second) = line.split_once(' ').unwrap();
                let name = second.to_owned();
                if first == "dir" {
                    curdir.borrow_mut().children.insert(
                        name,
                        Rc::new(RefCell::new(Directory {
                            name: second.to_owned(),
                            parent: curdir.clone().into(),
                            children: HashMap::new(),
                            files: Vec::new(),
                        })),
                    );
                } else {
                    curdir.borrow_mut().files.push(first.parse().unwrap());
                }
            }
        }
    }
    return root;
}
