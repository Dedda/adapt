type Result = std::result::Result<Vec<Actions>, String>;

pub enum Actions {
    Jump(usize),
    Exit(i32),
    Sync,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Code {
    Nop,
    IntData(usize),
    CharData(char),
    FlipType(usize),
    Jump(usize),
    JumpAddr(usize),
    Swap(usize, usize),
    Copy(usize, usize),
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
    Print(usize),
    Exit(i32),
}

impl Code {
    pub fn run(&self, code: &mut Vec<Code>) -> Result {
        match self {
            Self::FlipType(address_at) => {
                let address = read_int_from_address(&code, address_at.clone())?;
                let data = read_from_address(&code, address.clone())?;
                let address = address.clone();
                let data = data.clone();
                if let Code::IntData(i) = data {
                    let _ = std::mem::replace(&mut code[address], Code::CharData(char::from(i.clone() as u8)));
                    Ok(vec![Actions::Sync])
                } else if let Code::CharData(c) = data {
                    let _ = std::mem::replace(&mut code[address], Code::IntData(c.clone() as usize));
                    Ok(vec![Actions::Sync])
                } else {
                    Err(format!("Value at {} is not char or int", address))
                }
            },
            Self::Jump(addr ) => Ok(vec![Actions::Jump(addr.clone())]),
            Self::JumpAddr(addr_addr) => {
                let addr = read_int_from_address(&code, addr_addr.clone())?;
                Ok(vec![Actions::Jump(addr)])
            },
            Self::Swap(addr_addr_1, addr_addr_2) => {
                let addr_1 = read_int_from_address(&code, addr_addr_1.clone())?;
                let addr_2 = read_int_from_address(&code, addr_addr_2.clone())?;
                let data_1 = read_from_address(&code, addr_1)?.clone();
                let data_2 = read_from_address(&code, addr_2)?.clone();
                let _ = std::mem::replace(&mut code[addr_1], data_2);
                let _ = std::mem::replace(&mut code[addr_2], data_1);
                Ok(vec![Actions::Sync])
            },
            Self::Copy(addr_addr_1, addr_addr_2) => {
                let addr_1 = read_int_from_address(&code, addr_addr_1.clone())?;
                let addr_2 = read_int_from_address(&code, addr_addr_2.clone())?;
                let data_1 = read_from_address(&code, addr_1)?.clone();
                let _ = std::mem::replace(&mut code[addr_2], data_1);
                Ok(vec![Actions::Sync])
            },
            Self::Add(addr_addr_1, addr_addr_2) => {
                let addr_1 = read_int_from_address(&code, addr_addr_1.clone())?;
                let addr_2 = read_int_from_address(&code, addr_addr_2.clone())?;
                let int_1 = read_int_from_address(&code, addr_1)?;
                let int_2 = read_int_from_address(&code, addr_2)?;
                let added = Code::IntData(int_1 + int_2);
                let _ = std::mem::replace(&mut code[addr_2], added);
                Ok(vec![Actions::Sync])
            },
            Self::Sub(addr_addr_1, addr_addr_2) => {
                let addr_1 = read_int_from_address(&code, addr_addr_1.clone())?;
                let addr_2 = read_int_from_address(&code, addr_addr_2.clone())?;
                let int_1 = read_int_from_address(&code, addr_1)?;
                let int_2 = read_int_from_address(&code, addr_2)?;
                let subbed = Code::IntData(int_1 - int_2);
                let _ = std::mem::replace(&mut code[addr_2], subbed);
                Ok(vec![Actions::Sync])
            },
            Self::Mul(addr_addr_1, addr_addr_2) => {
                let addr_1 = read_int_from_address(&code, addr_addr_1.clone())?;
                let addr_2 = read_int_from_address(&code, addr_addr_2.clone())?;
                let int_1 = read_int_from_address(&code, addr_1)?;
                let int_2 = read_int_from_address(&code, addr_2)?;
                let mul = Code::IntData(int_1 * int_2);
                let _ = std::mem::replace(&mut code[addr_2], mul);
                Ok(vec![Actions::Sync])
            },
            Self::Div(addr_addr_1, addr_addr_2) => {
                let addr_1 = read_int_from_address(&code, addr_addr_1.clone())?;
                let addr_2 = read_int_from_address(&code, addr_addr_2.clone())?;
                let int_1 = read_int_from_address(&code, addr_1)?;
                let int_2 = read_int_from_address(&code, addr_2)?;
                let div = Code::IntData(int_1 / int_2);
                let _ = std::mem::replace(&mut code[addr_2], div);
                Ok(vec![Actions::Sync])
            },
            Self::Print(addr_addr) => {
                let addr = read_int_from_address(&code, addr_addr.clone())?;
                match code.get(addr.clone()) {
                    None => Err(format!("invalid address {}", addr)),
                    Some(Code::IntData(i)) => {
                        print!("{}", i);
                        Ok(vec![])
                    },
                    Some(Code::CharData(c)) => {
                        print!("{}", c);
                        Ok(vec![])
                    },
                    _ => Err(format!("Invalid data for print at {}", addr))
                }
            }
            Self::Exit(code) => Ok(vec![Actions::Exit(code.clone())]),
            _ => Ok(vec![]),
        }
    }

    pub fn present(&self) -> String {
        use Code::*;
        match self {
            Nop => String::new(),
            IntData(i) => format!("_{}", i),
            CharData(c) => c.to_string(),
            FlipType(addr) => format!("flip type {}", addr),
            Jump(addr) => format!("jump {}", addr),
            JumpAddr(addr_addr) => format!("jump addr {}", addr_addr),
            Swap(addr_addr_1, addr_addr_2) => format!("swap {} {}", addr_addr_1, addr_addr_2),
            Copy(addr_addr_1, addr_addr_2) => format!("copy {} {}", addr_addr_1, addr_addr_2),
            Add(addr_addr_1, addr_addr_2) => format!("add {} {}", addr_addr_1, addr_addr_2),
            Sub(addr_addr_1, addr_addr_2) => format!("sub {} {}", addr_addr_1, addr_addr_2),
            Mul(addr_addr_1, addr_addr_2) => format!("mul {} {}", addr_addr_1, addr_addr_2),
            Div(addr_addr_1, addr_addr_2) => format!("div {} {}", addr_addr_1, addr_addr_2),
            Print(addr_addr) => format!("print {}", addr_addr),
            Exit(code) => format!("exit {}", code),
        }
    }
}

fn read_from_address(code: &Vec<Code>, addr: usize) -> std::result::Result<&Code, String> {
    if let Some(cell) = code.get(addr) {
        Ok(cell)
    } else {
        Err(format!("Cannot load address {}", addr))
    }
}


fn read_int_from_address(code: &Vec<Code>, int_at: usize) -> std::result::Result<usize, String> {
    if let Code::IntData(int) = read_from_address(code, int_at)? {
        Ok(int.clone())
    } else {
        Err(format!("Data at {} is not an integer", int_at))
    }
}