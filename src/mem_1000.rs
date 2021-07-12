use crate::{
    defines::StructA,
    fn_ptr::fn_ptr_1000::fn_ptr_op_1000_1708,
    global::AppContext,
    misc::{empty_fn_1000_214a, ret_true_1000_2146},
    pass::pass_1000::{
        pass1_1000_0368, pass1_1000_0c32, pass1_1000_15ce, pass1_1000_1a54, pass1_1000_1afe,
        pass1_1000_1e61, pass1_1000_20a2, pass1_1000_25a8, pass1_1000_2913, pass1_1000_41e0,
        pass1_1000_52be, pass1_1000_5390,
    },
    string::string_1000::poss_str_op_1000_28dc,
    sys_api::{mixed_dos3_call_1000_3636, mixed_dos3_call_1000_39f2, _SHI_INVOKEERRORHANDLER1},
    util::{make_u16_ptr, CARRY2, CONCAT11, CONCAT22, SUB42},
    win_struct::HGLOBAL16,
    winapi::{
        FatalAppExit16, FatalExit, GLobalAlloc16, GlobalFree16, GlobalHandle16, GlobalPageLock16,
        GlobalPageUnlock16, GlobalReAlloc16, GlobalSize16, GlobalUnlock16, SegmentLimit,
        WIN16_GlobalLock16,
    },
};
use crate::defines::{Struct79, Struct_1000_05e2, Struct19};
use crate::winapi::GlobalDOSAlloc16;
use crate::pass::pass_1000::{pass1_1000_09ca, pass1_1000_05b4, pass1_1000_0782};

pub unsafe fn mem_op_1000_0052(param_1: u32, param_2: u32) {
    unimplemented!()
}

pub unsafe fn mem_op_1000_01b0(ctx: &mut AppContext, param_1: &mut StructA, param_2: u16) -> bool {
    let pu_var1: *mut u16;
    let pi_var2: *mut i16;
    let b_var3: bool;
    let u_var4: u16;
    let u_var5: u32;
    let d_var6: u32;
    let d_var7: u32;
    let u_var8: u32;
    let u_var9: u32;
    let u_var10: u16;
    let u_stack14: u16;
    let u_stack12: u16;
    let u_stack10: i16;
    let u_stack6: u16;
    let i_stack4: i16;

    u_stack14 = 0x0;
    // if (((param_1.offset(0x40)).read() | (param_1.offset(0x3e)).read()) == 0x0) {
    if param_1.field_0x40 | param_1.field_0x3e == 0 {
        u_var5 = param_1.field_0x36;
        d_var6 = mem_op_1000_1532(ctx,param_2);
        d_var7 = d_var6;
    } else {
        d_var6 = mem_op_1000_1532(ctx, param_2);
        u_var5 = d_var6;
        if ((d_var6 >> 0x10) != 0x0) || (0xffef < u_var5) {
            pass1_1000_1e61(ctx, param_2, 0x8, param_1, ctx.data_seg);
            return false;
        }
        if 0x1fff < u_var5 {
            u_var5 = 0x2000;
        }
        loop {
            u_var9 = u_var5;
            d_var7 = d_var6 + 0x18;
            if ((d_var7 >> 0x10) != 0x0) || (0xfff0 < d_var7) {
                d_var7 = 0xfff0;
            }
            b_var3 = mem_op_1000_14f2(
                param_1.field_0x16 | 0x1000,
                d_var7,
                param_1,
                ctx.data_seg,
                0,
                0,
                0,
            );
            i_stack4 = (d_var6 >> 0x10) as i16;
            u_stack6 = d_var6 as u16;
            if b_var3 {
                break;
            }
            u_var5 = u_var9 >> 0x1;
            if u_var5 < 0xc {
                u_var4 = pass1_1000_1e61(ctx,param_2, 0x2, param_1, ctx.data_seg);
                if u_var4 == 0x0 {
                    return '\x01' as u32 - (param_1 + 0xa) == 0x0;
                }
                d_var6 = mem_op_1000_1532(ctx, param_2);
                u_var5 = u_var9 & 0xfffe;
            }
        }
        u_var8 = pass1_1000_5390(
            u_stack6 - 0x42,
            i_stack4 - ((u_stack6 < 0x42) as i16),
            0xc,
            0x0,
        );
        u_var5 = u_var8 * 0xc + param_1 + 0x42;
    }
    pu_var1 = make_u16_ptr(param_1 + 0x1e);
    u_var9 = *pu_var1 as u32;
    *pu_var1 = *pu_var1 - d_var6;
    pi_var2 = (param_1 + 0x20);
    *pi_var2 = (*pi_var2 - (d_var6 >> 0x10)) - (u_var9 < d_var6);
    if u_var5 != 0x0 {
        u_var10 = 0x0;
        u_var9 = 0xc;
        d_var7 = mem_op_1000_1532(ctx,param_2);
        u_var8 = pass1_1000_5390(
            d_var7 - 0x42,
            (d_var7 >> 0x10) - (d_var7 < 0x42),
            u_var9,
            u_var10,
        );
        u_stack14 = u_var8 * 0xc + param_1 + 0x36;
    }
   // u_stack10 = (d_var7 >> 0x10);
    u_stack12 = d_var7;
    pu_var1 = (param_1 + 0x1e);
    u_var9 = *pu_var1;
    *pu_var1 = *pu_var1 + u_stack12;
    pi_var2 = (param_1 + 0x20);
    *pi_var2 = *pi_var2 + u_stack10 + CARRY2(u_var9, u_stack12);
    u_var9 = (param_1 + 0xa);
    loop {
        u_var10 = u_var5;
        (u_var10 + 0x4) = u_var9;
        u_var9 = u_var10;
        u_var5 = u_var10 + 0xc;
        if u_var10 >= u_stack14 {
            break;
        }
    }
    (param_1 + 0xa) = u_var10;
    return true;
}

