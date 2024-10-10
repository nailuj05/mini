.text
.balign 16
add:
	endbr64
	movl %edi, %eax
	addl %esi, %eax
	ret
.type add, @function
.size add, .-add
/* end function add */

.text
.balign 16
.globl main
main:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	movl $1, %esi
	movl $1, %edi
	callq add
	movl %eax, %esi
	leaq fmt(%rip), %rdi
	movl $0, %eax
	callq printf
	movl $0, %eax
	leave
	ret
.type main, @function
.size main, .-main
/* end function main */

.data
.balign 8
fmt:
	.ascii "One and one make %d!\n"
	.byte 0
/* end data */

.section .note.GNU-stack,"",@progbits
