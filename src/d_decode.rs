use crate::y86_defines::*;
use kaze::*;

pub fn sel_fwd_a<'a>(c: &'a Context<'a>) -> &'a Module<'a> {
    let sel = c.module("SelFwdA");

    let d_icode  = sel.input("D_icode", NIBBLE);
    let d_val_p  = sel.input("D_valP", QWORD);
    let d_src_a  = sel.input("d_srcA", NIBBLE);
    let d_rval_a = sel.input("d_rvalA", QWORD);
    let e_dst_e  = sel.input("e_dstE", NIBBLE);
    let e_val_e  = sel.input("e_valE", QWORD);
    let m_dst_e  = sel.input("M_dstE", NIBBLE);
    let m_val_e  = sel.input("M_valE", QWORD);
    let m_dst_m  = sel.input("M_dstM", NIBBLE);
    let m_val_m  = sel.input("m_valM", QWORD);
    let w_dst_e  = sel.input("W_dstE", NIBBLE);
    let w_val_e  = sel.input("W_valE", QWORD);
    let w_dst_m  = sel.input("W_dstM", NIBBLE);
    let w_val_m  = sel.input("W_valM", QWORD);

    // Orders matter
    let mux_w_dst_e = sel.mux(d_src_a.eq(w_dst_e), w_val_e, d_rval_a);
    let mux_w_dst_m = sel.mux(d_src_a.eq(w_dst_m), w_val_m, mux_w_dst_e);
    let mux_m_dst_e = sel.mux(d_src_a.eq(m_dst_e), m_val_e, mux_w_dst_m);
    let mux_m_dst_m = sel.mux(d_src_a.eq(m_dst_m), m_val_m, mux_m_dst_e);
    let mux_e_dst_e = sel.mux(d_src_a.eq(e_dst_e), e_val_e, mux_m_dst_m);
    let mux_1       = sel.mux(d_icode.eq(sel.lit(ICALL, NIBBLE)), d_val_p, mux_e_dst_e);
    let mux_2       = sel.mux(d_icode.eq(sel.lit(IJXX, NIBBLE)), d_val_p, mux_1);

    sel.output("d_valA", mux_2);

    sel
}

pub fn sel_fwd_b<'a>(c: &'a Context<'a>) -> &'a Module<'a> {
    let sel = c.module("SelFwdB");

    let d_src_b  = sel.input("d_srcB", NIBBLE);
    let d_rval_b = sel.input("d_rvalB", QWORD);
    let e_dst_e  = sel.input("e_dstE", NIBBLE);
    let e_val_e  = sel.input("e_valE", QWORD);
    let m_dst_e  = sel.input("M_dstE", NIBBLE);
    let m_val_e  = sel.input("M_valE", QWORD);
    let m_dst_m  = sel.input("M_dstM", NIBBLE);
    let m_val_m  = sel.input("m_valM", QWORD);
    let w_dst_e  = sel.input("W_dstE", NIBBLE);
    let w_val_e  = sel.input("W_valE", QWORD);
    let w_dst_m  = sel.input("W_dstM", NIBBLE);
    let w_val_m  = sel.input("W_valM", QWORD);

    let mux_w_dst_e = sel.mux(d_src_b.eq(w_dst_e), w_val_e, d_rval_b);
    let mux_w_dst_m = sel.mux(d_src_b.eq(w_dst_m), w_val_m, mux_w_dst_e);
    let mux_m_dst_e = sel.mux(d_src_b.eq(m_dst_e), m_val_e, mux_w_dst_m);
    let mux_m_dst_m = sel.mux(d_src_b.eq(m_dst_m), m_val_m, mux_m_dst_e);
    let mux_e_dst_e = sel.mux(d_src_b.eq(e_dst_e), e_val_e, mux_m_dst_m);

    sel.output("d_valB", mux_e_dst_e);

    sel
}