pub unsafe fn mem_op_1000_0308(param_1: i16, param_2: &mut StructA, param_3: u16) -> i16 {
    let i_var1: i16;
    let i_var2: i16;
    let b_var3: bool;
    let extraout_a_h: u8;
    let pi_var4: *mut i16;

    if (param_2 + 0xa) == 0x0 {
        b_var3 = mem_op_1000_01b0(ctx,param_2, param_3);
        if CONCAT11(extraout_a_h, b_var3) == 0x0 {
            return 0x0;
        }
    }
    i_var1 = (param_2 + 0xa);
    (param_2 + 0xa) = (i_var1 + 0x4);
    pi_var4 = (param_1 * 0x2 + param_2);
    if (*pi_var4 == 0x0) {
        (i_var1 + 0x6) = i_var1;
        (i_var1 + 0x4) = i_var1;
    } else {
        i_var2 = *pi_var4;
        (i_var1 + 0x6) = i_var2;
        (i_var1 + 0x4) = (i_var2 + 0x4);
        ((i_var2 + 0x4) + 0x6) = i_var1;
        (i_var2 + 0x4) = i_var1;
    }
    *pi_var4 = i_var1;
    return i_var1;
}

pub unsafe fn mem_op_1000_03c6(
    param_1: &mut u16,
    param_2: i16,
    param_3: u16,
    param_4: &mut StructA,
    param_5: &mut u16,
    param_6: u8,
    param_7: u8,
) -> u32 {
    let puVar1: *mut u16;
    let piVar2: *mut i16;
    let uVar3: u16;
    let uVar4: u16;
    let puVar5: *mut u16;
    let UVar6: u16;
    let uVar7: u16;
    let bVar8: bool;
    let DVar9: u32;
    let uStack20: u16;
    let uVar9: u16;

    uVar7 = CONCAT11(param_7, param_6);
    uVar3 = param_1 + 0xfff & 0xf000;
    puVar1 = (param_4 + 0x1e);
    uVar4 = uVar3 + *puVar1;
    uVar3 = param_2 + (0xf000 < param_1) + (param_4 + 0x20) + CARRY2(uVar3, *puVar1);
    puVar1 = (param_4 + 0x28);
    bVar8 = uVar3 < *puVar1;
    if (bVar8)
        || (bVar8
            || uVar3 == *puVar1
                && (
                    puVar1 = (param_4 + 0x26),
                    uVar4 < *puVar1 || uVar4 == *puVar1,
                ))
    {
        if param_3 == 0x3 {
            uStack20 = ((-((param_6 & 0x1) != 0x0) >> 0x8) & 0x1 | 0x20) << 0x8;
        } else {
            uStack20 = 0x1000;
        }
        uStack20 = (param_4 + 0x16) | uStack20;
        mem_op_1000_131c(ctx,uStack20, param_1, param_2, param_5);
        if (uVar3 | uStack20) != 0x0 {
            puVar5 = mem_op_1000_0308(param_3, param_4);
            if puVar5 != 0x0 {
                puVar5[0x4] = uStack20;
                puVar5[0x5] = uVar3;
                uVar9 = ctx.data_seg;
                ctx.PTR_LOOP_1050_000c = param_3 | 0xcad0;
                0x0 = param_4;
                ctx.PTR_LOOP_1050_0002 = ctx.data_seg;
                ctx.DAT_1050_0004 = puVar5;
                (ctx.DAT_1050_0004 + 0x2) = ctx.data_seg;
                ctx.PTR_LOOP_1050_000a = 0x0;
                DVar9 = mem_op_1000_1532(ctx,param_5);
                UVar6 = DVar9;
                if param_3 == 0x1 {
                    uVar7 = pass1_1000_0782(param_4, UVar6, 0x0);
                } else {
                    if param_3 == 0x3 {
                        pass1_1000_05b4(param_6, 0x0);
                    } else {
                        uVar7 = pass1_1000_09ca(UVar6, 0x0);
                    }
                }
               // param_2 = (DVar9 >> 0x10);
                *puVar5 = uVar7;
                puVar5[0x1] = 0x8000;
                puVar1 = (param_4 + 0x1e);
                uVar4 = *puVar1;
                *puVar1 = *puVar1 + UVar6;
                piVar2 = (param_4 + 0x20);
                *piVar2 = *piVar2 + param_2 + CARRY2(uVar4, UVar6);
                return uVar3;
            }
            mem_op_1000_13ce(param_5);
        }
    } else {
        pass1_1000_1e61(ctx,param_5, 0x7, param_4, ctx.data_seg);
    }
    return 0x0;
}

pub unsafe fn mem_op_1000_0510(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16,
) -> u32 {
    let pu_var1: *mut u16;
    let pi_var2: *mut i16;
    let b_var3: u8;
    let i_var4: i16;
    let u_var6: u16;
    let u_var7: u16;
    let u_var8: u16;
    let u_var9: u16;
    let u_var10: u16;
    let b_var11: bool;
    let d_var12: u32;
    let l_var13: i32;
    let u_var5: u16;

    i_var4 = param_2;
    u_var5 = (param_2 + 0x2);
    u_var6 = (param_2 + 0x4);
    b_var3 = (param_2 + 0xc);
    d_var12 = mem_op_1000_1532(ctx,param_3);
   // u_var9 = (d_var12 >> 0x10);
    u_var8 = d_var12;
    if (param_1 != 0x0) {
        u_var7 = (i_var4 + 0x1e);
        u_var10 = ((i_var4 + 0x20) - u_var9) - (u_var7 < u_var8);
        pu_var1 = (i_var4 + 0x24);
        b_var11 = u_var10 < *pu_var1;
        if ((b_var11 || u_var10 == *pu_var1) && (b_var11 || (u_var7 - u_var8 < (i_var4 + 0x22)))) {
            b_var11 = false;
            u_var9 = u_var10;
            // TODO: goto LAB_1000_0595;
        }
    }
    pass1_1000_0368(u_var6, b_var3 & 0x7, 0x0);
    pu_var1 = (ctx.s_version__d__d_1050_0012 + 0xc);
    u_var7 = *pu_var1;
    *pu_var1 = *pu_var1 - u_var8;
    pi_var2 = ctx.s_New_failed_in_Op__Op_1050_0020;
    *pi_var2 = (*pi_var2 - u_var9) - (u_var7 < u_var8);
    b_var11 = true;
    // TODO: LAB_1000_0595:
    if (b_var11) {
        (param_2 + 0xc) = 0x0;
        l_var13 = mem_op_1000_13ce(param_3);
        return CONCAT22((l_var13 >> 0x10), 0x1);
    }
    return u_var9 << 0x10;
}

