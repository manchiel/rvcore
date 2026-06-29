addi x5, x0, 42
addi x6, x0, 29
jal x1, suma
addi x28, x0, 100
beq x0, x0, kraj
suma:
add x7, x5, x6
jalr x0, 0(x1)
kraj:
addi x29, x0, 7
