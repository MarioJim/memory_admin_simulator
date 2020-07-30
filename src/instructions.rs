use std::convert::TryFrom;
use std::fmt;

#[derive(Debug)]
pub enum Instruction {
    Process {
        pid: u16,
        size: usize,
    },
    Access {
        address: usize,
        modifies: bool,
        pid: u16,
    },
    Free {
        pid: u16,
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
                let pid = string_to_u16(line_iter.next(), "P")?;
                let size = string_to_usize(line_iter.next(), "P")?;
                Ok(Instruction::Process { pid, size })
            }
            Some("A") => {
                let address = string_to_usize(line_iter.next(), "A")?;
                let modifies = string_to_bool(line_iter.next(), "A")?;
                let pid = string_to_u16(line_iter.next(), "A")?;
                Ok(Instruction::Access {
                    address,
                    modifies,
                    pid,
                })
            }
            Some("L") => {
                let pid = string_to_u16(line_iter.next(), "L")?;
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

fn parse_string<T: std::str::FromStr>(
    maybe_string: Option<&str>,
    instruction_name: &str,
    expected_type: String,
) -> Result<T, String> {
    match maybe_string {
        Some(string) => match string.parse::<T>() {
            Ok(result) => Ok(result),
            Err(_) => Err(format!(
                "No fue posible parsear \"{}\" a {}",
                string, expected_type,
            )),
        },
        None => Err(format!("Instrucción {} incompleta", instruction_name)),
    }
}

fn string_to_u16(maybe_string: Option<&str>, instruction_name: &str) -> Result<u16, String> {
    parse_string(
        maybe_string,
        instruction_name,
        format!("un entero no negativo menor a {}", u16::MAX),
    )
}

fn string_to_usize(maybe_string: Option<&str>, instruction_name: &str) -> Result<usize, String> {
    parse_string(
        maybe_string,
        instruction_name,
        format!("un entero no negativo menor a {}", usize::MAX),
    )
}

fn string_to_bool(maybe_string: Option<&str>, instruction_name: &str) -> Result<bool, String> {
    match parse_string(
        maybe_string,
        instruction_name,
        String::from("un booleano (0 / 1)"),
    ) {
        Ok(0) => Ok(false),
        Ok(1) => Ok(true),
        Ok(num) => Err(format!("Número {} no válido para un booleano", num)),
        Err(e) => Err(e),
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