pub unsafe fn mem_op_1000_05e2(
    ctx: &mut AppContext,
    param_1: &mut Struct_1000_05e2,
    param_2: i16,
    param_3: u16,
    param_4: &mut StructA,
    param_5: &mut u16,
) -> u32 {
    let pu_var1: *mut u16;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let b_var5: bool;
    let u_var6: u32;

    i_var2 = param_2 + (0xffeb < param_1);
    while ((b_var5 || u_var3 == *pu_var1)
        && (b_var5
            || (
                pu_var1 = (param_4 + 0x26),
                u_var4 < *pu_var1 || u_var4 == *pu_var1,
            )))
        && (u_var6 != 0x0
            || (
                u_var5 = pass1_1000_1e61(ctx, param_5, 0x2, param_4, ctx.data_seg),
                u_var5 != 0x0,
            ))
    {
        u_var3 = 0x3;
        u_var6._0_1_ = param_3;
        u_var6._1_1_ = (param_3 >> 0x8);
        u_var6._0_2_ = mem_op_1000_03c6(
            param_1 + 0x14,
            i_var2,
            0x3,
            param_4,
            param_5,
            u_var6,
            u_var6._1_1_,
        );
        if ((u_var6 | u_var3) != 0x0) {
            return CONCAT22(u_var6, u_var3 + 0x14);
        }
        u_var6 = mem_op_1000_0052(param_4, param_5);
        u_var3 = param_1 + 0x1013 & 0xf000;
        pu_var1 = (param_4 + 0x1e);
        u_var4 = u_var3 + *pu_var1;
        u_var3 = i_var2 + (0xf000 < param_1 + 0x14) + (param_4 + 0x20) + CARRY2(u_var3, *pu_var1);
        pu_var1 = (param_4 + 0x28);
        b_var5 = u_var3 < *pu_var1;
    }
    return 0x0;
}

pub unsafe fn mem_1000_0668(ctx: &mut AppContext, param_1: u16) -> u32 {
    let u_var1: u32;

    u_var1 = mem_op_1000_0510(ctx, 0x0, 0x0, param_1);
    return u_var1;
}

pub unsafe fn mem_1000_0670(
    param_1: u16,
    param_2: *mut i16,
    param_3: u16,
    param_4: *mut u32,
    param_5: i16,
    param_6: u16,
) -> u16 {
    let puVar1: *mut u16;
    let piVar2: *mut i16;
    let UVar3: &mut StructA;
    let UVar4: u16;
    let iVar5: i16;
    let UVar6: &mut StructA;
    let uVar7: u32;
    let uVar8: u16;
    let uVar9: u16;
    let UVar10: u16;
    let BVar11: bool;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u16;
    let DVar15: u32;
    let DVar16: u32;

    UVar3 = param_4;
    UVar4 = (param_4 + 0x2);
    DVar15 = mem_op_1000_1532(ctx,param_6);
    UVar6 = param_5 + (0xffeb < param_3);
    uVar7 = *param_4;
    uVar8 = -((param_1 & 0x1) != 0x0) & 0x100 | -((param_1 & 0x4) != 0x0) & 0x400 | (uVar7 + 0x16);
    if (param_2 == 0x0) {
        BVar11 = mem_op_1000_14f2(uVar8 | 0x2000, param_3 + 0x14, UVar6, param_4, ctx.data_seg);
        if (BVar11 == 0x0) {
            return 0x0;
        }
    } else {
        iVar5 = (param_4 + 0x1);
        uVar12 = (param_4 + 0x6);
        uVar14 = uVar12;
        loop {
            uVar13 = uVar14;
            uVar9 = uVar8 | 0x2000;
            mem_op_1000_1408(ctx,uVar9, param_3 + 0x14, UVar6, param_6);
            uVar14 = uVar13 | uVar9;
            if (uVar14 != 0x0) {
                break;
            }
            UVar10 = pass1_1000_1e61(ctx,param_6, 0x2, UVar3, UVar4);
            if UVar10 == 0 {
                break;
            }
        }
        if ((uVar13 | uVar9) == 0x0) {
            (param_2 + 0x2) = 0x0;
            *param_2 = 0x0;
            return 0x0;
        }
        (iVar5 + 0x8) = uVar9;
        (iVar5 + 0xa) = uVar13;
        *param_2 = uVar9 + 0x14;
        (param_2 + 0x2) = uVar13;
    }
    DVar16 = mem_op_1000_1532(ctx,param_6);
    uVar12 = (DVar16 - DVar15);
    puVar1 = (UVar3 + 0x1e);
    uVar8 = *puVar1;
    *puVar1 = *puVar1 + uVar12;
    piVar2 = (UVar3 + 0x20);
    *piVar2 = *piVar2 + (DVar16 - DVar15 >> 0x10) + CARRY2(uVar8, uVar12);
    return 0x1;
}

pub unsafe fn mem_op_1000_0838(ctx: &mut AppContext, param_1: &mut StructA, param_2: u16) -> u32 {
    let pu_var1: *mut u16;
    let pi_var2: *mut i16;
    let i_var3: i16;
    let pu_var4: *mut u16;
    let u_var5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let u_var8: u32;
    let pi_var9: *mut i16;
    let b_var10: bool;
    let u_stack6: u16;
    let pi_stack4: *mut i16;

    pi_var9 = (param_1 + 0x2);
    pi_stack4 = pi_var9;
    if (param_1 + 0x2) == 0x0 {
        // TODO: goto LAB_1000_085b;
    }
    loop {
        loop {
            if *pi_var9 != 0x0 {
                i_var3 = pi_var9[0x5];
                pu_var4 = ctx.PTR_LOOP_1050_000e;
                if pu_var4 != 0x0 {
                    ctx.PTR_LOOP_1050_000e = pu_var4;
                    pi_var2 = ctx.PTR_LOOP_1050_000a;
                    *pi_var2 = *pi_var2 + 0x1;
                    (param_1 + 0x2) = pi_var9;
                    return CONCAT22(i_var3, pu_var4);
                }
                *pi_var9 = 0x0;
            }
            pi_var9 = pi_var9[0x2];
            if pi_var9 == pi_stack4 {
                break;
            }
        }
        // TODO: LAB_1000_085b:
        if (param_1 + 0x18) == 0x0 {
            pass1_1000_1e61(ctx,param_2, 0x4, param_1, ctx.data_seg);
            return 0x0;
        }
        u_var5 = (param_1 + 0x1a);
        loop {
            u_stack6 = u_var5;
            u_var5 = 0x1;
            u_var8 = mem_op_1000_03c6(u_stack6, 0x0, 0x1, param_1, param_2, 0x0, '\0');
            if (u_var8 | u_var5) != 0x0 {
                break;
            }
            u_var5 = (param_1 + 0x1e);
            u_var6 = u_var5 + u_stack6;
            u_var5 = (param_1 + 0x20) + CARRY2(u_var5, u_stack6);
            pu_var1 = (param_1 + 0x28);
            b_var10 = *pu_var1 <= u_var5;
            if b_var10 {
                if b_var10 && u_var5 != *pu_var1 {
                    return 0x0;
                }
                pu_var1 = (param_1 + 0x26);
                if *pu_var1 <= u_var6 && u_var6 != *pu_var1 {
                    return 0x0;
                }
            }
            u_var5 = u_stack6 >> 0x1;
            if u_stack6 >> 0x1 < (param_1 + 0x18) + 0x14 {
                u_var7 = pass1_1000_1e61(ctx,param_2, 0x2, param_1, ctx.data_seg);
                u_var5 = u_stack6 & 0xfffe;
                if u_var7 == 0x0 {
                    return 0x0;
                }
            }
        }
        pi_var9 = (param_1 + 0x2);
        pi_stack4 = pi_var9[0x2];
    }
}

