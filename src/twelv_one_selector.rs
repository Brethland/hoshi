use crate::y86_defines::*;
use kaze::*;

pub fn sel_qword<'a>(c: &'a Context<'a>) {
    let f = c.module("12_1_selector_QWORD");

    let sel = f.input("s_sel", NIBBLE);

    let m1  = f.input("s_m1", QWORD);
    let m2  = f.input("s_m2", QWORD);
    let m3  = f.input("s_m3", QWORD);
    let m4  = f.input("s_m4", QWORD);
    let m5  = f.input("s_m5", QWORD);
    let m6  = f.input("s_m6", QWORD);
    let m7  = f.input("s_m7", QWORD);
    let m8  = f.input("s_m8", QWORD);
    let m9  = f.input("s_m9", QWORD);
    
    let m10  = f.input("s_m10", QWORD);
    let m11  = f.input("s_m11", QWORD);
    let m12  = f.input("s_m12", QWORD);

    let mux_1 = f.mux(f.lit(IHALT, NIBBLE).eq(sel), m1, f.lit(false, QWORD));
    let mux_2 = f.mux(f.lit(INOP, NIBBLE).eq(sel), m2, mux_1);
    let mux_3 = f.mux(f.lit(IRRMOVQ, NIBBLE).eq(sel), m3, mux_2);
    let mux_4 = f.mux(f.lit(IIRMOVQ, NIBBLE).eq(sel), m4, mux_3);
    let mux_5 = f.mux(f.lit(IRMMOVQ, NIBBLE).eq(sel), m5, mux_4);
    let mux_6 = f.mux(f.lit(IMRMOVQ, NIBBLE).eq(sel), m6, mux_5);
    let mux_7 = f.mux(f.lit(IOPQ, NIBBLE).eq(sel), m7, mux_6);
    let mux_8 = f.mux(f.lit(IJXX, NIBBLE).eq(sel), m8, mux_7);
    let mux_9 = f.mux(f.lit(ICALL, NIBBLE).eq(sel), m9, mux_8);

    let mux_10 = f.mux(f.lit(IRET, NIBBLE).eq(sel), m10, mux_9);
    let mux_11 = f.mux(f.lit(IPUSHQ, NIBBLE).eq(sel), m11, mux_10);
    let mux_12 = f.mux(f.lit(IPOPQ, NIBBLE).eq(sel), m12, mux_11);

    f.output("sel_out_QWORD", mux_12);
}

pub fn sel_nibble<'a>(c: &'a Context<'a>) {
    let f = c.module("12_1_selector_NIBBLE");

    let sel = f.input("s_sel", NIBBLE);

    let m1  = f.input("s_m1", NIBBLE);
    let m2  = f.input("s_m2", NIBBLE);
    let m3  = f.input("s_m3", NIBBLE);
    let m4  = f.input("s_m4", NIBBLE);
    let m5  = f.input("s_m5", NIBBLE);
    let m6  = f.input("s_m6", NIBBLE);
    let m7  = f.input("s_m7", NIBBLE);
    let m8  = f.input("s_m8", NIBBLE);
    let m9  = f.input("s_m9", NIBBLE);
    
    let m10  = f.input("s_m10", NIBBLE);
    let m11  = f.input("s_m11", NIBBLE);
    let m12  = f.input("s_m12", NIBBLE);

    let mux_1 = f.mux(f.lit(IHALT, NIBBLE).eq(sel), m1, f.lit(RNONE, NIBBLE));
    let mux_2 = f.mux(f.lit(INOP, NIBBLE).eq(sel), m2, mux_1);
    let mux_3 = f.mux(f.lit(IRRMOVQ, NIBBLE).eq(sel), m3, mux_2);
    let mux_4 = f.mux(f.lit(IIRMOVQ, NIBBLE).eq(sel), m4, mux_3);
    let mux_5 = f.mux(f.lit(IRMMOVQ, NIBBLE).eq(sel), m5, mux_4);
    let mux_6 = f.mux(f.lit(IMRMOVQ, NIBBLE).eq(sel), m6, mux_5);
    let mux_7 = f.mux(f.lit(IOPQ, NIBBLE).eq(sel), m7, mux_6);
    let mux_8 = f.mux(f.lit(IJXX, NIBBLE).eq(sel), m8, mux_7);
    let mux_9 = f.mux(f.lit(ICALL, NIBBLE).eq(sel), m9, mux_8);

    let mux_10 = f.mux(f.lit(IRET, NIBBLE).eq(sel), m10, mux_9);
    let mux_11 = f.mux(f.lit(IPUSHQ, NIBBLE).eq(sel), m11, mux_10);
    let mux_12 = f.mux(f.lit(IPOPQ, NIBBLE).eq(sel), m12, mux_11);

    f.output("sel_out_NIBBLE", mux_12);
}