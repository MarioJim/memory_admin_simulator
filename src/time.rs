use std::cmp;
use std::fmt;
use std::ops;

/// Para guardar el tiempo, en vez de guardarlo como un número con punto flotante implementamos
/// una estructura que guarda el tiempo en milésimas de segundo como un número no negativo
/// (u de unsigned) de 32 bits (32 de u32)
#[derive(Debug, Clone, Copy)]
pub struct Time(u32);

impl Time {
    /// Constructor de Time con un valor inicial de 0
    pub fn new() -> Self {
        Time(0)
    }

    /// Constructor de Time con un valor proveído en milisegundos
    pub const fn from_miliseconds(ms: u32) -> Self {
        Time(ms)
    }

    /// Constructor de Time con un valor inicial máximo
    pub fn max() -> Self {
        Time(u32::MAX)
    }
}

/// Este trait (interfaz en idioma Rust) permite definir una forma de imprimir el valor personalizada
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // En este caso imprimimos el número entre 1000 y con una s al final
        write!(f, "{}s", f64::from(*self) / 1000.0)
    }
}

// Fue necesario implementar operaciones básicas como:
/// Suma
impl ops::Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        Time(self.0 + rhs.0)
    }
}

/// Suma con asignación
impl ops::AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

/// Resta
impl ops::Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        Time(self.0 - rhs.0)
    }
}

/// Igualdad parcial
impl cmp::PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Igualdad
impl cmp::Eq for Time {}

/// Ordenamiento parcial
impl cmp::PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

/// Ordenamiento
impl cmp::Ord for Time {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

/// Casteo a un número de punto flotante
impl From<Time> for f64 {
    fn from(time: Time) -> Self {
        time.0 as f64
    }
}
