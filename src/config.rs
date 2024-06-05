use std::fs::{File, OpenOptions, remove_file};
use std::io::{self, Write, Read};
use git2::{Config, Repository};
use crate::utils::get_user_input;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use dirs::home_dir;
use std::path::PathBuf;
use std::env::current_dir;

pub fn create_config_file() {
    let (name, email) = get_user_input();
    let config_content = format!(
        "[user]\n\tname = {}\n\temail = {}\n",
        name, email
    );
    
    let current_dir = get_current_dir();
    let switch_gc_path = current_dir.join(".easy-git-config");

    let file_existed = switch_gc_path.exists();

    let mut file = File::create(&switch_gc_path).expect("Falha ao criar o arquivo .easy-git-config");
    file.write_all(config_content.as_bytes()).expect("Falha ao escrever no arquivo .easy-git-config");

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    writeln!(&mut stdout, "Arquivo .easy-git-config criado com sucesso.\n").unwrap();

    if !file_existed {
        add_include_if_to_global_gitconfig().unwrap();
    }
}

pub fn update_config_file() {
    let (name, email) = get_user_input();
    let config_content = format!(
        "[user]\n\tname = {}\n\temail = {}\n",
        name, email
    );
    
    let current_dir = get_current_dir();
    let switch_gc_path = current_dir.join(".easy-git-config");

    let mut file = File::create(&switch_gc_path).expect("Falha ao criar o arquivo .easy-git-config");
    file.write_all(config_content.as_bytes()).expect("Falha ao escrever no arquivo .easy-git-config");

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    writeln!(&mut stdout, "Arquivo .easy-git-config atualizado com sucesso.\n").unwrap();
}

fn get_home_dir() -> PathBuf {
    home_dir().expect("Não foi possível encontrar o diretório home")
}

fn get_current_dir() -> PathBuf {
    current_dir().expect("Não foi possível carregar o diretório atual")
}

fn read_global_gitconfig() -> io::Result<String> {
    let home_dir = get_home_dir();
    let global_gitconfig_path = home_dir.join(".gitconfig");
    let mut file = File::open(global_gitconfig_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn remove_existing_include_if(content: &str, current_dir: &str) -> String {
    let mut new_content = Vec::new();
    let mut skip = false;

    for line in content.lines() {
        if line.contains(&format!("[includeIf \"gitdir:{}/\"]", current_dir)) {
            skip = true;
        }

        if skip && line.trim().starts_with("path =") {
            skip = false;
            continue;
        }

        if !skip {
            new_content.push(line);
        }
    }

    new_content.join("\n")
}

fn remove_include_if_from_global_gitconfig() -> io::Result<()> {
    let home_dir = get_home_dir();
    let current_dir = get_current_dir();
    let current_dir_str = current_dir.display().to_string();
    let global_gitconfig_path = home_dir.join(".gitconfig");

    let content = read_global_gitconfig()?;

    let updated_content = remove_existing_include_if(&content, &current_dir_str);

    let mut file = OpenOptions::new().write(true).truncate(true).open(global_gitconfig_path)?;
    file.write_all(updated_content.as_bytes())?;
    Ok(())
}

fn add_include_if_to_global_gitconfig() -> io::Result<()> {
    let home_dir = get_home_dir();
    let current_dir = get_current_dir();
    let current_dir_str = current_dir.display().to_string();
    let global_gitconfig_path = home_dir.join(".gitconfig");

    let content = read_global_gitconfig()?;

    let updated_content = remove_existing_include_if(&content, &current_dir_str);

    let config_content = format!(
        "{}\n[includeIf \"gitdir:{}/\"]\n    path = {}/.easy-git-config\n",
        updated_content, current_dir_str, current_dir_str
    );

    let mut file = OpenOptions::new().write(true).truncate(true).open(global_gitconfig_path)?;
    file.write_all(config_content.as_bytes())?;
    Ok(())
}

pub fn delete_easy_git_config() {
    let current_dir = get_current_dir();
    let switch_gc_path = current_dir.join(".easy-git-config");

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    if switch_gc_path.exists() {
        match remove_file(&switch_gc_path) {
            Ok(_) => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
                writeln!(&mut stdout, "Arquivo .easy-git-config removido com sucesso.\n").unwrap();
                if let Err(e) = remove_include_if_from_global_gitconfig() {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                    writeln!(&mut stdout, "Erro ao remover o includeIf do .gitconfig global: {}\n", e).unwrap();
                } else {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
                    writeln!(&mut stdout, "includeIf removido do .gitconfig global com sucesso.\n").unwrap();
                }
            }
            Err(e) => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                writeln!(&mut stdout, "Erro ao remover o arquivo .easy-git-config: {}\n", e).unwrap();
            }
        }
    } else {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
        writeln!(&mut stdout, "O arquivo .easy-git-config não existe.\n").unwrap();
    }
}

