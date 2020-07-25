use std::convert::TryFrom;

#[derive(Debug)]
pub enum Instruction {
    Process {
        size: u16,
        pid: u16,
    },
    Access {
        address: u16,
        pid: u16,
        modifies: bool,
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
        if let Some(instruction_name) = line_iter.next() {
            match instruction_name {
                "P" => {
                    let size = parse_string_to_u16(line_iter.next(), "P")?;
                    let pid = parse_string_to_u16(line_iter.next(), "P")?;

                    Ok(Instruction::Process { size, pid })
                }
                "A" => {
                    let address = parse_string_to_u16(line_iter.next(), "A")?;
                    let pid = parse_string_to_u16(line_iter.next(), "A")?;
                    let modifies = parse_string_to_bool(line_iter.next(), "A")?;

                    Ok(Instruction::Access {
                        address,
                        pid,
                        modifies,
                    })
                }
                "L" => {
                    let pid = parse_string_to_u16(line_iter.next(), "L")?;
                    Ok(Instruction::Free { pid })
                }
                "C" => Ok(Instruction::Comment(format!(
                    "Comentario: \"{}\"",
                    &value[1..],
                ))),
                "F" => Ok(Instruction::End()),
                "E" => Ok(Instruction::Exit()),
                _ => Err(format!(
                    "Instrucción no reconocida: \"{}\"",
                    instruction_name,
                )),
            }
        } else {
            Err(String::from("Línea vacía"))
        }
    }
}

fn parse_string<T: std::str::FromStr>(
    maybe_string: Option<&str>,
    instruction_name: &str,
    expected_type: &'static str,
) -> Result<T, String> {
    match maybe_string {
        None => Err(format!("Instrucción {} incompleta", instruction_name)),
        Some(string) => match string.parse::<T>() {
            Ok(result) => Ok(result),
            Err(_) => Err(format!(
                "No fue posible parsear \"{}\" a {}",
                string, expected_type,
            )),
        },
    }
}

fn parse_string_to_u16(maybe_string: Option<&str>, instruction_name: &str) -> Result<u16, String> {
    parse_string(
        maybe_string,
        instruction_name,
        "un entero no negativo menor a 65,536",
    )
}

fn parse_string_to_bool(
    maybe_string: Option<&str>,
    instruction_name: &str,
) -> Result<bool, String> {
    match parse_string(maybe_string, instruction_name, "un booleano (0 / 1)") {
        Ok(num) => match num {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(format!("Valor {} no válido para un booleano", num)),
        },
        Err(e) => Err(e),
    }
}
