use std::ops::Range;

use crate::time::Time;
use crate::util;

/// Usamos un alias de número específicamente para el pid para que sea fácil cambiarlo
/// en caso de ser necesario
pub type PID = u16;

/// Guarda la información de un proceso:
/// - pid: número que identifica el proceso
/// - size: tamaño del proceso en bytes
/// - life: rango de tiempo del sistema desde que las páginas del proceso terminaron de cargarse hasta que las páginas del proceso terminaron de liberarse
/// - swap_ins: número de veces en las que ha sido necesario que una página del proceso se mueva hacia la memoria real del sistema
/// - swap_outs: número de veces en las que ha sido necesario que una página del proceso se mueva hacia el espacio swap del sistema
#[derive(Debug)]
pub struct Process {
    pid: PID,
    size: usize,
    life: Range<Time>,
    swap_ins: u16,
    swap_outs: u16,
}

impl Process {
    /// Constructor al que se le pasa el pid y el tamaño en bytes
    pub fn new(pid: PID, size: usize) -> Self {
        Process {
            pid,
            size,
            life: (Time::new()..Time::max()),
            swap_ins: 0,
            swap_outs: 0,
        }
    }

    /// Get para el pid del proceso
    pub fn get_pid(&self) -> PID {
        self.pid
    }

    /// Calcula el número de páginas dependiendo del tamaño de la página
    pub fn num_pages(&self, page_size: usize) -> usize {
        util::ceil_div(self.size, page_size)
    }

    /// Checa si el tamaño del proceso es mayor al de la dirección virtual
    pub fn includes_address(&self, address: usize) -> bool {
        address < self.size
    }

    /// Añade uno al contador de swap-ins
    pub fn add_swap_in(&mut self) {
        self.swap_ins += 1;
    }

    /// Añade uno al contador de swap-ins
    pub fn add_swap_out(&mut self) {
        self.swap_outs += 1;
    }

    /// Regresa un tuple formado por el número de swap-ins y swap-outs
    pub fn get_swaps(&self) -> (u16, u16) {
        (self.swap_ins, self.swap_outs)
    }

    // Set para el nacimiento del proceso
    pub fn set_birth(&mut self, birth: Time) {
        self.life.start = birth;
    }

    // Set para la muerte del proceso
    pub fn set_death(&mut self, death: Time) {
        self.life.end = death;
    }

    // Regresa un string con la vida del proceso
    pub fn display_life(&self) -> String {
        format!("{} - {}", self.life.start, self.life.end)
    }

    // Calcula el tiempo de turnaround del proceso
    pub fn calc_turnaround(&self) -> Time {
        self.life.end - self.life.start
    }
}

///Parte de la memoria virtual del proceso
/// Guarda dentro de ella:
/// - pid: número que identifica al proceso que pertenece
/// - index: índice de la página dentro de la memoria virtual del proceso
/// - created: tiempo del sistema en el que se creó la página
/// - accessed: tiempo del sistema la última vez que se accedió a la página
#[derive(Debug)]
pub struct ProcessPage {
    pid: PID,
    index: usize,
    created: Time,
    accessed: Time,
}

impl ProcessPage {
    /// Constructor de la estructura, recibe el pid, el índice dentro de la memoria virtual
    /// y el tiempo de creación
    pub fn new(pid: PID, index: usize, created: Time) -> Self {
        ProcessPage {
            pid,
            index,
            created,
            accessed: created,
        }
    }

    /// Get para el pid de la página
    pub fn get_pid(&self) -> PID {
        self.pid
    }

    /// Regresa un tuple formado por el pid y el índice de la página
    pub fn get_page_info(&self) -> (PID, usize) {
        (self.pid, self.index)
    }

    /// Get para una referencia al tiempo de creación del proceso
    pub fn get_created_time(&self) -> &Time {
        &self.created
    }

    /// Get para una referencia al tiempo de acceso del proceso
    pub fn get_accessed_time(&self) -> &Time {
        &self.accessed
    }

    /// Set para el tiempo de creación del proceso
    pub fn update_accessed_time(&mut self, accessed: Time) {
        self.accessed = accessed;
    }
}
