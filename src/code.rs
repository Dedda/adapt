use std::ops::*;

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
    JumpAddrCmp(usize, usize, usize, usize, usize),
    Swap(usize, usize),
    Copy(usize, usize),
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
    Del(usize),
    Print(usize),
    Exit(i32),
}

impl Code {
    pub fn run(&self, mut code: &mut Vec<Code>) -> Result {
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
            Self::JumpAddrCmp(left_addr_addr, right_addr_addr, lt_addr_addr, eq_addr_addr, gt_addr_addr) => {
                let left_addr = read_int_from_address(&code, left_addr_addr.clone())?;
                let left = read_int_from_address(&code, left_addr)?;
                let right_addr = read_int_from_address(&code, right_addr_addr.clone())?;
                let right = read_int_from_address(&code, right_addr)?;
                let sub = left as i128 - right as i128;
                let jump_addr = match sub.signum() {
                    -1 => {
                        read_int_from_address(&code, lt_addr_addr.clone())?
                    },
                    1 => {
                        read_int_from_address(&code, gt_addr_addr.clone())?
                    },
                    _ => {
                        read_int_from_address(&code, eq_addr_addr.clone())?
                    },
                };
                Ok(vec![Actions::Jump(jump_addr)])

            }
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
                let addr_2 = read_int_from_address(&code, addr_addr_2.clone())?;
                let data_1 = read_from_address_address(&code, addr_addr_1.clone())?.clone();
                let _ = std::mem::replace(&mut code[addr_2], data_1);
                Ok(vec![Actions::Sync])
            },
            Self::Add(addr_addr_1, addr_addr_2) => {
                run_binary_int_operation(&mut code, addr_addr_1, addr_addr_2, usize::add )
            },
            Self::Sub(addr_addr_1, addr_addr_2) => {
                run_binary_int_operation(&mut code, addr_addr_1, addr_addr_2, usize::sub )
            },
            Self::Mul(addr_addr_1, addr_addr_2) => {
                run_binary_int_operation(&mut code, addr_addr_1, addr_addr_2, usize::mul )
            },
            Self::Div(addr_addr_1, addr_addr_2) => {
                run_binary_int_operation(&mut code, addr_addr_1, addr_addr_2, usize::div )
            },
            Self::Del(del_addr_addr) => {
                let del_addr = read_int_from_address(&code, del_addr_addr.clone())?;
                let _ = code.remove(del_addr);
                Ok(vec![Actions::Sync])
            }
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
            JumpAddrCmp(left, right, lt, eq, gt) => format!("jump addr cmp {} {} {} {} {}", left, right, lt, eq, gt),
            Swap(addr_addr_1, addr_addr_2) => format!("swap {} {}", addr_addr_1, addr_addr_2),
            Copy(addr_addr_1, addr_addr_2) => format!("copy {} {}", addr_addr_1, addr_addr_2),
            Add(addr_addr_1, addr_addr_2) => format!("add {} {}", addr_addr_1, addr_addr_2),
            Sub(addr_addr_1, addr_addr_2) => format!("sub {} {}", addr_addr_1, addr_addr_2),
            Mul(addr_addr_1, addr_addr_2) => format!("mul {} {}", addr_addr_1, addr_addr_2),
            Div(addr_addr_1, addr_addr_2) => format!("div {} {}", addr_addr_1, addr_addr_2),
            Del(del_addr_addr) => format!("del {}", del_addr_addr),
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

fn read_from_address_address(code: &Vec<Code>, addr_addr: usize) -> std::result::Result<&Code, String> {
    let addr = read_int_from_address(&code, addr_addr)?;
    read_from_address(&code, addr)
}

fn read_int_from_address_address(code: &Vec<Code>, addr_addr: usize) -> std::result::Result<usize, String> {
    let addr = read_int_from_address(&code, addr_addr)?;
    read_int_from_address(&code, addr)
}

fn resolve_binary_int_operand_args(code: &Vec<Code>, left_addr_addr: &usize, right_addr_addr: &usize) -> std::result::Result<(usize, usize, usize), String> {
    let left = read_int_from_address_address(&code, left_addr_addr.clone())?;
    let dst = read_int_from_address(&code, right_addr_addr.clone())?;
    let right = read_int_from_address(&code, dst.clone())?;
    Ok((left, right, dst))
}

fn run_binary_int_operation(code: &mut Vec<Code>, left_addr_addr: &usize, right_addr_addr: &usize, op: impl Fn(usize, usize) -> usize) -> Result {
    let (left, right, dst) = resolve_binary_int_operand_args(&code, left_addr_addr, right_addr_addr)?;
    let result = op(left, right);
    let _ = std::mem::replace(&mut code[dst], Code::IntData(result));
    Ok(vec![Actions::Sync])
}

#[cfg(test)]
mod tests {
    mod read_from_address {
        use crate::code::{Code, read_from_address};

        #[test]
        fn read_existing() {
            let code = vec![Code::Nop, Code::IntData(123), Code::CharData('c')];
            assert_eq!(&Code::IntData(123), read_from_address(&code, 1).unwrap());
        }

        #[test]
        fn read_out_of_bounds() {
            let code = vec![Code::Nop, Code::IntData(123), Code::CharData('c')];
            assert!(read_from_address(&code, 3).is_err());
        }
    }
}
