pub fn ceil_division(top: usize, bot: usize) -> usize {
    match top % bot {
        0 => top / bot,
        _ => 1 + top / bot,
    }
}

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

pub fn string_to_u16(maybe_string: Option<&str>, instruction_name: &str) -> Result<u16, String> {
    parse_string(
        maybe_string,
        instruction_name,
        format!("un entero no negativo menor a {}", u16::MAX),
    )
}

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
