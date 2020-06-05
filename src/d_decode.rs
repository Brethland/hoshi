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
    
    let sel_v_a = src.instance("sel_v_a", "12_1_selector_NIBBLE");
    sel_v_a.drive_input("s_sel", d_icode);

    sel_v_a.drive_input("s_m1", r_none);
    sel_v_a.drive_input("s_m2", r_none);
    sel_v_a.drive_input("s_m3", d_r_a);
    sel_v_a.drive_input("s_m4", r_none);
    sel_v_a.drive_input("s_m5", d_r_a);
    sel_v_a.drive_input("s_m6", r_none);
    sel_v_a.drive_input("s_m7", d_r_a);
    sel_v_a.drive_input("s_m8", r_none);
    sel_v_a.drive_input("s_m9", r_none);
    sel_v_a.drive_input("s_m10", r_rsp);
    sel_v_a.drive_input("s_m11", d_r_a);
    sel_v_a.drive_input("s_m12", r_rsp);

    src.output("d_srcA", sel_v_a.output("sel_out_NIBBLE"));

    src
}

pub fn src_b<'a>(c: &'a Context<'a>) -> &'a Module<'a> {
    let src = c.module("SrcB");

    let d_icode = src.input("D_icode", NIBBLE);
    let d_r_b   = src.input("D_rB", NIBBLE);

    let r_none = src.lit(RNONE, NIBBLE);
    let r_rsp  = src.lit(RRSP, NIBBLE);
    
    let sel_v_b = src.instance("sel_v_b", "12_1_selector_NIBBLE");
    sel_v_b.drive_input("s_sel", d_icode);

    sel_v_b.drive_input("s_m1", r_none);
    sel_v_b.drive_input("s_m2", r_none);
    sel_v_b.drive_input("s_m3", r_none);
    sel_v_b.drive_input("s_m4", r_none);
    sel_v_b.drive_input("s_m5", d_r_b);
    sel_v_b.drive_input("s_m6", d_r_b);
    sel_v_b.drive_input("s_m7", d_r_b);
    sel_v_b.drive_input("s_m8", r_none);
    sel_v_b.drive_input("s_m9", r_rsp);
    sel_v_b.drive_input("s_m10", r_rsp);
    sel_v_b.drive_input("s_m11", r_rsp);
    sel_v_b.drive_input("s_m12", r_rsp);

    src.output("d_srcB", sel_v_b.output("sel_out_NIBBLE"));

    src
}

pub fn dst_e<'a>(c: &'a Context<'a>) -> &'a Module<'a> {
    let dst = c.module("DstE");

    let d_icode = dst.input("D_icode", NIBBLE);
    let d_r_b   = dst.input("D_rB", NIBBLE);

    let r_none = dst.lit(RNONE, NIBBLE);
    let r_rsp  = dst.lit(RRSP, NIBBLE);
    
    let sel_d_e = dst.instance("sel_d_e", "12_1_selector_NIBBLE");
    sel_d_e.drive_input("s_sel", d_icode);

    sel_d_e.drive_input("s_m1", r_none);
    sel_d_e.drive_input("s_m2", r_none);
    sel_d_e.drive_input("s_m3", d_r_b);
    sel_d_e.drive_input("s_m4", d_r_b);
    sel_d_e.drive_input("s_m5", r_none);
    sel_d_e.drive_input("s_m6", r_none);
    sel_d_e.drive_input("s_m7", d_r_b);
    sel_d_e.drive_input("s_m8", r_none);
    sel_d_e.drive_input("s_m9", r_rsp);
    sel_d_e.drive_input("s_m10", r_rsp);
    sel_d_e.drive_input("s_m11", r_rsp);
    sel_d_e.drive_input("s_m12", r_rsp);

    dst.output("d_dstE", sel_d_e.output("sel_out_NIBBLE"));

    dst
}

pub fn dst_m<'a>(c: &'a Context<'a>) -> &'a Module<'a> {
    let dst = c.module("DstM");

    let d_icode = dst.input("D_icode", NIBBLE);
    let d_r_a   = dst.input("D_rA", NIBBLE);

    let r_none = dst.lit(RNONE, NIBBLE);

    let mux_1 = dst.mux(d_icode.eq(dst.lit(IMRMOVQ, NIBBLE)), d_r_a, r_none);
    let mux_2 = dst.mux(d_icode.eq(dst.lit(IPOPQ, NIBBLE)), d_r_a, mux_1);

    dst.output("d_dstM", mux_2);

    dst
}