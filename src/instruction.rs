use std::convert::TryFrom;
use std::fmt;

use crate::process::PID;
use crate::util;

#[derive(Debug)]
pub enum Instruction {
    Process {
        pid: PID,
        size: usize,
    },
    Access {
        address: usize,
        modifies: bool,
        pid: PID,
    },
    Free {
        pid: PID,
    },
    Comment(String),
    End(),
    Exit(),
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut line_iter = value.split_ascii_whitespace();
        match line_iter.next() {
            Some("P") => {
                let size = util::string_to_usize(line_iter.next(), "P")?;
                let pid = util::string_to_pid(line_iter.next(), "P")?;
                Ok(Instruction::Process { pid, size })
            }
            Some("A") => {
                let address = util::string_to_usize(line_iter.next(), "A")?;
                let pid = util::string_to_pid(line_iter.next(), "A")?;
                let modifies = util::string_to_bool(line_iter.next(), "A")?;
                Ok(Instruction::Access {
                    address,
                    modifies,
                    pid,
                })
            }
            Some("L") => {
                let pid = util::string_to_pid(line_iter.next(), "L")?;
                Ok(Instruction::Free { pid })
            }
            Some("C") => Ok(Instruction::Comment(String::from(&value[2..]))),
            Some("F") => Ok(Instruction::End()),
            Some("E") => Ok(Instruction::Exit()),
            Some(other) => Err(format!("Instrucción no reconocida: \"{}\"", other)),
            None => Err(String::from("Línea vacía")),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Instruction::Process { pid, size } => write!(f, "P {} {}", *size, *pid),
            Instruction::Access {
                pid,
                address,
                modifies,
            } => write!(
                f,
                "A {} {} {}",
                *address,
                *pid,
                if *modifies { 1 } else { 0 },
            ),
            Instruction::Free { pid } => write!(f, "L {}", *pid),
            Instruction::Comment(string) => write!(f, "C {}", *string),
            Instruction::End() => write!(f, "F"),
            Instruction::Exit() => write!(f, "E"),
        }
    }
}
