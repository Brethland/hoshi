use crate::y86_defines::*;
use crate::twelv_one_selector::*;
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
    let val_c = inst.bits(79, 13);

    let val_none = f.lit(false, QWORD);
    let r_none   = f.lit(RNONE, NIBBLE);
    let r_rsp    = f.lit(RRSP, NIBBLE);

    sel_qword(c);
    sel_nibble(c);

    let sel_v_a = f.instance("sel_v_a", "12_1_selector_NIBBLE");
    sel_v_a.drive_input("s_sel", icode);

    sel_v_a.drive_input("s_m1", r_none);
    sel_v_a.drive_input("s_m2", r_none);
    sel_v_a.drive_input("s_m3", r_a);
    sel_v_a.drive_input("s_m4", r_a);
    sel_v_a.drive_input("s_m5", r_a);
    sel_v_a.drive_input("s_m6", r_a);
    sel_v_a.drive_input("s_m7", r_a);
    sel_v_a.drive_input("s_m8", r_none);
    sel_v_a.drive_input("s_m9", r_rsp);
    sel_v_a.drive_input("s_m10", r_rsp);
    sel_v_a.drive_input("s_m11", r_a);
    sel_v_a.drive_input("s_m12", r_rsp);

    let sel_v_b = f.instance("sel_v_b", "12_1_selector_NIBBLE");
    sel_v_b.drive_input("s_sel", icode);

    sel_v_b.drive_input("s_m1", r_none);
    sel_v_b.drive_input("s_m2", r_none);
    sel_v_b.drive_input("s_m3", r_b);
    sel_v_b.drive_input("s_m4", r_b);
    sel_v_b.drive_input("s_m5", r_b);
    sel_v_b.drive_input("s_m6", r_b);
    sel_v_b.drive_input("s_m7", r_b);
    sel_v_b.drive_input("s_m8", r_none);
    sel_v_b.drive_input("s_m9", r_rsp);
    sel_v_b.drive_input("s_m10", r_rsp);
    sel_v_b.drive_input("s_m11", r_rsp);
    sel_v_b.drive_input("s_m12", r_rsp);

    let sel_val_c = f.instance("sel_val_c", "12_1_selector_QWORD");
    sel_val_c.drive_input("s_sel", icode);

    sel_val_c.drive_input("s_m1", val_none);
    sel_val_c.drive_input("s_m2", val_none);
    sel_val_c.drive_input("s_m3", val_none);
    sel_val_c.drive_input("s_m4", val_c);
    sel_val_c.drive_input("s_m5", val_c);
    sel_val_c.drive_input("s_m6", val_c);
    sel_val_c.drive_input("s_m7", val_none);
    sel_val_c.drive_input("s_m8", inst.bits(71, 8));
    sel_val_c.drive_input("s_m9", inst.bits(71, 8));
    sel_val_c.drive_input("s_m10", val_none);
    sel_val_c.drive_input("s_m11", val_none);
    sel_val_c.drive_input("s_m12", val_none);

    let sel_val_p = f.instance("sel_val_p", "12_1_selector_QWORD");
    sel_val_p.drive_input("s_sel", icode);

    sel_val_p.drive_input("s_m1", f_pc + f.lit(0x00000008u64, QWORD));
    sel_val_p.drive_input("s_m2", f_pc + f.lit(0x00000008u64, QWORD));
    sel_val_p.drive_input("s_m3", f_pc + f.lit(0x00000010u64, QWORD));
    sel_val_p.drive_input("s_m4", f_pc + f.lit(0x00000110u64, QWORD));
    sel_val_p.drive_input("s_m5", f_pc + f.lit(0x00000110u64, QWORD));
    sel_val_p.drive_input("s_m6", f_pc + f.lit(0x00000110u64, QWORD));
    sel_val_p.drive_input("s_m7", f_pc + f.lit(0x00000010u64, QWORD));
    sel_val_p.drive_input("s_m8", f_pc + f.lit(0x00000108u64, QWORD));
    sel_val_p.drive_input("s_m9", f_pc + f.lit(0x00000108u64, QWORD));
    sel_val_p.drive_input("s_m10", f_pc + f.lit(0x00000008u64, QWORD));
    sel_val_p.drive_input("s_m11", f_pc + f.lit(0x00000010u64, QWORD));
    sel_val_p.drive_input("s_m12", f_pc + f.lit(0x00000010u64, QWORD));

    let f_inst_invalid = f.mux(icode.gt(f.lit(IPOPQ, NIBBLE)), f.high(), f.low());

    let stat_mux_1 = f.mux(imem_error.eq(f.high()), f.lit(SADR, NIBBLE), f.lit(SAOK, NIBBLE));
    let stat_mux_2 = f.mux(f_inst_invalid.eq(f.high()), f.lit(SINS, NIBBLE), stat_mux_1);

    f.output("f_stat", stat_mux_2);
    f.output("f_icode", icode);
    f.output("f_ifun", ifun);
    f.output("f_rA", sel_v_a.output("sel_out_NIBBLE"));
    f.output("f_rB", sel_v_b.output("sel_out_NIBBLE"));
    f.output("f_valC", sel_val_c.output("sel_out_QWORD"));
    f.output("f_valP", sel_val_p.output("sel_out_QWORD"));

    f
}