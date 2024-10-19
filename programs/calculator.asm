INIT [0x00] = 0b00000001 ; Add 1 to the memory at address 0x00
INIT [0x01] = 0b00000001 ; Add 1 to the memory at address 0x01

LOAD R1, [0x00] ; Load value from RAM to register
LOAD R2, [0x00] ; Load value from RAM to register

OUT R1 ; Print Register 1
OUT R2 ; Print Register 2

ADD R1, R2, R3 ; Add register 1 and register 2 and save result to the register 3

STORE R3, [0x20] ; Save register 3 value to RAM

OUT R3 ; print reguster 3

; Cleaning memory and registers
CLEAR [0x00]
CLEAR [0x01]
CLEAR [0x20]
CLEAR R1
CLEAR R2
CLEAR R3

HALT ; Exit