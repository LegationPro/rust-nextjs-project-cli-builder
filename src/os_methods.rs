use std::process::{Command, Output};
use std::path::PathBuf;
use colored::Colorize;
use execute::Execute;
use std::fs;

pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    return std::env::current_dir();
}

#[cfg(target_os = "macos")]
pub fn native_cmd_executer() {
    let shell = Command::new("sh");
    shell
}

#[cfg(target_os = "linux")]
pub fn native_cmd_executor() -> Command {
    let shell = Command::new("sh");
    shell
}

#[cfg(target_os = "windows")]
pub fn native_cmd_executor() -> Command {
    let mut cmd = Command::new("cmd");
    cmd
}

pub fn output_status(output: &Output) -> bool {
    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            return true;
        } else {
            return false;
        }
    }

    return false;
}

pub fn open_with_vs_code() -> bool {
    let output = native_cmd_executor()
        .arg("-c")
        .arg("code .")
        .execute_output()
        .unwrap();
    return output_status(&output)
}

pub fn has_yarn() -> bool {
    let output = native_cmd_executor()
        .arg("-c")
        .arg("yarn --version")
        .execute_output()
        .unwrap();

    return output_status(&output);
}

pub fn handle_yarn_installation() {
    if !has_yarn() {
        println!("{} {}", "Yarn package manager could not be found.\n", "Installing yarn...\n");
        let installed = install_yarn();

        if !installed {
            return println!("{}", "There was a problem installing yarn..".red());
        } else {
            println!("{}", "Installed yarn correctly!".green());
        }
    }
}

pub fn check_if_dir_exists(path_buf: PathBuf, dir_name: &str) -> bool {
    let paths = fs::read_dir(path_buf.as_path()).unwrap();

    for path in paths {
        let path = path.unwrap();
        let dir_name = path.file_name();
        let dir_path = path.metadata().unwrap();

        if dir_path.is_dir() && dir_name.to_str().unwrap() == dir_name {
            println!("Directory: {}", dir_name.to_string_lossy().to_string());
            return true
        }
    }

    return false
}


pub fn install_yarn() -> bool {
    let output = native_cmd_executor()
        .arg("-c")
        .arg("npm install --global yarn")
        .execute_output()
        .unwrap();

    return output_status(&output);
}

pub fn has_nodejs() -> bool {
    let output = native_cmd_executor()
        .arg("-c")
        .arg("node -v")
        .execute_output()
        .unwrap();
    
    return output_status(&output);
}