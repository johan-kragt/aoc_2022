use crate::reader::read_lines;
use colored::Colorize;
use std::cell::RefCell;
use std::rc::Rc;

pub fn day7() {
    let day = 7;
    println!(
        "{}{}{} ",
        "\nDay ".bold().bright_green(),
        day.to_string().bold().bright_red(),
        ":".bold().bright_green()
    );
    if let Ok(lines) = read_lines(format!("./data/day{}.txt", day)) {
        let mut current_dir = Rc::new(RefCell::new(Dir::new()));
        let root = &current_dir.clone();

        for line in lines {
            if let Ok(ip) = line {
                match &ip[0..4] {
                    "$ cd" => {
                        let x = current_dir.borrow().parent.clone();
                        match &ip[5..] {
                            "/" => {}
                            ".." => match x {
                                None => panic!("no parent"),
                                Some(dir) => current_dir = dir,
                            },
                            _ => {
                                let rc = Rc::new(RefCell::new(Dir::new()));
                                rc.borrow_mut().parent = Some(current_dir.clone());
                                current_dir.borrow_mut().children.push(rc.clone());
                                current_dir = rc;
                            }
                        }
                    }
                    "dir " => {}
                    "$ ls" => {}
                    _ => {
                        let ip: Vec<&str> = ip.split(' ').collect();
                        current_dir.borrow_mut().files.push(File {
                            //name: ip[1].to_string(),
                            size: ip[0].parse::<u32>().unwrap(),
                        })
                    }
                }
            }
        }
        println!(
            "{}",
            root.borrow()
                .report_dirs_under(100000)
                .to_string()
                .bright_white()
        );
        let vec = root
            .borrow()
            .list_dirs_freeing_more_then(30000000 - (70000000 - root.borrow().size()));
        println!("{}", vec.iter().min().unwrap().to_string().bright_yellow());
    }
}

#[derive(Debug)]
struct Dir {
    //name: String,
    files: Vec<File>,
    children: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    pub fn new() -> Dir {
        return Dir {
            //name: name,
            files: vec![],
            children: vec![],
            parent: None,
        };
    }

    /*pub fn print(&self, indent: usize) {
        println!("{:>width$} {}", &self.name, &self.size(), width = indent);
        &self.children.iter()
            .for_each(|f| f.borrow().print(indent + 2));
    }*/

    pub fn size(&self) -> u32 {
        &self.files.iter().map(|f| f.size).sum::<u32>()
            + &self.children.iter().map(|f| f.borrow().size()).sum::<u32>()
    }

    pub fn report_dirs_under(&self, limit: u32) -> u32 {
        let i = self
            .children
            .iter()
            .map(|f| f.borrow().report_dirs_under(limit))
            .sum::<u32>();
        if &self.size() < &limit {
            return i + &self.size();
        } else {
            return i;
        }
    }

    pub fn list_dirs_freeing_more_then(&self, limit: u32) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        self.children.iter().for_each(|x| {
            x.borrow()
                .list_dirs_freeing_more_then(limit)
                .iter()
                .for_each(|y| result.push(*y))
        });
        if self.size() > limit {
            result.push(self.size());
        }
        result
    }
}

#[derive(Debug)]
struct File {
    //name:String,
    size: u32,
}
