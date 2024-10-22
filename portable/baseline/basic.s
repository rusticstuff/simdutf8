.section __TEXT,__text,regular,pure_instructions
	.globl	simdutf8_portable::implementation::validate_utf8_basic_simd
	.p2align	2
simdutf8_portable::implementation::validate_utf8_basic_simd:
Lfunc_begin4:
	.cfi_startproc
	sub sp, sp, #128
	.cfi_def_cfa_offset 128
	stp x29, x30, [sp, #112]
	add x29, sp, #112
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	mov x10, #0
	and x8, x1, #0xffffffffffffffc0
	mov x9, x8
	cmp x10, x8
	b.hs LBB4_8
	add x11, x0, x10
	ldp q18, q17, [x11]
	ldp q16, q7, [x11, #32]
	add x10, x10, #64
	orr.16b v0, v17, v18
	orr.16b v1, v16, v7
	orr.16b v0, v0, v1
	umaxv.16b b0, v0
	fmov w11, s0
	tbz w11, #7, LBB4_1
	movi.2d v4, #0000000000000000
	ext.16b v2, v4, v18, #15
	ushr.16b v1, v2, #4
Lloh24:
	adrp x9, lCPI4_0@PAGE
Lloh25:
	ldr q0, [x9, lCPI4_0@PAGEOFF]
	tbl.16b v5, { v0 }, v1
	movi.16b v1, #15
	and.16b v3, v2, v1
Lloh26:
	adrp x9, lCPI4_1@PAGE
Lloh27:
	ldr q2, [x9, lCPI4_1@PAGEOFF]
	tbl.16b v6, { v2 }, v3
	ushr.16b v19, v18, #4
Lloh28:
	adrp x9, lCPI4_2@PAGE
Lloh29:
	ldr q3, [x9, lCPI4_2@PAGEOFF]
	tbl.16b v19, { v3 }, v19
	and.16b v5, v6, v5
	and.16b v19, v5, v19
	ext.16b v5, v4, v18, #14
	ext.16b v6, v4, v18, #13
	movi.16b v4, #223
	cmhi.16b v20, v5, v4
	movi.16b v5, #239
	cmhi.16b v6, v6, v5
	orr.16b v20, v6, v20
	movi.16b v6, #128
	and.16b v20, v20, v6
	eor.16b v19, v19, v20
	ext.16b v20, v18, v17, #15
	ushr.16b v21, v20, #4
	tbl.16b v21, { v0 }, v21
	and.16b v20, v20, v1
	tbl.16b v20, { v2 }, v20
	ushr.16b v22, v17, #4
	tbl.16b v22, { v3 }, v22
	and.16b v20, v20, v21
	and.16b v20, v20, v22
	ext.16b v21, v18, v17, #14
	ext.16b v18, v18, v17, #13
	cmhi.16b v21, v21, v4
	cmhi.16b v18, v18, v5
	orr.16b v18, v18, v21
	and.16b v18, v18, v6
	eor.16b v18, v20, v18
	orr.16b v18, v18, v19
	ext.16b v19, v17, v16, #15
	ushr.16b v20, v19, #4
	tbl.16b v20, { v0 }, v20
	and.16b v19, v19, v1
	tbl.16b v19, { v2 }, v19
	ushr.16b v21, v16, #4
	tbl.16b v21, { v3 }, v21
	and.16b v19, v19, v20
	and.16b v19, v19, v21
	ext.16b v20, v17, v16, #14
	ext.16b v17, v17, v16, #13
	cmhi.16b v20, v20, v4
	cmhi.16b v17, v17, v5
	orr.16b v17, v17, v20
	and.16b v17, v17, v6
	eor.16b v17, v19, v17
	ext.16b v19, v16, v7, #15
	ushr.16b v20, v19, #4
	tbl.16b v20, { v0 }, v20
	and.16b v19, v19, v1
	tbl.16b v19, { v2 }, v19
	ushr.16b v21, v7, #4
	tbl.16b v21, { v3 }, v21
	and.16b v19, v19, v20
	and.16b v19, v19, v21
	ext.16b v20, v16, v7, #14
	ext.16b v16, v16, v7, #13
	cmhi.16b v20, v20, v4
	cmhi.16b v16, v16, v5
	orr.16b v16, v16, v20
	and.16b v16, v16, v6
	eor.16b v16, v19, v16
	orr.16b v16, v17, v16
	orr.16b v23, v18, v16
Lloh30:
	adrp x9, lCPI4_3@PAGE
Lloh31:
	ldr q16, [x9, lCPI4_3@PAGEOFF]
	uqsub.16b v19, v7, v16
	cmp x10, x8
	b.hs LBB4_22
	mov x9, x10
	b LBB4_6
	ext.16b v19, v7, v20, #15
	ushr.16b v21, v19, #4
	tbl.16b v21, { v0 }, v21
	and.16b v19, v19, v1
	tbl.16b v19, { v2 }, v19
	ushr.16b v22, v20, #4
	tbl.16b v22, { v3 }, v22
	and.16b v19, v19, v21
	and.16b v19, v19, v22
	ext.16b v21, v7, v20, #14
	ext.16b v7, v7, v20, #13
	cmhi.16b v21, v21, v4
	cmhi.16b v7, v7, v5
	orr.16b v7, v7, v21
	and.16b v7, v7, v6
	eor.16b v7, v19, v7
	ext.16b v19, v20, v18, #15
	ushr.16b v21, v19, #4
	tbl.16b v21, { v0 }, v21
	and.16b v19, v19, v1
	tbl.16b v19, { v2 }, v19
	ushr.16b v22, v18, #4
	tbl.16b v22, { v3 }, v22
	and.16b v19, v19, v21
	and.16b v19, v19, v22
	ext.16b v21, v20, v18, #14
	ext.16b v20, v20, v18, #13
	cmhi.16b v21, v21, v4
	cmhi.16b v20, v20, v5
	orr.16b v20, v20, v21
	and.16b v20, v20, v6
	eor.16b v19, v19, v20
	orr.16b v7, v19, v7
	ext.16b v19, v18, v17, #15
	ushr.16b v20, v19, #4
	tbl.16b v20, { v0 }, v20
	and.16b v19, v19, v1
	tbl.16b v19, { v2 }, v19
	ushr.16b v21, v17, #4
	tbl.16b v21, { v3 }, v21
	and.16b v19, v19, v20
	and.16b v19, v19, v21
	ext.16b v20, v18, v17, #14
	ext.16b v18, v18, v17, #13
	cmhi.16b v20, v20, v4
	cmhi.16b v18, v18, v5
	orr.16b v18, v18, v20
	and.16b v18, v18, v6
	eor.16b v18, v19, v18
	ext.16b v19, v17, v24, #15
	ushr.16b v20, v19, #4
	tbl.16b v20, { v0 }, v20
	and.16b v19, v19, v1
	tbl.16b v19, { v2 }, v19
	ushr.16b v21, v24, #4
	tbl.16b v21, { v3 }, v21
	and.16b v19, v19, v20
	and.16b v19, v19, v21
	ext.16b v20, v17, v24, #14
	ext.16b v17, v17, v24, #13
	cmhi.16b v20, v20, v4
	cmhi.16b v17, v17, v5
	orr.16b v17, v17, v20
	and.16b v17, v17, v6
	eor.16b v17, v19, v17
	orr.16b v17, v18, v17
	orr.16b v19, v7, v17
	mov.16b v7, v24
	uqsub.16b v17, v24, v16
	orr.16b v23, v19, v23
	add x9, x9, #64
	mov.16b v19, v17
	cmp x9, x8
	b.hs LBB4_9
LBB4_6:
	add x10, x0, x9
	ldp q20, q18, [x10]
	ldp q17, q24, [x10, #32]
	orr.16b v21, v18, v20
	orr.16b v22, v17, v24
	orr.16b v21, v21, v22
	umaxv.16b b21, v21
	fmov w10, s21
	tbnz w10, #7, LBB4_5
	mov.16b v17, v19
	mov.16b v24, v7
	orr.16b v23, v19, v23
	add x9, x9, #64
	mov.16b v19, v17
	cmp x9, x8
	b.lo LBB4_6
	b LBB4_9
LBB4_8:
	movi.2d v23, #0000000000000000
	movi.2d v17, #0000000000000000
	movi.2d v24, #0000000000000000
LBB4_9:
	subs x2, x1, x9
	b.ls LBB4_18
LBB4_10:
	movi.2d v0, #0000000000000000
	stp q0, q0, [sp, #80]
	stp q0, q0, [sp, #48]
	add x1, x0, x9
	add x0, sp, #48
	subs x8, x2, #32
	b.hs LBB4_19
	subs x8, x2, #16
	b.hs LBB4_20
LBB4_12:
	subs x8, x2, #8
	b.hs LBB4_21
LBB4_13:
	cbz x2, LBB4_15
	stp q24, q23, [sp, #16]
	str q17, [sp]
	bl _memcpy
	ldp q17, q24, [sp]
	ldr q23, [sp, #32]
LBB4_15:
	ldp q3, q2, [sp, #48]
	ldp q1, q0, [sp, #80]
	orr.16b v4, v2, v3
	orr.16b v5, v1, v0
	orr.16b v4, v4, v5
	umaxv.16b b4, v4
	fmov w8, s4
	mov.16b v4, v17
	tbz w8, #7, LBB4_17
	ext.16b v4, v24, v3, #15
	ushr.16b v5, v4, #4
Lloh32:
	adrp x8, lCPI4_0@PAGE
Lloh33:
	ldr q6, [x8, lCPI4_0@PAGEOFF]
	tbl.16b v5, { v6 }, v5
	movi.16b v7, #15
	and.16b v4, v4, v7
Lloh34:
	adrp x8, lCPI4_1@PAGE
Lloh35:
	ldr q16, [x8, lCPI4_1@PAGEOFF]
	tbl.16b v4, { v16 }, v4
	ushr.16b v17, v3, #4
Lloh36:
	adrp x8, lCPI4_2@PAGE
Lloh37:
	ldr q18, [x8, lCPI4_2@PAGEOFF]
	tbl.16b v17, { v18 }, v17
	and.16b v4, v4, v5
	and.16b v4, v4, v17
	ext.16b v5, v24, v3, #14
	ext.16b v17, v24, v3, #13
	movi.16b v19, #223
	cmhi.16b v5, v5, v19
	movi.16b v20, #239
	cmhi.16b v17, v17, v20
	orr.16b v5, v17, v5
	movi.16b v17, #128
	and.16b v5, v5, v17
	eor.16b v4, v4, v5
	ext.16b v5, v3, v2, #15
	ushr.16b v21, v5, #4
	tbl.16b v21, { v6 }, v21
	and.16b v5, v5, v7
	tbl.16b v5, { v16 }, v5
	ushr.16b v22, v2, #4
	tbl.16b v22, { v18 }, v22
	and.16b v5, v5, v21
	and.16b v5, v5, v22
	ext.16b v21, v3, v2, #14
	ext.16b v3, v3, v2, #13
	cmhi.16b v21, v21, v19
	cmhi.16b v3, v3, v20
	orr.16b v3, v3, v21
	and.16b v3, v3, v17
	eor.16b v3, v5, v3
	orr.16b v3, v3, v4
	ext.16b v4, v2, v1, #15
	ushr.16b v5, v4, #4
	tbl.16b v5, { v6 }, v5
	and.16b v4, v4, v7
	tbl.16b v4, { v16 }, v4
	ushr.16b v21, v1, #4
	tbl.16b v21, { v18 }, v21
	and.16b v4, v4, v5
	and.16b v4, v4, v21
	ext.16b v5, v2, v1, #14
	ext.16b v2, v2, v1, #13
	cmhi.16b v5, v5, v19
	cmhi.16b v2, v2, v20
	orr.16b v2, v2, v5
	and.16b v2, v2, v17
	eor.16b v2, v4, v2
	ext.16b v4, v1, v0, #15
	ushr.16b v5, v4, #4
	tbl.16b v5, { v6 }, v5
	and.16b v4, v4, v7
	tbl.16b v4, { v16 }, v4
	ushr.16b v6, v0, #4
	tbl.16b v6, { v18 }, v6
	and.16b v4, v4, v5
	and.16b v4, v4, v6
	ext.16b v5, v1, v0, #14
	ext.16b v1, v1, v0, #13
	cmhi.16b v5, v5, v19
	cmhi.16b v1, v1, v20
	orr.16b v1, v1, v5
	and.16b v1, v1, v17
	eor.16b v1, v4, v1
	orr.16b v1, v2, v1
	orr.16b v17, v3, v1
Lloh38:
	adrp x8, lCPI4_3@PAGE
Lloh39:
	ldr q1, [x8, lCPI4_3@PAGEOFF]
	uqsub.16b v4, v0, v1
	orr.16b v23, v17, v23
	mov.16b v17, v4
LBB4_18:
	orr.16b v0, v17, v23
	umaxv.16b b0, v0
	fmov w8, s0
	tst w8, #0xff
	cset w0, ne
	.cfi_def_cfa wsp, 128
	ldp x29, x30, [sp, #112]
	add sp, sp, #128
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB4_19:
	.cfi_restore_state
	ldp q0, q1, [x1], #32
	stp q0, q1, [sp, #48]
	add x0, x0, #32
	mov x2, x8
	subs x8, x8, #16
	b.lo LBB4_12
LBB4_20:
	ldr q0, [x1], #16
	str q0, [x0], #16
	mov x2, x8
	subs x8, x8, #8
	b.lo LBB4_13
LBB4_21:
	ldr x9, [x1], #8
	str x9, [x0], #8
	mov x2, x8
	cbnz x8, LBB4_14
	b LBB4_15
LBB4_22:
	mov.16b v17, v19
	mov.16b v24, v7
	mov x9, x10
	subs x2, x1, x10
	b.hi LBB4_10
	b LBB4_18
	.loh AdrpLdr	Lloh30, Lloh31
	.loh AdrpAdrp	Lloh28, Lloh30
	.loh AdrpLdr	Lloh28, Lloh29
	.loh AdrpAdrp	Lloh26, Lloh28
	.loh AdrpLdr	Lloh26, Lloh27
	.loh AdrpAdrp	Lloh24, Lloh26
	.loh AdrpLdr	Lloh24, Lloh25
	.loh AdrpLdr	Lloh38, Lloh39
	.loh AdrpAdrp	Lloh36, Lloh38
	.loh AdrpLdr	Lloh36, Lloh37
	.loh AdrpAdrp	Lloh34, Lloh36
	.loh AdrpLdr	Lloh34, Lloh35
	.loh AdrpAdrp	Lloh32, Lloh34
	.loh AdrpLdr	Lloh32, Lloh33
