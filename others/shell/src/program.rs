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

    pub fn run_with_stdin(&self) -> std::io::Result<Output> {
        let prog = Command::new(self.shell)
            .args(&self.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        prog.wait_with_output()
    }

    pub fn run_with_input(&self, input: &[u8]) -> std::io::Result<Output> {
        let mut prog = Command::new(self.shell)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
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
        if cfg!(target_os = "windows") {
            ["/C", prog_name]
                .iter()
                .chain(args.iter())
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        } else {
            let arg = [prog_name]
                .iter()
                .chain(args.iter())
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            vec!["-c".to_string(), arg]
        }
    }
}
