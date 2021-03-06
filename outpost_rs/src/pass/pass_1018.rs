use crate::cleanup::{
    clenaup_win_ui_1018_4d22, delete_palette_1018_e16c, destroy_win_1008_628e,
    ui_cleanup_op_1040_782c,
};
use crate::defines::{Struct11, Struct18, Struct19, Struct29, Struct79};
use crate::draw::draw_1008::unk_draw_op_1008_61b2;
use crate::draw::draw_1020::{
    draw_op_1020_041e, draw_op_1020_9364, palette_op_1020_92c4, unk_draw_op_1020_0c3e,
    unk_draw_op_1020_7f7a, win_ui_palette_op_1020_0cd2,
};
use crate::file::file_1008::{read_file_1008_7bc8, read_file_1008_7cfe, read_file_1008_7dee};
use crate::fn_ptr::fn_ptr_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708};
use crate::fn_ptr::fn_ptr_1028::fn_ptr_1030_835a;
use crate::global::AppContext;
use crate::mem_1000::{mem_op_1000_160a, mem_op_1000_179c};
use crate::misc::empty_1008_8fc4;
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::{
    pass1_1000_07fc, pass1_1000_093a, pass1_1000_0ed4, pass1_1000_3de8, pass1_1000_4906,
    pass1_1000_49b2, pass1_1000_5586,
};
use crate::pass::pass_1008::{
    pass1_1008_3e54, pass1_1008_3e76, pass1_1008_3e94, pass1_1008_3eb4, pass1_1008_3f32,
    pass1_1008_3f62, pass1_1008_4772, pass1_1008_50c2, pass1_1008_5118, pass1_1008_5134,
    pass1_1008_5784, pass1_1008_57c4, pass1_1008_5b12, pass1_1008_5bdc, pass1_1008_5c34,
    pass1_1008_612e, pass1_1008_68c6, pass1_1008_6978, pass1_1008_6c90, pass1_1008_6cec,
    pass1_1008_87cc, pass1_1008_8bc6, pass1_1008_8faa, pass1_1008_92b2, pass1_1008_941a,
    pass1_1008_9f48, pass1_1008_dfa6, pass1_1008_e320, pass1_1008_e3ec, pass1_1008_e852,
};
use crate::pass::pass_1010::{
    pass1_1010_1d80, pass1_1010_1dda, pass1_1010_1ea6, pass1_1010_1f62, pass1_1010_2b50,
    pass1_1010_2b66, pass1_1010_3c9e, pass1_1010_3cd0, pass1_1010_4f20, pass1_1010_5f7a,
    pass1_1010_5fb0, pass1_1010_65d0, pass1_1010_71d6, pass1_1010_8018, pass1_1010_81f6,
    pass1_1010_bf1e, pass1_1010_eb66, pass1_1010_ec18,
};

use crate::pass::pass_1028::{pass1_1028_d69e, pass1_1028_dc52, pass1_1028_e1ec, pass1_1028_e4ec};
use crate::pass::pass_1030::{
    pass1_1030_1d58, pass1_1030_1d7c, pass1_1030_5a52, pass1_1030_5b6c, pass1_1030_62e4,
    pass1_1030_6522, pass1_1030_6c66, pass1_1030_6d80, pass1_1030_7c28, pass1_1030_82f0,
    pass1_1030_8326, pass1_1030_8344,
};
use crate::pass::pass_1038::{pass1_1038_2a0e, pass1_1038_2a5c, pass1_1038_4e78, pass1_1038_b6e0};
use crate::resources::find_n_load_rsrc_1010_4e9e;
use crate::string::string_1000::str_op_1000_3da4;
use crate::string::string_1008::{str_op_1008_60e8, string_1008_e586};
use crate::string::string_1010::load_string_1010_84ac;
use crate::string::string_1020::string_op_1020_c2f8;
use crate::struct_ops::struct_1008::{
    clear_struct_1008_3e38, pass1_1008_c6ae, pass1_1008_c6fa, pass1_1008_ca5a, pass1_1008_caa0,
    pass1_1008_cac6, set_struct_1008_574a, struct_op_1008_8e9e,
};
use crate::struct_ops::struct_1018::{
    struct_1018_4720, struct_1018_4790, struct_1018_47c8, struct_1018_4842, struct_1018_48b0,
    struct_1018_48e8, struct_1018_4920, struct_1020_0762, struct_op_1018_4cda,
};
use crate::struct_ops::struct_1020::{set_struct_op_1020_921c, struct_1020_3644};
use crate::struct_ops::struct_1028::struct_op_1028_933c;
use crate::struct_ops::struct_1030::struct_op_1030_73a8;
use crate::switch_ops::switch_1018::{switch_1018_3b9e, switch_1018_3ee6};
use crate::sys_api::{get_sys_metrics_1018_1ea0, get_sys_metrics_1018_4b1e};
use crate::ui::ui_1008::{
    pass1_1008_adf2, pass1_1008_ae0c, pass1_1008_ae26, pass1_1008_aed8, win_1008_5c5c,
    win_1008_5c9e,
};
use crate::ui::ui_1020::post_win_msg_1020_291a;
use crate::ui::ui_1040::send_dlg_item_msg_1040_d20c;
use crate::util::{CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, SUB42, ZEXT24};
use crate::win_struct::{SEGPTR, WNDCLASS16};

pub fn pass1_1018_0000(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u32;
    let u_var4: u16;
    let iVar5: i16;
    let BVar6: bool;
    let uVar7: u16;
    let uVar8: u16;
    let local_20: [u8; 10];
    let iStack16: i16;

    // Segment:    4
    // Offset:     00024460
    // Length:     ee6a
    // Min Alloc:  ee6a
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    uVar8 = param_2;
    // uVar7 = (param_2 >> 0x10);
    read_file_1008_7cfe(uVar8, 0x2, 0x1008, param_5);
    if (param_3 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d4;
    } else {
        iVar5 = param_1;
        // u_var4 = (param_1 >> 0x10);
        BVar6 = read_file_1008_7dee(uVar8, uVar7, iVar5 + 0x16, 0x0, u_var4, 0x4, 0x1008);
        if ((((BVar6 != 0x0)
            && (
                BVar6 = read_file_1008_7dee(uVar8, uVar7, iVar5 + 0x1a, 0x0, u_var4, 0x4, 0x1008),
                BVar6 != 0x0,
            ))
            && (
                BVar6 = read_file_1008_7dee(uVar8, uVar7, iVar5 + 0x20, 0x0, u_var4, 0x4, 0x1008),
                BVar6 != 0x0,
            ))
            && ((
                BVar6 = read_file_1008_7dee(uVar8, uVar7, iVar5 + 0x24, 0x0, u_var4, 0x4, 0x1008),
                BVar6 != 0x0
                    && (
                        BVar6 = read_file_1008_7dee(
                            uVar8,
                            uVar7,
                            iVar5 + 0x30,
                            0x0,
                            u_var4,
                            0x2,
                            0x1008,
                        ),
                        BVar6 != 0x0,
                    ),
            ) && (
                BVar6 = read_file_1008_7dee(uVar8, uVar7, iVar5 + 0x32, 0x0, u_var4, 0x2, 0x1008),
                BVar6 != 0x0,
            )))
        {
            if ((iVar5 + 0x30) != 0x0) {
                i_var2 = (iVar5 + 0x32);
                if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
                    ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_4, 0x1000);
                    ctx.PTR_LOOP_1050_5f2e = param_4;
                } else {
                }
                uVar7 = fn_ptr_op_1000_1708(
                    i_var2 * 0x6,
                    0x0,
                    0x1,
                    ctx.PTR_LOOP_1050_5f2c,
                    ctx.PTR_LOOP_1050_5f2e,
                    0x1000,
                );
                (iVar5 + 0x2c) = uVar7;
                (iVar5 + 0x2e) = ctx.PTR_LOOP_1050_5f2e;
                clear_struct_1008_3e38(CONCAT22(param_5, local_20));
                // TODO: refactor for loop
                // for (iStack16 = 0x0; pi_var1 = (iVar5 + 0x30),
                //     *pi_var1 != iStack16 && iStack16 <= *pi_var1; iStack16 += 0x1) {
                //   BVar6 = read_file_1008_7bc8(param_2,CONCAT22(param_5,local_20),0x1008,
                //                               param_5);
                //   if (BVar6 == 0x0) {
                //     ctx.PTR_LOOP_1050_0310 = 0x6d0;
                //     return;
                //   }
                //   u_var3 = (iVar5 + 0x2c);
                //   pass1_1008_3f62(
                //                   (u_var3 & 0xffff0000 | (u_var3 + iStack16 * 0x6)
                //                   ),CONCAT22(param_5,local_20));
                // }
            }
            return;
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}

pub fn pass1_1018_017c(param_1: u32, param_2: u16, param_3: u16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x1e) = param_2;
    pass1_1010_1f62(param_3, param_1 & 0xffff | u_var1 << 0x10, 0x4);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_0196(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u16,
    param_5: U32Ptr,
    param_6: u16,
) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u32;
    let u_var4: u32;
    let u_var5: u16;
    let u_var6: u32;
    let iVar7: i16;
    let uVar8: u16;
    let lVar9: i32;

    pass1_1030_8344(ctx, ctx.PTR_LOOP_1050_5748, param_3);
    // uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    if ((iVar7 + 0x2c) == 0x0) {
        (iVar7 + 0x32) = 0x5;
        if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_5, 0x1000);
            ctx.PTR_LOOP_1050_5f2e = param_5;
        } else {
        }
        u_var5 = fn_ptr_op_1000_1708(
            0x1e,
            0x0,
            0x1,
            ctx.PTR_LOOP_1050_5f2c,
            ctx.PTR_LOOP_1050_5f2e,
            0x1000,
        );
    } else {
        u_var5 = (iVar7 + 0x30) + 0x1;
        ctx.PTR_LOOP_1050_5f2e = param_5;
        if (u_var5 < (iVar7 + 0x32)) {
            // goto
            // LAB_1018_022a;
        }
        pi_var1 = (iVar7 + 0x32);
        *pi_var1 = *pi_var1 + 0x5;
        u_var3 = (iVar7 + 0x2c);
        lVar9 = pass1_1000_0ed4(
            ctx,
            0x1000,
            param_6,
            0x1,
            (iVar7 + 0x32) * 0x6,
            0x0,
            u_var3,
            (u_var3 >> 0x10),
        );
        ctx.PTR_LOOP_1050_5f2e = (lVar9 >> 0x10);
        u_var5 = lVar9;
    }
    (iVar7 + 0x2c) = u_var5;
    (iVar7 + 0x2e) = ctx.PTR_LOOP_1050_5f2e;
    //LAB_1018_022a:
    pass1_1030_8344(ctx, ctx.PTR_LOOP_1050_5748, param_2);
    u_var6 = (u_var5 + 0x10);
    pass1_1030_8344(ctx, ctx.PTR_LOOP_1050_5748, u_var6);
    i_var2 = (iVar7 + 0x30);
    pi_var1 = (iVar7 + 0x30);
    *pi_var1 = *pi_var1 + 0x1;
    u_var4 = (iVar7 + 0x2c);
    pass1_1008_3f62(
        (u_var4 & 0xffff0000 | (u_var4 + i_var2 * 0x6)),
        CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var6 + 0xc),
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_028c(param_1: u32, param_2: u32, param_3: u16, param_4: u16, param_5: u16) {
    let u_var1: u32;
    let ppcVar2: u32;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let iVar5: i16;
    let u_var6: u32;
    let puVar7: U32Ptr;
    let puVar8: U32Ptr;
    let uVar9: u16;
    let extraout_dx: u16;
    let u_var10: u16;
    let iVar11: i16;
    let unaff_DI: i16;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u16;
    let uVar15: u16;
    let iStack36: i16;
    let puStack28: u32;
    let local_18: [u8; 4];
    let uStack20: u16;
    let puStack12: U32Ptr;
    let uStack8: u16;
    let uStack6: u16;
    let puStack4: U32Ptr;

    pass1_1030_8344(ctx, ctx.PTR_LOOP_1050_5748, param_2);
    uStack6 = param_3;
    puStack4 = param_4;
    uStack8 = pass1_1030_5b00(CONCAT22(param_4, param_3));
    puStack12 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, uStack8, param_5, param_4, unaff_DI);
    pass1_1008_6c90(CONCAT22(param_5, local_18));
    pass1_1018_0b1e(puStack12, CONCAT22(param_5, local_18), param_5);
    puVar7 = (uStack20 >> 0xf);
    if ((puVar7 | uStack20) == 0x0) {
        pu_var3 = local_18;
        pass1_1030_6522(
            ctx.PTR_LOOP_1050_5740,
            CONCAT22(param_5, pu_var3),
            param_2,
            param_5,
        );
    } else {
        pu_var3 = local_18;
        pass1_1030_62e4(
            ctx.PTR_LOOP_1050_5740,
            CONCAT22(param_5, pu_var3),
            param_2,
            param_5,
        );
    }
    puStack28 = CONCAT22(puVar7, pu_var3);
    u_var4 = puVar7 | pu_var3;
    if (u_var4 == 0x0) {
        return;
    }
    puVar8 = puVar7;
    pass1_1018_04f2(param_1);
    uVar14 = 0x1c;
    uVar13 = 0x1000;
    mem_op_1000_179c(0x1c, puVar8, 0x1000);
    uVar9 = puVar8 | u_var4;
    iVar11 = param_1;
    // uVar12 = (param_1 >> 0x10);
    uVar15 = u_var4;
    if (uVar9 == 0x0) {
        (iVar11 + 0x12) = 0x0;
    } else {
        uVar13 = 0x1008;
        struct_op_1008_8e9e(CONCAT22(puVar8, u_var4), 0x6, 0x24);
        (iVar11 + 0x12) = u_var4;
        (iVar11 + 0x14) = uVar9;
    }
    ppcVar2 = (*puStack28 + 0x10);
    (**ppcVar2)(uVar13, pu_var3, puVar7, uVar14, uVar15);
    // TODO: refactor for loop
    // for (iStack36 = 0x0; iStack36 < u_var4; iStack36 += 0x1) {
    //   u_var6 = SEXT24(iStack36);
    //   ppcVar2 = (*puStack28 + 0x4);
    //   (**ppcVar2)();
    //   if ((extraout_dx | u_var6) != 0x0) {
    //     iVar5 = iStack36 / 0x6;
    //     u_var10 = iStack36 % 0x6;
    //     u_var1 = (iVar11 + 0xe);
    //     pass1_1018_dd7c(u_var1,(u_var1 >> 0x10),
    //                     CONCAT22(iStack36 % 0x6,iVar5),
    //                     u_var6 & 0xffff | extraout_dx << 0x10,u_var10,param_5);
    //     pass1_1008_8faa((iVar11 + 0x12),CONCAT22(u_var10,iVar5));
    //   }
    // }
    return;
}

pub fn pass1_1018_03ea(param_1: u32, param_2: i16, param_3: u16) {
    if (param_2 != 0x5) {
        return;
    }
    pass1_1010_1f62(param_3, param_1 & 0xffff0000 | (param_1 - 0xa), 0x5);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_0412(
    param_1: u32,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u32,
    param_6: u16,
    param_7: u8,
) {
    let pu_var1: U32Ptr;
    let local_128: [u8; 124];
    let uStack4: u16;

    uStack4 = 0x0;
    if (((0x72 < param_4) && (!SBORROW2(param_4, 0x73)))
        && (param_4 == 0x75
            || (param_4 - 0x74) < 0x1
            || (0x0 < (param_4 - 0x76) && ((param_4 - 0x77) < 0x2))))
    {
        uStack4 = 0x1;
    }
    struct_op_1028_933c(
        CONCAT22(param_6, local_128),
        param_2,
        uStack4,
        param_4,
        param_3,
        (param_3 >> 0x10),
        (param_1 + 0x24),
        param_5,
        param_6,
        param_7,
    );
    pu_var1 = local_128;
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_6, pu_var1));
    if (pu_var1 != 0x0) {
        pass1_1010_1f62(param_6, param_1, 0x6);
    }
    return;
}

pub fn pass1_1018_04a4(param_1: u32, param_2: u32) {
    (param_1 + 0x16) = param_2;
    return;
}

pub fn pass1_1018_04b8(param_1: u32) -> u32 {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x18), (param_1 + 0x16));
}

pub fn pass1_1018_04ca(param_1: u32, param_2: u32) {
    (param_1 + 0x1a) = param_2;
    return;
}

pub fn pass1_1018_04de(param_1: u32, param_2: u32) {
    (param_1 + 0x20) = param_2;
    return;
}

pub fn pass1_1018_04f2(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 0x12);
    u_var2 = (i_var4 + 0x14);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    (i_var4 + 0x12) = 0x0;
    return;
}

pub fn pass1_1018_0526(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xa));
    pass1_1010_eb66(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_078e(param_1: U32Ptr, param_2: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let u_var5: &mut Struct500;
    let u_var6: u16;
    let puStack26: U32Ptr;
    let paStack6: &mut Struct18;

    // u_var6 = (param_1 >> 0x10);
    u_var5 = param_1;
    *param_1 = 0x1874;
    u_var5.field_0x2 = 0x1018;
    u_var5.field_0x20 = 0x18b0;
    u_var5.field_0x22 = 0x1018;
    ctx.PTR_LOOP_1050_3960 = ctx.PTR_LOOP_1050_3960 + -0x1;
    (ctx.PTR_LOOP_1050_3962 + u_var5.field_0x12 * 0x2 + -0x4) = 0x0;
    if (ctx.PTR_LOOP_1050_3960 == 0x0) {
        fn_ptr_1000_17ce(ctx, ctx.PTR_LOOP_1050_3962, 0x1000);
        ctx.PTR_LOOP_1050_3962 = 0x0;
    }
    fn_ptr_1000_17ce(ctx, u_var5.field_0x94, 0x1000);
    fn_ptr_1000_17ce(ctx, u_var5.field_0x9a, 0x1000);
    fn_ptr_1000_17ce(ctx, u_var5.field_0x88, 0x1000);
    fn_ptr_1000_17ce(ctx, u_var5.field_0x8e, 0x1000);
    if (u_var5.field_0x2c != 0x0) {
        if (param_1 == 0x0) {
            pu_var3 = 0x0;
            u_var4 = 0x0;
        } else {
            pu_var3 = &u_var5.field_0x20;
            u_var4 = u_var6;
        }
        pass1_1010_1ea6(u_var5.field_0x2c, CONCAT22(u_var4, pu_var3), param_2);
    }
    if (u_var5.field_0x9e != 0x0) {
        if (param_1 == 0x0) {
            pu_var3 = 0x0;
            u_var4 = 0x0;
        } else {
            pu_var3 = &u_var5.field_0x20;
            u_var4 = u_var6;
        }
        pass1_1010_1ea6(u_var5.field_0x9e, CONCAT22(u_var4, pu_var3), param_2);
    }
    u_var1 = u_var5.field_0x60;
    u_var2 = u_var5.field_0x62;
    paStack6 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0x0) {
        pass1_1008_5118(CONCAT22(u_var2, u_var1));
        fn_ptr_1000_17ce(ctx, paStack6, 0x1000);
    }
    u_var5.field_0x4c = 0x0;
    if (param_1 == 0x0) {
        pu_var3 = 0x0;
        u_var6 = 0x0;
    } else {
        pu_var3 = &u_var5.field_0x20;
    }
    puStack26 = CONCAT22(u_var6, pu_var3);
    *puStack26 = 0x389a;
    pu_var3[0x1] = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_0902(param_1: U32Ptr, param_2: u32) {
    let u_var1: u32;
    let ppcVar2: u32;
    astruct_76 * *ppaVar3;
    astruct_76 * *ppaVar4;
    let iVar5: i16;
    let u_var6: u16;
    let uVar7: u32;
    let puVar8: u32;
    let puVar9: u32;

    puVar9 = (param_1 & 0xffff0000 | (param_1 + 0x28));
    ppaVar3 = (param_1 + 0x24);
    puVar8 = (param_1 & 0xffff0000 | ZEXT24(ppaVar3));
    u_var6 = param_1._2_2_;
    ppaVar4 = ppaVar3;
    pass1_1030_8344(ctx, ctx.PTR_LOOP_1050_5748, param_2);
    pass1_1030_5a52(CONCAT22(u_var6, ppaVar4), puVar8, puVar9);
    uVar7 = pass1_1008_4772(*ppaVar3);
    (param_1 + 0x5a) = uVar7;
    (param_1 + 0x5c) = (uVar7 >> 0x10);
    iVar5 = pass1_1018_17f0();
    (param_1 + 0x12) = iVar5 + 0x2;
    (iVar5 * 0x2 + ctx.PTR_LOOP_1050_3962) = 0x1;
    ppcVar2 = (*param_1 + 0x18);
    (**ppcVar2)();
    (param_1 + 0x3c) = param_2;
    u_var1 = (param_1 + 0x2c);
    uVar7 = pass1_1010_ec18(
        u_var1,
        (u_var1 >> 0x10),
        param_2 & 0xffff0000 | (param_1 + 0x3c),
        param_2,
        param_2._2_2_,
    );
    (param_1 + 0x7c) = uVar7;
    (param_1 + 0x7e) = (uVar7 >> 0x10);
    return;
}

pub fn pass1_1018_0a50(param_1: u32) -> u32 {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x84) == 0x2) {
        return CONCAT22((i_var1 + 0x2a), (i_var1 + 0x28));
    }
    return CONCAT22((i_var1 + 0x26), (i_var1 + 0x24));
}

pub fn pass1_1018_0a76(param_1: u32, param_2: u16) {
    let u_var1: u16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x84) == 0x1) {
        u_var1 = 0x2;
    } else {
        u_var1 = 0x1;
    }
    (param_1 + 0x84) = u_var1;
    pass1_1010_1f62(param_2, param_1 & 0xffff | u_var2 << 0x10, 0x4);
    return;
}

pub fn pass1_1018_0aa0(param_1: u32, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x14) = param_2;
    pass1_1018_04de((i_var1 + 0x2c), (i_var1 + 0x3c));
    return;
}

pub fn pass1_1018_0ac0(param_1: u32, param_2: u32) {
    (param_1 + 0x80) = param_2;
    return;
}

pub fn pass1_1018_0ad4(param_1: u32) -> u32 {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x82), (param_1 + 0x80));
}

pub fn pass1_1018_0ae8(param_1: u32, param_2: u16) {
    (param_1 + 0x5e) = param_2;
    return;
}

pub fn pass1_1018_0afa(param_1: u32) -> u16 {
    return (param_1 + 0x5e);
}

pub fn pass1_1018_0b08(param_1: u32) -> u32 {
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;

    u_var1 = (param_1 + 0x7c);
    // u_var3 = (u_var1 >> 0x10);
    i_var2 = u_var1;
    return CONCAT22((i_var2 + 0x6), (i_var2 + 0x4));
}

pub fn pass1_1018_0b1e(param_1: U32Ptr, param_2: U32Ptr, param_3: u16) {
    let i_var1: i16;
    let u_var2: u32;
    let i_var3: &mut Struct74;
    let u_var3: u16;
    let local_8: u16;
    let local_6: i16;
    let local_4: i16;

    i_var3 = param_1;
    i_var3 = &i_var3.field_0x30;
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | ZEXT24(i_var3)),
        CONCAT22(param_3, &local_8),
        CONCAT22(param_3, &local_6),
        CONCAT22(param_3, &local_4),
    );
    if (local_4 + -0x3 < 0x1) {
        local_4 = 0x3;
    }
    if (local_6 + -0x3 < 0x1) {
        local_6 = 0x3;
    }
    // u_var3 = (param_1 >> 0x10);
    u_var2 = i_var3.field_0x5a;
    i_var1 = (u_var2 + 0x4);
    if (i_var1 <= local_4 + 0x2) {
        local_4 = i_var1 + -0x3;
    }
    u_var2 = i_var3.field_0x5a;
    i_var1 = (u_var2 + 0x8);
    if (i_var1 <= local_6 + 0x2) {
        local_6 = i_var1 + -0x3;
    }
    pass1_1008_6cec(
        (param_1 & 0xffff0000 | &i_var3.field_0x40),
        local_8,
        CONCAT22(local_4 + 0x2, local_6 + 0x2),
        local_8,
        CONCAT22(local_4 + -0x3, local_6 + -0x3),
    );
    pass1_1008_3f62(param_2, (param_1 & 0xffff0000 | &i_var3.field_0x40));
    pass1_1008_3f62(
        (param_2 & 0xffff0000 | (param_2 + 0x6)),
        (param_1 & 0xffff0000 | &i_var3.field_0x46),
    );
    return;
}

