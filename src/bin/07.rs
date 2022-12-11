use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    fmt::Debug,
    ops::Deref,
    rc::{Rc, Weak},
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, digit1, space1},
    combinator::{map, map_res},
    sequence::{preceded, tuple},
    IResult,
};

trait Size {
    fn size(&self) -> u32;
}

#[derive(Debug)]
struct FS {
    root: NodePtr<Entry>,
    cur: NodePtr<Entry>,
}

impl FS {
    fn new() -> Self {
        let root = Node::new(Entry::root(), HashMap::new(), None);
        let root: NodePtr<_> = Rc::new(RefCell::new(root)).into();
        let cur = root.clone();
        Self { root, cur }
    }

    fn cd(&mut self, path: &str) {
        match path {
            "/" => self.cur = self.root.clone(),
            ".." => {
                let parent = self.cur.borrow().parent.clone().expect("no parent");
                self.cur = parent.upgrade().expect("parent is dropped").into();
            }
            s => {
                let cur = self.cur.borrow();
                let target = cur
                    .children
                    .iter()
                    .find(|(name, e)| name.as_str() == s && matches!(e.borrow().val, Entry::Dir(_)))
                    .expect("no dir")
                    .1
                    .clone();

                drop(cur);
                self.cur = target;
            }
        }
    }

    fn insert(&mut self, entry: Entry) {
        let name = entry.name();
        let mut cur = self.cur.borrow_mut();
        cur.children.insert(
            name.to_string(),
            NodePtr::from_node(Node::new(
                entry,
                HashMap::new(),
                Some(self.cur.clone().into_weak()),
            )),
        );
    }
}

impl IntoIterator for FS {
    type Item = NodePtr<Entry>;
    type IntoIter = NodeIter<Entry>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIter::new(self.root)
    }
}

#[derive(Debug)]
struct NodePtr<T>(Rc<RefCell<Node<T>>>);

impl<T> NodePtr<T> {
    fn is_leaf(&self) -> bool {
        self.borrow().is_leaf()
    }

    fn from_node(node: Node<T>) -> Self {
        Rc::new(RefCell::new(node)).into()
    }

    fn into_weak(self) -> NodeWeakPtr<T> {
        Rc::downgrade(&self.0)
    }

    fn val(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |v| &v.val)
    }

    fn val_mut(&self) -> RefMut<T> {
        RefMut::map(self.0.borrow_mut(), |v| &mut v.val)
    }

    fn set_seen(&self) {
        self.borrow_mut().set_seen();
    }

    fn is_seen(&self) -> bool {
        self.borrow().seen
    }
}

impl NodePtr<Entry> {
    fn size(&self) -> u32 {
        match self.val().deref() {
            Entry::File(s) => s.size(),
            Entry::Dir(_) => self
                .borrow()
                .children
                .values()
                .map(|e| e.size())
                .sum::<u32>(),
        }
    }

    fn is_dir(&self) -> bool {
        matches!(self.val().deref(), Entry::Dir(_))
    }

    fn is_file(&self) -> bool {
        matches!(self.val().deref(), Entry::File(_))
    }
}

impl<T> Deref for NodePtr<T> {
    type Target = Rc<RefCell<Node<T>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for NodePtr<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<Rc<RefCell<Node<T>>>> for NodePtr<T> {
    fn from(n: Rc<RefCell<Node<T>>>) -> Self {
        Self(n)
    }
}

struct NodeIter<T> {
    cur: NodePtr<T>,
    has_next: bool,
}

impl<T> NodeIter<T> {
    pub fn new(cur: NodePtr<T>) -> Self {
        Self {
            cur,
            has_next: true,
        }
    }
}

impl<T: Debug> NodeIter<T> {
    fn move_to_parent(&mut self) {
        let parent = self.cur.borrow().parent.clone();
        if let Some(parent) = parent {
            self.cur = parent.upgrade().expect("parent is dropped").into();
        } else {
            self.has_next = false;
        }
    }

    fn find_not_seen_children(&self) -> Option<NodePtr<T>> {
        let cur = self.cur.clone();
        let next = cur
            .borrow()
            .children
            .iter()
            .find(|(_, v)| !v.is_seen())
            .map(|(_, v)| v.clone());

        next
    }
}

impl<T: Debug> Iterator for NodeIter<T> {
    type Item = NodePtr<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next {
            None
        } else {
            let child = self.find_not_seen_children();
            if let Some(child) = child {
                self.cur = child;
                self.next()
            } else if !self.cur.is_seen() {
                self.cur.set_seen();
                Some(self.cur.clone())
            } else {
                self.move_to_parent();
                self.next()
            }
        }
    }
}

