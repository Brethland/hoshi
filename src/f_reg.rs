use crate::y86_defines::*;
use kaze::*;

pub fn f_reg<'a>(c: &'a Context<'a>) -> &'a Module {
    let f = c.module("FRegister");

    let f_stall = f.input("F_stall", BIT);
    let f_val_c  = f.input("F_valC", QWORD);
    let f_val_p  = f.input("F_valP", QWORD);
    let f_icode = f.input("F_icode", NIBBLE);

    let predict_pc = f.mux((f.lit(IJXX, NIBBLE).eq(f_icode) | f.lit(ICALL, NIBBLE).eq(f_icode)) & f_val_c.lt(f_val_p),
          f_val_c, f_val_p);
    
    let pred_pc = f.reg("predPC", QWORD);
    pred_pc.drive_next(f.mux(f.lit(ENABLE, BIT).eq(f_stall), pred_pc.value, predict_pc));

    f.output("F_predPC", pred_pc.value);

    f
}
