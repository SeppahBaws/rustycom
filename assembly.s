 .globl main
main:
 movl $1, %eax
 not %eax
 not %eax
 neg %eax
 neg %eax
 cmpl $0, %eax
 movl $0, %eax
 sete %al
 cmpl $0, %eax
 movl $0, %eax
 sete %al
 ret
