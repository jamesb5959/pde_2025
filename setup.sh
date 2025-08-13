#!/bin/bash
set -e

echo "[*] Building Rust project with cargo..."
cargo build

echo "[*] Extracting pde_VirtualBox_src.tar.xz..."
tar -xvf pde_VirtualBox_src.tar.xz

echo "[*] Extracting VMs.tar.xz..."
tar -xvf VMs.tar.xz

echo "[*] Entering VirtualBox driver directory..."
cd VirtualBox-7.0.10/out/linux.amd64/debug/bin

echo "[*] Running vboxdrv.sh setup..."
sudo ./vboxdrv.sh setup

cd ../../../../../..
