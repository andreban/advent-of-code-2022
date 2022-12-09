use std::{str::FromStr, collections::HashMap, process::Command};

const SAMPLE_INPUT: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

#[derive(Debug)]
pub struct InputParseError(String);

#[derive(Debug)]
pub enum Prompt {
    Cd(String),
    Ls,
}

impl FromStr for Prompt {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        match parts[1] {
            "cd" => Ok(Self::Cd(parts[2].to_string())),
            "ls" => Ok(Self::Ls),
            _ => Err(InputParseError(format!("Unexpected Promp {}", parts[1]))),
        }
    }
}

#[derive(Debug)]
pub enum Entry {
    Command(Prompt),
    File(String, usize),
    Directory(String),
}

impl FromStr for Entry {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts[0] == "$" {
            let prompt = s.parse::<Prompt>()?;
            Ok(Entry::Command(prompt))
        } else if parts[0] == "dir" {
            Ok(Entry::Directory(parts[1].to_string()))
        } else {
            let size = parts[0].parse::<usize>()
                .map_err(|e| InputParseError(e.to_string()))
                .unwrap();
            Ok(Entry::File(parts[1].to_string(), size))
        }
    }    
}

#[derive(Debug)]
enum Node {
    Directory(String, HashMap<String, Node>),
    File(String, usize),
}

#[derive(Debug)]
struct FileTree(Node);

impl FileTree {
    pub fn add_node(&mut self, path: &[String], name:String, new_node: Node) {
        let mut current_node = &mut self.0;
        for section in path {
            if let Node::Directory(_, directory) = current_node {
                current_node = directory.get_mut(section).unwrap();
            }
        }
        if let Node::Directory(_, directory) = current_node {
            directory.insert(name, new_node);
        }
    }

    fn calculate_node_size<F: FnMut(&str, usize)>(node: &Node, callback: &mut F) -> usize {
        let Node::Directory(dir_name, dir) = node else {
            return 0;
        };
        
        let mut sum = 0;
        for (_, node) in dir.iter() {
            let dir_size = Self::calculate_node_size(node, callback);
            sum += dir_size;
            if let Node::File(_, size) = node {
                sum += *size;
            }
        }
        callback(&dir_name, sum);
        sum
    }

    pub fn calculate_size<F: FnMut(&str, usize)>(&self, callback: &mut F) -> usize {
        let Node::Directory(_, dir) = &self.0 else {
            return 0;
        };
        let mut total_size = 0;
        for node in dir.values() {
            total_size += Self::calculate_node_size(node, callback);
        }
        total_size        
    }
}


fn main() {
    let mut root_node = FileTree(Node::Directory("root".to_string(), HashMap::new()));

    let mut full_path = vec![];
    let input = include_str!("day7.txt");

    for line in input.lines() {
        let entry = line.parse::<Entry>().unwrap();
        match entry {
            Entry::Command(prompt) => {
                match prompt {
                    Prompt::Ls => {},
                    Prompt::Cd(dir_name) => {
                        if dir_name == ".." {
                            full_path.pop();
                        } else {
                            root_node.add_node(
                                &full_path, dir_name.clone(),
                                Node::Directory(dir_name.clone(), HashMap::new()));
                            full_path.push(dir_name);
                        }
                    },
                }
            },
            Entry::File(name, size) => {
                root_node.add_node(&full_path, name.clone(), Node::File(name, size));
            }
            _ => {}   
        }
    }
    // println!("{:#?}", root_node);


    // Part 1
    println!("--===Part 1===---");
    let mut total_sum = 0;
    let total_size = root_node.calculate_size(&mut |name, size|{
        println!("{} -> {}", name, size);
        if size < 100000 {
            total_sum += size;
        }
    });


    // Part 2
    println!("---===Part2===---");
    let amount_free = 70000000 - total_size;
    let amount_to_delete = 30000000 - amount_free;
    println!("Total Used: {}; Amount Free: {}, Amount to delete: {}", total_size, amount_free, amount_to_delete);
    let mut delete_size = usize::MAX;
    root_node.calculate_size(&mut |name, size|{
        if size > amount_to_delete && size < delete_size{
            delete_size = size;
            println!("{} -> {}", name, size);
        }
    });

}

#[cfg(test)]
mod tests {

}