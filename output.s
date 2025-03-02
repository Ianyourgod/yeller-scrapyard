	.text
	.file	"main"
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	$10, 4(%rsp)
	movl	$10, %edi
	callq	fib@PLT
	movl	%eax, (%rsp)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc

	.globl	fib
	.p2align	4, 0x90
	.type	fib,@function
fib:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movl	%edi, 12(%rsp)
	movl	$0, 20(%rsp)
	xorl	%eax, %eax
	testl	%edi, %edi
	sete	%al
	movl	%eax, 40(%rsp)
	je	.LBB1_2
	movl	12(%rsp), %eax
	xorl	%ecx, %ecx
	addl	$-1, %eax
	movl	%eax, 36(%rsp)
	sete	%cl
	movl	%ecx, 44(%rsp)
	jne	.LBB1_3
.LBB1_2:
	movl	$1, %eax
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB1_3:
	.cfi_def_cfa_offset 64
	movl	12(%rsp), %edi
	addl	$-1, %edi
	movl	%edi, 28(%rsp)
	callq	fib@PLT
	movl	%eax, 16(%rsp)
	movl	12(%rsp), %edi
	addl	$-2, %edi
	movl	%edi, 24(%rsp)
	callq	fib@PLT
	movl	%eax, 48(%rsp)
	addl	16(%rsp), %eax
	movl	%eax, 32(%rsp)
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	fib, .Lfunc_end1-fib
	.cfi_endproc

	.section	".note.GNU-stack","",@progbits
