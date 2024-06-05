# Easy Git

Gerenciador de usuários GIT

## Sumário

- [Easy Git](#easy-git)
  - [Sumário](#sumário)
  - [Introdução](#introdução)
  - [Pré-requisitos](#pré-requisitos)
  - [Instalação do Rust e Cargo](#instalação-do-rust-e-cargo)
    - [Linux e MacOS](#linux-e-macos)
    - [Windows](#windows)
  - [Compilação e Instalação do easy-git](#compilação-e-instalação-do-easy-git)
    - [Passo 1: Clonar o Repositório](#passo-1-clonar-o-repositório)
    - [Passo 2: Compilar a Aplicação](#passo-2-compilar-a-aplicação)
    - [Passo 3: Instalar a Aplicação](#passo-3-instalar-a-aplicação)
  - [Uso](#uso)
    - [Funcionalidades](#funcionalidades)
  - [Contribuição](#contribuição)
  - [Licença](#licença)

## Introdução

`easy-git` é uma ferramenta simples e interativa para gerenciar usuários Git em repositórios locais e globais. Ela permite configurar e remover usuários de forma fácil e rápida.

## Pré-requisitos

Antes de instalar o `easy-git`, você precisa ter o Rust e o Cargo instalados em seu sistema.

## Instalação do Rust e Cargo

### Linux e MacOS

1. Abra seu terminal.
2. Execute o seguinte comando para instalar o Rust e o Cargo:

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

3. Siga as instruções na tela para completar a instalação.
4. Após a instalação, adicione o Cargo ao seu PATH (caso não tenha sido feito automaticamente):

    ```sh
    source $HOME/.cargo/env
    ```

### Windows

1. Baixe e execute o instalador do Rust para Windows em [rust-lang.org](https://www.rust-lang.org/tools/install).
2. Siga as instruções na tela para completar a instalação.

Para verificar se a instalação foi bem-sucedida, execute:

```sh
rustc --version
cargo --version
```

Você deve ver a versão do Rust e do Cargo instalados.

## Compilação e Instalação do easy-git

### Passo 1: Clonar o Repositório

Clone o repositório easy-git do GitHub:

```sh
git clone https://github.com/seu-usuario/easy-git.git
cd easy-git
```

### Passo 2: Compilar a Aplicação

**Linux:**

```sh
cargo build --release
```

**MacOS:**

```sh
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```

**Windows:**

```sh
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

### Passo 3: Instalar a Aplicação

Após a compilação, o binário estará disponível na pasta `target/release/`. Para instalar o easy-git, mova o binário para um diretório que esteja no seu PATH.

**Linux e MacOS:**

```sh
sudo cp target/release/easy-git /usr/local/bin/
```

**Windows:**

```powershell
Move-Item -Path "target\x86_64-pc-windows-gnu\release\easy-git.exe" -Destination "$env:ProgramFiles\easy-git\easy-git.exe"
```

## Uso

Para usar o easy-git, execute o seguinte comando no terminal:

```sh
easy-git
```

### Funcionalidades

- Configurar Usuário Global: Permite configurar o nome e o email do usuário globalmente.
- Configurar Usuário Local: Permite configurar o nome e o email do usuário para o repositório atual.
- Apagar Configuração Global: Remove a configuração global do nome e do email do usuário.
- Apagar Configuração Local: Remove a configuração local do nome e do email do usuário.
- Criar Arquivo de Configuração: Cria um arquivo .easy-git-config para o repositório atual.
- Atualizar Arquivo de Configuração: Atualiza o arquivo .easy-git-config do repositório atual.
- Apagar Configuração do easy-git: Remove o arquivo .easy-git-config e o includeIf correspondente do .gitconfig global.

Siga as instruções no menu interativo para selecionar a ação desejada.

## Contribuição

Se você encontrar um bug ou tiver uma sugestão de melhoria, por favor, abra uma issue ou envie um pull request no repositório easy-git.

## Licença

Este projeto está licenciado sob a MIT License.