pub fn pass1_1018_0bf4(param_1: u16, param_2: i16, param_3: u32, param_4: i16) {
    let u_var1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let lVar4: i32;
    let u_var5: u16;
    let local_14: [u8; 0xc];
    let local_8: u16;
    let local_6: u32;

    if (false) {
        return;
    }
    match (param_4) {
        0x1 => {
            (param_3 + 0xc) = 0x0;
            (param_3 + 0x7e) = 0x0;
            return;
        }
        0x4 => {
            pass1_1008_3eb4(
                (param_3 & 0xffff0000 | (param_3 + 0x10)),
                CONCAT22(param_1, &local_8),
                CONCAT22(param_1, &local_6),
                CONCAT22(param_1, &local_6 + 0x2),
            );
            u_var1 = (param_3 + 0xc);
            local_8 = (u_var1 + 0x1e);
            pass1_1008_3e76(
                (param_3 & 0xffff0000 | (param_3 + 0x10)),
                local_8,
                local_6,
                (local_6 >> 0x10),
            );
            pass1_1008_6c90(CONCAT22(param_1, local_14));
            pass1_1018_0b1e(
                (param_3 & 0xffff0000 | (param_3 - 0x20)),
                CONCAT22(param_1, local_14),
                param_1,
            );
        }
        //     TODO: goto LAB_1018_0c71;
        0x5 => {}
        0x6 => {
            u_var2 = param_3 - 0x20;
            u_var5 = param_3._2_2_;
            pass1_1018_0dc6(param_3 & 0xffff0000 | u_var2, param_1);
            pass1_1018_10c4(param_1, u_var5, param_3 & 0xffff0000 | u_var2);
            pass1_1018_1346(param_1, u_var5, (param_3 & 0xffff0000 | u_var2));
            //LAB_1018_0c71:
            (param_3 + 0x2c) = 0x0;
            lVar4 = (param_3 + 0x1c);
            u_var3 = (param_3 + 0x1e);
            u_var1 = (param_3 + 0xc);
            if ((u_var1 + 0x20) == lVar4) {
                pass1_1018_028c((param_3 + 0xc), (param_3 + 0x1c), lVar4, u_var3, param_1);
                (param_3 + 0x2c) = lVar4;
                (param_3 + 0x2e) = u_var3;
                pass1_1010_1f62(param_1, param_3 & 0xffff0000 | (param_3 - 0x20), param_4);
                return;
            }
        }

        0x14 => {
            u_var1 = (param_3 + 0xc);
            if ((u_var1 + 0x20) != (param_3 + 0x1c)) {
                post_win_msg_1020_291a(0x1020);
                return;
            }
        }
        0x15 => {
            u_var3 = pass1_1010_65d0(param_1, (param_3 + 0x7e), 0x88);
            if (u_var3 != 0x0) {
                (param_3 + 0x88) = 0x1;
                return;
            }
        }
    }
    return;
}

pub fn pass1_1018_0d76(param_1: u32) {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x2c);
    if ((u_var1 + 0x20) == (param_1 + 0x3c)) {
        return;
    }
    return;
}

pub fn pass1_1018_0d9a(param_1: u32, param_2: U32Ptr, param_3: U32Ptr) {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x7c);
    *param_3 = (u_var1 + 0x16);
    u_var1 = (param_1 + 0x7c);
    *param_2 = (u_var1 + 0x1a);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_0dc6(param_1: u32, param_2: u16) {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let i_var4: i16;
    let pu_var5: u32;
    let paVar6: &mut Struct18;
    let in_DX: U32Ptr;
    let puVar7: U32Ptr;
    let puVar8: U32Ptr;
    let uVar9: u16;
    let iVar13: &mut Struct91;
    let u_var10: u16;
    let local_32: u32;
    let uStack46: u16;
    let uStack44: u32;
    let paStack40: &mut Struct18;
    let paStack36: &mut Struct18;
    let paStack32: &mut Struct18;
    let uStack28: u32;
    let uStack24: u32;
    let local_14: [u8; 8];
    let uStack12: u16;
    let uStack10: u16;
    let uStack8: u16;
    let uStack6: u16;
    let i_stack4: i16;

    pass1_1028_dc52(CONCAT22(param_2, local_14), 0x1, 0x0, 0x400);
    // u_var10 = (param_1 >> 0x10);
    iVar13 = param_1;
    paStack36 = iVar13.field_0x94;
    fn_ptr_1000_17ce(ctx, paStack36, 0x1000);
    paStack40 = iVar13.field_0x9a;
    paStack32 = paStack40;
    fn_ptr_1000_17ce(ctx, paStack40, 0x1000);
    iVar13.field_0x94 = 0x0;
    iVar13.field_0x9a = 0x0;
    iVar13.field_0x92 = 0x0;
    iVar13.field_0x98 = 0x0;
    loop {
        pu_var2 = local_14;
        pass1_1028_e4ec(CONCAT22(param_2, pu_var2));
        uStack24 = CONCAT22(in_DX, pu_var2);
        puVar7 = (in_DX | pu_var2);
        if (puVar7 == 0x0) {
            break;
        }
        paVar6 = (pu_var2 + 0x200);
        paStack40 = paVar6;
        if (paVar6 == 0x8000001) {
            pu_var1 = &iVar13.field_0x92;
            *pu_var1 = *pu_var1 + 0x1;
            in_DX = puVar7;
        } else {
            if ((iVar13.field_0xa8 != 0x0)
                || (
                    pass1_1008_dfa6(iVar13.field_0xa2, (pu_var2 + 0x4), 0x4000001, param_2),
                    in_DX = puVar7,
                    paVar6 != 0x0,
                ))
            {
                in_DX = puVar7;
                pu_var1 = &iVar13.field_0x98;
                *pu_var1 = *pu_var1 + 0x1;
            }
        }
    }
    puVar8 = puVar7;
    if (iVar13.field_0x92 != 0x0) {
        uVar9 = iVar13.field_0x92;
        uStack44 = uStack44 & 0xffff0000 | uVar9;
        u_var3 = uVar9 * 0x6;
        mem_op_1000_179c(u_var3, 0x0, 0x1000);
        paStack32 = CONCAT22(puVar7, u_var3);
        puVar8 = (puVar7 | u_var3);
        if (puVar8 == 0x0) {
            iVar13.field_0x94 = 0x0;
        } else {
            pass1_1000_5586(0x3e38, 0x1008, uStack44, 0x6, u_var3, puVar7);
            iVar13.field_0x94 = paStack32;
        }
    }
    if (iVar13.field_0x98 != 0x0) {
        uVar9 = iVar13.field_0x98;
        uStack44 = uStack44 & 0xffff0000 | uVar9;
        u_var3 = uVar9 * 0x6;
        mem_op_1000_179c(u_var3, puVar8, 0x1000);
        paStack32 = CONCAT22(puVar8, u_var3);
        if ((puVar8 | u_var3) == 0x0) {
            iVar13.field_0x9a = 0x0;
        } else {
            pass1_1000_5586(0x3e38, 0x1008, uStack44, 0x6, u_var3, puVar8);
            iVar13.field_0x9a = paStack32;
        }
    }
    if (i_stack4 != 0x0) {
        uStack8 = 0x1;
        uStack6 = 0x0;
    }
    uStack28 = 0x0;
    uStack12 = uStack8;
    uStack10 = uStack6;
    //LAB_1018_0f74:
    pu_var2 = local_14;
    pass1_1028_e4ec(CONCAT22(param_2, pu_var2));
    uStack24 = CONCAT22(uStack6, pu_var2);
    uVar9 = uStack6 | pu_var2;
    if (uVar9 == 0x0) {
        return;
    }
    uStack44 = (pu_var2 + 0x200);
    paVar6 = (pu_var2 + 0x10);
    paStack40 = paVar6;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, paVar6);
    paStack36 = (paVar6 & 0xffff | uVar9 << 0x10);
    local_32 = (paVar6 + 0xc);
    uStack46 = (paVar6 + 0x10);
    pu_var5 = &local_32;
    if (uStack44 != 0x8000001) {
        // goto
        // LAB_1018_0ffc;
    }
    i_var4 = &iVar13.field_0x94;
    uStack6 = (&iVar13.field_0x94 + 0x2);
    uStack28 = uStack28 & 0xffff | (uStack28._2_2_ + 0x1) << 0x10;
    //   TODO: goto LAB_1018_0fe8;
    //LAB_1018_0ffc:
    if ((iVar13.field_0xa8 != 0x0)
        || (
            pass1_1008_dfa6(iVar13.field_0xa2, (uStack24 + 0x4), 0x4000001, param_2),
            uStack6 = uVar9,
            pu_var5 != 0x0,
        ))
    {
        i_var4 = &iVar13.field_0x9a;
        uStack6 = (&iVar13.field_0x9a + 0x2);
        uStack28 = uStack28 & 0xffff0000 | (uStack28 + 0x1);
        uStack28._2_2_ = uStack28;
        //LAB_1018_0fe8:
        pass1_1008_3f62(
            CONCAT22(uStack6, i_var4 + uStack28._2_2_ * 0x6),
            CONCAT22(param_2, &local_32),
        );
    }
    //   TODO: goto LAB_1018_0f74;
}

pub fn pass1_1018_1054(param_1: u32, param_2: U32Ptr, param_3: U32Ptr, param_4: u16) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x94) == 0x0) {
        pass1_1018_0dc6(param_1 & 0xffff | u_var2 << 0x10, param_4);
    }
    *param_3 = (i_var1 + 0x94);
    *param_2 = (i_var1 + 0x92);
    return;
}

pub fn pass1_1018_108c(param_1: u32, param_2: U32Ptr, param_3: U32Ptr, param_4: u16) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x9a) == 0x0) {
        pass1_1018_0dc6(param_1 & 0xffff | u_var2 << 0x10, param_4);
    }
    *param_3 = (i_var1 + 0x9a);
    *param_2 = (i_var1 + 0x98);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_10c4(param_1: u16, param_2: u16, param_3: u32) {
    let u_var1: u32;
    let ppcVar2: u32;
    let u_var3: u32;
    let i_var4: i16;
    let pu_var5: U32Ptr;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u32;
    let uVar9: u16;
    let puVar10: U32Ptr;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let extraout_DX_02: u16;
    let iVar11: i16;
    let uVar12: u16;
    let uVar13: u8;
    let bVar14: bool;
    let puVar15: u32;
    let uStack60: u32;
    let uStack56: u32;
    let uStack52: u32;
    let puStack48: u32;
    let puStack40: u32;
    let uStack30: u16;
    let uStack28: u16;
    let local_16: [u8; 8];
    let uStack14: u16;
    let uStack12: u16;
    let uStack10: u16;
    let uStack8: u16;
    let i_stack6: i16;
    let i_stack4: i16;

    // uVar12 = (param_3 >> 0x10);
    iVar11 = param_3;
    i_stack4 = (iVar11 + 0x86);
    fn_ptr_1000_17ce(ctx, (iVar11 + 0x88), 0x1000);
    (iVar11 + 0x86) = 0x0;
    (iVar11 + 0x88) = 0x0;
    pass1_1028_dc52(
        CONCAT13((param_1 >> 0x8), CONCAT12(param_1, local_16)),
        0x1,
        0x0,
        0x400,
    );
    uStack30 = 0x0;
    uStack28 = 0x0;
    loop {
        uVar9 = param_2;
        pu_var5 = local_16;
        pass1_1028_e4ec(CONCAT22(param_1, pu_var5));
        param_2 = uVar9 | pu_var5;
        if (param_2 == 0x0) {
            break;
        }
        if ((iVar11 + 0x3c) == (pu_var5 + 0x8)) {
            puVar15 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x2);
            // puVar10 = (puVar15 >> 0x10);
            u_var6 = puVar15;
            pass1_1038_4e78(u_var6, puVar10, CONCAT22(uVar9, pu_var5), puVar15);
            puStack48 = CONCAT22(puVar10, u_var6);
            u_var3 = *puStack48;
            ppcVar2 = u_var3 + 0x8;
            uVar9 = u_var6;
            (**ppcVar2)(&ctx.PTR_LOOP_1050_1038, u_var6, puVar10);
            bVar14 = CARRY2(uStack30, uVar9);
            uStack30 += uVar9;
            uStack28 = uStack28 + extraout_dx + bVar14;
            param_2 = extraout_dx;
            if (puStack48 != 0x0) {
                ppcVar2 = u_var3;
                (**ppcVar2)(0x38, u_var6, puVar10, 0x1);
                param_2 = extraout_DX_00;
            }
        }
    }
    if ((uStack28 | uStack30) != 0x0) {
        (iVar11 + 0x86) = uStack30;
        uVar7 = uStack30 * 0x6;
        mem_op_1000_179c(uVar7, 0x0, 0x1000);
        uStack52 = CONCAT22(param_2, uVar7);
        if ((param_2 | uVar7) == 0x0) {
            (iVar11 + 0x88) = 0x0;
        } else {
            pass1_1000_5586(0x3e38, 0x1008, uStack30, 0x6, uVar7, param_2);
            (iVar11 + 0x88) = uStack52;
        }
        if (i_stack6 != 0x0) {
            uStack10 = 0x1;
            uStack8 = 0x0;
        }
        i_var4 = 0x0;
        uStack14 = uStack10;
        uStack12 = uStack8;
        loop {
            uVar9 = uStack8;
            pu_var5 = local_16;
            pass1_1028_e4ec(CONCAT22(param_1, pu_var5));
            if ((uVar9 | pu_var5) == 0x0) {
                break;
            }
            uStack8 = uVar9 | pu_var5;
            if ((iVar11 + 0x3c) == (pu_var5 + 0x8)) {
                puVar15 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x2);
                // puVar10 = (puVar15 >> 0x10);
                u_var6 = puVar15;
                uVar13 = 0x38;
                pass1_1038_4e78(
                    u_var6,
                    puVar10,
                    CONCAT13((uVar9 >> 0x8), CONCAT12(uVar9, pu_var5)),
                    puVar15,
                );
                puStack40 = CONCAT22(puVar10, u_var6);
                ppcVar2 = (*puStack40 + 0x10);
                uVar9 = u_var6;
                (**ppcVar2)(0x38, u_var6, puVar10);
                uStack56 = CONCAT22(extraout_DX_01, uVar9);
                uStack8 = extraout_DX_01;
                // TODO: refactor for loop
                // for (uStack60 = 0x0; uStack60 < uStack56; uStack60 += 0x1) {
                //   uVar8 = uStack56;
                //   pass1_1030_1d58(puStack40);
                //   u_var1 = (iVar11 + 0x88);
                //   uVar13 = 0x8;
                //   pass1_1008_3f62(
                //                   (u_var1 & 0xff000000 |
                //                   CONCAT12((u_var1 >> 0x10),u_var1 + i_var4 * 0x6))
                //                   ,CONCAT22(uStack8,uVar8 + 0xc));
                //   i_var4 += 0x1;
                // }
                if (puStack40 != 0x0) {
                    ppcVar2 = *puStack40;
                    (**ppcVar2)(uVar13, u_var6, puVar10, 0x1);
                    uStack8 = extraout_DX_02;
                }
            }
        }
        if ((iVar11 + 0x86) != i_stack4) {
            pass1_1010_1f62(param_1, param_3, 0x6);
        }
    }
    return;
}

pub fn pass1_1018_1320(param_1: u32, param_2: U32Ptr, param_3: U32Ptr) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x88);
    *param_2 = (param_1 + 0x86);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_1346(param_1: u16, param_2: u16, param_3: &mut Struct93) {
    let ppcVar1: u32;
    let i_var2: i16;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let uVar7: u16;
    let extraout_DX_02: u16;
    let i_var9: &mut Struct93;
    let uVar8: u16;
    let uVar9: u8;
    let puVar10: u32;
    let u_var11: u32;
    let uVar12: u32;
    let uStack70: u32;
    let puStack56: u32;
    let uStack52: u32;
    let puStack48: u32;
    let uStack30: u32;
    let local_16: [u8; 8];
    let uStack14: u16;
    let uStack12: u16;
    let uStack10: u16;
    let uStack8: u16;
    let i_stack6: i16;
    let uStack4: u16;

    // uVar8 = (param_3 >> 0x10);
    i_var9 = param_3;
    uStack4 = i_var9.field_0x8c;
    fn_ptr_1000_17ce(ctx, i_var9.field_0x8e, 0x1000);
    i_var9.field_0x8c = 0x0;
    i_var9.field_0x8e = 0x0;
    pass1_1028_dc52(
        CONCAT13((param_1 >> 0x8), CONCAT12(param_1, local_16)),
        0x1,
        0x0,
        0x400,
    );
    uStack30 = 0x0;
    loop {
        uVar7 = param_2;
        pu_var3 = local_16;
        pass1_1028_e4ec(CONCAT22(param_1, pu_var3));
        param_2 = uVar7 | pu_var3;
        if (param_2 == 0x0) {
            break;
        }
        if (i_var9.field_0x3c == (pu_var3 + 0x8)) {
            puVar10 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x2);
            // puVar6 = (puVar10 >> 0x10);
            u_var4 = puVar10;
            uVar9 = 0x38;
            pass1_1038_4e78(u_var4, puVar6, CONCAT22(uVar7, pu_var3), puVar10);
            puStack48 = CONCAT22(puVar6, u_var4);
            ppcVar1 = (*puStack48 + 0x10);
            uVar7 = u_var4;
            (**ppcVar1)(&ctx.PTR_LOOP_1050_1038, u_var4, puVar6);
            uStack52 = CONCAT22(extraout_dx, uVar7);
            param_2 = extraout_dx;
            // TODO: refactor for loop
            // for (puStack56 = 0x0; puStack56 < uStack52;
            //     puStack56 = (puStack56 + 0x1)) {
            //   uVar9 = 0x30;
            //   u_var11 = pass1_1030_1d7c(uVar7,param_2,puStack48);
            //   param_2 = (u_var11 >> 0x10);
            //   if ((u_var11 + 0x12) == 0x9) {
            //     uStack30 += 0x1;
            //   }
            // }
            if (puStack48 != 0x0) {
                ppcVar1 = *puStack48;
                (**ppcVar1)(uVar9, u_var4, puVar6, 0x1);
                param_2 = extraout_DX_00;
            }
        }
    }
    if ((uStack30._2_2_ | uStack30) != 0x0) {
        i_var9.field_0x8c = uStack30;
        u_var5 = uStack30 * 0x6;
        mem_op_1000_179c(u_var5, 0x0, 0x1000);
        uStack70 = CONCAT22(param_2, u_var5);
        if ((param_2 | u_var5) == 0x0) {
            i_var9.field_0x8e = 0x0;
        } else {
            pass1_1000_5586(0x3e38, 0x1008, uStack30, 0x6, u_var5, param_2);
            i_var9.field_0x8e = uStack70;
        }
        if (i_stack6 != 0x0) {
            uStack10 = 0x1;
            uStack8 = 0x0;
        }
        i_var2 = 0x0;
        uStack14 = uStack10;
        uStack12 = uStack8;
        loop {
            uVar7 = uStack8;
            pu_var3 = local_16;
            pass1_1028_e4ec(CONCAT22(param_1, pu_var3));
            if ((uVar7 | pu_var3) == 0x0) {
                break;
            }
            uStack8 = uVar7 | pu_var3;
            if (i_var9.field_0x3c == (pu_var3 + 0x8)) {
                puVar10 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x2);
                // puVar6 = (puVar10 >> 0x10);
                u_var4 = puVar10;
                uVar9 = 0x38;
                pass1_1038_4e78(
                    u_var4,
                    puVar6,
                    CONCAT13((uVar7 >> 0x8), CONCAT12(uVar7, pu_var3)),
                    puVar10,
                );
                puStack56 = CONCAT22(puVar6, u_var4);
                ppcVar1 = (*puStack56 + 0x10);
                uVar7 = u_var4;
                (**ppcVar1)(0x38, u_var4, puVar6);
                uStack52 = CONCAT22(extraout_DX_01, uVar7);
                uStack8 = extraout_DX_01;
                // TODO: refactor for loop
                // for (puStack48 = 0x0; puStack48 < uStack52;
                //     puStack48 = (puStack48 + 0x1)) {
                //   u_var11 = uStack52;
                //   pass1_1030_1d58(puStack56);
                //   uVar9 = 0x30;
                //   uVar12 = struct_op_1030_73a8(u_var11 & 0xffff | uStack8 << 0x10);
                //   uVar7 = (uVar12 >> 0x10);
                //   if ((uVar12 + 0x12) == 0x9) {
                //     uVar12 = i_var9.field_0x8e;
                //     uVar9 = 0x8;
                //     pass1_1008_3f62(
                //                     (uVar12 & 0xff000000 |
                //                     CONCAT12((uVar12 >> 0x10),
                //                                     uVar12 + i_var2 * 0x6)),
                //                     CONCAT22(uStack8,u_var11 + 0xc));
                //     i_var2 += 0x1;
                //   }
                //   uStack8 = uVar7;
                // }
                if (puStack56 != 0x0) {
                    ppcVar1 = *puStack56;
                    (**ppcVar1)(uVar9, u_var4, puVar6, 0x1);
                    uStack8 = extraout_DX_02;
                }
            }
        }
        if (i_var9.field_0x8c != uStack4) {
            pass1_1010_1f62(param_1, param_3, 0x6);
        }
    }
    return;
}

pub fn pass1_1018_15f6(param_1: u32, param_2: U32Ptr, param_3: U32Ptr) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x8e);
    *param_2 = (param_1 + 0x8c);
    return;
}

pub fn pass1_1018_161c(param_1: u16, param_2: u32, param_3: U32Ptr, param_4: i16, param_5: i16) {
    let u_var1: u16;
    let u_var2: u16;
    let local_6: u32;

    pass1_1008_3e94(
        (param_2 & 0xffff0000 | (param_2 + 0x36)),
        CONCAT22(param_1, &local_6),
        CONCAT22(param_1, &local_6 + 0x2),
    );
    u_var1 = local_6._2_2_ + param_5 + -0x3;
    u_var2 = local_6 + param_4 + -0x3;
    local_6 = CONCAT22(u_var1, u_var2);
    pass1_1008_3e76(param_3, (param_2 + 0x44), u_var2, u_var1);
    return;
}

pub fn pass1_1018_1662(param_1: u32, param_2: i16, param_3: i16, param_4: u16) {
    let local_6: i16;
    let local_4: i16;

    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (param_1 + 0x36)),
        CONCAT22(param_4, &local_6),
        CONCAT22(param_4, &local_4),
    );
    pass1_1018_16b8(
        param_1,
        (param_1 + 0x44),
        CONCAT22(local_4 + param_3, local_6 + param_2),
        param_4,
    );
    return;
}

pub fn pass1_1018_169e(param_1: u32, param_2: u32, param_3: u16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    pass1_1018_16b8(
        param_1 & 0xffff | u_var1 << 0x10,
        (param_1 + 0x44),
        param_2,
        param_3,
    );
    return;
}

// WARNING: Unable to use type for symbol u_var2

