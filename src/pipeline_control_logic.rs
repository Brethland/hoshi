use crate::y86_defines::*;
use kaze::*;


pub fn pipeline<'a>(c: &'a Context<'a>) -> &'a Module {
    let p = c.module("Pipeline");

    let d_icode = p.input("D_icode", NIBBLE);
    let d_src_a = p.input("d_srcA", NIBBLE);
    let d_src_b = p.input("d_srcB", NIBBLE);
    let e_icode = p.input("E_icode", NIBBLE);
    let e_dst_e = p.input("E_dstE", NIBBLE);
    let e_dst_m = p.input("E_dstM", NIBBLE);
    let e_cnd   = p.input("e_cnd", BIT);
    let m_icode = p.input("M_icode", NIBBLE);
    let m_stat  = p.input("m_stat", NIBBLE);
    let w_stat  = p.input("w_stat", NIBBLE);

    let f_stall_reg = p.reg("F_stall_reg", BIT);
    f_stall_reg.default_value(false);
    f_stall_reg.drive_next(p.mux((e_icode.eq(p.lit(IMRMOVQ, NIBBLE)) | e_icode.eq(p.lit(IPOPQ, NIBBLE))) & (e_dst_e.eq(d_src_a) | e_dst_e.eq(d_src_b) | 
                                  e_dst_m.eq(d_src_b) | e_dst_m.eq(d_src_a)) | (d_icode.eq(p.lit(IRET, NIBBLE)) | e_icode.eq(p.lit(IRET, NIBBLE)) | 
                                  m_icode.eq(p.lit(IRET, NIBBLE))), p.high(), p.low()));

    let d_stall_reg = p.reg("D_stall_reg", BIT);
    d_stall_reg.default_value(false);
    d_stall_reg.drive_next(p.mux((e_icode.eq(p.lit(IMRMOVQ, NIBBLE)) | e_icode.eq(p.lit(IPOPQ, NIBBLE))) & (e_dst_m.eq(d_src_a) | e_dst_m.eq(d_src_b) |
                                  e_dst_e.eq(d_src_b) | e_dst_e.eq(d_src_a)), p.high(), p.low()));

    let d_bubble_reg = p.reg("D_bubble_reg", BIT);
    d_bubble_reg.default_value(false);
    d_bubble_reg.drive_next(p.mux((e_icode.eq(p.lit(IJXX, NIBBLE)) & !e_cnd) | !((e_icode.eq(p.lit(IMRMOVQ, NIBBLE)) | e_icode.eq(p.lit(IPOPQ, NIBBLE))) &
                                   (e_dst_e.eq(d_src_a) | e_dst_e.eq(d_src_b) | e_dst_m.eq(d_src_a) | e_dst_m.eq(d_src_b))) & (d_icode.eq(p.lit(IRET, NIBBLE)) |
                                    e_icode.eq(p.lit(IRET, NIBBLE)) | m_icode.eq(p.lit(IRET, NIBBLE))), p.high(), p.low()));

    let e_bubble_reg = p.reg("E_bubble_reg", BIT);
    e_bubble_reg.default_value(false);
    e_bubble_reg.drive_next(p.mux((e_icode.eq(p.lit(IJXX, NIBBLE)) & !e_cnd) | (e_icode.eq(p.lit(IMRMOVQ, NIBBLE)) | e_icode.eq(p.lit(IPOPQ, NIBBLE))) &
                                   (e_dst_e.eq(d_src_a) | e_dst_e.eq(d_src_b) | e_dst_m.eq(d_src_b) | e_dst_m.eq(d_src_a)), p.high(), p.low()));

    let m_bubble_reg = p.reg("M_bubble_reg", BIT);
    m_bubble_reg.default_value(false);
    m_bubble_reg.drive_next(p.mux(m_stat.eq(p.lit(SADR, NIBBLE)) | m_stat.eq(p.lit(SINS, NIBBLE)) | m_stat.eq(p.lit(SHLT, NIBBLE)) |
                                  w_stat.eq(p.lit(SADR, NIBBLE)) | w_stat.eq(p.lit(SINS, NIBBLE)) | w_stat.eq(p.lit(SHLT, NIBBLE)), p.high(), p.low()));

    let w_stall_reg = p.reg("W_stall_reg", BIT);
    w_stall_reg.default_value(false);
    w_stall_reg.drive_next(p.mux(w_stat.eq(p.lit(SADR, NIBBLE)) | w_stat.eq(p.lit(SINS, NIBBLE)) | w_stat.eq(p.lit(SHLT, NIBBLE)), p.high(), p.low()));

    p.output("F_stall", f_stall_reg.value);
    p.output("D_stall", d_stall_reg.value);
    p.output("W_stall", w_stall_reg.value);
    p.output("D_bubble", d_bubble_reg.value);
    p.output("E_bubble", e_bubble_reg.value);
    p.output("M_bubble", m_bubble_reg.value);
    
    p
}