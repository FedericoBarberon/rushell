use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::mpsc,
};

use crate::execution::{Executable, ExecutionResult};

enum Chunk {
    Stdout(Vec<u8>),
    Stderr(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub struct ExternalCmd {
    cmd: String,
    path: PathBuf,
    args: Vec<String>,
}

impl ExternalCmd {
    pub fn new(cmd: String, path: PathBuf, args: Vec<String>) -> Self {
        Self { cmd, path, args }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Executable for ExternalCmd {
    fn execute(
        &self,
        _input: &mut impl Read,
        output: &mut impl Write,
        error: &mut impl Write,
    ) -> ExecutionResult {
        let mut child = match Command::new(&self.cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(&self.args)
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                let _ = writeln!(error, "Failed to execute {} command: {e}", &self.cmd);
                return ExecutionResult::Continue;
            }
        };

        let mut stdout = child.stdout.take().unwrap();
        let mut stderr = child.stderr.take().unwrap();

        const CHANNEL_BOUND: usize = 32; // ~256KB pendientes como mucho, con buffers de 8KB
        let (tx, rx) = mpsc::sync_channel::<Chunk>(CHANNEL_BOUND);
        let tx_err = tx.clone();

        let stdout_thread = std::thread::spawn(move || {
            let mut buf = [0u8; 8 * 1024]; // Buffers de 8kb
            loop {
                match stdout.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        if tx.send(Chunk::Stdout(buf[..n].to_vec())).is_err() {
                            break;
                        }
                    }
                }
            }
        });

        let stderr_thread = std::thread::spawn(move || {
            let mut buf = [0u8; 8 * 1024]; // Buffers de 8kb
            loop {
                match stderr.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        if tx_err.send(Chunk::Stderr(buf[..n].to_vec())).is_err() {
                            break;
                        }
                    }
                }
            }
        });

        for chunk in rx {
            match chunk {
                Chunk::Stdout(bytes) => {
                    let _ = output.write_all(&bytes);
                }
                Chunk::Stderr(bytes) => {
                    let _ = error.write_all(&bytes);
                }
            }
        }

        let _ = stdout_thread.join();
        let _ = stderr_thread.join();
        let _ = child.wait();

        ExecutionResult::Continue
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        execution::ExecutionResult,
        tests::utilities::{TestBuffers, build_args},
    };

    use super::*;

    #[test]
    fn execute_external_cmd() {
        let cmd = "echo";

        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);
        let exit = ExternalCmd::new(cmd.into(), PathBuf::new(), build_args(&["foo"]));

        assert_eq!(
            exit.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        assert_eq!(String::from_utf8(output).unwrap(), "foo\n");
        assert!(error.is_empty());
        assert!(input.get_ref().is_empty());
    }
}
