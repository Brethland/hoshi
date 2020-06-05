use crate::y86_defines::*;
use kaze::*;

pub fn pc_sel<'a>(c: &'a Context<'a>) -> &'a Module {
    let f = c.module("SelectPC");

    let pred_pc = f.input("f_predPC", QWORD);
    let m_icode = f.input("f_M_icode", NIBBLE);
    let m_val_a = f.input("f_M_valA", QWORD);
    let w_icode = f.input("f_W_icode", NIBBLE);
    let w_val_m = f.input("f_W_valM", QWORD);
    let cnd     = f.input("f_cnd", BIT);

    let mux_1 = f.mux(f.lit(IJXX, NIBBLE).eq(m_icode) & f.low().eq(cnd), m_val_a, pred_pc);
    let mux_2 = f.mux(f.lit(IRET, NIBBLE).eq(w_icode), w_val_m, mux_1);

    f.output("f_pc", mux_2);

    f
}