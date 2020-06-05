use crate::y86_defines::*;
use kaze::*;

pub fn alu_module<'a>(c: &'a Context<'a>) -> &'a Module<'a> {
    let alu = c.module("Alu");

    let e_icode = alu.input("E_icode", NIBBLE);
    let e_ifun  = alu.input("E_ifun", NIBBLE);
    let e_val_a = alu.input("E_valA", QWORD);
    let e_val_b = alu.input("E_valB", QWORD);
    let e_val_c = alu.input("E_valC", QWORD);

    let e_ifun_appended = alu.mux(e_icode.eq(alu.lit(ICALL, NIBBLE)) | e_icode.eq(alu.lit(IPUSHQ, NIBBLE)), alu.lit(FSUBQ, NIBBLE), e_ifun);

    alu
}