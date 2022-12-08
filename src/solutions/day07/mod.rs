use itertools::Itertools;
use std::{cell::RefCell, fmt, rc::Rc};

pub fn solution1(data: String) -> i32 {
    let filesystem = read_filesystem(data);
    filesystem.borrow_mut().calculate_size();
    filesystem.as_ref().borrow().print_node(None);

    let mut dir_sizes = filesystem.as_ref().borrow().get_directories_size();
    dir_sizes = dir_sizes.into_iter().filter(|s| *s <= 100000).sorted().collect();
    println!("{:?}", dir_sizes);

    let result = dir_sizes.into_iter().sum();

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

const TOTAL_DISK_SPACE: i32 = 70000000;
const UPDATE_REQUIRED_SPACE: i32 = 30000000;

pub fn solution2(data: String) -> i32 {
    let filesystem = read_filesystem(data);
    filesystem.borrow_mut().calculate_size();

    let total_size = filesystem.as_ref().borrow().size;
    let unused_size = TOTAL_DISK_SPACE - total_size;

    let mut result = 0;
    if unused_size < UPDATE_REQUIRED_SPACE {
        let required_size = UPDATE_REQUIRED_SPACE - unused_size;
        let mut dir_sizes = filesystem.as_ref().borrow().get_directories_size();
        dir_sizes = dir_sizes.into_iter().filter(|s| *s >= required_size).sorted().collect();
        println!("{:?} > {}", &dir_sizes, required_size);

        result = dir_sizes.into_iter().min().expect("A solution must exists");
    }

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

struct Node {
    name: String,
    size: i32,
    files: Vec<Rc<RefCell<Node>>>, // Only for directories
    parent: Option<Rc<RefCell<Node>>>
}

impl fmt::Display for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self.is_dir() {
            true => write!(fmt, "- {} (dir, size: {})", &self.name, self.size),
            false => write!(fmt, "- {} (file, size: {})", &self.name, self.size)
        }
    }
}

impl Node {
    fn new(name: &str, size: Option<i32>, parent: Option<Rc<RefCell<Node>>>) -> Node {
        Node { name: String::from(name), size: size.unwrap_or(0), files: vec![], parent }
    }

    fn is_dir(&self) -> bool {
        self.files.len() > 0
    }

    fn insert_file(&mut self, name: &str, size: i32, parent: Option<Rc<RefCell<Node>>>) {
        let file_node = Node::new(name, Some(size), parent);
        self.files.push(Rc::new(RefCell::new(file_node)));
    }

    fn insert_dir(&mut self, name: &str, parent: Option<Rc<RefCell<Node>>>) {
        let dir_node = Node::new(name, None, parent);
        self.files.push(Rc::new(RefCell::new(dir_node)));
    }

    fn get_node_ref(&self, dir_name: &str) -> &Rc<RefCell<Node>> {
        &self.files.iter().find(|f| f.as_ref().borrow().name == dir_name).unwrap()
    }

    fn print_node(&self, depth: Option<i32>) {
        let tabulation = match depth {
            Some(d) => (0..d * 2).map(|_| " ").collect::<String>(),
            None => String::from("")
        };

        println!("{}{}", &tabulation, &self);
        for f in &self.files {
            f.as_ref().borrow().print_node(Some(depth.unwrap_or(0) + 1));
        }
    }

    fn calculate_size(&mut self) -> i32 {
        if !self.is_dir() {
            return self.size;
        }

        self.size = self.files.iter().map(|f| f.as_ref().borrow_mut().calculate_size()).sum();

        self.size
    }

    fn get_directories_size(&self) -> Vec<i32> {
        if !self.is_dir() {
            return vec![];
        }

        let mut sizes: Vec<i32> = vec![self.size];

        for sub_dir in self.files.iter() {
            for s in sub_dir.as_ref().borrow().get_directories_size() {
                sizes.push(s);
            }
        }

        sizes
    }
}

fn read_filesystem(data: String) -> Rc<RefCell<Node>> {
    // Root directory
    let root_node = Rc::new(RefCell::new(Node::new("/", None, None)));

    let mut current_dir = Rc::clone(&root_node);
    let mut lines_iter = data.lines();
    lines_iter.next(); // Skipping first line
    for line in lines_iter {
        if line == "$ ls" {
            continue;
        }

        if line.starts_with("dir") {
            // New directory
            let dir_name = line.replace("dir ", "");
            current_dir.borrow_mut().insert_dir(&dir_name, Some(Rc::clone(&current_dir)));
            continue;
        }

        if line.starts_with("$ cd") {
            // Changing current directory
            let path = line.replace("$ cd ", "");
            let current_clone = Rc::clone(&current_dir);
            if &path == ".." {
                current_dir = Rc::clone(current_clone.as_ref().borrow().parent.as_ref().unwrap());
            }
            else {
                current_dir = Rc::clone(current_clone.as_ref().borrow().get_node_ref(&path));
            }
            continue;
        }

        // New file
        let (fsize, fname) = line.split(" ").next_tuple().unwrap();
        current_dir.borrow_mut().insert_file(fname, fsize.parse::<i32>().unwrap(), Some(Rc::clone(&current_dir)));
    }

    root_node
}

/////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_file() -> String {
        let current_file = std::file!();
        let test_file = current_file.replace("mod.rs", "test.txt");
        return crate::read_file(&test_file).unwrap();
    }

    #[test]
    fn test_solution1() {
        let data = read_test_file();
        assert_eq!(95437, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(24933642, solution2(data));
    }
}
