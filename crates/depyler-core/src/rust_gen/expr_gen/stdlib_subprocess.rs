//! Subprocess stdlib code generation
//!
//! Handles conversion of Python subprocess module calls to Rust std::process::Command.

use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use anyhow::{bail, Result};
use syn::parse_quote;

use super::ExpressionConverter;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// Convert subprocess.run() to std::process::Command
    /// DEPYLER-0391: Subprocess module for executing system commands
    ///
    /// Maps Python subprocess.run() to Rust std::process::Command:
    /// - subprocess.run(cmd) → Command::new(cmd[0]).args(&cmd[1..]).status()
    /// - capture_output=True → .output() instead of .status()
    /// - cwd=path → .current_dir(path)
    /// - check=True → verify exit status (NOTE: add error handling tracked in DEPYLER-0424)
    ///
    /// Returns anonymous struct with: returncode, stdout, stderr
    ///
    /// # Complexity
    /// ≤10 (linear processing of kwargs)
    #[inline]
    pub(super) fn convert_subprocess_run(
        &mut self,
        args: &[HirExpr],
        kwargs: &[(Symbol, HirExpr)],
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            bail!("subprocess.run() requires at least 1 argument (command list)");
        }

        // First argument is the command list
        let cmd_expr = args[0].to_rust_expr(self.ctx)?;

        // Parse keyword arguments
        let mut capture_output = false;
        let mut _text = false;
        let mut cwd_expr: Option<syn::Expr> = None;
        let mut cwd_is_option = false; // DEPYLER-0950: Track if cwd is Option type
        let mut _check = false;

        for (key, value) in kwargs {
            match key.as_str() {
                "capture_output" => {
                    if let HirExpr::Literal(Literal::Bool(b)) = value {
                        capture_output = *b;
                    }
                }
                "text" => {
                    if let HirExpr::Literal(Literal::Bool(b)) = value {
                        _text = *b;
                    }
                }
                "cwd" => {
                    cwd_expr = Some(value.to_rust_expr(self.ctx)?);
                    // DEPYLER-0950: Check if cwd value is likely an Option type
                    // Variables with Optional type annotation or None default need if-let Some()
                    // Expressions like list indexing (which use .expect()) are already unwrapped
                    cwd_is_option = matches!(value, HirExpr::Var(v) if {
                        self.ctx.var_types.get(v).is_some_and(|t| matches!(t, Type::Optional(_)))
                    });
                }
                "check" => {
                    if let HirExpr::Literal(Literal::Bool(b)) = value {
                        _check = *b;
                    }
                }
                _ => {} // Ignore unknown kwargs for now
            }
        }

        // Build the Command construction
        // Python: subprocess.run(["echo", "hello"], capture_output=True, cwd="/tmp")
        // Rust: {
        //   let mut cmd = std::process::Command::new(&cmd_list[0]);
        //   cmd.args(&cmd_list[1..]);
        //   if cwd { cmd.current_dir(cwd); }
        //   let output = cmd.output()?;
        //   // Create result struct
        //   SubprocessResult {
        //     returncode: output.status.code().unwrap_or(-1),
        //     stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        //     stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        //   }
        // }

        // DEPYLER-0627: subprocess.run() returns CompletedProcess struct (not tuple)
        // Python's subprocess.run() returns CompletedProcess with .returncode, .stdout, .stderr
        // We generate a struct to match Python's API semantics.
        self.ctx.needs_completed_process = true;

        // DEPYLER-0517: Handle Option<String> for cwd parameter
        // DEPYLER-0950: Only use if-let Some() when cwd is actually an Option type
        // When cwd is a concrete expression (like list indexing), use it directly
        let result = if capture_output {
            // Use .output() to capture stdout/stderr
            if let Some(cwd) = cwd_expr {
                if cwd_is_option {
                    // cwd is Option<String> - need if-let to unwrap
                    parse_quote! {
                        {
                            let cmd_list = #cmd_expr;
                            let mut cmd = std::process::Command::new(&cmd_list[0]);
                            cmd.args(&cmd_list[1..]);
                            if let Some(dir) = #cwd {
                                cmd.current_dir(dir);
                            }
                            let output = cmd.output().expect("subprocess.run() failed");
                            CompletedProcess {
                                returncode: output.status.code().unwrap_or(-1),
                                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                            }
                        }
                    }
                } else {
                    // cwd is already a concrete path (String) - use directly
                    parse_quote! {
                        {
                            let cmd_list = #cmd_expr;
                            let mut cmd = std::process::Command::new(&cmd_list[0]);
                            cmd.args(&cmd_list[1..]);
                            cmd.current_dir(#cwd);
                            let output = cmd.output().expect("subprocess.run() failed");
                            CompletedProcess {
                                returncode: output.status.code().unwrap_or(-1),
                                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                            }
                        }
                    }
                }
            } else {
                parse_quote! {
                    {
                        let cmd_list = #cmd_expr;
                        let mut cmd = std::process::Command::new(&cmd_list[0]);
                        cmd.args(&cmd_list[1..]);
                        let output = cmd.output().expect("subprocess.run() failed");
                        CompletedProcess {
                            returncode: output.status.code().unwrap_or(-1),
                            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                        }
                    }
                }
            }
        } else {
            // Use .status() for exit code only (no capture)
            if let Some(cwd) = cwd_expr {
                if cwd_is_option {
                    // cwd is Option<String> - need if-let to unwrap
                    parse_quote! {
                        {
                            let cmd_list = #cmd_expr;
                            let mut cmd = std::process::Command::new(&cmd_list[0]);
                            cmd.args(&cmd_list[1..]);
                            if let Some(dir) = #cwd {
                                cmd.current_dir(dir);
                            }
                            let status = cmd.status().expect("subprocess.run() failed");
                            CompletedProcess {
                                returncode: status.code().unwrap_or(-1),
                                stdout: String::new(),
                                stderr: String::new(),
                            }
                        }
                    }
                } else {
                    // cwd is already a concrete path (String) - use directly
                    parse_quote! {
                        {
                            let cmd_list = #cmd_expr;
                            let mut cmd = std::process::Command::new(&cmd_list[0]);
                            cmd.args(&cmd_list[1..]);
                            cmd.current_dir(#cwd);
                            let status = cmd.status().expect("subprocess.run() failed");
                            CompletedProcess {
                                returncode: status.code().unwrap_or(-1),
                                stdout: String::new(),
                                stderr: String::new(),
                            }
                        }
                    }
                }
            } else {
                parse_quote! {
                    {
                        let cmd_list = #cmd_expr;
                        let mut cmd = std::process::Command::new(&cmd_list[0]);
                        cmd.args(&cmd_list[1..]);
                        let status = cmd.status().expect("subprocess.run() failed");
                        CompletedProcess {
                            returncode: status.code().unwrap_or(-1),
                            stdout: String::new(),
                            stderr: String::new(),
                        }
                    }
                }
            }
        };

        Ok(result)
    }

    /// Convert subprocess.Popen() to std::process::Command::spawn()
    /// DEPYLER-0931: Subprocess Popen for process management
    ///
    /// Maps Python subprocess.Popen() to Rust std::process::Command:
    /// - subprocess.Popen(cmd) → Command::new(cmd).spawn().expect("...")
    /// - subprocess.Popen(cmd, shell=True) → Command::new("sh").arg("-c").arg(cmd).spawn()
    ///
    /// Returns std::process::Child which has .wait(), .kill(), etc.
    ///
    /// # Complexity
    /// ≤10 (linear processing of kwargs)
    #[inline]
    pub(super) fn convert_subprocess_popen(
        &mut self,
        args: &[HirExpr],
        kwargs: &[(Symbol, HirExpr)],
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            bail!("subprocess.Popen() requires at least 1 argument (command)");
        }

        // First argument is the command
        let cmd_expr = args[0].to_rust_expr(self.ctx)?;

        // Parse keyword arguments
        let mut shell = false;
        let mut cwd_expr: Option<syn::Expr> = None;

        for (key, value) in kwargs {
            match key.as_str() {
                "shell" => {
                    if let HirExpr::Literal(Literal::Bool(b)) = value {
                        shell = *b;
                    }
                }
                "cwd" => {
                    cwd_expr = Some(value.to_rust_expr(self.ctx)?);
                }
                _ => {} // Ignore unknown kwargs for now
            }
        }

        // Build the Command construction
        // Python: subprocess.Popen(cmd, shell=True)
        // Rust: Command::new("sh").arg("-c").arg(cmd).spawn().expect("...")
        let result = if shell {
            // shell=True: run through shell
            if let Some(cwd) = cwd_expr {
                parse_quote! {
                    {
                        let mut popen_cmd = std::process::Command::new("sh");
                        popen_cmd.arg("-c").arg(#cmd_expr);
                        popen_cmd.current_dir(#cwd);
                        popen_cmd.spawn().expect("subprocess.Popen() failed")
                    }
                }
            } else {
                parse_quote! {
                    {
                        let mut popen_cmd = std::process::Command::new("sh");
                        popen_cmd.arg("-c").arg(#cmd_expr);
                        popen_cmd.spawn().expect("subprocess.Popen() failed")
                    }
                }
            }
        } else {
            // No shell: cmd is a list
            if let Some(cwd) = cwd_expr {
                parse_quote! {
                    {
                        let popen_list = #cmd_expr;
                        let mut popen_cmd = std::process::Command::new(&popen_list[0]);
                        popen_cmd.args(&popen_list[1..]);
                        popen_cmd.current_dir(#cwd);
                        popen_cmd.spawn().expect("subprocess.Popen() failed")
                    }
                }
            } else {
                parse_quote! {
                    {
                        let popen_list = #cmd_expr;
                        let mut popen_cmd = std::process::Command::new(&popen_list[0]);
                        popen_cmd.args(&popen_list[1..]);
                        popen_cmd.spawn().expect("subprocess.Popen() failed")
                    }
                }
            }
        };

        Ok(result)
    }
}
