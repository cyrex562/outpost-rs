use crate::defines::{Struct18, U32Ptr};
use crate::fn_ptr::fn_ptr_1000::fn_ptr_1000_17ce;
use crate::global::AppContext;
use crate::mem_1000::mem_op_1000_179c;
use crate::pass::pass_1008::{pass1_1008_b9ce, pass1_1008_ec94};
use crate::pass::pass_1010::{pass1_1010_2e02, pass1_1010_2e5c};
use crate::pass::pass_1018::pass1_1018_4808;
use crate::pass::pass_1028::pass1_1028_e1ec;
use crate::pass::pass_1030::pass1_1030_1d7c;
use crate::pass::pass_1038::{load_string_1038_4d28, pass1_1038_4e78};
use crate::string::string_1000::{str_op_1000_3da4, string_1000_3d3e};
use crate::string::string_1010::{
    load_string_1010_847e, load_string_1010_84ac, load_string_1010_84e0,
};
use crate::struct_ops::struct_1008::{pass1_1008_c6ae, pass1_1008_c6fa, set_struct_1008_574a};
use crate::struct_ops::struct_1018::struct_1018_47c8;
use crate::ui::ui_1008::set_stuct_1008_b0bc;
use crate::util::{write_struct_to_addr, CONCAT12, CONCAT13, CONCAT22, SUB42, ZEXT24};
use crate::winapi::wsprintf16;

pub fn str_op_1008_60e8(ctx: &mut AppContext, param_1: &mut String, param_2: &mut Struct18) -> u16 {
    let u_var1: u16;

    if param_1 != 0x0 {
        u_var1 = str_op_1000_3da4(param_1);
        u_var1 += 0x1;
        mem_op_1000_179c(ctx, u_var1, param_2, 0x1000);
        if (param_2 | u_var1) != 0x0 {
            // unk_str_op_1000_3d3e(CONCAT22(param_2, u_var1), param_1);
            return u_var1;
        }
    }
    return 0x0;
}

pub fn str_1008_6d8a(
    ctx: &mut AppContext,
    mut param_1: U32Ptr,
    param_2: &mut String,
    param_3: &mut Struct18,
    param_4: u16,
    param_5: u8,
    stack0xfffe: u16,
) -> U32Ptr {
    let mut u_var1: u16 = 0;
    let mut u_var2: u16 = 0;

    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x0;
    (param_1 + 0x4) = 0xffff;
    ctx.PTR_LOOP_1050_0312 = &ctx.DAT_1050_0004;
    sys_1000_3f9c(
        0x65a0,
        ctx.data_seg,
        ctx.PTR_s_SC_03d_1050_0314_1050_031c,
        (ctx.PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),
        0x4,
        &stack0xfffe,
        u_var2,
        0x1000,
        param_4,
        param_5,
    );
    u_var1 = str_op_1008_60e8(ctx, param_2, param_3);
    param_1 = u_var1 as u32;
    // (param_1 + 0x2) = param_3;
    write_struct_to_addr(param_1 + 0x2, param_3);
    return param_1;
}

pub fn load_string_1008_b1f0() -> String {
    let mut pc_var1: String;

    pc_var1 = load_string_1010_847e(
        ctx.PTR_LOOP_1050_14cc,
        (ctx.PTR_LOOP_1050_14cc >> 0x10),
        0x1010,
    );
    return pc_var1;
}

