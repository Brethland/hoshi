use crate::y86_defines::*;
use kaze::*;

pub fn addr<'a>(c: &'a Context<'a>) -> &'a Module {
    let m = c.module("Addr");

    let m_icode = m.input("M_icode", NIBBLE);
    let m_val_a = m.input("M_valA", QWORD);
    let m_val_e = m.input("M_valE", QWORD);

    let val_none = m.lit(false, QWORD);

    let mux_1 = m.mux(m_icode.eq(m.lit(IPOPQ, NIBBLE)) | m_icode.eq(m.lit(IRET, NIBBLE)), m_val_a, val_none);

    m.output("m_addr", m.mux(m_icode.eq(m.lit(IRMMOVQ, NIBBLE)) | m_icode.eq(m.lit(IPUSHQ, NIBBLE)) | m_icode.eq(m.lit(ICALL, NIBBLE)) | 
             m_icode.eq(m.lit(IMRMOVQ, NIBBLE)), m_val_e, mux_1));

    m
}

pub fn mem_control<'a>(c: &'a Context<'a>) -> &'a Module {
    let m = c.module("MemControl");

    let m_icode = m.input("M_icode", NIBBLE);

    m.output("m_read", m.mux(m_icode.eq(m.lit(IMRMOVQ, NIBBLE)) | m_icode.eq(m.lit(IPOPQ, NIBBLE)) | m_icode.eq(m.lit(IRET, NIBBLE)), m.high(), m.low()));
    m.output("m_write", m.mux(m_icode.eq(m.lit(IRMMOVQ, NIBBLE)) | m_icode.eq(m.lit(IPUSHQ, NIBBLE)) | m_icode.eq(m.lit(ICALL, NIBBLE)), m.high(), m.low()));

    m
}

pub fn m_stat<'a>(c: &'a Context<'a>) -> &'a Module {
    let m = c.module("MStat");

    let d_mem_error   = m.input("dmem_error", BIT);
    let m_stat_i      = m.input("M_stat", NIBBLE);

    m.output("m_stat", m.mux(d_mem_error.eq(m.high()), m.lit(SADR, NIBBLE), m_stat_i));

    m
}