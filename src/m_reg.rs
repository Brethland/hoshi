use crate::y86_defines::*;
use kaze::*;

pub fn m_reg<'a>(c: &'a Context<'a>) -> &'a Module {
    let m = c.module("MRegister");

    let m_stat   = m.input("M_stat_i", NIBBLE);
    let m_icode  = m.input("M_icode_i", NIBBLE);
    let m_val_a  = m.input("M_valA_i", QWORD);
    let m_val_e  = m.input("M_valE_i", QWORD);
    let m_dst_e  = m.input("M_dstE_i", NIBBLE);
    let m_dst_m  = m.input("M_dstM_i", NIBBLE);
    let m_cnd    = m.input("M_cnd_i", BIT);

    let m_reg_stat = m.reg("M_reg_stat", NIBBLE);
    m_reg_stat.drive_next(m_stat);
    let m_reg_icode = m.reg("M_reg_icode", NIBBLE);
    m_reg_icode.drive_next(m_icode);
    let m_reg_val_a = m.reg("M_reg_valA", QWORD);
    m_reg_val_a.drive_next(m_val_a);
    let m_reg_val_e = m.reg("M_reg_valE", QWORD);
    m_reg_val_e.drive_next(m_val_e);
    let m_reg_dst_e = m.reg("M_reg_dstE", NIBBLE);
    m_reg_dst_e.drive_next(m_dst_e);
    let m_reg_dst_m = m.reg("M_reg_dstM", NIBBLE);
    m_reg_dst_m.drive_next(m_dst_m);
    let m_reg_cnd = m.reg("M_reg_cnd", BIT);
    m_reg_cnd.drive_next(m_cnd);

    m.output("M_stat", m_reg_stat.value);
    m.output("M_icode", m_reg_icode.value);
    m.output("M_valA", m_reg_val_a.value);
    m.output("M_valE", m_reg_val_e.value);
    m.output("M_dstE", m_reg_dst_e.value);
    m.output("M_dstM", m_reg_dst_m.value);
    m.output("M_cnd", m_reg_cnd.value);

    m
}