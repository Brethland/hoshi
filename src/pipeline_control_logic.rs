use crate::y86_defines::*;
use kaze::*;


pub fn pipeline<'a>(c: &'a Context<'a>) -> &'a Module {
    let p = c.module("Pipeline");

    let d_icode = p.input("D_icode", NIBBLE);
    let d_src_a = p.input("d_srcA", NIBBLE);
    let d_src_b = p.input("d_srcB", NIBBLE);
    let e_icode = p.input("E_icode", NIBBLE);
    let e_dst_e = p.input("E_dstE", NIBBLE);
    let d_dst_m = p.input("E_dstM", NIBBLE);
    let e_cnd   = p.input("e_cnd", BIT);
    let m_icode = p.input("M_icode", NIBBLE);
    let m_stat  = p.input("m_stat", NIBBLE);
    let w_stat  = p.input("w_stat", NIBBLE);
    
    p
}