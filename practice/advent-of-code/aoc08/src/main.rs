use std::io::{self, Read, Write};
use std::error::Error;
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let numbers: Vec<i32> = input.trim().split(' ')
        .map(|x| x.parse::<i32>().unwrap()).collect();

    let mut root = Node::new(numbers[0], numbers[1]);
    create_tree(0, &mut root, &numbers);

    part1(&root)?;
    part2(&root)?;
    
    Ok(())
}

fn create_tree(start: usize, node: &mut Node, numbers: &Vec<i32>) -> usize {
    if node.header.number_of_nodes == 0 {
        let end = start + 2 + node.header.number_of_metadatas as usize;
        for i in 0..node.header.number_of_metadatas as usize {
            node.metadata.push(numbers[start + 2 + i]);
        }
        return end;
    } else {
        let number_of_nodes = node.header.number_of_nodes as usize;
        let mut start = start + 2;
        for _ in 0..number_of_nodes {
            let mut child = Node::new(numbers[start], numbers[start + 1]);
            start = create_tree(start, &mut child, numbers);
            node.children.push(child);
        }
        let end = start + node.header.number_of_metadatas as usize;
        for i in 0..node.header.number_of_metadatas as usize {
            node.metadata.push(numbers[start + i]);
        }
        return end;
    }
}

fn part1(root: &Node) -> Result<()> {
    let mut answer: i32 = 0;
    let mut stack = vec![];
    stack.push(root);
    while !stack.is_empty() {
        let cur = stack.pop().unwrap();
        stack.extend(&cur.children);
        answer += cur.metadata.iter().sum::<i32>();
    }

    writeln!(io::stdout(), "the sum of all metadata entries: {}", answer)?;
    Ok(())
}

fn part2(root: &Node) -> Result<()> {
    let mut answer = 0;let mut stack = vec![];
    stack.push(root);
    while !stack.is_empty() {
        let cur = stack.pop().unwrap();
        let number_of_nodes = cur.header.number_of_nodes;
        if cur.header.number_of_nodes == 0 {
            answer += cur.metadata.iter().sum::<i32>();
        }
        for &i in cur.metadata.iter() {
            if i == 0 {
                continue;
            }
            if i - 1 < number_of_nodes {
                stack.push(&cur.children[i as usize -1]);
            }
        }
    }
    writeln!(io::stdout(), "the value of the root node: {}", answer)?;
    Ok(())
}

#[derive(Debug)]
struct Node {
    header: Header,
    children: Vec<Node>,
    metadata: Vec<i32>,
}

#[derive(Debug)]

struct Header {
    number_of_nodes: i32,
    number_of_metadatas: i32,
}

impl Node {
    fn new(number_of_nodes: i32, number_of_metadatas: i32) -> Self {
        Node { 
            header: Header { number_of_nodes, number_of_metadatas}, 
            children: vec![], 
            metadata: vec![] 
        }
    }
}