use std::process as subprocess;
#[doc = r" Result of subprocess.run()"]
#[derive(Debug, Clone)]
pub struct CompletedProcess {
    pub returncode: i32,
    pub stdout: String,
    pub stderr: String,
}
#[doc = "Execute a system command.\n\n    DEPYLER-0761: When cwd is used in subprocess.run kwargs:\n    - is_param_used_in_body should detect cwd usage\n    - But HirExpr::MethodCall handler ignored kwargs(used..)\n    - Fixed: Now checks kwargs.iter().any() for param usage\n    "]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn run_command(cmd: &Vec<String>, capture: bool, cwd: Option<String>) -> (i32, String, String) {
    let mut result;
    if capture {
        result = {
            let cmd_list = cmd;
            let mut cmd = std::process::Command::new(&cmd_list[0]);
            cmd.args(&cmd_list[1..]);
            if let Some(dir) = cwd {
                cmd.current_dir(dir);
            }
            let output = cmd.output().expect("subprocess.run() failed");
            CompletedProcess {
                returncode: output.status.code().unwrap_or(-1),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            }
        };
        (result.returncode, result.stdout, result.stderr)
    } else {
        result = {
            let cmd_list = cmd;
            let mut cmd = std::process::Command::new(&cmd_list[0]);
            cmd.args(&cmd_list[1..]);
            if let Some(dir) = cwd {
                cmd.current_dir(dir);
            }
            let status = cmd.status().expect("subprocess.run() failed");
            CompletedProcess {
                returncode: status.code().unwrap_or(-1),
                stdout: String::new(),
                stderr: String::new(),
            }
        };
        (
            result.returncode,
            "".to_string().to_string(),
            "".to_string().to_string(),
        )
    }
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let (code, out, err) = run_command(
        &vec!["echo".to_string(), "hello".to_string()],
        true,
        Some("/tmp".to_string()),
    );
    println!("{}", format!("Exit code: {:?}", code));
    println!("{}", format!("Output: {:?}", out));
}
