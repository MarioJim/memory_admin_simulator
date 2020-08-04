use std::convert::TryFrom;
use std::fmt;

use crate::process::PID;
use crate::util;

#[derive(Debug)]
/// Usamos este enum para definir el grupo de peticiones que se pueden incluir en el input, así como los argumentos de cada una
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

impl<'a> TryFrom<&'a str> for Instruction {
    type Error = (&'a str, String);

    /// Esta función se encarga parsear la instrucción del input
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut line_iter = value.split_ascii_whitespace();
        match line_iter.next() {
            // En el caso de que se identifiqué correctamente la instrucción P, se retornará un outcome exitoso
            Some("P") => {
                let size = util::string_to_usize(line_iter.next(), "P")
                    .map_err(|err_message| (value, err_message))?;
                let pid = util::string_to_pid(line_iter.next(), "P")
                    .map_err(|err_message| (value, err_message))?;
                Ok(Instruction::Process { pid, size })
            }
            // En el caso de que se identifiqué correctamente la instrucción A, se retornará un outcome exitoso
            Some("A") => {
                let address = util::string_to_usize(line_iter.next(), "A")
                    .map_err(|err_message| (value, err_message))?;
                let pid = util::string_to_pid(line_iter.next(), "A")
                    .map_err(|err_message| (value, err_message))?;
                let modifies = util::string_to_bool(line_iter.next(), "A")
                    .map_err(|err_message| (value, err_message))?;
                Ok(Instruction::Access {
                    address,
                    modifies,
                    pid,
                })
            }
            // En el caso de que se identifiqué correctamente la instrucción L, se retornará un outcome exitoso
            Some("L") => {
                let pid = util::string_to_pid(line_iter.next(), "L")
                    .map_err(|err_message| (value, err_message))?;
                Ok(Instruction::Free { pid })
            }
            // En el caso de que se identifiqué correctamente la instrucción C, se retornará un outcome exitoso
            Some("C") => Ok(Instruction::Comment(String::from(&value[2..]))),
            // En el caso de que se identifiqué correctamente la instrucción F, se retornará un outcome exitoso
            Some("F") => Ok(Instruction::End()),
            // En el caso de que se identifiqué correctamente la instrucción E, se retornará un outcome exitoso
            Some("E") => Ok(Instruction::Exit()),
            // En el caso de que el input incluya una instrucción agena a la función del programa se retornará un outcome fallido/error
            Some(other) => Err((value, format!("Instrucción no reconocida: \"{}\"", other))),
            // En el caso de que el input incluya una instrucción vacía se retornará un outcome fallido/error
            None => Err((value, String::from("Línea vacía"))),
        }
    }
}

impl fmt::Display for Instruction {
    /// Usamos esta función para identificar el tipo de instrucción que se quiere mostrar
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // En este caso el comando match se encarga de ejecutar el código que imprimirá lo que se hará en cada solicitud
        match &self {
            // Se imprime la cantidad de bytes que se asignarán al proceso de la solicitud
            Instruction::Process { pid, size } => {
                writeln!(f, "P {} {}", *size, *pid)?;
                write!(f, "Asignar {} bytes al proceso {}", *size, *pid)
            }
            // Se imprime la dirección virtual y el proceso correspondiente de la que se accesará
            Instruction::Access {
                pid,
                address,
                modifies,
            } => {
                writeln!(
                    f,
                    "A {} {} {}",
                    *address,
                    *pid,
                    if *modifies { 1 } else { 0 },
                )?;
                write!(
                    f,
                    "Obtener la dirección real correspondiente a la dirección virtual {} del proceso {}{}",
                    *address,
                    *pid,
                    if *modifies { " y modificar dicha dirección" } else { "" },
                )
            }
            // Se imprime el proceso del que se liberaran todos los marcos de página que este ocupa en la memoria virtual
            Instruction::Free { pid } => {
                writeln!(f, "L {}", *pid)?;
                write!(
                    f,
                    "Liberar los marcos de página ocupados por el proceso {}",
                    *pid
                )
            }
            // Se imprime el comentario que se ingresó en el input
            Instruction::Comment(string) => write!(f, "C {}", *string),
            // Se imprime el fin del conjunto de solicitudes
            Instruction::End() => {
                writeln!(f, "F")?;
                write!(f, "Fin. Reporte de salida:")
            }
            // Se imprime lo que representa el final del archivo
            Instruction::Exit() => write!(f, "E"),
        }
    }
}
