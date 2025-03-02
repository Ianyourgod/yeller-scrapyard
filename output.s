	.text
	.file	"main"
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movl	$72, %edi
	callq	putchar@PLT
	movl	%eax, 44(%rsp)
	movl	$101, %edi
	callq	putchar@PLT
	movl	%eax, 24(%rsp)
	movl	$108, %edi
	callq	putchar@PLT
	movl	%eax, 4(%rsp)
	movl	$108, %edi
	callq	putchar@PLT
	movl	%eax, 28(%rsp)
	movl	$111, %edi
	callq	putchar@PLT
	movl	%eax, 36(%rsp)
	movl	$44, %edi
	callq	putchar@PLT
	movl	%eax, 40(%rsp)
	movl	$32, %edi
	callq	putchar@PLT
	movl	%eax, 32(%rsp)
	movl	$87, %edi
	callq	putchar@PLT
	movl	%eax, (%rsp)
	movl	$111, %edi
	callq	putchar@PLT
	movl	%eax, 20(%rsp)
	movl	$114, %edi
	callq	putchar@PLT
	movl	%eax, 16(%rsp)
	movl	$108, %edi
	callq	putchar@PLT
	movl	%eax, 52(%rsp)
	movl	$100, %edi
	callq	putchar@PLT
	movl	%eax, 12(%rsp)
	movl	$33, %edi
	callq	putchar@PLT
	movl	%eax, 48(%rsp)
	movl	$10, %edi
	callq	putchar@PLT
	movl	%eax, 8(%rsp)
	xorl	%eax, %eax
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc

	.section	".note.GNU-stack","",@progbits
