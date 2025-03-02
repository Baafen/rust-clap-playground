use std::fmt;

pub struct CmdPath {
    pub path: Vec<String>,
}

impl CmdPath {
    pub fn new() -> CmdPath {
        CmdPath{
            path: Vec::new(),
        }
    }

    pub fn get_path(&self) -> Vec<&str> {
        let mut vec: Vec<&str> = Vec::new();

        for p in &self.path {
            vec.push(&p)
        }

        vec
    }
}

impl fmt::Display for CmdPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.join("/"))
    }
}
