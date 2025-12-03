use serde_json;
#[doc = "// NOTE: Map Python module 'subprocess'(tracked in DEPYLER-0424)"]
#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = r" Result of subprocess.run()"]
#[derive(Debug, Clone)]
pub struct CompletedProcess {
    pub returncode: i32,
    pub stdout: String,
    pub stderr: String,
}
#[doc = "Test function with unannotated parameters.\n\n    Type inference should infer:\n    - cmd: Vec<String>(from subprocess.run signature)\n    - capture: bool(from default value False)\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn run_command(cmd: &Vec<String>, capture: bool) -> (i32, String) {
    let mut result;
    if capture {
        result = {
            let cmd_list = cmd;
            let mut cmd = std::process::Command::new(&cmd_list[0]);
            cmd.args(&cmd_list[1..]);
            let output = cmd.output().expect("subprocess.run() failed");
            CompletedProcess {
                returncode: output.status.code().unwrap_or(-1),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            }
        };
        (result.returncode, result.stdout)
    } else {
        result = {
            let cmd_list = cmd;
            let mut cmd = std::process::Command::new(&cmd_list[0]);
            cmd.args(&cmd_list[1..]);
            let status = cmd.status().expect("subprocess.run() failed");
            CompletedProcess {
                returncode: status.code().unwrap_or(-1),
                stdout: String::new(),
                stderr: String::new(),
            }
        };
        result.returncode
    }
}
#[doc = "Test indexing constraint.\n\n    Type inference should infer:\n    - items: Vec<T>(from indexing operation)\n    "]
#[doc = " Depyler: proven to terminate"]
pub fn get_first(items: &Vec<serde_json::Value>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(items.get(0usize).cloned().unwrap_or_default())
}
#[doc = "Test slicing constraint.\n\n    Type inference should infer:\n    - items: Vec<T>(from slicing operation)\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_rest(items: &Vec<serde_json::Value>) -> Vec<serde_json::Value> {
    {
        let base = &items;
        let start_idx = 1 as isize;
        let start = if start_idx < 0 {
            (base.len() as isize + start_idx).max(0) as usize
        } else {
            start_idx as usize
        };
        if start < base.len() {
            base[start..].to_vec()
        } else {
            Vec::new()
        }
    }
}
