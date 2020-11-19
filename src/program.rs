use crate::code::{Actions, Code};
use crate::parse::parse;
use itertools::Itertools;

pub struct Program {
    file_name: String,
    code: Vec<Code>,
}

impl Program {
    pub fn from_file(file_name: String) -> Self {
        let code = parse(&read_file(&file_name)).unwrap();
        Self {
            code,
            file_name,
        }
    }

    pub fn run(&mut self) -> i32 {
        let mut counter = 0;
        loop {
            if let Some(inst) = self.code.get(counter) {
                counter += 1;
                let result = inst.clone().run(&mut self.code);
                match result {
                    Ok(actions) => {
                        for action in actions.iter() {
                            match action {
                                Actions::Jump(address) => counter = address.clone(),
                                Actions::Exit(code) => return code.clone(),
                                Actions::Sync => {
                                    let src = self.code.iter().map(|c| c.present()).join("\n");
                                    write_file(&self.file_name, &src);
                                },
                            }
                        }
                    },
                    Err(msg) => {
                        eprintln!("{}", msg);
                        return 1;
                    }
                }
            } else {
                break;
            }
        }
        0
    }
}

fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

fn write_file(file_name: &str, contents: &str) {
    std::fs::write(file_name, contents).unwrap();
}