#[derive(Debug)]
pub struct Memory {
    pub page_size: u16,
    pub m_size: usize,
    pub m: Vec<Option<ProcessFrame>>,
    pub s_size: usize,
    pub s: Vec<Option<ProcessFrame>>,
}

impl Memory {
    pub fn new(page_size: u16, m_size: usize, s_size: usize) -> Memory {
        let m_frames: usize = m_size / (page_size as usize);
        let s_frames: usize = s_size / (page_size as usize);
        Memory {
            page_size,
            m_size,
            m: vec![None; m_frames],
            s_size,
            s: vec![None; s_frames],
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessFrame {
    pub pid: u16,
    pub size: u8,
}