pub fn load_string_1008_b65a(
    param_1: i32,
    in_string_2: &mut String,
    param_3: i32,
    param_4: u16,
    unaff_ss: u16,
) {
    pass1_1008_b9ce(param_1 as u32, CONCAT22(param_4, param_3._2_2_), unaff_ss);
    load_string_1010_84e0(
        0x1010,
        ctx.PTR_LOOP_1050_14cc,
        0x3ff,
        in_string_2,
        param_3 as i16,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn load_str_and_sprintf_1008_b69c(
    param_1: &mut Struct25,
    param_2: U32Ptr,
    string_2: &mut String,
) {
    let ppcVar1: u32;
    let mut string_1: String;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let iVar5: &mut Struct25;
    let u_var5: u16;
    let paVar6: &mut Struct26;
    let uVar7: u32;
    let iStack516: i16;
    let local_202: [u8; 0x100];
    let local_102: [u8; 0x100];

    string_1 = local_202;
    load_string_1010_84e0(
        0x1010,
        ctx.PTR_LOOP_1050_14cc,
        0x100,
        &mut string_1,
        param_2 as i16,
    );
    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    if iVar5.field_0xa == 0x0 {
        mem_op_1000_179c(0xc, param_3, 0x1000);
        if ((param_3 | string_1) == 0x0) {
            paVar6 = 0x0;
        } else {
            paVar6 = set_struct_1008_574a(CONCAT22(param_3, string_1));
        }
        &iVar5.field_0xa = paVar6;
        (&iVar5.field_0xa + 0x2) = (paVar6 >> 0x10);
        // for (iStack516 = 0x1; iStack516 < 0x6; iStack516 += 0x1) {
        //   mem_op_1000_179c(0x12,(paVar6 >> 0x10),0x1000);
        //   if (paVar6 == 0x0) {
        //     uVar7 = 0x0;
        //   }
        //   else {
        //     uVar7 = set_stuct_1008_b0bc(paVar6);
        //   }
        //   u_var3 = (uVar7 >> 0x10);
        //   u_var4 = u_var3;
        //   wsprintf16(&ctx.PTR_LOOP_1050_1000,local_102,param_2);
        //   u_var2 = str_op_1008_60e8(CONCAT22(param_2,local_102),u_var4);
        //   (uVar7 + 0x4) = u_var2;
        //   (uVar7 + 0x6) = u_var4;
        //   ppcVar1 = (*iVar5.field_0xa + 0x8);
        //   paVar6 = (**ppcVar1)();
        // }
        iVar5.field_0x22 = 0x5;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn load_str_and_sprintf_1008_b78a(
    param_1: i32,
    param_2: U32Ptr,
    uparam_3: &mut String,
    param_4: u16,
) {
    let pi_var1: U32Ptr;
    let ppcVar2: u32;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u32;
    let local_206: [u8; 0x100];
    let local_106: [u8; 0x100];
    let i_stack6: i16;
    let uStack4: u16;

    mem_op_1000_179c(0x12, param_3, 0x1000);
    if ((param_3 | param_4) == 0x0) {
        u_var6 = 0x0;
    } else {
        u_var6 = set_stuct_1008_b0bc(CONCAT22(param_3, param_4));
    }
    // uStack4 = (u_var6 >> 0x10);
    load_string_1010_84e0(
        0x1010,
        ctx.PTR_LOOP_1050_14cc,
        0x100,
        local_206,
        param_2 as i16,
    );
    i_stack6 = u_var6 as i16;
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1 as i16;
    pi_var1 = (i_var4 + 0x22) as u32;
    *pi_var1 = *pi_var1 + 0x1;
    wsprintf16(0x1010, local_106, param_2);
    i_stack6 = u_var6 as i16;
    u_var3 = str_op_1008_60e8(CONCAT22(param_2 as u16, local_106), (u_var6 >> 0x10));
    i_stack6 = u_var6 as i16;
    (i_stack6 + 0x4) = u_var3 as i16;
    (i_stack6 + 0x6) = (u_var6 >> 0x10) as i16;
    ppcVar2 = ((i_var4 + 0xa) + 0x8) as u32;
    (**ppcVar2)(ctx.s_tile2_bmp_1050_1538, (i_var4 + 0xa), i_stack6, uStack4);
    return;
}

pub fn unk_str_op_1008_d1c6(param_1: u32, param_2: u32) {
    let i_var1: i16;
    let u_var2: u32;
    let ppc_var3: u32;
    let bVar4: bool;
    let pu_var5: u32;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u8;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: u16;
    let puVar11: U32Ptr;
    let extraout_DX_01: U32Ptr;
    let uVar12: u16;
    let puVar13: U32Ptr;
    let extraout_DX_02: U32Ptr;
    let puVar14: U32Ptr;
    let uVar15: u16;
    let iVar16: i16;
    let valist: U32Ptr;
    let uVar17: u16;
    let puVar18: u32;
    let uVar19: u32;
    let uStack52: u16;
    let uStack20: u32;
    let uStack14: u32;
    let puStack10: u32;

    // valist = (param_1 >> 0x10);
    iVar16 = param_1 as i16;
    pu_var5 = (iVar16 + 0x12) as u32;
    puVar13 = (iVar16 + 0x14) as u32;
    if ((puVar13 | pu_var5) != 0x0) {
        ppc_var3 = *pu_var5;
        (**ppc_var3)();
        puVar13 = extraout_dx;
    }
    mem_op_1000_179c(0xc, puVar13 as u16, 0x1000);
    if ((puVar13 | pu_var5) == 0x0) {
        pu_var5 = 0x0;
        uVar17 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(puVar13 as u16, pu_var5 as u16));
        uVar17 = extraout_DX_00;
    }
    (iVar16 + 0x12) = pu_var5 as i16;
    (iVar16 + 0x14) = uVar17 as i16;
    puVar18 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x2);
    // puVar11 = (puVar18 >> 0x10);
    u_var6 = puVar18 as u16;
    uVar17 = SUB42(&ctx.PTR_LOOP_1050_1038, 0x0);
    pass1_1038_4e78(u_var6, puVar11, param_2, puVar18);
    puStack10 = CONCAT22(puVar11 as u16, u_var6);
    ppc_var3 = (*puStack10 + 0x10);
    uVar9 = u_var6;
    (**ppc_var3)(&ctx.PTR_LOOP_1050_1038, u_var6, puVar11);
    uStack14 = CONCAT22(extraout_DX_01 as u16, uVar9);
    bVar4 = false;
    puVar13 = extraout_DX_01;
    // for (uStack20 = 0x0; uStack20 < uStack14; uStack20 += 0x1) {
    //   uVar17 = 0x1030;
    //   uVar19 = pass1_1030_1d7c(uVar9,puVar13,puStack10);
    //   uVar12 = (uVar19 >> 0x10);
    //   uVar15 = uVar19;
    //   puVar13 = (uVar12 | uVar15);
    //   if (((puVar13 != 0x0) && ((uVar15 + 0x1c) != 0x8000002)) &&
    //      ((i_var1 = (uVar15 + 0x12), i_var1 == 0x5 || (i_var1 == 0x6)))) {
    //     puVar13 = ((uVar15 + 0x6) & 0xff);
    //     pass1_1020_bd80((uVar15 + 0xc));
    //     wsprintf16(0x1020,(iVar16 + 0x22),valist);
    //     uVar7 = str_op_1008_60e8((param_1 & 0xffff0000 | (iVar16 + 0x22)),
    //                              puVar13);
    //     uVar17 = 0x1000;
    //     puVar14 = puVar13;
    //     uVar8 = uVar7;
    //     mem_op_1000_179c(0x12,puVar13,0x1000);
    //     uStack52 = puVar14 | uVar8;
    //     if (uStack52 == 0x0) {
    //       uVar8 = 0x0;
    //       uStack52 = 0x0;
    //     }
    //     else {
    //       uVar17 = 0x1018;
    //       pass1_1018_4808(CONCAT22(puVar14,uVar8),0x1,
    //                       CONCAT13((puVar13 >> 0x8),
    //                                CONCAT12(puVar13,uVar7)),(uVar15 + 0x4));
    //     }
    //     u_var2 = (iVar16 + 0x12);
    //     ppc_var3 = ((iVar16 + 0x12) + 0x4);
    //     (**ppc_var3)(uVar17,u_var2,(u_var2 >> 0x10),uVar8,uStack52);
    //     bVar4 = true;
    //     puVar13 = extraout_DX_02;
    //   }
    // }
    if (!bVar4) {
        load_string_1010_84ac(ctx.PTR_LOOP_1050_14cc, 0x1010);
        uVar9 = uStack14 as u16;
        uVar17 = 0x1000;
        puVar14 = puVar13;
        mem_op_1000_179c(0x12, puVar13 as u16, 0x1000);
        uVar15 = (puVar14 | uVar9) as u16;
        if (uVar15 == 0x0) {
            uVar9 = 0x0;
            u_var10 = 0x0;
        } else {
            uVar17 = 0x1018;
            pass1_1018_4808(
                CONCAT22(puVar14 as u16, uVar9),
                0x0,
                uStack14 & 0xffff | ZEXT24(puVar13 as u16) << 0x10,
                0x0,
            );
            u_var10 = uVar15 as u8;
        }
        u_var2 = (iVar16 + 0x12) as u32;
        ppc_var3 = ((iVar16 + 0x12) + 0x4) as u32;
        (**ppc_var3)(uVar17, u_var2, (u_var2 >> 0x10), uVar9, u_var10);
    }
    if ((puVar11 | u_var6) != 0x0) {
        ppc_var3 = *puStack10;
        (**ppc_var3)(uVar17, u_var6, puVar11, 0x1);
    }
    return;
}

pub fn unk_str_op_1008_d4f6(param_1: u32, param_2: u32) {
    let lVar1: i32;
    let pu_var2: u32;
    let u_var3: u32;
    let ppc_var4: u32;
    let bVar5: bool;
    let puVar6: u32;
    let b_var7: bool;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let puVar11: u32;
    let uVar12: u32;
    let uVar13: u8;
    let extraout_dx: U32Ptr;
    let puVar14: U32Ptr;
    let extraout_DX_00: u16;
    let uVar15: u16;
    let extraout_DX_01: U32Ptr;
    let pWVar16: U32Ptr;
    let pWVar17: U32Ptr;
    let puVar18: U32Ptr;
    let uVar19: u16;
    let iVar20: i16;
    let iVar21: i16;
    let uVar22: u16;
    let valist: U32Ptr;
    let uVar23: u32;
    let uStack58: u16;
    let uStack20: u32;

    // uVar22 = (param_2 >> 0x10);
    iVar20 = param_2 as i16;
    lVar1 = (iVar20 + 0x200) as i32;
    // valist = (param_1 >> 0x10);
    iVar21 = param_1 as i16;
    puVar6 = (iVar21 + 0xe) as u32;
    puVar14 = (iVar21 + 0x10) as u32;
    if ((puVar14 | puVar6) != 0x0) {
        ppc_var4 = *puVar6;
        (**ppc_var4)();
        puVar14 = extraout_dx;
    }
    mem_op_1000_179c(0xc, puVar14 as u16, 0x1000);
    if ((puVar14 | puVar6) == 0x0) {
        puVar6 = 0x0;
        uVar15 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(puVar14 as u16, puVar6 as u16));
        uVar15 = extraout_DX_00;
    }
    (iVar21 + 0xe) = puVar6 as i16;
    (iVar21 + 0x10) = uVar15 as i16;
    pu_var2 = (iVar20 + 0xc) as u32;
    ppc_var4 = (*pu_var2 + 0x10);
    puVar11 = pu_var2;
    (**ppc_var4)(0x1000, pu_var2, (iVar20 + 0xe));
    uVar12 = puVar11 & 0xffff | ZEXT24(extraout_DX_01 as u16) << 0x10;
    bVar5 = false;
    // for (uStack20 = 0x0; uStack20 < uVar12; uStack20 += 0x1) {
    //   uVar23 = pass1_1030_1d7c((puVar11 & 0xffff),extraout_DX_01,pu_var2);
    //   uVar19 = (uVar23 >> 0x10);
    //   u_var10 = uVar23;
    //   if ((((uVar19 | u_var10) != 0x0) && ((u_var10 + 0x1c) != 0x8000002)) &&
    //      ((iVar20 = (u_var10 + 0x12), iVar20 == 0x5 || (iVar20 == 0x6)))) {
    //     uVar9 = (u_var10 + 0xc);
    //     b_var7 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0,uVar9,0x34);
    //     if ((b_var7 == 0x0) && ((u_var10 + 0x1c) != lVar1)) {
    //       pass1_1020_bd80(uVar9);
    //       pWVar16 = valist;
    //       wsprintf16(0x1020,(iVar21 + 0x22),valist);
    //       uVar8 = str_op_1008_60e8((param_1 & 0xffff0000 |
    //                                        ZEXT24((iVar21 + 0x22))),pWVar16)
    //       ;
    //       uVar22 = 0x1000;
    //       pWVar17 = pWVar16;
    //       uVar9 = uVar8;
    //       mem_op_1000_179c(0x14,pWVar16,0x1000);
    //       uStack58 = pWVar17 | uVar9;
    //       if (uStack58 == 0x0) {
    //         uVar9 = 0x0;
    //         uStack58 = 0x0;
    //       }
    //       else {
    //         uVar22 = 0x1018;
    //         struct_1018_47c8(CONCAT22(pWVar17,uVar9),0x1,CONCAT22(pWVar16,uVar8),
    //                          (u_var10 + 0xc),(u_var10 + 0x4));
    //       }
    //       u_var3 = (iVar21 + 0xe);
    //       ppc_var4 = ((iVar21 + 0xe) + 0x4);
    //       (**ppc_var4)(uVar22,u_var3,(u_var3 >> 0x10),uVar9,uStack58);
    //       bVar5 = true;
    //     }
    //   }
    // }
    if (!bVar5) {
        puVar14 = extraout_DX_01;
        load_string_1010_84ac(ctx.PTR_LOOP_1050_14cc, 0x1010);
        uVar22 = 0x1000;
        puVar18 = puVar14;
        u_var10 = uVar12 as u16;
        mem_op_1000_179c(0x14, puVar14 as u16, 0x1000);
        uVar19 = (puVar18 | u_var10) as u16;
        if (uVar19 == 0x0) {
            u_var10 = 0x0;
            uVar13 = 0x0;
        } else {
            uVar22 = 0x1018;
            struct_1018_47c8(
                CONCAT22(puVar18 as u16, u_var10),
                0x0,
                CONCAT13(
                    ((puVar14 >> 0x8) as u16),
                    CONCAT12(puVar14 as u8, uVar12 as u16),
                ),
                0x0,
                0x0,
            );
            uVar13 = uVar19 as u8;
        }
        u_var3 = (iVar21 + 0xe) as u32;
        ppc_var4 = ((iVar21 + 0xe) + 0x4) as u32;
        (**ppc_var4)(uVar22, u_var3, (u_var3 >> 0x10), u_var10, uVar13);
    }
    return;
}

pub fn string_1008_e586(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u16,
) -> u32 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let mut string_2: String;

    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_3 as u16);
    pu_var2 = (param_5 | param_4) as u32;
    if (pu_var2 == 0x0) {
        return 0x0;
    }
    u_var1 = param_4;
    mem_op_1000_179c(0x80, pu_var2 as u16, 0x1000);
    string_2 = load_string_1038_4d28(CONCAT22(param_5, param_4));
    string_1000_3d3e(CONCAT22(pu_var2 as u16, u_var1), &mut string_2);
    return CONCAT22(pu_var2 as u16, u_var1);
}

