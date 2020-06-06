use crate::y86_defines::*;
use kaze::*;

pub fn w_reg<'a>(c: &'a Context<'a>) -> &'a Module {
    let w = c.module("WRegister");

    let w_stall  = w.input("W_stall", BIT);
    let w_stat   = w.input("W_stat_i", NIBBLE);
    let w_icode  = w.input("W_icode_i", NIBBLE);
    let w_val_m  = w.input("W_valM_i", QWORD);
    let w_val_e  = w.input("W_valE_i", QWORD);
    let w_dst_e  = w.input("W_dstE_i", NIBBLE);
    let w_dst_m  = w.input("W_dstM_i", NIBBLE);

    let w_reg_stat = w.reg("W_reg_stat", NIBBLE);
    w_reg_stat.drive_next(w_stat);
    let w_reg_icode = w.reg("W_reg_icode", NIBBLE);
    w_reg_icode.drive_next(w.mux(w_stall.eq(w.lit(ENABLE, BIT)), w_reg_icode.value, w_icode));
    let w_reg_val_m = w.reg("W_reg_valM", QWORD);
    w_reg_val_m.drive_next(w.mux(w_stall.eq(w.lit(ENABLE, BIT)), w_reg_val_m.value, w_val_m));
    let w_reg_val_e = w.reg("W_reg_valE", QWORD);
    w_reg_val_e.drive_next(w.mux(w_stall.eq(w.lit(ENABLE, BIT)), w_reg_val_e.value, w_val_e));
    let w_reg_dst_e = w.reg("W_reg_dstE", NIBBLE);
    w_reg_dst_e.drive_next(w.mux(w_stall.eq(w.lit(ENABLE, BIT)), w_reg_dst_e.value, w_dst_e));
    let w_reg_dst_m = w.reg("W_reg_dstM", NIBBLE);
    w_reg_dst_m.drive_next(w.mux(w_stall.eq(w.lit(ENABLE, BIT)), w_reg_dst_m.value, w_dst_m));

    w.output("W_stat", w_reg_stat.value);
    w.output("W_icode", w_reg_icode.value);
    w.output("W_valM", w_reg_val_m.value);
    w.output("W_valE", w_reg_val_e.value);
    w.output("W_dstE", w_reg_dst_e.value);
    w.output("W_dstM", w_reg_dst_m.value);

    w
}