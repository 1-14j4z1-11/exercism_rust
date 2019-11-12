use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::iter::Iterator;
use std::process::{Command, Output, Stdio};

#[derive(Debug)]
pub struct Program {
    shell: &'static str,
    args: Vec<String>,
}

impl Program {
    pub fn new(name: &str, args: &[&str]) -> Self {
        Program {
            shell: Self::shell_name(),
            args: Self::shell_args_from(name, args),
        }
    }

    pub fn run(&self, input: &[u8]) -> std::io::Result<Output> {
        let envs = env::vars().collect::<HashMap<String, String>>();
        let mut prog = Command::new(self.shell)
            .args(&self.args)
            .current_dir(".")
            .envs(&envs)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = prog.stdin.as_mut().unwrap();
        stdin.write_all(input)?;

        prog.wait_with_output()
    }

    fn shell_name() -> &'static str {
        if cfg!(target_os = "windows") {
            "cmd"
        } else {
            "sh"
        }
    }

    fn shell_args_from(prog_name: &str, args: &[&str]) -> Vec<String> {
        let first_args = if cfg!(target_os = "windows") {
            ["/C", prog_name]
        } else {
            ["-c", prog_name]
        };

        first_args
            .iter()
            .chain(args.iter())
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }
}
