	.text
.LC0:
	.string	 "%d\n"
printint:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$16, %rsp
	movl	%edi, -4(%rbp)
	movl	-4(%rbp), %eax
	movl	%eax, %esi
	leaq	.LC0(%rip), %rdi
	movl	$0, %eax
	call	printf@PLT
	nop
	leave
	ret

	.globl	main
	.type	main, @function
main:
	pushq	%rbp
	movq	%rsp, %rbp


	.comm	a,8,8
	movq	$2, %r8
	movq	$2, %r9
	cmpq	%r9, %r8
	sete	%r9b
	andq	$255,%r9
	movq	%r9, a(%rip)
	.comm	b,8,8
	movq	$10, %r8
	movq	%r8, b(%rip)
	.comm	c,8,8
	movq	b(%rip), %r8
	movq	$3, %r9
	imulq	%r8, %r9
	movq	%r9, c(%rip)
	.comm	d,8,8
	movq	c(%rip), %r8
	movq	$2, %r9
	movq	%r8, %rax
	cqo
	idivq	%r9
	movq	%rax, %r8
	movq	%r8, d(%rip)
	movq	d(%rip), %r8
	movq	d(%rip), %r9
	movq	$15, %r10
	cmpq	%r10, %r9
	sete	%r10b
	andq	$255,%r10
	movq	%r10, d(%rip)
	movq	d(%rip), %r9
	movq	%r9, %rdi
	call	printint

	movl	$0, %eax
	popq	%rbp
	ret
