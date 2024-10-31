# Depdem
Managment service demon

## Compilar
TARGET_CC=x86_64-unknown-linux-gnu cargo build --release --target x86_64-unknown-linux-gnu

## Generar paquete
dpkg-deb --build depdem

## Instalar
sudo dpkg -i depdem.deb

## Desinstalar
sudo service depdem stop
sudo systemctl disable depdem
sudo dpkg -r depdem
