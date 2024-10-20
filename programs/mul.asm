; programs/mul.asm
; Example program that demonstrates the MUL instruction

INIT [0x00] = 0b00001010  ; Initialize memory at address 0x00 with binary 10
INIT [0x01] = 0b00000110  ; Initialize memory at address 0x01 with binary 6

LOAD R0, [0x00]            ; Load value from address 0x00 into register R0
LOAD R1, [0x01]            ; Load value from address 0x01 into register R1

MUL R0, R1, R2             ; Multiply R0 and R1, store result in R2

STORE R2, [0x20]           ; Store the result in memory address 0x20

OUT R2                     ; Display the register R2

CLEAR [0x20]               ; Clear the value in memory address 0x20
CLEAR [0x00]               ; Clear the value in memory address 0x00
CLEAR [0x01]               ; Clear the value in memory address 0x01

HALT                       ; Stop execution