pub fn pass1_1018_16b8(param_1: u32, param_2: u16, param_3: u32, param_4: u16) {
    let i_var1: i16;
    let u_var3: u32;
    let lVar4: i32;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let uVar8: u16;
    let local_6: [u8; 2];
    let local_4: [u8; 2];
    let u_var2: u32;

    if (param_3._2_2_ + -0x3 < 0x1) {
        param_3 = CONCAT22(0x3, param_3);
    }
    if (param_3 + -0x3 < 0x1) {
        param_3 = CONCAT22(param_3._2_2_, 0x3);
    }
    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    u_var2 = (iVar6 + 0x5a);
    i_var1 = (u_var2 + 0x4);
    if (i_var1 <= param_3._2_2_ + 0x2) {
        param_3 = param_3 & 0xffff | (i_var1 - 0x3) << 0x10;
    }
    u_var3 = (iVar6 + 0x5a);
    i_var1 = (u_var3 + 0x8);
    if (i_var1 <= param_3 + 0x2) {
        param_3 = param_3 & 0xffff0000 | (i_var1 - 0x3);
    }
    // uVar8 = (param_3 >> 0x10);
    pass1_1008_3e76(
        (param_1 & 0xffff0000 | (iVar6 + 0x30)),
        param_2,
        param_3,
        uVar8,
    );
    u_var5 = uVar7;
    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (iVar6 + 0x36)),
        CONCAT22(param_4, local_6),
        CONCAT22(param_4, local_4),
    );
    pass1_1008_3e76((param_1 & 0xffff0000 | (iVar6 + 0x36)), 0x0, param_3, uVar8);
    (iVar6 + 0x4c) = 0x0;
    lVar4 = (iVar6 + 0x3c);
    u_var3 = (iVar6 + 0x2c);
    if ((u_var3 + 0x20) == lVar4) {
        pass1_1018_028c((iVar6 + 0x2c), (iVar6 + 0x3c), lVar4, u_var5, param_4);
        (iVar6 + 0x4c) = lVar4;
        (iVar6 + 0x4e) = u_var5;
        pass1_1010_1f62(param_4, param_1, 0x4);
    }
    return;
}

pub fn pass1_1018_179e(param_1: u32, param_2: u32, param_3: u16, param_4: u16) {
    let local_8: u16;
    let local_6: u32;

    pass1_1008_3eb4(
        param_2,
        CONCAT22(param_4, &local_8),
        CONCAT22(param_4, &local_6),
        CONCAT22(param_4, &local_6 + 0x2),
    );
    pass1_1018_16b8(param_1, local_8, local_6, param_4);
    return;
}

pub fn pass1_1018_17ce(param_1: u32, param_2: u32, param_3: u32, param_4: u16, param_5: u8) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    pass1_1018_0412(
        (param_1 + 0x2c),
        param_2,
        CONCAT22(param_3, (param_2 >> 0x10)),
        (param_3 >> 0x10),
        (param_1 + 0x3c),
        param_4,
        param_5,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_17f0() -> i16 {
    let i_stack4: i16;

    i_stack4 = 0x0;
    while (i_stack4 < 0x4 && ((i_stack4 * 0x2 + ctx.PTR_LOOP_1050_3962) != 0x0)) {
        i_stack4 += 0x1;
    }
    return i_stack4;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_181c(param_1: u32, param_2: &mut String, param_3: u8, param_4: u16) {
    let in_AH: u8;
    let u_var1: u16;

    u_var1 = CONCAT11(in_AH, param_3);
    pass1_1030_8344(ctx, ctx.PTR_LOOP_1050_5748, (param_1 + 0x3c));
    pass1_1030_5b6c(CONCAT22(param_4, u_var1), param_2, param_4);
    return;
}

pub fn pass1_1018_1842(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0x20));
    pass1_1018_078e(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_18b8(param_1: u16, param_2: &mut Struct55, param_3: u16) {
    let pu_var1: U32Ptr;
    let i_var3: &mut Struct55;
    let unaff_DI: i16;
    let u_var3: &mut Struct55;
    let pu_var2: U32Ptr;
    let paVar3: &mut Struct43;
    let u_var4: u32;
    let piVar5: U32Ptr;
    let u_var6: u16;
    let piVar7: U32Ptr;
    let uVar8: u16;
    let local_6: i16;
    let local_4: i16;
    let u_var1: u16;

    get_sys_metrics_1018_4b1e(param_2, 0x0, param_3);
    // u_var3 = (param_2 >> 0x10);
    i_var3 = param_2;
    i_var3.field_0x20 = 0x389a;
    i_var3.field_0x22 = 0x1008;
    i_var3.field_0x20 = 0x3aa8;
    i_var3.field_0x22 = 0x1008;
    &i_var3.field_0x24 = 0x0;
    i_var3.field_0x28 = 0x4;
    pu_var2 = clear_struct_1008_3e38((param_2 & 0xffff0000 | ZEXT24(i_var3 + 0x1)));
    // pu_var1 = (pu_var2 >> 0x10);
    &i_var3[0x1].field_0x6 = 0x0;
    i_var3[0x1].field_0xa = 0x0;
    &i_var3[0x1].field_0xc = 0x0;
    i_var3[0x1].field_0x10 = 0x0;
    i_var3[0x1].field_0x1c = 0x0;
    param_2.field_0x0 = 0x1fb0;
    i_var3.field_0x2 = 0x1018;
    i_var3.field_0x20 = 0x1fec;
    i_var3.field_0x22 = 0x1018;
    pass1_1000_4906((param_2 & 0xffff0000 | &i_var3[0x1].field_0x14), 0x0, 0x8);
    piVar7 = &local_4;
    piVar5 = &local_6;
    u_var6 = param_1;
    uVar8 = param_1;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x48, param_1, pu_var1, unaff_DI);
    pass1_1008_3e94(
        (pu_var2 & 0xffff0000 | (pu_var2 + 0xe)),
        CONCAT22(u_var6, piVar5),
        CONCAT22(uVar8, piVar7),
    );
    paVar3 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x9a, param_1);
    i_var3.field_0x24 = paVar3;
    &i_var3.field_0x26 = (paVar3 >> 0x10);
    u_var4 = pass1_1008_4772((paVar3 & 0xffff0000 | i_var3.field_0x24));
    // u_var1 = (u_var4 >> 0x10);
    pass1_1000_4906((param_2 & 0xffff0000 | &i_var3.field_0x32), 0x0, 0x8);
    i_var3.field_0x36 = (u_var4 + 0x4);
    i_var3.field_0x38 = (u_var4 + 0x8);
    i_var3.field_0x2a = local_4 + 0x14;
    i_var3.field_0x2c = local_6 + 0x14;
    get_sys_metrics_1018_1ea0(param_2, 0x1000);
    pass1_1008_3e76(
        (param_2 & 0xffff0000 | ZEXT24(i_var3 + 0x1)),
        0x0,
        0x88,
        0x99,
    );
    return;
}

pub fn pass1_1018_1a04(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let puVar4: U32Ptr;
    let iVar5: &mut Struct501;
    let u_var5: u16;
    let puStack14: U32Ptr;

    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = 0x1fb0;
    iVar5.field_0x2 = 0x1018;
    iVar5.field_0x20 = 0x1fec;
    iVar5.field_0x22 = 0x1018;
    pu_var1 = iVar5.field_0x24;
    u_var2 = iVar5.field_0x26;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    fn_ptr_1000_17ce(ctx, iVar5.field_0x40, 0x1000);
    if (param_1 == 0x0) {
        puVar4 = 0x0;
        u_var5 = 0x0;
    } else {
        puVar4 = &iVar5.field_0x20;
    }
    puStack14 = CONCAT22(u_var5, puVar4);
    *puStack14 = 0x389a;
    puVar4[0x1] = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_1a8e(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let lVar1: i32;
    let i_var2: &mut Struct653;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let piVar4: U32Ptr;
    let local_8: i16;
    let uStack6: u32;

    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (i_var2.field_0x44 != 0x0) {
        if (i_var2.field_0x46 != 0x0) {
            lVar1 = i_var2.field_0x46;
            (lVar1 + 0xe) = 0x0;
            i_var2.field_0x46 = 0x0;
        }
        piVar4 = &i_var2.field_0x4a;
        *piVar4 = *piVar4 + 0x1;
        return;
    }
    piVar4 = CONCAT22(param_4, &local_8);
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, param_4, param_2, param_3);
    pass1_1010_bf1e(pu_var3, piVar4, pu_var3, (pu_var3 >> 0x10), param_4);
    i_var2.field_0x44 = local_8;
    i_var2.field_0x40 = uStack6;
    pass1_1018_1ce8(param_4, param_1 & 0xffff | u_var2 << 0x10);
    return;
}

pub fn pass1_1018_1b02(param_1: u16, param_2: u32, param_3: i16) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u32;
    let u_var4: &mut Struct96;
    let iVar5: &mut Struct95;
    let u_var5: u16;
    let iStack12: i16;
    let local_6: u16;
    let local_4: [u8; 2];

    iStack12 = 0x0;
    loop {
        // u_var5 = (param_2 >> 0x10);
        iVar5 = param_2;
        pi_var1 = &iVar5.field_0x44;
        if (*pi_var1 == iStack12 || *pi_var1 < iStack12) {
            break;
        }
        u_var2 = iVar5.field_0x40;
        u_var4 = u_var2;
        u_var4 = u_var4 + iStack12;
        u_var2 &= 0xffff0000;
        u_var3 = ZEXT24(u_var4);
        pi_var1 = &u_var4.field_0x6;
        *pi_var1 = *pi_var1 + param_3 * 0x2 + -0x1;
        // u_var5 = (u_var2 >> 0x10);
        if (0x23 < u_var4.field_0x6) {
            u_var4.field_0x6 = 0x0;
        }
        if (u_var4.field_0x6 < 0x0) {
            u_var4.field_0x6 = 0x23;
        }
        pass1_1008_3f62((u_var2 | &u_var4.field_0x10), (u_var2 | u_var3));
        u_var4.field_0x16 = u_var4.field_0xa;
        pass1_1008_3e94(
            (u_var2 | u_var3),
            CONCAT22(param_1, &local_6),
            CONCAT22(param_1, local_4),
        );
        pass1_1008_3e76(
            (u_var2 | u_var3),
            0x0,
            local_6,
            ((u_var4.field_0x8 * 0x24 + u_var4.field_0x6) * 0x2 + 0x3c20),
        );
        u_var4.field_0xa = (u_var4.field_0x6 * 0x2 + 0x3966);
        iStack12 += 0x1;
    }
    pass1_1010_1f62(param_1, param_2, 0xd);
    return;
}

pub fn pass1_1018_1c9a(param_1: u32, param_2: i16) -> u16 {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let unaff_SS: u16;
    let uStack10: i16;

    iStack10 = 0x0;
    loop {
        // u_var4 = (param_1 >> 0x10);
        pi_var1 = (param_1 + 0x44);
        if (*pi_var1 == iStack10 || *pi_var1 < iStack10) {
            return 0x0;
        }
        u_var2 = (param_1 + 0x40);
        u_var3 = u_var2 + iStack10 * 0x18;
        if (((u_var3 + 0xc) * 0x1e + 0x3c32) == param_2) {
            break;
        }
        iStack10 += 0x1;
    }
    pass1_1018_1eda(param_1, u_var2 & 0xffff0000 | u_var3, unaff_SS);
    return 0x1;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1018_1ce8(param_1: u16, param_2: u32) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let iStack26: i16;
    let local_18: [u8; 2];
    let local_16: [u8; 2];
    let iStack20: i16;
    let iStack18: i16;
    let iStack16: i16;
    let local_e: u16;
    let local_c: i16;
    let local_a: i16;
    let iStack8: i16;
    let uStack6: u32;

    // u_var5 = (param_2 >> 0x10);
    i_var3 = param_2;
    uStack6 = (i_var3 + 0x40);
    iStack8 = 0x0;
    loop {
        pi_var1 = (i_var3 + 0x44);
        if (*pi_var1 == iStack8 || *pi_var1 < iStack8) {
            return;
        }
        pass1_1008_3eb4(
            (uStack6 & 0xffff0000 | (iStack8 * 0x18 + uStack6)),
            CONCAT22(param_1, &local_e),
            CONCAT22(param_1, &local_c),
            CONCAT22(param_1, &local_a),
        );
        local_a /= 0xa;
        iStack16 = local_c % 0xa;
        if (iStack16 != 0x0) {
            if (iStack16 < 0x6) {
                local_c -= iStack16;
            } else {
                local_c += 0xa - iStack16;
            }
        }
        iStack18 = pass1_1000_49b2(local_e);
        iStack18 /= 0x5;
        if (0x14 < iStack18) {
            iStack18 = 0x14;
            i_var2 = pass1_1000_49b2(local_e);
            local_e = (local_e / i_var2) * 0x64;
        }
        iStack16 = pass1_1000_49b2(local_e);
        iStack16 %= 0x5;
        if (iStack16 != 0x0) {
            if (local_e < 0x0) {
                i_var2 = iStack16;
                if (0x2 < iStack16) {
                    i_var2 = iStack16 + -0x5;
                }
                local_e += i_var2;
            } else {
                if (iStack16 < 0x3) {
                    local_e -= iStack16;
                } else {
                    local_e += 0x5 - iStack16;
                }
            }
        }
        iStack20 = (iStack18 * 0x48 + 0x3c20);
        if (local_c < iStack20) {
            // TODO: refactor for loop
            // for (iStack26 = iStack18; iStack26 < 0x15; iStack26 += 0x1) {
            //   pi_var1 = (iStack26 * 0x48 + 0x3c20);
            //   if (*pi_var1 == local_c || *pi_var1 < local_c) {
            //     iStack18 = iStack26;
            //     break;
            //   }
            // }
        }
        pass1_1008_3e94(
            (param_2 & 0xffff0000 | (i_var3 + 0x3a)),
            CONCAT22(param_1, local_18),
            CONCAT22(param_1, local_16),
        );
        u_var4 = iStack8 * 0x18 + uStack6;
        (u_var4 + 0x6) = local_a;
        (u_var4 + 0x8) = iStack18;
        pass1_1008_3e76(
            (uStack6 & 0xffff0000 | u_var4),
            0x0,
            local_e,
            ((iStack18 * 0x24 + local_a) * 0x2 + 0x3c20),
        );
        (u_var4 + 0xa) = (local_a * 0x2 + 0x3966);
        iStack8 += 0x1;
    }
}

pub fn pass1_1018_1e78(param_1: u32, param_2: i16) -> u32 {
    let u_var1: u32;

    if (param_2 == -0x1) {
        u_var1 = (param_1 + 0x46);
        param_2 = (u_var1 + 0xc);
    }
    return CONCAT22(0x1050, param_2 * 0x1e + 0x3c18);
}

pub fn pass1_1018_1eda(param_1: u32, param_2: u32, param_3: u16) {
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0x46) != 0x0) {
        u_var1 = (i_var2 + 0x46);
        (u_var1 + 0xe) = 0x2;
    }
    (i_var2 + 0x46) = param_2;
    (param_2 + 0xe) = 0x1;
    pass1_1010_1f62(param_3, param_1, 0xd);
    return;
}

pub fn pass1_1018_1f1a(param_1: u32, param_2: i16) -> u16 {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;
    let i_stack6: i16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0x56) == 0x0) {
        return 0x0;
    }
    i_stack6 = 0x0;
    loop {
        pi_var1 = (i_var2 + 0x56);
        if (*pi_var1 == i_stack6 || *pi_var1 < i_stack6) {
            return 0x0;
        }
        if ((i_var2 + 0x4e + i_stack6 * 0x2) == param_2) {
            break;
        }
        i_stack6 += 0x1;
    }
    return 0x1;
}

pub fn pass1_1018_1f6a(
    param_1: u16,
    param_2: &mut Struct18,
    param_3: u8,
    param_4: u16,
) -> Struct18 {
    param_2 = (param_2 & 0xffff0000 | (param_2 - 0x20));
    pass1_1018_1a04(param_2, param_4);
    if ((param_3 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_2, 0x1000);
    }
    return param_2;
}

pub fn pass1_1018_1f7a(param_1: i16, param_2: u16) -> u32 {
    return CONCAT22(param_2, param_1 + 0x2a);
}

pub fn pass1_1018_1f8a(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16 {
    pass1_1018_1a04(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_1ff4(param_1: &mut Struct634, param_2: &mut Struct19, param_3: u16) -> u32 {
    let pi_var1: U32Ptr;
    let unaff_DI: i16;
    let unaff_SS: u16;
    let paVar2: &mut Struct79;
    let pi_var3: U32Ptr;
    let piVar4: U32Ptr;
    let u_var5: u16;
    let local_a: i16;
    let local_8: i16;
    let uStack6: u32;

    paVar2 = set_struct_fields_1010_1d48(CONCAT22(param_2, param_1), param_3);
    &param_1.field_0xa = 0xb9010b;
    param_1.field_0xe = 0x170035;
    CONCAT22(param_2, param_1) = 0x21e8;
    param_1.field_0x2 = 0x1018;
    piVar4 = &local_8;
    pi_var3 = &local_a;
    u_var5 = unaff_SS;
    uStack6 = mixed_1010_20ba(
        ctx.PTR_LOOP_1050_0ed0,
        0x48,
        unaff_SS,
        (paVar2 >> 0x10),
        unaff_DI,
    );
    pass1_1008_3e94(
        (uStack6 & 0xffff0000 | (uStack6 + 0xe)),
        CONCAT22(unaff_SS, pi_var3),
        CONCAT22(u_var5, piVar4),
    );
    pi_var1 = &param_1.field_0xa;
    *pi_var1 = *pi_var1 + local_8;
    pi_var1 = &param_1.field_0xc;
    *pi_var1 = *pi_var1 + local_a;
    pass1_1000_4906(CONCAT22(param_2, param_1 + 0x1), 0x0, 0x7f4);
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1018_2076(param_1: U32Ptr, param_2: u16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = 0x21e8;
    (param_1 + 0x2) = 0x1018;
    pass1_1018_209c(param_1 & 0xffff | u_var1 << 0x10);
    pass1_1010_1d80(param_1, param_2);
    return;
}

pub fn pass1_1018_209c(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let i_stack4: i16;

    i_stack4 = 0x0;
    loop {
        // u_var5 = (param_1 >> 0x10);
        i_var4 = param_1 + 0x12;
        pu_var1 = (i_var4 + i_stack4 * 0x4);
        u_var2 = (i_var4 + i_stack4 * 0x4 + 0x2);
        if ((u_var2 | pu_var1) != 0x0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
        (param_1 + i_stack4 * 0x4 + 0x12) = 0x0;
        i_stack4 += 0x1;
        if (i_stack4 < 0x1fd) == false {
            break;
        }
    }
    return;
}

pub fn pass1_1018_20ee(param_1: u32, param_2: &mut i16) {
    let b_var1: bool;
    let in_DX: u16;
    let u_var2: u16;

    b_var1 = pass1_1008_aed8(param_2);
    if (b_var1 == 0x0) {
        return;
    }
    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + *param_2 * 0x4 + 0x12) == 0x0) {
        pass1_1018_216e(param_1 & 0xffff | u_var2 << 0x10, param_2, in_DX);
    }
    pass1_1008_ae26(param_2);
    return;
}

pub fn pass1_1018_214e(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: i16) {
    pass1_1008_3e76(
        param_3,
        0x0,
        (param_4 * 0x4 + 0x3e90),
        (param_4 * 0x4 + 0x3e8e),
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_216e(param_1: u32, param_2: u32, param_3: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let uStack8: u16;

    uStack8 = pass1_1008_adf2(param_2);
    u_var1 = pass1_1008_ae0c(param_2);
    // TODO: refactor for loop
    // for (; uStack8 <= u_var1; uStack8 += 0x1) {
    //   u_var2 = u_var1;
    //   pass1_1010_8018(ctx.PTR_LOOP_1050_14cc,uStack8,param_3,0x1010);
    //   u_var3 = (param_1 >> 0x10);
    //   (param_1 + uStack8 * 0x4 + 0x12) = u_var2;
    //   (param_1 + uStack8 * 0x4 + 0x14) = param_3;
    // }
    return;
}

pub fn pass1_1018_21c2(param_1: u32, param_2: u8, param_3: u16) -> u32 {
    pass1_1018_2076(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_21f8() -> u16 {
    let pu_var1: U32Ptr;

    pass1_1008_3e54(&USHORT_1048_4210, 0x0, 0x195, 0x1);
    pass1_1008_3e54(&USHORT_1050_65ca, 0x0, 0xe0, 0x1b1);
    pass1_1008_3e54(&USHORT_1050_65d0, 0x0, 0x17a, 0x72);
    pass1_1008_3e54(&USHORT_1050_65d6, 0x0, 0xde, 0x93);
    pass1_1008_3e54(&USHORT_1050_65dc, 0x0, 0x177, 0x1da);
    pass1_1008_3e54(&USHORT_1048_4216, 0x0, 0x195, 0x21c);
    pass1_1008_3e54(&USHORT_1048_421c, 0x0, 0x1b6, 0x22c);
    pass1_1008_3e54(&USHORT_1048_4222, 0x0, 0x109, 0x5);
    pu_var1 = pass1_1008_3e54(&USHORT_1048_4228, 0x0, 0xfd, 0x1fd);
    return pu_var1;
}

pub fn pass1_1018_2440(param_1: &mut Struct11, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let piVar4: U32Ptr;
    let u_var6: u16;
    let u_var5: &mut Struct502;
    let uVar7: u16;
    let puStack6: U32Ptr;

    // uVar7 = (param_1 >> 0x10);
    u_var5 = param_1;
    param_1 = 0x2ada;
    u_var5.field_0x2 = 0x1018;
    u_var5.field_0x1c = (s_fem132_wav_1050_2aec + 0x6);
    u_var5.field_0x1e = 0x1018;
    if (ctx.PTR_LOOP_1050_0388 != 0x0) {
        if (param_1 == 0x0) {
            piVar4 = 0x0;
            u_var6 = 0x0;
        } else {
            piVar4 = &u_var5.field_0x1c;
            u_var6 = uVar7;
        }
        param_2 = 0x1008;
        pass1_1008_92b2(ctx.PTR_LOOP_1050_0388, 0x73, CONCAT22(u_var6, piVar4));
    }
    pu_var1 = u_var5.field_0x2a;
    u_var2 = u_var5.field_0x2c;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(param_2, pu_var1, u_var2, 0x1);
    }
    pu_var1 = u_var5.field_0x6e;
    u_var2 = u_var5.field_0x70;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(param_2, pu_var1, u_var2, 0x1);
    }
    if (param_1 == 0x0) {
        piVar4 = 0x0;
        uVar7 = 0x0;
    } else {
        piVar4 = &u_var5.field_0x1c;
    }
    puStack6 = CONCAT22(uVar7, piVar4);
    *puStack6 = 0x389a;
    piVar4[0x1] = 0x1008;
    clenaup_win_ui_1018_4d22(param_1, param_2);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_2504(param_1: u16, param_2: u16) {
    let u_var1: u16;

    pass1_1030_8344(ctx, ctx.PTR_LOOP_1050_5748, 0x4000001);
    if ((param_2 | param_1) != 0x0) {
        u_var1 = pass1_1028_d69e(**ctx.PTR_LOOP_1050_5748);
        if (u_var1 == 0x0) {
            return;
        }
    }
    return;
}

pub fn pass1_1018_2548(param_1: u16, param_2: u16, param_3: U32Ptr) {
    pass1_1008_3f62(param_3, &USHORT_1048_4228);
    return;
}

pub fn pass1_1018_255e(param_1: u32) -> u16 {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x26) != 0x0) {
        u_var1 = (param_1 + 0x26);
        return (u_var1 + 0xa);
    }
    return 0x0;
}

pub fn pass1_1018_2580(
    param_1: u32,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u16,
    param_6: u8,
) -> Vec<u8> {
    let pu_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;
    let ulocal_8: [u8; 0x6];

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0x20) == 0x0) {
        return 0x6ad;
    }
    clear_struct_1008_3e38(CONCAT22(param_5, local_8));
    pass1_1018_161c(
        param_5,
        (i_var2 + 0x20),
        CONCAT22(param_5, local_8),
        param_3,
        (param_3 >> 0x10),
    );
    pu_var1 = local_8;
    pass1_1018_17ce(
        (i_var2 + 0x20),
        CONCAT22(pu_var1, param_2),
        CONCAT22(param_4, param_5),
        param_5,
        param_6,
    );
    return pu_var1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_25d2(
    param_1: u32,
    param_2: u16,
    param_3: u32,
    param_4: i16,
    param_5: u16,
) -> u16 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let pu_var5: U32Ptr;
    let local_8: [u8; 6];

    // u_var3 = (param_1 >> 0x10);
    if ((param_1 + 0x20) == 0x0) {
        return 0x0;
    }
    puVar4 = clear_struct_1008_3e38(CONCAT22(param_5, local_8));
    // pu_var2 = (puVar4 >> 0x10);
    pass1_1018_161c(
        param_5,
        (param_1 + 0x20),
        CONCAT22(param_5, local_8),
        param_3,
        (param_3 >> 0x10),
    );
    pu_var5 = CONCAT22(param_5, local_8);
    puVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x32, param_5, pu_var2, param_4);
    u_var1 = puVar4;
    pass1_1010_71d6(puVar4, param_2, pu_var5, u_var1, (puVar4 >> 0x10), param_5);
    return u_var1;
}

pub fn pass1_1018_2646(param_1: u16, param_2: u16, param_3: U32Ptr) {
    pass1_1008_3f62(param_3, &USHORT_1048_4222);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_265c(ctx: &mut AppContext) -> u32 {
    let u_var1: u32;

    u_var1 = pass1_1030_8326(ctx);
    return u_var1;
}

pub fn pass1_1018_266a(param_1: u32) -> u16 {
    return (param_1 + 0x44);
}

pub fn pass1_1018_2678(param_1: u16, param_2: u16, param_3: U32Ptr) {
    pass1_1008_3f62(param_3, &USHORT_1048_4216);
    return;
}

pub fn pass1_1018_268e(param_1: u32) -> u32 {
    let i_var1: &mut Struct287;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var1 = param_1;
    if (i_var1.field_0x42 != 0x0) {
        &i_var1.field_0x40 = 0x0;
        i_var1.field_0x44 = 0x1;
    }
    i_var2 = i_var1.field_0x3e * 0x4;
    return CONCAT22(
        (&i_var1[0x1].field_0x2 + i_var2),
        (&i_var1[0x1].field_0x0 + i_var2),
    );
}

pub fn pass1_1018_26c2(param_1: u16, param_2: u16, param_3: U32Ptr) {
    pass1_1008_3f62(param_3, &USHORT_1048_421c);
    return;
}

pub fn pass1_1018_26d8(param_1: u16, param_2: u16, param_3: i16, param_4: U32Ptr) {
    pass1_1008_3f62(param_4, CONCAT22(0x1050, &USHORT_1050_65ca + param_3 * 0x6));
    return;
}

pub fn pass1_1018_26f8(param_1: u16, param_2: u16, param_3: U32Ptr) {
    pass1_1008_3f62(param_3, &USHORT_1048_4210);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_270e(
    param_1: u32,
    param_2: i16,
    param_3: u16,
    param_4: U32Ptr,
    param_5: i16,
    param_6: u16,
) {
    let ppcVar1: u32;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let extraout_dx: U32Ptr;
    let iVar5: &mut Struct655;
    let u_var5: u16;
    let puVar6: U32Ptr;

    iVar5 = param_1;
    // u_var5 = (param_1 >> 0x10);
    if (param_2 == 0x0) {
        if ((iVar5.field_0x20 == 0x0) || (u_var2 = iVar5.field_0x20, (u_var2 + 0x8) != param_3)) {
            puVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, param_3, param_6, param_4, param_5);
            if (iVar5.field_0x20 != 0x0) {
                if (param_1 == 0x0) {
                    i_var3 = 0x0;
                    u_var4 = 0x0;
                } else {
                    i_var3 = &iVar5.field_0x1c;
                    u_var4 = u_var5;
                }
                pass1_1010_1ea6(iVar5.field_0x20, CONCAT22(u_var4, i_var3), param_6);
            }
            iVar5.field_0x20 = puVar6;
            if (param_1 == 0x0) {
                param_3 = 0x0;
                u_var4 = 0x0;
            } else {
                param_3 = &iVar5.field_0x1c;
                u_var4 = u_var5;
            }
            u_var2 = iVar5.field_0x20;
            ppcVar1 = (iVar5.field_0x20 + 0x4);
            (**ppcVar1)(0x1010, u_var2, (u_var2 >> 0x10), 0x0, param_3, u_var4);
            param_4 = extraout_dx;
        }
        pass1_1018_2862(param_1);
        if ((param_4 | param_3) != 0x0) {
            iVar5.field_0x24 = 0x1;
        }
        pass1_1010_1f62(param_6, param_1, 0x7);
    } else {
        if (((&iVar5.field_0x20 + 0x2) | &iVar5.field_0x20) != 0x0) {
            if (param_1 == 0x0) {
                i_var3 = 0x0;
                u_var4 = 0x0;
            } else {
                i_var3 = &iVar5.field_0x1c;
                u_var4 = u_var5;
            }
            pass1_1010_1ea6(iVar5.field_0x20, CONCAT22(u_var4, i_var3), param_6);
            iVar5.field_0x20 = 0x0;
            return;
        }
    }
    return;
}

pub fn pass1_1018_280c(param_1: u32) {
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0x24) == 0x0) {
        return;
    }
    (i_var2 + 0x24) = 0x0;
    if ((i_var2 + 0x20) == 0x0) {
        (i_var2 + 0x26) = 0x0;
    } else {
        u_var1 = (i_var2 + 0x20);
        (i_var2 + 0x26) = (u_var1 + 0x4c);
    }
    return;
}

