use crate::y86_defines::*;
use kaze::*;

pub fn e_reg<'a>(c: &'a Context<'a>) -> &'a Module {
    let e = c.module("ERegister");

    let e_bubble = e.input("E_bubble_i", BIT);
    let e_stat   = e.input("E_stat_i", NIBBLE);
    let e_icode  = e.input("E_icode_i", NIBBLE);
    let e_ifun   = e.input("E_ifun_i", NIBBLE);
    let e_val_c  = e.input("E_valC_i", QWORD);
    let e_val_a  = e.input("E_valA_i", QWORD);
    let e_val_b  = e.input("E_valB_i", QWORD);
    let e_dst_e  = e.input("E_dstE_i", NIBBLE);
    let e_dst_m  = e.input("E_dstM_i", NIBBLE);

    let e_reg_stat = e.reg("E_reg_stat", NIBBLE);
    e_reg_stat.drive_next(e_stat);

    let e_reg_icode = e.reg("E_reg_icode", NIBBLE);
    e_reg_icode.drive_next(e.mux(e_bubble.eq(e.lit(ENABLE, BIT)), e.lit(INOP, NIBBLE), e_icode));
    let e_reg_ifun = e.reg("E_reg_ifun", NIBBLE);
    e_reg_ifun.drive_next(e.mux(e_bubble.eq(e.lit(ENABLE, BIT)), e.lit(FNONE, NIBBLE), e_ifun));

    let e_reg_val_c = e.reg("E_reg_valC", QWORD);
    e_reg_val_c.drive_next(e.mux(e_bubble.eq(e.lit(ENABLE, BIT)), e.lit(false, QWORD), e_val_c));
    let e_reg_val_a = e.reg("E_reg_valA", QWORD);
    e_reg_val_a.drive_next(e.mux(e_bubble.eq(e.lit(ENABLE, BIT)), e.lit(false, QWORD), e_val_a));
    let e_reg_val_b = e.reg("E_reg_valB", QWORD);
    e_reg_val_b.drive_next(e.mux(e_bubble.eq(e.lit(ENABLE, BIT)), e.lit(false, QWORD), e_val_b));

    let e_reg_dst_e = e.reg("E_reg_dstE", NIBBLE);
    e_reg_dst_e.drive_next(e.mux(e_bubble.eq(e.lit(ENABLE, BIT)), e.lit(RNONE, NIBBLE), e_dst_e));
    let e_reg_dst_m = e.reg("E_reg_dstM", NIBBLE);
    e_reg_dst_m.drive_next(e.mux(e_bubble.eq(e.lit(ENABLE, BIT)), e.lit(RNONE, NIBBLE), e_dst_m));

    e.output("E_stat", e_reg_stat.value);
    e.output("E_icode", e_reg_icode.value);
    e.output("E_ifun", e_reg_ifun.value);
    e.output("E_valC", e_reg_val_c.value);
    e.output("E_valA", e_reg_val_a.value);
    e.output("E_valB", e_reg_val_b.value);
    e.output("E_dstE", e_reg_dst_e.value);
    e.output("E_dstM", e_reg_dst_m.value);

    e
}