pub fn src_a<'a>(c: &'a Context<'a>) -> &'a Module<'a> {
    let src = c.module("SrcA");

    let d_icode = src.input("D_icode", NIBBLE);
    let d_r_a   = src.input("D_rA", NIBBLE);

    let r_none = src.lit(RNONE, NIBBLE);
    let r_rsp  = src.lit(RRSP, NIBBLE);

    let mux_1 = src.mux(d_icode.eq(src.lit(IHALT, NIBBLE)) | d_icode.eq(src.lit(INOP, NIBBLE)) | d_icode.eq(src.lit(IIRMOVQ, NIBBLE)) |
                         d_icode.eq(src.lit(IMRMOVQ, NIBBLE)) | d_icode.eq(src.lit(IJXX, NIBBLE)) | d_icode.eq(src.lit(ICALL, NIBBLE)), r_none, r_rsp);
    let mux_2 = src.mux(d_icode.eq(src.lit(IRRMOVQ, NIBBLE)) | d_icode.eq(src.lit(IRMMOVQ, NIBBLE)) | d_icode.eq(src.lit(IOPQ, NIBBLE)) |
                         d_icode.eq(src.lit(IPUSHQ, NIBBLE)), d_r_a, mux_1);

    src.output("d_srcA", mux_2);

    src
}

pub fn src_b<'a>(c: &'a Context<'a>) -> &'a Module<'a> {
    let src = c.module("SrcB");

    let d_icode = src.input("D_icode", NIBBLE);
    let d_r_b   = src.input("D_rB", NIBBLE);

    let r_none = src.lit(RNONE, NIBBLE);
    let r_rsp  = src.lit(RRSP, NIBBLE);

    let mux_1 = src.mux(d_icode.eq(src.lit(IHALT, NIBBLE)) | d_icode.eq(src.lit(INOP, NIBBLE)) | d_icode.eq(src.lit(IIRMOVQ, NIBBLE)) |
                         d_icode.eq(src.lit(IRRMOVQ, NIBBLE)) | d_icode.eq(src.lit(IJXX, NIBBLE)), r_none, r_rsp);
    let mux_2 = src.mux(d_icode.eq(src.lit(IMRMOVQ, NIBBLE)) | d_icode.eq(src.lit(IRMMOVQ, NIBBLE)) | d_icode.eq(src.lit(IOPQ, NIBBLE)), d_r_b, mux_1);

    src.output("d_srcB", mux_2);

    src
}

pub fn dst_e<'a>(c: &'a Context<'a>) -> &'a Module<'a> {
    let dst = c.module("DstE");

    let d_icode = dst.input("D_icode", NIBBLE);
    let d_r_b   = dst.input("D_rB", NIBBLE);

    let r_none = dst.lit(RNONE, NIBBLE);
    let r_rsp  = dst.lit(RRSP, NIBBLE);

    let mux_1 = dst.mux(d_icode.eq(dst.lit(IHALT, NIBBLE)) | d_icode.eq(dst.lit(INOP, NIBBLE)) | d_icode.eq(dst.lit(IMRMOVQ, NIBBLE)) |
                         d_icode.eq(dst.lit(IRMMOVQ, NIBBLE)) | d_icode.eq(dst.lit(IJXX, NIBBLE)), r_none, r_rsp);
    let mux_2 = dst.mux(d_icode.eq(dst.lit(IIRMOVQ, NIBBLE)) | d_icode.eq(dst.lit(IRRMOVQ, NIBBLE)) | d_icode.eq(dst.lit(IOPQ, NIBBLE)), d_r_b, mux_1);

    dst.output("d_dstE", mux_2);

    dst
}

pub fn dst_m<'a>(c: &'a Context<'a>) -> &'a Module<'a> {
    let dst = c.module("DstM");

    let d_icode = dst.input("D_icode", NIBBLE);
    let d_r_a   = dst.input("D_rA", NIBBLE);

    let r_none = dst.lit(RNONE, NIBBLE);

    let mux_1 = dst.mux(d_icode.eq(dst.lit(IMRMOVQ, NIBBLE)) | d_icode.eq(dst.lit(IPOPQ, NIBBLE)), d_r_a, r_none);

    dst.output("d_dstM", mux_1);

    dst
}