pub unsafe fn mem_op_1000_0a48(
    ctx: &mut AppContext,
    param_1: u8,
    param_2: u32,
    param_4: u32,
    param_5: u16,
    in_stack_00000005: u8,
) -> i32 {
    let u_var1: u16;
    let pu_var2: *mut u16;
    let u_var4: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;
    let pu_var1: *mut u16;

   // u_var4 = (param_4 >> 0x10);
    if ((param_4 + 0x14) == -0x4153) {
        if ((param_3 != 0x0) || (true && ((ctx.s_version__d__d_1050_0012 + 0x6) < param_2))) {
            if ((param_3 != 0x0) || (true && ((ctx.s_version__d__d_1050_0012 + 0xa) < param_2))) {
                u_var5 = mem_op_1000_05e2(ctx, param_2, param_3, _param_1 & 0xfffd, 0x0, param_5);
            } else {
                u_var5 = mem_op_1000_0b20(_param_1 & 0xfffd, 0x0, param_2, param_5);
            }
        } else {
            if ((false) || (param_2 != 0x0)) {
                u_var5 = mem_op_1000_0838(ctx, 0x0, param_5);
               // u_var3 = (u_var5 >> 0x10);
                pu_var2 = u_var5;
                if ((u_var5 != 0x0) && ((param_1 & 0x1) != 0x0)) {
                    u_var1 = (ctx.s_version__d__d_1050_0012 + 0x6);
                    // TODO: refactor for loop
                    // for (u_var4 = u_var1 >> 0x1; u_var4 != 0x0; u_var4 -= 0x1) {
                    //   pu_var1 = pu_var2;
                    //   pu_var2 = pu_var2 + 0x1;
                    //   *pu_var1 = 0x0;
                    // }
                    if ((u_var1 & 0x1) != 0x0) {
                        *pu_var2 = 0x0;
                    }
                }
            } else {
                pass1_1000_1e61(ctx, param_5, 0x4, param_4, u_var4);
                u_var5 = 0x0;
            }
        }
        return u_var5;
    }
    pass1_1000_1e61(ctx, param_5, 0xa, 0x0, 0x0);
    return 0x0;
}

pub unsafe fn mem_op_1000_0b20(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: &mut StructA,
    param_3: u16,
    param_4: &mut u16,
) -> u16 {
    let pu_var1: *mut u16;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let pu_var7: *mut u16;
    let u_var8: u16;
    let b_var9: bool;
    let u_var10: u32;
    let u_stack20: u16;
    let pu_stack6: *mut u16;

    u_var8 = SUB42(ctx.data_seg, 0x0);
    u_var2 = param_1 & 0x2;
    u_var4 = param_3 + 0x5 & 0xfffc;
    u_var4 = u_var4 - 0x8 & !-(u_var4 < 0x8);
    u_var5 = u_var4 + 0x8;
    pu_var7 = (u_var2 * 0x2 + param_2);
    u_stack20 = param_1;
    pu_stack6 = pu_var7;
    if (pu_var7 == 0x0) {
        // TODO: goto LAB_1000_0b64;
    }
    loop {
        loop {
            if ((u_var5 <= *pu_var7)
                && (
                    u_var10 = pass1_1000_0c32(u_var5, u_stack20, 0x0),
                    u_var10 != 0x0,
                ))
            {
                (u_var2 * 0x2 + param_2) = pu_var7;
                return u_var10;
            }
            pu_var7 = pu_var7[0x2];
            if pu_var7 == pu_stack6 {
                break;
            }
        }
        //LAB_1000_0b64:
        if ((((u_stack20 & 0x2) == 0x0) || ((u_stack20 & 0x40) != 0x0))
            || ((param_2 + 0x32) == 0x0))
        {
            //LAB_1000_0b9e:
            if (((u_stack20 & 0x10) != 0x0)
                || (
                    u_var3 = u_var2,
                    u_var6 = mem_op_1000_03c6(
                        (param_2 + 0x1a),
                        0x0,
                        u_var2,
                        param_2,
                        param_4,
                        0x0,
                        '\0',
                    ),
                    (u_var6 | u_var3) == 0x0,
                ))
            {
                if ((u_stack20 & 0x20) == 0x0) {
                    u_var2 = u_var4 + 0x1007 & 0xf000;
                    pu_var1 = (param_2 + 0x1e);
                    u_var4 = u_var2 + *pu_var1;
                    u_var2 = (param_2 + 0x20) + CARRY2(u_var2, *pu_var1);
                    pu_var1 = (param_2 + 0x28);
                    b_var9 = u_var2 < *pu_var1;
                    if ((b_var9 || u_var2 == *pu_var1)
                        && (b_var9
                            || (
                                pu_var1 = (param_2 + 0x26),
                                u_var4 < *pu_var1 || u_var4 == *pu_var1,
                            )))
                    {
                        u_var10 = mem_op_1000_05e2(ctx, u_var5, 0x0, u_stack20, param_2, param_4);
                        return u_var10;
                    }
                }
                return 0x0;
            }
        } else {
            *param_4 = 0x1000;
            u_var3 = (param_2 + 0x32)();
            if (u_var3 < u_var5) {
                // TODO: goto LAB_1000_0b9e;
            }
            u_stack20 |= 0x40;
        }
        pu_var7 = (u_var2 * 0x2 + param_2);
        pu_stack6 = pu_var7[0x2];
    }
}

