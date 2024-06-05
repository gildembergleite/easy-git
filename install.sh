#!/bin/bash

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
else
    echo "Sistema operacional não suportado"
    exit 1
fi

curl -L -o easy-git https://github.com/seu-usuario/nome-do-repositorio/releases/download/v0.1.0/easy-git-$OS
chmod +x easy-git
sudo mv easy-git /usr/local/bin/

echo "easy-git instalado com sucesso!"
