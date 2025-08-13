#!/bin/bash
set -e

echo "[*] Entering VirtualBox driver directory..."
cd VirtualBox-7.0.10/out/linux.amd64/debug/bin

echo "[*] Running vboxdrv.sh start..."
sudo ./vboxdrv.sh start

cd ../../../../../..

echo "[*] Launching Rust TUI app in new terminal..."
gnome-terminal -- bash -c "./target/debug/pde; exec bash"

echo "[*] Launching 'pde' VM..."
./VirtualBox-7.0.10/out/linux.amd64/debug/bin/VirtualBoxVM --startvm "pde"
