addi x5, x0, 0
addi x6, x0, 1
addi x7, x0, 10
addi x28, x0, 0
fib_loop:
add x29, x5, x6
add x5, x0, x6
add x6, x0, x29
addi x28, x28, 1
blt x28, x7, fib_loop