type NodeWeakPtr<T> = Weak<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    seen: bool,
    children: HashMap<String, NodePtr<T>>,
    parent: Option<NodeWeakPtr<T>>,
}

impl<T> Node<T> {
    fn new(val: T, children: HashMap<String, NodePtr<T>>, parent: Option<NodeWeakPtr<T>>) -> Self {
        Self {
            val,
            seen: false,
            children,
            parent,
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn set_seen(&mut self) {
        self.seen = true;
    }
}

trait Name {
    type Item<'a>
    where
        Self: 'a;
    fn name(&self) -> Self::Item<'_>;
}

impl Name for Entry {
    type Item<'a> = &'a str;

    fn name(&self) -> Self::Item<'_> {
        match self {
            Entry::File(f) => f.name.as_ref(),
            Entry::Dir(d) => d.name.as_ref(),
        }
    }
}

#[derive(Debug)]
enum Entry {
    File(File),
    Dir(Directory),
}

impl Entry {
    fn root() -> Self {
        Entry::Dir(Directory::new("/".to_string()))
    }
}

impl From<LsResult> for Entry {
    fn from(r: LsResult) -> Self {
        match r {
            LsResult::File(size, name) => Entry::File(File::new(name, size)),
            LsResult::Dir(name) => Entry::Dir(Directory::new(name)),
        }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Directory { name }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl Size for File {
    fn size(&self) -> u32 {
        self.size
    }
}

impl File {
    pub fn new(name: impl Into<String>, size: u32) -> Self {
        File {
            name: name.into(),
            size,
        }
    }
}

enum LsResult {
    File(u32, String),
    Dir(String),
}

enum Command {
    Cd(String),
    Ls,
}

fn parse_command_line(s: &str) -> IResult<&str, Command> {
    preceded(tag("$ "), alt((parse_cd, parse_ls)))(s)
}

fn parse_cd(s: &str) -> IResult<&str, Command> {
    map(preceded(tag("cd "), parse_ident), |s: &str| {
        Command::Cd(s.into())
    })(s)
}

fn parse_ls(s: &str) -> IResult<&str, Command> {
    map(tag("ls"), |_| Command::Ls)(s)
}

fn parse_dir_name(s: &str) -> IResult<&str, &str> {
    preceded(tag("dir "), alpha1)(s)
}

fn parse_ident(s: &str) -> IResult<&str, &str> {
    take_while(|v: char| !v.is_whitespace())(s)
}

fn parse_file(s: &str) -> IResult<&str, (u32, String)> {
    map_res(
        tuple((digit1, space1, parse_ident)),
        |(size, _, name): (&str, &str, &str)| {
            Ok::<_, <u32 as FromStr>::Err>((size.parse()?, name.into()))
        },
    )(s)
}

fn parse_ls_result(s: &str) -> IResult<&str, LsResult> {
    alt((
        map(parse_file, |(s, n)| LsResult::File(s, n)),
        map(parse_dir_name, |n| LsResult::Dir(n.into())),
    ))(s)
}

fn process(s: &str) -> IResult<&str, u32> {
    let mut fs = FS::new();
    let mut lines = s.lines().peekable();
    loop {
        let Some(s) = lines.next() else {
            break;
        };

        let (remain, command) = parse_command_line(s)?;
        assert!(remain.is_empty());
        match command {
            Command::Cd(dir) => fs.cd(&dir),
            Command::Ls => {
                let mut results = Vec::new();
                while let Some(s) = lines.peek() {
                    match parse_ls_result(s) {
                        Ok(result) => {
                            results.push(result.1);
                            lines.next();
                        }
                        Err(_) => break,
                    }
                }
                for result in results.into_iter().map(Into::into) {
                    fs.insert(result);
                }
            }
        }
    }
    // dbg!(&fs);

    let sum = fs
        .into_iter()
        .filter_map(|v| {
            let size = v.size();
            if size <= 100000 && v.is_dir() {
                Some(size)
            } else {
                None
            }
        })
        .sum();
    Ok(("", sum))
}

pub fn part_one(input: &str) -> Option<u32> {
    let a = process(input).expect("err");
    Some(a.1)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
