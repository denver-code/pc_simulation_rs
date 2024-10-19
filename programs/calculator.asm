INIT [0x00] = 0b00000001 ; Add 1 to the memory
INIT [0x01] = 0b00000001 ; b

LOAD R1, [0x00]
LOAD R2, [0x00]

OUT R1
OUT R2

ADD R1, R2, R3 ; c = a + b

STORE R3, [0x20] ; Save c to RAM

OUT R3 ; print c

; Cleaning memory and registers
CLEAR [0x00]
CLEAR [0x01]
CLEAR [0x20]
CLEAR R1
CLEAR R2
CLEAR R3

HALT