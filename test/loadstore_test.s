addi x5, x0, 42
addi x6, x0, 29
lui x7, 0x80000
sd x5, 512(x7)
sd x6, 520(x7)
ld x28, 512(x7)
ld x29, 520(x7)
add x30, x28, x29
