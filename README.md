# Depdem
Managment service demon

## Requerimientos para compilar en Mac
rustup target add x86_64-unknown-linux-musl

brew install FiloSottile/musl-cross/musl-cross

En el archivo .cargo/config.toml agregar:
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"

## Compilar
TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl

## Generar paquete
dpkg-deb --build depdem

## Instalar
sudo dpkg -i depdem.deb

## Desinstalar
sudo service depdem stop
sudo systemctl disable depdem
sudo dpkg -r depdem
