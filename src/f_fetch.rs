use crate::y86_defines::*;
use kaze::*;

pub fn f_fetch<'a>(c: &'a Context<'a>) -> &'a Module {
    let f = c.module("FFetch");

    let inst        = f.input("f_inst_mem", INST);
    let imem_error = f.input("f_inst_imem_error", BIT);
    let f_pc       = f.input("f_pc", QWORD);

    let icode = f.mux(imem_error, f.lit(INOP, NIBBLE), inst.bits(3, 0));
    let ifun  = f.mux(imem_error, f.lit(FNONE, NIBBLE), inst.bits(7, 4));
    let r_a   = inst.bits(11, 8);
    let r_b   = inst.bits(15, 12);
    let val_c = inst.bits(79, 16);

    let val_none = f.lit(false, QWORD);
    let r_none   = f.lit(RNONE, NIBBLE);
    let r_rsp    = f.lit(RRSP, NIBBLE);

    let mux_sel_1 = f.mux(icode.eq(f.lit(IHALT, NIBBLE)) | icode.eq(f.lit(INOP, NIBBLE)) | icode.eq(f.lit(IJXX, NIBBLE)), r_none, r_a);
    let mux_sel_r_a = f.mux(icode.eq(f.lit(ICALL, NIBBLE)) | icode.eq(f.lit(IRET, NIBBLE)) | icode.eq(f.lit(IPOPQ, NIBBLE)), r_rsp, mux_sel_1);

    let mux_sel_2 = f.mux(icode.eq(f.lit(IHALT, NIBBLE)) | icode.eq(f.lit(INOP, NIBBLE)) | icode.eq(f.lit(IJXX, NIBBLE)), r_none, r_b);
    let mux_sel_r_b = f.mux(icode.eq(f.lit(ICALL, NIBBLE)) | icode.eq(f.lit(IRET, NIBBLE)) | icode.eq(f.lit(IPOPQ, NIBBLE)) | icode.eq(f.lit(IPUSHQ, NIBBLE)), r_rsp, mux_sel_2);

    let mux_sel_3 = f.mux(icode.eq(f.lit(IIRMOVQ, NIBBLE)) | icode.eq(f.lit(IRMMOVQ, NIBBLE)) | icode.eq(f.lit(IMRMOVQ, NIBBLE)), val_c, val_none);
    let mux_sel_val_c = f.mux(icode.eq(f.lit(IJXX, NIBBLE)) | icode.eq(f.lit(ICALL, NIBBLE)), inst.bits(71, 8), mux_sel_3);

    let mux_sel_4 = f.mux(icode.eq(f.lit(IHALT, NIBBLE)) | icode.eq(f.lit(INOP, NIBBLE)) | icode.eq(f.lit(IRET, NIBBLE)), f_pc + f.lit(0x00000008u64, QWORD), f_pc + f.lit(0x00000010u64, QWORD));
    let mux_sel_5 = f.mux(icode.eq(f.lit(IIRMOVQ, NIBBLE)) | icode.eq(f.lit(IRRMOVQ, NIBBLE)) | icode.eq(f.lit(IMRMOVQ, NIBBLE)), f_pc + f.lit(0x00000110u64, QWORD), mux_sel_4);
    let mux_sel_val_p = f.mux(icode.eq(f.lit(IJXX, NIBBLE)) | icode.eq(f.lit(ICALL, NIBBLE)), f_pc + f.lit(0x00000108u64, QWORD), mux_sel_5);

    let f_inst_invalid = f.mux(icode.gt(f.lit(IPOPQ, NIBBLE)), f.high(), f.low());

    let stat_mux_1 = f.mux(f_inst_invalid.eq(f.high()), f.lit(SINS, NIBBLE), f.lit(SAOK, NIBBLE));
    let stat_mux_2 = f.mux(imem_error.eq(f.high()), f.lit(SADR, NIBBLE), stat_mux_1);

    f.output("f_stat", stat_mux_2);
    f.output("f_icode", icode);
    f.output("f_ifun", ifun);
    f.output("f_rA", mux_sel_r_a);
    f.output("f_rB", mux_sel_r_b);
    f.output("f_valC", mux_sel_val_c);
    f.output("f_valP", mux_sel_val_p);

    f
}