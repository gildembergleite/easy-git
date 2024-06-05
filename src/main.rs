mod config;
mod utils;

use dialoguer::{theme::ColorfulTheme, Select};
use git2::Repository;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::env;
use crate::config::{configure_global_user, configure_local_user, create_config_file, update_config_file, unset_global_user, unset_local_user, delete_easy_git_config};

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    writeln!(&mut stdout, "Gerenciador de usuários GIT\n").unwrap();

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

fn show_config_options() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let current_dir = env::current_dir().expect("Não foi possível carregar o diretório atual");
    let switch_gc_path = current_dir.join(".easy-git-config");
    let option_text = if switch_gc_path.exists() {
        "Atualizar configuração do easy-git"
    } else {
        "Criar arquivo de configuração para o easy-git"
    };

    let options = if switch_gc_path.exists() {
        vec![
            "Atualizar configuração do easy-git",
            "Apagar configuração do easy-git",
            "Configurar usuário global",
            "Configurar usuário local",
            "Apagar configuração global",
            "Apagar configuração local",
            "Sair do programa",
        ]
    } else {
        vec![
            option_text,
            "Configurar usuário global",
            "Configurar usuário local",
            "Apagar configuração global",
            "Apagar configuração local",
            "Sair do programa",
        ]
    };

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Escolha uma opção")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => {
            if option_text == "Atualizar configuração do repositório atual" {
                update_config_file();
            } else {
                create_config_file();
            }
        },
        1 => {
            if switch_gc_path.exists() {
                delete_easy_git_config();
            } else {
                configure_global_user();
            }
        },
        2 => configure_local_user(),
        3 => unset_global_user(),
        4 => unset_local_user(),
        5 => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
            writeln!(&mut stdout, "Saindo do programa.\n").unwrap();

            std::process::exit(0);
        }
        _ => unreachable!(),
    }
}