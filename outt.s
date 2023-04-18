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
	movq	$2, %r10
	addq	%r9, %r10
	movq	$5, %r9
	movq	$8, %r11
	imulq	%r9, %r11
	movq	$2, %r9
	movq	%r11, %rax
	cqo
	idivq	%r9
	movq	%rax, %r11
	subq	%r11, %r10
	imulq	%r8, %r10
	movq	%r10, a(%rip)
	movq	a(%rip), %r8
	movq	$3, %r9
	addq	%r8, %r9
	movq	$4, %r8
	movq	$1, %r10
	addq	%r8, %r10
	movq	%r9, %rax
	cqo
	idivq	%r10
	movq	%rax, %r9
	movq	%r9, %rdi
	call	printint

	movl	$0, %eax
	popq	%rbp
	ret
