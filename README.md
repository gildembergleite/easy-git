# Easy Git Config by RUST

Claro! Aqui está um exemplo de README.md detalhado, que mostra como instalar Rust e Cargo, além de instalar e usar o easy-git:

markdown
Copiar código
# easy-git

Gerenciador de usuários GIT

## Sumário

- [Easy Git Config by RUST](#easy-git-config-by-rust)
- [easy-git](#easy-git)
  - [Sumário](#sumário)
  - [Introdução](#introdução)
  - [Pré-requisitos](#pré-requisitos)
  - [Instalação do Rust e Cargo](#instalação-do-rust-e-cargo)
    - [Linux e MacOS](#linux-e-macos)
    - [Windows](#windows)
    - [Detalhamento dos Passos](#detalhamento-dos-passos)

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
Você deve ver a versão do Rust e do Cargo instalados.

Compilação e Instalação do easy-git
Passo 1: Clonar o Repositório
Clone o repositório easy-git do GitHub:

sh
Copiar código
git clone https://github.com/seu-usuario/easy-git.git
cd easy-git
Passo 2: Compilar a Aplicação
Linux
sh
Copiar código
cargo build --release
MacOS
sh
Copiar código
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
Windows
sh
Copiar código
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
Passo 3: Instalar a Aplicação
Após a compilação, o binário estará disponível na pasta target/release/. Para instalar o easy-git, mova o binário para um diretório que esteja no seu PATH.

Linux e MacOS
sh
Copiar código
sudo cp target/release/easy-git /usr/local/bin/
Windows
powershell
Copiar código
Move-Item -Path "target\x86_64-pc-windows-gnu\release\easy-git.exe" -Destination "$env:ProgramFiles\easy-git\easy-git.exe"
Uso
Para usar o easy-git, execute o seguinte comando no terminal:

sh
Copiar código
easy-git
Funcionalidades
Configurar Usuário Global: Permite configurar o nome e o email do usuário globalmente.
Configurar Usuário Local: Permite configurar o nome e o email do usuário para o repositório atual.
Apagar Configuração Global: Remove a configuração global do nome e do email do usuário.
Apagar Configuração Local: Remove a configuração local do nome e do email do usuário.
Criar Arquivo de Configuração: Cria um arquivo .easy-git-config para o repositório atual.
Atualizar Arquivo de Configuração: Atualiza o arquivo .easy-git-config do repositório atual.
Apagar Configuração do easy-git: Remove o arquivo .easy-git-config e o includeIf correspondente do .gitconfig global.
Siga as instruções no menu interativo para selecionar a ação desejada.

Contribuição
Se você encontrar um bug ou tiver uma sugestão de melhoria, por favor, abra uma issue ou envie um pull request no repositório easy-git.

Licença
Este projeto está licenciado sob a MIT License.

markdown
Copiar código

### Detalhamento dos Passos

- **Instalação do Rust e Cargo**: Incluí instruções específicas para Linux, MacOS e Windows.
- **Compilação e Instalação do easy-git**: Detalhei o processo de clonagem do repositório, compilação para diferentes sistemas operacionais e instalação do binário.
- **Uso**: Expliquei como usar o `easy-git` e listei as funcionalidades disponíveis.
- **Contribuição e Licença**: Adicionei seções para contribuição e detalhes de licença.

Esse `README.md` deve fornecer todas as informações necessárias para instalar, compilar e usar o `easy-git`, além de facilitar contribuições futuras.
