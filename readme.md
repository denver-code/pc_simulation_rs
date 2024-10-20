# PC Simulation  
Basic Rust app that will simulate some of the flows on computer, including: 
- RAM  (`256 bytes`) - can be read, written and dumped for inspection.    
  
  Addressed via hexadecimal addressed. 

  Each memory ceal is 8 bits (1 byte) wide
- CPU  - Supports basic arithmetic and logic operations (`ADD, AND, OR, NAND, NOR, XOR, NOT`).  
  
  ATM asm-like code directly executed by the CPU, I'm not sure how it supposed to be.   

  CPU supports `8 general-purpose 8-bit` registers to hold data during execution.  
- More on the way, but it's still area for a research.  

Simulation has option to write simple assembly-like programs with small set of instructions:  
- `VER` - Toggle verbosity to enable/disable detailed execution logs.
- `MOV` - Assingning RAM/Another Register's/Immediate Value to the register
- `INIT` - Initialize memory addresses with values.
- `LOAD` -  Load a value from memory into a register.
- `ADD` - Add values in two registers and store the result into the third register
- `STORE` - Store a value from a register into memory.
- `OUT` - Output register or memory/values.
- `CLEAR` - Clear the register or memory
- `HALT` - Stop the execution.
- `IF/ELSE` -  If and else statement that supports basic operations between registers, memory and values.

While I'm aiming to make it as low-level and realistic as possible - some of the features jsut could't be realisied due to number of reasons, one of them - I'm still researching about flows and how everything is working.  

### Run asm-like code  
```bash
BIOS> filename.asm
```  
You can play around with some example programs:  
- `calculator.asm` - Addition calculator program
- `gates.asm`  - Test of the Logic Gates
- `if_test.asm` - Simple IF ELSE logic
- `program.asm` - Simple collection of different instructions
- `mov_test.asm` - Test of MOV instruction
- `div.asm` - Division Program
- `incdec.asm` - Increment and Decrement Register
- `mul.asm` - Multiplication Program
- `qmov.asm` - QMOV Test
- `sub.asm` - Substract Instruction

### Program example  
```assembly
; Example program
VER = 1                    ; Enable debug (VERBOSE) prints of execution
INIT [0x08] = 0b00000101   ; Initialize memory at address 0x08 with binary 5
INIT [0x10] = 0b00001010   ; Initialize memory at address 0x10 with binary 10
INIT [0x1F] = 0b00001010

LOAD R1, [0x08]            ; Load value from address 0x08 into register R1
LOAD R2, [0x10]            ; Load value from address 0x10 into register R2
ADD R1, R2, R3             ; Add R1 and R2, store result in R3

STORE R3, [0x20]           ; Store the result in memory address 0x20

OUT R3                     ; Display the register R3

CLEAR [0x20]               ; Clear the value in memory address 0x20
CLEAR [0x08]               ; Clear the value in memory address 0x08
CLEAR [0x10]               ; Clear the value in memory address 0x10
CLEAR [0x1F]               ; Clear the value in memory address 0x1F

VER = 0                    ; Disable debug (VERBOSE) prints of execution
HALT                       ; Stop execution
```

## Getting Started  
1. Make sure you have rust installed  
2. Clone the repo:  
   ```bash
    git clone https://github.com/denver-code/pc_simulation_rs
    cd pc_simulation_rs
   ```
3. Run or Build the project:  
   ```bash
    # Build
    cargo build --release (Optional)
    # Run
    cargo run
   ```

### Usage
#### BIOS Commands

Once the simulator is powered on, you can use the following commands in the BIOS prompt:
```
- address [hex]: View the value stored in memory at the given address (in hexadecimal in square brackets).

- memory_dump: Display the entire contents of the RAM.
    
- exit: Quit the BIOS.

- [filename].asm: Load and run an assembly-like program from a file.
```