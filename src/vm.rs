use std::{
    collections::{HashMap, LinkedList},
    fs::File,
    io::Read,
};

#[derive(Debug)]
pub struct VirtualMachine {
    graph: NodeGraph,
}

impl VirtualMachine {
    pub fn new(path: &str) -> Self {
        let mut graph_file = File::open(format!("{path}/graph.json")).expect("could not open file");
        let mut buffer = String::new();
        graph_file
            .read_to_string(&mut buffer)
            .expect("could not read file");

        let graph: HashMap<String, Vec<String>> =
            serde_json::from_str(&buffer).expect("could not convert from json");
        println!("{:?}", graph);

        let mut ids: HashMap<String, usize> = HashMap::new();
        let mut nodes = vec![];

        for node in graph.keys() {
            ids.insert(node.clone(), nodes.len());

            let node = NodeMachine::new(format!("{path}/{node}.k"));
            nodes.push(node);
        }

        let mut adj_list = HashMap::new();
        for (node, neighbors) in graph {
            let node_idx = ids[&node];

            let neighbor_idx = neighbors.iter().map(|n| ids[n]).collect();

            adj_list.insert(node_idx, neighbor_idx);
        }

        Self {
            graph: NodeGraph::from(nodes, adj_list),
        }
    }

    pub fn execute(&mut self) {
        self.graph.nodes[0].execute();
    }
}

#[derive(Debug)]
pub struct NodeGraph {
    nodes: Vec<NodeMachine>,
    adj_list: HashMap<usize, Vec<usize>>,
}

impl NodeGraph {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            adj_list: HashMap::new(),
        }
    }

    pub fn from(nodes: Vec<NodeMachine>, adj_list: HashMap<usize, Vec<usize>>) -> Self {
        Self { nodes, adj_list }
    }
}

#[derive(Debug)]
pub struct NodeMachine {
    byte_code: Vec<u8>,
    pc: usize,
    stack: LinkedList<u32>,
    memory: Vec<u8>,
}

impl NodeMachine {
    pub fn new(path: String) -> Self {
        let mut file = File::open(path).expect("could not open file");

        let mut byte_code = vec![];
        file.read_to_end(&mut byte_code)
            .expect("could not read file");

        Self {
            byte_code,
            pc: 0,
            stack: LinkedList::new(),
            memory: vec![],
        }
    }

    pub fn execute(&mut self) {
        while self.pc < self.byte_code.len() {
            let opcode = self.byte_code[self.pc];
            match opcode {
                0x10 => {
                    let data: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    self.stack.push_back(data);

                    self.pc += 4;
                }
                0x11 => {
                    let data: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    self.stack.push_back(data);

                    self.pc += 4;
                }
                0x12 => {
                    self.stack.pop_back();
                }
                0x13 => {
                    self.stack.push_back(self.pc as u32);
                }
                0x20 => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    let addition = addr + 4 - self.memory.len() as u32;

                    for _ in 0..addition {
                        self.memory.push(0);
                    }

                    self.pc += 4;
                }
                0x21 => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    let addition = addr + 4 - self.memory.len() as u32;

                    for _ in 0..addition {
                        self.memory.push(0);
                    }

                    self.pc += 4;
                }
                0x22 => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    let data = (self.memory[addr as usize] as u32) << 24
                        | (self.memory[addr as usize + 1] as u32) << 16
                        | (self.memory[addr as usize + 2] as u32) << 8
                        | (self.memory[addr as usize + 3] as u32);

                    self.stack.push_back(data);

                    self.pc += 4;
                }
                0x23 => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    let data = (self.memory[addr as usize] as u32) << 24
                        | (self.memory[addr as usize + 1] as u32) << 16
                        | (self.memory[addr as usize + 2] as u32) << 8
                        | (self.memory[addr as usize + 3] as u32);

                    self.stack.push_back(data);

                    self.pc += 4;
                }
                0x24 => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    let data = self.stack.pop_back().unwrap();

                    self.memory[addr as usize] = ((data & 0xFF000000) >> 24) as u8;
                    self.memory[addr as usize + 1] = ((data & 0x00FF0000) >> 16) as u8;
                    self.memory[addr as usize + 2] = ((data & 0x0000FF00) >> 8) as u8;
                    self.memory[addr as usize + 3] = (data & 0x000000FF) as u8;

                    self.pc += 4;
                }
                0x25 => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    let data = self.stack.pop_back().unwrap();

                    self.memory[addr as usize] = ((data & 0xFF000000) >> 24) as u8;
                    self.memory[addr as usize + 1] = ((data & 0x00FF0000) >> 16) as u8;
                    self.memory[addr as usize + 2] = ((data & 0x0000FF00) >> 8) as u8;
                    self.memory[addr as usize + 3] = (data & 0x000000FF) as u8;

                    self.pc += 4;
                }
                0x26 => {}
                0x27 => {}
                0x30 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = a + b;
                    self.stack.push_back(res);
                }
                0x31 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = a + b;
                    self.stack.push_back(res);
                }
                0x32 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = a - b;
                    self.stack.push_back(res);
                }
                0x33 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = a - b;
                    self.stack.push_back(res);
                }
                0x34 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = a * b;
                    self.stack.push_back(res);
                }
                0x35 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = a * b;
                    self.stack.push_back(res);
                }
                0x36 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = a / b;
                    self.stack.push_back(res);
                }
                0x37 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = a / b;
                    self.stack.push_back(res);
                }
                0x50 => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    let b = self.stack.pop_back().unwrap();

                    if b != 0 {
                        self.pc = addr as usize;
                        self.pc -= 1;
                    } else {
                        self.pc += 4;
                    }
                }
                0x51 => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    let b = self.stack.pop_back().unwrap();

                    if b == 0 {
                        self.pc = addr as usize;
                        self.pc -= 1;
                    } else {
                        self.pc += 4;
                    }
                }
                0x52 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = if a == b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x53 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = if a != b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x54 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = if a < b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x55 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = if a <= b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x56 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = if a > b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x57 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = if a >= b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x58 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = if (a != 0) && (b != 0) { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x59 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = if (a != 0) || (b != 0) { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x5A => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    self.pc = addr as usize;
                    self.pc -= 1;
                }
                0x5B => {
                    self.pc = self.byte_code.len() + 1;
                }
                _ => {
                    println!("unrecognized opcode! halting");
                    break;
                }
            }
            self.pc += 1;
        }
        println!("{:?}", self.stack);
    }
}