pub fn load_string_1008_ee56() -> U32Ptr {
    let mut pcVar1: String;

    pcVar1 = load_string_1010_847e(
        ctx.PTR_LOOP_1050_14cc,
        (ctx.PTR_LOOP_1050_14cc >> 0x10),
        0x1010,
    );
    return pcVar1;
}

pub fn pass1_1008_ee72(param_1: u16, param_2: u16, param_3: i16) {
    let ppcVar1: u32;
    let u_var2: u32;

    if ((param_1 + 0x56) == 0x0) {
        ppcVar1 = (CONCAT22(param_2, param_1) + 0x10);
        (**ppcVar1)();
    }
    u_var2 = pass1_1010_2e02(CONCAT22(param_2, param_1), param_3);
    pass1_1010_2e5c(param_1, param_2, u_var2);
    return;
}

pub fn pass1_1008_eea6() -> u16 {
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_eeac(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    uparam_4: &mut String,
    param_5: i16,
    param_6: u16,
) -> bool {
    let u_var1: u16;
    let cVar2: u8;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let uVar7: u16;

    uVar7 = (param_3 + 0x12);
    puVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_6, param_4, param_5);
    // u_var4 = (puVar6 >> 0x10);
    u_var1 = puVar6;
    u_var5 = u_var4;
    if (uVar7 == 0x7d) {
        pass1_1010_a5ca(u_var1, u_var4, 0x7c, 0x7d, u_var4);
        if (uVar7 != 0x0) {
            return false;
        }
        pass1_1010_a5ca(u_var1, u_var4, 0x7d, 0x0, u_var5);
        if (uVar7 != 0x0) {
            return false;
        }
        u_var3 = uVar7;
        uVar7 = 0x78;
    } else {
        u_var3 = uVar7;
        if (uVar7 < 0x7e) {
            cVar2 = uVar7;
            u_var3 = uVar7 & 0xff00;
            if ((cVar2 + 0x8d) == 0x0) {
                uVar7 = 0x9;
                u_var3 = u_var3 | (cVar2 + 0x8d);
            } else {
                if ((cVar2 + 0x89) == 0x0) {
                    uVar7 = 0x2e;
                    u_var3 = u_var3 | (cVar2 + 0x89);
                } else {
                    u_var3 |= (cVar2 + 0x87);
                    if ((cVar2 + 0x87) == 0x0) {
                        uVar7 = 0x5b;
                    }
                }
            }
        }
    }
    pass1_1010_a5ca(u_var1, u_var4, uVar7, u_var3, u_var5);
    return u_var3 == 0x0;
}

pub fn pass1_1008_ef38(param_1: u32) -> u16 {
    let u_var1: u32;

    u_var1 = (param_1 + 0x16);
    return (u_var1 + 0x2) as u16;
}

pub fn pass1_1008_ef4a() -> u16 {
    return 0x41;
}

pub fn pass1_1008_ef50(param_1: U32Ptr, param_2: u8) -> u16 {
    pass1_1008_ec94(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1 as u16;
}

pub fn pass1_1008_ef76(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    let unaff_SS: u16;

    pass1_1008_ed00(param_1, unaff_SS);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}
