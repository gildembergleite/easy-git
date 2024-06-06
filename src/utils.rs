use dialoguer::Input;
use std::process::Command;

pub fn get_user_input() -> (String, String) {
    let name: String = Input::new()
        .with_prompt("Digite o nome do usuário")
        .interact_text()
        .unwrap();

    let email: String = Input::new()
        .with_prompt("Digite o email do usuário")
        .interact_text()
        .unwrap();

    (name, email)
}

pub fn get_confirmation(prompt: &str) -> String {
    let confirmation: String = Input::new()
        .with_prompt(prompt)
        .default("N".to_string())
        .interact_text()
        .unwrap();

    confirmation
}

pub fn is_git_installed() -> bool {
    match Command::new("git").arg("--version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub fn install_git() -> bool {
    let os = std::env::consts::OS;

    let output = match os {
        "windows" => Command::new("powershell")
            .args(&["-Command", "winget install --id Git.Git -e --source winget"])
            .output(),
        "macos" => Command::new("sh")
            .arg("-c")
            .arg("brew install git")
            .output(),
        "linux" => {
            let distro_output = Command::new("sh")
                .arg("-c")
                .arg("cat /etc/*-release")
                .output()
                .expect("Falha ao detectar a distribuição Linux");

            let distro_info = String::from_utf8_lossy(&distro_output.stdout);

            if distro_info.contains("Debian") || distro_info.contains("Ubuntu") {
                Command::new("sh")
                    .arg("-c")
                    .arg("sudo apt-get update && sudo apt-get install -y git")
                    .output()
            } else if distro_info.contains("Fedora") || distro_info.contains("CentOS") {
                Command::new("sh")
                    .arg("-c")
                    .arg("sudo dnf install -y git")
                    .output()
            } else {
                return false;
            }
        }
        _ => return false,
    };

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
