use std::collections::HashMap;

use lazy_static::lazy_static;
use strum::Display;
use strum_macros::EnumString;

#[allow(non_camel_case_types)]
#[derive(EnumString, Display, Debug, Clone, Copy)]
pub enum InstrMnem {
    addx,
    addcx,
    addex,
    addi,
    addic,
    #[strum(serialize = "addic.")]
    addic_,
    addis,
    addmex,
    addzex,
    divwx,
    divwux,
    mulhwx,
    mulhwux,
    mulli,
    mullwx,
    negx,
    subfx,
    subfcx,
    subfex,
    subficx,
    subfmex,
    subfzex,
    cmp,
    cmpi,
    cmpl,
    cmpli,
    andx,
    andcx,
    #[strum(serialize = "andi.")]
    andi_,
    #[strum(serialize = "andis.")]
    andis_,
    cntlzwx,
    eqvx,
    extsbx,
    extshx,
    nandx,
    norx,
    orx,
    orcx,
    ori,
    oris,
    xorx,
    xori,
    xoris,
    rlwimix,
    rlwinmx,
    rlwnmx,
    slwx,
    srawx,
    srawix,
    srwx,
    faddx,
    faddsx,
    fdivx,
    fdivsx,
    fmulx,
    fmulsx,
    fresx,
    frsqrtex,
    fsubx,
    fsubsx,
    fselx,
    fmaddx,
    fmaddsx,
    fmsubx,
    fmsubsx,
    fnmaddx,
    fnmaddsx,
    fnmsubx,
    fnmsubsx,
    fctiwx,
    fctiwzx,
    frspx,
    fcmpo,
    fcmpu,
    mcrfs,
    mffsx,
    mtfsb0x,
    mtfsb1x,
    mtfsfx,
    mtfsfix,
    lbz,
    lbzu,
    lbzux,
    lbzx,
    lha,
    lhau,
    lhaux,
    lhax,
    lhz,
    lhzu,
    lhzux,
    lhzx,
    lwz,
    lwzu,
    lwzux,
    lwzx,
    stb,
    stbu,
    stbux,
    stbx,
    sth,
    sthu,
    sthux,
    sthx,
    stw,
    stwu,
    stwux,
    stwx,
    lhbrx,
    lwbrx,
    sthbrx,
    stwbrx,
    lmw,
    stmw,
    lswi,
    lswx,
    stswi,
    stswx,
    eieio,
    isync,
    lwarx,
    #[strum(serialize = "stwcx.")]
    stwcx_,
    sync,
    lfd,
    lfdu,
    lfdux,
    lfdx,
    lfs,
    lfsu,
    lfsux,
    lfsx,
    stfd,
    stfdu,
    stfdux,
    stfdx,
    stfiwx,
    stfs,
    stfsu,
    stfsux,
    stfsx,
    fabsx,
    fmrx,
    fnabsx,
    fnegx,
    bx,
    bcx,
    bcctrx,
    bclrx,
    crand,
    crandc,
    creqv,
    crnand,
    crnor,
    cror,
    crorc,
    crxor,
    mcrf,
    rfi,
    sc,
    tw,
    twi,
    mcrxr,
    mfcr,
    mfmsr,
    mfspr,
    mftb,
    mtcrf,
    mtmsr,
    mtspr,
    dcbf,
    dcbi,
    dcbst,
    dcbt,
    dcbtst,
    dcbz,
    icbi,
    mfsr,
    mfsrin,
    mtsr,
    mtsrin,
    tlbie,
    tlbsync1,
    eciwx,
    ecowx,
    psq_lx,
    psq_stx,
    psq_lux,
    psq_stux,
    psq_l,
    psq_lu,
    psq_st,
    psq_stu,
    ps_div,
    ps_sub,
    ps_add,
    ps_sel,
    ps_res,
    ps_mul,
    ps_rsqrte,
    ps_msub,
    ps_madd,
    ps_nmsub,
    ps_nmadd,
    ps_neg,
    ps_mr,
    ps_nabs,
    ps_abs,
    ps_sum0,
    ps_sum1,
    ps_muls0,
    ps_muls1,
    ps_madds0,
    ps_madds1,
    ps_cmpu0,
    ps_cmpo0,
    ps_cmpu1,
    ps_cmpo1,
    ps_merge00,
    ps_merge01,
    ps_merge10,
    ps_merge11,
    dcbz_l,
}

#[derive(Debug)]
pub enum InstrLookup {
    Final(InstrMnem),
    SecondaryLookup(HashMap<usize, InstrMnem>),
}

lazy_static! {
    #[rustfmt::skip]
    static ref decodingFields: HashMap<String, u32> =
        HashMap::from([
        ("AA".to_string(),0b1 << (31-30)),
        ("BD".to_string(),(((2u32.pow((16i32-29i32).abs() as u32))-1) << (31-29)) as u32)]);
}

/*
AA 30 30
BD 16 29
BI 11 15
BO 6 10
crbA 11 15
crbB 16 20
crbD 6 10
crfD 6 8
crfS 11 13
CRM 12 19
d (16-31, 20-31)
FM 7 14
frA 11 15
frB 16 20
frC 21 25
frD 6 10
frS 6 10
I (17-19, 22-24)
IMM 16 19
LI 6 29
LK 31 31
MB 21 25
ME 26 30
NB 16 20
OE 21 21
OPCD 0 5
rA 11 15
rB 16 20
Rc 31 31
rD 6 10
rS 6 10
SH 16 20
SIMM 16 31
SR 12 15
TO 6 10
UIMM 16 31
XO (21-30, 22-30, 25-30, 26-30)
*/