pub fn pass1_1018_2862(param_1: u32) {
    let lVar1: i32;
    let i_var2: &mut Struct654;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (i_var2.field_0x20 == 0x0) {
        i_var2.field_0x26 = 0x0;
    } else {
        lVar1 = i_var2.field_0x20;
        i_var2.field_0x26 = (lVar1 + 0x4c);
    }
    return;
}

pub fn pass1_1018_289c(param_1: u32, param_2: i16, param_3: u16, param_4: u16) {
    let u_var1: u16;

    if (param_2 == 0x1) {
        (param_1 + 0x4) = 0x0;
        return;
    }
    if (param_2 == 0x2) {
        pass1_1018_2922(param_1 & 0xffff0000 | (param_1 - 0x1c));
    } else {
        if ((((param_2 + -0x3 < 0x1) || (SBORROW2(param_2 + -0x3, 0x1))) || (0x1 < param_2 + -0x5))
            || ((param_1 + 0x4) == 0x0))
        {
            return;
        }
        u_var1 = param_1 - 0x1c;
        pass1_1018_2862(param_1 & 0xffff0000 | u_var1);
        if ((param_3 | u_var1) != 0x0) {
            (param_1 + 0x8) = 0x1;
        }
    }
    pass1_1010_1f62(param_4, param_1 & 0xffff0000 | (param_1 - 0x1c), param_2);
    return;
}

pub fn pass1_1018_2922(param_1: u32) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 0x40) != 0x0)
        && (
            pi_var1 = (i_var2 + 0x3e),
            *pi_var1 = *pi_var1 + 0x1,
            0x9 < (i_var2 + 0x3e),
        ))
    {
        (i_var2 + 0x3e) = 0x0;
        (i_var2 + 0x42) = 0x1;
    }
    return;
}

pub fn pass1_1018_2aa3() {
    pass1_1018_21f8();
    return;
}

pub fn pass1_1018_2aa8(param_1: &mut Struct11, param_2: u8, param_3: u16) -> Struct11 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0x1c));
    pass1_1018_2440(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_2afa(param_1: U32Ptr) {
    fn_ptr_1000_17ce(ctx, *param_1, 0x1000);
    return;
}

pub fn pass1_1018_2c60(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let u_var6: &mut Struct503;
    let uVar7: u16;
    let puStack6: U32Ptr;

    // uVar7 = (param_1 >> 0x10);
    u_var6 = param_1;
    *param_1 = 0x32d8;
    u_var6.field_0x2 = 0x1018;
    u_var6.field_0x20 = 0x3314;
    u_var6.field_0x22 = 0x1018;
    if (u_var6.field_0x182 != 0x0) {
        if (param_1 == 0x0) {
            puVar4 = 0x0;
            u_var5 = 0x0;
        } else {
            puVar4 = &u_var6.field_0x20;
            u_var5 = uVar7;
        }
        pass1_1010_1ea6(u_var6.field_0x182, CONCAT22(u_var5, puVar4), param_2);
    }
    fn_ptr_1000_17ce(ctx, u_var6.field_0x186, 0x1000);
    pu_var1 = u_var6.field_0x24;
    u_var2 = u_var6.field_0x26;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(0x1000, pu_var1, u_var2, 0x1);
    }
    if (param_1 == 0x0) {
        puVar4 = 0x0;
        uVar7 = 0x0;
    } else {
        puVar4 = &u_var6.field_0x20;
    }
    puStack6 = CONCAT22(uVar7, puVar4);
    *puStack6 = 0x389a;
    puVar4[0x1] = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}

pub fn pass1_1018_2d22(param_1: u32, param_2: &mut i16, param_3: U32Ptr, param_4: i16) {
    let u_var1: u32;

    *param_3 = 0x0;
    *param_2 = 0x0;
    u_var1 = pass1_1008_4772((param_1 + 0x24));
    *param_2 = (u_var1 + 0x8) + -0x14;
    if (param_4 == 0xbb8) {
        *param_3 = 0x5;
    }
    if (param_4 == 0xbba) {
        *param_3 = 0x23;
    }
    if (param_4 == 0xbb9) {
        *param_3 = 0x75;
    }
    return;
}

pub fn pass1_1018_2d84(param_1: u16, param_2: u32) {
    pass1_1018_2e28(param_2);
    pass1_1020_bd80(param_1);
    return;
}

pub fn pass1_1018_2d9a(param_1: u32) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var3 = (i_var4 + 0x180) | (i_var4 + 0x17e);
    if (u_var3 != 0x0) {
        pi_var1 = (i_var4 + 0x174);
        *pi_var1 = *pi_var1 + -0x1;
        if (*pi_var1 < 0x0) {
            u_var2 = (i_var4 + 0x17e);
            u_var3 = (u_var2 + 0xa) - 0x1;
            (i_var4 + 0x174) = u_var3;
        }
        pass1_1018_2e28(param_1);
        (i_var4 + 0x176) = u_var3;
    }
    return;
}

pub fn pass1_1018_2dde(param_1: u32) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let i_var3: i16;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if (((i_var4 + 0x180) | (i_var4 + 0x17e)) != 0x0) {
        pi_var1 = (i_var4 + 0x174);
        *pi_var1 = *pi_var1 + 0x1;
        i_var3 = (i_var4 + 0x174);
        u_var2 = (i_var4 + 0x17e);
        pi_var1 = (u_var2 + 0xa);
        if (*pi_var1 == i_var3 || *pi_var1 < i_var3) {
            (i_var4 + 0x174) = 0x0;
        }
        pass1_1018_2e28(param_1);
        (i_var4 + 0x176) = i_var3;
    }
    return;
}

pub fn pass1_1018_2e28(param_1: u32) {
    let lVar1: i32;
    let extraout_dx: u16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    lVar1 = (param_1 + 0x174);
    empty_1008_8fc4((param_1 + 0x17e), lVar1);
    if ((extraout_dx | lVar1) != 0x0) {
        return;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_2e5e(param_1: u16, param_2: u16, param_3: u16, param_4: u32) {
    let u_var1: u16;
    let lVar1: i32;
    let i_var4: &mut Struct126;
    let u_var2: u16;

    // u_var2 = (param_4 >> 0x10);
    i_var4 = param_4;
    if (i_var4.field_0x17e == 0x0) {
        pass1_1030_82f0(param_1, ctx.PTR_LOOP_1050_5748, i_var4.field_0x17a);
        &i_var4.field_0x17e = param_2;
        (&i_var4.field_0x17e + 0x2) = param_3;
    }
    if ((i_var4.field_0x17e != 0x0) && (lVar1 = i_var4.field_0x17e, (lVar1 + 0xa) != 0x0)) {
        lVar1 = i_var4.field_0x174;
        empty_1008_8fc4(i_var4.field_0x17e, lVar1);
        u_var1 = lVar1;
        pass1_1018_2e28(param_4);
        i_var4.field_0x176 = u_var1;
        return;
    }
    return;
}

pub fn pass1_1018_2ee4(param_1: u32, param_2: u16, param_3: u16) {
    let u_var1: u32;
    let cVar2: u8;
    let u_var3: u16;

    if (param_2 != 0x12) {
        if (param_2 < 0x13) {
            cVar2 = param_2;
            if (cVar2 == '\x01') {
                (param_1 + 0x162) = 0x0;
                return;
            }
            if (('\x02' < (cVar2 + -0x1)) && ((cVar2 + -0x4) < '\x03')) {}
            //       TODO: goto LAB_1018_2f06;
        }
        return;
    }
    u_var1 = (param_1 + 0x162);
    (param_1 + 0x15a) = (u_var1 + 0x24);
    //LAB_1018_2f06:
    u_var3 = param_1 - 0x20;
    pass1_1018_31fa(
        param_1 & 0xffff0000 | u_var3,
        u_var3,
        param_1._2_2_,
        param_3,
    );
    pass1_1010_1f62(param_3, param_1 & 0xffff0000 | u_var3, param_2);
    return;
}

pub fn pass1_1018_2fe8(param_1: u32, param_2: u16, param_3: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let iVar7: i16;
    let extraout_dx: u16;
    let uVar8: u16;
    let i_var9: i16;
    let u_var10: u16;

    // u_var10 = (param_1 >> 0x10);
    i_var9 = param_1;
    u_var6 = (i_var9 + 0x174);
    u_var2 = (i_var9 + 0x17e);
    iVar7 = (u_var2 + 0xa);
    if (iVar7 != 0x0) {
        if ((i_var9 + 0x186) != 0x0) {
            u_var3 = str_op_1000_3da4((i_var9 + 0x186));
            (i_var9 + 0x174) = 0x0;
            loop {
                if (iVar7 <= (i_var9 + 0x174)) {
                    break;
                }
                u_var4 = (i_var9 + 0x174);
                u_var2 = (i_var9 + 0x17e);
                empty_1008_8fc4(u_var2, (u_var2 >> 0x10), u_var4, u_var4 >> 0xf);
                uVar8 = extraout_dx;
                pass1_1018_2e28(param_1);
                u_var4 = pass1_1020_bd80(u_var4);
                u_var5 = pass1_1000_3de8(
                    CONCAT22(uVar8, u_var4),
                    (i_var9 + 0x186),
                    &mut u_var3,
                    param_2,
                    param_3,
                );
                if (u_var5 == 0x0) {
                    break;
                }
                pi_var1 = (i_var9 + 0x174);
                *pi_var1 = *pi_var1 + 0x1;
            }
            if ((i_var9 + 0x174) < iVar7) {
                pass1_1018_2e28(param_1);
                (i_var9 + 0x176) = iVar7;
                return;
            }
            (i_var9 + 0x174) = u_var6;
            pass1_1018_2e28(param_1);
            (i_var9 + 0x176) = u_var6;
        }
    }
    return;
}

pub fn pass1_1018_30ca(param_1: u32, param_2: &mut String, param_3: u16) {
    let u_var1: u16;
    let i_var3: &mut Struct504;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var3 = param_1;
    fn_ptr_1000_17ce(ctx, &i_var3.field_0x186, 0x1000);
    u_var1 = str_op_1008_60e8(param_2, param_3);
    i_var3.field_0x186 = u_var1;
    i_var3.field_0x188 = param_3;
    return;
}

pub fn pass1_1018_30fc(param_1: u32, param_2: &[u16], param_3: U32Ptr) {
    let u_var1: u16;
    let u_var2: u32;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let u_var5: u16;
    let lVar6: i32;
    let puVar7: U32Ptr;
    let extraout_dx: u16;
    let uVar8: u16;
    let puStack18: u32;
    let i_stack6: i16;

    *param_2 = 0x0;
    // uVar8 = (param_1 >> 0x10);
    u_var2 = (param_1 + 0x17e);
    u_var1 = (u_var2 + 0xa);
    if (u_var1 != 0x0) {
        u_var4 = u_var1;
        mem_op_1000_179c(0x6, param_3, 0x1000);
        puStack18 = CONCAT22(param_3, u_var4);
        puVar7 = (param_3 | u_var4);
        if (puVar7 == 0x0) {
            *param_2 = 0x0;
        } else {
            *puStack18 = 0x0;
            (u_var4 + 0x4) = 0x0;
            *param_2 = puStack18;
        }
        u_var5 = u_var1 * 0x2;
        mem_op_1000_179c(u_var5, puVar7, 0x1000);
        pu_var3 = *param_2;
        *pu_var3 = u_var5;
        (pu_var3 + 0x2) = puVar7;
        (*param_2 + 0x4) = u_var1;
        // TODO: refactor for loop
        // for (i_stack6 = 0x0; i_stack6 < u_var1; i_stack6 += 0x1) {
        //   lVar6 = i_stack6;
        //   empty_1008_8fc4((param_1 + 0x17e),lVar6);
        //   (*param_2 + i_stack6 * 0x2) =
        //        (lVar6 + 0x2e);
        // }
    }
    return;
}

pub fn pass1_1018_31d0(param_1: u32) -> u16 {
    let u_var1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if (((param_1 + 0x17e) != 0x0) && (u_var1 = (param_1 + 0x17e), (u_var1 + 0xa) != 0x0)) {
        return 0x1;
    }
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_31fa(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let i_var3: i16;
    let i_var4: i16;
    let lVar5: i32;
    let iVar6: i16;
    let uVar7: u16;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pass1_1030_82f0(param_4, ctx.PTR_LOOP_1050_5748, (iVar6 + 0x17a));
    (iVar6 + 0x17e) = param_2;
    (iVar6 + 0x180) = param_3;
    if (((param_3 | (iVar6 + 0x17e)) != 0x0)
        && (
            u_var2 = (iVar6 + 0x17e),
            i_var4 = (u_var2 + 0xa),
            i_var4 != 0x0,
        ))
    {
        (iVar6 + 0x174) = 0x0;
        loop {
            if (i_var4 <= (iVar6 + 0x174)) {
                break;
            }
            lVar5 = (iVar6 + 0x174);
            empty_1008_8fc4((iVar6 + 0x17e), lVar5);
            i_var3 = lVar5;
            pass1_1018_2e28(param_1);
            if ((iVar6 + 0x176) == i_var3) {
                break;
            }
            pi_var1 = (iVar6 + 0x174);
            *pi_var1 = *pi_var1 + 0x1;
        }
        if (i_var4 <= (iVar6 + 0x174)) {
            (iVar6 + 0x174) = 0x0;
        }
        pass1_1018_2e28(param_1);
        (iVar6 + 0x176) = i_var4;
    }
    return;
}

pub fn pass1_1018_32a6(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0x20));
    pass1_1018_2c60(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_331c(
    param_1: &mut Struct638,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: U32Ptr,
) {
    let u_var1: u16;
    let unaff_DI: i16;
    let pu_var2: U32Ptr;

    pass1_1008_ca5a(param_1, param_2, param_3);
    &param_1.field_0x122 = 0x0;
    param_1.field_0x126 = 0x0;
    param_1.field_0x12a = 0x0;
    param_1.field_0x12e = 0x0;
    param_1.field_0x130 = 0x0;
    param_1.field_0x132 = 0x0;
    param_1.field_0x136 = 0x0;
    param_1.field_0x13a = 0x0;
    param_1.field_0x13c = 0x0;
    param_1.field_0x13e = 0x0;
    param_1.field_0x142 = 0x0;
    CONCAT22(param_2, param_1) = &ctx.PTR_LOOP_1050_470c;
    param_1.field_0x2 = 0x1018;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3b, param_4, param_5, unaff_DI);
    u_var1 = pu_var2;
    param_1.field_0x122 = u_var1;
    param_1.field_0x124 = (pu_var2 >> 0x10);
    *&param_1.field_0x22 = 0x0;
    pass1_1008_612e(0x8, 0xc, u_var1);
    param_1.field_0x13c = u_var1;
    return;
}

pub fn pass1_1018_33b4(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let iVar5: &mut Struct505;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = &ctx.PTR_LOOP_1050_470c;
    iVar5.field_0x2 = 0x1018;
    pu_var1 = iVar5.field_0x136;
    u_var2 = iVar5.field_0x138;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    &iVar5.field_0x136 = 0x0;
    fn_ptr_1000_17ce(ctx, iVar5.field_0x126, 0x1000);
    fn_ptr_1000_17ce(ctx, iVar5.field_0x12a, 0x1000);
    pass1_1008_caa0(param_1, param_2);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_3424(param_1: u32, param_2: i16, param_3: u16, param_4: u16) {
    let u_var1: u32;
    let u_var2: u16;
    let i_var3: i16;
    let u_var4: u16;
    let uStack10: u32;
    let uStack6: u32;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var1 = (i_var3 + 0x122);
    pass1_1008_e852(u_var1, (u_var1 >> 0x10), (i_var3 + 0x126), param_4, param_3);
    uStack6 = CONCAT22(param_3, param_2);
    u_var1 = (i_var3 + 0x122);
    pass1_1008_e852(u_var1, (u_var1 >> 0x10), (i_var3 + 0x12a), param_4, param_3);
    uStack10 = CONCAT22(param_3, param_2);
    pass1_1030_8344(ctx, ctx.PTR_LOOP_1050_5748, uStack6);
    u_var2 = param_3;
    i_var3 = param_2;
    pass1_1030_8344(ctx, ctx.PTR_LOOP_1050_5748, uStack10);
    if ((i_var3 + 0x200) == (param_2 + 0x200)) {
        return;
    }
    return;
}

pub fn pass1_1018_34a6(param_1: u32) {
    pass1_1018_3d6c(param_1);
    return;
}

pub fn pass1_1018_36e6(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x12e) = param_4;
    (i_var1 + 0x130) = param_3;
    (i_var1 + 0x132) = param_2;
    (i_var1 + 0x134) = 0x0;
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_3710(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: u16) {
    let u_var1: u32;
    let ppcVar2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let iVar6: i16;
    let uVar7: u16;
    let in_AF: u8;
    let lVar8: i32;
    let puVar9: U32Ptr;
    let local_12a: [u8; 118];
    let uStack18: u32;
    let puStack14: U32Ptr;
    let uStack10: u32;
    let puStack6: U32Ptr;

    puStack6 = 0x0;
    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    uStack10 = switch_1018_3b9e(param_1, (iVar6 + 0x12e), param_3, param_4, param_2);
    u_var4 = (iVar6 + 0x12e) - 0x188;
    uStack18 = (uStack10 & 0xffff0000 | u_var4);
    match (u_var4) {
        0x0 => {
            lVar8 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
            // pu_var5 = (lVar8 >> 0x10);
            i_var3 = lVar8;
            mem_op_1000_179c(0x10, pu_var5, 0x1000);
            if (lVar8 != 0x0) {
                uStack18 = struct_1018_4790(lVar8, (iVar6 + 0x132), 0x0, (i_var3 + 0xe));
                puStack6 = uStack18;
                //       TODO: goto switchD_1018_393f_caseD_6;
            }
        }
        0x1 => {
            puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
            // pu_var5 = (puVar9 >> 0x10);
            i_var3 = puVar9;
            mem_op_1000_179c(0x14, pu_var5, 0x1000);
            u_var4 = (puVar9 >> 0x10) | puVar9;
            if (puVar9 != 0x0) {
                struct_1018_47c8(
                    puVar9,
                    (iVar6 + 0x132),
                    0x0,
                    (i_var3 + 0x12),
                    (i_var3 + 0xe),
                );
                uStack18 = (puVar9 & 0xffff | u_var4 << 0x10);
                puStack6 = uStack18;
                //       TODO: goto switchD_1018_393f_caseD_6;
            }
        }
        0x2 => {
            puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
            // pu_var5 = (puVar9 >> 0x10);
            i_var3 = puVar9;
            mem_op_1000_179c(0x12, pu_var5, 0x1000);
            u_var4 = (puVar9 >> 0x10) | puVar9;
            if (puVar9 != 0x0) {
                pass1_1018_4808(puVar9, (iVar6 + 0x132), 0x0, (i_var3 + 0xe));
                uStack18 = (puVar9 & 0xffff | u_var4 << 0x10);
                puStack6 = uStack18;
                //       TODO: goto switchD_1018_393f_caseD_6;
            }
        }
        0x3 => {
            puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
            // pu_var5 = (puVar9 >> 0x10);
            i_var3 = puVar9;
            mem_op_1000_179c(0x14, pu_var5, 0x1000);
            if (puVar9 != 0x0) {
                uStack18 = struct_1018_4842(puVar9, (iVar6 + 0x132), 0x0, (i_var3 + 0xe));
                puStack6 = uStack18;
                //       TODO: goto switchD_1018_393f_caseD_6;
            }
        }
        0x4 => {
            puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
            // pu_var5 = (puVar9 >> 0x10);
            i_var3 = puVar9;
            mem_op_1000_179c(0x10, pu_var5, 0x1000);
            if (puVar9 != 0x0) {
                uStack18 = struct_1018_48b0(puVar9, (iVar6 + 0x132), 0x0, (i_var3 + 0xe));
                puStack6 = uStack18;
                //       TODO: goto switchD_1018_393f_caseD_6;
            }
        }
        0x5 => {
            puVar9 = pass1_1008_57f0(uStack10, (iVar6 + 0x130), param_2);
            // pu_var5 = (puVar9 >> 0x10);
            i_var3 = puVar9;
            mem_op_1000_179c(0x12, pu_var5, 0x1000);
            if (puVar9 != 0x0) {
                uStack18 = struct_1018_4920(puVar9, (iVar6 + 0x132), 0x0, (i_var3 + 0xe));
                puStack6 = uStack18;
            }
        }
        _ => {} //     TODO: goto switchD_1018_393f_caseD_6;
    }
    uStack18 = 0x0;
    puStack6 = uStack18;
    // switchD_1018_393f_caseD_6:
    u_var1 = (iVar6 + 0x122);
    pass1_1008_e852(
        u_var1,
        (u_var1 >> 0x10),
        (iVar6 + 0x126),
        param_2,
        (uStack18 >> 0x10),
    );
    u_var1 = (iVar6 + 0x122);
    puStack14 = uStack18;
    pass1_1008_e852(
        u_var1,
        (u_var1 >> 0x10),
        (iVar6 + 0x12a),
        param_2,
        (uStack18 >> 0x10),
    );
    pass1_1038_2a0e(
        CONCAT22(param_2, local_12a),
        (iVar6 + 0x136),
        puStack6,
        uStack18,
        puStack14,
        param_2,
        in_AF,
    );
    fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_2, local_12a));
    (iVar6 + 0x136) = 0x0;
    ppcVar2 = (*param_1 + 0x10);
    (**ppcVar2)(0x1030, param_1);
    pass1_1038_2a5c(CONCAT22(param_2, local_12a));
    return;
}

pub fn pass1_1018_3a42(param_1: u32, param_2: u32, param_3: u16, param_4: u16) {
    let u_var1: u32;

    u_var1 = (param_1 + 0x122);
    pass1_1008_e852(u_var1, (u_var1 >> 0x10), param_2, param_4, param_3);
    return;
}

pub fn pass1_1018_3a5c(param_1: u32, param_2: u32, param_3: u32, param_4: u16) {
    pass1_1008_e320((param_1 + 0x122), param_2, param_3, param_4);
    return;
}

pub fn pass1_1018_3a7a(param_1: u32, param_2: u32, param_3: u16, param_4: u16) -> u32 {
    let u_var1: u32;
    let u_var2: u32;

    u_var1 = (param_1 + 0x122);
    u_var2 = string_1008_e586(u_var1, (u_var1 >> 0x10), param_2, param_3, param_4);
    return u_var2;
}

pub fn pass1_1018_3a94(param_1: u32, param_2: U32Ptr, param_3: U32Ptr, param_4: u16) {
    pass1_1008_e3ec((param_1 + 0x122), param_2, param_3, param_4);
    return;
}

pub fn pass1_1018_3ab2(param_1: u32, param_2: i16, param_3: i16, param_4: u16) -> u16 {
    let u_var1: u16;
    let u_var2: u16;
    let i_var3: i16;
    let lVar4: i32;
    let uStack22: u16;
    let local_10: [u8; 8];
    let iStack8: i16;
    let uStack6: u32;

    if (0x5 < param_3 - 0x188) {
        return 0x0;
    }
    i_var3 = param_1;
    // u_var2 = (param_1 >> 0x10);
    match (param_3) {
        0x188 => {
            u_var1 = (i_var3 + 0xa);
            u_var2 = (i_var3 + 0xc);
        }
        0x189 => {
            u_var1 = (i_var3 + 0xe);
            u_var2 = (i_var3 + 0x10);
        }
        0x18a => {
            u_var1 = (i_var3 + 0x12);
            u_var2 = (i_var3 + 0x14);
        }
        0x18b => {
            u_var1 = (i_var3 + 0x16);
            u_var2 = (i_var3 + 0x18);
        }
        0x18c => {
            u_var1 = (i_var3 + 0x1a);
            u_var2 = (i_var3 + 0x1c);
        }
        0x18d => {
            u_var1 = (i_var3 + 0x1e);
            u_var2 = (i_var3 + 0x20);
        }
    }
    uStack6 = CONCAT22(u_var2, u_var1);
    iStack8 = 0x0;
    pass1_1008_5784(CONCAT22(param_4, local_10), uStack6);
    loop {
        lVar4 = pass1_1008_5b12(local_10, param_4);
        // u_var2 = (lVar4 >> 0x10);
        if ((lVar4 == 0x0) || (iStack8 == param_2)) {
            break;
        }
        iStack8 += 0x1;
    }
    uStack22 = 0x0;
    if (lVar4 != 0x0) {
        if ((lVar4 + 0xa) == 0x0) {
            uStack22 = (lVar4 + 0x8);
        } else {
            uStack22 = 0xffff;
        }
    }
    return uStack22;
}

pub fn pass1_1018_3cda(param_1: U32Ptr, param_2: &mut String, param_3: &mut String) {
    let ppcVar1: u32;
    let u_var2: u16;
    let extraout_dx: u16;
    let u_var3: u16;
    let iVar5: &mut Struct506;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    iVar5 = param_1;
    ppcVar1 = (*param_1 + 0x10);
    (**ppcVar1)();
    u_var3 = extraout_dx;
    fn_ptr_1000_17ce(ctx, &iVar5.field_0x126, 0x1000);
    fn_ptr_1000_17ce(ctx, &iVar5.field_0x12a, 0x1000);
    u_var2 = str_op_1008_60e8(param_3, u_var3);
    iVar5.field_0x126 = u_var2;
    iVar5.field_0x128 = u_var3;
    u_var2 = str_op_1008_60e8(param_2, u_var3);
    iVar5.field_0x12a = u_var2;
    iVar5.field_0x12c = u_var3;
    return;
}

pub fn pass1_1018_3d44(param_1: u32, param_2: U32Ptr, param_3: U32Ptr) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x126);
    *param_2 = (param_1 + 0x12a);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_3d6c(param_1: u32) {
    let bVar1: u8;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let iVar6: &mut Struct679;
    let u_var5: u16;
    let u_var6: u32;
    let uVar7: u32;

    // u_var5 = (param_1 >> 0x10);
    iVar6 = param_1;
    u_var4 = iVar6.field_0x142;
    u_var2 = u_var4 + 0x1e;
    if (iVar6.field_0x144 + 0x1 == (u_var4 < 0xffe2)) {
        if (u_var2 != 0x3c) {
            if (0x3c < u_var2) {
                return;
            }
            bVar1 = u_var2;
            if (bVar1 == 0x14) {
                iVar6.field_0x142 = 0xffec;
                //LAB_1018_3e3d:
                iVar6.field_0x144 = -0x1;
                return;
            }
            if (0x14 < bVar1) {
                if (bVar1 == 0x1e) {
                    if (ctx.PTR_LOOP_1050_13ae < 0x1) {
                        return;
                    }
                    if (SBORROW2(ctx.PTR_LOOP_1050_13ae, 0x1)) {
                        return;
                    }
                    if (ctx.PTR_LOOP_1050_13ae != &ctx.PTR_LOOP_1050_0002
                        && 0x0 < (ctx.PTR_LOOP_1050_13ae + -0x1))
                    {
                        pu_var3 = ctx.PTR_LOOP_1050_13ae + -0x3;
                        if (pu_var3 == 0x0) {
                            pass1_1008_612e(0x1, 0x64, 0x0);
                            if (pu_var3 < 0x32) {
                                u_var4 = 0xa;
                            } else {
                                u_var4 = 0xfff6;
                            }
                            iVar6.field_0x142 = u_var4;
                            iVar6.field_0x144 = u_var4 >> 0xf;
                            return;
                        }
                        if (pu_var3 != (&ctx.PTR_LOOP_1050_0000 + 0x1)) {
                            return;
                        }
                        iVar6.field_0x142 = 0xfff6;
                        //             TODO: goto LAB_1018_3e3d;
                    }
                    iVar6.field_0x142 = 0xa;
                } else {
                    if (bVar1 == 0x28) {
                        iVar6.field_0x142 = 0x14;
                    } else {
                        if (bVar1 != 0x32) {
                            return;
                        }
                        iVar6.field_0x142 = 0x1e;
                    }
                }
                iVar6.field_0x144 = 0x0;
                return;
            }
            if (bVar1 != 0x0) {
                if (bVar1 != 0xa) {
                    return;
                }
                iVar6.field_0x142 = 0xffe2;
                //         TODO: goto LAB_1018_3e3d;
            }
        }
        uVar7 = 0x5;
        u_var6 = pass1_1030_8326();
        if (u_var6 % uVar7 == 0x0) {
            &iVar6.field_0x142 = 0x0;
            return;
        }
    }
    return;
}

