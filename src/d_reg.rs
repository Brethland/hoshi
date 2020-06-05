use crate::y86_defines::*;
use kaze::*;

pub fn d_reg<'a>(c: &'a Context<'a>) -> &'a Module {
    let d = c.module("DRegister");

    let d_stall  = d.input("D_stall_i", BIT);
    let d_bubble = d.input("D_bubble_i", BIT);
    let d_stat   = d.input("D_stat_i", NIBBLE);
    let d_icode  = d.input("D_icode_i", NIBBLE);
    let d_ifun   = d.input("D_ifun_i", NIBBLE);
    let d_r_a    = d.input("D_rA_i", NIBBLE);
    let d_r_b    = d.input("D_rB_i", NIBBLE);
    let d_val_c  = d.input("D_valC_i", QWORD);
    let d_val_p  = d.input("D_valP_i", QWORD);

    let d_reg_stat = d.reg("D_reg_stat", NIBBLE);
    d_reg_stat.drive_next(d_stat);

    let d_reg_icode = d.reg("D_reg_icode", NIBBLE);
    d_reg_icode.drive_next(d.mux(d_stall.eq(d.lit(ENABLE, BIT)), d_reg_icode.value, 
                                d.mux(d_bubble.eq(d.lit(ENABLE, BIT)), d.lit(INOP, NIBBLE), d_icode)));
    let d_reg_ifun = d.reg("D_reg_ifun", NIBBLE);
    d_reg_ifun.drive_next(d.mux(d_stall.eq(d.lit(ENABLE, BIT)), d_reg_ifun.value, 
                               d.mux(d_bubble.eq(d.lit(ENABLE, BIT)), d.lit(FNONE, NIBBLE), d_ifun)));

    let d_reg_r_a = d.reg("D_reg_rA", NIBBLE);
    d_reg_r_a.drive_next(d.mux(d_stall.eq(d.lit(ENABLE, BIT)), d_reg_r_a.value,
                              d.mux(d_bubble.eq(d.lit(ENABLE, BIT)), d.lit(RNONE, NIBBLE), d_r_a)));
    let d_reg_r_b = d.reg("D_reg_rB", NIBBLE);
    d_reg_r_b.drive_next(d.mux(d_stall.eq(d.lit(ENABLE, BIT)), d_reg_r_b.value,
                              d.mux(d_bubble.eq(d.lit(ENABLE, BIT)), d.lit(RNONE, NIBBLE), d_r_b)));

    let d_reg_val_c = d.reg("D_reg_valC", QWORD);
    d_reg_val_c.drive_next(d.mux(d_stall.eq(d.lit(ENABLE, BIT)), d_reg_val_c.value,
                              d.mux(d_bubble.eq(d.lit(ENABLE, BIT)), d.lit(false, QWORD), d_val_c)));
    let d_reg_val_p = d.reg("D_reg_valP", QWORD);
    d_reg_val_p.drive_next(d.mux(d_stall.eq(d.lit(ENABLE, BIT)), d_reg_val_p.value,
                              d.mux(d_bubble.eq(d.lit(ENABLE, BIT)), d.lit(false, QWORD), d_val_p)));

    d.output("D_stat", d_reg_stat.value);
    d.output("D_icode", d_reg_icode.value);
    d.output("D_ifun", d_reg_ifun.value);
    d.output("D_rA", d_reg_r_a.value);
    d.output("D_rB", d_reg_r_b.value);
    d.output("D_valC", d_reg_val_c.value);
    d.output("D_valP", d_reg_val_p.value);

    d
}