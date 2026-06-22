.section __TEXT,__text,regular,pure_instructions
	.globl	simdutf8_portable::implementation::validate_utf8_compat_simd
	.p2align	2
simdutf8_portable::implementation::validate_utf8_compat_simd:
Lfunc_begin5:
	.cfi_startproc
	sub sp, sp, #224
	.cfi_def_cfa_offset 224
	stp x22, x21, [sp, #176]
	stp x20, x19, [sp, #192]
	stp x29, x30, [sp, #208]
	add x29, sp, #208
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_remember_state
	mov x19, x1
	mov x20, x0
	mov x21, #0
	and x9, x1, #0xffffffffffffffc0
	movi.2d v20, #0000000000000000
Lloh40:
	adrp x10, lCPI5_0@PAGE
Lloh41:
	ldr q22, [x10, lCPI5_0@PAGEOFF]
Lloh42:
	adrp x10, lCPI5_1@PAGE
Lloh43:
	ldr q23, [x10, lCPI5_1@PAGEOFF]
Lloh44:
	adrp x10, lCPI5_2@PAGE
Lloh45:
	ldr q24, [x10, lCPI5_2@PAGEOFF]
Lloh46:
	adrp x10, lCPI5_3@PAGE
Lloh47:
	ldr q21, [x10, lCPI5_3@PAGEOFF]
	mov w10, #1
	movi.16b v0, #15
	movi.16b v1, #223
	movi.16b v2, #239
	movi.16b v3, #128
	movi.2d v18, #0000000000000000
	movi.2d v19, #0000000000000000
	cmp x21, x9
	tbz w10, #0, LBB5_4
LBB5_1:
	b.hs LBB5_15
LBB5_2:
	add x11, x20, x21
	ldp q7, q6, [x11]
	ldp q5, q4, [x11, #32]
	orr.16b v16, v6, v7
	orr.16b v17, v5, v4
	orr.16b v16, v16, v17
	umaxv.16b b16, v16
	fmov w11, s16
	tbnz w11, #7, LBB5_11
	add x21, x21, #64
	cmp x21, x9
	b.lo LBB5_2
	b LBB5_15
	b.hs LBB5_15
	add x11, x20, x21
	ldp q6, q5, [x11]
	ldp q4, q7, [x11, #32]
	orr.16b v16, v5, v6
	orr.16b v17, v4, v7
	orr.16b v16, v16, v17
	umaxv.16b b16, v16
	fmov w11, s16
	tbnz w11, #7, LBB5_7
	orr.16b v20, v20, v18
	umaxv.16b b4, v20
	fmov w11, s4
	cbz w11, LBB5_13
	b LBB5_29
	ext.16b v17, v19, v6, #15
	ext.16b v18, v19, v6, #14
	ext.16b v16, v19, v6, #13
	mov.16b v19, v7
	ushr.16b v7, v17, #4
	tbl.16b v7, { v22 }, v7
	and.16b v17, v17, v0
	tbl.16b v17, { v23 }, v17
	and.16b v7, v17, v7
	ushr.16b v17, v6, #4
	tbl.16b v17, { v24 }, v17
	and.16b v7, v7, v17
	cmhi.16b v17, v18, v1
	cmhi.16b v16, v16, v2
	orr.16b v16, v16, v17
	and.16b v16, v16, v3
	eor.16b v7, v7, v16
	ext.16b v16, v6, v5, #15
	ushr.16b v17, v16, #4
	tbl.16b v17, { v22 }, v17
	and.16b v16, v16, v0
	tbl.16b v16, { v23 }, v16
	ushr.16b v18, v5, #4
	tbl.16b v18, { v24 }, v18
	and.16b v16, v16, v17
	and.16b v16, v16, v18
	ext.16b v17, v6, v5, #14
	ext.16b v6, v6, v5, #13
	cmhi.16b v17, v17, v1
	cmhi.16b v6, v6, v2
	orr.16b v6, v6, v17
	and.16b v6, v6, v3
	eor.16b v6, v16, v6
	ext.16b v16, v5, v4, #15
	ushr.16b v17, v16, #4
	tbl.16b v17, { v22 }, v17
	and.16b v16, v16, v0
	tbl.16b v16, { v23 }, v16
	ushr.16b v18, v4, #4
	tbl.16b v18, { v24 }, v18
	and.16b v16, v16, v17
	and.16b v16, v16, v18
	ext.16b v17, v5, v4, #14
	ext.16b v5, v5, v4, #13
	cmhi.16b v17, v17, v1
	cmhi.16b v5, v5, v2
	orr.16b v5, v5, v17
	and.16b v5, v5, v3
	eor.16b v5, v16, v5
	ext.16b v16, v4, v19, #15
	ushr.16b v17, v16, #4
	tbl.16b v17, { v22 }, v17
	and.16b v16, v16, v0
	tbl.16b v16, { v23 }, v16
	ushr.16b v18, v19, #4
	tbl.16b v18, { v24 }, v18
	and.16b v16, v16, v17
	and.16b v16, v16, v18
	ext.16b v17, v4, v19, #14
	ext.16b v4, v4, v19, #13
	cmhi.16b v17, v17, v1
	cmhi.16b v4, v4, v2
	orr.16b v4, v4, v17
	and.16b v4, v4, v3
	eor.16b v4, v16, v4
	orr.16b v7, v7, v20
	orr.16b v5, v6, v5
	orr.16b v5, v7, v5
	orr.16b v20, v5, v4
	umaxv.16b b4, v20
	fmov w11, s4
	cbnz w11, LBB5_29
	add x11, x21, #64
	cmp x11, x9
	b.hs LBB5_14
	add x12, x20, x21
	ldp q6, q5, [x12, #64]
	ldp q4, q7, [x12, #96]
	orr.16b v16, v5, v6
	orr.16b v17, v4, v7
	orr.16b v16, v16, v17
	umaxv.16b b16, v16
	fmov w12, s16
	mov x21, x11
	tbnz w12, #7, LBB5_7
	uqsub.16b v18, v19, v21
	mov x21, x11
	orr.16b v20, v20, v18
	umaxv.16b b4, v20
	fmov w11, s4
	cbz w11, LBB5_13
	b LBB5_29
	ext.16b v16, v19, v7, #15
	ushr.16b v17, v16, #4
	tbl.16b v17, { v22 }, v17
	and.16b v16, v16, v0
	tbl.16b v16, { v23 }, v16
	ushr.16b v18, v7, #4
	tbl.16b v18, { v24 }, v18
	and.16b v16, v16, v17
	and.16b v16, v16, v18
	ext.16b v17, v19, v7, #14
	ext.16b v18, v19, v7, #13
	cmhi.16b v17, v17, v1
	cmhi.16b v18, v18, v2
	orr.16b v17, v18, v17
	and.16b v17, v17, v3
	eor.16b v16, v16, v17
	ext.16b v17, v7, v6, #15
	ushr.16b v18, v17, #4
	tbl.16b v18, { v22 }, v18
	and.16b v17, v17, v0
	tbl.16b v17, { v23 }, v17
	ushr.16b v19, v6, #4
	tbl.16b v19, { v24 }, v19
	and.16b v17, v17, v18
	and.16b v17, v17, v19
	ext.16b v18, v7, v6, #14
	ext.16b v7, v7, v6, #13
	cmhi.16b v18, v18, v1
	cmhi.16b v7, v7, v2
	orr.16b v7, v7, v18
	and.16b v7, v7, v3
	eor.16b v7, v17, v7
	ext.16b v17, v6, v5, #15
	ushr.16b v18, v17, #4
	tbl.16b v18, { v22 }, v18
	and.16b v17, v17, v0
	tbl.16b v17, { v23 }, v17
	ushr.16b v19, v5, #4
	tbl.16b v19, { v24 }, v19
	and.16b v17, v17, v18
	and.16b v17, v17, v19
	ext.16b v18, v6, v5, #14
	ext.16b v6, v6, v5, #13
	cmhi.16b v18, v18, v1
	cmhi.16b v6, v6, v2
	orr.16b v6, v6, v18
	and.16b v6, v6, v3
	eor.16b v6, v17, v6
	ext.16b v17, v5, v4, #15
	ushr.16b v18, v17, #4
	tbl.16b v18, { v22 }, v18
	and.16b v17, v17, v0
	tbl.16b v17, { v23 }, v17
	ushr.16b v19, v4, #4
	tbl.16b v19, { v24 }, v19
	and.16b v17, v17, v18
	and.16b v17, v17, v19
	ext.16b v18, v5, v4, #14
	ext.16b v5, v5, v4, #13
	cmhi.16b v18, v18, v1
	cmhi.16b v5, v5, v2
	orr.16b v5, v5, v18
	and.16b v5, v5, v3
	eor.16b v5, v17, v5
	orr.16b v7, v16, v7
	orr.16b v5, v6, v5
	orr.16b v5, v7, v5
	orr.16b v20, v5, v20
	umaxv.16b b5, v20
	fmov w11, s5
	cbnz w11, LBB5_29
	uqsub.16b v18, v4, v21
	mov.16b v19, v4
	add x21, x21, #64
	eor w10, w10, #0x1
	cmp x21, x9
	tbz w10, #0, LBB5_4
	b LBB5_1
LBB5_14:
	uqsub.16b v18, v19, v21
	mov x21, x11
LBB5_15:
	subs x2, x19, x21
	b.ls LBB5_24
	movi.2d v0, #0000000000000000
	stp q0, q0, [x29, #-64]
	stp q0, q0, [x29, #-96]
	add x1, x20, x21
	sub x0, x29, #96
	subs x9, x2, #32
	b.hs LBB5_26
	subs x9, x2, #16
	b.hs LBB5_27
LBB5_18:
	subs x9, x2, #8
	str q20, [sp, #96]
	b.hs LBB5_28
LBB5_19:
	cbz x2, LBB5_21
	mov x22, x8
	stp q22, q21, [sp, #64]
	stp q24, q23, [sp, #32]
	stp q18, q19, [sp]
	bl _memcpy
	ldp q18, q19, [sp]
	ldp q24, q23, [sp, #32]
	ldp q22, q21, [sp, #64]
	mov x8, x22
LBB5_21:
	ldp q3, q2, [x29, #-96]
	ldp q1, q0, [x29, #-64]
	orr.16b v4, v2, v3
	orr.16b v5, v1, v0
	orr.16b v4, v4, v5
	umaxv.16b b4, v4
	fmov w9, s4
	mov.16b v4, v18
	tbz w9, #7, LBB5_23
	ext.16b v4, v19, v3, #15
	ushr.16b v5, v4, #4
	tbl.16b v5, { v22 }, v5
	movi.16b v6, #15
	and.16b v4, v4, v6
	tbl.16b v4, { v23 }, v4
	ushr.16b v7, v3, #4
	tbl.16b v7, { v24 }, v7
	and.16b v4, v4, v5
	and.16b v4, v4, v7
	ext.16b v5, v19, v3, #14
	ext.16b v7, v19, v3, #13
	movi.16b v16, #223
	cmhi.16b v5, v5, v16
	movi.16b v17, #239
	cmhi.16b v7, v7, v17
	orr.16b v5, v7, v5
	movi.16b v7, #128
	and.16b v5, v5, v7
	eor.16b v4, v4, v5
	ext.16b v5, v3, v2, #15
	ushr.16b v18, v5, #4
	tbl.16b v18, { v22 }, v18
	and.16b v5, v5, v6
	tbl.16b v5, { v23 }, v5
	ushr.16b v19, v2, #4
	tbl.16b v19, { v24 }, v19
	and.16b v5, v5, v18
	and.16b v5, v5, v19
	ext.16b v18, v3, v2, #14
	ext.16b v3, v3, v2, #13
	cmhi.16b v18, v18, v16
	cmhi.16b v3, v3, v17
	orr.16b v3, v3, v18
	and.16b v3, v3, v7
	eor.16b v3, v5, v3
	orr.16b v3, v3, v4
	ext.16b v4, v2, v1, #15
	ushr.16b v5, v4, #4
	tbl.16b v5, { v22 }, v5
	and.16b v4, v4, v6
	tbl.16b v4, { v23 }, v4
	ushr.16b v18, v1, #4
	tbl.16b v18, { v24 }, v18
	and.16b v4, v4, v5
	and.16b v4, v4, v18
	ext.16b v5, v2, v1, #14
	ext.16b v2, v2, v1, #13
	cmhi.16b v5, v5, v16
	cmhi.16b v2, v2, v17
	orr.16b v2, v2, v5
	and.16b v2, v2, v7
	eor.16b v2, v4, v2
	ext.16b v4, v1, v0, #15
	ushr.16b v5, v4, #4
	tbl.16b v5, { v22 }, v5
	and.16b v4, v4, v6
	tbl.16b v4, { v23 }, v4
	ushr.16b v6, v0, #4
	tbl.16b v6, { v24 }, v6
	and.16b v4, v4, v5
	and.16b v4, v4, v6
	ext.16b v5, v1, v0, #14
	ext.16b v1, v1, v0, #13
	cmhi.16b v5, v5, v16
	cmhi.16b v1, v1, v17
	orr.16b v1, v1, v5
	and.16b v1, v1, v7
	eor.16b v1, v4, v1
	orr.16b v1, v2, v1
	orr.16b v18, v3, v1
	uqsub.16b v4, v0, v21
	ldr q20, [sp, #96]
	orr.16b v20, v18, v20
	mov.16b v18, v4
LBB5_24:
	orr.16b v0, v18, v20
	umaxv.16b b0, v0
	fmov w9, s0
	cbnz w9, LBB5_29
	mov w9, #2
	strb w9, [x8, #8]
	.cfi_def_cfa wsp, 224
	ldp x29, x30, [sp, #208]
	ldp x20, x19, [sp, #192]
	ldp x22, x21, [sp, #176]
	add sp, sp, #224
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	ret
LBB5_26:
	.cfi_restore_state
	ldp q0, q1, [x1], #32
	stp q0, q1, [x29, #-96]
	add x0, x0, #32
	mov x2, x9
	subs x9, x9, #16
	b.lo LBB5_18
LBB5_27:
	ldr q0, [x1], #16
	str q0, [x0], #16
	mov x2, x9
	subs x9, x9, #8
	str q20, [sp, #96]
	b.lo LBB5_19
LBB5_28:
	ldr x10, [x1], #8
	str x10, [x0], #8
	mov x2, x9
	cbnz x9, LBB5_20
	b LBB5_21
LBB5_29:
	mov x0, x8
	mov x1, x20
	mov x2, x19
	mov x3, x21
	.cfi_def_cfa wsp, 224
	ldp x29, x30, [sp, #208]
	ldp x20, x19, [sp, #192]
	ldp x22, x21, [sp, #176]
	add sp, sp, #224
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	b simdutf8_portable::implementation::helpers::get_compat_error
	.loh AdrpLdr	Lloh46, Lloh47
	.loh AdrpAdrp	Lloh44, Lloh46
	.loh AdrpLdr	Lloh44, Lloh45
	.loh AdrpAdrp	Lloh42, Lloh44
	.loh AdrpLdr	Lloh42, Lloh43
	.loh AdrpAdrp	Lloh40, Lloh42
	.loh AdrpLdr	Lloh40, Lloh41