pub unsafe fn mem_op_1000_131c(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: &mut u16,
    param_3: &mut i16,
    param_4: &mut u16,
) {
    let h_var1: HGLOBAL16;
    let b_var2: bool;
    let l_var3: i32;
    let u_stack10: u16;
    let u_stack8: u16;
    let i_stack6: i16;

    l_var3 = CONCAT22(u_stack8, u_stack10) as i32;
    i_stack6 = 0x1;
    if ((param_1 & 0x1000) != 0x0) && (param_3 != 0x0 || (0xfff0 < param_2)) {
        *param_2 = 0xfff0;
        *param_3 = 0x0;
    }
    if (param_1 & 0x4) != 0x0 {
        l_var3 = mem_op_1000_1558(param_2, param_3, param_4);
    }
    loop {
        h_var1 = GLobalAlloc16(param_4, CONCAT22(param_3, param_2));
        u_stack10 = l_var3;
        if h_var1 != 0x0 {
            break;
        }
        b_var2 = i_stack6 != 0x0;
        i_stack6 = i_stack6 + -0x1;
        *param_4 = ctx.s_tile2_bmp_1050_1538;
        if b_var2 == false {
            break;
        }
    }
    if (param_1 & 0x4) != 0x0 {
        if h_var1 != 0x0 {
            GlobalPageLock16(ctx.s_tile2_bmp_1050_1538);
        }
        pass1_1000_15ce(u_stack10, (l_var3 >> 0x10), ctx.s_tile2_bmp_1050_1538);
    }
    if h_var1 != 0x0 {
        WIN16_GlobalLock16(ctx.s_tile2_bmp_1050_1538);
        return;
    }
    return;
}

pub fn mem_op_1000_13ce(param_1: u16) -> i32 {
    let HVar1: HGLOBAL16;
    let uVar2: u16;
    let DVar3: u32;

    DVar3 = GlobalHandle16(param_1);
   // uVar2 = (DVar3 >> 0x10);
    if (DVar3 != 0x0) {
        HVar1 = GlobalFree16(ctx.s_tile2_bmp_1050_1538);
        return CONCAT22(uVar2, (HVar1 == 0x0));
    }
    return (uVar2 << 0x10);
}

pub fn mem_op_1000_1408(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: &mut u16,
    param_3: &mut StructA,
    param_4: u16,
) {
    let h_var1: HGLOBAL16;
    let b_var2: bool;
    let d_var3: u32;
    let i_stack12: i16;
    let u_stack8: u16;

    d_var3 = GlobalHandle16(param_4);
    u_stack8 = 0x32;
    i_stack12 = 0x1;
    if (((param_1 & 0x1000) != 0x0) && (param_3 != 0x0 || (0xfff0 < param_2))) {
        *param_2 = 0xfff0;
        *param_3 = 0x0;
    }
    if ((param_1 & 0x100) != 0x0) {
        u_stack8 = 0x72;
    }
    if ((param_1 & 0x804) != 0x0) {
        u_stack8 &= 0xfffd;
    }
    if (d_var3 != 0x0) {
        if ((param_1 & 0x4) != 0x0) {
            GlobalPageUnlock16(ctx.s_tile2_bmp_1050_1538);
        }
        loop {
            h_var1 = GlobalReAlloc16(
                ctx.s_tile2_bmp_1050_1538,
                CONCAT22(param_2, u_stack8),
                param_3,
            );
            if h_var1 != 0x0 {
                break;
            }
            u_stack8 &= 0xffcf;
            b_var2 = i_stack12 != 0x0;
            i_stack12 = i_stack12 + -0x1;
            if !b_var2 {
                break;
            }
        }
        if (h_var1 != 0x0) && ((param_1 & 0x4) != 0x0) {
            GlobalPageLock16(ctx.s_tile2_bmp_1050_1538 as HGLOBAL16);
        }
        if (h_var1 != 0x0) {
            WIN16_GlobalLock16(ctx.s_tile2_bmp_1050_1538);
            return;
        }
    }
    return;
}

pub fn mem_op_1000_14f2(
    param_1: u16,
    param_2: u32,
    param_3: &mut StructA,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
) -> bool {
    // if (((param_1 & 0x1000) != 0x0) || ((param_3 == 0x0 && (param_2 < 0xfff1)))) {
    if (param_1 & 0x1000) || (param_2 < 0xfff10000) {
        mem_op_1000_1408(ctx,param_1 & 0xfdff | 0x800, param_2, param_7, 0);
        if ((param_6 | param_5) != 0x0) {
            return true;
        }
    }
    return false;
}

pub fn mem_op_1000_1532(ctx: &mut AppContext, param_1: u16) -> u32 {
    let d_var1: u32;

    d_var1 = GlobalHandle16(param_1);
    if (d_var1 != 0x0) {
        d_var1 = GlobalSize16(ctx.s_tile2_bmp_1050_1538);
        return d_var1;
    }
    return 0x0;
}

