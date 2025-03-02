	.text
	.file	"main"
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	movl	$10, %ecx
	xorl	%eax, %eax
	movl	$10, %edx
	subl	$1, %edx
	jb	.LBB0_2
	.p2align	4, 0x90
.LBB0_3:
	addl	%ecx, %eax
	movl	%edx, %ecx
	subl	$1, %edx
	jae	.LBB0_3
.LBB0_2:
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc

	.section	".note.GNU-stack","",@progbits
