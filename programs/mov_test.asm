; Example of using MOV instruction for:
; 1. Placing Immediate Value to the register
; 2. Placing Another register's value to the register
; 3. Placing RAM ADDRESS Value to the register (works same as LOAD instrution)

MOV R1, 0b00000011              ; MOVE Immediate value to the Register
OUT R1                          ; Print Register 1

MOV R2, R1                      ; MOVE Copy Register 1 value to the Register 2
IF R2 == R1 THEN OUT 0b01010101 ; Quick IF statement to check if R2 actually same as R1, if yes - print 0b01010101
OUT R2                          ; Print Register 2

INIT [0x01] = 0b00000010        ; Assign number 2(10)/0b00000010(2) to the 0x01 RAM Address
MOV R3, [0x01]                  ; MOVE Copy Value located at RAM address to the Register 3
OUT R3                          ; Print Register 3

CLEAR R1                        ; Clear Register 1
CLEAR R2                        ; Clear Register 2
CLEAR R3                        ; Clear Register 3

HALT                            ; Stop The Program