pub fn mem_op_1000_1558(param_1: u16, param_2: u16, param_3: &mut u16) -> i32 {
    let u_var1: u16;
    let d_var2: u32;
    let u_stack12: u16;
    let u_stack10: u16;
    let u_stack8: u16;

    u_stack12 = 0x0;
    u_stack10 = 0x0;
    u_stack8 = 0x8;
    if ((param_2 < 0x9) && (param_2 < 0x8 || (param_1 == 0x0))) {
        while ((param_2 < u_stack8) || (param_2 <= u_stack8 && (param_1 <= u_stack10))) {
            loop {
                d_var2 = CONCAT22(uStack10, param_3);
                *param_3 = ctx.s_tile2_bmp_1050_1538;
                d_var2 = GlobalDOSAlloc16(DVar2);
                uVar1 = d_var2;
                if (uVar1 == 0x0) {
                    break;
                }
                0x0 = 0x0;
                ctx.PTR_LOOP_1050_0002 = u_stack12;
                u_stack12 = uVar1;
            }
            uVar1 = u_stack8 & 0x1;
            u_stack8 >>= 0x1;
            u_stack10 = u_stack10 >> 0x1 | (u_var1 != 0x0) << 0xf;
        }
    }
    return (u_stack12 << 0x10) as i32;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn mem_op_1000_160a(ctx: &mut AppContext, param_1: &mut Struct79, param_2: u16) -> u16 {
    let pu_var1 = ret_true_1000_2146();
    if pu_var1 == 0x0 {
        return pu_var1;
    }
    if (ctx.PTR_LOOP_1050_5f2e | ctx.PTR_LOOP_1050_5f2c) == 0x0 {
        ctx.DAT_1050_5f30 = 0x1;
        ctx.DAT_1050_5f32 = 0x1;
        ctx._PTR_LOOP_1050_5f2c = mem_op_1000_18ec(ctx, ctx.DAT_1050_5f46, param_1, param_2);
        if ctx._PTR_LOOP_1050_5f2c != 0x0 {
            if ctx.PTR_LOOP_1050_5f42 != 0x0 {
                pass1_1000_1a54(
                    ctx.PTR_LOOP_1050_5f42,
                    ctx._PTR_LOOP_1050_5f2c,
                    (ctx._PTR_LOOP_1050_5f2c >> 0x10),
                    param_2,
                );
            }
            ctx.PTR_LOOP_1050_5f2e = (ctx._PTR_LOOP_1050_5f2c >> 0x10);
            if ctx.DAT_1050_5f44 != 0xffff {
                pass1_1000_1afe(
                    ctx.DAT_1050_5f44,
                    ctx.PTR_LOOP_1050_5f2c,
                    ctx.PTR_LOOP_1050_5f2e,
                    0x1000,
                );
            }
        }
    }
    empty_fn_1000_214a();
    return ctx.PTR_LOOP_1050_5f2c;
}

pub unsafe fn mem_1000_167a(ctx: &mut AppContext, param_1: u16, param_2: u16, param_3: u16) -> u16 {
    let pu_var1: *mut u8;
    let l_var2: i32;

    if ((ctx.PTR_LOOP_1050_5f2e | ctx.PTR_LOOP_1050_5f2c) == 0x0) {
        pu_var1 = mem_op_1000_160a(ctx, param_3, param_2);
        if ((param_3 | pu_var1) == 0x0) {
            return 0x0;
        }
    }
    l_var2 = mem_op_1000_0a48(
        ctx,
        0x0,
        param_1,
        0x0,
        CONCAT22(ctx.PTR_LOOP_1050_5f2e, PTR_LOOP_1050_5f2c),
        param_2,
    );
    return l_var2;
}

pub unsafe fn mem_op_1000_179c(ctx: &mut AppContext, param_1: u16, param_2: &mut Struct19, param_3: u16) {
    let pu_var1: u16;
    let pu_var2: u16;
    pu_var1 = ctx.PTR_LOOP_1050_5f2c;
    pu_var2 = ctx.PTR_LOOP_1050_5f2e;
    if (ctx.PTR_LOOP_1050_5f2e | ctx.PTR_LOOP_1050_5f2c) == 0x0 {
        pu_var1 = mem_op_1000_160a(ctx, param_2, param_3);
        pu_var2 = param_2;
    }
    fn_ptr_op_1000_1708(ctx,param_1, 0x0, 0x0, pu_var1, pu_var2, param_3);
    return;
}

pub unsafe fn mem_op_1000_18ec(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: &mut Struct79,
    param_3: u16,
) -> u32 {
    let u_var1: u32;
    u_var1 = mem_op_1000_1902(ctx, param_1, 0x0, 0x0, 0xc, param_3, param_2);
    return u_var1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn mem_op_1000_1902(
    ctx: &mut AppContext,
    param_1: &mut u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: &mut u16,
) -> u32 {
    let p_uvar1: u16;
    let uvar2: u16;
    let bvar3: bool;
    let u_var3: u16;
    let uvar5: u16;
    let p_uvar6: *mut u16;
    let dvar7: u32;
    let u_var8: u32;
    let pu_var1: *mut u16;

    if ((param_1 & 0x8000) != 0x0) && (_SHI_INVOKEERRORHANDLER1 != -0x6f70) {
        *param_1 |= 0x1;
    }
    uvar5 = param_6;
    if true {
        loop {
            u_var3 = uvar5;
            // p_uvar1 = make_u16_ptr((*param_1 & 0xfffb | 0x1000) as u32);
            p_uvar1 = (*param_1 & 0xfffb | 0x1000);
            mem_op_1000_131c(ctx, p_uvar1, 0x100, 0x0, param_5);
            uvar5 = u_var3 | p_uvar1;
            if uvar5 != 0x0 {
                break;
            };
            uvar2 = pass1_1000_1e61(ctx,param_5, 0x2, 0x0, 0x0);
            if uvar2 == 0 {
                break;
            }
        }
        if (u_var3 | p_uvar1) != 0x0 {
            p_uvar1[0x17] = &ctx.PTR_PTR_1050_5f1a;
            p_uvar1[0x18] = ctx.data_seg;
            p_uvar1[0x15] = ctx.PTR_LOOP_1050_5f1e;
            p_uvar1[0x16] = ctx.PTR_LOOP_1050_5f20;
            p_uvar6 = p_uvar1;
            ctx.PTR_LOOP_1050_5f1e = p_uvar1;
            ctx.PTR_LOOP_1050_5f20 = u_var3;
            // TODO: refactor for loop
            // for (iVar4 = 0x5; iVar4 != 0x0; iVar4 += -0x1) {
            //   pu_var1 = p_uvar6;
            //   p_uvar6 = p_uvar6 + 0x1;
            //   *pu_var1 = 0x0;
            // }
            p_uvar1[0x5] = 0x0;
            p_uvar1[0x7] = 0x0;
            p_uvar1[0x6] = 0x0;
            p_uvar1[0x9] = 0x0;
            p_uvar1[0x8] = 0x0;
            p_uvar1[0xa] = 0xbead;
            p_uvar1[0xb] = param_1 & 0xfffd;
            p_uvar1[0xc] = 0x0;
            p_uvar1[0xd] = 0x2000;
            p_uvar1[0xe] = 0x800;
            dvar7 = mem_op_1000_1532(ctx, param_5);
            p_uvar1[0xf] = dvar7;
            p_uvar1[0x10] = (dvar7 >> 0x10);
            p_uvar1[0x12] = 0x0;
            p_uvar1[0x11] = 0x0;
            p_uvar1[0x13] = 0xfffe;
            p_uvar1[0x14] = 0xffff;
            p_uvar1[0x19] = 0x0;
            p_uvar1[0x1a] = 0x0;
            p_uvar1[0x20] = 0x0;
            p_uvar1[0x1f] = 0x0;
            bvar3 = pass1_1000_1afe(param_4, p_uvar1, u_var3);
            if bvar3 != 0x0 {
                if (param_3 | param_2) != 0x0 {
                    p_uvar6 = p_uvar1;
                    uvar5 = u_var3;
                    u_var8 = pass1_1000_52be(param_2, param_3, param_4, 0x0);
                    pass1_1000_010c(0x1, u_var8, (u_var8 >> 0x10), p_uvar6, uvar5, param_5);
                }
                return CONCAT22(u_var3, p_uvar1);
            }
            mem_op_1000_1b9a(0x0, p_uvar1, u_var3, param_5);
        }
    }
    return 0x0;
}

pub unsafe fn mem_op_1000_1b68(param_1: u16, param_2: u16, param_3: u16, param_4: u16) -> u32 {
    let uVar1: u32;

    if (param_3 + 0x14) != -0x4153 {
        pass1_1000_1e61(ctx,param_2, 0xa, 0x0, 0x0);
        return param_1 << 0x10;
    }
    uVar1 = mem_op_1000_1b9a(0x0, param_3, param_4, param_2);
    return uVar1;
}

pub unsafe fn mem_op_1000_1b9a(param_1: u16, param_2: u32, param_3: u16, param_4: u16) -> u32 {
    let uVar1: u16;
    let uVar2: u32;
    let uVar3: u16;
    let uVar4: u16;
    let iVar5: i16;
    let lVar6: i32;
    let puStack8: *mut u16;
    let uStack4: u16;

    (param_2 + 0x14) = 0x0;
    uStack4 = 0x0;
    loop {
        iVar5 = (uStack4 * 0x2);
        if (iVar5 != 0x0) {
            loop {
                uVar2 = (iVar5 + 0x8);
                (uVar2 + 0xc) = 0x0;
                mem_op_1000_13ce(param_4);
                iVar5 = (iVar5 + 0x4);
                if (uStack4 * 0x2) == iVar5 {
                    break;
                }
            }
        }
        uStack4 += 0x1;
        if uStack4 >= 0x5 {
            break;
        }
    }
    uVar4 = (param_2 + 0x12);
    uVar3 = (param_2 + 0x10);
    while (true) {
        puStack8 = CONCAT22(uVar4, uVar3);
        if ((uVar4 | uVar3) == 0x0) {
            break;
        }
        uVar1 = *puStack8;
        uVar4 = (uVar3 + 0x2);
        mem_op_1000_13ce(param_4);
        uVar3 = uVar1;
    }
    pass1_1000_20a2(param_2, param_3);
    lVar6 = mem_op_1000_13ce(param_4);
    return CONCAT22((lVar6 >> 0x10), 0x1);
}

pub fn mem_op_1000_1dfa(param_1: i16, param_2: u8, param_3: u16, param_4: u16) -> bool {
    let uVar1: u32;
    let uVar2: u16;

    if ((param_2 & 0x4) == 0x0) {
        uVar2 = (((-((param_2 & 0x2) == 0x0) >> 0x8) & 0xfe) + 0x92) << 0x8;
    } else {
        uVar2 = 0x1800;
    }
    if (((param_4 == 0x0) || (false))
        || ((param_4 & 0xff00 & (((-((param_2 & 0x4) == 0x0) >> 0x8) & 0x82) + 0x18) << 0x8)
            != uVar2))
    {
        return 0x1;
    }
    if (param_1 != 0x0) {
        uVar1 = SegmentLimit(param_4);
        if (CARRY2(param_3, param_1 - 0x1)) {
            return 0x1;
        }
        if (uVar1 < param_3 + (param_1 - 0x1)) {
            return 0x1;
        }
    }
    return 0x0;
}

pub fn mem_op_1000_21b6(param_1: u16, param_2: u16) -> bool {
    let BVar1: bool;

    BVar1 = mem_op_1000_1dfa(0x0, 0x4, param_1, param_2);
    return BVar1 == 0x0;
}

pub unsafe fn mem_1000_2bb6(
    param_1: u16,
    param_2: &mut i16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u8,
    param_8: u16,
) -> u16 {
    let piVar1: *mut i16;
    let iVar2: i16;
    let piVar3: *mut i16;
    let bVar4: u8;
    let puVar5: *mut u8;
    let puVar6: *mut u8;
    let puVar7: *mut u8;
    let uStack4: u16;
    let iStack2: i16;

    piVar3 = param_2;
    iStack2 = param_3 + 0x1;
    uStack4 = SUB42(ctx.data_seg, 0x0);
    bVar4 = (param_2 + 0x5);
    if (((bVar4 & 0x82) != 0x0) && ((bVar4 & 0x40) == 0x0)) {
        param_2[0x2] = 0x0;
        if ((bVar4 & 0x1) != 0x0) {
            if ((bVar4 & 0x10) == 0x0) {
                // TODO: goto LAB_1000_2c37;
            }
            *param_2 = param_2[0x3];
            bVar4 &= 0xfe;
        }
        (param_2 + 0x5) = bVar4 & 0xef | 0x2;
        puVar7 = (param_2 + 0xb);
        if (((bVar4 & 0x8) == 0x0)
            && ((bVar4 & 0x4) != 0x0
                || (((param_2 + 0x78) & 0x1) == 0x0
                    && ((ctx.PTR_LOOP_1050_61ec != 0x0
                        && ((param_2 == 0x621c || (param_2 == 0x6228))
                            && ((puVar7[0x5f90] & 0x40) != 0x0)))
                        || (
                            mem_1000_2ce8(ctx,param_2, param_8, param_5),
                            ((piVar3 + 0x5) & 0x8) == 0x0,
                        )))))
        {
            puVar5 = mixed_dos3_call_1000_39f2(
                puVar7,
                CONCAT22(param_6, &param_1),
                (&ctx.PTR_LOOP_1050_0000 + 0x1),
                param_4,
                param_5,
                param_6,
                param_7,
            );
            puVar6 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
        } else {
            iVar2 = piVar3[0x3];
            puVar6 = (*piVar3 - iVar2);
            *piVar3 = iVar2 + 0x1;
            piVar3[0x2] = piVar3[0x79] + -0x1;
            if (puVar6 == 0x0) {
                puVar5 = 0x0;
                if ((puVar7[0x5f90] & 0x20) != 0x0) {
                    mixed_dos3_call_1000_3636(puVar7, 0x0, 0x0, 0x2, &iStack2);
                    puVar5 = 0x0;
                    puVar6 = puVar5;
                }
            } else {
                puVar5 = mixed_dos3_call_1000_39f2(
                    puVar7,
                    CONCAT22(piVar3[0x4], piVar3[0x3]),
                    puVar6,
                    param_4,
                    param_5,
                    param_6,
                    param_7,
                );
            }
            *(piVar3 + 0x3) = param_1;
        }
        if (puVar5 == puVar6) {
            return param_1 & 0xff;
        }
    }
    //LAB_1000_2c37:
    piVar1 = piVar3 + 0x5;
    piVar1 = piVar1 | 0x20;
    return 0xffff;
}

pub unsafe fn mem_1000_2ce8(ctx: &mut AppContext, param_1: &mut i16, param_2: &mut u16, param_3: u16) {
    let piVar1: *mut i16;
    let uVar2: u16;

    uVar2 = mem_1000_167a(ctx, 0x200, param_3, param_2);
    if (param_2 == 0x0) {
        piVar1 = param_1 + 0x5;
        piVar1 = piVar1 | 0x4;
        param_1[0x79] = 0x1;
        *param_2 = ctx.data_seg;
        uVar2 = param_1 + 0xf1;
    } else {
        piVar1 = param_1 + 0x5;
        piVar1 = piVar1 | 0x8;
        param_1[0x79] = 0x200;
    }
    param_1[0x1] = param_2;
    *param_1 = uVar2;
    param_1[0x4] = param_2;
    param_1[0x3] = uVar2;
    param_1[0x2] = 0x0;
    return;
}

pub unsafe fn mixed_mem_op_1000_3c51(
    param_1: &mut HGLOBAL16,
    param_2: &mut HGLOBAL16,
    param_3: i16,
    param_4: u16,
    param_5: &mut u16,
    param_6: i16,
) -> u32 {
    let piVar1: *mut i16;
    let mut pcVar2: String;
    let mut string_1: String;
    let piVar3: *mut i16;
    let uVar4: u16;
    let flags: u16;
    let HVar5: HGLOBAL16;
    let piVar6: *mut i16;
    let mut pcVar7: String;
    let DVar8: u32;
    let HVar9: HGLOBAL16;
    let iVar10: i16;
    let iVar11: i16;

    if (((param_3 + 0x2) & 0x4) == 0x0) {
        HVar9 = (param_3 + 0x6);
        flags = 0x0;
        HVar5 = param_1;
        if (param_1 == 0x0) {
            if (false) {
                // goto LAB_1000_3cb6;
            }
            flags = 0x1;
        }
        uVar4 = 0x2;
        if (true) {
            uVar4 = 0x20;
        }
        *param_5 = ctx.s_tile2_bmp_1050_1538;
        *param_1 = GlobalReAlloc16(0x1000, CONCAT22(param_1, uVar4), flags);
        if (param_1 == 0x0) {
            //LAB_1000_3cb6:
            return CONCAT22(HVar5, param_1);
        }
        if (param_1 == HVar9) {
            *param_5 = ctx.s_tile2_bmp_1050_1538;
            *param_1 = *param_2;
            DVar8 = GlobalSize16(ctx.s_tile2_bmp_1050_1538);
            if (DVar8 != 0x0) {
                HVar5 = param_1;
                if (((HVar9 + 0x2) & 0x4) != 0x0) {
                    HVar5 = param_1 - 0x1;
                    (HVar9 - 0x2) = HVar5;
                }
                //         TODO: goto LAB_1000_3cb6;
            }
        }
    }
    iVar11 = 0x12;
    iVar10 = 0x12;
    pass1_1000_25a8(param_4, param_5);
    pass1_1000_2913(iVar10, param_4, param_5);
    string_1 = poss_str_op_1000_28dc(ctx, iVar11);
    if (string_1 != 0x0) {
        iVar10 = 0x9;
        if (*string_1 == 'M') {
            iVar10 = 0xf;
        }
        string_1 = string_1 + iVar10;
        iVar10 = 0x22;
        pcVar7 = string_1;
        loop {
            if (iVar10 == 0x0) {
                break;
            }
            iVar10 += -0x1;
            pcVar2 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
            if *pcVar2 == '\r' {
                break;
            }
        }
        pcVar7[-0x1] = '\0';
    }
    FatalAppExit16(param_5, string_1);
    FatalExit();
    piVar6 = ctx.PTR_LOOP_1050_63fe;
    loop {
        piVar1 = piVar6;
        piVar6 = piVar6 + 0x1;
        iVar10 = *piVar1;
        piVar3 = piVar6;
        if (iVar10 == param_6) || (piVar3 = (iVar10 + 0x1), piVar3 == 0x0) {
            return CONCAT22(param_6, piVar3);
        }
        iVar10 = -0x1;
        loop {
            if (iVar10 == 0x0) {
                break;
            }
            iVar10 += -0x1;
            piVar1 = piVar6;
            piVar6 = (piVar6 + 0x1);
            if *piVar1 == '\0' {
                break;
            }
        }
    }
}

pub fn free_mem_1000_407a(param_1: u16, param_2: u16, param_3: u16) {
    let uVar1: u16;
    let handle: HGLOBAL16;

    handle = 0x1000;
    if (false) {
        uVar1 = pass1_1000_41e0(param_2);
        if (uVar1 == 0x0) {
            return;
        }
        handle = ctx.s_tile2_bmp_1050_1538;
        GlobalUnlock16(0x1000);
    }
    GlobalFree16(handle);
    return;
}
