mod config;
mod utils;

use dialoguer::{theme::ColorfulTheme, Select};
use git2::Repository;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::env;
use crate::config::{
    configure_global_user, configure_local_user, create_config_file,
    update_config_file, unset_global_user, unset_local_user, delete_easy_git_config,
};
use crate::utils::{install_git, is_git_installed, get_confirmation};

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    writeln!(&mut stdout, "Gerenciador de usuários GIT\n").unwrap();

    if !is_git_installed() {
        prompt_git_installation();
    }

    match Repository::open(".") {
        Ok(_) => {
            show_config_options();
        }
        Err(_) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
            writeln!(&mut stdout, "Este diretório não possui um repositório Git.\n").unwrap();

            match env::current_dir() {
                Ok(path) => {
                    let path_str = path.to_str().unwrap_or("Caminho inválido");
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).unwrap();
                    writeln!(&mut stdout, "Diretório atual: {}\n", path_str).unwrap();
                }
                Err(e) => {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                    writeln!(&mut stdout, "Erro ao obter o diretório atual: {}\n", e).unwrap();
                }
            }

            let options = &["Iniciar um repositório Git", "Sair do programa"];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Escolha uma opção")
                .default(0)
                .items(&options[..])
                .interact()
                .unwrap();

            match selection {
                0 => {
                    match Repository::init(".") {
                        Ok(_) => {
                            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
                            writeln!(&mut stdout, "Repositório Git inicializado com sucesso.\n").unwrap();
                            show_config_options();
                        }
                        Err(e) => {
                            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                            writeln!(&mut stdout, "Falha ao inicializar o repositório Git: {}\n", e).unwrap();
                        }
                    }
                }
                1 => {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
                    writeln!(&mut stdout, "Saindo do programa.\n").unwrap();

                    std::process::exit(0);
                }
                _ => unreachable!(),
            }
        }
    }

    stdout.reset().unwrap();
}

fn prompt_git_installation() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
    writeln!(&mut stdout, "Git não está instalado.\n").unwrap();

    let options = &["Instalar o git nesta máquina", "Sair da aplicação"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Escolha uma opção")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => {
            if install_git() {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
                writeln!(&mut stdout, "Git instalado com sucesso. Reinicie a aplicação.\n").unwrap();
                std::process::exit(0);
            } else {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                writeln!(&mut stdout, "Falha ao instalar o Git. Por favor, instale o Git manualmente.\n").unwrap();
                std::process::exit(0);
            }
        }
        1 => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
            writeln!(&mut stdout, "Saindo da aplicação.\n").unwrap();
            std::process::exit(0);
        }
        _ => unreachable!(),
    }
}

fn show_config_options() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let current_dir = env::current_dir().expect("Não foi possível carregar o diretório atual");
    let switch_gc_path = current_dir.join(".easy-git-config");
    let options: Vec<&str>;
    let selection;

    if switch_gc_path.exists() {
        options = vec![
            "Atualizar configuração do easy-git",
            "Apagar configuração do easy-git",
            "Configurar usuário global",
            "Configurar usuário local",
            "Apagar configuração global",
            "Apagar configuração local",
            "Sair do programa",
        ];
        selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Escolha uma opção")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        match selection {
            0 => update_config_file(),
            1 => confirm_and_execute("Apagar configuração do easy-git", delete_easy_git_config),
            2 => configure_global_user(),
            3 => configure_local_user(),
            4 => confirm_and_execute("Apagar configuração global", unset_global_user),
            5 => confirm_and_execute("Apagar configuração local", unset_local_user),
            6 => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
                writeln!(&mut stdout, "Saindo do programa.\n").unwrap();
                std::process::exit(0);
            }
            _ => unreachable!(),
        }
    } else {
        options = vec![
            "Criar arquivo de configuração para o easy-git",
            "Configurar usuário global",
            "Configurar usuário local",
            "Apagar configuração global",
            "Apagar configuração local",
            "Sair do programa",
        ];
        selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Escolha uma opção")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        match selection {
            0 => create_config_file(),
            1 => configure_global_user(),
            2 => configure_local_user(),
            3 => confirm_and_execute("Apagar configuração global", unset_global_user),
            4 => confirm_and_execute("Apagar configuração local", unset_local_user),
            5 => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
                writeln!(&mut stdout, "Saindo do programa.\n").unwrap();
                std::process::exit(0);
            }
            _ => unreachable!(),
        }
    }
}

fn confirm_and_execute<F>(prompt: &str, func: F)
where
    F: Fn(),
{
    let confirmation = get_confirmation(&format!("Tem certeza que deseja {}? (s/N)", prompt));
    
    if confirmation.to_lowercase() == "s" {
        func();
    } else {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
        writeln!(&mut stdout, "Ação cancelada.\n").unwrap();
    }
}
