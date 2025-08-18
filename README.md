## Setup

1. Install dependencies:  
   - Rust and Cargo  
   - Make, GCC, and required build tools  

2. Run the setup script:  

   ```bash
   ./setup.sh
   ```

   This will:
   - Build the Rust Question code with Cargo  
   - Extract the VirtualBox debug source tree  
   - Extract the VM images  
   - Run `vboxdrv.sh setup` to prepare the VirtualBox kernel module  

---

## Usage

1. Start the environment and training app:  

   ```bash
   ./start.sh
   ```

   This will:
   - Start the VirtualBox driver (`vboxdrv`)  
   - Launch the Rust TUI trainer in a new terminal  
   - Boot the **pde VM** using the debug build of VirtualBox  

---

## Rust Question code

This presents a sequence of **questions** with hints, showing:  
- Starting VirtualBox in debug mode  
- Attaching GDB to `VirtualBoxVM`  
- Setting `solib-search-path`  
- Filtering breakpoints (VLAN ID >= 4096)  
- Inspecting locals, continuing execution  
- Loading/unloading kernel modules in the guest  

### Controls
- Type your answer and press **Enter**  
- Type `hint` to see a clue  
- Press **Esc** then `q` to quit
