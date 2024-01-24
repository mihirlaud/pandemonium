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
        println!("{:?}", self.byte_code);
        println!("{} bytes", self.byte_code.len());
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
                    let data = u32::from_be_bytes([
                        self.byte_code[self.pc + 1],
                        self.byte_code[self.pc + 2],
                        self.byte_code[self.pc + 3],
                        self.byte_code[self.pc + 4],
                    ]);

                    self.stack.push_back(data);

                    self.pc += 4;
                }
                0x12 => {
                    self.stack.pop_back();
                }
                0x13 => {
                    let offset: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    self.stack.push_back(self.pc as u32 + offset);

                    self.pc += 4;
                }
                0x14 => {
                    let data = self.byte_code[self.pc + 1] as u32;

                    self.stack.push_back(data);

                    self.pc += 1;
                }
                0x15 => {
                    let data = self.byte_code[self.pc + 1] as u32;

                    self.stack.push_back(data);

                    self.pc += 1;
                }
                0x20 => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    if addr + 4 > self.memory.len() as u32 {
                        let addition = addr + 4 - self.memory.len() as u32;

                        for _ in 0..addition {
                            self.memory.push(0);
                        }
                    }

                    self.pc += 4;
                }
                0x21 => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    if addr + 4 > self.memory.len() as u32 {
                        let addition = addr + 4 - self.memory.len() as u32;

                        for _ in 0..addition {
                            self.memory.push(0);
                        }
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
                0x28 => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    if addr + 1 > self.memory.len() as u32 {
                        let addition = addr + 1 - self.memory.len() as u32;

                        for _ in 0..addition {
                            self.memory.push(0);
                        }
                    }

                    self.pc += 4;
                }
                0x29 => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    let data = self.memory[addr as usize] as u32;

                    self.stack.push_back(data);

                    self.pc += 4;
                }
                0x2A => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    let data = self.stack.pop_back().unwrap();

                    self.memory[addr as usize] = (data & 0xFF) as u8;

                    self.pc += 4;
                }
                0x2C => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    if addr + 1 > self.memory.len() as u32 {
                        let addition = addr + 1 - self.memory.len() as u32;

                        for _ in 0..addition {
                            self.memory.push(0);
                        }
                    }

                    self.pc += 4;
                }
                0x2D => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    let data = self.memory[addr as usize] as u32;

                    self.stack.push_back(data);

                    self.pc += 4;
                }
                0x2E => {
                    let addr: u32 = ((self.byte_code[self.pc + 1] as u32) << 24)
                        | ((self.byte_code[self.pc + 2] as u32) << 16)
                        | ((self.byte_code[self.pc + 3] as u32) << 8)
                        | (self.byte_code[self.pc + 4] as u32);

                    let data = self.stack.pop_back().unwrap();

                    self.memory[addr as usize] = (data & 0xFF) as u8;

                    self.pc += 4;
                }
                0x30 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = a + b;
                    self.stack.push_back(res);
                }
                0x31 => {
                    let b = self.stack.pop_back().unwrap();
                    let b = f32::from_be_bytes(b.to_be_bytes());
                    let a = self.stack.pop_back().unwrap();
                    let a = f32::from_be_bytes(a.to_be_bytes());
                    let res = a + b;
                    let res = u32::from_be_bytes(res.to_be_bytes());
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
                    let b = f32::from_be_bytes(b.to_be_bytes());
                    let a = self.stack.pop_back().unwrap();
                    let a = f32::from_be_bytes(a.to_be_bytes());
                    let res = a - b;
                    let res = u32::from_be_bytes(res.to_be_bytes());
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
                    let b = f32::from_be_bytes(b.to_be_bytes());
                    let a = self.stack.pop_back().unwrap();
                    let a = f32::from_be_bytes(a.to_be_bytes());
                    let res = a * b;
                    let res = u32::from_be_bytes(res.to_be_bytes());
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
                    let b = f32::from_be_bytes(b.to_be_bytes());
                    let a = self.stack.pop_back().unwrap();
                    let a = f32::from_be_bytes(a.to_be_bytes());
                    let res = a / b;
                    let res = u32::from_be_bytes(res.to_be_bytes());
                    self.stack.push_back(res);
                }
                0x38 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = a + b;
                    self.stack.push_back(res);
                }
                0x39 => {
                    let b = self.stack.pop_back().unwrap();
                    let a = self.stack.pop_back().unwrap();
                    let res = a - b;
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
                    let res = self.stack.pop_back().unwrap();

                    match self.stack.pop_back() {
                        Some(i) => {
                            self.pc = i as usize - 1;
                        }
                        None => {
                            self.pc = self.byte_code.len();
                        }
                    }

                    self.stack.push_back(res);
                }
                0x5C => {
                    let b = self.stack.pop_back().unwrap();
                    let b = f32::from_be_bytes(b.to_be_bytes());
                    let a = self.stack.pop_back().unwrap();
                    let a = f32::from_be_bytes(a.to_be_bytes());
                    let res = if a == b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x5D => {
                    let b = self.stack.pop_back().unwrap();
                    let b = f32::from_be_bytes(b.to_be_bytes());
                    let a = self.stack.pop_back().unwrap();
                    let a = f32::from_be_bytes(a.to_be_bytes());
                    let res = if a != b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x5E => {
                    let b = self.stack.pop_back().unwrap();
                    let b = f32::from_be_bytes(b.to_be_bytes());
                    let a = self.stack.pop_back().unwrap();
                    let a = f32::from_be_bytes(a.to_be_bytes());
                    let res = if a < b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x5F => {
                    let b = self.stack.pop_back().unwrap();
                    let b = f32::from_be_bytes(b.to_be_bytes());
                    let a = self.stack.pop_back().unwrap();
                    let a = f32::from_be_bytes(a.to_be_bytes());
                    let res = if a <= b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x60 => {
                    let b = self.stack.pop_back().unwrap();
                    let b = f32::from_be_bytes(b.to_be_bytes());
                    let a = self.stack.pop_back().unwrap();
                    let a = f32::from_be_bytes(a.to_be_bytes());
                    let res = if a > b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x61 => {
                    let b = self.stack.pop_back().unwrap();
                    let b = f32::from_be_bytes(b.to_be_bytes());
                    let a = self.stack.pop_back().unwrap();
                    let a = f32::from_be_bytes(a.to_be_bytes());
                    let res = if a >= b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x62 => {
                    let b = self.stack.pop_back().unwrap() == 1;
                    let a = self.stack.pop_back().unwrap() == 1;
                    let res = if a == b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x63 => {
                    let b = self.stack.pop_back().unwrap() == 1;
                    let a = self.stack.pop_back().unwrap() == 1;
                    let res = if a != b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x64 => {
                    let b = self.stack.pop_back().unwrap() == 1;
                    let a = self.stack.pop_back().unwrap() == 1;
                    let res = if a < b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x65 => {
                    let b = self.stack.pop_back().unwrap() == 1;
                    let a = self.stack.pop_back().unwrap() == 1;
                    let res = if a <= b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x66 => {
                    let b = self.stack.pop_back().unwrap() == 1;
                    let a = self.stack.pop_back().unwrap() == 1;
                    let res = if a > b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                0x67 => {
                    let b = self.stack.pop_back().unwrap() == 1;
                    let a = self.stack.pop_back().unwrap() == 1;
                    let res = if a >= b { 1 } else { 0 };
                    self.stack.push_back(res);
                }
                _ => {
                    println!("unrecognized opcode {opcode} !!! halting");
                    break;
                }
            }
            self.pc += 1;
        }
        println!("{:?}", self.stack);
        println!("{:?}", self.memory);
    }
}
