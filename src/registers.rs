use crate::y86_defines::*;
use kaze::*;

pub fn registers<'a>(c: &'a Context<'a>) -> &'a Module<'a> {
    let r = c.module("Registers");

    let src_a = r.input("srcA", NIBBLE);
    let src_b = r.input("srcB", NIBBLE);
    let dst_e = r.input("dstE", NIBBLE);
    let dst_m = r.input("dstM", NIBBLE);
    let val_e = r.input("valE", QWORD);
    let val_m = r.input("valM", QWORD);

    let val_none = r.lit(false, QWORD);

    let regs = [r.reg("rax", QWORD), r.reg("rcx", QWORD), r.reg("rdx", QWORD), r.reg("rbx", QWORD),
                r.reg("rsp", QWORD), r.reg("rbp", QWORD), r.reg("rsi", QWORD), r.reg("rdi", QWORD),
                r.reg("r8",  QWORD), r.reg("r9",  QWORD), r.reg("r10", QWORD), r.reg("r11", QWORD),
                r.reg("r12", QWORD), r.reg("r13", QWORD), r.reg("r14", QWORD)];

    for i in 0..15 {
        regs[i].default_value(false);
        regs[i].drive_next(r.mux(dst_m.eq(r.lit(i as u32, NIBBLE)), val_m, r.mux(dst_e.eq(r.lit(i as u32, NIBBLE)), val_e, regs[i].value)));
    }

    let mut val_a = val_none;
    let mut val_b = val_none;

    for k in 0..15 {
        val_a = r.mux(src_a.eq(r.lit(k as u32, NIBBLE)), regs[k].value, val_none);
        val_b = r.mux(src_b.eq(r.lit(k as u32, NIBBLE)), regs[k].value, val_none);
    }

    r.output("rvalA", val_a);
    r.output("rvalB", val_b);

    r
}