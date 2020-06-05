use crate::y86_defines::*;
use kaze::*;

pub fn alu_module<'a>(c: &'a Context<'a>) -> &'a Module<'a> {
    let alu = c.module("Alu");

    let e_icode = alu.input("E_icode", NIBBLE);
    let e_ifun  = alu.input("E_ifun", NIBBLE);
    let e_val_a = alu.input("E_valA", QWORD);
    let e_val_b = alu.input("E_valB", QWORD);
    let e_val_c = alu.input("E_valC", QWORD);
    let m_stat  = alu.input("m_stat", NIBBLE);
    let w_stat  = alu.input("W_stat", NIBBLE);
    let e_dst_e = alu.input("E_dstE", NIBBLE);

    let val_none = alu.lit(false, QWORD);

    let e_ifun_appended = alu.mux(e_icode.eq(alu.lit(ICALL, NIBBLE)) | e_icode.eq(alu.lit(IPUSHQ, NIBBLE)), alu.lit(FSUBQ, NIBBLE), e_ifun);

    let alu_a_1 = alu.mux(e_icode.eq(alu.lit(IRRMOVQ, NIBBLE)) | e_icode.eq(alu.lit(IOPQ, NIBBLE)), e_val_a, val_none);
    let alu_a_2 = alu.mux(e_icode.eq(alu.lit(IIRMOVQ, NIBBLE)) | e_icode.eq(alu.lit(IRMMOVQ, NIBBLE)) | e_icode.eq(alu.lit(IMRMOVQ, NIBBLE)), e_val_c, alu_a_1);
    let alu_a   = alu.mux(e_icode.eq(alu.lit(ICALL, NIBBLE)) | e_icode.eq(alu.lit(IPUSHQ, NIBBLE)) | e_icode.eq(alu.lit(IRET, NIBBLE)) | 
                          e_icode.eq(alu.lit(IPOPQ, NIBBLE)), alu.lit(8u64, QWORD), alu_a_2);

    let alu_b   = alu.mux(e_icode.eq(alu.lit(IRMMOVQ, NIBBLE)) | e_icode.eq(alu.lit(IMRMOVQ, NIBBLE)) | e_icode.eq(alu.lit(IOPQ, NIBBLE)) | 
                          e_icode.eq(alu.lit(ICALL, NIBBLE)) | e_icode.eq(alu.lit(IPUSHQ, NIBBLE)) | e_icode.eq(alu.lit(IRET, NIBBLE)) | 
                          e_icode.eq(alu.lit(IPOPQ, NIBBLE)), e_val_b, val_none);

    let alu_fun = alu.mux(e_icode.eq(alu.lit(IOPQ, NIBBLE)) | e_icode.eq(alu.lit(ICALL, NIBBLE)) | e_icode.eq(alu.lit(IPUSHQ, NIBBLE)), e_ifun_appended, alu.lit(ALUADD, NIBBLE));

    let execute_a = alu.mux(alu_fun.eq(alu.lit(FSUBQ, NIBBLE)), alu_b - alu_a, alu_b + alu_a);
    let execute_b = alu.mux(alu_fun.eq(alu.lit(FADDQ, NIBBLE)), alu_a & alu_b, execute_a);
    let execute   = alu.mux(alu_fun.eq(alu.lit(FXORQ, NIBBLE)), alu_a ^ alu_b, execute_b);
    
    let set_cc = alu.mux(e_icode.eq(alu.lit(IOPQ, NIBBLE)) & m_stat.eq(alu.lit(SAOK, NIBBLE)) & w_stat.eq(alu.lit(SAOK, NIBBLE)), alu.high(), alu.low());

    let zf = alu.reg("ZF", BIT);
    zf.drive_next(alu.mux(set_cc.eq(alu.high()), execute.eq(val_none), zf.value));
    let sf = alu.reg("SF", BIT);
    sf.drive_next(alu.mux(set_cc.eq(alu.high()), execute.lt_signed(val_none), sf.value));
    let of = alu.reg("OF", BIT);
    of.drive_next(alu.mux(set_cc.eq(alu.high()), alu_a.lt_signed(val_none).eq(alu_b.lt_signed(val_none)) 
                  & execute.lt_signed(val_none).ne(alu_a.lt_signed(val_none)), of.value));

    let cnd_cmovle = alu.mux(e_icode.eq(alu.lit(IRRMOVQ, NIBBLE)) & e_ifun.eq(alu.lit(FCMOVLE, NIBBLE)), (sf.value ^ of.value) | zf.value, alu.high());
    let cnd_cmovl = alu.mux(e_icode.eq(alu.lit(IRRMOVQ, NIBBLE)) & e_ifun.eq(alu.lit(FCMOVL, NIBBLE)), sf.value ^ of.value, cnd_cmovle);
    let cnd_cmove = alu.mux(e_icode.eq(alu.lit(IRRMOVQ, NIBBLE)) & e_ifun.eq(alu.lit(FCMOVE, NIBBLE)), zf.value, cnd_cmovl);
    let cnd_cmovne = alu.mux(e_icode.eq(alu.lit(IRRMOVQ, NIBBLE)) & e_ifun.eq(alu.lit(FCMOVNE, NIBBLE)), !zf.value, cnd_cmove);
    let cnd_cmovge = alu.mux(e_icode.eq(alu.lit(IRRMOVQ, NIBBLE)) & e_ifun.eq(alu.lit(FCMOVGE, NIBBLE)), !(sf.value ^ of.value), cnd_cmovne);
    let cnd_cmovg = alu.mux(e_icode.eq(alu.lit(IRRMOVQ, NIBBLE)) & e_ifun.eq(alu.lit(FCMOVG, NIBBLE)), !(sf.value ^ of.value) & !zf.value, cnd_cmovge);

    let cnd_jle = alu.mux(e_icode.eq(alu.lit(IJXX, NIBBLE)) & e_ifun.eq(alu.lit(FJLE, NIBBLE)), (sf.value ^ of.value) | zf.value, cnd_cmovg);
    let cnd_jl = alu.mux(e_icode.eq(alu.lit(IJXX, NIBBLE)) & e_ifun.eq(alu.lit(FJL, NIBBLE)), sf.value ^ of.value, cnd_jle);
    let cnd_je = alu.mux(e_icode.eq(alu.lit(IJXX, NIBBLE)) & e_ifun.eq(alu.lit(FJE, NIBBLE)), zf.value, cnd_jl);
    let cnd_jne = alu.mux(e_icode.eq(alu.lit(IJXX, NIBBLE)) & e_ifun.eq(alu.lit(FJNE, NIBBLE)), !zf.value, cnd_je);
    let cnd_jge = alu.mux(e_icode.eq(alu.lit(IJXX, NIBBLE)) & e_ifun.eq(alu.lit(FJGE, NIBBLE)), !(sf.value ^ of.value), cnd_jne);
    let cnd = alu.mux(e_icode.eq(alu.lit(IJXX, NIBBLE)) & e_ifun.eq(alu.lit(FJG, NIBBLE)), !(sf.value ^ of.value) & !zf.value, cnd_jge);

    let dst_e = alu.mux(e_icode.eq(alu.lit(IRRMOVQ, NIBBLE)) & cnd.eq(alu.low()), alu.lit(RNONE, NIBBLE), e_dst_e);

    alu.output("e_valE", execute);
    alu.output("e_dstE", dst_e);
    alu.output("e_cnd", cnd);

    alu
}