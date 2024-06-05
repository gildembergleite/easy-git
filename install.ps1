if ($PSVersionTable.PSVersion.Major -ge 6) {
    $OS = "windows"
} else {
    Write-Host "Sistema operacional não suportado"
    exit 1
}

Invoke-WebRequest -Uri "https://github.com/seu-usuario/nome-do-repositorio/releases/download/v0.1.0/easy-git-windows.exe" -OutFile "easy-git.exe"
Move-Item -Path "easy-git.exe" -Destination "$env:ProgramFiles\easy-git\easy-git.exe"

Write-Host "easy-git instalado com sucesso!"
