use std::ops::Range;

use crate::process::PID;

/// Esta función recibe dos números enteros y los divide regresando el número entero
/// igual o mayor a la división
pub fn ceil_div(top: usize, bot: usize) -> usize {
    match top % bot {
        0 => top / bot,
        _ => 1 + top / bot,
    }
}

/// Esta función se encarga de parsear un string y regresar un elemento genérico. En el caso de
/// que el parseo sea exitoso se retornará un outcome exitoso, en el caso contrario se retornará
/// un outcome fallido y su correspondiente mensaje de error
pub fn parse_string<T: std::str::FromStr>(
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

/// Esta función se encarga de parsear un string y regresar un número entero. En el caso de que el
/// parseo sea exitoso se retornará un outcome exitoso, en el caso contrario se retornará un
/// outcome fallido y su correspondiente mensaje de error
pub fn string_to_pid(maybe_string: Option<&str>, instruction_name: &str) -> Result<PID, String> {
    parse_string(
        maybe_string,
        instruction_name,
        format!("un entero no negativo menor a {}", PID::MAX),
    )
}

/// Esta función se encarga de parsear un string y regresar un número entero. En el caso de que el
/// parseo sea exitoso se retornará un outcome exitoso, en el caso contrario se retornará un
/// outcome fallido y su correspondiente mensaje de error
pub fn string_to_usize(
    maybe_string: Option<&str>,
    instruction_name: &str,
) -> Result<usize, String> {
    parse_string(
        maybe_string,
        instruction_name,
        format!("un entero no negativo menor a {}", usize::MAX),
    )
}

/// Esta función se encarga de parsear un string y regresar un booleano. En el caso de que el
/// parseo sea exitoso se retornará un outcome exitoso, en el caso contrario se retornará un
/// outcome fallido y su correspondiente mensaje de error
pub fn string_to_bool(maybe_string: Option<&str>, instruction_name: &str) -> Result<bool, String> {
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

/// Esta función se encarga de retornar un string que represente un rango sólo si el vector
/// tiene elementos
pub fn display_ranges_vec(vector: &Vec<Range<usize>>) -> Option<String> {
    if vector.is_empty() {
        None
    } else {
        Some(
            vector
                .iter()
                .map(|range| {
                    if range.start != range.end {
                        format!("{} a {}", range.start, range.end)
                    } else {
                        format!("{}", range.start)
                    }
                })
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

/// Esta función recibe un número y una referencia a una lista de rangos, y añade el número a
/// los rangos
pub fn add_index_to_vec_of_ranges(index: usize, vec_of_ranges: &mut Vec<Range<usize>>) {
    match vec_of_ranges.last_mut() {
        // Si el final del último rango más uno es igual al número, le suma uno al final del rango
        Some(Range { start: _, end }) if *end + 1 == index => *end = index,
        // Si el vector de rangos está vacío o el último rango no cumple con el caso anterior
        // se añade un nuevo rango a la lista de rangos
        Some(_) | None => vec_of_ranges.push(Range {
            start: index,
            end: index,
        }),
    }
}
