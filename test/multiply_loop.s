addi x5, x0, 5
addi x6, x0, 4
addi x7, x0, 0
addi x28, x0, 0
mul_loop:
add x7, x7, x5
addi x28, x28, 1
blt x28, x6, mul_loop