pub fn pass1_1018_3e8c(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: U32Ptr) {
    *param_4 = 0x1;
    *param_3 = 0x19;
    return;
}

pub fn pass1_1018_3ea4(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    pass1_1008_cac6(param_1);
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 0x136);
    u_var2 = (i_var4 + 0x138);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(0x1008, pu_var1, u_var2, 0x1);
    }
    (i_var4 + 0x136) = 0x0;
    return;
}

pub fn pass1_1018_427c(param_1: u32) {
    let i_var1: i16;
    let in_AX: u16;
    let in_DX: u16;
    let u_var2: u16;
    let u_var3: u16;
    let unaff_SS: u16;
    let u_var4: u32;
    let lVar5: i32;

    // u_var3 = (param_1 >> 0x10);
    u_var2 = param_1;
    u_var4 = switch_1018_3b9e(param_1, (u_var2 + 0x12e), in_AX, in_DX, unaff_SS);
    i_var1 = (u_var2 + 0x12e);
    if (i_var1 == 0x188) {
        lVar5 = pass1_1008_57f0(u_var4, (u_var2 + 0x130), unaff_SS);
        pass1_1018_456a(u_var2, u_var3, (lVar5 + 0xe));
    } else {
        if (i_var1 == 0x18b) {
            lVar5 = pass1_1008_57f0(u_var4, (u_var2 + 0x130), unaff_SS);
            pass1_1018_45d4(u_var2, u_var3, (lVar5 + 0xe));
        } else {
            if (i_var1 == 0x18c) {
                lVar5 = pass1_1008_57f0(u_var4, (u_var2 + 0x130), unaff_SS);
                pass1_1018_451e(u_var2, u_var3, (lVar5 + 0xe));
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_435e(
    param_1: u32,
    param_2: i32,
    param_3: i16,
    param_4: i16,
    param_5: u16,
    param_6: u16,
) {
    let u_var1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;

    if (param_3 < param_4) {
        param_4 = param_3;
    }
    u_var2 = 0x0;
    // u_var4 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x122);
    pass1_1008_e852(
        u_var1,
        (u_var1 >> 0x10),
        (param_1 + 0x126),
        param_6,
        param_5,
    );
    pass1_1030_8344(ctx, ctx.PTR_LOOP_1050_5748, CONCAT22(param_5, u_var2));
    loop {
        loop {
            u_var3 = u_var2;
            pass1_1008_612e(param_4, param_3, u_var3);
            u_var2 = (u_var3 * 0x2 + 0x411c);
            if (u_var2 == 0x0) == false {
                break;
            }
        }
        if u_var2 != 0x1 {
            pass1_1008_612e(0x1, u_var2, u_var2);
        }
        u_var2 -= 0x1;
        switch_1018_3ee6(param_1, param_2, u_var2, u_var3, param_5);
        param_5 |= u_var2;
        if (param_5 == 0x0) == false {
            break;
        }
    }
    return;
}

pub fn pass1_1018_451e(param_1: u16, param_2: u16, param_3: i16) -> u16 {
    let uStack6: u16;

    if param_3 == 0x7 {
        uStack6 = 0x9;
    } else {
        if param_3 == 0x8 {
            uStack6 = 0xa;
        } else {
            if param_3 == 0xc {
                uStack6 = 0x19;
            } else {
                if param_3 == 0xd {
                    uStack6 = 0x3;
                } else {
                    uStack6 = 0x8;
                }
            }
        }
    }
    return uStack6;
}

pub fn pass1_1018_456a(param_1: u16, param_2: u16, param_3: u16) -> u16 {
    let uStack6: u16;

    if (false) {
        // switchD_1018_45a3_caseD_17:
        uStack6 = 0x1;
    } else {
        match (param_3) {
            0x11 | 0x12 | 0x13 | 0x14 | 0x15 => {
                uStack6 = 0x2;
            }
            0x16 | 0x1e => {
                uStack6 = 0x3;
            }
            _ => {}
            //       TODO: goto switchD_1018_45a3_caseD_17;
            0x1d | 0x21 => {
                uStack6 = 0x4;
            }
        }
    }
    return uStack6;
}

pub fn pass1_1018_45d4(param_1: u16, param_2: u16, param_3: i16) -> u16 {
    let uStack6: u16;

    if (param_3 == 0x3) {
        uStack6 = 0x16;
    } else {
        if (param_3 == 0x4) {
            uStack6 = 0x17;
        } else {
            uStack6 = 0x14;
        }
    }
    return uStack6;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_4608(param_1: u16, param_2: u32, param_3: u32, param_4: u32) -> u32 {
    let u_var1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let lVar7: i32;
    let mut pcVar8: String;
    let mut pcVar9: String;
    let uStack26: u32;
    let uStack22: u32;
    let local_a: [u8; 8];

    u_var1 = (param_2 + 0x122);
    pass1_1008_5784(CONCAT22(param_1, local_a), (u_var1 + 0xa));
    loop {
        lVar7 = pass1_1008_5b12(local_a, param_1);
        // u_var5 = (lVar7 >> 0x10);
        u_var2 = lVar7;
        u_var6 = u_var5 | u_var2;
        if (lVar7 == 0x0) {
            return 0x0;
        }
        u_var1 = (u_var2 + 0x4);
        u_var3 = u_var2;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
        uStack22 = CONCAT22(u_var6, u_var3);
        u_var1 = (u_var2 + 0x8);
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
        uStack26 = CONCAT22(u_var6, u_var3);
        pcVar8 = pass1_1038_4d28(uStack22);
        pcVar9 = pass1_1038_4d28(uStack26);
        i_var4 = pass1_1000_3d7a(param_4, pcVar8);
        if ((i_var4 == 0x0) && (i_var4 = pass1_1000_3d7a(param_3, pcVar9), i_var4 == 0x0)) {
            break;
        }
        i_var4 = pass1_1000_3d7a(param_3, pcVar8);
        if ((i_var4 == 0x0) && (i_var4 = pass1_1000_3d7a(param_4, pcVar9), i_var4 == 0x0)) {
            return lVar7;
        }
    }
    return lVar7;
}

pub fn pass1_1018_46e6(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16 {
    pass1_1018_33b4(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_4760(param_1: U32Ptr) {
    let i_var2: &mut Struct507;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var2 = param_1;
    *param_1 = &ctx.PTR_LOOP_1050_4aa6;
    i_var2.field_0x2 = 0x1018;
    fn_ptr_1000_17ce(ctx, i_var2.field_0x4, 0x1000);
    *param_1 = 0x389a;
    i_var2.field_0x2 = 0x1008;
    return;
}

pub fn pass1_1018_4808(param_1: U32Ptr, param_2: u32, param_3: u32, param_4: u32) {
    let i_var1: i16;
    let u_var2: u16;

    struct_1018_4720(param_1, param_2, param_3);
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0xe) = param_4;
    *param_1 = &ctx.PTR_LOOP_1050_4aa2;
    (i_var1 + 0x2) = 0x1018;
    (i_var1 + 0xc) = 0x3;
    return;
}

pub fn pass1_1018_4882(param_1: U32Ptr) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = &ctx.PTR_LOOP_1050_4a8e;
    (param_1 + 0x2) = 0x1018;
    fn_ptr_1000_17ce(ctx, (param_1 + 0x10), 0x1000);
    pass1_1018_4760(param_1);
    return;
}

pub fn pass1_1018_495a(param_1: U32Ptr, param_2: u8) -> u16 {
    pass1_1018_4760(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_4980(param_1: U32Ptr, param_2: u8) -> u16 {
    pass1_1018_4760(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_49a6(param_1: U32Ptr, param_2: u8) -> u16 {
    pass1_1018_4760(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_49cc(param_1: U32Ptr, param_2: u8) -> u16 {
    pass1_1018_4760(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_49f2(param_1: U32Ptr, param_2: u8) -> u16 {
    pass1_1018_4882(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_4a18(param_1: U32Ptr, param_2: u8) -> u16 {
    pass1_1018_4760(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_4a3e(param_1: U32Ptr, param_2: u8) -> u16 {
    pass1_1018_4760(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_4a64(param_1: U32Ptr, param_2: u8) -> u16 {
    pass1_1018_4760(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_4aaa(
    param_1: &mut Struct19,
    param_2: &mut Struct19,
    param_3: u16,
    param_4: U32Ptr,
    param_5: u16,
) {
    struct_op_1018_4cda(param_1, param_2, param_3);
    CONCAT22(param_2, param_1) = 0x4b06;
    (param_1 + 0x2) = 0x1018;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x9a, param_4, param_5);
    ctx.PTR_LOOP_1050_4230 = CONCAT22(param_2, param_1);
    return;
}

pub fn pass1_1018_4ae0(param_1: &mut Struct11, param_2: u8, param_3: u16) -> Struct11 {
    clenaup_win_ui_1018_4d22(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_4b78(param_1: U32Ptr, param_2: u16) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let pu_var5: u32;

    pu_var2 = param_1._2_2_;
    pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24((param_1 + 0xa))), 0x0, 0x8);
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x18)), 0x0, 0x8);
    puVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_2, pu_var2, param_1._2_2_);
    pu_var5 = pass1_1010_5f7a(puVar4, (puVar4 >> 0x10), 0x0, (param_1 + 0x12));
    // u_var3 = (pu_var5 >> 0x10);
    if ((u_var3 | pu_var5) != 0x0) {
        (param_1 + 0xa) = *pu_var5;
        (param_1 + 0xe) = (pu_var5 + 0x4);
    }
    ppcVar1 = (*param_1 + 0x20);
    (**ppcVar1)(0x1010, param_1);
    if (((param_1 + 0xe) == 0x0) && ((param_1 + 0x10) == 0x0)) {
        (param_1 + 0xa) = (param_1 + 0x18);
        (param_1 + 0xc) = (param_1 + 0x1a);
    }
    (param_1 + 0xe) = (param_1 + 0x1c);
    (param_1 + 0x10) = (param_1 + 0x1e);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_4c2c(param_1: u32, param_2: U32Ptr, param_3: u16, param_4: u16) {
    let pu_var1: U32Ptr;

    (param_1 + 0xa) = *param_2;
    (param_1 + 0xe) = param_2[0x1];
    pu_var1 = mixed_1010_20ba(
        ctx.PTR_LOOP_1050_0ed0,
        0x2,
        param_4,
        param_1._2_2_,
        param_1._2_2_,
    );
    pass1_1010_5fb0(
        pu_var1,
        0x0,
        (param_1 + 0xa),
        param_1._2_2_,
        (param_1 + 0x12),
    );
    return;
}

pub fn pass1_1018_4c78(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16 {
    pass1_1010_1d80(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_4dce(param_1: U32Ptr, param_2: u16, param_3: U32Ptr, param_4: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let unaff_DI: i16;
    let pu_var3: U32Ptr;

    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x48, param_4, param_3, unaff_DI);
    // u_var2 = (pu_var3 >> 0x10);
    ppcVar1 = (*param_1 + 0x10);
    (**ppcVar1)(0x1010, param_1, param_2, (pu_var3 + 0xc), (pu_var3 + 0xa));
    return;
}

pub fn pass1_1018_5032(param_1: &mut Struct11, param_2: u8, param_3: u16) -> Struct11 {
    clenaup_win_ui_1018_4d22(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_5070(param_1: &mut Struct641, param_2: u16, param_3: u16) {
    set_struct_fields_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0xa = 0x0;
    param_1.field_0xe = 0x0;
    param_1.field_0x12 = 0x0;
    param_1.field_0x16 = 0x0;
    CONCAT22(param_2, param_1) = 0x56d2;
    param_1.field_0x2 = 0x1018;
    return;
}

pub fn pass1_1018_50ac(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x56d2;
    (i_var4 + 0x2) = 0x1018;
    pu_var1 = (i_var4 + 0xe);
    u_var2 = (i_var4 + 0x10);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_50ea(param_1: u32, param_2: u16, param_3: u32) {
    let i_var1: i16;
    let ppcVar2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let i_var8: i16;
    let uVar9: u16;
    let u_var10: u16;
    let paStack6: &mut Struct99;

    paStack6 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
    // u_var6 = (paStack6 >> 0x10);
    u_var4 = paStack6;
    if ((u_var6 | u_var4) == 0x0) {
        paStack6 = 0x0;
    } else {
        paStack6.field_0x0 = 0x389a;
        (u_var4 + 0x2) = 0x1008;
        (u_var4 + 0x4) = 0x0;
        (u_var4 + 0x6) = 0x0;
        (u_var4 + 0x8) = 0x0;
        (u_var4 + 0xa) = 0x0;
        (u_var4 + 0xc) = 0x0;
        paStack6.field_0x0 = 0x56ce;
        (u_var4 + 0x2) = 0x1018;
    }
    // uVar9 = (paStack6 >> 0x10);
    uVar7 = paStack6;
    (uVar7 + 0xa) = param_2;
    // u_var10 = (param_1 >> 0x10);
    i_var8 = param_1;
    u_var3 = (i_var8 + 0xa);
    i_var1 = (u_var3 + 0xc);
    if (i_var1 == 0x1) {
        u_var3 = (i_var8 + 0xa);
        u_var5 = (u_var3 + 0xe);
        (uVar7 + 0x4) = u_var5;
    } else {
        if (i_var1 == 0x5) {
            u_var3 = (i_var8 + 0xa);
            u_var5 = (u_var3 + 0xe);
            (uVar7 + 0x6) = u_var5;
        } else {
            if (i_var1 != 0x6) {
                if ((uVar9 | uVar7) == 0x0) {
                    return;
                }
                ppcVar2 = paStack6;
                (**ppcVar2)();
                return;
            }
            u_var3 = (i_var8 + 0xa);
            u_var5 = (u_var3 + 0xe);
            (uVar7 + 0x8) = u_var5;
        }
    }
    pass1_1030_6c66(param_3, 0x1, paStack6, u_var5, (u_var6 | u_var4), 0x1030);
    return;
}

pub fn pass1_1018_51d2(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 0xe);
    u_var2 = (i_var4 + 0x10);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    (i_var4 + 0xe) = 0x0;
    return;
}

pub fn pass1_1018_5206(param_1: u32, param_2: u32, param_3: u16) -> u32 {
    let i_var1: i16;
    let u_var2: u16;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u32;
    let local_a: [u8; 8];

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0xa) = 0x0;
    pass1_1008_5784(CONCAT22(param_3, local_a), (i_var3 + 0xe));
    loop {
        u_var5 = pass1_1008_5b12(local_a, param_3);
        // u_var2 = (u_var5 >> 0x10);
        (i_var3 + 0xa) = u_var5;
        (i_var3 + 0xc) = u_var2;
        if ((u_var2 | (i_var3 + 0xa)) == 0x0) {
            break;
        }
        u_var5 = (i_var3 + 0xa);
        i_var1 = pass1_1000_3d7a((u_var5 + 0x4), param_2);
        if (i_var1 != 0x0) == false {
            break;
        }
    }
    return CONCAT22((i_var3 + 0xc), (i_var3 + 0xa));
}

pub fn pass1_1018_526a(param_1: u32, param_2: u32, param_3: u16) -> u32 {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0xe) == 0x0) {
        pass1_1018_5292(param_1 & 0xffff | u_var2 << 0x10, param_2, param_3);
    }
    return CONCAT22((i_var1 + 0x10), (i_var1 + 0xe));
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_5292(param_1: u32, param_2: u32, param_3: u16) {
    let i_var1: i16;
    let u_var2: u32;
    let ppc_var3: u32;
    let u_var4: u16;
    let BVar5: bool;
    let puVar6: U32Ptr;
    let iVar7: i16;
    let mut pcVar8: String;
    let uVar9: u16;
    let puVar10: u32;
    let puVar11: u32;
    let uVar12: u32;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let uVar13: u16;
    let extraout_DX_01: U32Ptr;
    let puVar14: U32Ptr;
    let extraout_DX_02: u16;
    let extraout_DX_03: u16;
    let puVar15: U32Ptr;
    let extraout_DX_04: U32Ptr;
    let uVar16: u16;
    let extraout_DX_05: u16;
    let extraout_DX_06: u16;
    let extraout_DX_07: U32Ptr;
    let extraout_DX_08: U32Ptr;
    let iVar17: i16;
    let uVar18: u16;
    let uVar19: u16;
    let pu_var20: U32Ptr;
    let uStack50: u16;
    let local_26: [u8; 8];
    let uStack30: u32;
    let uStack26: u32;
    let uStack22: u32;
    let puStack18: u32;
    let puStack16: U32Ptr;
    let puStack14: u32;
    let puStack12: U32Ptr;
    let uStack10: u16;
    let uStack8: u32;
    let uStack4: u16;

    // uVar18 = (param_1 >> 0x10);
    iVar17 = param_1;
    puStack18 = (iVar17 + 0xe);
    uVar12 = ZEXT24(puStack18);
    puVar14 = (iVar17 + 0x10);
    puStack16 = puVar14;
    puStack14 = puStack18;
    puStack12 = puVar14;
    if ((puVar14 | puStack18) != 0x0) {
        ppc_var3 = *puStack18;
        (**ppc_var3)();
        puVar14 = extraout_dx;
    }
    mem_op_1000_179c(0xc, puVar14, 0x1000);
    puStack18 = uVar12;
    puStack16 = puVar14;
    if ((puVar14 | puStack18) == 0x0) {
        uVar12 = 0x0;
        puVar14 = 0x0;
    } else {
        set_struct_1008_574a((uVar12 & 0xffff | ZEXT24(puVar14) << 0x10));
        puVar14 = extraout_DX_00;
    }
    (iVar17 + 0xe) = uVar12;
    (iVar17 + 0x10) = puVar14;
    // TODO: refactor for loop
    // for (uStack4 = 0x21; -0x1 < uStack4; uStack4 -= 0x1) {
    //   uStack22 = pass1_1030_7c28(param_2,uStack4,uVar12,puVar14,param_3);
    //   uVar12 = uStack22 & 0xffff;
    //   uVar13 = uVar12;
    //   puVar14 = ((uStack22 >> 0x10) | uVar13);
    //   if (puVar14 != 0x0) {
    //     string_1020_c0ca(uStack4);
    //     u_var4 = str_op_1008_60e8(CONCAT22(puVar14,uVar13),puVar14);
    //     uVar12 = u_var4;
    //     uStack26 = CONCAT22(puVar14,u_var4);
    //     mem_op_1000_179c(0x10,puVar14,0x1000);
    //     puStack14 = uVar12;
    //     puStack12 = puVar14;
    //     if ((puVar14 | puStack14) == 0x0) {
    //       uVar12 = 0x0;
    //       uVar13 = 0x0;
    //     }
    //     else {
    //       struct_1018_4790(uVar12 & 0xffff | ZEXT24(puVar14) << 0x10,uStack22,uStack26,
    //                        uStack4);
    //       uVar13 = extraout_DX_02;
    //     }
    //     uStack30 = uVar12 & 0xffff | uVar13 << 0x10;
    //     u_var2 = (iVar17 + 0xe);
    //     ppc_var3 = ((iVar17 + 0xe) + 0x4);
    //     (**ppc_var3)(0x1000,u_var2,(u_var2 >> 0x10),uVar12,uVar13);
    //     puVar14 = extraout_DX_01;
    //   }
    // }
    uStack8 = struct_op_1030_73a8(param_2);
    uStack10 = (uStack8 + 0xc);
    BVar5 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, uStack10, 0x4);
    if (BVar5 != 0x0) {
        uStack30 = uStack8;
        uStack26 = (uStack8 + 0x20);
        pass1_1008_5784(CONCAT22(param_3, local_26), uStack26);
        loop {
            puVar6 = local_26;
            pass1_1008_5b12(puVar6, param_3);
            uStack22 = CONCAT22(extraout_DX_03, puVar6);
            puVar14 = (extraout_DX_03 | puVar6);
            if (puVar14 == 0x0) {
                break;
            }
            i_var1 = (puVar6 + 0x6);
            iVar7 = i_var1 + -0x7;
            if (iVar7 == 0x0) {
                //LAB_1018_53f0:
                pcVar8 = string_op_1020_c222((puVar6 + 0x6));
                uVar9 = str_op_1008_60e8(CONCAT22(puVar14, pcVar8), puVar14);
                puVar15 = puVar14;
                u_var4 = uVar9;
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack18 = u_var4;
                puStack16 = puVar15;
                if ((puVar15 | u_var4) == 0x0) {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                } else {
                    // uVar19 = (uStack22 >> 0x10);
                    pu_var20 = struct_1018_48b0(
                        CONCAT22(puVar15, u_var4),
                        (uStack22 + 0xa),
                        CONCAT22(puVar14, uVar9),
                        (uStack22 + 0x6),
                    );
                    // uVar16 = (pu_var20 >> 0x10);
                    uVar19 = SUB42(pu_var20, 0x0);
                }
                u_var2 = (iVar17 + 0xe);
                ppc_var3 = ((iVar17 + 0xe) + 0x4);
                (**ppc_var3)(0x1000, u_var2, (u_var2 >> 0x10), uVar19, uVar16);
                puVar14 = extraout_DX_04;
            } else {
                if (((0x5 < iVar7) && (!SBORROW2(iVar7, 0x6))) && (i_var1 + -0xd < 0x2)) {}
                //         TODO: goto LAB_1018_53f0;
            }
            // uVar19 = (uStack22 >> 0x10);
            if ((uStack22 + 0x8) != 0x0) {
                pcVar8 = string_op_1020_c2f8((uStack22 + 0x8));
                puVar10 = str_op_1008_60e8(CONCAT22(puVar14, pcVar8), puVar14);
                puVar15 = puVar14;
                puVar11 = puVar10;
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack14 = puVar11;
                puStack12 = puVar15;
                if ((puVar15 | puVar11) == 0x0) {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                } else {
                    // uVar19 = (uStack22 >> 0x10);
                    pu_var20 = struct_1018_48e8(
                        CONCAT22(puVar15, puVar11),
                        (uStack22 + 0xa),
                        CONCAT22(puVar14, puVar10),
                        (uStack22 + 0x8),
                    );
                    // uVar16 = (pu_var20 >> 0x10);
                    uVar19 = SUB42(pu_var20, 0x0);
                }
                u_var2 = (iVar17 + 0xe);
                ppc_var3 = ((iVar17 + 0xe) + 0x4);
                (**ppc_var3)(0x1000, u_var2, (u_var2 >> 0x10), uVar19, uVar16);
            }
        }
    }
    // uVar19 = (param_2 >> 0x10);
    uVar12 = (param_2 + 0x3e);
    uVar13 = (param_2 + 0x40);
    uStack50 = uVar12;
    if ((uVar13 | uStack50) != 0x0) {
        pass1_1008_5784(
            CONCAT22(param_3, local_26),
            uVar12 & 0xffff | uVar13 << 0x10,
        );
        loop {
            puVar6 = local_26;
            pass1_1008_5b12(puVar6, param_3);
            puVar14 = (extraout_DX_05 | puVar6);
            if (puVar14 == 0x0) {
                break;
            }
            if ((puVar6 + 0x4) != 0x0) {
                pcVar8 = string_1020_c0d8((puVar6 + 0x4));
                uVar13 = str_op_1008_60e8(CONCAT22(puVar14, pcVar8), puVar14);
                uStack30 = CONCAT22(puVar14, uVar13);
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack18 = uVar13;
                puStack16 = puVar14;
                if ((puVar14 | uVar13) == 0x0) {
                    uVar13 = 0x0;
                    uVar19 = 0x0;
                } else {
                    struct_1018_4790(
                        CONCAT22(puVar14, uVar13),
                        (puVar6 + 0xa),
                        uStack30,
                        (puVar6 + 0x4),
                    );
                    uVar19 = extraout_DX_06;
                }
                uStack26 = CONCAT22(uVar19, uVar13);
                u_var2 = (iVar17 + 0xe);
                ppc_var3 = ((iVar17 + 0xe) + 0x4);
                (**ppc_var3)(0x1000, u_var2, (u_var2 >> 0x10), uVar13, uVar19);
                puVar14 = extraout_DX_07;
            }
            if ((puVar6 + 0x6) != 0x0) {
                pcVar8 = string_op_1020_c222((puVar6 + 0x6));
                puVar11 = str_op_1008_60e8(CONCAT22(puVar14, pcVar8), puVar14);
                uStack30 = CONCAT22(puVar14, puVar11);
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack14 = puVar11;
                puStack12 = puVar14;
                if ((puVar14 | puVar11) == 0x0) {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                } else {
                    pu_var20 = struct_1018_48b0(
                        CONCAT22(puVar14, puVar11),
                        (puVar6 + 0xa),
                        uStack30,
                        (puVar6 + 0x6),
                    );
                    // uVar16 = (pu_var20 >> 0x10);
                    uVar19 = SUB42(pu_var20, 0x0);
                }
                uStack26 = CONCAT22(uVar16, uVar19);
                u_var2 = (iVar17 + 0xe);
                ppc_var3 = ((iVar17 + 0xe) + 0x4);
                (**ppc_var3)(0x1000, u_var2, (u_var2 >> 0x10), uVar19, uVar16);
                puVar14 = extraout_DX_08;
            }
            if ((puVar6 + 0x8) != 0x0) {
                pcVar8 = string_op_1020_c2f8((puVar6 + 0x8));
                uVar13 = str_op_1008_60e8(CONCAT22(puVar14, pcVar8), puVar14);
                uStack30 = CONCAT22(puVar14, uVar13);
                mem_op_1000_179c(0x10, puVar14, 0x1000);
                puStack18 = uVar13;
                puStack16 = puVar14;
                if ((puVar14 | uVar13) == 0x0) {
                    uVar19 = 0x0;
                    uVar16 = 0x0;
                } else {
                    pu_var20 = struct_1018_48e8(
                        CONCAT22(puVar14, uVar13),
                        (puVar6 + 0xa),
                        uStack30,
                        (puVar6 + 0x8),
                    );
                    // uVar16 = (pu_var20 >> 0x10);
                    uVar19 = SUB42(pu_var20, 0x0);
                }
                uStack26 = CONCAT22(uVar16, uVar19);
                u_var2 = (iVar17 + 0xe);
                ppc_var3 = ((iVar17 + 0xe) + 0x4);
                (**ppc_var3)(0x1000, u_var2, (u_var2 >> 0x10), uVar19, uVar16);
            }
        }
    }
    return;
}

pub fn pass1_1018_567c(param_1: U32Ptr, param_2: u8) -> u16 {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = 0x389a;
    (param_1)[0x1] = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        pass1_1000_093a(param_1, u_var1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_56a8(param_1: u32, param_2: u8, param_3: u16) -> u32 {
    pass1_1018_50ac(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_56e6(param_1: &mut Struct19, param_2: u16, param_3: u16) -> u16 {
    set_struct_fields_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 0xa) = 0x0;
    CONCAT22(param_2, param_1) = 0x5830;
    (param_1 + 0x2) = 0x1018;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1018_5714(param_1: U32Ptr, param_2: u16) {
    *param_1 = 0x5830;
    (param_1 + 0x2) = 0x1018;
    pass1_1010_1d80(param_1, param_2);
    return;
}

pub fn pass1_1018_5732(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) -> u32 {
    let u_var1: u32;

    u_var1 = pass1_1030_6d4e(param_3, param_4, param_5, param_6);
    return u_var1;
}

pub fn pass1_1018_5742(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: u32) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let u_var3: u32;
    let bVar4: bool;
    let pu_var5: u32;
    let u_var6: u32;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let uStack16: u32;

    bVar4 = false;
    pu_var1 = (param_3 + 0x4);
    ppcVar2 = (*pu_var1 + 0x10);
    pu_var5 = pu_var1;
    (**ppcVar2)();
    u_var3 = pu_var5 & 0xffff | extraout_dx << 0x10;
    uStack16 = 0x0;
    loop {
        if (u_var3 <= uStack16) {
            //LAB_1018_579f:
            if (!bVar4) {
                if (param_3 != 0x0) {
                    ppcVar2 = *param_3;
                    (**ppcVar2)();
                }
                param_3 = 0x0;
            }
            pass1_1030_6d80(param_4, param_3);
            return;
        }
        ppcVar2 = (*pu_var1 + 0x4);
        u_var6 = u_var3;
        (**ppcVar2)();
        if ((extraout_DX_00 | u_var6) != 0x0) {
            bVar4 = true;
            //       TODO: goto LAB_1018_579f;
        }
        uStack16 += 0x1;
    }
}

pub fn pass1_1018_57d2(param_1: u32, param_2: u32) {
    (param_1 + 0xa) = param_2;
    return;
}

pub fn pass1_1018_57e6(param_1: u32, param_2: i32, param_3: u16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    send_dlg_item_msg_1040_d20c((param_1 + 0xa), param_2, &ctx.PTR_LOOP_1050_1040, param_3);
    (param_1 + 0xa) = 0x0;
    return;
}

pub fn pass1_1018_580a(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16 {
    pass1_1018_5714(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn struct_1018_5840(param_1: &mut Struct20, param_2: u16, param_3: u16, param_4: u16) {
    let extraout_dx: U32Ptr;
    let u_var1: u16;
    let i_var2: &mut Struct130;
    let unaff_DI: i16;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    unk_draw_op_1020_7f7a(param_1, 0x6, CONCAT22(param_3, param_2));
    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    i_var2.field_0xee = 0x0;
    &i_var2.field_0xf2 = 0x0;
    i_var2.field_0xf6 = 0x0;
    param_1.field_0x0 = (s_Alloc__s_1050_5a5b + 0x7);
    i_var2.field_0x2 = 0x1018;
    i_var2.field_0xe2 = 0x5afe;
    i_var2.field_0xe4 = 0x1018;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x27, param_4, extraout_dx, unaff_DI);
    // u_var1 = (pu_var3 >> 0x10);
    i_var2.field_0xf2 = pu_var3;
    i_var2.field_0xf4 = u_var1;
    i_var2.field_0xe6 = i_var2.field_0xf2;
    i_var2.field_0xe8 = u_var1;
    return;
}

pub fn pass1_1018_58b6(param_1: U32Ptr) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = (s_Alloc__s_1050_5a5b + 0x7);
    (i_var1 + 0x2) = 0x1018;
    (i_var1 + 0xe2) = 0x5afe;
    (i_var1 + 0xe4) = 0x1018;
    pass1_1020_808e(param_1);
    return;
}

pub fn pass1_1018_5932(param_1: u32, param_2: u16, param_3: u16) -> u16 {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;

    // u_var4 = (param_1 >> 0x10);
    u_var3 = param_1;
    u_var2 = (u_var3 + 0xf0) | (u_var3 + 0xee);
    if (u_var2 != 0x0) {
        ppcVar1 = ((u_var3 + 0xee) + 0x8);
        u_var5 = (**ppcVar1)();
        // param_2 = (u_var5 >> 0x10);
        u_var2 = u_var5;
    }
    if ((u_var3 + 0xea) == 0x0) {
        (u_var3 + 0xea) = 0x1;
        u_var5 = pass1_1038_af40(
            ctx.PTR_LOOP_1050_5b7c,
            (u_var3 + 0x8),
            ((u_var3 + 0xf6) * 0x2 + 0x4238),
            param_2,
            u_var3,
            &ctx.PTR_LOOP_1050_1038,
            param_3,
        );
        u_var2 = u_var5;
    }
    return u_var2;
}

pub fn pass1_1018_5a2e(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_58b6(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Unable to use type for symbol u_var4
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_5b06(param_1: &mut Struct132, param_2: u16, param_3: u16, param_4: u16) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let pu_var3: u32;
    let u_var5: u16;
    let u_var6: u32;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let puVar9: U32Ptr;
    let u_var10: u16;
    let unaff_DI: i16;
    let puVar11: U32Ptr;
    let paVar12: &mut Struct76;
    let uVar13: u32;
    let uVar14: u16;
    let uVar15: u16;
    let paVar16: &mut Struct132;
    let uVar17: u16;
    let local_c: [u8; 6];
    let puStack6: U32Ptr;
    let u_var4: u16;

    set_struct_op_1020_921c(CONCAT22(param_2, param_1), param_3);
    param_1.field_0x14 = 0x0;
    param_1.field_0x18 = 0x0;
    puVar11 = clear_struct_1008_3e38(CONCAT22(param_2, &param_1.field_0x1c));
    CONCAT22(param_2, param_1) = &ctx.PTR_LOOP_1050_5e1a;
    param_1.field_0x2 = 0x1018;
    puStack6 = mixed_1010_20ba(
        ctx.PTR_LOOP_1050_0ed0,
        0x48,
        param_4,
        (puVar11 >> 0x10),
        unaff_DI,
    );
    puVar11 = clear_struct_1008_3e38(CONCAT22(param_4, local_c));
    // puVar7 = (puVar11 >> 0x10);
    pass1_1008_3f62(
        CONCAT22(param_4, local_c),
        (puStack6 & 0xffff0000 | (puStack6 + 0xe)),
    );
    puVar11 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x27, param_4, puVar7, unaff_DI);
    // uVar8 = (puVar11 >> 0x10);
    &param_1.field_0x14 = puVar11;
    (&param_1.field_0x14 + 0x2) = uVar8;
    uVar15 = 0x0;
    uVar14 = &param_1.field_0x14;
    ppcVar2 = (*param_1.field_0x14 + 0x4);
    paVar16 = param_1;
    uVar17 = param_2;
    (**ppcVar2)();
    param_1.field_0x6 = param_1.field_0x14;
    pu_var3 = param_1.field_0x14;
    pu_var1 = (pu_var3 + 0xa);
    u_var6 = CONCAT22(param_2, &param_1.field_0xa);
    ppcVar2 = (*pu_var1 + 0x8);
    (**ppcVar2)(
        0x1010,
        pu_var1,
        (pu_var1 >> 0x10),
        u_var6,
        uVar14,
        uVar8,
        uVar15,
        paVar16,
        uVar17,
    );
    param_1.field_0x12 = u_var6;
    draw_op_1020_9364(CONCAT22(param_2, param_1), 0x1020, param_4);
    pu_var3 = param_1.field_0x14;
    pass1_1008_3f62(
        CONCAT22(param_2, &param_1.field_0x1c),
        (pu_var3 & 0xffff0000 | (pu_var3 + 0x52)),
    );
    pass1_1008_3f32(
        CONCAT22(param_2, &param_1.field_0x1c),
        CONCAT22(param_4, local_c),
    );
    paVar12 = pass1_1008_9f48(param_1.field_0x14);
    uVar13 = pass1_1008_4772(paVar12);
    // puVar9 = (uVar13 >> 0x10);
    u_var4 = uVar13;
    puVar7 = puVar9;
    u_var5 = u_var4;
    mem_op_1000_179c(0x14, puVar9, 0x1000);
    u_var10 = puVar7 | u_var5;
    if (u_var10 == 0x0) {
        param_1.field_0x18 = 0x0;
    } else {
        pass1_1008_50c2(
            CONCAT22(puVar7, u_var5),
            (u_var4 + 0x8),
            (u_var4 + 0x4),
            CONCAT22(param_2, &param_1.field_0x1c),
            pu_var1,
        );
        &param_1.field_0x18 = u_var5;
        (&param_1.field_0x18 + 0x2) = u_var10;
    }
    pass1_1008_5134(param_1.field_0x18);
    param_1.field_0x22 = param_1.field_0x1c;
    param_1.field_0x24 = param_1.field_0x1e;
    param_1.field_0x26 = (u_var4 + 0x4) + param_1.field_0x22 + 0x1;
    param_1.field_0x28 = (u_var4 + 0x8) + param_1.field_0x24 + 0x1;
    return;
}

pub fn pass1_1018_5cc8(param_1: U32Ptr, param_2: u16) {
    let u_var1: u16;
    let paVar2: &mut Struct18;
    let i_var3: &mut Struct508;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    *param_1 = &ctx.PTR_LOOP_1050_5e1a;
    i_var3.field_0x2 = 0x1018;
    paVar2 = &i_var3.field_0x18;
    u_var1 = i_var3.field_0x1a;
    if ((u_var1 | paVar2) != 0x0) {
        pass1_1008_5118(paVar2 & 0xffff | u_var1 << 0x10);
        fn_ptr_1000_17ce(ctx, paVar2, 0x1000);
    }
    if (i_var3.field_0x14 != 0x0) {
        pass1_1010_1ea6(
            i_var3.field_0x14,
            param_1 & 0xffff | u_var3 << 0x10,
            param_2,
        );
        pass1_1010_1dda(i_var3.field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

pub fn pass1_1018_5df4(param_1: U32Ptr, param_2: u8, param_3: u16) -> u16 {
    pass1_1018_5cc8(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_5e26(param_1: &mut Struct57, param_2: u16) -> Struct57 {
    let u_var1: u16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfd0, param_2);
    // u_var1 = (param_1 >> 0x10);
    param_1 = 0x6128;
    (param_1 + 0x2) = 0x1018;
    (param_1 + 0x74) = 0x1;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_5e5a(param_1: U32Ptr) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = 0x6128;
    (param_1 + 0x2) = 0x1018;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1018_5e86(param_1: U32Ptr) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x6c);
    (**ppcVar1)();
    return;
}

pub fn pass1_1018_5ffa(param_1: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 0x92);
    u_var2 = (i_var4 + 0x94);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    (i_var4 + 0x92) = 0x0;
    pass1_1010_1dda((i_var4 + 0x8e));
    (i_var4 + 0x8e) = 0x0;
    return;
}

pub fn pass1_1018_6048(param_1: u32) -> u16 {
    let ppcVar1: u32;

    ppcVar1 = ((param_1 + 0x92) + 0x8);
    (**ppcVar1)();
    return 0x0;
}

pub fn pass1_1018_6102(param_1: U32Ptr, param_2: u8) -> u16 {
    pass1_1018_5e5a(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_6198(
    param_1: U32Ptr,
    param_2: u32,
    param_3: u16,
    param_4: U32Ptr,
    param_5: i16,
    param_6: u16,
) {
    let i_var1: &mut Struct657;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    *param_1 = 0x3aa8;
    i_var1.field_0x2 = 0x1008;
    i_var1.field_0x4 = param_3;
    *param_1 = 0x3ab0;
    i_var1.field_0x2 = 0x1008;
    &i_var1.field_0x6 = 0x0;
    i_var1.field_0xa = param_2;
    *param_1 = 0x66c0;
    i_var1.field_0x2 = 0x1018;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x39, param_6, param_4, param_5);
    i_var1.field_0x6 = pu_var2;
    i_var1.field_0x8 = (pu_var2 >> 0x10);
    return;
}

pub fn pass1_1018_620c(param_1: U32Ptr) {
    let i_var1: &mut Struct509;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x66c0;
    i_var1.field_0x2 = 0x1018;
    *param_1 = 0x3ab0;
    i_var1.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    return;
}

pub fn pass1_1018_642e(param_1: u16, param_2: u16, param_3: &mut i16, param_4: i16) {
    *param_3 = 0x64 - param_4 >> 0x1;
    return;
}

pub fn pass1_1018_659a(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: U32Ptr,
    param_5: u16,
) -> u32 {
    let pi_var1: U32Ptr;
    let iStack18: i16;
    let local_6: i16;
    let local_4: i16;

    pi_var1 = &local_6;
    pass1_1008_3e94(
        param_3,
        CONCAT22(param_5, pi_var1),
        CONCAT22(param_5, &local_4),
    );
    mem_op_1000_179c(0xc, param_4, 0x1000);
    // TODO: refactor for loop
    // for (iStack18 = 0x0; iStack18 < 0x3; iStack18 += 0x1) {
    //   pi_var1[iStack18 * 0x2] = (iStack18 * 0x4 + 0x4248) + local_4;
    //   pi_var1[iStack18 * 0x2 + 0x1] = (iStack18 * 0x4 + 0x424a) + local_6;
    // }
    return CONCAT22(param_4, pi_var1);
}

pub fn pass1_1018_6630(param_1: u32, param_2: u16, param_3: u16) {
    let u_var1: u32;
    let dialog_id_5: u16;
    let u_var2: u16;
    let i_var3: i16;
    let u_var4: u16;
    let iStack12: i16;
    let SStack10: SEGPTR;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    find_n_load_rsrc_1010_4e9e((i_var3 + 0x6), 0x1010);
    if ((param_3 | param_2) != 0x0) {
        SStack10 = param_2;
        // TODO: refactor for loop
        // for (iStack12 = 0x0; iStack12 < 0x9; iStack12 += 0x1) {
        //   u_var1 = (i_var3 + 0x6);
        //   dialog_id_5 = pass1_1010_4f20(u_var1,(u_var1 >> 0x10),iStack12)
        //   ;
        //   u_var1 = (i_var3 + 0xa);
        //   set_window_text_1018_6066
        //             (u_var1,(u_var1 >> 0x10),SStack10,param_3,dialog_id_5
        //              ,0x1010);
        //   u_var2 = str_op_1000_3da4(CONCAT22(param_3,SStack10));
        //   SStack10 += u_var2 + 0x1;
        // }
    }
    return;
}

pub fn pass1_1018_669a(param_1: u32, param_2: u8) -> u32 {
    pass1_1018_620c(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_673c(param_1: U32Ptr) {
    let i_var1: &mut Struct510;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x6880;
    i_var1.field_0x2 = 0x1018;
    i_var1.field_0xe2 = 0x691c;
    i_var1.field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_6768(param_1: u32, param_2: u16, param_3: u16) -> u16 {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;

    // u_var4 = (param_1 >> 0x10);
    u_var3 = param_1;
    u_var2 = (u_var3 + 0xf0) | (u_var3 + 0xee);
    if (u_var2 != 0x0) {
        ppcVar1 = ((u_var3 + 0xee) + 0x8);
        u_var5 = (**ppcVar1)();
        // param_2 = (u_var5 >> 0x10);
        u_var2 = u_var5;
    }
    if ((u_var3 + 0xea) == 0x0) {
        (u_var3 + 0xea) = 0x1;
        u_var5 = pass1_1038_af40(
            ctx.PTR_LOOP_1050_5b7c,
            (u_var3 + 0x8),
            0x16,
            param_2,
            u_var3,
            &ctx.PTR_LOOP_1050_1038,
            param_3,
        );
        u_var2 = u_var5;
    }
    return u_var2;
}

pub fn pass1_1018_681a(param_1: i32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    pu_var1 = (param_1 + 0xee);
    u_var2 = (param_1 + 0xf0);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

pub fn pass1_1018_684c(param_1: U32Ptr, param_2: u8) -> u16 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_673c(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_6924(
    param_1: &mut Struct658,
    param_2: u16,
    param_3: u16,
    param_4: i16,
    param_5: u16,
) {
    let ppcVar1: u32;
    let u_var2: u32;
    let i_var3: i16;
    let extraout_dx: U32Ptr;
    let u_var4: u16;
    let pu_var5: U32Ptr;

    set_struct_op_1020_921c(CONCAT22(param_2, param_1), param_3);
    &param_1.field_0x14 = 0x0;
    CONCAT22(param_2, param_1) = 0x6a02;
    param_1.field_0x2 = 0x1018;
    pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0xb, param_5, extraout_dx, param_4);
    // u_var4 = (pu_var5 >> 0x10);
    param_1.field_0x14 = pu_var5;
    param_1.field_0x16 = u_var4;
    param_1.field_0x6 = param_1.field_0x14;
    param_1.field_0x8 = u_var4;
    u_var2 = &param_1.field_0x14;
    i_var3 = &param_1.field_0xa;
    ppcVar1 = ((u_var2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_1.field_0x12 = i_var3;
    draw_op_1020_9364(CONCAT22(param_2, param_1), 0x1020, param_5);
    return;
}

pub fn pass1_1018_69ac(param_1: U32Ptr) {
    let i_var1: &mut Struct511;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x6a02;
    i_var1.field_0x2 = 0x1018;
    if (i_var1.field_0x14 != 0x0) {
        pass1_1010_1dda(i_var1.field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

pub fn pass1_1018_69dc(param_1: U32Ptr, param_2: u8) -> u16 {
    pass1_1018_69ac(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_6c1e(param_1: U32Ptr, param_2: u8) {
    let u_var1: &mut Struct512;
    let u_var2: u16;

    u_var1 = param_1;
    u_var1 = u_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(u_var1)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    u_var1.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    u_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_7da6(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_7dee(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_7e36(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_7e7e(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_7ec6(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_7f0e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct671;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_7f56(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_7f9e(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_7fe6(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct672;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_802e(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8076(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_80be(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8106(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_814e(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8196(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_81de(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8226(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_826e(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_82b6(param_1: U32Ptr, param_2: u8) {
    let i_var1: i16;
    let u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    // u_var2 = (param_1 >> 0x10);
    *param_1 = 0x380a;
    (i_var1 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_82fe(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct517;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8346(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct518;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_838e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct519;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_83d6(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct673;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_841e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct520;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8466(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct521;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_84ae(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct522;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_84f6(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct523;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_853e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct524;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8586(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct525;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_85ce(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct526;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8616(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct527;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_865e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct528;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_86a6(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct529;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_86ee(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct530;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8736(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct531;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_877e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct532;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_87c6(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct533;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_880e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct534;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8856(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct535;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_889e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct536;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_88e6(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct537;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_892e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct538;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8976(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct539;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_89be(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct540;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8a06(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct541;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8a4e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct674;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8a96(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct542;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8ade(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct543;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8b26(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct544;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8b6e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct545;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8bb6(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct546;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8bfe(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct547;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8c46(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct548;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8c8e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct549;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8cd6(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct675;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8d1e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct550;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8d66(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct551;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8dae(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct552;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8df6(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct553;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8e3e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct554;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8e86(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct555;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8ece(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct676;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8f16(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct556;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8f5e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct677;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8fa6(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct557;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_8fee(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct558;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_9036(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct559;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_907e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct560;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_90c6(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct561;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_910e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct562;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_9156(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct563;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_919e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct564;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_91e6(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct565;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_922e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct566;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_9276(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct567;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_92be(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct568;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_9306(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct569;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_934e(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct570;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

pub fn pass1_1018_9396(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct571;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_c402(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u32,
    param_6: u32,
    param_7: u32,
    param_8: u32,
    param_9: u16,
    param_10: u16,
) {
    let i_var1: i16;
    let pu_var2: U32Ptr;
    let i_var4: &mut Struct20;
    let unaff_DI: i16;
    let u_var4: &mut Struct20;
    let pu_var3: U32Ptr;

    struct_1020_0762(
        param_1,
        CONCAT22(param_5, param_4),
        CONCAT22(param_6, (param_5 >> 0x10)),
        (param_6 >> 0x10),
        param_7,
        param_8,
        param_9,
    );
    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    i_var4[0x1].field_0x14 = 0x0;
    i_var4[0x1].field_0x16 = 0x0;
    i_var4[0x1].field_0x18 = 0x0;
    i_var4[0x1].field_0x1a = 0x0;
    i_var4[0x1].field_0x1c = 0x2;
    i_var4[0x1].field_0x26 = 0x0;
    i_var4[0x1].field_0x2a = 0x0;
    i_var4[0x1].field_0x2c = 0x1e0190;
    i_var4[0x1].field_0x30 = 0x0;
    param_1.field_0x0 = 0xc8bc;
    i_var4.field_0x2 = 0x1018;
    pu_var2 = pass1_1000_4906(
        (param_1 & 0xffff0000 | ZEXT24(&i_var4[0x1].field_0x1e)),
        0x0,
        0x8,
    );
    if ((param_3 == 0x0) || (param_2 != 0x0)) {
        if ((param_2 & param_3) == 0x0) {
            // goto
            // LAB_1018_c4bb;
        }
        pu_var2 = pass1_1008_5fd8(param_9, param_10);
    } else {
        load_string_1010_84ac(ctx.PTR_LOOP_1050_14cc, 0x1010);
    }
    &i_var4[0x1].field_0x26 = pu_var2;
    (&i_var4[0x1].field_0x26 + 0x2) = param_10;
    //LAB_1018_c4bb:
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x48, param_9, param_10, unaff_DI);
    i_var1 = pu_var3;
    i_var4[0x1].field_0x14 = (i_var1 + 0xa);
    i_var4[0x1].field_0x16 = (i_var1 + 0xc);
    pass1_1008_3e94(
        (pu_var3 & 0xffff0000 | (i_var1 + 0xe)),
        (param_1 & 0xffff0000 | ZEXT24(&i_var4[0x1].field_0x1a)),
        (param_1 & 0xffff0000 | ZEXT24(&i_var4[0x1].field_0x18)),
    );
    return;
}

pub fn pass1_1018_c896(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_c958(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let local_6: [u8; 4];

    u_var3 = 0xf1;
    u_var4 = 0x9a;
    pu_var2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x8d);
    // u_var1 = (pu_var2 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x0,
        0x732,
        0x26,
        CONCAT22(pu_var2, 0x1f40),
        CONCAT22(u_var3, u_var1),
        CONCAT22(param_2, u_var4),
        param_3,
        param_4,
        u_var1,
    );
    param_1.field_0x0 = 0xdc5a;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_c9a6(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let local_6: [u8; 4];

    u_var3 = 0xf2;
    u_var4 = 0xa0;
    pu_var2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x8e);
    // u_var1 = (pu_var2 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x0,
        0x733,
        0x27,
        CONCAT22(pu_var2, 0x1b58),
        CONCAT22(u_var3, u_var1),
        CONCAT22(param_2, u_var4),
        param_3,
        param_4,
        u_var1,
    );
    param_1.field_0x0 = 0xd6de;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_c9f4(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let local_6: [u8; 4];

    u_var3 = 0xf3;
    u_var5 = 0xa6;
    puVar4 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x8f);
    // u_var2 = (puVar4 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x0,
        0x734,
        0x28,
        CONCAT22(puVar4, 0x32c8),
        CONCAT22(u_var3, u_var2),
        CONCAT22(param_2, u_var5),
        param_3,
        param_4,
        u_var2,
    );
    // u_var3 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xda86;
    (param_1 + 0x2) = 0x1018;
    pi_var1 = (param_1 + 0x10e);
    *pi_var1 = *pi_var1 + -0x19;
    return param_1;
}

pub fn pass1_1018_ca48(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let local_6: [u8; 4];

    u_var3 = 0xf4;
    u_var4 = 0xa1;
    pu_var2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x90);
    // u_var1 = (pu_var2 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x0,
        0x735,
        0x29,
        CONCAT22(pu_var2, 0x36b0),
        CONCAT22(u_var3, u_var1),
        CONCAT22(param_2, u_var4),
        param_3,
        param_4,
        u_var1,
    );
    param_1.field_0x0 = 0xd50a;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_ca96(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let local_6: [u8; 4];

    u_var3 = 0xf5;
    u_var5 = 0xbf;
    puVar4 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x92);
    // u_var2 = (puVar4 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x737,
        0x736,
        0x2a,
        CONCAT22(puVar4, 0x6590),
        CONCAT22(u_var3, u_var2),
        CONCAT22(param_2, u_var5),
        param_3,
        param_4,
        u_var2,
    );
    // u_var3 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xd8b2;
    (param_1 + 0x2) = 0x1018;
    pi_var1 = (param_1 + 0x10e);
    *pi_var1 = *pi_var1 + 0x64;
    return param_1;
}

pub fn pass1_1018_caea(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let local_6: [u8; 4];

    u_var3 = 0xf6;
    u_var4 = 0x93;
    pu_var2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x93);
    // u_var1 = (pu_var2 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x0,
        0x738,
        0x2b,
        CONCAT22(pu_var2, 0x2328),
        CONCAT22(u_var3, u_var1),
        CONCAT22(param_2, u_var4),
        param_3,
        param_4,
        u_var1,
    );
    param_1.field_0x0 = 0xdbbe;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cb38(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let local_6: [u8; 4];

    u_var3 = 0xf7;
    u_var4 = 0x94;
    pu_var2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x94);
    // u_var1 = (pu_var2 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x0,
        0x739,
        0x2c,
        CONCAT22(pu_var2, 0x32c8),
        CONCAT22(u_var3, u_var1),
        CONCAT22(param_2, u_var4),
        param_3,
        param_4,
        u_var1,
    );
    param_1.field_0x0 = 0xd642;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cb86(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let local_6: [u8; 4];

    u_var3 = 0xf8;
    u_var5 = 0xc2;
    puVar4 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x96);
    // u_var2 = (puVar4 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x0,
        0x73a,
        0x2d,
        CONCAT22(puVar4, 0x2328),
        CONCAT22(u_var3, u_var2),
        CONCAT22(param_2, u_var5),
        param_3,
        param_4,
        u_var2,
    );
    // u_var3 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xd9ea;
    (param_1 + 0x2) = 0x1018;
    pi_var1 = (param_1 + 0x10e);
    *pi_var1 = *pi_var1 + 0x64;
    return param_1;
}

pub fn pass1_1018_cbda(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let local_6: [u8; 4];

    u_var3 = 0xf9;
    u_var4 = 0xc5;
    pu_var2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x97);
    // u_var1 = (pu_var2 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x0,
        0x73b,
        0x2e,
        CONCAT22(pu_var2, 0x2af8),
        CONCAT22(u_var3, u_var1),
        CONCAT22(param_2, u_var4),
        param_3,
        param_4,
        u_var1,
    );
    param_1.field_0x0 = 0xd46e;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cc28(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let local_6: [u8; 4];
    let u_var3: u16;
    let u_var4: u16;

    u_var3 = 0xfa;
    u_var4 = 0xa3;
    pu_var2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x98);
    // u_var1 = (pu_var2 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x0,
        0x73c,
        0x2f,
        CONCAT22(pu_var2, 0x2710),
        CONCAT22(u_var3, u_var1),
        CONCAT22(param_2, u_var4),
        param_3,
        param_4,
        u_var1,
    );
    param_1.field_0x0 = 0xd816;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cc76(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let local_6: [u8; 4];

    u_var3 = 0xfb;
    u_var4 = 0xa8;
    pu_var2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x99);
    // u_var1 = (pu_var2 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x73e,
        0x73d,
        0x30,
        CONCAT22(pu_var2, 0x61a8),
        CONCAT22(u_var3, u_var1),
        CONCAT22(param_2, u_var4),
        param_3,
        param_4,
        u_var1,
    );
    param_1.field_0x0 = 0xdb22;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_ccc4(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let local_6: [u8; 4];

    u_var3 = 0xfc;
    u_var4 = 0xa9;
    pu_var2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x9b);
    // u_var1 = (pu_var2 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x740,
        0x73f,
        0x31,
        CONCAT22(pu_var2, 0x59d8),
        CONCAT22(u_var3, u_var1),
        CONCAT22(param_2, u_var4),
        param_3,
        param_4,
        u_var1,
    );
    param_1.field_0x0 = 0xd5a6;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cd12(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let local_6: [u8; 4];

    u_var3 = 0xfd;
    u_var4 = 0x7c;
    pu_var2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x9c);
    // u_var1 = (pu_var2 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x0,
        0x741,
        0x32,
        CONCAT22(pu_var2, 0x2710),
        CONCAT22(u_var3, u_var1),
        CONCAT22(param_2, u_var4),
        param_3,
        param_4,
        u_var1,
    );
    param_1.field_0x0 = 0xd94e;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cd60(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let local_6: [u8; 4];

    u_var3 = 0xfe;
    u_var4 = 0xc9;
    pu_var2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x0);
    // u_var1 = (pu_var2 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x0,
        0x0,
        0x33,
        CONCAT22(pu_var2, 0x2710),
        CONCAT22(u_var3, u_var1),
        CONCAT22(param_2, u_var4),
        param_3,
        param_4,
        u_var1,
    );
    param_1.field_0x0 = 0xd3d2;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_cf74(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u32,
    param_4: u16,
) -> Struct20 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let local_6: [u8; 4];

    u_var3 = 0xfe;
    u_var4 = 0xcf;
    pu_var2 = pass1_1008_941a(CONCAT22(param_4, local_6), 0x1, 0x80);
    // u_var1 = (pu_var2 >> 0x10);
    pass1_1018_c402(
        param_1,
        0x0,
        0x0,
        0x34,
        CONCAT22(pu_var2, 0x2710),
        CONCAT22(u_var3, u_var1),
        CONCAT22(param_2, u_var4),
        param_3,
        param_4,
        u_var1,
    );
    param_1.field_0x0 = 0xd77a;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

pub fn pass1_1018_d198(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d1be(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d1e4(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d20a(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d230(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d256(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d27c(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d2a2(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d2c8(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d2ee(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d314(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d33a(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d360(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d386(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_d3ac(param_1: &mut Struct29, param_2: u8) -> Struct29 {
    destroy_window_1018_c518(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_dcf6(param_1: U32Ptr) -> u16 {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    *param_1 = 0xdf3c;
    (param_1 + 0x2) = 0x1018;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_dd1e(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: u16,
    param_5: u16,
    param_6: i16,
    param_7: u32,
) -> u32 {
    let u_var1: u16;
    let uStack6: u32;

    pass1_1010_81f6(0x1010, param_1, ctx.PTR_LOOP_1050_14cc, 0x0, param_7._2_2_);
    uStack6 = CONCAT22(param_3, param_2);
    mem_op_1000_179c(0x46, param_3, 0x1000);
    u_var1 = param_3 | param_2;
    if (u_var1 == 0x0) {
        param_2 = 0x0;
        u_var1 = 0x0;
    } else {
        pass1_1008_87cc(
            CONCAT22(param_3, param_2),
            param_6,
            param_7,
            param_7._2_2_,
            uStack6,
            0x0,
            param_1,
        );
    }
    pass1_1008_8bc6(param_1, u_var1, CONCAT22(u_var1, param_2));
    return CONCAT22(u_var1, param_2);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_dd7c(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: u16,
    param_6: u16,
) {
    let u_var1: u16;
    let u_var2: u32;
    let ppc_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let puVar8: U32Ptr;
    let uVar9: u16;
    let extraout_dx: U32Ptr;
    let puVar10: U32Ptr;
    let unaff_DI: i16;
    let puVar11: U32Ptr;
    let puVar12: u32;
    let iVar13: i16;
    let uVar14: u16;
    let lStack32: i32;
    let uStack20: u16;
    let uStack12: u16;

    u_var5 = param_4._3_1_;
    // iVar13 = (param_3 >> 0x10);
    if (param_4._3_1_ == 0x0) {
        puVar11 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_6, param_5, unaff_DI);
        // puVar8 = (puVar11 >> 0x10);
        if ((puVar11 + 0x1e) == 0x0) {
            uStack20 = param_4;
            uVar14 = param_4;
        } else {
            if (param_4 - 0x7 == 0x0) {
                uStack20 = 0x6;
                param_4._0_2_ = param_4 - 0x7;
            } else {
                if (param_4 - 0x8 == 0x0) {
                    uStack20 = 0x7;
                    param_4._0_2_ = param_4 - 0x8;
                } else {
                    uStack20 = 0x8;
                    param_4._0_2_ = param_4 - 0x9;
                }
            }
            uVar14 = 0x6;
        }
        pass1_1010_81f6(0x1010, param_6, ctx.PTR_LOOP_1050_14cc, 0x0, uVar14);
        u_var5 = puVar8 | param_4;
        if ((u_var5 != 0x0)
            && (
                puVar10 = puVar8,
                mem_op_1000_179c(0x46, puVar8, 0x1000),
                (puVar10 | u_var5) != 0x0,
            ))
        {
            pass1_1008_87cc(
                CONCAT22(puVar10, u_var5),
                param_3,
                iVar13,
                uStack20,
                CONCAT13((puVar8 >> 0x8), CONCAT12(puVar8, param_4)),
                param_4,
                param_6,
            );
        }
    } else {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4);
        puVar12 = struct_op_1030_73a8(CONCAT22(param_5, u_var5));
        // uVar9 = (puVar12 >> 0x10);
        u_var4 = puVar12;
        if ((uVar9 | u_var4) != 0x0) {
            u_var2 = (u_var5 + 0x2e);
            uStack12 = u_var2;
            if (((u_var5 + 0x30) | uStack12) == 0x0) {
                lStack32 = 0x0;
            } else {
                lStack32 = (uStack12 + 0x200);
            }
            u_var5 = (u_var4 + 0x1c);
            u_var1 = (u_var4 + 0x1e);
            u_var6 = u_var1 | u_var5;
            if (u_var6 != 0x0) {
                lStack32 = CONCAT22(u_var1, u_var5);
                u_var6 = u_var5;
            }
            ppc_var3 = (*puVar12 + 0x14);
            (**ppc_var3)(0x1030, u_var4, uVar9);
            puVar8 = extraout_dx;
            uVar7 = u_var6;
            pass1_1010_81f6(0x1010, param_6, ctx.PTR_LOOP_1050_14cc, lStack32, u_var6);
            puVar10 = puVar8;
            uVar14 = uVar7;
            mem_op_1000_179c(0x46, puVar8, 0x1000);
            u_var5 = puVar10 | uVar14;
            if (u_var5 == 0x0) {
                uVar14 = 0x0;
                u_var5 = 0x0;
            } else {
                pass1_1008_87cc(
                    CONCAT22(puVar10, uVar14),
                    param_3,
                    iVar13,
                    u_var6,
                    CONCAT22(puVar8, uVar7),
                    param_4,
                    param_6,
                );
            }
            pass1_1008_8bc6(
                param_6,
                u_var5,
                CONCAT13((u_var5 >> 0x8), CONCAT12(u_var5, uVar14)),
            );
        }
    }
    return;
}

pub fn pass1_1018_df10(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1018_df92(param_1: i32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    destroy_win_1008_628e(param_1, 0x1008);
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 0xe2);
    u_var2 = (i_var4 + 0xe4);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(0x1008, pu_var1, u_var2, 0x1);
    }
    (i_var4 + 0xe2) = 0x0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_dfd4(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16, param_5: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = param_1;
    delete_palette_1018_e16c((u_var1 + 0xe2), param_4);
    if ((u_var1 + 0xe6) == 0x0) {
        (u_var1 + 0xe6) = 0x1;
        pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x36, param_5, param_2, param_3);
        pass1_1038_af40(
            ctx.PTR_LOOP_1050_5b7c,
            (u_var1 + 0x8),
            0x8,
            (pu_var3 >> 0x10),
            u_var1,
            &ctx.PTR_LOOP_1050_1038,
            param_5,
        );
    }
    return;
}

pub fn pass1_1018_e01c(param_1: &mut Struct18, param_2: u8) {
    let i_var1: &mut Struct572;
    let u_var1: u16;

    i_var1 = param_1;
    i_var1 = i_var1 + 0x1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(i_var1)));
    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x380a;
    i_var1.field_0x2 = 0x1008;
    param_1.field_0x0 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn struct_1018_e100(param_1: U32Ptr, param_2: u16, param_3: U32Ptr, param_4: u16) -> u16 {
    let i_var1: &mut Struct268;
    let unaff_DI: i16;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    *param_1 = 0x3aa8;
    i_var1.field_0x2 = 0x1008;
    i_var1.field_0x4 = param_2;
    *param_1 = 0x3ab0;
    i_var1.field_0x2 = 0x1008;
    &i_var1.field_0x6 = 0x0;
    *param_1 = 0xe228;
    i_var1.field_0x2 = 0x1018;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x36, param_4, param_3, unaff_DI);
    i_var1.field_0x6 = pu_var2;
    i_var1.field_0x8 = (pu_var2 >> 0x10);
    return param_1;
}

pub fn pass1_1018_e1ee(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x3ab0;
    (param_1 + 0x2) = 0x1008;
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_e230(param_1: u16, param_2: &mut Struct20, param_3: u16, param_4: u16) {
    let extraout_dx: U32Ptr;
    let u_var1: u16;
    let i_var2: &mut Struct128;
    let unaff_DI: i16;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    unk_draw_op_1020_7f7a(param_2, 0x4, CONCAT22(param_4, param_3));
    // u_var2 = (param_2 >> 0x10);
    i_var2 = param_2;
    i_var2.field_0xee = 0x0;
    &i_var2.field_0xf2 = 0x0;
    param_2.field_0x0 = 0xe44e;
    i_var2.field_0x2 = 0x1018;
    i_var2.field_0xe2 = 0xe4ea;
    i_var2.field_0xe4 = 0x1018;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x26, param_1, extraout_dx, unaff_DI);
    // u_var1 = (pu_var3 >> 0x10);
    i_var2.field_0xf2 = pu_var3;
    i_var2.field_0xf4 = u_var1;
    i_var2.field_0xe6 = i_var2.field_0xf2;
    i_var2.field_0xe8 = u_var1;
    return;
}

pub fn pass1_1018_e2a0(param_1: U32Ptr) {
    let i_var1: &mut Struct573;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0xe44e;
    i_var1.field_0x2 = 0x1018;
    i_var1.field_0xe2 = 0xe4ea;
    i_var1.field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_e2cc(param_1: u32, param_2: u16) {
    let pi_var1: U32Ptr;
    let ppcVar2: u32;
    let u_var3: u16;
    let u_var4: u32;
    let pu_var5: U32Ptr;
    let puVar6: U32Ptr;
    let iVar7: &mut Struct269;
    let uVar7: u16;
    let puVar8: U32Ptr;
    let puStack10: u32;
    let local_6: [u8; 4];

    // uVar7 = (param_1 >> 0x10);
    iVar7 = param_1;
    if (iVar7.field_0xee != 0x0) {
        ppcVar2 = (*iVar7.field_0xee + 0x8);
        (**ppcVar2)();
    }
    if (iVar7.field_0xea == 0x0) {
        iVar7.field_0xea = 0x1;
        puVar8 = pass1_1008_941a(CONCAT22(param_2, local_6), 0x1, 0x7a);
        // pu_var5 = (puVar8 >> 0x10);
        u_var4 = ZEXT24(local_6);
        win_1008_5c9e(
            ctx.PTR_LOOP_1050_02a0,
            CONCAT22(param_2, local_6),
            local_6,
            pu_var5,
            param_2,
        );
        iVar7.field_0xec = u_var4;
        mem_op_1000_179c(0x112, pu_var5, 0x1000);
        puVar6 = (pu_var5 | u_var4);
        if (puVar6 == 0x0) {
            u_var3 = 0x0;
            puStack10 = 0x0;
        } else {
            pi_var1 = &iVar7.field_0xcc;
            *pi_var1 = *pi_var1 + 0x1;
            struct_1020_3644(
                (u_var4 & 0xffff | ZEXT24(pu_var5) << 0x10),
                iVar7.field_0xcc,
                param_1,
                param_2,
            );
            u_var3 = u_var4;
            puStack10 = (u_var4 & 0xffff | ZEXT24(puVar6) << 0x10);
        }
        pass1_1008_6978(
            param_1,
            0x0,
            puStack10 & 0xffff0000 | u_var3,
            u_var3,
            puVar6,
        );
        ppcVar2 = (*puStack10 + 0xc);
        (**ppcVar2)();
    }
    return;
}

pub fn pass1_1018_e3e8(param_1: i32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    pu_var1 = (param_1 + 0xee);
    u_var2 = (param_1 + 0xf0);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

pub fn pass1_1018_e41a(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_e2a0(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_e4f2(
    param_1: &mut Struct659,
    param_2: u16,
    param_3: u16,
    param_4: i16,
    param_5: u16,
) {
    let ppcVar1: u32;
    let u_var2: u32;
    let i_var3: i16;
    let extraout_dx: U32Ptr;
    let u_var4: u16;
    let pu_var5: U32Ptr;

    set_struct_op_1020_921c(CONCAT22(param_2, param_1), param_3);
    &param_1.field_0x14 = 0x0;
    CONCAT22(param_2, param_1) = 0xe5d0;
    param_1.field_0x2 = 0x1018;
    pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x26, param_5, extraout_dx, param_4);
    // u_var4 = (pu_var5 >> 0x10);
    param_1.field_0x14 = pu_var5;
    param_1.field_0x16 = u_var4;
    param_1.field_0x6 = param_1.field_0x14;
    param_1.field_0x8 = u_var4;
    u_var2 = &param_1.field_0x14;
    i_var3 = &param_1.field_0xa;
    ppcVar1 = ((u_var2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_1.field_0x12 = i_var3;
    draw_op_1020_9364(CONCAT22(param_2, param_1), 0x1020, param_5);
    return;
}

pub fn pass1_1018_e57a(param_1: U32Ptr) {
    let i_var1: &mut Struct575;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0xe5d0;
    i_var1.field_0x2 = 0x1018;
    if (i_var1.field_0x14 != 0x0) {
        pass1_1010_1dda(i_var1.field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

pub fn pass1_1018_e5aa(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1018_e57a(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_e5dc(param_1: u16, param_2: &mut Struct20, param_3: u16, param_4: u16) {
    let extraout_dx: U32Ptr;
    let u_var1: u16;
    let i_var2: &mut Struct20;
    let unaff_DI: i16;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    unk_draw_op_1020_7f7a(param_2, 0x9, CONCAT22(param_4, param_3));
    // u_var2 = (param_2 >> 0x10);
    i_var2 = param_2;
    &i_var2[0x1].field_0xc = 0x0;
    i_var2[0x1].field_0x10 = 0x0;
    param_2.field_0x0 = 0xe790;
    i_var2.field_0x2 = 0x1018;
    (i_var2 + 0x1).field_0x0 = 0xe82c;
    i_var2[0x1].field_0x2 = 0x1018;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0xa, param_1, extraout_dx, unaff_DI);
    // u_var1 = (pu_var3 >> 0x10);
    &i_var2[0x1].field_0x10 = pu_var3;
    (&i_var2[0x1].field_0x10 + 0x2) = u_var1;
    &i_var2[0x1].field_0x4 = &i_var2[0x1].field_0x10;
    (&i_var2[0x1].field_0x4 + 0x2) = u_var1;
    return;
}

pub fn pass1_1018_e64c(param_1: U32Ptr) {
    let i_var1: &mut Struct576;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0xe790;
    i_var1.field_0x2 = 0x1018;
    i_var1.field_0xe2 = 0xe82c;
    i_var1.field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_e678(param_1: u32, param_2: u16, param_3: u16) -> u16 {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;

    // u_var4 = (param_1 >> 0x10);
    u_var3 = param_1;
    u_var2 = (u_var3 + 0xf0) | (u_var3 + 0xee);
    if (u_var2 != 0x0) {
        ppcVar1 = ((u_var3 + 0xee) + 0x8);
        u_var5 = (**ppcVar1)();
        // param_2 = (u_var5 >> 0x10);
        u_var2 = u_var5;
    }
    if ((u_var3 + 0xea) == 0x0) {
        (u_var3 + 0xea) = 0x1;
        u_var5 = pass1_1038_af40(
            ctx.PTR_LOOP_1050_5b7c,
            (u_var3 + 0x8),
            0x15,
            param_2,
            u_var3,
            &ctx.PTR_LOOP_1050_1038,
            param_3,
        );
        u_var2 = u_var5;
    }
    return u_var2;
}

pub fn pass1_1018_e72a(param_1: i32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    pu_var1 = (param_1 + 0xee);
    u_var2 = (param_1 + 0xf0);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

pub fn pass1_1018_e75c(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_e64c(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_e834(
    param_1: &mut Struct660,
    param_2: u16,
    param_3: u16,
    param_4: i16,
    param_5: u16,
) {
    let ppcVar1: u32;
    let u_var2: u32;
    let i_var3: i16;
    let extraout_dx: U32Ptr;
    let u_var4: u16;
    let pu_var5: U32Ptr;

    set_struct_op_1020_921c(CONCAT22(param_2, param_1), param_3);
    &param_1.field_0x14 = 0x0;
    CONCAT22(param_2, param_1) = 0xe912;
    param_1.field_0x2 = 0x1018;
    pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0xa, param_5, extraout_dx, param_4);
    // u_var4 = (pu_var5 >> 0x10);
    param_1.field_0x14 = pu_var5;
    param_1.field_0x16 = u_var4;
    param_1.field_0x6 = param_1.field_0x14;
    param_1.field_0x8 = u_var4;
    u_var2 = &param_1.field_0x14;
    i_var3 = &param_1.field_0xa;
    ppcVar1 = ((u_var2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_1.field_0x12 = i_var3;
    draw_op_1020_9364(CONCAT22(param_2, param_1), 0x1020, param_5);
    return;
}

pub fn pass1_1018_e8bc(param_1: U32Ptr) {
    let i_var1: &mut Struct577;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0xe912;
    i_var1.field_0x2 = 0x1018;
    if (i_var1.field_0x14 != 0x0) {
        pass1_1010_1dda(i_var1.field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

pub fn pass1_1018_e8ec(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1018_e8bc(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_e91e(param_1: &mut Struct20, param_2: u16, param_3: u16, param_4: u16) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let pu_var3: U32Ptr;
    let extraout_dx: U32Ptr;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let unaff_DI: i16;
    let puVar6: U32Ptr;
    let uVar7: u16;
    let iVar7: &mut Struct127;

    iVar7 = param_1;
    // uVar7 = (param_1 >> 0x10);
    unk_draw_op_1020_7f7a(param_1, 0x3, CONCAT22(param_3, param_2));
    iVar7.field_0xee = 0x0;
    &iVar7.field_0xf2 = 0x0;
    iVar7.field_0xf6 = 0x0;
    param_1.field_0x0 = 0xebd0;
    iVar7.field_0x2 = 0x1018;
    iVar7.field_0xe2 = 0xec6c;
    iVar7.field_0xe4 = 0x1018;
    puVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x28, param_4, extraout_dx, unaff_DI);
    // puVar4 = (puVar6 >> 0x10);
    iVar7.field_0xf2 = puVar6;
    iVar7.field_0xf4 = puVar4;
    iVar7.field_0xe6 = iVar7.field_0xf2;
    iVar7.field_0xe8 = puVar4;
    puVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x32, param_4, puVar4, unaff_DI);
    &iVar7.field_0xf6 = puVar6;
    (&iVar7.field_0xf6 + 0x2) = (puVar6 >> 0x10);
    if (param_1 == 0x0) {
        pu_var3 = 0x0;
        u_var5 = 0x0;
    } else {
        pu_var3 = &iVar7.field_0xe2;
        u_var5 = uVar7;
    }
    pu_var1 = iVar7.field_0xf6;
    ppcVar2 = (*iVar7.field_0xf6 + 0x4);
    (**ppcVar2)(0x1010, pu_var1, (pu_var1 >> 0x10), 0xb, pu_var3, u_var5);
    return;
}

pub fn pass1_1018_e9de(param_1: U32Ptr) {
    let i_var1: &mut Struct578;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0xebd0;
    i_var1.field_0x2 = 0x1018;
    i_var1.field_0xe2 = 0xec6c;
    i_var1.field_0xe4 = 0x1018;
    pass1_1020_808e(param_1);
    return;
}

pub fn pass1_1018_ea66(param_1: u32, param_2: u16) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let i_var3: i16;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let local_6: [u8; 4];

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0xee) != 0x0) {
        ppcVar1 = ((i_var3 + 0xee) + 0x8);
        (**ppcVar1)();
    }
    if ((i_var3 + 0xea) == 0x0) {
        (i_var3 + 0xea) = 0x1;
        pu_var5 = pass1_1008_941a(CONCAT22(param_2, local_6), 0x1, 0x95);
        pu_var2 = local_6;
        win_1008_5c9e(
            ctx.PTR_LOOP_1050_02a0,
            CONCAT22(param_2, pu_var2),
            pu_var2,
            (pu_var5 >> 0x10),
            param_2,
        );
        (i_var3 + 0xec) = pu_var2;
        unk_win_op_1010_7300((i_var3 + 0xf6), 0x0, 0x8, 0x0);
    }
    return;
}

pub fn pass1_1018_eb3e(param_1: i32, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pu_var1 = (iVar6 + 0xee);
    u_var2 = (iVar6 + 0xf0);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    if ((iVar6 + 0xf6) != 0x0) {
        if (param_1 == 0x0) {
            i_var4 = 0x0;
            u_var5 = 0x0;
        } else {
            i_var4 = iVar6 + 0xe2;
            u_var5 = uVar7;
        }
        pass1_1010_1ea6((iVar6 + 0xf6), CONCAT22(u_var5, i_var4), param_2);
    }
    destroy_win_1008_628e(param_1, 0x1008);
    return;
}

pub fn pass1_1018_eb9c(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_e9de(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1018_ec74(param_1: &mut Struct661, param_2: i16, param_3: u16, param_4: u16) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let extraout_dx: U32Ptr;
    let pu_var5: U32Ptr;
    let puVar6: U32Ptr;
    let uVar7: u32;
    let uVar8: u16;
    let uVar9: u16;
    let paVar10: &mut Struct661;
    let iVar11: i16;

    set_struct_op_1020_921c(CONCAT22(param_2, param_1), param_3);
    param_1.field_0x14 = 0x0;
    clear_struct_1008_3e38(CONCAT22(param_2, &param_1.field_0x18));
    puVar6 = clear_struct_1008_3e38(CONCAT22(param_2, &param_1.field_0x1e));
    &param_1.field_0x24 = 0x0;
    CONCAT22(param_2, param_1) = 0x1cc;
    param_1.field_0x2 = 0x1020;
    puVar6 = mixed_1010_20ba(
        ctx.PTR_LOOP_1050_0ed0,
        0x28,
        param_4,
        (puVar6 >> 0x10),
        param_2,
    );
    // u_var4 = (puVar6 >> 0x10);
    &param_1.field_0x14 = puVar6;
    (&param_1.field_0x14 + 0x2) = u_var4;
    uVar9 = 0x0;
    uVar8 = &param_1.field_0x14;
    ppcVar2 = (*param_1.field_0x14 + 0x4);
    paVar10 = param_1;
    iVar11 = param_2;
    (**ppcVar2)();
    param_1.field_0x6 = param_1.field_0x14;
    pu_var1 = param_1.field_0x14;
    pass1_1010_2b50(
        pu_var1,
        (pu_var1 >> 0x10),
        CONCAT22(param_2, &param_1.field_0x18),
    );
    uVar7 = pass1_1010_2b66(param_1.field_0x14);
    param_1.field_0x24 = uVar7;
    param_1.field_0x26 = (uVar7 >> 0x10);
    pu_var1 = param_1.field_0x14;
    pu_var1 = (pu_var1 + 0xa);
    u_var3 = CONCAT22(param_2, &param_1.field_0xa);
    ppcVar2 = (*pu_var1 + 0x8);
    (**ppcVar2)(
        0x1010,
        pu_var1,
        (pu_var1 >> 0x10),
        u_var3,
        uVar8,
        u_var4,
        uVar9,
        paVar10,
        iVar11,
    );
    param_1.field_0x12 = u_var3;
    pu_var5 = extraout_dx;
    draw_op_1020_9364(CONCAT22(param_2, param_1), 0x1020, param_4);
    puVar6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x48, param_4, pu_var5, param_2);
    pass1_1008_3f62(
        CONCAT22(param_2, &param_1.field_0x1e),
        (puVar6 & 0xffff0000 | (puVar6 + 0xe)),
    );
    pass1_1008_3f32(
        CONCAT22(param_2, &param_1.field_0x18),
        CONCAT22(param_2, &param_1.field_0x1e),
    );
    return;
}

pub fn pass1_1018_ed98(param_1: U32Ptr, param_2: u16) {
    let i_var1: &mut Struct579;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x1cc;
    i_var1.field_0x2 = 0x1020;
    if (i_var1.field_0x14 != 0x0) {
        pass1_1010_1ea6(
            i_var1.field_0x14,
            param_1 & 0xffff | u_var1 << 0x10,
            param_2,
        );
        pass1_1010_1dda(i_var1.field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

pub fn pass1_1020_01a6(param_1: &mut Struct18, param_2: u8, param_3: u16) -> Struct18 {
    pass1_1018_ed98(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1020_01d8(
    param_1: &mut Struct20,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: u32,
    param_9: u16,
) {
    unk_draw_op_1008_61b2(
        CONCAT22(param_2, param_1),
        param_3,
        param_7,
        param_8,
        param_9,
    );
    (param_1 + 0x1) = 0x0;
    param_1[0x1].field_0x4 = 0x0;
    param_1[0x1].field_0x8 = param_6;
    param_1[0x1].field_0xa = param_5;
    param_1[0x1].field_0xc = param_4;
    CONCAT22(param_2, param_1) = 0x45a;
    param_1.field_0x2 = 0x1020;
    return;
}

pub fn pass1_1020_022c(param_1: &mut Struct29) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct580;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x45a;
    i_var4.field_0x2 = 0x1020;
    pu_var1 = i_var4.field_0xe6;
    u_var2 = i_var4.field_0xe8;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pass1_1008_57c4((param_1 & 0xffff0000 | &i_var4.field_0xd2));
    *param_1 = 0x380a;
    i_var4.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    i_var4.field_0x2 = 0x1008;
    return;
}

pub fn pass1_1020_028c(param_1: u16, param_2: u16, param_3: i16) {
    pass1_1010_3c9e((param_1 + 0xe2));
    pass1_1008_68c6(param_1, param_2, param_3, 0x1008);
    return;
}

pub fn pass1_1020_02ae(param_1: i32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pass1_1010_3cd0((i_var4 + 0xe2));
    destroy_win_1008_628e(param_1, 0x1008);
    pu_var1 = (i_var4 + 0xe6);
    u_var2 = (i_var4 + 0xe8);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(0x1008, pu_var1, u_var2, 0x1);
    }
    (i_var4 + 0xe6) = 0x0;
    pass1_1010_1dda((i_var4 + 0xe2));
    (i_var4 + 0xe2) = 0x0;
    return;
}

pub fn pass1_1020_0434(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1020_022c(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_04f6(param_1: U32Ptr, param_2: u16, param_3: U32Ptr, param_4: i16, param_5: u16) {
    let ppcVar1: u32;
    let i_var2: i16;
    let u_var3: u16;
    let extraout_dx: U32Ptr;
    let i_var4: &mut Struct662;
    let u_var4: u16;
    let pu_var5: U32Ptr;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x389a;
    i_var4.field_0x2 = 0x1008;
    *param_1 = 0x3aa8;
    i_var4.field_0x2 = 0x1008;
    i_var4.field_0x4 = param_2;
    *param_1 = 0x3ab0;
    i_var4.field_0x2 = 0x1008;
    i_var4.field_0x6 = 0x0;
    i_var4.field_0xa = 0x0;
    i_var4.field_0xc = 0x0;
    i_var4.field_0xe = 0x0;
    i_var4.field_0x10 = 0x0;
    *param_1 = 0x75a;
    i_var4.field_0x2 = 0x1020;
    pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x1, param_5, param_3, param_4);
    // u_var3 = (pu_var5 >> 0x10);
    &i_var4.field_0x6 = pu_var5;
    (&i_var4.field_0x6 + 0x2) = u_var3;
    ppcVar1 = (*i_var4.field_0x6 + 0x4);
    (**ppcVar1)(0x1010, &i_var4.field_0x6, u_var3, 0x0, param_1);
    pu_var5 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x48, param_5, extraout_dx, param_4);
    i_var2 = pu_var5;
    i_var4.field_0xa = (i_var2 + 0xa);
    i_var4.field_0xc = (i_var2 + 0xc);
    pass1_1008_3e94(
        (pu_var5 & 0xffff0000 | (i_var2 + 0xe)),
        (param_1 & 0xffff0000 | ZEXT24(&i_var4.field_0x10)),
        (param_1 & 0xffff0000 | ZEXT24(&i_var4.field_0xe)),
    );
    return;
}

pub fn pass1_1020_05d6(param_1: U32Ptr, param_2: u16) {
    let i_var1: &mut Struct581;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x75a;
    i_var1.field_0x2 = 0x1020;
    if (i_var1.field_0x6 != 0x0) {
        pass1_1010_1ea6(i_var1.field_0x6, param_1 & 0xffff | u_var1 << 0x10, param_2);
    }
    *param_1 = 0x3ab0;
    i_var1.field_0x2 = 0x1008;
    *param_1 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    return;
}

pub fn pass1_1020_0734(param_1: &mut Struct18, param_2: u8, param_3: u16) -> Struct18 {
    pass1_1020_05d6(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1020_07aa(param_1: u32, param_2: u16, param_3: i16, param_4: u16, param_5: u16) {
    let i_var1: i16;
    let u_var2: u16;
    let local_16: [u8; 14];

    draw_op_1020_041e(param_1, param_4);
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0xf0) == 0x0) {
        (i_var1 + 0xf0) = 0x1;
        pass1_1008_5bdc(CONCAT22(param_5, local_16), param_3, param_5);
        win_1008_5c9e(
            CONCAT22(param_5, local_16),
            (param_1 & 0xffff0000 | (i_var1 + 0xf2)),
            local_16,
            param_2,
            param_5,
        );
        pass1_1008_5c34(CONCAT22(param_5, local_16));
    }
    return;
}

pub fn pass1_1020_07f4(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1020_022c(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_08b6(param_1: &WNDCLASS16, param_2: &mut Struct20, param_3: u16, param_4: i32) {
    let i_var1: &mut Struct20;
    let u_var1: u16;
    let paVar2: &mut Struct20;

    paVar2 = unk_draw_op_1008_61b2(param_2, 0x1, param_3, param_4, param_1);
    // u_var1 = (param_2 >> 0x10);
    i_var1 = param_2;
    &i_var1[0x1].field_0x4 = 0x0;
    (&i_var1[0x1].field_0x4 + 0x2) = 0x0;
    param_2.field_0x0 = 0xb0e;
    i_var1.field_0x2 = 0x1020;
    win_1008_5c5c(
        param_1,
        0x0,
        (paVar2 >> 0x10),
        ctx.PTR_LOOP_1050_02a0,
        0x1d4,
    );
    return;
}

pub fn pass1_1020_0a0c(param_1: i32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    destroy_win_1008_628e(param_1, 0x1008);
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 0xe2);
    u_var2 = (i_var4 + 0xe4);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(0x1008, pu_var1, u_var2, 0x1);
    }
    (i_var4 + 0xe2) = 0x0;
    (i_var4 + 0xe6) = 0x0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1020_0a52(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u32;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = param_1;
    unk_draw_op_1020_0c3e((u_var1 + 0xe2), param_3);
    if ((u_var1 + 0xe6) == 0x0) {
        (u_var1 + 0xe6) = 0x1;
        (ctx.PTR_LOOP_1050_5b7c + 0xae) = 0x99;
        u_var3 = pass1_1038_af40(
            ctx.PTR_LOOP_1050_5b7c,
            (u_var1 + 0x8),
            0x6,
            param_2,
            u_var1,
            &ctx.PTR_LOOP_1050_1038,
            param_4,
        );
        (u_var1 + 0xe8) = u_var3;
        (u_var1 + 0xea) = (u_var3 >> 0x10);
    }
    return;
}

pub fn pass1_1020_0aa6(param_1: u32, param_2: u16) {
    win_ui_palette_op_1020_0cd2((param_1 + 0xe2), param_2, 0);
    return;
}

pub fn pass1_1020_0abc(param_1: u32) {
    let ppcVar1: u32;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0xe6) != 0x0) {
        ppcVar1 = ((param_1 + 0xe8) + 0x10);
        (**ppcVar1)();
    }
    return;
}

pub fn pass1_1020_0ae8(param_1: &mut Struct63, param_2: u8, param_3: u16) -> Struct63 {
    send_win_msg_1020_08fe(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}
