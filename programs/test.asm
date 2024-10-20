INIT [0x00] = 0b00000001      ; Initialize memory location 0x00 to 1
INIT [0x01] = 0b00000010      ; Initialize memory location 0x01 to 2
INIT [0x02] = 0b00000011      ; Initialize memory location 0x02 to 3
INIT [0x03] = 0b00000100      ; Initialize memory location 0x03 to 4

; Load values into registers
LOAD R0, [0x00]               ; Load value from memory address 0x00 into R0 (1)
LOAD R1, [0x01]               ; Load value from memory address 0x01 into R1 (2)
LOAD R2, [0x02]               ; Load value from memory address 0x02 into R2 (3)
LOAD R3, [0x03]               ; Load value from memory address 0x03 into R3 (4)

; Perform arithmetic operations
ADD R0, R1, R4                ; R4 = R0 + R1 (1 + 2 = 3)
SUB R3, R2, R5                ; R5 = R3 - R2 (4 - 3 = 1)
MUL R1, R2, R6                ; R6 = R1 * R2 (2 * 3 = 6)
DIV R3, R1, R7                ; R7 = R3 / R1 (4 / 2 = 2)

; Test AND operation
AND R0, R1, R8                ; R8 = R0 AND R1 (1 AND 2 = 0)
OR R0, R1, R9                 ; R9 = R0 OR R1 (1 OR 2 = 3)
XOR R0, R1, R10               ; R10 = R0 XOR R1 (1 XOR 2 = 3)

; Testing conditional with IF
IF R4 == 3 THEN MOV R8, 0b11111111   ; Set R8 to 0b00000000 if condition is false

; Test output instructions
OUT R4                        ; Output value of R4 (should be 3)
OUT R5                        ; Output value of R5 (should be 1)
OUT R6                        ; Output value of R6 (should be 6)
OUT R7                        ; Output value of R7 (should be 2)
OUT R8                        ; Output the result of IF condition check (should be 0b11111111)

HALT                          ; Stop execution