pub fn configure_global_user() {
    let (name, email) = get_user_input();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();

    match Config::open_default() {
        Ok(mut config) => {
            if let Err(e) = config.set_str("user.name", &name) {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                writeln!(&mut stdout, "Erro ao configurar o nome do usuário global: {}", e).unwrap();
            } else {
                writeln!(&mut stdout, "Nome do usuário global configurado como: {}", name).unwrap();
            }

            if let Err(e) = config.set_str("user.email", &email) {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                writeln!(&mut stdout, "Erro ao configurar o email do usuário global: {}", e).unwrap();
            } else {
                writeln!(&mut stdout, "Email do usuário global configurado como: {}", email).unwrap();
            }
        }
        Err(e) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
            writeln!(&mut stdout, "Erro ao abrir a configuração global do Git: {}", e).unwrap();
        }
    }
}

pub fn configure_local_user() {
    let (name, email) = get_user_input();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();

    let current_dir = get_current_dir();
    match Repository::open(current_dir) {
        Ok(repo) => {
            match repo.config() {
                Ok(mut repo_config) => {
                    if let Err(e) = repo_config.set_str("user.name", &name) {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                        writeln!(&mut stdout, "Erro ao configurar o nome do usuário local: {}", e).unwrap();
                    } else {
                        writeln!(&mut stdout, "Nome do usuário local configurado como: {}", name).unwrap();
                    }

                    if let Err(e) = repo_config.set_str("user.email", &email) {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                        writeln!(&mut stdout, "Erro ao configurar o email do usuário local: {}", e).unwrap();
                    } else {
                        writeln!(&mut stdout, "Email do usuário local configurado como: {}", email).unwrap();
                    }
                }
                Err(e) => {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                    writeln!(&mut stdout, "Erro ao abrir a configuração do repositório Git: {}", e).unwrap();
                }
            }
        }
        Err(e) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
            writeln!(&mut stdout, "Erro ao abrir o repositório Git: {}", e).unwrap();
        }
    }
}

pub fn unset_global_user() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    match Config::open_default() {
        Ok(mut config) => {
            if let Err(e) = config.remove("user.name") {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                writeln!(&mut stdout, "Erro ao remover o nome do usuário global: {}", e).unwrap();
            } else {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
                writeln!(&mut stdout, "Nome do usuário global removido com sucesso.").unwrap();
            }

            if let Err(e) = config.remove("user.email") {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                writeln!(&mut stdout, "Erro ao remover o email do usuário global: {}", e).unwrap();
            } else {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
                writeln!(&mut stdout, "Email do usuário global removido com sucesso.").unwrap();
            }
        }
        Err(e) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
            writeln!(&mut stdout, "Erro ao abrir a configuração global do Git: {}", e).unwrap();
        }
    }
}

pub fn unset_local_user() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let current_dir = get_current_dir();
    match Repository::open(current_dir) {
        Ok(repo) => {
            match repo.config() {
                Ok(mut repo_config) => {
                    if let Err(e) = repo_config.remove("user.name") {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                        writeln!(&mut stdout, "Erro ao remover o nome do usuário local: {}", e).unwrap();
                    } else {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
                        writeln!(&mut stdout, "Nome do usuário local removido com sucesso.").unwrap();
                    }

                    if let Err(e) = repo_config.remove("user.email") {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                        writeln!(&mut stdout, "Erro ao remover o email do usuário local: {}", e).unwrap();
                    } else {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
                        writeln!(&mut stdout, "Email do usuário local removido com sucesso.").unwrap();
                    }
                }
                Err(e) => {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                    writeln!(&mut stdout, "Erro ao abrir a configuração do repositório Git: {}", e).unwrap();
                }
            }
        }
        Err(e) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
            writeln!(&mut stdout, "Erro ao abrir o repositório Git: {}", e).unwrap();
        }
    }
}
