; Example of using MOV instruction for:
; 1. Placing Immediate Value to the register
; 2. Placing Another register's value to the register
; 3. Placing RAM ADDRESS Value to the register (works same as LOAD instrution)
; Doing the same but for RAM Address and not register also possible.

MOV R1, 0b00000011              ; MOVE Immediate value to the Register
OUT R1                          ; Print Register 1

MOV [0x01], 0b00000010          ; Load Emmediate value to the RAM at Address 0x01
MOV [0x02], 0b11111111          ; Load Emmediate value to the RAM at Address 0x02

MOV R2, R1                      ; MOVE Copy Register 1 value to the Register 2
IF R2 == R1 THEN OUT 0b01010101 ; Quick IF statement to check if R2 actually same as R1, if yes - print 0b01010101
OUT R2                          ; Print Register 2

MOV R3, [0x01]                  ; MOVE Copy Value located at RAM address to the Register 3
OUT R3                          ; Print Register 3

CLEAR R1                        ; Clear Register 1
CLEAR R2                        ; Clear Register 2
CLEAR R3                        ; Clear Register 3
CLEAR [0x01]                    ; Clear Memory at address 0x01
CLEAR [0x02]                    ; Clear Memory at address 0x02

HALT                            ; Stop The Program