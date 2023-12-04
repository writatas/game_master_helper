#![allow(dead_code)]
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use anyhow::{Ok, Error};

pub fn install_whisper_cpp_model(path_to_dir: &str) -> Result<(), Error> {
    let whisper_path = Path::new(path_to_dir);
    if !whisper_path.exists() {
        let whisper_url = "https://github.com/ggerganov/whisper.cpp.git";
        let _ = Command::new("git")
            .args(&["clone", whisper_url])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let whisper_dir = whisper_path.join("whisper.cpp");
        let mut make_command = Command::new("make");
        if std::env::consts::OS == "windows" {
            make_command.arg("Makefile.win");
        }
        let _ = make_command.current_dir(&whisper_dir).status();
        let _ = fs::create_dir_all(&whisper_path);
        let _ = fs::rename(whisper_dir.join("whisper"), whisper_path.join("whisper"));

        // write the next lines here
        if std::env::consts::OS == "linux" {
          let _ = Command::new("bash")
              .args(&["./models/download-ggml-model.sh", "base.en"])
              .current_dir(&whisper_path)
              .stdout(Stdio::null())
              .stderr(Stdio::null())
              .status();
            }
    }
    Ok(())
}