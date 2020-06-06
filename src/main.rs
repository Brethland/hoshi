use kaze::*;

use std::fs::File;
use std::io::Result;

pub mod y86_defines;
use y86_defines::*;
pub mod pipeline_control_logic;
use pipeline_control_logic::*;
pub mod registers;
use registers::*;

pub mod f_reg;
use f_reg::*;
pub mod f_fetch;
use f_fetch::*;
pub mod pc_sel;
use pc_sel::*;

pub mod d_reg;
use d_reg::*;
pub mod d_decode;
use d_decode::*;

pub mod e_reg;
use e_reg::*;
pub mod e_execute;
use e_execute::*;

pub mod m_reg;
use m_reg::*;
pub mod m_memory;
use m_memory::*;

pub mod w_reg;
use w_reg::*;

fn main() -> Result<()> {
    let y86_context = Context::new();

    pipeline(&y86_context); registers(&y86_context); f_reg(&y86_context); f_fetch(&y86_context);
    pc_sel(&y86_context); d_reg(&y86_context); sel_fwd_a(&y86_context); fwd_b(&y86_context);
    src_a(&y86_context); src_b(&y86_context); dst_e(&y86_context); dst_m(&y86_context);
    e_reg(&y86_context); alu_module(&y86_context); m_reg(&y86_context); addr(&y86_context);
    mem_control(&y86_context); m_stat(&y86_context); w_reg(&y86_context);

    let y86 = y86_context.module("Y86_CPU");

    let inst = y86.input("MemInst", INST);
    let imem = y86.input("ImemError", BIT);
    let dmem = y86.input("DmemError", BIT);
    let m_val_m = y86.input("m_valM", QWORD);

    let pipecon = y86.instance("PipeCon", "Pipeline");
    let regs    = y86.instance("Regs", "Registers");
    let freg    = y86.instance("FReg", "FRegister");
    let fetch   = y86.instance("Fetch", "FFetch");
    let selpc   = y86.instance("SelPC", "SelectPC");
    let dreg    = y86.instance("DReg", "DRegister");
    let sfa     = y86.instance("SFA", "SelFwdA");
    let fb      = y86.instance("FB", "FwdB");
    let srca    = y86.instance("DSrcA", "SrcA");
    let srcb    = y86.instance("DSrcB", "SrcB");
    let dste    = y86.instance("DDstE", "DstE");
    let dstm    = y86.instance("DDstM", "DstM");
    let ereg    = y86.instance("EReg", "ERegister");
    let alu     = y86.instance("AluModule", "Alu");
    let mreg    = y86.instance("MReg", "MRegister");
    let addre   = y86.instance("Address", "Addr");
    let memcon  = y86.instance("MemCon", "MemControl");
    let mstat   = y86.instance("MStatus", "MStat");
    let wreg    = y86.instance("WReg", "WRegister");

    pipecon.drive_input("D_icode", dreg.output("D_icode"));
    pipecon.drive_input("d_srcA", srca.output("d_srcA"));
    pipecon.drive_input("d_srcB", srcb.output("d_srcB"));
    pipecon.drive_input("E_icode", ereg.output("E_icode"));
    pipecon.drive_input("E_dstE", ereg.output("E_dstE"));
    pipecon.drive_input("E_dstM", ereg.output("E_dstM"));
    pipecon.drive_input("e_cnd", alu.output("e_cnd"));
    pipecon.drive_input("M_icode", mreg.output("M_icode"));
    pipecon.drive_input("m_stat", mstat.output("m_stat"));
    pipecon.drive_input("W_stat", wreg.output("W_stat"));

    regs.drive_input("srcA", srca.output("d_srcA"));
    regs.drive_input("srcB", srcb.output("d_srcB"));
    regs.drive_input("dstE", wreg.output("W_dstE"));
    regs.drive_input("dstM", wreg.output("W_dstM"));
    regs.drive_input("valE", wreg.output("W_valE"));
    regs.drive_input("valM", wreg.output("W_valM"));

    freg.drive_input("F_stall", pipecon.output("F_stall"));
    freg.drive_input("F_valC", fetch.output("f_valC"));
    freg.drive_input("F_valP", fetch.output("f_valP"));
    freg.drive_input("F_icode", fetch.output("f_icode"));

    fetch.drive_input("f_inst_mem", inst);
    fetch.drive_input("f_inst_imem_error", imem);
    fetch.drive_input("f_pc", selpc.output("f_pc"));

    selpc.drive_input("f_predPC", freg.output("F_predPC"));
    selpc.drive_input("f_M_icode", mreg.output("M_icode"));
    selpc.drive_input("f_M_valA", mreg.output("M_valA"));
    selpc.drive_input("f_W_icode", wreg.output("W_icode"));
    selpc.drive_input("f_W_valM", wreg.output("W_valM"));
    selpc.drive_input("f_cnd", mreg.output("M_cnd"));

    dreg.drive_input("D_stall_i", pipecon.output("D_stall"));
    dreg.drive_input("D_bubble_i", pipecon.output("D_bubble"));
    dreg.drive_input("D_stat_i", fetch.output("f_stat"));
    dreg.drive_input("D_icode_i", fetch.output("f_icode"));
    dreg.drive_input("D_ifun_i", fetch.output("f_ifun"));
    dreg.drive_input("D_rA_i", fetch.output("f_rA"));
    dreg.drive_input("D_rB_i", fetch.output("f_rB"));
    dreg.drive_input("D_valC_i", fetch.output("f_valC"));
    dreg.drive_input("D_valP_i", fetch.output("f_valP"));

    sfa.drive_input("D_icode", dreg.output("D_icode"));
    sfa.drive_input("D_valP", dreg.output("D_valP"));
    sfa.drive_input("d_srcA", srca.output("d_srcA"));
    sfa.drive_input("d_rvalA", regs.output("rvalA"));
    sfa.drive_input("e_dstE", alu.output("e_dstE"));
    sfa.drive_input("e_valE", alu.output("e_valE"));
    sfa.drive_input("M_dstE", mreg.output("M_dstE"));
    sfa.drive_input("M_valE", mreg.output("M_valE"));
    sfa.drive_input("M_dstM", mreg.output("M_dstM"));
    sfa.drive_input("m_valM", m_val_m);
    sfa.drive_input("W_dstE", wreg.output("W_dstE"));
    sfa.drive_input("W_valE", wreg.output("W_valE"));
    sfa.drive_input("W_dstM", wreg.output("W_dstM"));
    sfa.drive_input("W_valM", wreg.output("W_valM"));

    fb.drive_input("d_srcB", srcb.output("d_srcB"));
    fb.drive_input("d_rvalB", regs.output("rvalB"));
    fb.drive_input("e_dstE", alu.output("e_dstE"));
    fb.drive_input("e_valE", alu.output("e_valE"));
    fb.drive_input("M_dstE", mreg.output("M_dstE"));
    fb.drive_input("M_valE", mreg.output("M_valE"));
    fb.drive_input("M_dstM", mreg.output("M_dstM"));
    fb.drive_input("m_valM", m_val_m);
    fb.drive_input("W_dstE", wreg.output("W_dstE"));
    fb.drive_input("W_valE", wreg.output("W_valE"));
    fb.drive_input("W_dstM", wreg.output("W_dstM"));
    fb.drive_input("W_valM", wreg.output("W_valM"));

    srca.drive_input("D_icode", dreg.output("D_icode"));
    srca.drive_input("D_rA", dreg.output("D_rA"));

    srcb.drive_input("D_icode", dreg.output("D_icode"));
    srcb.drive_input("D_rB", dreg.output("D_rB"));

    dste.drive_input("D_icode", dreg.output("D_icode"));
    dste.drive_input("D_rB", dreg.output("D_rB"));

    dstm.drive_input("D_icode", dreg.output("D_icode"));
    dstm.drive_input("D_rA", dreg.output("D_rA"));

    ereg.drive_input("E_bubble_i", pipecon.output("E_bubble"));
    ereg.drive_input("E_stat_i", dreg.output("D_stat"));
    ereg.drive_input("E_icode_i", dreg.output("D_icode"));
    ereg.drive_input("E_ifun_i", dreg.output("D_ifun"));
    ereg.drive_input("E_valC_i", dreg.output("D_valC"));
    ereg.drive_input("E_valA_i", sfa.output("d_valA"));
    ereg.drive_input("E_valB_i", fb.output("d_valB"));
    ereg.drive_input("E_dstE_i", dste.output("d_dstE"));
    ereg.drive_input("E_dstM_i", dstm.output("d_dstM"));

    alu.drive_input("E_icode", ereg.output("E_icode"));
    alu.drive_input("E_ifun", ereg.output("E_ifun"));
    alu.drive_input("E_valA", ereg.output("E_valA"));
    alu.drive_input("E_valB", ereg.output("E_valB"));
    alu.drive_input("E_valC", ereg.output("E_valC"));
    alu.drive_input("m_stat", mstat.output("m_stat"));
    alu.drive_input("W_stat", wreg.output("W_stat"));
    alu.drive_input("E_dstE", ereg.output("E_dstE"));

    mreg.drive_input("M_bubble", pipecon.output("M_bubble"));
    mreg.drive_input("M_stat_i", ereg.output("E_stat"));
    mreg.drive_input("M_icode_i", ereg.output("E_icode"));
    mreg.drive_input("M_valE_i", alu.output("e_valE"));
    mreg.drive_input("M_valA_i", ereg.output("E_valA"));
    mreg.drive_input("M_dstE_i", alu.output("e_dstE"));
    mreg.drive_input("M_dstM_i", ereg.output("E_dstM"));
    mreg.drive_input("M_cnd_i", alu.output("e_cnd"));

    addre.drive_input("M_icode", mreg.output("M_icode"));
    addre.drive_input("M_valA", mreg.output("M_valA"));
    addre.drive_input("M_valE", mreg.output("M_valE"));

    memcon.drive_input("M_icode", mreg.output("M_icode"));

    mstat.drive_input("dmem_error", dmem);
    mstat.drive_input("M_stat", mreg.output("M_stat"));

    wreg.drive_input("W_stall", pipecon.output("W_stall"));
    wreg.drive_input("W_stat_i", mstat.output("m_stat"));
    wreg.drive_input("W_icode_i", mreg.output("M_icode"));
    wreg.drive_input("W_valE_i", mreg.output("M_valE"));
    wreg.drive_input("W_valM_i", m_val_m);
    wreg.drive_input("W_dstE_i", mreg.output("M_dstE"));
    wreg.drive_input("W_dstM_i", mreg.output("M_dstM"));

    y86.output("f_pc", selpc.output("f_pc"));
    y86.output("m_addr", addre.output("m_addr"));
    y86.output("m_read", memcon.output("m_read"));
    y86.output("m_write", memcon.output("m_write"));
    y86.output("m_data", mreg.output("M_valA"));

    let file = File::create("verilog.v").unwrap();

    verilog::generate(y86, &file)?;

    Ok(())
}
