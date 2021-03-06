use crate::bad::bad_1030_1312;
use crate::cleanup::{destroy_win_1040_7b98, ui_cleanup_op_1040_782c};
use crate::defines::{Struct18, U32Ptr};
use crate::draw::draw_1040::unk_draw_op_1040_b0f8;
use crate::file::file_1008::{
    file_1008_7548, read_file_1008_7c6e, read_file_1008_7dee, write_to_file_1008_7a22,
    write_to_file_1008_7cac,
};
use crate::file::file_1030::{file_1030_1730, read_file_1030_33f0};
use crate::file::write::write_to_file_1030_32e4;
use crate::fn_ptr::fn_ptr_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708};
use crate::fn_ptr::fn_ptr_1020::fn_ptr_1020_ba7e;
use crate::fn_ptr::fn_ptr_1028::fn_ptr_1030_835a;
use crate::mem_1000::{mem_op_1000_160a, mem_op_1000_179c};
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::{pass1_1000_07fc, pass1_1000_4906, pass1_1000_5586};
use crate::pass::pass_1008::{
    pass1_1008_3f62, pass1_1008_4d84, pass1_1008_5784, pass1_1008_5b12, pass1_1008_79f0,
    pass1_1008_7c2a, pass1_1008_b340, pass1_1008_b63a,
};
use crate::pass::pass_1010::{
    pass1_1010_038e, pass1_1010_041a, pass1_1010_0886, pass1_1010_08e2, pass1_1010_1ea6,
    pass1_1010_3770, pass1_1010_8fba,
};
use crate::pass::pass_1020::{pass1_1020_ba94, pass1_1020_bb16, pass1_1020_c3ae, pass1_1020_c42e};
use crate::pass::pass_1028::{
    pass1_1028_45e2, pass1_1028_6228, pass1_1028_62c8, pass1_1028_6356, pass1_1028_6408,
    pass1_1028_6744, pass1_1028_678c, pass1_1028_b58e, pass1_1028_bdac, pass1_1028_e1ec,
};
use crate::pass::pass_1030::{
    pass1_1030_1358, pass1_1030_16d6, pass1_1030_18b2, pass1_1030_1d58, pass1_1030_1d7c,
    pass1_1030_314c, pass1_1030_3258, pass1_1030_326a, pass1_1030_375a, pass1_1030_38b8,
    pass1_1030_38f2, pass1_1030_5b1c, pass1_1030_64ce, pass1_1030_6a2c, pass1_1030_6b86,
    pass1_1030_6c4c, pass1_1030_6d4e, pass1_1030_6e9c, pass1_1030_6fa0, pass1_1030_72d0,
    pass1_1030_73ee, pass1_1030_7c50, pass1_1030_8e3c, pass1_1030_9ecc, pass1_1030_9ef2,
    pass1_1030_9f40, pass1_1030_aefa, pass1_1030_affc, pass1_1030_b768, pass1_1030_bcae,
    pass1_1030_bd74, pass1_1030_cc44, pass1_1030_ce72, pass1_1030_d0a8, pass1_1030_d180,
};
use crate::pass::pass_1040::pass1_1040_b040;
use crate::string::string_1008::str_op_1008_60e8;
use crate::string::string_1010::load_string_1010_847e;
use crate::string::string_1030::vsprintf_op_1030_840a;
use crate::struct_ops::struct_1008::{
    clear_struct_1008_3e38, pass1_1008_c6ae, pass1_1008_c6fa, set_struct_1008_574a,
    struct_1008_4c58,
};
use crate::struct_ops::struct_1020::struct_1020_c444;
use crate::struct_ops::struct_1030::{
    struct_1030_11aa, struct_1030_17ce, struct_op_1030_1cd8, struct_op_1030_73a8,
};
use crate::struct_ops::struct_1040::{struct_1040_a598, struct_1040_b082};
use crate::switch_ops::switch_1008::switch_1008_72bc;
use crate::switch_ops::switch_1020::switch_1020_c3b4;

use crate::ui::ui_1040::enable_win_1040_9234;
use crate::util::{
    read_string_from_addr, CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, SUB42, ZEXT24,
};
use crate::win_struct::{HWND16, SEGPTR};
use crate::winapi::SetDlgItemInt16;

pub fn pass1_1038_0000(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;
    mem_op_1000_179c(ctx, 0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10 = 0xb96;
        (param_2 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_008e(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: U32Ptr,
    param_5: i16,
    param_6: u16,
) {
    let i_var1: i16;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let puVar7: U32Ptr;
    let puVar8: U32Ptr;
    let i_var9: i16;
    let u_var10: u16;
    let puVar11: U32Ptr;
    let puVar12: U32Ptr;
    let iStack32: i16;
    let iStack12: i16;
    let u_var6: u32;

    // u_var10 = (param_3 >> 0x10);
    i_var9 = param_3;
    if ((i_var9 + 0x4) != 0x4000001) {
        return;
    }
    puVar11 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2c, param_6, param_4, param_5);
    // puVar7 = (puVar11 >> 0x10);
    u_var3 = puVar11;
    puVar8 = puVar7;
    u_var4 = u_var3;
    pass1_1008_612e(0x1, 0x64, u_var3);
    iStack12 = 0x0;
    i_var1 = (u_var3 + 0xa);
    if (i_var1 == 0x1) {
        iStack12 = 0x15;
    } else {
        if (i_var1 != 0x2) {
            if (i_var1 == 0x3) {
                iStack12 = 0x16;
            } else {
                if (i_var1 == 0x4) {
                    if (u_var4 < 0x32) {
                        iStack12 = 0x17;
                    } else {
                        iStack12 = 0x18;
                    }
                } else {
                    if (i_var1 == 0x5) {
                        iStack12 = 0x19;
                    }
                }
            }
        }
    }
    if (iStack12 != 0x0) {
        puVar12 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_6, puVar8, param_5);
        // puVar8 = (puVar12 >> 0x10);
        pass1_1010_043a(
            puVar12 & 0xffff | ZEXT24(puVar8) << 0x10,
            (i_var9 + 0x4),
            iStack12,
            param_6,
        );
    }
    pass1_1008_eb74(puVar11, 0x0, puVar8, param_5, param_6);
    if ((((u_var3 + 0xe) | (u_var3 + 0xc)) == 0x0) && ((i_var9 + 0x18) < 0xc9)) {
        u_var2 = *ctx.PTR_LOOP_1050_65e2;
        u_var4 = u_var2;
        u_var6 = u_var2;
        pass1_1008_612e(0x0, 0x8, u_var4);
        u_var5 = u_var6;
        // iStack32 = (u_var2 >> 0x10);
        (u_var3 + 0xc) = u_var5 + u_var4 + 0x1e;
        (u_var3 + 0xe) =
            (u_var5 >> 0xf) + iStack32 + CARRY2(u_var5, u_var4) + (0xffe1 < u_var5 + u_var4);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_01c0(param_1: u16, param_2: u16, param_3: u32, param_4: u16) {
    let i_var1: i16;
    let pu_var2: u32;
    let ppc_var3: u32;
    let u_var4: u32;
    let u_var5: u32;
    let BVar6: bool;
    let puVar7: U32Ptr;
    let puVar8: u32;
    let uVar9: u32;
    let puVar10: U32Ptr;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var11: u16;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u16;
    let puVar15: u32;
    let uVar16: u32;
    let uVar17: u32;
    let uVar18: u8;
    let uStack50: u32;
    let uStack30: u32;
    let uStack18: u32;
    let local_e: [u8; 2];
    let puStack12: u32;
    let uStack8: u16;
    let puStack6: U32Ptr;
    let i_stack4: i16;

    i_stack4 = 0x0;
    puVar15 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x29);
    // puVar10 = (puVar15 >> 0x10);
    uVar12 = puVar15;
    uStack8 = uVar12;
    puStack6 = puVar10;
    pass1_1038_4e78(uVar12, puVar10, param_3, puVar15);
    puStack12 = CONCAT22(puVar10, uVar12);
    uVar14 = 0x1030;
    uVar16 = pass1_1030_bcae(local_e, param_4);
    uVar13 = uVar16;
    ppc_var3 = (*puStack12 + 0x10);
    (**ppc_var3)(0x1030, puStack12, (puStack12 >> 0x10));
    uStack18 = CONCAT22(extraout_dx, uVar13);
    // uVar13 = (param_3 >> 0x10);
    pu_var2 = (param_3 + 0xc);
    uVar13 = (param_3 + 0xe);
    uVar18 = SUB41(pu_var2, 0x0);
    ppc_var3 = (*pu_var2 + 0x10);
    puVar8 = pu_var2;
    (**ppc_var3)();
    uVar16 = puVar8 & 0xffff | extraout_DX_00 << 0x10;
    uStack30 = 0x0;
    uVar12 = extraout_DX_00;
    loop {
        if (uStack18 <= uStack30) {
            if (puStack12 != 0x0) {
                ppc_var3 = *puStack12;
                (**ppc_var3)(uVar14, puStack12, (puStack12 >> 0x10), 0x1, uVar18, uVar13);
            }
            return;
        }
        uVar14 = 0x1030;
        uVar9 = uStack18;
        pass1_1030_1d58(puStack12);
        u_var5 = uVar12;
        i_var1 = (uVar9 + 0x10);
        u_var11 = uVar12;
        // for (uStack50 = 0x0; uVar12 = u_var11, uStack50 < uVar16; uStack50 += 0x1) {
        //   uVar14 = 0x1030;
        //   uVar17 = uVar16;
        //   pass1_1030_1d58(pu_var2);
        //   u_var4 = uVar17 & 0xffff | u_var11 << 0x10;
        //   uVar12 = u_var11 | uVar17;
        //   if ((uVar12 != 0x0) && (uVar12 = u_var11, (uVar17 + 0x10) == i_var1)) {
        //     uVar17 = struct_op_1030_73a8(u_var4);
        //     uVar12 = (uVar17 >> 0x10);
        //     uVar14 = 0x1008;
        //     BVar6 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0,
        //                             (uVar17 + 0xc),0x30);
        //     if (BVar6 == 0x0) {
        //       puVar7 = local_e;
        //       uVar14 = 0x1030;
        //       pass1_1030_bd74(puVar7,param_4,u_var4,uVar9 & 0xffff | u_var5 << 0x10,
        //                       param_4);
        //       if (puVar7 < 0x6) {
        //         i_stack4 += 0x1;
        //         break;
        //       }
        //     }
        //   }
        //   u_var11 = uVar12;
        // }
        uStack30 += 0x1;
    }
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_0340(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u16,
    param_6: u16,
    param_7: u8,
) {
    let u_var1: u16;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let local_13a: [u8; 0x11c];
    let uStack30: u32;
    let uStack26: u32;
    let uStack22: u32;
    let local_12: u16;
    let uStack16: u16;
    let local_e: i16;
    let uStack12: u16;
    let uStack10: u32;
    let uStack6: u32;

    uStack6 = *ctx.PTR_LOOP_1050_65e2;
    uStack10 = 0x0;
    uStack12 = 0x0;
    i_var3 = param_4;
    // u_var4 = (param_4 >> 0x10);
    pass1_1038_4cea(
        param_4,
        CONCAT22(param_6, &local_12),
        CONCAT22(param_6, &local_e),
    );
    u_var2 = (i_var3 + 0x1f6);
    uStack22 = u_var2;
    pass1_1030_38b8();
    u_var1 = u_var2;
    uStack26 = u_var2 & 0xffff | param_5 << 0x10;
    if (param_3 == 0x0) {
        if (local_e != 0x8) {
            uStack10 = (u_var2 & 0xffff | param_5 << 0x10) / 0x4;
            uStack12 = 0x8;
            //       TODO: goto LAB_1038_054b;
        }
    } else {
        if (param_3 < 0xb) {
            if (local_e != 0x7) {
                uStack10 = (u_var2 & 0xffff | param_5 << 0x10) / 0xa;
                uStack12 = 0x7;
                //         TODO: goto LAB_1038_054b;
            }
        } else {
            if (param_3 < 0x1a) {
                if (local_e != 0x6) {
                    uStack10 = (u_var2 & 0xffff | param_5 << 0x10) / 0x14;
                    uStack12 = 0x6;
                    //           TODO: goto LAB_1038_054b;
                }
            } else {
                if (param_3 < 0x33) {
                    if (local_e != 0x5) {
                        uStack10 = (u_var2 & 0xffff | param_5 << 0x10) / 0x64;
                        uStack12 = 0x5;
                        //             TODO: goto LAB_1038_054b;
                    }
                } else {
                    if (param_3 < 0x4c) {
                        if (uStack6 % 0x3 != 0x0) {
                            // goto
                            // LAB_1038_054b;
                        }
                        if (local_e != 0x4) {
                            uStack10 = uStack26 / 0x64;
                            uStack12 = 0x4;
                            //               TODO: goto LAB_1038_054b;
                        }
                    } else {
                        if (param_3 < 0x65) {
                            if (uStack6 % 0x5 != 0x0) {
                                // goto
                                // LAB_1038_054b;
                            }
                            if (local_e != 0x3) {
                                uStack10 = uStack26 / 0x64;
                                uStack12 = 0x3;
                                //                 TODO: goto LAB_1038_054b;
                            }
                        } else {
                            if (param_3 < 0x97) {
                                if (uStack6 % 0xa != 0x0) {
                                    // goto
                                    // LAB_1038_054b;
                                }
                                if (local_e != 0x2) {
                                    uStack10 = uStack26 / 0x64;
                                    uStack12 = 0x2;
                                    //                   TODO: goto LAB_1038_054b;
                                }
                            } else {
                                if ((0xc8 < param_3) || (uStack6 % 0x14 != 0x0)) {
                                    // goto
                                    // LAB_1038_054b;
                                }
                                if (local_e != 0x1) {
                                    uStack10 = uStack26 / 0x64;
                                    uStack12 = 0x1;
                                    //                   TODO: goto LAB_1038_054b;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if ((uStack16 <= param_5) && (uStack16 < param_5 || (local_12 < u_var1))) {
        u_var1 = local_12;
        param_5 = uStack16;
    }
    uStack10 = CONCAT22(param_5, u_var1);
    //LAB_1038_054b:
    if (uStack12 != 0x0) {
        if ((uStack26 != 0x0) && (uStack10 == 0x0)) {
            uStack10 = 0x1;
        }
        pass1_1038_4cd0(param_4, uStack10, uStack12);
    }
    if ((uStack10._2_2_ | uStack10) != 0x0) {
        if ((i_var3 + 0x200) == 0x8000001) {
            uStack30._0_2_ = 0x2;
        } else {
            uStack30._0_2_ = 0x1;
        }
        uStack30 = CONCAT22(0x400, uStack30);
        pass1_1028_9944(
            CONCAT22(param_6, local_13a),
            uStack10,
            CONCAT22(0x400, uStack30),
            (i_var3 + 0x4),
            param_6,
            param_7,
        );
        fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_6, local_13a));
        pass1_1028_9992(CONCAT22(param_6, local_13a));
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_05d8(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: u32,
    param_6: u16,
    param_7: u8,
) {
    let pu_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let local_158: [u8; 118];
    let uStack64: u32;
    let local_34: u16;
    let uStack50: u16;
    let uStack34: u32;
    let uStack30: u32;
    let uStack26: u32;
    let uStack22: u32;
    let local_12: u32;
    let local_e: i16;
    let uStack12: u16;
    let uStack10: u32;
    let uStack6: u32;

    uStack6 = *ctx.PTR_LOOP_1050_65e2;
    uStack10 = 0x0;
    uStack12 = 0x0;
    pass1_1038_4cea(
        param_4,
        CONCAT22(param_6, &local_12),
        CONCAT22(param_6, &local_e),
    );
    uStack22 = 0x0;
    uStack26 = 0x0;
    uStack30 = 0x0;
    pass1_1028_dc52(
        CONCAT13((param_6 >> 0x8), CONCAT12(param_6, &local_34)),
        0x1,
        0x0,
        0x400,
    );
    loop {
        loop {
            u_var3 = param_5;
            pu_var1 = &local_34;
            pass1_1028_e4ec(CONCAT22(param_6, pu_var1));
            uStack34 = CONCAT22(u_var3, pu_var1);
            u_var4 = u_var3 | pu_var1;
            param_5 = u_var4;
            if (u_var4 == 0x0) {
                // goto
                // LAB_1038_0668;
            }
            if ((pu_var1 + 0x100) != 0x8000002) == false {
                break;
            }
        }
        uStack22 = CONCAT22(u_var3, pu_var1);
        u_var2 = (pu_var1 + 0xfb);
        uStack26 = u_var2;
        pass1_1030_38b8();
        uStack30 = u_var2 & 0xffff | u_var4 << 0x10;
        u_var4 |= u_var2;
        param_5 = u_var4;
        if (u_var4 == 0x0) == false {
            break;
        }
    }
    //LAB_1038_0668:
    local_34 = 0x389a;
    uStack50 = 0x1008;
    if ((uStack22._2_2_ | uStack22) == 0x0) {
        return;
    }
    if (param_3 == 0x3e8) {
        if (local_e != 0x10) {
            uStack10 = uStack30 / 0x4;
            uStack12 = 0x10;
            //       TODO: goto LAB_1038_0841;
        }
    } else {
        if (param_3 < 0x3de) {
            if (param_3 < 0x3cf) {
                if (param_3 < 0x3b6) {
                    if (param_3 < 0x39d) {
                        if (param_3 < 0x384) {
                            if (param_3 < 0x352) {
                                if ((param_3 < 0x320) || (uStack6 % 0x14 != 0x0)) {
                                    // goto
                                    // LAB_1038_0841;
                                }
                                if (local_e != 0x9) {
                                    uStack10 = uStack30 / 0x64;
                                    uStack12 = 0x9;
                                    //                   TODO: goto LAB_1038_0841;
                                }
                            } else {
                                if (uStack6 % 0xa != 0x0) {
                                    // goto
                                    // LAB_1038_0841;
                                }
                                if (local_e != 0xa) {
                                    uStack10 = uStack30 / 0x64;
                                    uStack12 = 0xa;
                                    //                   TODO: goto LAB_1038_0841;
                                }
                            }
                        } else {
                            if (uStack6 % 0x5 != 0x0) {
                                // goto
                                // LAB_1038_0841;
                            }
                            if (local_e != 0xb) {
                                uStack10 = uStack30 / 0x64;
                                uStack12 = 0xb;
                                //                 TODO: goto LAB_1038_0841;
                            }
                        }
                    } else {
                        if (uStack6 % 0x3 != 0x0) {
                            // goto
                            // LAB_1038_0841;
                        }
                        if (local_e != 0xc) {
                            uStack10 = uStack30 / 0x64;
                            uStack12 = 0xc;
                            //               TODO: goto LAB_1038_0841;
                        }
                    }
                } else {
                    if (local_e != 0xd) {
                        uStack10 = uStack30 / 0x64;
                        uStack12 = 0xd;
                        //             TODO: goto LAB_1038_0841;
                    }
                }
            } else {
                if (local_e != 0xe) {
                    uStack10 = uStack30 / 0x14;
                    uStack12 = 0xe;
                    //           TODO: goto LAB_1038_0841;
                }
            }
        } else {
            if (local_e != 0xf) {
                uStack10 = uStack30 / 0xa;
                uStack12 = 0xf;
                //         TODO: goto LAB_1038_0841;
            }
        }
    }
    u_var2 = uStack30;
    if (local_12 < uStack30) {
        u_var2 = local_12 & 0xffff;
        uStack30._2_2_ = local_12._2_2_;
    }
    uStack10 = u_var2 & 0xffff | uStack30._2_2_ << 0x10;
    //LAB_1038_0841:
    if (uStack12 != 0x0) {
        if ((uStack30 != 0x0) && (uStack10 == 0x0)) {
            uStack10 = 0x1;
        }
        pass1_1038_4cd0(param_4, uStack10, uStack12);
    }
    if ((uStack10._2_2_ | uStack10) != 0x0) {
        // u_var5 = (param_4 >> 0x10);
        if ((param_4 + 0x200) == 0x8000001) {
            uStack64 = (uStack22 + 0x4);
        } else {
            uStack64 = 0x4000001;
        }
        pass1_1028_9944(
            CONCAT22(param_6, local_158),
            uStack10,
            (param_4 + 0x4),
            uStack64,
            param_6,
            param_7,
        );
        fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_6, local_158));
        pass1_1028_9992(CONCAT22(param_6, local_158));
    }
    return;
}

pub fn pass1_1038_08d4(
    param_1: u16,
    param_2: i32,
    param_3: u32,
    param_4: u32,
    param_5: u16,
    param_6: u8,
) {
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u16;
    let local_16: u16;
    let uStack20: u16;
    let i_stack4: i16;

    i_stack4 = 0x0;
    pass1_1028_dc52(CONCAT22(param_5, &local_16), 0x1, 0x0, 0x400);
    loop {
        pu_var1 = &local_16;
        pass1_1028_e4ec(CONCAT22(param_5, pu_var1));
        u_var2 = param_4;
        u_var3 = u_var2 | pu_var1;
        param_4 = param_4 & 0xffff0000 | u_var3;
        if (u_var3 == 0x0) {
            // goto
            // LAB_1038_0917;
        }
        if ((pu_var1 + 0x100) != 0x8000002) == false {
            break;
        }
    }
    i_stack4 = 0x1;
    //LAB_1038_0917:
    local_16 = 0x389a;
    uStack20 = 0x1008;
    if (i_stack4 != 0x0) {
        if (param_2 < 0xc90000) {
            pass1_1038_0340(
                param_1,
                param_2,
                param_2._2_2_,
                param_3,
                u_var3,
                param_5,
                param_6,
            );
            return;
        }
        if (0x31fffff < param_2) {
            pass1_1038_05d8(
                param_1,
                param_2,
                param_2._2_2_,
                param_3,
                param_4,
                param_5,
                param_6,
            );
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_095e(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: i16,
    param_7: u16,
) {
    let ppcVar1: u32;
    let bVar2: bool;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let u_var5: u32;
    let u_var6: u32;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let uVar9: u8;
    let puVar10: u32;
    let u_var11: u32;
    let iVar12: i16;
    let uStack58: u32;
    let uStack54: u32;
    let local_28: [u8; 2];
    let uStack38: u32;
    let uStack34: u32;
    let puStack30: u32;
    let uStack26: u16;
    let puStack24: U32Ptr;
    let puStack22: u32;
    let uStack18: u32;
    let iStack14: i16;
    let iStack12: i16;
    let uStack10: u32;
    let paStack6: &mut Struct67;

    paStack6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, param_7, param_5, param_6);
    uStack10 = *ctx.PTR_LOOP_1050_65e2;
    // uVar8 = (param_4 >> 0x10);
    if (uStack10 % 0xa == 0x0) {
        if (param_3 < 0xc9) {
            iVar12 = 0x3f;
        } else {
            if (param_3 < 0x320) {
                // goto
                // LAB_1038_09c3;
            }
            iVar12 = 0x3e;
        }
        post_win_msg_1008_a0e4(
            paStack6,
            0x0,
            0x0,
            0x1,
            (param_4 + 0x4),
            iVar12,
            0x1008,
            param_7,
        );
    }
    //LAB_1038_09c3:
    iStack12 = (param_4 + 0x22);
    iStack14 = 0x0;
    uStack18 = *ctx.PTR_LOOP_1050_65e2;
    if (iStack12 < 0x4b) {
        if (iStack12 < 0x3c) {
            if (iStack12 < 0x32) {
                // goto
                // LAB_1038_0a1c;
            }
            u_var3 = 0x1e;
        } else {
            u_var3 = 0xf;
        }
    } else {
        u_var3 = 0x5;
    }
    if ((uStack18 & 0xffff | (ctx.PTR_LOOP_1050_65e2 + 0x2) << 0x10) % u_var3 == 0x0) {
        iStack14 = 0x1;
    }
    //LAB_1038_0a1c:
    if (iStack14 != 0x0) {
        puVar10 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0xf);
        // puVar7 = (puVar10 >> 0x10);
        u_var3 = puVar10;
        pass1_1038_4e78(u_var3, puVar7, param_4, puVar10);
        puStack22 = CONCAT22(puVar7, u_var3);
        puVar10 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x1a);
        // puVar7 = (puVar10 >> 0x10);
        u_var3 = puVar10;
        uStack26 = u_var3;
        puStack24 = puVar7;
        pass1_1038_4d6e(param_4, puVar10, u_var3, puVar7);
        puStack30 = CONCAT22(puVar7, u_var3);
        ppcVar1 = (*puStack22 + 0x10);
        (**ppcVar1)(0x1008, puStack22, (puStack22 >> 0x10));
        uStack34 = CONCAT22(puVar7, u_var3);
        ppcVar1 = (*puStack30 + 0x10);
        (**ppcVar1)(0x1008, puStack30, (puStack30 >> 0x10));
        uStack38 = CONCAT22(puVar7, u_var3);
        u_var11 = pass1_1030_bcae(local_28, param_7);
        uStack54 = 0x0;
        loop {
            u_var11 >>= 0x10;
            uVar9 = 0x30;
            if (uStack34 <= uStack54) {
                break;
            }
            u_var6 = uStack34;
            pass1_1030_1d58(puStack22);
            u_var6 = u_var6 & 0xffff | u_var11 << 0x10;
            bVar2 = false;
            // for (uStack58 = 0x0; uStack58 < uStack38; uStack58 += 0x1) {
            //   u_var5 = uStack38;
            //   pass1_1030_1d58(puStack30);
            //   puVar4 = local_28;
            //   pass1_1030_bd74(puVar4,param_7,u_var6,u_var5 & 0xffff | u_var11 << 0x10,
            //                   param_7);
            //   if (puVar4 < 0x6) {
            //     bVar2 = true;
            //     break;
            //   }
            // }
            u_var11 = struct_op_1030_73a8(u_var6);
            if (!bVar2) {
                uVar9 = 0x28;
                func_0x10285ca0(0x1030, u_var11, (u_var11 >> 0x10));
                break;
            }
            uStack54 += 0x1;
        }
        if (puStack22 != 0x0) {
            ppcVar1 = *puStack22;
            (**ppcVar1)(uVar9, puStack22, (puStack22 >> 0x10), 0x1);
        }
        if (puStack30 != 0x0) {
            ppcVar1 = *puStack30;
            (**ppcVar1)(uVar9, puStack30, (puStack30 >> 0x10), 0x1);
        }
    }
    return;
}

pub fn pass1_1038_0b6a(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_0ba6(
    param_1: &mut Struct100,
    param_2: i16,
    param_3: u16,
    param_4: u8,
) -> Struct100 {
    let pu_var1: U32Ptr;
    let i_var2: &mut Struct701;
    let u_var2: u16;
    let paVar3: &mut Struct100;
    let puVar4: U32Ptr;

    paVar3 = struct_op_1028_d1dc(param_3, param_4, param_1, 0x270f);
    // pu_var1 = (paVar3 >> 0x10);
    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    &i_var2.field_0x108 = 0x0;
    param_1.field_0x0 = 0x1c2e;
    i_var2.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | &i_var2.field_0x8),
        ctx.s_SCMove_1050_59d8,
    );
    puVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_3, pu_var1, param_2);
    i_var2.field_0x108 = puVar4;
    i_var2.field_0x10a = (puVar4 >> 0x10);
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_0c00(
    param_1: u32,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u8,
) {
    let ppcVar1: u32;
    let u_var2: u32;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let puVar8: U32Ptr;
    let uVar9: u32;
    let u_var10: u16;
    let puVar11: u32;
    let puStack32: u32;
    let uStack24: u32;
    let local_14: [u8; 12];

    pass1_1028_dc52(
        CONCAT13((param_5 >> 0x8), CONCAT12(param_5, local_14)),
        0x1,
        0x0,
        0x400,
    );
    loop {
        pu_var3 = local_14;
        pass1_1028_e4ec(CONCAT22(param_5, pu_var3));
        u_var6 = param_2;
        uStack24 = CONCAT22(u_var6, pu_var3);
        uVar9 = param_2 & 0xffff0000 | (u_var6 | pu_var3);
        if ((u_var6 | pu_var3) == 0x0) {
            break;
        }
        pass1_1038_0e78(param_1, CONCAT22(u_var6, pu_var3), param_5);
        pass1_1038_1220(param_1, CONCAT22(u_var6, pu_var3), uVar9, param_5);
        // u_var10 = (uVar9 >> 0x10);
        puVar11 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x1);
        // puVar7 = (puVar11 >> 0x10);
        u_var4 = puVar11;
        pass1_1038_4d6e(CONCAT22(u_var6, pu_var3), puVar11, u_var4, puVar7);
        puStack32 = CONCAT22(puVar7, u_var4);
        ppcVar1 = (*puStack32 + 0x10);
        u_var5 = u_var4;
        puVar8 = puVar7;
        (**ppcVar1)(0x1008, u_var4, puVar7);
        param_2 = CONCAT22(u_var10, puVar8 | u_var5);
        if ((puVar8 | u_var5) != 0x0) {
            u_var2 = (param_1 + 0x108);
            if ((u_var2 + 0x82) != 0x0) {
                pass1_1038_19a0(
                    param_1,
                    CONCAT22(puVar7, u_var4),
                    CONCAT22(u_var6, pu_var3),
                    param_5,
                    param_6,
                );
            }
            pass1_1038_1940(
                param_1,
                CONCAT22(puVar7, u_var4),
                uStack24,
                param_3,
                param_4,
                param_5,
            );
        }
        if (puStack32 != 0x0) {
            ppcVar1 = *puStack32;
            (**ppcVar1)(0x8, u_var4, puVar7, 0x1);
        }
        pass1_1038_1c3e(param_1, uStack24, param_3, param_4, 0x1008, param_5);
    }
    return;
}

pub fn pass1_1038_0cf0(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let iVar5: i16;
    let puVar6: u32;
    let uVar7: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x10c, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // uVar7 = (param_1 >> 0x10);
        iVar5 = param_1;
        (param_2 + 0x4) = (iVar5 + 0x4);
        pu_var3 = (iVar5 + 0x8);
        puVar6 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = puVar6;
        //   puVar6 = puVar6 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        (param_2 + 0x108) = (iVar5 + 0x108);
        *puStack10 = 0x1c2e;
        (param_2 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    }
    return;
}

pub fn pass1_1038_0d8e(param_1: u16, param_2: u16, param_3: u32, param_4: u32, param_5: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let lStack10: i32;
    let uStack4: u16;

    u_var1 = pass1_1030_d0a8(param_4);
    u_var2 = pass1_1030_d144(param_4);
    lStack10 = u_var2;
    u_var2 = u_var2 >> 0xf | u_var2;
    uStack4 = u_var1;
    if (u_var2 != 0x0) {
        loop {
            u_var3 = pass1_1028_6744(param_5, param_3, uStack4);
            u_var2 |= u_var3;
            if (u_var2 != 0x0) {
                pass1_1028_6228(param_3, 0x1, 0x0, uStack4, param_5);
                lStack10 += -0x1;
                pass1_1030_d180(param_4, uStack4);
            }
            if (lStack10 == 0x0) {
                return;
            }
            uStack4 = pass1_1030_d0a8(param_4);
            if (uStack4 != u_var1) == false {
                break;
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_0e00(param_1: u32, param_2: U32Ptr, param_3: u32, param_4: u16, param_5: u16) {
    let ppcVar1: u32;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u32;
    let uStack10: u32;
    let uStack6: u32;

    ppcVar1 = (*param_2 + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_dx, param_4);
    // for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
    //   ppcVar1 = (*param_2 + 0x4);
    //   u_var4 = uStack6;
    //   (**ppcVar1)();
    //   u_var3 = u_var4;
    //   u_var2 = extraout_DX_00 | u_var3;
    //   if (u_var2 != 0x0) {
    //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var3,extraout_DX_00);
    //     u_var4 = struct_op_1030_73a8(CONCAT22(u_var2,u_var3));
    //     u_var3 = (u_var4 >> 0x10);
    //     if ((u_var3 | u_var4) != 0x0) {
    //       pass1_1038_0d8e(param_1,(param_1 >> 0x10),
    //                       u_var4 & 0xffff | u_var3 << 0x10,param_3,param_5);
    //     }
    //   }
    // }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_0e78(param_1: u32, param_2: u32, param_3: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let extraout_dx: u16;
    let puVar6: U32Ptr;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let uVar7: u16;
    let uVar8: u16;
    let puVar9: u32;
    let u_var10: u32;
    let uStack22: u32;
    let uStack18: u32;
    let puStack14: u32;
    let puStack10: u32;

    puVar9 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x4);
    // pu_var5 = (puVar9 >> 0x10);
    u_var2 = puVar9;
    pass1_1038_4d6e(param_2, puVar9, u_var2, pu_var5);
    puStack10 = CONCAT22(pu_var5, u_var2);
    u_var10 = *puStack10;
    ppcVar1 = u_var10 + 0x8;
    u_var3 = u_var2;
    (**ppcVar1)(0x1008, u_var2, pu_var5);
    if ((extraout_dx | u_var3) == 0x0) {
        if (puStack10 != 0x0) {
            ppcVar1 = u_var10;
            (**ppcVar1)(0x8, u_var2, pu_var5, 0x1);
            return;
        }
    } else {
        uVar8 = 0x1008;
        puVar9 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x1e);
        // puVar6 = (puVar9 >> 0x10);
        u_var3 = puVar9;
        pass1_1038_4d6e(param_2, puVar9, u_var3, puVar6);
        puStack14 = CONCAT22(puVar6, u_var3);
        ppcVar1 = (*puStack14 + 0x10);
        u_var4 = u_var3;
        (**ppcVar1)(0x1008, u_var3, puVar6);
        uStack18 = CONCAT22(extraout_DX_00, u_var4);
        // for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
        //   ppcVar1 = (*puStack14 + 0x4);
        //   u_var10 = uStack18;
        //   (**ppcVar1)();
        //   u_var4 = u_var10;
        //   uVar7 = extraout_DX_01 | u_var4;
        //   if (uVar7 != 0x0) {
        //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var4,extraout_DX_01);
        //     uVar8 = 0x1030;
        //     u_var10 = struct_op_1030_73a8(CONCAT22(uVar7,u_var4));
        //     if (((u_var10 >> 0x10) | u_var10) != 0x0) {
        //       pass1_1038_0e00(param_1,puStack10,u_var10,u_var10,param_3);
        //     }
        //   }
        // }
        if (puStack10 != 0x0) {
            ppcVar1 = *puStack10;
            (**ppcVar1)(uVar8, u_var2, pu_var5, 0x1);
        }
        if (puStack14 != 0x0) {
            ppcVar1 = *puStack14;
            (**ppcVar1)(uVar8, u_var3, puVar6, 0x1);
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_0f8c(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: u32,
    param_5: u16,
    param_6: u32,
    param_7: u16,
    param_8: u16,
) {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let ppcVar5: u32;
    let u_var6: u32;
    let qVar7: u64;
    let puVar8: U32Ptr;
    let uVar9: u32;
    let u_var10: u16;
    let u_var11: u16;
    let uVar12: u16;
    let iVar13: i16;
    let uVar14: u16;
    let paStack80: &mut Struct99;
    let uStack76: u16;
    let local_30: [u8; 4];
    let uStack44: u32;
    let puStack40: u32;
    let uStack36: u32;
    let local_20: [u8; 4];
    let puStack28: u32;
    let uStack24: u16;
    let uStack22: u16;
    let uStack20: u16;
    let uStack18: u16;
    let uStack16: u32;
    let uStack12: u32;
    let uStack8: u16;
    let uStack6: u32;

    uStack6 = 0x64;
    uStack8 = 0x0;
    ppcVar5 = (*param_3 + 0x10);
    (**ppcVar5)(param_7, param_3);
    uStack12 = CONCAT22(param_6, param_5);
    uStack16 = 0x0;
    loop {
        if (uStack12 <= uStack16) {
            return;
        }
        ppcVar5 = (*param_3 + 0x4);
        uVar9 = uStack12;
        u_var11 = param_6;
        (**ppcVar5)(
            param_7,
            param_3,
            (param_3 >> 0x10),
            uStack16,
            (uStack16 >> 0x10),
        );
        uStack18 = u_var11;
        uVar12 = uVar9;
        u_var11 = uStack18 | uVar12;
        param_6 = u_var11;
        uStack20 = uVar12;
        if (u_var11 != 0x0) {
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uVar12);
            uStack22 = u_var11;
            param_7 = 0x1030;
            uStack24 = uVar12;
            puStack28 = struct_op_1030_73a8(CONCAT22(uStack22, uVar12));
            param_6 = puStack28 >> 0x10;
            puVar8 = local_20;
            ppcVar5 = (*puStack28 + 0x40);
            (**ppcVar5)(0x1030, puStack28, (puStack28 >> 0x10), puVar8, param_8);
            if (puVar8 == 0x0) {
                uStack36 = pass1_1028_62c8(puStack28, param_8);
                uVar9 = uStack36 >> 0x10;
                uStack8 = 0x1;
                puStack40 = (param_4 + 0x22);
                pass1_1008_5784(CONCAT22(param_8, local_30), puStack40);
                loop {
                    u_var11 = uVar9;
                    puVar8 = local_30;
                    param_7 = 0x1008;
                    pass1_1008_5b12(puVar8, param_8);
                    param_6 = (u_var11 | puVar8);
                    if ((u_var11 | puVar8) == 0x0) {
                        break;
                    }
                    u_var2 = (puVar8 + 0x4);
                    u_var3 = (puVar8 + 0x6);
                    u_var4 = (puVar8 + 0x8);
                    uVar12 = (puVar8 + 0xa);
                    u_var6 = (puVar8 + 0xc) / uVar12;
                    uVar9 = uStack36;
                    if (uStack6 < uStack36) {
                        uVar9 = uStack6 & 0xffff;
                        uStack36._2_2_ = uStack6._2_2_;
                    }
                    u_var10 = uStack36._2_2_ | uVar9;
                    param_6 = u_var10;
                    if (u_var10 == 0x0) {
                        break;
                    }
                    qVar7 = (uVar9 & 0xffff | uStack36._2_2_ << 0x10) / u_var6;
                    param_6 = qVar7 >> 0x10;
                    uStack76 = qVar7;
                    if (uStack76 == 0x0) {
                        break;
                    }
                    if (uStack76 < uVar12) {
                        pi_var1 = (puVar8 + 0xc);
                        *pi_var1 = *pi_var1 - uVar9;
                        pi_var1 = (puVar8 + 0xa);
                        *pi_var1 = *pi_var1 - uStack76;
                    } else {
                        ppcVar5 = (*puStack40 + 0xc);
                        (**ppcVar5)(0x1008, puStack40, (puStack40 >> 0x10), puVar8, u_var11);
                        uStack44 = 0x0;
                        uStack76 = uVar12;
                    }
                    paStack80 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
                    // uVar12 = (paStack80 >> 0x10);
                    u_var11 = paStack80;
                    if ((uVar12 | u_var11) == 0x0) {
                        paStack80 = 0x0;
                    } else {
                        paStack80.field_0x0 = 0x389a;
                        (u_var11 + 0x2) = 0x1008;
                        (u_var11 + 0x4) = 0x0;
                        (u_var11 + 0x6) = 0x0;
                        (u_var11 + 0x8) = 0x0;
                        (u_var11 + 0xa) = 0x0;
                        (u_var11 + 0xc) = 0x0;
                        paStack80.field_0x0 = 0x56ce;
                        (u_var11 + 0x2) = 0x1018;
                    }
                    // uVar14 = (paStack80 >> 0x10);
                    iVar13 = paStack80;
                    (iVar13 + 0xa) = uStack76;
                    u_var6 = uStack76 * u_var6;
                    uVar9 = u_var6 >> 0x10;
                    (iVar13 + 0xc) = u_var6;
                    (iVar13 + 0x4) = u_var2;
                    (iVar13 + 0x6) = u_var3;
                    (iVar13 + 0x8) = u_var4;
                    pass1_1028_6408(puStack28, paStack80, param_8);
                }
            } else {
                ppcVar5 = (*param_3 + 0x8);
                (**ppcVar5)(0x1030, param_3, 0x0, 0x0, uStack16, (uStack16 >> 0x10));
            }
        }
        uStack16 += 0x1;
    }
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_11b0(
    param_1: u32,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: u16,
    param_5: u32,
    param_6: u16,
) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u32;
    let uStack10: u32;
    let uStack6: u32;

    ppcVar1 = (*param_3 + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(param_5, param_4);
    uStack10 = 0x0;
    loop {
        if (uStack6 <= uStack10) {
            return;
        }
        ppcVar1 = (*param_3 + 0x4);
        u_var3 = uStack6;
        (**ppcVar1)();
        u_var2 = u_var3;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
        u_var3 = struct_op_1030_73a8(CONCAT22(param_5, u_var2));
        param_5 = param_5 & 0xffff0000 | u_var3 >> 0x10;
        u_var2 = u_var3;
        pass1_1038_0f8c(
            param_1,
            (param_1 >> 0x10),
            param_2,
            u_var3,
            u_var2,
            param_5,
            0x1030,
            param_6,
        );
        if (u_var2 == 0x0) {
            break;
        }
        uStack10 += 0x1;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_1220(param_1: u32, param_2: u32, param_3: u32, param_4: u16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let puVar8: U32Ptr;
    let u_var10: u16;
    let uVar9: u32;
    let puVar11: u32;
    let uVar12: u8;
    let puStack14: u32;
    let puStack10: u32;

    // u_var10 = (param_3 >> 0x10);
    puVar11 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x4);
    // puVar6 = (puVar11 >> 0x10);
    u_var3 = puVar11;
    pass1_1038_4d6e(param_2, puVar11, u_var3, puVar6);
    puStack10 = CONCAT22(puVar6, u_var3);
    ppcVar1 = (*puStack10 + 0x10);
    puVar7 = puVar6;
    u_var4 = u_var3;
    (**ppcVar1)(0x1008, u_var3, puVar6);
    if ((puVar7 != 0x0) || (u_var4 != 0x0)) {
        puVar11 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x5);
        // puVar8 = (puVar11 >> 0x10);
        u_var4 = puVar11;
        pass1_1038_4d6e(param_2, puVar11, u_var4, puVar8);
        puStack14 = CONCAT22(puVar8, u_var4);
        uVar12 = u_var4;
        u_var2 = *puStack14;
        ppcVar1 = u_var2 + 0x8;
        puVar7 = puVar8;
        u_var5 = u_var4;
        (**ppcVar1)(0x1008, uVar12, puVar8);
        uVar9 = CONCAT22(u_var10, puVar7);
        if (((puVar7 != 0x0) || (u_var5 != 0x0))
            && (
                pass1_1038_11b0(
                    param_1,
                    CONCAT13((puVar6 >> 0x8), CONCAT12(puVar6, u_var3)),
                    CONCAT22(puVar8, u_var4),
                    u_var5,
                    uVar9,
                    param_4,
                ),
                u_var5 == 0x0,
            ))
        {
            if (puStack14 == 0x0) {
                return;
            }
            ppcVar1 = u_var2;
            (**ppcVar1)(0x8, uVar12, puVar8, 0x1);
            return;
        }
        // u_var10 = (uVar9 >> 0x10);
        if (puStack14 != 0x0) {
            ppcVar1 = *puStack14;
            (**ppcVar1)(0x8, uVar12, puVar8, 0x1);
        }
        puVar11 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x6);
        // puVar8 = (puVar11 >> 0x10);
        u_var4 = puVar11;
        pass1_1038_4d6e(param_2, puVar11, u_var4, puVar8);
        puStack14 = CONCAT22(puVar8, u_var4);
        ppcVar1 = (*puStack14 + 0x10);
        puVar7 = puVar8;
        u_var5 = u_var4;
        (**ppcVar1)(0x8, u_var4, puVar8);
        if ((puVar7 != 0x0) || (u_var5 != 0x0)) {
            pass1_1038_11b0(
                param_1,
                CONCAT22(puVar6, u_var3),
                CONCAT22(puVar8, u_var4),
                u_var5,
                CONCAT22(u_var10, puVar7),
                param_4,
            );
        }
        if (puStack14 != 0x0) {
            ppcVar1 = *puStack14;
            (**ppcVar1)(0x8, u_var4, puVar8, 0x1);
        }
    }
    if (puStack10 != 0x0) {
        ppcVar1 = *puStack10;
        (**ppcVar1)(0x8, u_var3, puVar6, 0x1);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_134a(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: U32Ptr,
    param_5: U32Ptr,
    param_6: u16,
    param_7: u16,
) {
    let ppcVar1: u32;
    let u_var2: u16;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var3: u16;
    let unaff_SS: u16;
    let u_var4: u32;
    let pu_var5: u32;
    let uStack6: u32;

    ppcVar1 = (*param_5 + 0x10);
    pu_var5 = param_5;
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_dx, param_6);
    *param_3 = 0x0;
    loop {
        if (uStack6 <= *param_4) {
            return;
        }
        u_var4 = *param_4;
        *param_4 = *param_4 + 0x1;
        ppcVar1 = (*param_5 + 0x4);
        (**ppcVar1)(param_7, param_5, u_var4, pu_var5);
        u_var2 = u_var4;
        u_var3 = extraout_DX_00;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
        u_var4 = struct_op_1030_73a8(CONCAT22(u_var3, u_var2));
        // u_var3 = (u_var4 >> 0x10);
        param_7 = &USHORT_1050_1028;
        u_var4 = pass1_1028_45e2(u_var4 & 0xffff | u_var3 << 0x10, u_var4, u_var3, unaff_SS);
        // u_var3 = (u_var4 >> 0x10);
        param_3 = u_var4;
        (param_3 + 0x2) = u_var3;
        if ((u_var3 | param_3) == 0x0) == false {
            break;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_13da(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: U32Ptr,
    param_5: U32Ptr,
    param_6: u16,
    param_7: u16,
) {
    let ppcVar1: u32;
    let u_var2: u16;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var3: u16;
    let u_var4: u32;
    let pu_var5: u32;
    let uStack6: u32;

    ppcVar1 = (*param_5 + 0x10);
    pu_var5 = param_5;
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_dx, param_6);
    *param_3 = 0x0;
    loop {
        if (uStack6 <= *param_4) {
            return;
        }
        u_var4 = *param_4;
        *param_4 = *param_4 + 0x1;
        ppcVar1 = (*param_5 + 0x4);
        (**ppcVar1)(param_7, param_5, u_var4, pu_var5);
        u_var2 = u_var4;
        u_var3 = extraout_DX_00;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
        if ((u_var3 | u_var2) == 0x0) {
            return;
        }
        u_var4 = struct_op_1030_73a8(CONCAT22(u_var3, u_var2));
        // u_var3 = (u_var4 >> 0x10);
        if ((u_var3 | u_var4) == 0x0) {
            return;
        }
        param_7 = &USHORT_1050_1028;
        u_var4 = pass1_1028_3c32((u_var4 & 0xffff | u_var3 << 0x10));
        // u_var3 = (u_var4 >> 0x10);
        param_3 = u_var4;
        (param_3 + 0x2) = u_var3;
        if ((u_var3 | param_3) == 0x0) == false {
            break;
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_1482(
    param_1: u32,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: u16,
) {
    let ppcVar1: u32;
    let sVar2: i64;
    let u_var3: u16;
    let puVar4: u32;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u32;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let u_var10: u16;
    let u_var11: u16;
    let uVar12: u8;
    let uVar13: u8;
    let uVar14: u16;
    let lStack74: i32;
    let local_46: u32;
    let local_42: [u16; 0x4];
    let uStack58: u16;
    let uStack56: u16;
    let puStack54: u32;
    let puStack50: u32;
    let uStack46: u32;
    let uStack42: u16;
    let uStack40: u16;
    let uStack38: u16;
    let uStack36: u16;
    let uStack34: u32;
    let uStack30: u16;
    let uStack28: u16;
    let uStack26: u16;
    let uStack24: u16;
    let uStack22: u32;
    let uStack18: u32;
    let uStack14: u32;
    let local_a: u32;
    let local_6: u32;

    local_6 = 0x0;
    local_a = 0x0;
    puVar4 = &local_a;
    // u_var11 = (param_1 >> 0x10);
    u_var3 = param_1;
    pass1_1038_134a(
        u_var3,
        u_var11,
        CONCAT22(param_6, puVar4),
        CONCAT22(param_6, &local_6),
        param_3,
        puVar4,
        param_4,
    );
    uStack14 = CONCAT22(param_5, puVar4);
    ppcVar1 = (*param_2 + 0x10);
    (**ppcVar1)(param_4, param_2);
    uStack18 = CONCAT22(param_5, puVar4);
    uStack22 = 0x0;
    loop {
        if (uStack18 <= uStack22) {
            return;
        }
        uStack14._2_2_ |= uStack14;
        if (uStack14._2_2_ == 0x0) {
            return;
        }
        pass1_1028_b58e(uStack14);
        uStack26 = uStack14._2_2_;
        uStack24 = uStack18._2_2_;
        pass1_1038_1a30(
            u_var3,
            u_var11,
            CONCAT22(uStack18._2_2_, uStack14._2_2_),
            &USHORT_1050_1028,
        );
        uStack30 = uStack14._2_2_;
        uStack28 = uStack18._2_2_;
        if ((uStack18._2_2_ | uStack14._2_2_) != 0x0) {
            sVar2 = CONCAT22(uStack18._2_2_, uStack14._2_2_) * 0x64;
            u_var5 = (sVar2 >> 0x20);
            uVar7 = sVar2 >> 0x1;
            ppcVar1 = (*param_2 + 0x4);
            uStack34 = uVar7;
            (**ppcVar1)(&USHORT_1050_1028, param_2, uStack22, (uStack22 >> 0x10));
            u_var6 = uVar7;
            uStack38 = u_var6;
            uStack36 = u_var5;
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var6);
            uStack42 = u_var6;
            uStack40 = u_var5;
            uStack46 = struct_op_1030_73a8(CONCAT22(u_var5, u_var6));
            puStack50 = (uStack46 + 0x28);
            puStack54 = 0x0;
            uStack56 = (puStack50 + 0x4);
            // for (uStack58 = 0x0; u_var5 = uStack56, uStack58 < uStack56; uStack58 += 0x1) {
            //   pass1_1020_bb16(puStack50,CONCAT22(param_6,&local_46),
            //
            //                   CONCAT13((param_6 >> 0x8),CONCAT12(param_6,local_42)),
            //                   uStack58);
            //   if (((local_46 != 0x0) && (0xd < local_42[0])) && (local_42[0] < 0x1d)) {
            //     uVar7 = local_46;
            //     if (uStack34 < local_46) {
            //       uVar7 = uStack34 & 0xffff;
            //       local_46._2_2_ = uStack34._2_2_;
            //     }
            //     u_var5 = uVar7;
            //     if ((local_a._2_2_ <= local_46._2_2_) &&
            //        ((local_a._2_2_ < local_46._2_2_ || (local_a < u_var5)))) {
            //       u_var5 = local_a;
            //       local_46._2_2_ = local_a._2_2_;
            //     }
            //     lStack74 = CONCAT22(local_46._2_2_,u_var5);
            //     uStack34 = CONCAT22(uStack34._2_2_ +
            //                         (-(uStack34 < u_var5) - local_46._2_2_),
            //                         uStack34 - u_var5);
            //     local_a = CONCAT22(local_a._2_2_ +
            //                        (-(local_a < u_var5) - local_46._2_2_),
            //                        local_a - u_var5);
            //     puVar9 = local_46._2_2_;
            //     if (puStack54 == 0x0) {
            //       puVar8 = local_46._2_2_;
            //       u_var10 = u_var5;
            //       mem_op_1000_179c(0xa,local_46._2_2_,0x1000);
            //       puVar9 = (puVar8 | u_var10);
            //       if (puVar9 == 0x0) {
            //         u_var10 = 0x0;
            //         puVar9 = 0x0;
            //       }
            //       else {
            //         pass1_1020_ba3e(CONCAT22(puVar8,u_var10),0x5,0x5,param_8,param_7);
            //       }
            //       puStack54 = CONCAT22(puVar9,u_var10);
            //     }
            //     pass1_1020_bb8a(puStack54,u_var5,CONCAT22(local_42[0],local_46._2_2_),
            //                     param_8,param_6);
            //     uVar7 = local_46 - lStack74;
            //     pass1_1020_bb8a(puStack50,uVar7,
            //                     CONCAT22(local_42[0],(uVar7 >> 0x10)),param_8,param_6);
            //     if (local_a == 0x0) {
            //       pass1_1038_1b3a(u_var3,u_var11,uStack14,puStack54,param_6,uVar7,param_7,
            //                       param_8);
            //       puStack54 = 0x0;
            //       uVar7 = ZEXT24(&local_a);
            //       pass1_1038_134a(u_var3,u_var11,CONCAT22(param_6,&local_a),
            //                       CONCAT22(param_6,&local_6),param_3,&local_a,0x1020);
            //       u_var5 = uVar7;
            //       uStack14 = uVar7 & 0xffff | ZEXT24(puVar9) << 0x10;
            //       u_var10 = puVar9 | u_var5;
            //       if (u_var10 != 0x0) {
            //         uVar12 = 0x64;
            //         uVar13 = 0x0;
            //         uVar14 = 0x0;
            //         pass1_1028_b58e(uVar7 & 0xffff | ZEXT24(puVar9) << 0x10);
            //         uStack26 = u_var5;
            //         uStack24 = u_var10;
            //         pass1_1038_1a30(u_var3,u_var11,CONCAT22(u_var10,u_var5),
            //                         &USHORT_1050_1028);
            //         uVar7 = (CONCAT22(u_var10,u_var5) *
            //                        CONCAT22(uVar14,CONCAT11(uVar13,uVar12))) >> 0x1;
            //         uStack34 = uVar7;
            //         uStack30 = u_var5;
            //         uStack28 = u_var10;
            //       }
            //     }
            //     u_var5 = uVar7;
            //     if ((uStack34 == 0x0) || (local_a == 0x0)) break;
            //   }
            // }
            if (puStack54 != 0x0) {
                pass1_1038_1b3a(
                    u_var3, u_var11, uStack14, puStack54, param_6, u_var5, param_7, param_8,
                );
                puStack54 = 0x0;
            }
        }
        uStack22 += 0x1;
    }
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_16f2(
    param_1: u32,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: u16,
    param_9: u8,
) {
    let plVar1: &i32;
    let ppcVar2: u32;
    let u_var3: u16;
    let puVar4: u32;
    let u_var5: u16;
    let puVar6: u32;
    let puVar7: u32;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let puVar12: U32Ptr;
    let uVar13: u32;
    let uVar14: u32;
    let uVar15: u32;
    let lVar16: i32;
    let uVar17: u16;
    let lStack68: i32;
    let puStack56: u32;
    let puStack52: u32;
    let plStack50: &i32;
    let uStack46: u16;
    let uStack42: u32;
    let uStack22: u32;
    let uStack18: u32;
    let uStack14: u32;
    let local_a: u32;
    let local_6: u32;

    local_6 = 0x0;
    local_a = 0x0;
    puVar6 = &local_a;
    // uVar17 = (param_1 >> 0x10);
    u_var3 = param_1;
    pass1_1038_13da(
        u_var3,
        uVar17,
        CONCAT22(param_8, puVar6),
        CONCAT22(param_8, &local_6),
        param_3,
        puVar6,
        param_7,
    );
    uStack14 = CONCAT22(param_4, puVar6);
    uVar8 = param_4 | puVar6;
    if (uVar8 != 0x0) {
        ppcVar2 = (*param_2 + 0x10);
        (**ppcVar2)(param_7, param_2);
        uStack18 = CONCAT22(uVar8, puVar6);
        // for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
        //   ppcVar2 = (*param_2 + 0x4);
        //   uVar15 = uStack18;
        //   u_var10 = uVar8;
        //   (**ppcVar2)(param_7,param_2,uStack22,(uStack22 >> 0x10));
        //   u_var5 = uVar15;
        //   pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var5,u_var10);
        //   param_7 = 0x1030;
        //   uVar15 = struct_op_1030_73a8(CONCAT22(u_var10,u_var5));
        //   u_var11 = (uVar15 >> 0x10);
        //   uVar9 = uVar15;
        //   pass1_1038_1a30(u_var3,uVar17,CONCAT22(u_var10,u_var5),0x1030);
        //   if ((u_var11 | uVar9) != 0x0) {
        //     uStack42 = (CONCAT22(u_var11,uVar9) * 0x64) >> 0x1;
        //     plVar1 = *(long **)(u_var5 + 0x22);
        //     uVar9 = (u_var5 + 0x24);
        //     uVar13 = uVar9;
        //     uStack46 = plVar1;
        //     if ((uVar9 | uStack46) != 0x0) {
        //       plStack50 = 0x0;
        //       puVar6 = pass1_1028_0d80(uVar15);
        //       puStack56 = 0x0;
        //       puStack52 = puVar6;
        //       loop {
        //         lVar16 = pass1_1020_bae6(uStack46,CONCAT22(puStack52,
        //                                                    (plVar1 >> 0x10)),
        //                                  puStack52,uVar13,param_8);
        //         uVar9 = (lVar16 >> 0x10);
        //         puVar7 = lVar16;
        //         uVar13 = (uVar9 | puVar7);
        //         if ((uVar9 | puVar7) != 0x0) {
        //           uVar14 = uVar9;
        //           if ((uStack42._2_2_ <= uVar9) &&
        //              ((uStack42._2_2_ < uVar9 || (uStack42 < puVar7)))) {
        //             uVar14 = uStack42._2_2_;
        //             puVar7 = uStack42;
        //           }
        //           if ((local_a._2_2_ <= uVar14) &&
        //              ((local_a._2_2_ < uVar14 || (local_a < puVar7)))) {
        //             uVar14 = local_a._2_2_;
        //             puVar7 = local_a;
        //           }
        //           puVar12 = uVar14;
        //           lStack68 = CONCAT22(puVar12,puVar7);
        //           uStack42 = CONCAT22((uStack42._2_2_ - puVar12) -
        //                               (uStack42 < puVar7),
        //
        //                               (uStack42 - puVar7));
        //           local_a = CONCAT22((local_a._2_2_ - puVar12) -
        //                              (local_a < puVar7),
        //                              (local_a - puVar7))
        //           ;
        //           uVar13 = uVar14;
        //           if (plStack50 == 0x0) {
        //             puVar4 = puVar7;
        //             mem_op_1000_179c(0xa,puVar12,0x1000);
        //             uVar13 = (puVar12 | puVar4);
        //             if ((puVar12 | puVar4) == 0x0) {
        //               puVar4 = 0x0;
        //               uVar13 = 0x0;
        //             }
        //             else {
        //               pass1_1020_ba3e(CONCAT22(puVar12,puVar4),0x5,0x5,param_6,param_5
        //                              );
        //             }
        //             plStack50 = CONCAT22(uVar13,puVar4);
        //           }
        //           pass1_1020_bb8a(plStack50,puVar7,uVar14 | ZEXT24(puStack52) << 0x10,
        //                           param_6,param_8);
        //           pass1_1020_bb8a(plVar1,(lVar16 - lStack68),
        //                           CONCAT22(puStack52,((lVar16 - lStack68) >> 0x10)
        //                                   ),param_6,param_8);
        //           uVar9 = uVar13;
        //           puStack56 = puStack52;
        //           puVar7 = puStack52;
        //           if (local_a == 0x0) {
        //             pass1_1038_1ac6(u_var3,uVar17,uStack14,plStack50,puStack52,
        //                             param_8,param_9);
        //             plStack50 = 0x0;
        //             puVar7 = &local_a;
        //             pass1_1038_13da(u_var3,uVar17,CONCAT22(param_8,puVar7),
        //                             CONCAT22(param_8,&local_6),param_3,puVar7
        //                             ,0x1020);
        //             uStack14 = CONCAT22(uVar9,puVar7);
        //             uVar13 = (uVar9 | puVar7);
        //             if ((uVar9 | puVar7) == 0x0) {
        //               return;
        //             }
        //           }
        //         }
        //         param_7 = 0x1020;
        //         if ((uStack42 == 0x0) || (local_a == 0x0)) break;
        //         param_7 = &USHORT_1050_1028;
        //         puVar7 = pass1_1028_0d80(uVar15);
        //         if ((puVar7 == puStack56) ||
        //            ((puStack52 = puVar7, puStack56 == 0x0 && (puVar7 == puVar6))
        //            )) break;
        //       }
        //       if (plStack50 != 0x0) {
        //         pass1_1038_1ac6(u_var3,uVar17,uStack14,plStack50,puVar7,param_8,
        //                         param_9);
        //       }
        //     }
        //   }
        // }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_1940(
    param_1: u32,
    param_2: U32Ptr,
    param_3: u32,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let extraout_dx: u16;
    let pu_var5: u32;
    let puStack10: u32;

    pu_var5 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x3);
    // puVar4 = (pu_var5 >> 0x10);
    u_var2 = pu_var5;
    pass1_1038_4d6e(param_3, pu_var5, u_var2, puVar4);
    puStack10 = CONCAT22(puVar4, u_var2);
    ppcVar1 = (*puStack10 + 0x10);
    u_var3 = u_var2;
    (**ppcVar1)(0x1008, u_var2, puVar4);
    if ((extraout_dx | u_var3) != 0x0) {
        pass1_1038_1482(
            param_1,
            param_2,
            puStack10,
            0x1008,
            extraout_dx | u_var3,
            param_6,
            param_4,
            param_5,
        );
    }
    if (puStack10 != 0x0) {
        ppcVar1 = *puStack10;
        (**ppcVar1)(0x1008, u_var2, puVar4, 0x1);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_19a0(param_1: u32, param_2: U32Ptr, param_3: u32, param_4: u16, param_5: u8) {
    let ppcVar1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let extraout_dx: u16;
    let ppcVar6: u32;
    let puVar7: u32;
    let puStack10: u32;

    puVar7 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x2);
    // pu_var5 = (puVar7 >> 0x10);
    u_var3 = puVar7;
    pass1_1038_4d6e(param_3, puVar7, u_var3, pu_var5);
    puStack10 = CONCAT22(pu_var5, u_var3);
    u_var2 = *puStack10;
    ppcVar6 = u_var2;
    ppcVar1 = ppcVar6 + 0x8;
    u_var4 = u_var3;
    (**ppcVar1)(0x1008, u_var3, pu_var5);
    if ((extraout_dx | u_var4) == 0x0) {
        vsprintf_op_1030_840a(s_mineToSmelter__no_mines_1050_59df, 0x1030, param_4, 0x0);
        if (puStack10 != 0x0) {
            ppcVar1 = ppcVar6;
            (**ppcVar1)(0x1030, u_var3, pu_var5, 0x1);
            return;
        }
    } else {
        pass1_1038_16f2(
            param_1,
            puStack10,
            param_2,
            extraout_dx | u_var4,
            ppcVar6,
            (u_var2 >> 0x10),
            0x1008,
            param_4,
            param_5,
        );
        if (puStack10 != 0x0) {
            ppcVar1 = *puStack10;
            (**ppcVar1)(0x1008, u_var3, pu_var5, 0x1);
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_1a30(param_1: u16, param_2: u16, param_3: u32, param_4: u16) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let u_var3: u16;
    let u_var4: u32;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let uStack18: u32;
    let uStack10: u32;
    let uStack6: u16;

    // u_var5 = (param_3 >> 0x10);
    pu_var1 = (param_3 + 0x1e);
    uVar7 = (param_3 + 0x20);
    uStack6 = pu_var1;
    u_var3 = uVar7 | uStack6;
    if (u_var3 != 0x0) {
        ppcVar2 = (*pu_var1 + 0x10);
        u_var6 = uStack6;
        (**ppcVar2)();
        uStack10 = CONCAT22(extraout_dx, u_var3);
        // for (uStack18 = 0x0; uStack18 < uStack10; uStack18 += 0x1) {
        //   ppcVar2 = (*pu_var1 + 0x4);
        //   u_var4 = uStack10;
        //   (**ppcVar2)(param_4,uStack6,(pu_var1 >> 0x10),uStack18,u_var6,uVar7);
        //   if ((extraout_DX_00 | u_var4) != 0x0) {
        //     param_4 = &USHORT_1050_1028;
        //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var4,extraout_DX_00);
        //   }
        // }
        return;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_1ac6(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: i16,
    param_6: u16,
    param_7: u8,
) {
    let extraout_dx: u16;
    let local_118: [u8; 112];
    let uStack6: u32;

    pass1_1028_b58e(param_3);
    uStack6 = CONCAT22(extraout_dx, param_5);
    pass1_1030_e8a0(
        CONCAT22(param_6, local_118),
        param_4,
        (param_3 + 0xc),
        (param_5 + 0x4),
        param_6,
        param_7,
    );
    pass1_1028_d52c(
        *ctx.PTR_LOOP_1050_5748,
        *ctx.PTR_LOOP_1050_65e2 + 0x1,
        CONCAT22(param_6, local_118),
    );
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1038_1b3a(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: U32Ptr,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: u16,
) {
    let extraout_dx: i16;
    let local_1a: u32;
    let local_16: [u16; 0x2];
    let uStack18: u16;
    let uStack16: u16;
    let uStack14: u32;
    let uStack10: u32;
    let uStack6: u32;

    pass1_1028_b58e(param_3);
    uStack6 = CONCAT22(extraout_dx, param_6);
    uStack10 = param_3;
    uStack14 = pass1_1028_45e2(param_3, param_3, extraout_dx, param_5);
    uStack16 = (param_4 + 0x4);
    // for (uStack18 = 0x0; uStack18 < uStack16; uStack18 += 0x1) {
    //   pass1_1020_bb16(param_4,CONCAT22(param_5,&local_1a),
    //                   CONCAT22(param_5,local_16),uStack18);
    //   if (uStack14 < local_1a) {
    //     pass1_1030_7ddc(uStack6,uStack14,local_16[0],uStack14,uStack14._2_2_,param_7,
    //                     param_8,param_5);
    //     uStack14 = 0x0;
    //   }
    //   else {
    //     uStack14 -= local_1a;
    //     pass1_1030_7ddc(uStack6,local_1a,local_16[0],local_1a,uStack14._2_2_,param_7,
    //                     param_8,param_5);
    //   }
    //   if (uStack14 == 0x0) break;
    // }
    if (param_4 != 0x0) {
        fn_ptr_1020_ba7e(param_4);
        fn_ptr_1000_17ce(ctx, param_4, 0x1000);
    }
    return;
}

pub fn pass1_1038_1c02(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_1c3e(
    param_1: u32,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let u_var1: u16;
    let pu_var2: u32;
    let ppc_var3: u32;
    let u_var4: u32;
    let u_var5: u16;
    let iVar6: i16;
    let b_var7: bool;
    let puVar8: u32;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u32;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u16;
    let uStack26: u32;
    let uStack14: u32;

    // u_var10 = (param_2 >> 0x10);
    pu_var2 = (param_2 + 0xc);
    u_var10 = (param_2 + 0xe);
    ppc_var3 = (*pu_var2 + 0x10);
    puVar8 = pu_var2;
    uVar14 = pu_var2;
    (**ppc_var3)();
    u_var4 = puVar8 & 0xffff | extraout_dx << 0x10;
    uStack14 = 0x0;
    loop {
        if (u_var4 <= uStack14) {
            return;
        }
        ppc_var3 = (*pu_var2 + 0x4);
        u_var11 = u_var4;
        (**ppc_var3)(
            param_5,
            pu_var2,
            (pu_var2 >> 0x10),
            uStack14,
            uVar14,
            u_var10,
        );
        u_var5 = u_var11;
        uVar9 = extraout_DX_00 | u_var5;
        if (uVar9 != 0x0) {
            param_5 = &USHORT_1050_1028;
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var5);
            uStack26 = CONCAT22(uVar9, u_var5);
            iVar6 = (u_var5 + 0x34);
            if ((iVar6 != 0x0) && ((u_var5 + 0x36) != 0x0)) {
                uVar12 = param_1;
                // uVar13 = (param_1 >> 0x10);
                pass1_1038_201a(uVar12, uVar13, CONCAT22(uVar9, u_var5), iVar6, uVar9);
                if (iVar6 == 0x0) {
                    u_var11 = struct_op_1030_73a8(uStack26);
                    u_var1 = (u_var11 + 0xc);
                    param_5 = 0x1008;
                    b_var7 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x1);
                    if (b_var7 == 0x0) {
                        param_5 = 0x1008;
                        b_var7 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x2);
                        if (b_var7 == 0x0) {
                            b_var7 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x5);
                            if (b_var7 == 0x0) {
                                param_5 = 0x1008;
                                b_var7 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var1, 0x6);
                                if (b_var7 == 0x0) {
                                    // goto
                                    // LAB_1038_1c76;
                                }
                            }
                            param_5 = 0x1008;
                            pass1_1038_2306(uVar12, uVar13, uStack26);
                        } else {
                            pass1_1038_26ee(uVar12, uVar13, uStack26, param_3, param_4, param_6);
                        }
                    } else {
                        pass1_1038_24e8(uVar12, uVar13, uStack26, param_3, param_4, param_6);
                    }
                }
            }
        }
        //LAB_1038_1c76:
        uStack14 += 0x1;
    }
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_1d68(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: u32,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: u32,
) {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let ppcVar6: u32;
    let uVar7: u32;
    let uVar8: u16;
    let bVar9: bool;
    let puVar10: U32Ptr;
    let u_var11: u32;
    let uVar12: u16;
    let uVar13: u16;
    let iVar14: i16;
    let puVar15: u32;
    let paStack82: &mut Struct99;
    let uStack78: u16;
    let uStack52: u32;
    let local_30: [u8; 4];
    let uStack44: u32;
    let puStack40: u32;
    let uStack36: u32;
    let local_20: [u8; 4];
    let puStack28: u32;
    let uStack24: u16;
    let uStack22: u16;
    let uStack20: u16;
    let uStack18: u16;
    let uStack16: u32;
    let uStack12: u32;
    let uStack8: u16;
    let uStack6: u32;

    uStack6 = 0x64;
    uStack8 = 0x0;
    ppcVar6 = (*param_3 + 0x10);
    puVar15 = param_3;
    (**ppcVar6)();
    uStack12 = CONCAT22(param_8, param_5);
    uStack16 = 0x0;
    loop {
        if (uStack12 <= uStack16) {
            return;
        }
        ppcVar6 = (*param_3 + 0x4);
        u_var11 = uStack12;
        uVar13 = param_8;
        (**ppcVar6)(param_6, param_3, uStack16, puVar15);
        uStack18 = uVar13;
        uVar12 = u_var11;
        uVar13 = uStack18 | uVar12;
        param_8 = uVar13;
        uStack20 = uVar12;
        if (uVar13 != 0x0) {
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uVar12);
            uStack22 = uVar13;
            param_6 = 0x1030;
            uStack24 = uVar12;
            puStack28 = struct_op_1030_73a8(CONCAT22(uStack22, uVar12));
            param_8 = puStack28 >> 0x10;
            puVar10 = local_20;
            ppcVar6 = (*puStack28 + 0x40);
            (**ppcVar6)(0x1030, puStack28, (puStack28 >> 0x10), puVar10, param_7);
            if (puVar10 == 0x0) {
                uStack36 = pass1_1028_62c8(puStack28, param_7);
                u_var11 = uStack36 >> 0x10;
                uStack8 = 0x1;
                puStack40 = (param_4 + 0x22);
                pass1_1008_5784(CONCAT22(param_7, local_30), puStack40);
                loop {
                    uVar13 = u_var11;
                    puVar10 = local_30;
                    param_6 = 0x1008;
                    pass1_1008_5b12(puVar10, param_7);
                    uStack52 = CONCAT22(uVar13, puVar10);
                    param_8 = (uVar13 | puVar10);
                    if ((uVar13 | puVar10) == 0x0) {
                        break;
                    }
                    u_var2 = (puVar10 + 0x4);
                    i_var3 = (puVar10 + 0x6);
                    u_var4 = (puVar10 + 0x8);
                    uVar12 = (puVar10 + 0xc);
                    u_var5 = (puVar10 + 0xa);
                    uVar8 = uVar12 / u_var5;
                    u_var11 = uVar12 % u_var5;
                    bVar9 = false;
                    if (((0x0 < i_var3) && (!SBORROW2(i_var3, 0x1)))
                        && (i_var3 == 0x5 || i_var3 + -0x1 < 0x4 || (i_var3 == 0x8)))
                    {
                        bVar9 = true;
                    }
                    if (bVar9) {
                        u_var11 = uStack36;
                        if (uStack6 < uStack36) {
                            u_var11 = uStack6 & 0xffff;
                            uStack36._2_2_ = uStack6._2_2_;
                        }
                        uVar12 = uStack36._2_2_ | u_var11;
                        param_8 = uVar12;
                        if (uVar12 == 0x0) {
                            break;
                        }
                        uStack78 = ((u_var11 & 0xffff | uStack36._2_2_ << 0x10) / uVar8);
                        if (uStack78 < u_var5) {
                            pi_var1 = (puVar10 + 0xc);
                            *pi_var1 = *pi_var1 - u_var11;
                            pi_var1 = (puVar10 + 0xa);
                            *pi_var1 = *pi_var1 - uStack78;
                        } else {
                            ppcVar6 = (*puStack40 + 0xc);
                            (**ppcVar6)(0x1008, puStack40, (puStack40 >> 0x10), uStack52);
                            uStack44 = 0x0;
                            uStack78 = u_var5;
                        }
                        paStack82 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
                        // uVar12 = (paStack82 >> 0x10);
                        uVar13 = paStack82;
                        if ((uVar12 | uVar13) == 0x0) {
                            paStack82 = 0x0;
                        } else {
                            paStack82.field_0x0 = 0x389a;
                            (uVar13 + 0x2) = 0x1008;
                            (uVar13 + 0x4) = 0x0;
                            (uVar13 + 0x6) = 0x0;
                            (uVar13 + 0x8) = 0x0;
                            (uVar13 + 0xa) = 0x0;
                            (uVar13 + 0xc) = 0x0;
                            paStack82.field_0x0 = 0x56ce;
                            (uVar13 + 0x2) = 0x1018;
                        }
                        // uVar13 = (paStack82 >> 0x10);
                        iVar14 = paStack82;
                        (iVar14 + 0xa) = uStack78;
                        uVar7 = uStack78 * uVar8;
                        u_var11 = uVar7 >> 0x10;
                        (iVar14 + 0xc) = uVar7;
                        (iVar14 + 0x4) = u_var2;
                        (iVar14 + 0x6) = i_var3;
                        (iVar14 + 0x8) = u_var4;
                        pass1_1028_6408(puStack28, (paStack82 & 0xffff | uVar13 << 0x10), param_7);
                    }
                }
            } else {
                ppcVar6 = (*param_3 + 0x8);
                (**ppcVar6)(0x1030, param_3, 0x0, uStack16);
            }
        }
        uStack16 += 0x1;
    }
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_1faa(
    param_1: u32,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: u16,
    param_5: u32,
    param_6: u16,
) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u32;
    let uStack10: u32;
    let uStack6: u32;

    ppcVar1 = (*param_3 + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(param_5, param_4);
    uStack10 = 0x0;
    loop {
        if (uStack6 <= uStack10) {
            return;
        }
        ppcVar1 = (*param_3 + 0x4);
        u_var3 = uStack6;
        (**ppcVar1)();
        u_var2 = u_var3;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
        u_var3 = struct_op_1030_73a8(CONCAT22(param_5, u_var2));
        param_5 = param_5 & 0xffff0000 | u_var3 >> 0x10;
        u_var2 = u_var3;
        pass1_1038_1d68(
            param_1,
            (param_1 >> 0x10),
            param_2,
            u_var3,
            u_var2,
            0x1030,
            param_6,
            param_5,
        );
        if (u_var2 == 0x0) {
            break;
        }
        uStack10 += 0x1;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_201a(param_1: u16, param_2: u16, param_3: u32, param_4: u16, param_5: u16) {
    let pu_var1: U32Ptr;
    let i_var2: i16;
    let ppc_var3: u32;
    let lVar4: i32;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u32;
    let puVar10: U32Ptr;
    let u_var11: u32;
    let uVar12: u32;
    let iVar12: &mut Struct416;
    let uVar13: u16;
    let puVar14: U32Ptr;
    let uVar15: u16;
    let puVar16: u32;
    let uVar17: u16;
    let lStack24: i32;
    let lStack20: i32;
    let uStack10: u16;
    let u_var5: &mut Struct413;

    // uVar17 = (param_3 >> 0x10);
    uVar15 = 0x1030;
    puVar16 = pass1_1030_6b16(param_3);
    // u_var6 = (puVar16 >> 0x10);
    u_var5 = puVar16;
    if ((u_var6 | u_var5) == 0x0) {
        return;
    }
    iVar12 = param_3;
    i_var2 = iVar12.field_0x34;
    lVar4 = i_var2;
    uVar12 = lVar4 * 0x64;
    // puVar10 = (uVar12 >> 0x10);
    uVar7 = uVar12;
    uStack10 = 0x0;
    lStack20 = 0x0;
    if (u_var5.field_0x4 == 0x0) {
        if (u_var5.field_0x6 == 0x0) {
            if (u_var5.field_0x8 == 0x0) {
                // goto
                // LAB_1038_2102;
            }
            uVar8 = pass1_1020_c42e(u_var5.field_0x8);
            u_var11 = u_var5.field_0xa * uVar8;
            // puVar14 = (u_var11 >> 0x10);
            if (u_var11 + lVar4 * -0x64 != 0x0 && uVar12 <= u_var11) {
                u_var11 = uVar12 & 0xffff;
                puVar14 = puVar10;
            }
            uVar12 = u_var11 & 0xffff | ZEXT24(puVar14) << 0x10;
            uVar9 = (u_var11 & 0xffff | ZEXT24(puVar14) << 0x10) / uVar8;
            pu_var1 = &u_var5.field_0xa;
            *pu_var1 = *pu_var1 - uVar9;
            uStack10 = (uVar12 / 0x64);
            uVar12 = uVar12 % 0x64;
            u_var11 = uVar12;
            if (uVar12 != 0x0) {
                uStack10 += 0x1;
                u_var11 = uStack10;
            }
            uVar7 = u_var11;
            mem_op_1000_179c(0x2a, uVar12, 0x1000);
            puVar10 = (uVar12 | uVar7);
            if (puVar10 == 0x0) {
                // goto
                // LAB_1038_20fa;
            }
            pass1_1038_6838(
                CONCAT22(uVar12, uVar7),
                uVar9,
                u_var5.field_0x8,
                uStack10,
                iVar12.field_0x4,
            );
        } else {
            uVar8 = switch_1020_c3b4(u_var5.field_0x6);
            u_var11 = u_var5.field_0xa * uVar8;
            // puVar14 = (u_var11 >> 0x10);
            if (u_var11 + lVar4 * -0x64 != 0x0 && uVar12 <= u_var11) {
                u_var11 = uVar12 & 0xffff;
                puVar14 = puVar10;
            }
            uVar12 = u_var11 & 0xffff | ZEXT24(puVar14) << 0x10;
            uVar9 = (u_var11 & 0xffff | ZEXT24(puVar14) << 0x10) / uVar8;
            pu_var1 = &u_var5.field_0xa;
            *pu_var1 = *pu_var1 - uVar9;
            uStack10 = (uVar12 / 0x64);
            uVar12 = uVar12 % 0x64;
            u_var11 = uVar12;
            if (uVar12 != 0x0) {
                uStack10 += 0x1;
                u_var11 = uStack10;
            }
            uVar7 = u_var11;
            mem_op_1000_179c(0x2a, uVar12, 0x1000);
            puVar10 = (uVar12 | uVar7);
            if (puVar10 == 0x0) {
                // goto
                // LAB_1038_20fa;
            }
            pass1_1038_675c(
                CONCAT22(uVar12, uVar7),
                uVar9,
                u_var5.field_0x6,
                uStack10,
                iVar12.field_0x4,
            );
        }
    } else {
        uVar13 = u_var5.field_0xa;
        puVar14 = 0x0;
        if ((puVar10 < 0x1) && (0x7fff < puVar10 || (uVar7 < uVar13))) {
            uVar13 = uVar7;
            puVar14 = puVar10;
        }
        lStack24 = CONCAT22(puVar14, uVar13);
        pu_var1 = &u_var5.field_0xa;
        *pu_var1 = *pu_var1 - uVar13;
        uStack10 = (lStack24 / 0x64);
        u_var11 = lStack24 % 0x64;
        uVar12 = u_var11;
        if (u_var11 != 0x0) {
            uStack10 += 0x1;
            uVar12 = uStack10;
        }
        uVar7 = uVar12;
        mem_op_1000_179c(0x2a, u_var11, 0x1000);
        puVar10 = (u_var11 | uVar7);
        if (puVar10 == 0x0) {
            //LAB_1038_20fa:
            uVar15 = 0x1000;
            lStack20 = 0x0;
            //       TODO: goto LAB_1038_2102;
        }
        pass1_1038_6590(
            CONCAT22(u_var11, uVar7),
            uVar13,
            puVar14,
            u_var5.field_0x4,
            uStack10,
            iVar12.field_0x4,
        );
    }
    uVar15 = 0x1000;
    lStack20 = CONCAT22(puVar10, uVar7);
    //LAB_1038_2102:
    if (lStack20 != 0x0) {
        pass1_1038_7a5a(ctx.PTR_LOOP_1050_5a64);
        uVar15 = 0x1030;
        uVar7 = uStack10;
        pass1_1030_6c4c(param_3, i_var2 - uStack10);
    }
    if (u_var5.field_0xa == 0x0) {
        if ((u_var6 | u_var5) != 0x0) {
            ppc_var3 = *puVar16;
            (**ppc_var3)(uVar15, u_var5, u_var6, 0x1);
        }
    } else {
        pass1_1030_6c66(param_3, 0x0, puVar16, uVar7, puVar10, 0x1030);
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_2306(param_1: u16, param_2: u16, param_3: u32) {
    let pi_var1: U32Ptr;
    let pu_var2: u32;
    let ppc_var3: u32;
    let qVar4: u64;
    let pu_var5: u32;
    let uVar9: &mut Struct417;
    let u_var6: u32;
    let puVar7: u32;
    let uVar8: u16;
    let u_var10: u16;
    let iVar11: &mut Struct419;
    let iVar12: &mut Struct418;
    let u_var11: u16;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u32;
    let uStack42: u32;
    let uStack34: u32;
    let uStack30: u16;
    let uStack24: u32;
    let uStack12: u32;
    let iStack8: i16;

    uVar13 = 0x1030;
    uVar14 = struct_op_1030_73a8(param_3);
    uStack24 = uVar14 >> 0x10;
    // u_var11 = (param_3 >> 0x10);
    iVar11 = param_3;
    iStack8 = iVar11.field_0x34;
    uStack12 = 0x64;
    pu_var2 = (uVar14 + 0x22);
    puVar7 = pu_var2;
    loop {
        uVar8 = uStack24;
        // uVar12 = (pu_var2 >> 0x10);
        ppc_var3 = (*pu_var2 + 0x10);
        (**ppc_var3)(uVar13, pu_var2, uVar12);
        uVar9 = puVar7;
        uVar14 = puVar7 & 0xffff;
        pu_var5 = (uVar14 | uVar8 << 0x10);
        if ((uVar8 | uVar9) == 0x0) {
            break;
        }
        if (uVar9.field_0xa == 0x0) {
            uStack24 = (uVar8 | uVar9);
            if ((uVar8 | uVar9) != 0x0) {
                ppc_var3 = *pu_var5;
                (**ppc_var3)(uVar13, uVar9, uVar8, 0x1);
            }
        } else {
            uStack24 = 0x0;
            uStack30 = 0x0;
            if (uVar9.field_0x6 == 0x0) {
                if (uVar9.field_0x8 != 0x0) {
                    uStack30 = pass1_1020_c42e(uVar9.field_0x8);
                    //           TODO: goto LAB_1038_2385;
                }
            } else {
                uStack30 = switch_1020_c3b4(uVar9.field_0x6);
                //LAB_1038_2385:
                uVar13 = 0x1020;
                uStack24 = (uVar9.field_0xa * uStack30);
            }
            uStack12._2_2_ = 0x0;
            if (uStack12 < uStack24) {
                uStack24 = uStack12 & 0xffff;
            }
            uStack34 = uStack24 | uStack12._2_2_ << 0x10;
            uStack24 |= uStack12._2_2_ << 0x10;
            qVar4 = uStack24 / uStack30;
            u_var6 = qVar4;
            uStack24 %= uStack30;
            pi_var1 = &uVar9.field_0xa;
            *pi_var1 = *pi_var1 - qVar4;
            if (*pi_var1 == 0x0) {
                uStack24 = (uVar8 | uVar9);
                if ((uVar8 | uVar9) != 0x0) {
                    ppc_var3 = *pu_var5;
                    (**ppc_var3)(uVar13, uVar9, uVar8, 0x1);
                }
            } else {
                ppc_var3 = (*pu_var2 + 0x8);
                (**ppc_var3)(uVar13, pu_var2, uVar12, uVar9, uVar8);
            }
            uStack12 -= uStack34;
            puVar7 = 0x0;
            uStack42 = 0x0;
            iVar12 = uVar14;
            if (iVar12.field_0x6 == 0x0) {
                if (iVar12.field_0x8 != 0x0) {
                    mem_op_1000_179c(0x2a, uStack24, 0x1000);
                    u_var10 = uStack24 | puVar7;
                    uVar14 = u_var10;
                    if (u_var10 == 0x0) {
                        // goto
                        // LAB_1038_244e;
                    }
                    pass1_1038_6838(
                        (puVar7 & 0xffff | uStack24 << 0x10),
                        u_var6,
                        iVar12.field_0x8,
                        0x1,
                        iVar11.field_0x4,
                    );
                    //           TODO: goto LAB_1038_24b3;
                }
            } else {
                mem_op_1000_179c(0x2a, uStack24, 0x1000);
                u_var10 = uStack24 | puVar7;
                uVar14 = u_var10;
                if (u_var10 == 0x0) {
                    //LAB_1038_244e:
                    uVar13 = 0x1000;
                    uStack42 = 0x0;
                    uStack24 = uVar14;
                } else {
                    pass1_1038_675c(
                        (puVar7 & 0xffff | uStack24 << 0x10),
                        u_var6,
                        iVar12.field_0x6,
                        0x1,
                        iVar11.field_0x4,
                    );
                    //LAB_1038_24b3:
                    uVar13 = 0x1000;
                    uStack42 = puVar7 & 0xffff | uVar14 << 0x10;
                    uStack24 = uVar14;
                }
            }
            if (uStack42 != 0x0) {
                pass1_1038_7a5a(ctx.PTR_LOOP_1050_5a64);
                iStack8 += -0x1;
                if (iStack8 == 0x0) {
                    break;
                }
                uStack12 = 0x64;
            }
        }
    }
    pass1_1030_6c4c(param_3, iStack8);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_24e8(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let u_var1: u16;
    let u_var2: u32;
    let pu_var3: U32Ptr;
    let puVar4: U32Ptr;
    let iVar5: i16;
    let u_var6: u16;
    let uStack30: u16;
    let paStack28: &mut Struct18;
    let local_18: u32;
    let local_14: u16;
    let uStack18: u16;
    let uStack16: u32;
    let puStack12: u32;
    let iStack8: i16;
    let uStack6: u32;

    uStack6 = struct_op_1030_73a8(param_3);
    // puVar4 = (uStack6 >> 0x10);
    // u_var6 = (param_3 >> 0x10);
    iVar5 = param_3;
    iStack8 = (iVar5 + 0x34);
    puStack12 = (uStack6 + 0x28);
    uStack16 = 0x64;
    uStack18 = (puStack12 + 0x4);
    u_var2 = uStack18;
    mem_op_1000_179c(0xa, puVar4, 0x1000);
    u_var1 = u_var2;
    pu_var3 = (puVar4 | u_var1);
    if (pu_var3 == 0x0) {
        u_var1 = 0x0;
        pu_var3 = 0x0;
    } else {
        pass1_1020_ba3e(
            (u_var2 & 0xffff | ZEXT24(puVar4) << 0x10),
            0x5,
            0x5,
            param_5,
            param_4,
        );
    }
    paStack28 = CONCAT22(pu_var3, u_var1);
    // for (uStack30 = 0x0; u_var2 = uStack18, uStack30 < uStack18; uStack30 += 0x1) {
    //   pass1_1020_bb16(puStack12,CONCAT22(param_6,&local_18),
    //                   CONCAT22(param_6,&local_14),uStack30);
    //   if (local_18 != 0x0) {
    //     u_var2 = local_18;
    //     uStack16._2_2_ = local_18._2_2_;
    //     if (uStack16 < local_18) {
    //       u_var2 = uStack16 & 0xffff;
    //     }
    //     u_var1 = u_var2;
    //     u_var2 = u_var2 & 0xffff | ZEXT24(uStack16._2_2_) << 0x10;
    //     local_18 = CONCAT22(local_18._2_2_ +
    //                         (-(local_18 < u_var1) - uStack16._2_2_),
    //                         local_18 - u_var1);
    //     pu_var3 = uStack16._2_2_;
    //     pass1_1020_bb8a(puStack12,local_18 - u_var1,
    //                     CONCAT22(local_14,local_18._2_2_ +
    //                                       (-(local_18 < u_var1) -
    //                                       uStack16._2_2_)),param_5,param_6);
    //     pass1_1020_bb70(paStack28,u_var1,CONCAT22(local_14,uStack16._2_2_),param_5,
    //                     param_4,param_6);
    //     uStack16 -= u_var2;
    //     if (uStack16 == 0x0) {
    //       mem_op_1000_179c(0x2a,pu_var3,0x1000);
    //       puVar4 = (pu_var3 | u_var2);
    //       if (puVar4 == 0x0) {
    //         u_var2 = 0x0;
    //       }
    //       else {
    //         pass1_1038_666e((u_var2 & 0xffff | ZEXT24(pu_var3) << 0x10),
    //                         paStack28,0x1,(iVar5 + 0x4));
    //       }
    //       pass1_1038_7a5a(ctx.PTR_LOOP_1050_5a64);
    //       mem_op_1000_179c(0xa,puVar4,0x1000);
    //       pu_var3 = (puVar4 | u_var2);
    //       if (pu_var3 == 0x0) {
    //         u_var2 = 0x0;
    //         pu_var3 = 0x0;
    //       }
    //       else {
    //         pass1_1020_ba3e((u_var2 & 0xffff | ZEXT24(puVar4) << 0x10),0x5,0x5,
    //                         param_5,param_4);
    //       }
    //       paStack28 = (u_var2 & 0xffff | ZEXT24(pu_var3) << 0x10);
    //       iStack8 += -0x1;
    //       if (iStack8 == 0x0) break;
    //       uStack16 = 0x64;
    //     }
    //   }
    // }
    pass1_1020_ba94(paStack28);
    pu_var3 = (pu_var3 | u_var2);
    if (pu_var3 == 0x0) {
        if (paStack28 != 0x0) {
            fn_ptr_1020_ba7e(paStack28);
            fn_ptr_1000_17ce(ctx, paStack28, 0x1000);
        }
    } else {
        mem_op_1000_179c(0x2a, pu_var3, 0x1000);
        if ((pu_var3 | u_var2) != 0x0) {
            pass1_1038_666e(
                (u_var2 & 0xffff | ZEXT24(pu_var3) << 0x10),
                paStack28,
                0x1,
                (iVar5 + 0x4),
            );
        }
        pass1_1038_7a5a(ctx.PTR_LOOP_1050_5a64);
    }
    pass1_1030_6c4c(param_3, iStack8);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_26ee(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u32;
    let puVar4: U32Ptr;
    let pu_var5: U32Ptr;
    let puVar6: U32Ptr;
    let iVar7: i16;
    let uVar8: u16;
    let uVar9: u32;
    let u_var10: u32;
    let uStack36: u32;
    let uStack20: u16;
    let puStack18: U32Ptr;
    let uStack16: u32;
    let uStack12: u16;
    let uStack10: u16;
    let iStack8: i16;

    uVar9 = struct_op_1030_73a8(param_3);
    // puVar6 = (uVar9 >> 0x10);
    // uVar8 = (param_3 >> 0x10);
    iVar7 = param_3;
    iStack8 = (iVar7 + 0x34);
    uStack12 = pass1_1028_0d80(uVar9);
    u_var3 = uStack12;
    uStack16 = 0x64;
    mem_op_1000_179c(0xa, puVar6, 0x1000);
    puVar4 = (puVar6 | u_var3);
    if (puVar4 == 0x0) {
        u_var3 = 0x0;
        puVar4 = 0x0;
    } else {
        pass1_1020_ba3e(
            (u_var3 & 0xffff | ZEXT24(puVar6) << 0x10),
            0x5,
            0x5,
            param_5,
            param_4,
        );
    }
    uStack20 = u_var3;
    uStack10 = uStack12;
    puStack18 = puVar4;
    loop {
        u_var10 = pass1_1030_7c28(param_3, uStack10, u_var3, puVar4, param_6);
        // puVar6 = (u_var10 >> 0x10);
        u_var1 = u_var10;
        puVar4 = (puVar6 | u_var1);
        if (puVar4 != 0x0) {
            pu_var5 = puVar6;
            u_var2 = u_var1;
            if ((uStack16._2_2_ <= puVar6) && (uStack16._2_2_ < puVar6 || (uStack16 < u_var1))) {
                pu_var5 = uStack16._2_2_;
                u_var2 = uStack16;
            }
            uStack36 = CONCAT22(pu_var5, u_var2);
            puVar4 = pu_var5;
            pass1_1030_7d1c(
                param_3,
                u_var1 - u_var2,
                CONCAT22(uStack10, puVar6 + (-(u_var1 < u_var2) - pu_var5)),
                u_var2,
                pu_var5,
                param_4,
                param_5,
                param_6,
            );
            pass1_1020_bb70(
                CONCAT22(puStack18, uStack20),
                u_var2,
                CONCAT22(uStack10, pu_var5),
                param_5,
                param_4,
                param_6,
            );
            uStack16 -= uStack36;
            if (uStack16 == 0x0) {
                mem_op_1000_179c(0x2a, puVar4, 0x1000);
                uStack10 = uStack36;
                puVar6 = (puVar4 | uStack10);
                if (puVar6 == 0x0) {
                    uStack10 = 0x0;
                } else {
                    pass1_1038_666e(
                        (uStack36 & 0xffff | ZEXT24(puVar4) << 0x10),
                        CONCAT22(puStack18, uStack20),
                        0x1,
                        (iVar7 + 0x4),
                    );
                }
                pass1_1038_7a5a(ctx.PTR_LOOP_1050_5a64);
                mem_op_1000_179c(0xa, puVar6, 0x1000);
                puVar4 = (puVar6 | uStack10);
                if (puVar4 == 0x0) {
                    uStack10 = 0x0;
                    puVar4 = 0x0;
                } else {
                    pass1_1020_ba3e(CONCAT22(puVar6, uStack10), 0x5, 0x5, param_5, param_4);
                }
                iStack8 += -0x1;
                uStack20 = uStack10;
                puStack18 = puVar4;
                if (iStack8 == 0x0) {
                    break;
                }
                uStack16 = 0x64;
            }
        }
        uStack10 = pass1_1028_0d80(uVar9);
        u_var3 = uStack10;
        if (uStack12 == 0x0) {
            uStack12 = uStack10;
        }
        if (uStack12 != uStack10) == false {
            break;
        }
    }
    pass1_1020_ba94(CONCAT22(puStack18, uStack20));
    puVar4 = (puVar4 | uStack10);
    if (puVar4 == 0x0) {
        if ((puStack18 | uStack20) != 0x0) {
            fn_ptr_1020_ba7e(CONCAT22(puStack18, uStack20));
            fn_ptr_1000_17ce(ctx, CONCAT22(puStack18, uStack20), 0x1000);
        }
    } else {
        mem_op_1000_179c(0x2a, puVar4, 0x1000);
        if ((puVar4 | uStack10) != 0x0) {
            pass1_1038_666e(
                CONCAT22(puVar4, uStack10),
                CONCAT22(puStack18, uStack20),
                0x1,
                (iVar7 + 0x4),
            );
        }
        pass1_1038_7a5a(ctx.PTR_LOOP_1050_5a64);
    }
    pass1_1030_6c4c(param_3, iStack8);
    return;
}

pub fn pass1_1038_28d8(param_1: &mut Struct100, param_2: u16, param_3: u8) -> Struct100 {
    struct_op_1028_d1dc(param_2, param_3, param_1, 0x3a97);
    param_1.field_0x0 = 0x29fe;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        ctx.s_SCRoboMove_1050_59f8,
    );
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_290e(param_1: u16, param_2: u16) -> u16 {
    let unaff_SI: u16;
    let unaff_DI: u16;
    let unaff_SS: u16;
    let in_AF: u8;

    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, 0x1);
    if ((param_2 | param_1) != 0x0) {
        pass1_1038_4918(
            CONCAT22(param_2, param_1),
            param_1,
            param_2 | param_1,
            unaff_SS,
            in_AF,
        );
    }
    pass1_1038_7a76(ctx.PTR_LOOP_1050_5a64, unaff_SI, unaff_DI, unaff_SS);
    return 0x1;
}

pub fn pass1_1038_2944(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let pu_var3: u32;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x108, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        (param_2 + 0x2) = 0x1008;
        // u_var6 = (param_1 >> 0x10);
        (param_2 + 0x4) = (param_1 + 0x4);
        pu_var3 = (param_1 + 0x8);
        pu_var5 = (param_2 + 0x8);
        // for (i_var4 = 0x40; i_var4 != 0x0; i_var4 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = pu_var3;
        //   pu_var3 = pu_var3 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        (param_2 + 0x2) = &USHORT_1050_1028;
        *puStack10 = 0x29fe;
        (param_2 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    }
    return;
}

pub fn pass1_1038_29d2(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    param_1.field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_2a0e(
    param_1: &mut Struct100,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u16,
    param_7: u8,
) {
    let i_var1: i16;
    let u_var2: u16;

    struct_op_1028_d1dc(param_6, param_7, param_1, 0x2af7);
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x108) = param_5;
    (i_var1 + 0x10c) = param_4;
    (i_var1 + 0x110) = param_3;
    (i_var1 + 0x114) = param_2;
    param_1.field_0x0 = 0x309a;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return;
}

pub fn pass1_1038_2a5c(param_1: U32Ptr) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x309a;
    (i_var4 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pu_var1 = (i_var4 + 0x114);
    u_var2 = (i_var4 + 0x116);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pu_var1 = (i_var4 + 0x110);
    u_var2 = (i_var4 + 0x112);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    *param_1 = 0x389a;
    (i_var4 + 0x2) = 0x1008;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_2ac2(
    param_1: u32,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u8,
) -> u16 {
    let u_var1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let uStack10: u32;
    let uStack6: u32;

    // u_var3 = (param_1 >> 0x10);
    u_var2 = param_1;
    u_var1 = (u_var2 + 0x108);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    uStack6 = CONCAT22(param_3, param_2);
    u_var1 = (u_var2 + 0x10c);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    uStack10 = CONCAT22(param_3, param_2);
    pass1_1038_2c82(
        u_var2,
        u_var3,
        (u_var2 + 0x110),
        CONCAT22(param_3, param_2),
        uStack6,
        param_4,
        param_5,
        &USHORT_1050_1028,
        param_6,
        param_7,
    );
    pass1_1038_2c82(
        u_var2,
        u_var3,
        (u_var2 + 0x114),
        uStack6,
        uStack10,
        param_4,
        param_5,
        &USHORT_1050_1028,
        param_6,
        param_7,
    );
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_2b2e(param_1: u32, param_2: u16, param_3: u16) -> u16 {
    let u_var1: u32;
    let u_var2: u16;
    let unaff_SI: u16;
    let unaff_DI: u16;
    let u_var3: u16;
    let unaff_SS: u16;
    let uStack6: u32;

    // u_var3 = (param_1 >> 0x10);
    u_var2 = param_1;
    u_var1 = (u_var2 + 0x108);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    uStack6 = CONCAT22(param_3, param_2);
    u_var1 = (u_var2 + 0x10c);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    pass1_1038_2f92(
        u_var2,
        u_var3,
        (u_var2 + 0x110),
        CONCAT22(param_3, param_2),
        unaff_SI,
        unaff_DI,
        unaff_SS,
    );
    pass1_1038_2f92(
        u_var2,
        u_var3,
        (u_var2 + 0x114),
        uStack6,
        unaff_SI,
        unaff_DI,
        unaff_SS,
    );
    return 0x1;
}

pub fn pass1_1038_2b9a(param_1: u32, param_2: &mut Struct422, param_3: U32Ptr) {
    let pu_var1: u32;
    let pu_var2: u32;
    let i_var3: i16;
    let iVar5: &mut Struct421;
    let puVar4: u32;
    let pu_var5: u32;
    let u_var6: u16;
    let puStack10: U32Ptr;

    mem_op_1000_179c(0x118, param_3, 0x1000);
    puStack10 = CONCAT22(param_3, param_2);
    iVar5 = param_1;
    // u_var6 = (param_1 >> 0x10);
    if ((param_3 | param_2) != 0x0) {
        *puStack10 = 0x389a;
        param_2.field_0x2 = 0x1008;
        param_2.field_0x4 = iVar5.field_0x4;
        puVar4 = &iVar5.field_0x8;
        pu_var5 = &param_2.field_0x8;
        // for (i_var3 = 0x40; i_var3 != 0x0; i_var3 += -0x1) {
        //   pu_var2 = pu_var5;
        //   pu_var5 = pu_var5 + 0x1;
        //   pu_var1 = puVar4;
        //   puVar4 = puVar4 + 0x1;
        //   *pu_var2 = *pu_var1;
        // }
        *puStack10 = 0x6ad2;
        param_2.field_0x2 = &USHORT_1050_1028;
        param_2.field_0x108 = iVar5.field_0x108;
        param_2.field_0x10c = iVar5.field_0x10c;
        param_2.field_0x110 = iVar5.field_0x110;
        param_2.field_0x114 = iVar5.field_0x114;
        *puStack10 = 0x309a;
        param_2.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    }
    iVar5.field_0x114 = 0x0;
    iVar5.field_0x110 = 0x0;
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_2c82(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u16,
    param_7: u16,
    param_8: u16,
    param_9: u16,
    param_10: u8,
) {
    let pu_var1: U32Ptr;
    let piVar2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;
    let lVar6: i32;
    let ppcVar7: u32;
    let uVar8: u16;
    let puVar9: u32;
    let iVar10: i16;
    let u_var11: u32;
    let puVar12: U32Ptr;
    let puVar13: U32Ptr;
    let uVar14: u16;
    let iVar16: i16;
    let iVar15: &mut Struct702;
    let uVar17: u16;
    let uVar18: u16;
    let puVar19: U32Ptr;
    let pu_var20: U32Ptr;
    let uVar21: u8;
    let uStack22: u32;
    let local_12: u32;
    let puStack14: U32Ptr;
    let uStack10: u32;
    let uStack6: u32;

    // uVar17 = (param_5 >> 0x10);
    uVar14 = param_5;
    uStack6 = (uVar14 + 0x200);
    // uVar18 = (param_4 >> 0x10);
    iVar16 = param_4;
    uStack10 = (iVar16 + 0x200);
    puVar13 = (iVar16 + 0x202);
    // puVar19 = (param_3 >> 0x10);
    iVar15 = param_3;
    iVar10 = iVar15.field_0xc;
    if (iVar10 == 0x1) {
        puStack14 = param_3;
        pass1_1038_52b8(
            param_5,
            &iVar15.field_0x8,
            &iVar15.field_0xe,
            param_6,
            param_7,
            param_8,
            param_9,
        );
        return;
    }
    if (iVar10 == 0x2) {
        puStack14 = param_3;
        if (iVar15.field_0xe != 0x0) {
            pass1_1038_3efc(uVar14, uVar17, param_4, iVar15.field_0xe, iVar15, puVar19);
            return;
        }
        pass1_1020_a43e(param_9, puVar19, CONCAT22(param_9, &local_12));
        uStack22 = (puStack14 + 0x8);
        loop {
            uStack22 += -0x1;
            if ((uStack22._2_2_ | uStack22) == 0x0) {
                break;
            }
            pass1_1020_a6ee(
                CONCAT13((param_9 >> 0x8), CONCAT12(param_9, &local_12)),
                (puStack14 + 0x12),
                &local_12,
                uStack22._2_2_ | uStack22,
                param_7,
                param_9,
                param_10,
            );
        }
    } else {
        if (iVar10 == 0x3) {
            pass1_1038_3f38(param_5, param_4, iVar15.field_0xe, 0x0, puVar13);
            return;
        }
        uStack6._2_2_ = (uStack6 >> 0x10);
        if (iVar10 == 0x4) {
            ctx.PTR_LOOP_1050_5f2e = (uStack6._2_2_ & 0xff);
            if ((uStack6 == 0x1) && ((uStack6 & 0xff0000) == 0x0)) {
                local_12 = (uVar14 + 0x1f6);
                pass1_1030_3694(
                    local_12,
                    &iVar15.field_0xe,
                    &iVar15.field_0x8,
                    0x0,
                    0x1030,
                    param_9,
                );
                (&iVar15.field_0xe + 0x2) = local_12;
                iVar15.field_0x12 = ctx.PTR_LOOP_1050_5f2e;
            } else {
                if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
                    ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e, 0x1000);
                } else {
                }
                uVar14 = fn_ptr_op_1000_1708(
                    0x16c,
                    0x0,
                    0x1,
                    ctx.PTR_LOOP_1050_5f2c,
                    ctx.PTR_LOOP_1050_5f2e,
                    0x1000,
                );
                (&iVar15.field_0xe + 0x2) = uVar14;
                iVar15.field_0x12 = ctx.PTR_LOOP_1050_5f2e;
                iVar10 = &iVar15.field_0xe;
                if (iVar10 != 0x3) {
                    if (iVar10 != 0x4) {
                        u_var5 = (&iVar15.field_0xe + 0x2);
                        (u_var5 + 0x28) = &iVar15.field_0x8;
                        return;
                    }
                    u_var5 = (&iVar15.field_0xe + 0x2);
                    (u_var5 + 0xdc) = &iVar15.field_0x8;
                    return;
                }
                u_var5 = (&iVar15.field_0xe + 0x2);
                (u_var5 + 0x64) = &iVar15.field_0x8;
            }
        } else {
            if (iVar10 == 0x5) {
                if (&iVar15.field_0xe == 0xc) {
                    if ((uStack6 == 0x1) && ((uStack6 & 0xff0000) == 0x0)) {
                        u_var5 = (uVar14 + 0x1f6);
                        u_var3 = iVar15.field_0x8;
                        iVar10 = iVar15.field_0xa;
                        uVar8 = -u_var3;
                        // uVar18 = (u_var5 >> 0x10);
                        iVar16 = u_var5;
                        pu_var1 = (iVar16 + 0x170);
                        u_var4 = *pu_var1;
                        *pu_var1 = *pu_var1 + uVar8;
                        piVar2 = (iVar16 + 0x172);
                        *piVar2 = (*piVar2 - (iVar10 + (u_var3 != 0x0))) + CARRY2(u_var4, uVar8);
                    }
                } else {
                    pass1_1038_43cc(
                        uVar14,
                        uVar17,
                        iVar15.field_0x8,
                        &iVar15.field_0xe,
                        iVar15,
                        puVar19,
                    );
                }
            } else {
                iVar10 += -0x7;
                if (iVar10 == 0x0) {
                    lVar6 = iVar15.field_0xe;
                    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, lVar6);
                    puVar12 = puVar13;
                    pass1_1038_349e(CONCAT22(puVar13, iVar10), (iVar16 + 0x200));
                    uVar21 = (puVar13 >> 0x8);
                    pass1_1038_4d0e(CONCAT13(uVar21, CONCAT12(puVar13, iVar10)), 0x258);
                    pass1_1038_4d0e(CONCAT13(uVar21, CONCAT12(puVar13, iVar10)), 0x258);
                    pu_var20 =
                        mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3b, param_9, puVar12, param_7);
                    // puVar13 = (pu_var20 >> 0x10);
                    pass1_1008_de58(pu_var20, iVar15.field_0xe, 0x4000001, param_9);
                    pu_var20 =
                        mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_9, puVar13, param_7);
                    // puVar13 = (pu_var20 >> 0x10);
                    u_var11 = (pu_var20 + 0x20);
                    pass1_1030_8344(
                        ctx.PTR_LOOP_1050_5748,
                        (ctx.PTR_LOOP_1050_5748 >> 0x10),
                        u_var11,
                    );
                    local_12 = u_var11 & 0xffff | ZEXT24(puVar13) << 0x10;
                    uVar14 = pass1_1030_5b00(u_var11 & 0xffff | ZEXT24(puVar13) << 0x10);
                    puStack14 =
                        mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, uVar14, param_9, puVar13, param_7);
                    puVar9 = (puStack14 + 0x20);
                    ppcVar7 = (*puVar9 + 0x4);
                    (**ppcVar7)(0x1010, puVar9, (puStack14 >> 0x10), 0x6);
                }
            }
        }
    }
    return;
}

pub fn pass1_1038_2f92(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: u16,
    param_6: u16,
    param_7: u16,
) {
    let pu_var1: U32Ptr;
    let piVar2: U32Ptr;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let uVar7: u32;
    let i_var8: i16;
    let i_var9: i16;
    let u_var10: u16;
    let u_var11: u16;
    let uStack10: i16;

    // u_var10 = (param_4 >> 0x10);
    i_var8 = param_4;
    u_var6 = (i_var8 + 0x200);
    // u_var11 = (param_3 >> 0x10);
    i_var9 = param_3;
    i_var3 = (i_var9 + 0xc);
    if (i_var3 == 0x1) {
        uVar7 = (i_var9 + 0x8);
        pass1_1038_3cc0(
            param_4,
            uVar7,
            (uVar7 >> 0x10),
            (i_var9 + 0xe),
            param_5,
            param_6,
            param_7,
        );
        return;
    }
    if (i_var3 == 0x4) {
        pass1_1030_355c((i_var8 + 0x1f6), (i_var9 + 0x10));
        return;
    }
    if (i_var3 == 0x5) {
        if ((i_var9 + 0xe) != 0xc) {
            pass1_1038_5798(param_4, (i_var9 + 0x8), (i_var9 + 0xe));
            return;
        }
        iStack10 = u_var6;
        if ((iStack10 == 0x1) && ((u_var6 & 0xff0000) == 0x0)) {
            uVar7 = (i_var8 + 0x1f6);
            u_var4 = (i_var9 + 0x8);
            i_var3 = (i_var9 + 0xa);
            // u_var10 = (uVar7 >> 0x10);
            i_var8 = uVar7;
            pu_var1 = (i_var8 + 0x170);
            u_var5 = *pu_var1;
            *pu_var1 = *pu_var1 + u_var4;
            piVar2 = (i_var8 + 0x172);
            *piVar2 = *piVar2 + i_var3 + CARRY2(u_var5, u_var4);
            return;
        }
    }
    return;
}

pub fn pass1_1038_3074(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_2a5c(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_30aa(param_1: U32Ptr, param_2: u16) {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let iVar5: &mut Struct423;
    let u_var5: u16;
    let puVar6: U32Ptr;

    puVar6 = struct_1030_17ce(param_1, 0x0, 0x0);
    // pu_var2 = (puVar6 >> 0x10);
    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    iVar5.field_0x10 = 0x0;
    iVar5.field_0x14 = 0x0;
    iVar5.field_0x18 = 0x258;
    iVar5.field_0x1a = 0x258;
    iVar5.field_0x1c = 0x0;
    iVar5.field_0x1e = 0x0;
    iVar5.field_0x22 = 0x0;
    iVar5.field_0x24 = 0x32;
    &iVar5.field_0x1f6 = 0x0;
    iVar5.field_0x1fa = 0x0;
    iVar5.field_0x1fe = 0x0;
    iVar5.field_0x200 = 0x8000001;
    iVar5.field_0x204 = 0x0;
    iVar5.field_0x206 = 0x0;
    iVar5.field_0x208 = 0x1;
    iVar5.field_0x20a = 0x0;
    iVar5.field_0x20c = 0x0;
    iVar5.field_0x20e = 0x0;
    iVar5.field_0x210 = 0x0;
    iVar5.field_0x214 = 0x0;
    iVar5.field_0x216 = 0x0;
    iVar5.field_0x21a = 0x0;
    *param_1 = 0x6504;
    iVar5.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar5.field_0x26), 0x0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar5.field_0xba), 0x0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar5.field_0x14e), 0x0, 0x54);
    pu_var1 = pass1_1000_4906((param_1 & 0xffff0000 | &iVar5.field_0x1a2), 0x0, 0x54);
    mem_op_1000_179c(0x1b0, pu_var2, 0x1000);
    pu_var3 = (pu_var2 | pu_var1);
    if (pu_var3 == 0x0) {
        &iVar5.field_0x1f6 = 0x0;
    } else {
        pass1_1030_314c(
            CONCAT22(pu_var2, pu_var1),
            iVar5.field_0x4,
            pu_var3,
            param_2,
        );
        iVar5.field_0x1f6 = pu_var1;
        iVar5.field_0x1f8 = pu_var3;
    }
    mem_op_1000_179c(0x1e, pu_var3, 0x1000);
    u_var4 = pu_var3 | pu_var1;
    if (u_var4 == 0x0) {
        pu_var1 = 0x0;
        u_var4 = 0x0;
    } else {
        struct_1020_c444(CONCAT22(pu_var3, pu_var1), 0x64, 0xc8);
    }
    iVar5.field_0xc = pu_var1;
    iVar5.field_0xe = u_var4;
    return;
}

pub fn pass1_1038_3222(
    param_1: U32Ptr,
    param_2: u32,
    param_3: u32,
    param_4: u16,
    param_5: U32Ptr,
    param_6: u8,
    param_7: U32Ptr,
) {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let iVar5: &mut Struct363;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let ulocal_16: [u8; 0x14];

    puVar6 = pass1_1030_183c(param_1, 0x0, 0x0, 0x4000000, param_3, param_4, param_5);
    // pu_var2 = (puVar6 >> 0x10);
    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    iVar5.field_0x10 = param_2;
    iVar5.field_0x14 = 0x0;
    iVar5.field_0x18 = 0x258;
    iVar5.field_0x1a = 0x258;
    iVar5.field_0x1c = 0x0;
    iVar5.field_0x1e = 0x0;
    iVar5.field_0x22 = 0x0;
    iVar5.field_0x24 = 0x32;
    &iVar5.field_0x1f6 = 0x0;
    &iVar5.field_0x1fa = 0x0;
    iVar5.field_0x1fe = 0x0;
    iVar5.field_0x200 = 0x8000001;
    iVar5.field_0x204 = 0x0;
    iVar5.field_0x206 = 0x0;
    iVar5.field_0x208 = 0x1;
    iVar5.field_0x20a = 0x0;
    iVar5.field_0x20c = 0x0;
    iVar5.field_0x20e = 0x0;
    iVar5.field_0x210 = 0x0;
    iVar5.field_0x214 = 0x0;
    iVar5.field_0x216 = 0x0;
    iVar5.field_0x21a = 0x0;
    *param_1 = 0x6504;
    iVar5.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar5.field_0x26), 0x0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar5.field_0xba), 0x0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &iVar5.field_0x14e), 0x0, 0x54);
    pu_var1 = pass1_1000_4906((param_1 & 0xffff0000 | &iVar5.field_0x1a2), 0x0, 0x54);
    mem_op_1000_179c(0x1b0, pu_var2, 0x1000);
    u_var3 = pu_var2 | pu_var1;
    if (u_var3 == 0x0) {
        &iVar5.field_0x1f6 = 0x0;
    } else {
        pass1_1030_314c(
            CONCAT22(pu_var2, pu_var1),
            &iVar5.field_0x4,
            u_var3,
            param_7,
        );
        iVar5.field_0x1f6 = pu_var1;
        iVar5.field_0x1f8 = u_var3;
    }
    pu_var2 = (iVar5.field_0x6 & 0xff);
    sys_1000_3f9c(
        local_16,
        param_7,
        0x5a1a,
        ctx.data_seg,
        &iVar5.field_0x4,
        &stack0xfffe,
        u_var5,
        0x1000,
        param_7,
        param_6,
    );
    u_var3 = str_op_1008_60e8(CONCAT22(param_7, local_16), pu_var2);
    iVar5.field_0x1fa = u_var3;
    iVar5.field_0x1fc = pu_var2;
    mem_op_1000_179c(0x1e, pu_var2, 0x1000);
    u_var4 = pu_var2 | u_var3;
    if (u_var4 == 0x0) {
        &iVar5.field_0xc = 0x0;
    } else {
        struct_1020_c444(CONCAT22(pu_var2, u_var3), 0x64, 0xc8);
        iVar5.field_0xc = u_var3;
        iVar5.field_0xe = u_var4;
    }
    return;
}

pub fn pass1_1038_33f8(param_1: U32Ptr) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x6504;
    (i_var4 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pu_var1 = (i_var4 + 0x14);
    u_var2 = (i_var4 + 0x16);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pu_var1 = (i_var4 + 0x1f6);
    u_var2 = (i_var4 + 0x1f8);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    fn_ptr_1000_17ce(ctx, (i_var4 + 0x1fa), 0x1000);
    pu_var1 = (i_var4 + 0x210);
    u_var2 = (i_var4 + 0x212);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(0x1000, pu_var1, u_var2, 0x1);
    }
    fn_ptr_1000_17ce(ctx, (i_var4 + 0x21a), 0x1000);
    pass1_1030_18b2(param_1);
    return;
}

pub fn pass1_1038_349e(param_1: u32, param_2: u32) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let extraout_dx: u16;
    let u_var4: u16;
    let u_var5: u16;
    let extraout_DX_00: u16;
    let iVar7: &mut Struct685;
    let u_var6: u16;
    let puVar7: u32;
    let uVar8: u16;
    let uVar9: u16;
    let uStack10: u32;
    let uStack6: u32;

    // u_var6 = (param_1 >> 0x10);
    iVar7 = param_1;
    iVar7.field_0x200 = param_2;
    pass1_1038_4d0e(param_1, 0x258);
    u_var3 = param_2;
    pass1_1038_4d0e(param_1, 0x258);
    iVar7.field_0x204 = 0x0;
    iVar7.field_0x206 = 0x0;
    puVar7 = iVar7.field_0xc;
    uVar8 = SUB42(puVar7, 0x0);
    // uVar9 = (puVar7 >> 0x10);
    ppcVar1 = (*iVar7.field_0xc + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_dx, u_var3);
    u_var5 = extraout_dx;
    // for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
    //   puVar7 = pass1_1030_1d7c(u_var3,u_var5,iVar7.field_0xc);
    //   u_var4 = (puVar7 >> 0x10);
    //   u_var2 = puVar7;
    //   u_var5 = u_var4 | u_var2;
    //   if (u_var5 != 0x0) {
    //     ppcVar1 = (*puVar7 + 0x58);
    //     (**ppcVar1)(0x1030,u_var2,u_var4,param_1,u_var6,uVar8,uVar9);
    //     (u_var2 + 0x1c) = 0x0;
    //     u_var5 = extraout_DX_00;
    //   }
    // }
    return;
}

pub fn pass1_1038_354a(param_1: u32, param_2: u16, param_3: U32Ptr) {
    let u_var1: u16;
    let i_var1: &mut Struct424;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if (&i_var1.field_0x21a == 0x0) {
        mem_op_1000_179c(0xa, param_3, 0x1000);
        u_var1 = param_3 | param_2;
        if (u_var1 == 0x0) {
            &i_var1.field_0x21a = 0x0;
        } else {
            pass1_1030_9ecc(CONCAT22(param_3, param_2), param_1);
            &i_var1.field_0x21a = param_2;
            i_var1.field_0x21c = u_var1;
        }
    }
    pass1_1030_9ef2(&i_var1.field_0x21a);
    return;
}

pub fn pass1_1038_35a8(param_1: u32, param_2: u16, param_3: u16, param_4: U32Ptr) {
    let u_var1: u16;
    let i_var3: &mut Struct425;
    let u_var2: u16;
    let unaff_SS: u16;
    let in_AF: u8;

    // u_var2 = (param_1 >> 0x10);
    i_var3 = param_1;
    if (&i_var3.field_0x21a == 0x0) {
        mem_op_1000_179c(0xa, param_4, 0x1000);
        u_var1 = param_4 | param_3;
        if (u_var1 == 0x0) {
            &i_var3.field_0x21a = 0x0;
        } else {
            pass1_1030_9ecc(CONCAT22(param_4, param_3), param_1);
            &i_var3.field_0x21a = param_3;
            i_var3.field_0x21c = u_var1;
        }
    }
    pass1_1030_9f40(&i_var3.field_0x21a, param_2, unaff_SS, in_AF);
    return;
}

pub fn pass1_1038_3608(param_1: u32) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    fn_ptr_1000_17ce(ctx, (param_1 + 0x21a), 0x1000);
    (param_1 + 0x21a) = 0x0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_362e(param_1: u32) {
    let in_AX: u16;
    let in_DX: U32Ptr;
    let i_var1: i16;
    let unaff_DI: i16;
    let u_var2: u16;
    let unaff_SS: u16;
    let paVar3: &mut Struct67;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x214) == 0x0) {
        pass1_1038_4f54(param_1 & 0xffff | u_var2 << 0x10, 0x1f, in_AX);
        if (in_AX == 0x0) {
            (i_var1 + 0x214) = 0x14;
        } else {
            (i_var1 + 0x214) = 0x28;
        }
        paVar3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37, unaff_SS, in_DX, unaff_DI);
        post_win_msg_1008_a0e4(
            paVar3,
            0x0,
            0x0,
            0x1,
            (i_var1 + 0x4),
            0x38,
            0x1008,
            unaff_SS,
        );
        (i_var1 + 0x216) = 0x0;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_3698(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let pi_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let u_var3: u32;
    let ppc_var4: u32;
    let u_var5: u16;
    let BVar6: bool;
    let uVar7: u16;
    let uVar8: u16;
    let lVar9: i32;
    let u_var10: u32;
    let u_var11: u16;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u32;
    let iVar15: i16;
    let uVar16: u16;
    let uVar17: u32;
    let uStack32: u32;
    let uStack18: u32;
    let uStack14: u32;
    let uStack10: u32;
    let uStack6: u32;

    // uVar16 = (param_1 >> 0x10);
    iVar15 = param_1;
    if ((iVar15 + 0x214) == 0x0) {
        return;
    }
    pass1_1030_38b8();
    uStack6 = CONCAT22(param_3, param_2);
    uStack6 -= (iVar15 + 0x216);
    if (0x0 < uStack6) {
        uStack6 += 0x3;
        uStack10 = uStack6 / 0x5;
        uVar14 = uStack6 % 0x5;
        if ((iVar15 + 0xc) == 0x0) {
            u_var5 = 0x0;
            uVar14 = 0x0;
        } else {
            u_var3 = (iVar15 + 0xc);
            ppc_var4 = ((iVar15 + 0xc) + 0x10);
            lVar9 = uStack10;
            (**ppc_var4)(0x1030, u_var3, (u_var3 >> 0x10));
            u_var5 = lVar9;
        }
        uStack14 = CONCAT22(uVar14, u_var5);
        // z
        u_var5 = u_var10;
        pass1_1030_38b8();
        uStack6 = CONCAT22(uVar12, u_var5);
        uStack6 -= (iVar15 + 0x216);
        uStack6._2_2_ = (uStack6 >> 0x10);
        if ((uStack6._2_2_ | uStack6) != 0x0) {
            uStack32 = uStack6 / (iVar15 + 0x214);
            if (uStack32 < 0x1) {
                uStack32 = 0x1;
            }
            pass1_1030_375a((iVar15 + 0x1f6), 0x0, uStack32, param_4);
        }
    }
    pi_var1 = (iVar15 + 0x214);
    *pi_var1 = *pi_var1 + -0x1;
    return;
}

pub fn pass1_1038_387e(param_1: u32, param_2: i16, param_3: i16, param_4: u32, param_5: u16) {
    let ppcVar1: u32;
    let lVar2: i32;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u32;
    let u_var6: u32;
    let uVar7: u32;
    let extraout_dx: U32Ptr;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let u_var10: u16;
    let extraout_DX_00: u16;
    let u_var11: u16;
    let iVar10: &mut Struct302;
    let uVar12: u16;
    let iStack22: i16;
    let uStack12: u16;
    let uStack10: u32;
    let uStack6: u32;

    if (param_2 != param_3) {
        iVar10 = param_1;
        // uVar12 = (param_1 >> 0x10);
        if (param_2 < param_3) {
            uStack12 = param_3 - param_2;
            if ((iVar10.field_0x210 == 0x0) || (lVar2 = iVar10.field_0x210, (lVar2 + 0xa) == 0x0)) {
                if (iVar10.field_0xc == 0x0) {
                    u_var11 = 0x0;
                    puVar8 = 0x0;
                } else {
                    ppcVar1 = (*iVar10.field_0xc + 0x10);
                    u_var11 = uStack12;
                    (**ppcVar1)();
                    puVar8 = extraout_dx;
                }
                uStack6 = CONCAT22(puVar8, u_var11);
                // for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
                //   u_var6 = uStack6;
                //   pass1_1030_1d58(iVar10.field_0xc);
                //   puVar9 = (puVar8 | u_var6);
                //   if ((puVar9 != 0x0) &&
                //      (u_var3 = pass1_1030_6fa0(u_var6 & 0xffff | ZEXT24(puVar8) << 0x10),
                //      u_var3 == 0xb)) {
                //     pass1_1030_7c50(CONCAT13((puVar8 >> 0x8),
                //                              CONCAT12(puVar8,u_var6)),
                //                     uStack12,0x4,uStack12,puVar9);
                //     return;
                //   }
                //   puVar8 = puVar9;
                // }
            } else {
                lVar2 = iVar10.field_0x210;
                u_var6 = (lVar2 + 0xa);
                // for (uStack10 = 0x0; uStack10 < u_var6; uStack10 += 0x1) {
                //   u_var5 = u_var6;
                //   bad_1030_1312();
                //   u_var11 = u_var5;
                //   u_var10 = param_5 | u_var11;
                //   if (((u_var10 != 0x0) &&
                //       (pass1_1030_cc44(u_var11,param_5,uStack12,param_4,0x4), u_var11 != 0x0)) &&
                //      (uStack12 -= u_var11, uStack12 == 0x0)) {
                //     return;
                //   }
                //   param_5 = u_var10;
                // }
            }
        } else {
            iStack22 = param_2 - param_3;
            if ((iVar10.field_0x210 == 0x0) || (lVar2 = iVar10.field_0x210, (lVar2 + 0xa) == 0x0)) {
                if (iVar10.field_0xc == 0x0) {
                    i_var4 = 0x0;
                    u_var11 = 0x0;
                } else {
                    ppcVar1 = (*iVar10.field_0xc + 0x10);
                    i_var4 = iStack22;
                    (**ppcVar1)();
                    u_var11 = extraout_DX_00;
                }
                uStack6 = CONCAT22(u_var11, i_var4);
                // for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
                //   u_var6 = uStack6;
                //   pass1_1030_1d58(iVar10.field_0xc);
                //   u_var10 = u_var11 | u_var6;
                //   if ((u_var10 != 0x0) &&
                //      (u_var3 = pass1_1030_6fa0(u_var6 & 0xffff | u_var11 << 0x10),
                //      u_var3 == 0xb)) {
                //     pass1_1030_6e9c(CONCAT13((u_var11 >> 0x8),
                //                              CONCAT12(u_var11,u_var6)),iStack22,
                //                     0x4);
                //     return;
                //   }
                //   u_var11 = u_var10;
                // }
            } else {
                lVar2 = iVar10.field_0x210;
                u_var6 = (lVar2 + 0xa);
                // for (uStack10 = 0x0; uStack10 < u_var6; uStack10 += 0x1) {
                //   uVar7 = u_var6;
                //   bad_1030_1312();
                //   u_var5 = param_5;
                //   u_var11 = uVar7;
                //   param_5 |= u_var11;
                //   if (param_5 != 0x0) {
                //     pass1_1030_ce72(u_var5 << 0x10 | uVar7 & 0xffff,iStack22,param_4,0x4);
                //     iStack22 -= u_var11;
                //     if (iStack22 == 0x0) {
                //       return;
                //     }
                //   }
                // }
            }
        }
    }
    return;
}

pub fn pass1_1038_3aa6(param_1: u32, param_2: u16, param_3: u16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u32;
    let u_var5: u32;
    let extraout_dx: u16;
    let u_var6: u16;
    let uVar7: u16;
    let i_var8: i16;
    let uVar9: u16;
    let uStack12: u32;
    let uStack8: u32;

    // uVar9 = (param_1 >> 0x10);
    i_var8 = param_1;
    if (((i_var8 + 0x210) == 0x0) || (u_var2 = (i_var8 + 0x210), (u_var2 + 0xa) == 0x0)) {
        if ((i_var8 + 0xc) == 0x0) {
            param_2 = 0x0;
            u_var6 = 0x0;
        } else {
            ppcVar1 = ((i_var8 + 0xc) + 0x10);
            (**ppcVar1)();
            u_var6 = extraout_dx;
        }
        uStack8 = CONCAT22(u_var6, param_2);
        // for (uStack12 = 0x0; uStack12 < uStack8; uStack12 += 0x1) {
        //   u_var4 = uStack8;
        //   pass1_1030_1d58((i_var8 + 0xc));
        //   uVar7 = u_var6 | u_var4;
        //   if ((uVar7 != 0x0) &&
        //      (u_var3 = pass1_1030_6fa0(u_var4 & 0xffff | u_var6 << 0x10), u_var3 == 0xb)) {
        //     pass1_1030_6b86(u_var4 & 0xffff | u_var6 << 0x10,0xb,0x1030);
        //     return;
        //   }
        //   u_var6 = uVar7;
        // }
    } else {
        u_var2 = (i_var8 + 0x210);
        u_var4 = (u_var2 + 0xa);
        // for (uStack12 = 0x0; uStack12 < u_var4; uStack12 += 0x1) {
        //   u_var5 = u_var4;
        //   bad_1030_1312();
        //   u_var6 = param_3 | u_var5;
        //   if (u_var6 != 0x0) {
        //     pass1_1030_ce2e(u_var5,param_3,0x4);
        //   }
        //   param_3 = u_var6;
        // }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_3ba0(param_1: u32) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let cVar3: u8;
    let puVar4: u32;
    let u_var5: u32;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u32;
    let puVar9: U32Ptr;
    let extraout_dx: U32Ptr;
    let puVar10: U32Ptr;
    let u_var11: u16;
    let iVar13: &mut Struct428;
    let uVar12: u16;
    let uVar13: u16;
    let unaff_SS: u16;
    let puVar14: u32;
    let uVar15: u32;
    let uStack20: u32;

    // uVar12 = (param_1 >> 0x10);
    iVar13 = param_1;
    pu_var1 = &iVar13.field_0x210;
    u_var6 = (&iVar13.field_0x210 + 0x2);
    if ((u_var6 | pu_var1) != 0x0) {
        ppcVar2 = *pu_var1;
        (**ppcVar2)();
    }
    puVar14 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x1e);
    // puVar9 = (puVar14 >> 0x10);
    uVar8 = puVar14 & 0xffff;
    pass1_1038_4d6e(param_1, puVar14, uVar8, puVar9);
    u_var5 = uVar8 & 0xffff;
    puVar4 = (u_var5 | ZEXT24(puVar9) << 0x10);
    ppcVar2 = (*puVar4 + 0x10);
    (**ppcVar2)(0x1008, uVar8, puVar9);
    u_var6 = uVar8;
    if ((extraout_dx == 0x0) && (false || (u_var6 < 0x5))) {
        u_var6 = 0x5;
    }
    u_var6 += 0x1;
    uVar13 = 0x1000;
    puVar10 = extraout_dx;
    uVar7 = u_var6;
    mem_op_1000_179c(0x1c, extraout_dx, 0x1000);
    u_var11 = puVar10 | uVar7;
    if (u_var11 == 0x0) {
        iVar13.field_0x210 = 0x0;
    } else {
        u_var11 = u_var6 >> 0xf;
        cVar3 = (u_var6 >> 0x8);
        uVar13 = 0x1030;
        struct_1030_11aa(
            CONCAT22(puVar10, uVar7),
            0x5,
            CONCAT13(cVar3 >> 0xf, CONCAT12(cVar3 >> 0x7, u_var6)),
            unaff_SS,
        );
        &iVar13.field_0x210 = u_var6;
        (&iVar13.field_0x210 + 0x2) = u_var11;
    }
    uVar15 = iVar13.field_0x210;
    (uVar15 + 0x1a) = 0x0;
    // for (uStack20 = 0x0; uStack20 < (uVar8 & 0xffff | ZEXT24(extraout_dx) << 0x10);
    //     uStack20 += 0x1) {
    //   uVar15 = pass1_1030_1d7c((uVar8 & 0xffff),u_var11,puVar4);
    //   u_var6 = (uVar15 >> 0x10);
    //   u_var11 = u_var6 | uVar15;
    //   if (u_var11 != 0x0) {
    //     pass1_1030_1358(iVar13.field_0x210,uVar15,u_var6,uStack20 + 0x1,unaff_SS);
    //   }
    //   uVar13 = 0x1030;
    // }
    if (puVar4 != 0x0) {
        ppcVar2 = *puVar4;
        (**ppcVar2)(uVar13, u_var5, puVar9, 0x1);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_3cc0(
    param_1: u32,
    param_2: u16,
    param_3: U32Ptr,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
) {
    let lVar1: i32;
    let ppcVar2: u32;
    let u_var3: u16;
    let puVar4: u32;
    let u_var5: u16;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let extraout_DX_01: u16;
    let extraout_DX_02: u16;
    let u_var6: u16;
    let extraout_DX_03: U32Ptr;
    let puVar7: U32Ptr;
    let extraout_DX_04: U32Ptr;
    let puVar8: u32;
    let puVar9: U32Ptr;
    let u_var10: u16;
    let puVar11: u32;
    let uVar12: u32;
    let uVar13: u32;
    let uVar14: u8;
    let uVar15: u8;
    let uVar16: u8;
    let uVar17: u8;
    let puStack26: u32;
    let uStack22: u32;
    let uStack18: u32;
    let uStack14: u32;
    let puStack10: u32;

    if (param_4 == 0x1e) {
        u_var10 = 0x1008;
        puVar11 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x27);
        // puVar9 = (puVar11 >> 0x10);
        puVar8 = puVar11;
        pass1_1038_4e78(puVar8, puVar9, param_1, puVar11);
        puStack10 = CONCAT22(puVar9, puVar8);
        ppcVar2 = (*puStack10 + 0x10);
        puVar4 = puVar8;
        (**ppcVar2)(0x1008, puVar8, puVar9);
        uStack14 = CONCAT22(extraout_DX_00, puVar4);
        puVar7 = extraout_DX_00;
        // for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
        //   uVar12 = pass1_1030_1d7c(puVar4,puVar7,puStack10);
        //   puVar7 = ((uVar12 >> 0x10) | uVar12);
        //   if (puVar7 != 0x0) {
        //     u_var5 = pass1_1030_bfb8(uVar12,param_7);
        //     puStack26 = CONCAT22(puVar7,u_var5);
        //     puVar7 = (puVar7 | u_var5);
        //     if (puVar7 != 0x0) {
        //       pass1_1028_b58e(uVar12);
        //       if (CONCAT22(param_3,param_2) <= puStack26) {
        //         u_var10 = 0x1030;
        //         pass1_1030_7ddc(CONCAT22(extraout_DX_01,u_var5),
        //                         CONCAT13((param_3 >> 0x8),
        //                                  CONCAT12(param_3,param_2)),0x1e,param_2,param_3
        //                         ,param_5,param_6,param_7);
        //         break;
        //       }
        //       puVar7 = param_3;
        //       pass1_1030_7ddc(CONCAT22(extraout_DX_01,u_var5),puStack26,0x1e,param_2,
        //                       param_3,param_5,param_6,param_7);
        //       lVar1 = CONCAT22(param_3,param_2) - puStack26;
        //       param_2 = lVar1;
        //       param_3 = (lVar1 >> 0x10);
        //     }
        //   }
        //   u_var10 = 0x1030;
        // }
        puStack26 = puStack10;
        if (puStack10 == 0x0) {
            return;
        }
    } else {
        if (param_4 != 0x21) {
            u_var10 = 0x1008;
            puVar11 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x3);
            // puVar7 = (puVar11 >> 0x10);
            u_var3 = puVar11;
            pass1_1038_4e78(u_var3, puVar7, param_1, puVar11);
            puStack26 = CONCAT22(puVar7, u_var3);
            ppcVar2 = (*puStack26 + 0x10);
            (**ppcVar2)(0x1008, u_var3, puVar7);
            uStack22 = CONCAT22(extraout_dx, u_var3);
            uStack18 = 0x0;
            puVar7 = extraout_dx;
            //LAB_1038_3e9c:
            if (uStack18 < uStack22) {
                u_var10 = 0x1030;
                uVar12 = pass1_1030_1d7c(u_var3, puVar7, puStack26);
                puVar7 = ((uVar12 >> 0x10) | uVar12);
                if (puVar7 == 0x0) {
                    // goto
                    // LAB_1038_3e98;
                }
                u_var10 = SUB42(&USHORT_1050_1028, 0x0);
                uVar13 = pass1_1028_45e2(uVar12, uVar12, puVar7, param_7);
                u_var6 = uVar13;
                puVar7 = ((uVar13 >> 0x10) | u_var6);
                if (puVar7 == 0x0) {
                    // goto
                    // LAB_1038_3e98;
                }
                pass1_1028_b58e(uVar12);
                uVar12 = CONCAT22(param_3, param_2);
                if (uVar13 < uVar12) {
                    u_var10 = 0x1030;
                    puVar7 = param_3;
                    pass1_1030_7ddc(
                        CONCAT22(extraout_DX_04, u_var6),
                        uVar13,
                        param_4,
                        param_2,
                        param_3,
                        param_5,
                        param_6,
                        param_7,
                    );
                    lVar1 = CONCAT22(param_3, param_2) - uVar13;
                    param_2 = lVar1;
                    // param_3 = (lVar1 >> 0x10);
                    //           TODO: goto LAB_1038_3e98;
                }
                uVar16 = SUB21(param_3, 0x0);
                uVar17 = (param_3 >> 0x8);
                uVar14 = u_var6;
                uVar15 = (u_var6 >> 0x8);
                puVar7 = extraout_DX_04;
                //LAB_1038_3e67:
                u_var10 = 0x1030;
                pass1_1030_7ddc(
                    CONCAT22(puVar7, CONCAT11(uVar15, uVar14)),
                    CONCAT13(uVar17, CONCAT12(uVar16, param_2)),
                    param_4,
                    uVar12,
                    param_3,
                    param_5,
                    param_6,
                    param_7,
                );
            }
            //       TODO: goto LAB_1038_3e6c;
        }
        u_var10 = 0x1008;
        puVar11 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0xa);
        // puVar7 = (puVar11 >> 0x10);
        u_var3 = puVar11;
        pass1_1038_4e78(u_var3, puVar7, param_1, puVar11);
        puStack26 = CONCAT22(puVar7, u_var3);
        ppcVar2 = (*puStack26 + 0x10);
        (**ppcVar2)(0x1008, u_var3, puVar7);
        uStack22 = CONCAT22(extraout_DX_02, u_var3);
        u_var6 = extraout_DX_02;
        //     for (uStack18 = 0x0; uStack18 < uStack22; uStack18 += 0x1) {
        //       u_var10 = 0x1030;
        //       uVar13 = pass1_1030_1d7c(u_var3,u_var6,puStack26);
        //       uVar12 = uVar13 & 0xffff;
        //       u_var6 = (uVar13 >> 0x10) | uVar12;
        //       if (u_var6 != 0x0) {
        //         uVar16 = SUB21(param_3,0x0);
        //         uVar17 = (param_3 >> 0x8);
        //         pass1_1028_b58e(uVar13);
        //         uVar14 = uVar12;
        //         uVar15 = (uVar12 >> 0x8);
        //         param_3 = extraout_DX_03;
        //         puVar7 = extraout_DX_03;
        // //         TODO: goto LAB_1038_3e67;
        //       }
        //     }
        //LAB_1038_3e6c:
        if (puStack26 == 0x0) {
            return;
        }
        // puVar9 = (puStack26 >> 0x10);
        puVar8 = puStack26;
    }
    ppcVar2 = *puVar8;
    (**ppcVar2)(u_var10, puStack26, puVar9, 0x1);
    return;
    //LAB_1038_3e98:
    uStack18 += 0x1;
    //   TODO: goto LAB_1038_3e9c;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_3efc(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: i16,
    param_6: u16,
) {
    let ppcVar1: u32;
    let puStack6: u32;

    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4);
    puStack6 = CONCAT22(param_6, param_5);
    (param_5 + 0x1c) = (param_3 + 0x4);
    ppcVar1 = (*puStack6 + 0x58);
    (**ppcVar1)(&USHORT_1050_1028, param_5, param_6, param_3);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_3f38(param_1: U32Ptr, param_2: U32Ptr, param_3: u32, param_4: i16, param_5: u16) {
    let ppcVar1: u32;
    let i_var2: i16;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let pu_var3: u32;
    let u_var4: u16;
    let u_var5: u32;
    let u_var6: u16;
    let uStack10: u32;
    let puStack6: u32;

    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_3);
    puStack6 = CONCAT22(param_5, param_4);
    i_var2 = param_4;
    pass1_1028_b58e(CONCAT22(param_5, param_4));
    uStack10 = CONCAT22(extraout_dx, i_var2);
    u_var5 = (i_var2 + 0x4);
    ppcVar1 = (*param_1 + 0x18);
    (**ppcVar1)(&USHORT_1050_1028, param_1, u_var5);
    u_var6 = 0x0;
    u_var4 = 0x0;
    ppcVar1 = (*param_2 + 0x8);
    pu_var3 = param_2;
    (**ppcVar1)();
    pass1_1030_73ee(uStack10, (param_2 + 0x4), extraout_DX_00);
    ppcVar1 = (*puStack6 + 0x58);
    (**ppcVar1)(
        0x1030, param_4, param_5, param_2, pu_var3, u_var4, u_var5, u_var6,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_3fb0(param_1: u32) {
    let u_var1: u32;

    u_var1 = (param_1 + 0x200);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_3fca(param_1: u32, param_2: u16, param_3: u16) {
    let u_var1: u32;
    let ppcVar2: u32;
    let u_var3: u16;
    let extraout_dx: u16;
    let u_var4: u16;
    let extraout_DX_00: u16;
    let u_var5: u16;
    let iVar6: i16;
    let unaff_DI: i16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u32;
    let puVar11: U32Ptr;
    let uVar12: u8;
    let uVar13: u8;
    let uVar14: u8;
    let uVar15: u8;
    let uVar16: u16;
    let iStack38: i16;
    let local_24: i16;
    let local_22: [u8; 2];
    let piStack32: U32Ptr;
    let uStack30: u16;
    let puStack28: U32Ptr;
    let uStack26: u16;
    let uStack24: u16;
    let uStack22: u32;
    let uStack18: u16;
    let uStack16: u16;
    let paStack14: &mut Struct18;
    let paStack10: &mut Struct18;
    let uStack6: u32;

    // uVar7 = (param_1 >> 0x10);
    u_var5 = param_1;
    if ((u_var5 + 0xc) == 0x0) {
        param_2 = 0x0;
        u_var4 = 0x0;
    } else {
        ppcVar2 = ((u_var5 + 0xc) + 0x10);
        (**ppcVar2)();
        u_var4 = extraout_dx;
    }
    uStack6 = CONCAT22(u_var4, param_2);
    ctx.PTR_LOOP_1050_5f2e = (u_var4 | param_2);
    if (ctx.PTR_LOOP_1050_5f2e != 0x0) {
        if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e, 0x1000);
        } else {
        }
        u_var3 = fn_ptr_op_1000_1708(
            uStack6 << 0x2,
            0x0,
            0x1,
            ctx.PTR_LOOP_1050_5f2c,
            ctx.PTR_LOOP_1050_5f2e,
            0x1000,
        );
        paStack10 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var3);
        if (ctx.PTR_LOOP_1050_5f2c == 0x0) {
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e, 0x1000);
        } else {
        }
        uVar9 = 0x1000;
        u_var3 = fn_ptr_op_1000_1708(
            uStack6 << 0x2,
            0x0,
            0x1,
            ctx.PTR_LOOP_1050_5f2c,
            ctx.PTR_LOOP_1050_5f2e,
            0x1000,
        );
        paStack14 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var3);
        // for (uStack22 = 0x0; uStack22 < uStack6; uStack22 += 0x1) {
        //   u_var1 = (u_var5 + 0xc);
        //   ppcVar2 = ((u_var5 + 0xc) + 0x4);
        //   u_var10 = uStack6;
        //   (**ppcVar2)(uVar9,u_var1,(u_var1 >> 0x10),uStack22,
        //               (uStack22 >> 0x10));
        //   u_var4 = u_var10;
        //   ctx.PTR_LOOP_1050_5f2e = (extraout_DX_00 | u_var4);
        //   uStack18 = u_var4;
        //   uStack16 = extraout_DX_00;
        //   if (ctx.PTR_LOOP_1050_5f2e != 0x0) {
        //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var4,extraout_DX_00);
        //     uStack22 *= 0x4;
        //     uVar8 = (paStack10 >> 0x10);
        //     iVar6 = paStack10;
        //     (uStack22 + iVar6) = u_var4;
        //     (uStack22 + iVar6 + 0x2) = ctx.PTR_LOOP_1050_5f2e;
        //     uVar9 = 0x1030;
        //     u_var10 = struct_op_1030_73a8(CONCAT22(ctx.PTR_LOOP_1050_5f2e,
        //                                           (uStack22 + iVar6)));
        //     ctx.PTR_LOOP_1050_5f2e = (u_var10 >> 0x10);
        //     uVar8 = (paStack14 >> 0x10);
        //     (paStack14 + uStack22) = u_var10;
        //     (paStack14 + uStack22 + 0x2) = ctx.PTR_LOOP_1050_5f2e;
        //   }
        // }
        // for (uStack22 = 0x0; uStack22 < uStack6; uStack22 += 0x1) {
        //   uVar9 = (paStack14 >> 0x10);
        //   iVar6 = paStack14;
        //   if (((uStack22 * 0x4 + iVar6) != 0x0) &&
        //      (u_var1 = (uStack22 * 0x4 + iVar6),
        //      (u_var1 + 0x1a) = 0x0,
        //      u_var1 = (uStack22 * 0x4 + iVar6),
        //      (u_var1 + 0x12) == 0x5)) {
        //     pass1_1028_bdac(*(u32 **)(uStack22 * 0x4 + iVar6),0x6,
        //                     &USHORT_1050_1028);
        //   }
        // }
        (u_var5 + 0x204) = 0x0;
        puVar11 = mixed_1010_20ba(
            ctx.PTR_LOOP_1050_0ed0,
            0x2,
            param_3,
            ctx.PTR_LOOP_1050_5f2e,
            unaff_DI,
        );
        // uStack30 = (puVar11 >> 0x10);
        uStack26 = SUB42(puVar11, 0x0);
        puStack28 = ctx.PTR_LOOP_1050_13ae;
        if (ctx.PTR_LOOP_1050_13ae == (&ctx.PTR_LOOP_1050_0000 + 0x1)) {
            (u_var5 + 0x204) = 0x1;
        }
        uStack24 = uStack30;
        pass1_1038_5a96(u_var5, uVar7, uStack6, paStack14);
        pass1_1038_5cc6(param_1, uStack6, paStack14, paStack10, 0x0, 0x2);
        pass1_1038_5b3c(u_var5, uVar7, uStack6, paStack14);
        pass1_1038_5cc6(param_1, uStack6, paStack14, paStack10, 0x0, 0x1);
        uVar14 = SUB21(local_22, 0x0);
        uVar15 = (local_22 >> 0x8);
        piStack32 = &local_24;
        uVar12 = SUB21(piStack32, 0x0);
        uVar13 = (piStack32 >> 0x8);
        u_var1 = (u_var5 + 0x8);
        u_var3 = param_3;
        uVar16 = param_3;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1);
        pass1_1030_5b1c(
            CONCAT22(uStack30, piStack32),
            CONCAT22(u_var3, CONCAT11(uVar13, uVar12)),
            CONCAT22(uVar16, CONCAT11(uVar15, uVar14)),
        );
        // for (iStack38 = 0x1; iStack38 <= local_24; iStack38 += 0x1) {
        //   pass1_1038_58e6(u_var5,uVar7,uStack6,paStack14,paStack10,iStack38,
        //                   param_3);
        //   pass1_1038_5cc6(param_1,uStack6,paStack14,paStack10,iStack38,0x3);
        // }
        pass1_1038_5a16(u_var5, uVar7, uStack6, paStack14);
        // for (uStack22 = 0x0; uStack22 < uStack6; uStack22 += 0x1) {
        //   uVar9 = (paStack14 >> 0x10);
        //   iVar6 = paStack14;
        //   if (((uStack22 * 0x4 + iVar6) != 0x0) &&
        //      (u_var1 = (uStack22 * 0x4 + iVar6),
        //      (u_var1 + 0x12) != 0x5)) {
        //     u_var1 = (uStack22 * 0x4 + iVar6);
        //     ppcVar2 = (
        //                               (uStack22 * 0x4 + iVar6) + 0x28);
        //     (**ppcVar2)(0x1030,u_var1,(u_var1 >> 0x10));
        //   }
        // }
        fn_ptr_1000_17ce(ctx, paStack10, 0x1000);
        fn_ptr_1000_17ce(ctx, paStack14, 0x1000);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_42cc(param_1: u32, param_2: u16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let bVar3: bool;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let extraout_dx: u16;
    let uVar8: u16;
    let extraout_DX_00: u16;
    let i_var9: i16;
    let u_var10: u16;
    let u_var11: u16;
    let puVar12: u32;
    let puVar13: u32;
    let uStack24: u32;
    let uStack18: u32;
    let puStack10: u32;

    // u_var10 = (param_1 >> 0x10);
    i_var9 = param_1;
    if ((i_var9 + 0x1f6) == 0x0) {
        return;
    }
    u_var11 = 0x1008;
    puVar12 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x2d);
    // puVar7 = (puVar12 >> 0x10);
    u_var4 = puVar12;
    pass1_1038_4d6e(param_1, puVar12, u_var4, puVar7);
    puStack10 = CONCAT22(puVar7, u_var4);
    ppcVar1 = (*puStack10 + 0x10);
    u_var5 = u_var4;
    (**ppcVar1)(0x1008, u_var4, puVar7);
    uStack18 = CONCAT22(extraout_dx, u_var5);
    bVar3 = false;
    uVar8 = extraout_dx;
    // for (uStack24 = 0x0; uStack24 < uStack18; uStack24 += 0x1) {
    //   u_var11 = 0x1030;
    //   puVar13 = pass1_1030_1d7c(u_var5,uVar8,puStack10);
    //   u_var6 = puVar13;
    //   uVar8 = (puVar13 >> 0x10) | u_var6;
    //   if (uVar8 != 0x0) {
    //     ppcVar1 = (*puVar13 + 0x50);
    //     (**ppcVar1)();
    //     uVar8 = extraout_DX_00;
    //     if (u_var6 != 0x0) {
    //       bVar3 = true;
    //     }
    //   }
    // }
    if (bVar3) {
        u_var2 = (i_var9 + 0x1f6);
        (u_var2 + 0x1aa) = 0x0;
    } else {
        u_var11 = 0x1030;
        pass1_1030_38b8();
        uVar8 |= uStack18;
        if (uVar8 != 0x0) {
            u_var11 = 0x1030;
            pass1_1030_326a((i_var9 + 0x1f6), uStack18, uVar8, param_2);
        }
    }
    if (puStack10 != 0x0) {
        ppcVar1 = *puStack10;
        (**ppcVar1)(u_var11, u_var4, puVar7, 0x1);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_43cc(
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
    param_6: i16,
) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;
    let puVar6: U32Ptr;
    let extraout_dx: u16;
    let uVar7: u16;
    let i_var8: i16;
    let i_var9: i16;
    let u_var10: u16;
    let puVar11: u32;
    let uVar12: u32;
    let uStack22: u32;
    let uStack18: u32;
    let puStack14: u32;

    if (param_4 == 0x5) {
        pass1_1038_4900(CONCAT22(param_2, param_1));
        return;
    }
    pass1_1038_53ba(CONCAT22(param_2, param_1), param_4);
    if ((param_6 != 0x0) || (param_5 != 0x0)) {
        i_var8 = param_4 * 0x4;
        u_var2 = (param_1 + i_var8 + 0x14e);
        i_var9 = ((param_1 + i_var8 + 0x150) - (param_3 >> 0xf)) - (u_var2 < param_3);
        (param_1 + i_var8 + 0x14e) = u_var2 - param_3;
        (param_1 + i_var8 + 0x150) = i_var9;
        if (i_var9 < 0x0) {
            (param_1 + i_var8 + 0x14e) = 0x0;
        }
        u_var10 = 0x1008;
        puVar11 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x1e);
        // puVar6 = (puVar11 >> 0x10);
        u_var2 = puVar11;
        pass1_1038_4e78(u_var2, puVar6, CONCAT22(param_2, param_1), puVar11);
        puStack14 = CONCAT22(puVar6, u_var2);
        ppcVar1 = (*puStack14 + 0x10);
        u_var3 = u_var2;
        (**ppcVar1)(0x1008, u_var2, puVar6);
        uStack18 = CONCAT22(extraout_dx, u_var3);
        uVar7 = extraout_dx;
        // for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
        //   uVar12 = pass1_1030_1d7c(u_var3,uVar7,puStack14);
        //   uVar7 = (uVar12 >> 0x10);
        //   u_var5 = uVar12 & 0xffff;
        //   for (; u_var4 = u_var5, param_3 != 0x0; param_3 -= 0x1) {
        //     pass1_1030_cf78(uVar12,param_4);
        //     u_var5 = u_var4;
        //     if (u_var4 == 0x0) break;
        //   }
        //   u_var10 = 0x1030;
        //   if (param_3 == 0x0) break;
        // }
        if (puStack14 != 0x0) {
            ppcVar1 = *puStack14;
            (**ppcVar1)(u_var10, u_var2, puVar6, 0x1);
            return;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_44d8(
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
    param_6: i16,
) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;
    let puVar6: U32Ptr;
    let extraout_dx: u16;
    let uVar7: u16;
    let uVar8: u16;
    let i_var9: &mut Struct697;
    let iVar10: i16;
    let u_var11: u16;
    let puVar12: u32;
    let uVar13: u32;
    let uStack22: u32;
    let uStack18: u32;
    let puStack14: u32;

    if (param_4 == 0x5) {
        pass1_1038_4900(CONCAT22(param_2, param_1));
        return;
    }
    pass1_1038_53ba(CONCAT22(param_2, param_1), param_4);
    if ((param_6 != 0x0) || (param_5 != 0x0)) {
        i_var9 = (param_4 * 0x4);
        u_var2 = (i_var9 + param_1 + 0x14e);
        iVar10 = ((i_var9 + param_1 + 0x150) - (param_3 >> 0xf)) - (u_var2 < param_3);
        (i_var9 + param_1 + 0x14e) = u_var2 - param_3;
        (i_var9 + param_1 + 0x150) = iVar10;
        if (iVar10 < 0x0) {
            (i_var9 + param_1 + 0x14e) = 0x0;
        }
        u_var11 = 0x1008;
        puVar12 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x1e);
        // puVar6 = (puVar12 >> 0x10);
        u_var2 = puVar12;
        pass1_1038_4e78(u_var2, puVar6, CONCAT22(param_2, param_1), puVar12);
        puStack14 = CONCAT22(puVar6, u_var2);
        ppcVar1 = (*puStack14 + 0x10);
        u_var3 = u_var2;
        (**ppcVar1)(0x1008, u_var2, puVar6);
        uStack18 = CONCAT22(extraout_dx, u_var3);
        uVar7 = extraout_dx;
        // for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
        //   uVar13 = pass1_1030_1d7c(u_var3,uVar7,puStack14);
        //   uVar8 = (uVar13 >> 0x10);
        //   u_var5 = uVar13 & 0xffff;
        //   uVar7 = uVar8;
        //   for (; u_var4 = u_var5, param_3 != 0x0; param_3 -= 0x1) {
        //     pass1_1030_d00c(uVar13,uVar8,param_4);
        //     u_var5 = u_var4;
        //     if (u_var4 == 0x0) break;
        //   }
        //   u_var11 = 0x1030;
        //   if (param_3 == 0x0) break;
        // }
        if (puStack14 != 0x0) {
            ppcVar1 = *puStack14;
            (**ppcVar1)(u_var11, u_var2, puVar6, 0x1);
            return;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_45e4(param_1: u32, param_2: u16, param_3: i16, param_4: u16) {
    let pi_var1: U32Ptr;
    let ppcVar2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let i_var8: i16;
    let i_var9: i16;
    let iVar10: i16;
    let puVar11: U32Ptr;
    let iVar12: i16;
    let uVar13: u16;
    let uVar14: u16;
    let bVar15: bool;
    let puVar16: u32;
    let uStack28: u16;
    let puStack22: u32;

    // uVar14 = (param_1 >> 0x10);
    iVar12 = param_1;
    pass1_1030_38f2((iVar12 + 0x1f6), 0x2, param_4);
    i_var8 = param_3;
    u_var4 = param_2;
    pass1_1030_38f2((iVar12 + 0x1f6), 0x1, param_4);
    bVar15 = param_2 < u_var4;
    uVar13 = param_2 - u_var4;
    iVar10 = param_3 - i_var8;
    pass1_1030_38f2((iVar12 + 0x1f6), 0x4, param_4);
    i_var9 = i_var8;
    u_var5 = u_var4;
    pass1_1030_38f2((iVar12 + 0x1f6), 0x3, param_4);
    uVar7 = (iVar12 + 0x24);
    u_var6 = uVar7 + (u_var4 - u_var5);
    iVar10 = (uVar7 >> 0xf)
        + ((i_var8 - i_var9) - (u_var4 < u_var5))
        + CARRY2(uVar7, u_var4 - u_var5)
        + (iVar10 - bVar15)
        + CARRY2(u_var6, uVar13);
    if ((iVar10 < 0x0) || (iVar10 < 0x1 && (u_var6 + uVar13 == 0x0))) {
        iVar10 = -0x1;
    } else {
        iVar10 = 0x1;
    }
    pi_var1 = (iVar12 + 0x24);
    *pi_var1 = *pi_var1 + iVar10;
    puVar16 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x16);
    // puVar11 = (puVar16 >> 0x10);
    uVar7 = puVar16;
    pass1_1038_4d6e(param_1, puVar16, uVar7, puVar11);
    puStack22 = CONCAT22(puVar11, uVar7);
    u_var3 = *puStack22;
    ppcVar2 = u_var3 + 0x8;
    u_var5 = uVar7;
    (**ppcVar2)(0x1008, uVar7, puVar11);
    if (puStack22 != 0x0) {
        ppcVar2 = u_var3;
        (**ppcVar2)(0x1008, uVar7, puVar11, 0x1);
    }
    pi_var1 = (iVar12 + 0x24);
    *pi_var1 = *pi_var1 + u_var5 * 0x2;
    iVar10 = (iVar12 + 0x24);
    if (0x64 < iVar10) {
        iVar10 = 0x64;
    }
    (iVar12 + 0x24) = iVar10;
    if (iVar10 < 0x0) {
        iVar10 = 0x0;
    }
    (iVar12 + 0x24) = iVar10;
    iVar10 /= 0xa;
    uStack28 = 0x10;
    if (iVar10 < 0xb) {
        uStack28 = 0x14;
    } else {
        if (iVar10 < 0x15) {
            uStack28 = 0x13;
        } else {
            if (iVar10 < 0x1f) {
                uStack28 = 0x12;
            } else {
                if (iVar10 < 0x29) {
                    uStack28 = 0x11;
                } else {
                    if (iVar10 < 0x33) {
                        uStack28 = 0x10;
                    } else {
                        if (iVar10 < 0x3d) {
                            uStack28 = 0xf;
                        } else {
                            if (iVar10 < 0x47) {
                                uStack28 = 0xe;
                            } else {
                                if (iVar10 < 0x51) {
                                    uStack28 = 0xd;
                                } else {
                                    if (iVar10 < 0x5b) {
                                        uStack28 = 0xc;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pass1_1030_3258((iVar12 + 0x1f6), uStack28);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_4760(param_1: u32) {
    let pu_var1: U32Ptr;
    let ppcVar2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let iVar5: i16;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let puVar8: U32Ptr;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let extraout_DX_02: u16;
    let uVar9: u16;
    let extraout_DX_03: u16;
    let extraout_DX_04: u16;
    let iVar10: &mut Struct700;
    let u_var10: u16;
    let u_var11: u16;
    let unaff_SS: u16;
    let puVar12: u32;
    let uVar13: u32;
    let uVar14: u8;
    let puVar15: U32Ptr;
    let uStack26: u32;
    let uStack22: u32;
    let puStack14: u32;
    let puStack10: u32;

    // u_var10 = (param_1 >> 0x10);
    iVar10 = param_1;
    pu_var1 = &iVar10.field_0x22;
    *pu_var1 = *pu_var1 + iVar10.field_0x20c;
    puVar12 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x26);
    // puVar7 = (puVar12 >> 0x10);
    u_var6 = puVar12;
    pass1_1038_4d6e(param_1, puVar12, u_var6, puVar7);
    puStack10 = CONCAT22(puVar7, u_var6);
    u_var11 = 0x1008;
    puVar12 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x1a);
    // puVar8 = (puVar12 >> 0x10);
    u_var3 = puVar12;
    pass1_1038_4d6e(param_1, puVar12, u_var3, puVar8);
    puStack14 = CONCAT22(puVar8, u_var3);
    ppcVar2 = (*puStack14 + 0x10);
    u_var4 = u_var3;
    (**ppcVar2)(0x1008, u_var3, puVar8);
    uVar14 = u_var6;
    puVar15 = puVar7;
    if ((extraout_dx | u_var4) == 0x0) {
        ppcVar2 = (*puStack10 + 0x10);
        (**ppcVar2)();
        pu_var1 = &iVar10.field_0x22;
        *pu_var1 = *pu_var1 + u_var4;
        uVar9 = extraout_DX_00;
    } else {
        ppcVar2 = (*puStack10 + 0x10);
        (**ppcVar2)();
        uStack22 = CONCAT22(extraout_DX_03, u_var4);
        uVar9 = extraout_DX_03;
        //     for (uStack26 = 0x0; uStack26 < uStack22; uStack26 += 0x1) {
        //       uVar13 = pass1_1030_1d7c(u_var4,uVar9,puStack10);
        //       iVar5 = uVar13;
        //       u_var11 = SUB42(&USHORT_1050_1028,0x0);
        //       func_0x10285a94();
        //       if (iVar5 == 0x2) {
        //         if ((*ctx.PTR_LOOP_1050_65e2 & 0x1) == 0x0) goto LAB_1038_485e;
        //       }
        //       else {
        //         if (iVar5 != 0x3) {
        // //LAB_1038_485e:
        //           pu_var1 = &iVar10.field_0x22;
        //           *pu_var1 = *pu_var1 + 0x1;
        //         }
        //       }
        //       uVar9 = extraout_DX_04;
        //     }
    }
    if (puStack10 != 0x0) {
        ppcVar2 = *puStack10;
        (**ppcVar2)(u_var11, u_var6, puVar7, 0x1, uVar14, puVar15);
        uVar9 = extraout_DX_01;
    }
    if (puStack14 != 0x0) {
        ppcVar2 = *puStack14;
        (**ppcVar2)(u_var11, u_var3, puVar8, 0x1);
        uVar9 = extraout_DX_02;
    }
    pass1_1038_45e4(param_1, puStack14, uVar9, unaff_SS);
    if (0x32 < iVar10.field_0x24) {
        pu_var1 = &iVar10.field_0x22;
        *pu_var1 = *pu_var1 - 0x1;
    }
    if (iVar10.field_0x24 < 0x32) {
        pu_var1 = &iVar10.field_0x22;
        *pu_var1 = *pu_var1 + 0x1;
    }
    if (iVar10.field_0x18 < 0xfa) {
        pu_var1 = &iVar10.field_0x22;
        *pu_var1 = *pu_var1 + 0x2;
    } else {
        if (iVar10.field_0x18 < 0x1c2) {
            pu_var1 = &iVar10.field_0x22;
            *pu_var1 = *pu_var1 + 0x1;
        } else {
            if (0x225 < iVar10.field_0x18) {
                if (iVar10.field_0x18 < 0x2ee) {
                    pu_var1 = &iVar10.field_0x22;
                    *pu_var1 = *pu_var1 - 0x1;
                } else {
                    pu_var1 = &iVar10.field_0x22;
                    *pu_var1 = *pu_var1 - 0x2;
                }
            }
        }
    }
    u_var6 = iVar10.field_0x22;
    if (0x64 < u_var6) {
        u_var6 = 0x64;
    }
    iVar10.field_0x22 = u_var6;
    if (u_var6 < 0x0) {
        u_var6 = 0x0;
    }
    iVar10.field_0x22 = u_var6;
    return;
}

pub fn pass1_1038_48e0(param_1: u32, param_2: i16) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = (param_1 + 0x20e) + param_2;
    if (0xa < i_var1) {
        i_var1 = 0xa;
    }
    (param_1 + 0x20e) = i_var1;
    return;
}

pub fn pass1_1038_4900(param_1: u32) {
    let pi_var1: U32Ptr;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    pi_var1 = (param_1 + 0x20e);
    *pi_var1 = *pi_var1 + -0x1;
    if (*pi_var1 < 0x0) {
        (param_1 + 0x20e) = 0x0;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_4918(param_1: u32, param_2: i16, param_3: u16, param_4: u16, param_5: u8) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let i_var3: i16;
    let puVar4: u32;
    let u_var5: u16;
    let u_var6: u16;
    let iVar7: i16;
    let i_var8: i16;
    let uVar9: u16;
    let u_var10: u16;
    let u_var11: u32;
    let bStack347: u8;
    let local_14a: [u8; 4];
    let puStack326: u32;
    let local_144: [u8; 124];
    let local_20: u32;
    let uStack28: u16;
    let uStack26: u32;
    let uStack18: u32;
    let uStack14: u32;
    let uStack10: u32;
    let uStack6: u32;

    // uVar9 = (param_1 >> 0x10);
    iVar7 = param_1;
    if ((iVar7 + 0x4) != 0x4000001) {
        return;
    }
    u_var2 = (iVar7 + 0x8);
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
    uStack6 = CONCAT22(param_3, param_2);
    uStack10 = (param_2 + 0x10);
    // u_var10 = (uStack10 >> 0x10);
    i_var8 = uStack10;
    if ((i_var8 + 0x1c) == 0x0) {
        return;
    }
    uStack14 = 0x0;
    match (iVar7 + 0x20e) {
        0x1 => {
            uStack14._0_2_ = 0x1e;
        }
        0x2 => {
            uStack14._0_2_ = 0x1c;
        }
        0x3 => {
            uStack14._0_2_ = 0x1a;
        }

        0x4 => {
            uStack14._0_2_ = 0x18;
        }

        0x5 => {
            uStack14._0_2_ = 0x16;
        }

        0x6 => {
            uStack14._0_2_ = 0x14;
        }

        0x7 => {
            uStack14._0_2_ = 0x12;
        }

        0x8 => {
            uStack14._0_2_ = 0x10;
        }

        0x9 => {
            uStack14._0_2_ = 0xe;
        }

        0xa => {
            uStack14._0_2_ = 0xc;
        }

        _ => {} //     TODO: goto switchD_1038_49cf_caseD_a;
    }
    uStack14 = uStack14;
    // switchD_1038_49cf_caseD_a:
    uStack18 = *ctx.PTR_LOOP_1050_65e2;
    if ((uStack14 != 0x0)
        && (((uStack18 & 0xffff | (ctx.PTR_LOOP_1050_65e2 + 0x2) << 0x10) % uStack14) == 0x0))
    {
        pi_var1 = (i_var8 + 0x1c);
        *pi_var1 = *pi_var1 + -0x1;
        pi_var1 = (i_var8 + 0x1a);
        *pi_var1 = *pi_var1 + 0x1;
        i_var3 = (i_var8 + 0x1a) * 0x6 + (i_var8 + 0x16);
        u_var10 = (i_var8 + 0x18);
        local_20 = (i_var3 + -0x6);
        uStack28 = (i_var3 + -0x2);
        puStack326 = &local_20;
        puVar4 = &local_20;
        pass1_1030_64ce(
            param_4,
            puVar4,
            u_var10,
            ctx.PTR_LOOP_1050_5740,
            CONCAT22(param_4, puVar4),
            (iVar7 + 0x8),
            CONCAT22(param_4, local_14a),
        );
        uStack26 = *puVar4;
        u_var6 = (puVar4 + 0x2);
        bStack347 = (uStack26 >> 0x18);
        u_var5 = bStack347;
        if (bStack347 != 0x0) {
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, uStack26);
            u_var11 = struct_op_1030_73a8(CONCAT22(u_var6, u_var5));
            // u_var6 = (u_var11 >> 0x10);
            if ((u_var6 | u_var11) != 0x0) {
                i_var8 = (u_var11 + 0xc);
                if (i_var8 < 0x1) {
                    return;
                }
                if (SBORROW2(i_var8, 0x1)) {
                    return;
                }
                if (0x8 < i_var8 + -0x1) {
                    return;
                }
            }
        }
        struct_op_1028_87f0(
            param_4,
            param_5,
            CONCAT22(param_4, local_144),
            0x0,
            0x0,
            0x10,
            &local_20,
            param_4,
            (iVar7 + 0x4),
            (iVar7 + 0x8),
        );
        fn_ptr_1030_835a(ctx.PTR_LOOP_1050_5748, CONCAT22(param_4, local_144));
    }
    return;
}

pub fn pass1_1038_4b20(param_1: u32, param_2: u32, param_3: u32, param_4: u16) {
    let u_var1: u32;

    u_var1 = (param_1 + 0xc);
    pass1_1020_c4f4(u_var1, param_2, (param_2 >> 0x10), param_3, u_var1, param_4);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_4b40(param_1: u32, param_2: u16, param_3: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u32;
    let extraout_dx: u16;
    let u_var4: u16;
    let extraout_DX_00: u16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let uStack14: u32;
    let uStack10: u32;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0xc) == 0x0) {
        param_2 = 0x0;
        u_var4 = 0x0;
    } else {
        ppcVar1 = ((iVar6 + 0xc) + 0x10);
        (**ppcVar1)();
        u_var4 = extraout_dx;
    }
    uStack10 = CONCAT22(u_var4, param_2);
    // for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    //   ppcVar1 = ((iVar6 + 0xc) + 0x4);
    //   u_var3 = uStack10;
    //   (**ppcVar1)(param_3,(iVar6 + 0xc));
    //   u_var2 = u_var3;
    //   u_var5 = extraout_DX_00 | u_var2;
    //   if (u_var5 != 0x0) {
    //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var2,extraout_DX_00);
    //     param_3 = 0x1030;
    //     struct_op_1030_73a8(CONCAT22(u_var5,u_var2));
    //   }
    // }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_4c1a(param_1: u32, param_2: u16, param_3: u32, param_4: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u32;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let uVar8: u32;
    let uStack14: u32;
    let uStack10: u32;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    uVar8 = (iVar6 + 0xc);
    ppcVar1 = ((iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uStack10 = CONCAT22(param_3, param_2);
    // for (uStack14 = 0x0; u_var5 = param_3, uStack14 < uStack10; uStack14 += 0x1) {
    //   ppcVar1 = ((iVar6 + 0xc) + 0x4);
    //   u_var4 = uStack10;
    //   (**ppcVar1)(param_4,(iVar6 + 0xc),uStack14,uVar8);
    //   u_var2 = u_var4;
    //   param_3 = (u_var5 | u_var2);
    //   if ((u_var5 | u_var2) != 0x0) {
    //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var2,u_var5);
    //     u_var3 = pass1_1030_6fa0(CONCAT22(param_3,u_var2));
    //     param_4 = 0x1008;
    //     pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0,u_var3,0xe);
    //   }
    // }
    return;
}

pub fn pass1_1038_4cba() {
    pass1_1030_38b8();
    return;
}

pub fn pass1_1038_4cd0(param_1: u32, param_2: u32, param_3: u16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x1c) = param_3;
    (param_1 + 0x1e) = param_2;
    return;
}

pub fn pass1_1038_4cea(param_1: u32, param_2: U32Ptr, param_3: U32Ptr) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x1c);
    *param_2 = (param_1 + 0x1e);
    return;
}

pub fn pass1_1038_4d0e(param_1: u32, param_2: u16) {
    let i_var1: &mut Struct686;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x1a = i_var1.field_0x18;
    i_var1.field_0x18 = param_2;
    return;
}

pub fn load_string_1038_4d28(address: U32Ptr) -> String {
    return read_string_from_addr(CONCAT22(
        ((address + 0x1fc) as u16),
        ((address + 0x1fa) as u16),
    ));
}

pub fn pass1_1038_4d3c(param_1: u32, param_2: &mut String, param_3: u16) {
    let u_var1: u16;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    fn_ptr_1000_17ce(ctx, (i_var2 + 0x1fa), 0x1000);
    u_var1 = str_op_1008_60e8(param_2, param_3);
    (i_var2 + 0x1fa) = u_var1;
    (i_var2 + 0x1fc) = param_3;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_4d6e(param_1: u32, param_2: U32Ptr, param_3: u16, param_4: U32Ptr) {
    let pi_var1: U32Ptr;
    let ppcVar2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var5: u16;
    let extraout_DX_01: u16;
    let u_var6: u16;
    let iVar7: i16;
    let uVar8: u16;
    let uVar9: u32;
    let iStack30: i16;
    let uStack26: u32;
    let uStack14: u32;
    let uStack10: u32;
    let puStack6: u32;

    mem_op_1000_179c(0x18, param_4, 0x1000);
    if ((param_4 | param_3) == 0x0) {
        param_3 = 0x0;
        uVar8 = 0x0;
    } else {
        struct_op_1030_1cd8(CONCAT22(param_4, param_3), 0x5, 0x5);
        uVar8 = extraout_dx;
    }
    puStack6 = CONCAT22(uVar8, param_3);
    // uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    if ((iVar7 + 0xc) == 0x0) {
        param_3 = 0x0;
        u_var5 = 0x0;
    } else {
        ppcVar2 = ((iVar7 + 0xc) + 0x10);
        (**ppcVar2)();
        u_var5 = extraout_DX_00;
    }
    uStack10 = CONCAT22(u_var5, param_3);
    uStack14 = 0x0;
    loop {
        if (uStack10 <= uStack14) {
            return;
        }
        ppcVar2 = ((iVar7 + 0xc) + 0x4);
        uVar9 = uStack10;
        (**ppcVar2)();
        u_var3 = uVar9;
        u_var6 = extraout_DX_01 | u_var3;
        if (u_var6 != 0x0) {
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var3);
            uStack26 = CONCAT22(u_var6, u_var3);
            u_var4 = pass1_1030_6fa0(CONCAT22(u_var6, u_var3));
            iStack30 = 0x0;
            loop {
                pi_var1 = (param_2 + 0x4);
                if (*pi_var1 == iStack30 || *pi_var1 < iStack30) {
                    break;
                }
                if ((*param_2 + iStack30 * 0x2) == u_var4) {
                    uVar9 = struct_op_1030_73a8(uStack26);
                    if ((uVar9 + 0x12) == 0x5) {
                        ppcVar2 = (*puStack6 + 0xc);
                        (**ppcVar2)();
                    }
                    break;
                }
                iStack30 += 0x1;
            }
        }
        uStack14 += 0x1;
    }
}

pub fn pass1_1038_4e78(param_1: u16, param_2: U32Ptr, param_3: u32, param_4: U32Ptr) {
    let pi_var1: U32Ptr;
    let ppcVar2: u32;
    let u_var3: u16;
    let u_var4: u32;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var5: u16;
    let extraout_DX_01: u16;
    let u_var6: u16;
    let iVar7: i16;
    let uVar8: u16;
    let iStack26: i16;
    let uStack14: u32;
    let uStack10: u32;
    let puStack6: u32;

    mem_op_1000_179c(0x18, param_2, 0x1000);
    if ((param_2 | param_1) == 0x0) {
        param_1 = 0x0;
        uVar8 = 0x0;
    } else {
        struct_op_1030_1cd8(CONCAT22(param_2, param_1), 0x5, 0x5);
        uVar8 = extraout_dx;
    }
    puStack6 = CONCAT22(uVar8, param_1);
    // uVar8 = (param_3 >> 0x10);
    iVar7 = param_3;
    if ((iVar7 + 0xc) == 0x0) {
        param_1 = 0x0;
        u_var5 = 0x0;
    } else {
        ppcVar2 = ((iVar7 + 0xc) + 0x10);
        (**ppcVar2)();
        u_var5 = extraout_DX_00;
    }
    uStack10 = CONCAT22(u_var5, param_1);
    uStack14 = 0x0;
    loop {
        if (uStack10 <= uStack14) {
            return;
        }
        u_var4 = uStack10;
        pass1_1030_1d58((iVar7 + 0xc));
        u_var6 = u_var5 | u_var4;
        if (u_var6 != 0x0) {
            u_var3 = pass1_1030_6fa0(u_var4 & 0xffff | u_var5 << 0x10);
            iStack26 = 0x0;
            loop {
                pi_var1 = (param_4 + 0x4);
                if (*pi_var1 == iStack26 || *pi_var1 < iStack26) {
                    break;
                }
                if ((*param_4 + iStack26 * 0x2) == u_var3) {
                    ppcVar2 = (*puStack6 + 0xc);
                    (**ppcVar2)();
                    u_var6 = extraout_DX_01;
                    break;
                }
                iStack26 += 0x1;
            }
        }
        uStack14 += 0x1;
        u_var5 = u_var6;
    }
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_4f54(param_1: u32, param_2: u16, param_3: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let BVar3: bool;
    let u_var4: u32;
    let extraout_dx: u16;
    let u_var5: u16;
    let u_var6: u16;
    let iVar7: i16;
    let uVar8: u16;
    let uStack10: u32;
    let uStack6: u32;

    // uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    if ((iVar7 + 0xc) == 0x0) {
        param_3 = 0x0;
        u_var5 = 0x0;
    } else {
        ppcVar1 = ((iVar7 + 0xc) + 0x10);
        (**ppcVar1)();
        u_var5 = extraout_dx;
    }
    uStack6 = CONCAT22(u_var5, param_3);
    uStack10 = 0x0;
    loop {
        if (uStack6 <= uStack10) {
            return;
        }
        u_var4 = uStack6;
        pass1_1030_1d58((iVar7 + 0xc));
        u_var6 = u_var5 | u_var4;
        if (u_var6 != 0x0) {
            u_var2 = pass1_1030_6fa0(u_var4 & 0xffff | u_var5 << 0x10);
            BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var2, param_2);
            if (BVar3 != 0x0) {
                return;
            }
        }
        uStack10 += 0x1;
        u_var5 = u_var6;
    }
}

pub fn pass1_1038_4fd8(param_1: u16, param_2: u32, param_3: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u32;
    let extraout_dx: u16;
    let u_var4: u16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let uStack10: u32;
    let uStack6: u32;

    // uVar7 = (param_2 >> 0x10);
    iVar6 = param_2;
    if ((iVar6 + 0xc) == 0x0) {
        param_1 = 0x0;
        u_var4 = 0x0;
    } else {
        ppcVar1 = ((iVar6 + 0xc) + 0x10);
        (**ppcVar1)();
        u_var4 = extraout_dx;
    }
    uStack6 = CONCAT22(u_var4, param_1);
    uStack10 = 0x0;
    loop {
        if (uStack6 <= uStack10) {
            return;
        }
        u_var3 = uStack6;
        pass1_1030_1d58((iVar6 + 0xc));
        u_var5 = u_var4 | u_var3;
        if (u_var5 != 0x0) {
            u_var2 = pass1_1030_6fa0(u_var3 & 0xffff | u_var4 << 0x10);
            if (u_var2 == param_3) {
                return;
            }
        }
        uStack10 += 0x1;
        u_var4 = u_var5;
    }
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_5050(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u32;
    let extraout_dx: u16;
    let u_var4: u16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let uStack14: u32;
    let uStack10: u32;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0xc) == 0x0) {
        param_3 = 0x0;
        u_var4 = 0x0;
    } else {
        ppcVar1 = ((iVar6 + 0xc) + 0x10);
        (**ppcVar1)();
        u_var4 = extraout_dx;
    }
    uStack10 = CONCAT22(u_var4, param_3);
    // for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    //   u_var3 = uStack10;
    //   pass1_1030_1d58((iVar6 + 0xc));
    //   u_var5 = u_var4 | u_var3;
    //   if (u_var5 != 0x0) {
    //     u_var2 = pass1_1030_6fa0(u_var3 & 0xffff | u_var4 << 0x10);
    //     pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0,u_var2,param_2);
    //   }
    //   u_var4 = u_var5;
    // }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_50e0(param_1: u32, param_2: u16, param_3: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let BVar3: bool;
    let extraout_dx: u16;
    let u_var4: u16;
    let u_var5: u16;
    let iVar6: i16;
    let uVar7: u16;
    let uVar8: u32;
    let uStack14: u32;
    let uStack10: u32;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0xc) == 0x0) {
        param_3 = 0x0;
        u_var4 = 0x0;
    } else {
        ppcVar1 = ((iVar6 + 0xc) + 0x10);
        (**ppcVar1)();
        u_var4 = extraout_dx;
    }
    uStack10 = CONCAT22(u_var4, param_3);
    // for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    //   uVar8 = uStack10;
    //   pass1_1030_1d58((iVar6 + 0xc));
    //   u_var5 = u_var4 | uVar8;
    //   if (u_var5 != 0x0) {
    //     u_var2 = pass1_1030_6fa0(uVar8 & 0xffff | u_var4 << 0x10);
    //     BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0,u_var2,param_2);
    //     if (BVar3 != 0x0) {
    //       uVar8 = struct_op_1030_73a8(uVar8 & 0xffff | u_var4 << 0x10);
    //       u_var5 = (uVar8 >> 0x10);
    //     }
    //   }
    //   u_var4 = u_var5;
    // }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_518c(param_1: u32, param_2: u16, param_3: u16) {
    let pu_var1: U32Ptr;
    let u_var2: u32;
    let ppc_var3: u32;
    let u_var4: u16;
    let u_var5: u32;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let u_var6: u16;
    let iVar7: i16;
    let i_var8: i16;
    let i_var9: i16;
    let u_var10: u16;
    let u_var11: u16;
    let bVar12: bool;
    let uVar13: u32;
    let iStack34: i16;
    let uStack32: u32;
    let puStack28: u32;
    let uStack10: u32;
    let uStack6: u32;

    // u_var10 = (param_1 >> 0x10);
    iVar7 = param_1;
    if ((iVar7 + 0x206) == 0x0) {
        if ((iVar7 + 0xc) == 0x0) {
            param_2 = 0x0;
            u_var11 = 0x0;
        } else {
            u_var2 = (iVar7 + 0xc);
            ppc_var3 = ((iVar7 + 0xc) + 0x10);
            (**ppc_var3)(param_3, u_var2, (u_var2 >> 0x10));
            u_var11 = extraout_dx;
        }
        uStack6 = CONCAT22(u_var11, param_2);
        // for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
        //   u_var2 = (iVar7 + 0xc);
        //   ppc_var3 = ((iVar7 + 0xc) + 0x4);
        //   u_var5 = uStack6;
        //   (**ppc_var3)(param_3,u_var2,(u_var2 >> 0x10),uStack10,
        //               (uStack10 >> 0x10));
        //   u_var4 = u_var5;
        //   u_var6 = extraout_DX_00 | u_var4;
        //   if (u_var6 != 0x0) {
        //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var4,extraout_DX_00);
        //     param_3 = 0x1030;
        //     uVar13 = struct_op_1030_73a8(CONCAT22(u_var6,u_var4));
        //     u_var6 = (uVar13 >> 0x10);
        //     i_var8 = (uVar13 + 0x12);
        //     u_var4 = uVar13 + 0x14;
        //     u_var5 = u_var4;
        //     puStack28 = (uVar13 & 0xffff0000 | u_var4);
        //     uStack32 = 0x0;
        //     if ((i_var8 == 0x4) || (i_var8 == 0x5)) {
        //       u_var5 = *puStack28;
        //       uStack32 = u_var5;
        //     }
        //     if (uStack32 != 0x0) {
        //       for (iStack34 = 0x11; iStack34 < 0x25; iStack34 += 0x1) {
        //         if ((((iVar7 + 0x204) == 0x0) || (iStack34 == 0x23)) ||
        //            (iStack34 == 0x24)) {
        //           empty_1038_540a();
        //           i_var8 = iStack34 * 0x4;
        //           u_var11 = (uStack32 >> 0x10);
        //           i_var9 = uStack32;
        //           pu_var1 = (i_var8 + i_var9 + 0x2);
        //           bVar12 = *pu_var1 < u_var6;
        //           if ((bVar12 || *pu_var1 == u_var6) &&
        //              ((bVar12 ||
        //               (pu_var1 = (i_var8 + i_var9),
        //               *pu_var1 < u_var5 || *pu_var1 == u_var5)))) {
        //             pass1_1038_5770(param_1,(i_var8 + i_var9),iStack34);
        //           }
        //         }
        //       }
        //     }
        //   }
        // }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_52b8(
    param_1: u32,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
) {
    let u_var1: u32;
    let ppcVar2: u32;
    let u_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let extraout_dx: u16;
    let u_var6: u16;
    let extraout_DX_00: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u32;
    let iVar11: i16;
    let uVar12: u16;
    let uStack26: u16;
    let iStack24: i16;
    let uStack22: u32;
    let uStack14: u32;
    let uStack10: u32;
    let iVar10: &mut Struct601;

    i_var4 = -param_2;
    iVar11 = param_1;
    pass1_1038_5694(
        param_1,
        CONCAT22(-(param_2._2_2_ + (param_2 != 0x0)), i_var4),
        param_3,
    );
    if (param_3 != 0x24) {
        // uVar8 = (param_1 >> 0x10);
        if ((iVar11 + 0xc) == 0x0) {
            i_var4 = 0x0;
            u_var6 = 0x0;
        } else {
            u_var1 = (iVar11 + 0xc);
            ppcVar2 = ((iVar11 + 0xc) + 0x10);
            (**ppcVar2)(param_6, u_var1, (u_var1 >> 0x10));
            u_var6 = extraout_dx;
        }
        uStack10 = CONCAT22(u_var6, i_var4);
        // for (uStack14 = 0x0; u_var3 = param_2, uStack14 < uStack10; uStack14 += 0x1) {
        //   u_var1 = (iVar11 + 0xc);
        //   ppcVar2 = ((iVar11 + 0xc) + 0x4);
        //   uVar9 = uStack10;
        //   (**ppcVar2)(param_6,u_var1,(u_var1 >> 0x10),uStack14,
        //               (uStack14 >> 0x10));
        //   u_var5 = uVar9;
        //   uVar7 = extraout_DX_00 | u_var5;
        //   if (uVar7 != 0x0) {
        //     uVar12 = param_3;
        //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var5,extraout_DX_00);
        //     uStack22 = CONCAT22(uVar7,u_var5);
        //     param_6 = 0x1030;
        //     uVar9 = pass1_1030_7c28(CONCAT22(uVar7,u_var5),uVar12,u_var5,uVar7,param_7);
        //     uVar7 = (uVar9 >> 0x10);
        //     u_var5 = uVar9;
        //     if ((uVar7 | u_var5) != 0x0) {
        //       if (uVar9 < param_2) {
        //         param_2 -= uVar9;
        //         uStack26 = 0x0;
        //         iStack24 = 0x0;
        //       }
        //       else {
        //         uStack26 = u_var5 - param_2;
        //         iStack24 = (uVar7 - param_2._2_2_) - (u_var5 < param_2);
        //         param_2 = 0x0;
        //         uVar9 = u_var3;
        //       }
        //       param_6 = 0x1030;
        //       pass1_1030_7d1c(uStack22,uStack26,CONCAT22(param_3,iStack24),uVar9,
        //                       param_2._2_2_,param_4,param_5,param_7);
        //       if (param_2 == 0x0) {
        //         return;
        //       }
        //     }
        //   }
        // }
    }
    return;
}

pub fn pass1_1038_53ba(param_1: u32, param_2: i16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a2 + param_2 * 0x4) < (param_1 + 0x14e + param_2 * 0x4)) {
        return;
    }
    return;
}

pub fn empty_1038_540a() {
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_5464(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let u_var1: u32;
    let ppcVar2: u32;
    let u_var3: u16;
    let u_var4: u32;
    let extraout_dx: u16;
    let extraout_DX_00: u16;
    let extraout_DX_01: u16;
    let extraout_DX_02: u16;
    let u_var5: u16;
    let iVar6: i16;
    let iVar7: i16;
    let uVar8: u16;
    let uVar9: u16;
    let local_2e: u16;
    let uStack44: u16;
    let local_2a: u16;
    let uStack40: u16;
    let puStack34: u32;
    let uStack30: u16;
    let uStack28: u16;
    let puStack26: u32;
    let uStack22: u32;
    let uStack18: u16;
    let uStack16: u16;
    let uStack14: u32;
    let uStack10: u32;
    let uStack6: u32;

    pass1_1038_56ba(param_1);
    pass1_1038_57c0(param_1);
    // uVar8 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0xc) == 0x0) {
        param_2 = 0x0;
        u_var5 = 0x0;
    } else {
        u_var1 = (iVar6 + 0xc);
        ppcVar2 = ((iVar6 + 0xc) + 0x10);
        (**ppcVar2)(param_3, u_var1, (u_var1 >> 0x10));
        u_var5 = extraout_dx;
    }
    uStack10 = CONCAT22(u_var5, param_2);
    // for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    //   u_var1 = (iVar6 + 0xc);
    //   ppcVar2 = ((iVar6 + 0xc) + 0x4);
    //   u_var4 = uStack10;
    //   (**ppcVar2)(param_3,u_var1,(u_var1 >> 0x10),uStack14,
    //               (uStack14 >> 0x10));
    //   u_var3 = u_var4;
    //   u_var5 = extraout_DX_02 | u_var3;
    //   uStack18 = u_var3;
    //   uStack16 = extraout_DX_02;
    //   if (u_var5 != 0x0) {
    //     param_3 = &USHORT_1050_1028;
    //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var3,extraout_DX_02);
    //     uStack22 = CONCAT22(u_var5,u_var3);
    //     puStack26 = *(u32 **)(u_var3 + 0x22);
    //     if (((u_var3 + 0x24) | puStack26) == 0x0) {
    //       uStack28 = 0x0;
    //     }
    //     else {
    //       uStack28 = (puStack26 + 0x4);
    //     }
    //     for (uStack30 = 0x0; uStack30 < uStack28; uStack30 += 0x1) {
    //       param_3 = 0x1020;
    //       pass1_1020_bb16(puStack26,
    //                       CONCAT13((param_4 >> 0x8),
    //                                         CONCAT12(param_4,&local_2e)),
    //                       CONCAT22(param_4,&local_2a),uStack30);
    //       if (CONCAT22(uStack44,local_2e) != 0x0) {
    //         pass1_1038_5694(param_1,CONCAT22(uStack44,local_2e),local_2a);
    //       }
    //     }
    //     uVar9 = (uStack22 >> 0x10);
    //     puStack34 = (uStack22 + 0x1e);
    //     u_var5 = (uStack22 + 0x20);
    //     u_var3 = u_var5 | puStack34;
    //     if (u_var3 == 0x0) {
    //       u_var3 = 0x0;
    //     }
    //     else {
    //       ppcVar2 = (*puStack34 + 0x10);
    //       (**ppcVar2)(param_3,puStack34,u_var5);
    //       u_var5 = extraout_DX_00;
    //     }
    //     uStack28 = u_var3;
    //     for (uStack30 = 0x0; uStack30 < uStack28; uStack30 += 0x1) {
    //       ppcVar2 = (*puStack34 + 0x4);
    //       u_var3 = uStack28;
    //       (**ppcVar2)(param_3,puStack34,(puStack34 >> 0x10),uStack30,0x0);
    //       u_var5 = extraout_DX_01 | u_var3;
    //       local_2a = u_var3;
    //       uStack40 = extraout_DX_01;
    //       if (u_var5 != 0x0) {
    //         param_3 = &USHORT_1050_1028;
    //         pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var3,extraout_DX_01);
    //         iVar7 = (u_var3 + 0xc) * 0x4;
    //         (iVar6 + iVar7 + 0x14e) = (iVar6 + 0x14e + iVar7) + 0x1;
    //       }
    //     }
    //   }
    // }
    u_var4 = uStack10;
    pass1_1030_38f2((iVar6 + 0x1f6), 0x3, param_4);
    u_var3 = u_var4;
    uStack6._0_2_ = u_var3;
    uStack6._2_2_ = u_var5;
    pass1_1030_38f2((iVar6 + 0x1f6), 0x4, param_4);
    uStack6 = CONCAT22(
        uStack6._2_2_ + u_var5 + CARRY2(uStack6, u_var3),
        uStack6 + u_var3,
    );
    if (uStack6 == 0x0) {
        pass1_1030_38f2((iVar6 + 0x1f6), 0x2, param_4);
        uStack6 = CONCAT22(u_var5, u_var3);
    }
    u_var1 = (iVar6 + 0x1f6);
    uStack6 += (u_var1 + 0x170);
    pass1_1038_5694(param_1, uStack6, 0x24);
    return;
}

pub fn pass1_1038_565e(param_1: u16, param_2: U32Ptr, param_3: u32) -> u32 {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u32;
    let local_4: [u8; 2];

    // u_var2 = (param_3 >> 0x10);
    i_var1 = param_3;
    u_var3 = pass1_1030_8e3c(
        param_1,
        local_4,
        param_2,
        CONCAT22(param_1, local_4),
        (i_var1 + 0x4),
    );
    pass1_1038_582c(param_3, u_var3);
    return CONCAT22((i_var1 + 0x16), (i_var1 + 0x14));
}

pub fn pass1_1038_5694(param_1: u32, param_2: i32, param_3: i16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + param_3 * 0x4 + 0x26) = (param_1 + 0x26 + param_3 * 0x4) + param_2;
    return;
}

pub fn pass1_1038_56ba(param_1: u32) {
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x26)), 0x0, 0x94);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_56d6(param_1: u32, param_2: i16) {
    let ppcVar1: u32;
    let i_var2: i16;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let u_var5: u32;
    let extraout_dx: u16;
    let u_var6: u16;
    let extraout_DX_00: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let uStack10: u32;
    let uStack6: u32;

    i_var2 = param_1;
    uVar9 = 0x1000;
    pu_var3 = pass1_1000_4906((param_1 & 0xffff0000 | (i_var2 + 0xba)), 0x0, 0x94);
    if (param_2 != 0x0) {
        // uVar8 = (param_1 >> 0x10);
        if ((i_var2 + 0xc) == 0x0) {
            pu_var3 = 0x0;
            u_var6 = 0x0;
        } else {
            ppcVar1 = ((i_var2 + 0xc) + 0x10);
            (**ppcVar1)();
            u_var6 = extraout_dx;
        }
        uStack6 = CONCAT22(u_var6, pu_var3);
        // for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
        //   ppcVar1 = ((i_var2 + 0xc) + 0x4);
        //   u_var5 = uStack6;
        //   (**ppcVar1)(uVar9,(i_var2 + 0xc));
        //   u_var4 = u_var5;
        //   uVar7 = extraout_DX_00 | u_var4;
        //   if (uVar7 != 0x0) {
        //     pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,u_var4,extraout_DX_00);
        //     uVar9 = 0x1030;
        //     pass1_1030_72d0(CONCAT22(uVar7,u_var4));
        //   }
        // }
    }
    return;
}

pub fn pass1_1038_5770(param_1: u32, param_2: i32, param_3: i16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + param_3 * 0x4 + 0xba) = (param_1 + 0xba + param_3 * 0x4) + param_2;
    return;
}

pub fn pass1_1038_5798(param_1: u32, param_2: i32, param_3: i16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + param_3 * 0x4 + 0x14e) = (param_1 + 0x14e + param_3 * 0x4) + param_2;
    return;
}

pub fn pass1_1038_57c0(param_1: u32) {
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x14e)), 0x0, 0x54);
    return;
}

pub fn pass1_1038_57dc(param_1: u32, param_2: i32, param_3: i16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + param_3 * 0x4 + 0x1a2) = (param_1 + 0x1a2 + param_3 * 0x4) + param_2;
    return;
}

pub fn pass1_1038_5804(param_1: u32, param_2: i32, param_3: i16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    (param_1 + param_3 * 0x4 + 0x1a2) = (param_1 + 0x1a2 + param_3 * 0x4) - param_2;
    return;
}

pub fn pass1_1038_582c(param_1: u32, param_2: u32) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 0x14);
    u_var2 = (i_var4 + 0x16);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    (i_var4 + 0x14) = param_2;
    return;
}

pub fn pass1_1038_5860(param_1: u32, param_2: u16, param_3: u32, param_4: i16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let u_var3: u32;
    let extraout_dx: u16;
    let extraout_DX_00: i16;
    let i_var4: i16;
    let u_var5: u16;
    let uStack14: u32;
    let i_stack6: i16;
    let i_stack4: i16;

    if (param_4 == 0x0) {
        // u_var5 = (param_1 >> 0x10);
        i_var4 = param_1;
        ppcVar1 = ((i_var4 + 0xc) + 0x10);
        u_var2 = param_3;
        (**ppcVar1)();
        u_var2 = u_var2 & 0xffff | extraout_dx << 0x10;
        // for (uStack14 = 0x0; uStack14 < u_var2; uStack14 += 0x1) {
        //   ppcVar1 = ((i_var4 + 0xc) + 0x4);
        //   u_var3 = u_var2;
        //   (**ppcVar1)();
        //   i_stack6 = param_3;
        //   if ((u_var3 == i_stack6) &&
        //      (i_stack4 = (param_3 >> 0x10), extraout_DX_00 == i_stack4)) {
        //     return;
        //   }
        // }
        ppcVar1 = ((i_var4 + 0xc) + 0xc);
        (**ppcVar1)();
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_58e6(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: i16,
    param_7: u16,
) {
    let i_var1: i16;
    let ppcVar2: u32;
    let u_var3: u32;
    let Bvar4: bool;
    let pu_var5: u32;
    let u_var6: u16;
    let iVar7: i16;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u32;
    let local_12: u32;
    let iStack14: i16;
    let iStack12: i16;
    let uStack6: u32;

    // for (uStack6 = 0x0; uStack6 < param_3; uStack6 += 0x1) {
    //   uVar9 = (param_4 >> 0x10);
    //   iVar7 = param_4;
    //   if (((uStack6 * 0x4 + iVar7) != 0x0) &&
    //      (u_var3 = (uStack6 * 0x4 + iVar7),
    //      BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0,(u_var3 + 0xc),0x2e)
    //      , BVar4 != 0x0)) {
    //     uVar8 = (param_5 >> 0x10);
    //     i_var1 = (uStack6 * 0x4 + param_5);
    //     uVar8 = (uStack6 * 0x4 + param_5 + 0x2);
    //     local_12 = (i_var1 + 0xc);
    //     iStack12 = (i_var1 + 0x10);
    //     iStack14 = iStack12;
    //     if (iStack12 == param_6) {
    //       iStack14 = iStack12 + -0x1;
    //       u_var10 = pass1_1028_bb24((uStack6 * 0x4 + iVar7));
    //       u_var6 = (u_var10 >> 0x10);
    //       pu_var5 = &local_12;
    //       pass1_1030_627e(param_7,pu_var5,u_var6,PTR_LOOP_1050_5740,
    //                       CONCAT22(param_7,pu_var5),
    //                       u_var10 & 0xffff | u_var6 << 0x10);
    //       pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2,pu_var5,u_var6);
    //       if ((u_var6 | pu_var5) != 0x0) {
    //         u_var10 = struct_op_1030_73a8(CONCAT22(u_var6,pu_var5));
    //         u_var6 = (u_var10 + 0x1a);
    //         if (((u_var6 & 0x2) != 0x0) && ((u_var6 & 0x1) != 0x0)) {
    //           u_var3 = (uStack6 * 0x4 + iVar7);
    //           (u_var3 + 0x1a) = 0x3;
    //           ppcVar2 = (
    //                                     (uStack6 * 0x4 + iVar7) + 0x28);
    //           (**ppcVar2)();
    //         }
    //       }
    //     }
    //   }
    // }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_5a16(param_1: u16, param_2: u16, param_3: u32, param_4: u32) {
    let ppcVar1: u32;
    let u_var2: u32;
    let BVar3: bool;
    let i_var4: i16;
    let u_var5: u16;
    let uStack6: u32;

    // for (uStack6 = 0x0; uStack6 < param_3; uStack6 += 0x1) {
    //   u_var5 = (param_4 >> 0x10);
    //   i_var4 = param_4;
    //   if (((uStack6 * 0x4 + i_var4) != 0x0) &&
    //      (u_var2 = (uStack6 * 0x4 + i_var4),
    //      BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0,(u_var2 + 0xc),0x2f)
    //      , BVar3 != 0x0)) {
    //     u_var2 = (uStack6 * 0x4 + i_var4);
    //     (u_var2 + 0x1a) = 0x3;
    //     ppcVar1 = ((uStack6 * 0x4 + i_var4)
    //                        + 0x28);
    //     (**ppcVar1)();
    //   }
    // }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_5a96(param_1: u16, param_2: u16, param_3: u32, param_4: u32) {
    let ppcVar1: u32;
    let u_var2: u32;
    let BVar3: bool;
    let i_var4: i16;
    let u_var5: u16;
    let uStack6: u32;

    // for (uStack6 = 0x0; uStack6 < param_3; uStack6 += 0x1) {
    //   u_var5 = (param_4 >> 0x10);
    //   i_var4 = param_4;
    //   if (((uStack6 * 0x4 + i_var4) != 0x0) &&
    //      (u_var2 = (uStack6 * 0x4 + i_var4),
    //      BVar3 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0,(u_var2 + 0xc),0x2c)
    //      , BVar3 != 0x0)) {
    //     ppcVar1 = ((uStack6 * 0x4 + i_var4)
    //                        + 0x54);
    //     (**ppcVar1)();
    //     if (BVar3 != 0x0) {
    //       u_var2 = (i_var4 + uStack6 * 0x4);
    //       (u_var2 + 0x1a) = 0x3;
    //       ppcVar1 = (
    //                                 (uStack6 * 0x4 + i_var4) + 0x28);
    //       (**ppcVar1)();
    //       u_var2 = (i_var4 + uStack6 * 0x4);
    //       (u_var2 + 0x1a) = 0x2;
    //     }
    //   }
    // }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_5b3c(param_1: u16, param_2: u16, param_3: u32, param_4: u32) {
    let ppcVar1: u32;
    let u_var2: u32;
    let u_var3: u32;
    let Bvar4: bool;
    let iVar5: i16;
    let u_var6: u16;
    let uStack6: u32;

    // for (uStack6 = 0x0; uStack6 < param_3; uStack6 += 0x1) {
    //   u_var6 = (param_4 >> 0x10);
    //   iVar5 = param_4;
    //   if ((((uStack6 * 0x4 + iVar5) != 0x0) &&
    //       (u_var2 = (uStack6 * 0x4 + iVar5),
    //       BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0,(u_var2 + 0xc),0x2d
    //                              ), BVar4 != 0x0)) &&
    //      (ppcVar1 = (
    //                                 (uStack6 * 0x4 + iVar5) + 0x50),
    //      (**ppcVar1)(), BVar4 != 0x0)) {
    //     u_var2 = (uStack6 * 0x4 + iVar5);
    //     u_var3 = (uStack6 * 0x4 + iVar5);
    //     (u_var3 + 0x1a) = (u_var2 + 0x1a) | 0x1;
    //     ppcVar1 = ((uStack6 * 0x4 + iVar5)
    //                        + 0x28);
    //     (**ppcVar1)();
    //   }
    // }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_5be8(
    param_1: u32,
    param_2: u16,
    param_3: i16,
    param_4: U32Ptr,
    param_5: u16,
    param_6: u16,
    param_7: u16,
) -> u16 {
    let i_var1: i16;
    let u_var2: u16;
    let i_var3: i16;
    let Bvar4: bool;
    let u_var5: u16;
    let u_var6: u32;
    let iStack14: i16;
    let uStack10: u32;

    pass1_1030_627e(
        param_7,
        param_5,
        param_6,
        ctx.PTR_LOOP_1050_5740,
        param_4,
        (param_1 + 0x8),
    );
    u_var5 = param_6 | param_5;
    if (u_var5 != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_5, param_6);
        uStack10 = CONCAT22(u_var5, param_5);
        iStack14 = 0x7a;
        if (0x0 < (param_4 + 0x4)) {
            if (param_3 == 0x7b) {
                param_3 = 0x7e;
            } else {
                if (param_3 == 0x7c) {
                    param_3 = 0x7d;
                }
            }
            iStack14 = 0x7f;
        }
        u_var6 = struct_op_1030_73a8(uStack10);
        // u_var2 = (u_var6 >> 0x10);
        i_var3 = u_var6;
        if (((((i_var3 + 0x1a) & param_2) == 0x0)
            && ((
                i_var1 = (i_var3 + 0xc),
                i_var1 == iStack14 || (i_var1 == param_3),
            ) || (
                BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, i_var1, 0x2b),
                BVar4 != 0x0,
            )))
            && ((i_var3 + 0x12) != 0x7))
        {
            (i_var3 + 0x1a) = (i_var3 + 0x1a) | param_2;
            return 0x1;
        }
    }
    return 0x0;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1038_5cc6(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: i16,
    param_6: u16,
) {
    let u_var1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let unaff_SS: u16;
    let puVar6: U32Ptr;
    let local_14: i16;
    let local_12: i16;
    let local_10: i16;
    let uStack14: u32;
    let local_a: i16;
    let iStack8: i16;
    let i_stack4: i16;

    puVar6 = clear_struct_1008_3e38(CONCAT22(unaff_SS, &local_a));
    // u_var4 = (puVar6 >> 0x10);
    loop {
        i_stack4 = 0x0;
        // for (uStack14 = 0x0; uStack14 < param_2; uStack14 += 0x1) {
        //   u_var5 = (param_4 >> 0x10);
        //   if ((uStack14 * 0x4 + param_4) != 0x0) {
        //     u_var1 = (uStack14 * 0x4 + param_4);
        //     pass1_1008_3f62(CONCAT22(unaff_SS,&local_a),
        //                     (u_var1 & 0xffff0000 | (u_var1 + 0xc)));
        //     pass1_1008_3eb4(CONCAT22(unaff_SS,&local_a),
        //                     CONCAT22(unaff_SS,&local_14),
        //                     CONCAT22(unaff_SS,&local_12),
        //                     CONCAT22(unaff_SS,&local_10));
        //     if (local_14 == param_5) {
        //       u_var5 = (param_3 >> 0x10);
        //       if (((uStack14 * 0x4 + param_3) != 0x0) &&
        //          (u_var2 = (uStack14 * 0x4 + param_3),
        //          ((u_var2 + 0x1a) & param_6) != 0x0)) {
        //         iStack8 = local_12 + -0x1;
        //         u_var3 = pass1_1038_5be8(param_1,param_6,0x7b,
        //                                 CONCAT22(unaff_SS,&local_a),&local_a,
        //                                 u_var4,unaff_SS);
        //         if (u_var3 != 0x0) {
        //           i_stack4 = 0x1;
        //         }
        //         iStack8 = local_12 + 0x1;
        //         u_var3 = pass1_1038_5be8(param_1,param_6,0x7b,
        //                                 CONCAT22(unaff_SS,&local_a),&local_a,
        //                                 u_var4,unaff_SS);
        //         if (u_var3 != 0x0) {
        //           i_stack4 = 0x1;
        //         }
        //         iStack8 = local_12;
        //         local_a = local_10 + -0x1;
        //         u_var3 = pass1_1038_5be8(param_1,param_6,0x7c,
        //                                 CONCAT22(unaff_SS,&local_a),&local_a,
        //                                 u_var4,unaff_SS);
        //         if (u_var3 != 0x0) {
        //           i_stack4 = 0x1;
        //         }
        //         local_a = local_10 + 0x1;
        //         u_var3 = pass1_1038_5be8(param_1,param_6,0x7c,
        //                                 CONCAT22(unaff_SS,&local_a),&local_a,
        //                                 u_var4,unaff_SS);
        //         if (u_var3 != 0x0) {
        //           i_stack4 = 0x1;
        //         }
        //       }
        //     }
        //   }
        // }
        if (i_stack4 != 0x0) == false {
            break;
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1038_5e16(param_1: u32, param_2: u32, param_3: i16, param_4: u16, param_5: u16) {
    let b_var1: bool;
    let pu_var2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let local_14: [u32; 0x2];
    let local_c: u32;
    let puStack6: u32;

    pass1_1030_16d6(param_1, param_2, param_5);
    if (param_3 != 0x0) {
        // u_var4 = (param_1 >> 0x10);
        i_var3 = param_1;
        pu_var2 = (i_var3 + 0xc);
        puStack6 = pu_var2;
        pass1_1008_7898(param_2, pu_var2, pu_var2, param_4, 0x1008, param_5);
        if (pu_var2 != 0x0) {
            local_14[0] = (i_var3 + 0x10);
            u_var5 = param_2;
            // u_var6 = (param_2 >> 0x10);
            b_var1 = write_to_file_1008_7e1c(u_var5, u_var6, local_14, param_5, 0x4, 0x1008);
            if (b_var1 != 0x0) {
                local_c._0_2_ = (i_var3 + 0x18);
                b_var1 = write_to_file_1008_7e1c(u_var5, u_var6, &local_c, param_5, 0x2, 0x1008);
                if (b_var1 != 0x0) {
                    local_c._0_2_ = (i_var3 + 0x1a);
                    b_var1 =
                        write_to_file_1008_7e1c(u_var5, u_var6, &local_c, param_5, 0x2, 0x1008);
                    if (b_var1 != 0x0) {
                        local_c = CONCAT22(local_c._2_2_, (i_var3 + 0x1c));
                        b_var1 =
                            write_to_file_1008_7e1c(u_var5, u_var6, &local_c, param_5, 0x2, 0x1008);
                        if (b_var1 != 0x0) {
                            local_c = (i_var3 + 0x1e);
                            b_var1 = write_to_file_1008_7e1c(
                                u_var5, u_var6, &local_c, param_5, 0x4, 0x1008,
                            );
                            if (b_var1 != 0x0) {
                                local_c = local_c & 0xffff0000 | (i_var3 + 0x22);
                                b_var1 = write_to_file_1008_7e1c(
                                    u_var5, u_var6, &local_c, param_5, 0x2, 0x1008,
                                );
                                if (b_var1 != 0x0) {
                                    local_c = local_c & 0xffff0000 | (i_var3 + 0x24);
                                    b_var1 = write_to_file_1008_7e1c(
                                        u_var5, u_var6, &local_c, param_5, 0x2, 0x1008,
                                    );
                                    if (b_var1 != 0x0) {
                                        b_var1 = write_to_file_1008_7e1c(
                                            u_var5,
                                            u_var6,
                                            i_var3 + 0x26,
                                            u_var4,
                                            0x94,
                                            0x1008,
                                        );
                                        if (b_var1 != 0x0) {
                                            b_var1 = write_to_file_1008_7e1c(
                                                u_var5,
                                                u_var6,
                                                i_var3 + 0x14e,
                                                u_var4,
                                                0x54,
                                                0x1008,
                                            );
                                            if (b_var1 != 0x0) {
                                                b_var1 = write_to_file_1008_7e1c(
                                                    u_var5,
                                                    u_var6,
                                                    i_var3 + 0x1a2,
                                                    u_var4,
                                                    0x54,
                                                    0x1008,
                                                );
                                                if (b_var1 != 0x0) {
                                                    write_to_file_1030_32e4(
                                                        (i_var3 + 0x1f6),
                                                        param_2,
                                                        param_5,
                                                    );
                                                    b_var1 = pass1_1008_7c2a(
                                                        param_2,
                                                        (i_var3 + 0x1fa),
                                                        0x1008,
                                                    );
                                                    if (b_var1 != 0x0) {
                                                        local_c =
                                                            local_c & 0xffff0000 | (i_var3 + 0x1fe);
                                                        b_var1 = write_to_file_1008_7e1c(
                                                            u_var5, u_var6, &local_c, param_5, 0x2,
                                                            0x1008,
                                                        );
                                                        if (b_var1 != 0x0) {
                                                            local_c = (i_var3 + 0x200);
                                                            b_var1 = write_to_file_1008_7e1c(
                                                                u_var5, u_var6, &local_c, param_5,
                                                                0x4, 0x1008,
                                                            );
                                                            if (b_var1 != 0x0) {
                                                                local_c = local_c & 0xffff0000
                                                                    | (i_var3 + 0x204);
                                                                b_var1 = write_to_file_1008_7e1c(
                                                                    u_var5, u_var6, &local_c,
                                                                    param_5, 0x2, 0x1008,
                                                                );
                                                                if (b_var1 != 0x0) {
                                                                    local_c = local_c & 0xffff0000
                                                                        | (i_var3 + 0x206);
                                                                    b_var1 =
                                                                        write_to_file_1008_7e1c(
                                                                            u_var5, u_var6,
                                                                            &local_c, param_5, 0x2,
                                                                            0x1008,
                                                                        );
                                                                    if (b_var1 != 0x0) {
                                                                        local_c = local_c
                                                                            & 0xffff0000
                                                                            | (i_var3 + 0x208);
                                                                        b_var1 =
                                                                            write_to_file_1008_7e1c(
                                                                                u_var5, u_var6,
                                                                                &local_c, param_5,
                                                                                0x2, 0x1008,
                                                                            );
                                                                        if (b_var1 != 0x0) {
                                                                            local_c = local_c
                                                                                & 0xffff0000
                                                                                | (i_var3 + 0x20a);
                                                                            b_var1 = write_to_file_1008_7e1c
                                                        (u_var5,u_var6,&local_c,
                                                         param_5,0x2,0x1008);
                                                                            if (b_var1 != 0x0) {
                                                                                local_c = local_c
                                                                                    & 0xffff0000
                                                                                    | (i_var3
                                                                                        + 0x20c);
                                                                                b_var1 = write_to_file_1008_7e1c
                                                          (u_var5,u_var6,&local_c,
                                                           param_5,0x2,0x1008);
                                                                                if (b_var1 != 0x0) {
                                                                                    local_c = local_c & 0xffff0000 |
                                                    (i_var3 + 0x20e);
                                                                                    b_var1 = write_to_file_1008_7e1c
                                                            (u_var5,u_var6,&local_c,
                                                             param_5,0x2,0x1008);
                                                                                    if (b_var1
                                                                                        != 0x0)
                                                                                    {
                                                                                        local_c = local_c & 0xffff0000 |
                                                      (i_var3 + 0x214);
                                                                                        b_var1 = write_to_file_1008_7e1c
                                                              (u_var5,u_var6,
                                                               &local_c,param_5,
                                                               0x2,0x1008);
                                                                                        if (b_var1
                                                                                            != 0x0)
                                                                                        {
                                                                                            local_c = (i_var3 + 0x216);
                                                                                            b_var1 = write_to_file_1008_7e1c
                                                                (u_var5,u_var6,
                                                                 &local_c,param_5,
                                                                 0x4,0x1008);
                                                                                            if (b_var1 != 0x0) {
                                                return;
                                              }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return;
}

pub fn file_1038_6118(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16) {
    let u_var1: u16;
    let pu_var2: u32;
    let BVar3: bool;
    let u_var4: u16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let i_var9: &mut Struct429;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let SVar11: SEGPTR;
    let paStack1046: &mut Struct18;
    let uStack1042: u16;
    let local_408: [u8; 400];
    let local_8: u16;
    let local_6: u32;

    file_1030_1730(param_1, param_2);
    if (param_3 == 0x0) {
        return;
    }
    local_6 = 0x0;
    pu_var2 = &local_6;
    file_1008_7548(param_2, CONCAT22(param_5, pu_var2), 0x1008, param_5);
    if (pu_var2 != 0x0) {
        // uVar8 = (param_1 >> 0x10);
        i_var9 = param_1;
        i_var9.field_0xc = local_6;
        uVar9 = param_2;
        // u_var10 = (param_2 >> 0x10);
        BVar3 = read_file_1008_7dee(uVar9, u_var10, &i_var9.field_0x10, 0x0, uVar8, 0x4, 0x1008);
        if (((((BVar3 != 0x0)
            && (
                BVar3 = read_file_1008_7dee(
                    uVar9,
                    u_var10,
                    &i_var9.field_0x18,
                    0x0,
                    uVar8,
                    0x2,
                    0x1008,
                ),
                BVar3 != 0x0,
            ))
            && (
                BVar3 = read_file_1008_7dee(
                    uVar9,
                    u_var10,
                    &i_var9.field_0x1a,
                    0x0,
                    uVar8,
                    0x2,
                    0x1008,
                ),
                BVar3 != 0x0,
            ))
            && ((
                BVar3 = read_file_1008_7dee(uVar9, u_var10, &local_8, 0x0, param_5, 0x2, 0x1008),
                BVar3 != 0x0
                    && (
                        BVar3 = read_file_1008_7dee(
                            uVar9,
                            u_var10,
                            &i_var9.field_0x1e,
                            0x0,
                            uVar8,
                            0x4,
                            0x1008,
                        ),
                        BVar3 != 0x0,
                    ),
            )))
            && (
                BVar3 = read_file_1008_7dee(
                    uVar9,
                    u_var10,
                    &i_var9.field_0x22,
                    0x0,
                    uVar8,
                    0x2,
                    0x1008,
                ),
                BVar3 != 0x0,
            ))
        {
            i_var9.field_0x1c = local_8;
            BVar3 =
                read_file_1008_7dee(uVar9, u_var10, &i_var9.field_0x24, 0x0, uVar8, 0x2, 0x1008);
            if ((BVar3 != 0x0)
                && (
                    u_var4 = read_file_1008_7dee(
                        uVar9,
                        u_var10,
                        &i_var9.field_0x26,
                        0x0,
                        uVar8,
                        0x94,
                        0x1008,
                    ),
                    u_var4 != 0x0,
                ))
            {
                if (ctx.PTR_LOOP_1050_0312 < 0x2) {
                    u_var5 = 0x54;
                    SVar11 = 0x54;
                    mem_op_1000_179c(0x54, param_4, 0x1000);
                    paStack1046 = CONCAT22(param_4, u_var4);
                    BVar3 = read_file_1008_7dee(
                        uVar9, u_var10, u_var4, u_var5, param_4, SVar11, 0x1008,
                    );
                    if (BVar3 == 0x0) {
                        //LAB_1038_626a:
                        ctx.PTR_LOOP_1050_0310 = 0x6d2;
                        fn_ptr_1000_17ce(ctx, paStack1046, 0x1000);
                        return;
                    }
                    uStack1042 = 0x0;
                    loop {
                        u_var5 = switch_1008_72bc(uVar9, u_var10, uStack1042);
                        u_var1 = (uStack1042 * 0x4 + u_var4 + 0x2);
                        (&i_var9.field_0x14e + u_var5 * 0x4) = (uStack1042 * 0x4 + u_var4);
                        (&i_var9.field_0x150 + u_var5 * 0x4) = u_var1;
                        uStack1042 += 0x1;
                        if (uStack1042 < 0x15) == false {
                            break;
                        }
                    }
                    BVar3 = read_file_1008_7dee(uVar9, u_var10, u_var4, 0x0, param_4, 0x54, 0x1008);
                    if (BVar3 == 0x0) {
                        // goto
                        // LAB_1038_626a;
                    }
                    uStack1042 = 0x0;
                    loop {
                        u_var5 = switch_1008_72bc(uVar9, u_var10, uStack1042);
                        puVar7 = (uStack1042 * 0x4 + u_var4 + 0x2);
                        (&i_var9.field_0x1a2 + u_var5 * 0x4) = (uStack1042 * 0x4 + u_var4);
                        (&i_var9.field_0x1a4 + u_var5 * 0x4) = puVar7;
                        uStack1042 += 0x1;
                        if (uStack1042 < 0x15) == false {
                            break;
                        }
                    }
                    fn_ptr_1000_17ce(ctx, paStack1046, 0x1000);
                    param_4 = puVar7;
                } else {
                    BVar3 = read_file_1008_7dee(
                        uVar9,
                        u_var10,
                        &i_var9.field_0x14e,
                        0x0,
                        uVar8,
                        0x54,
                        0x1008,
                    );
                    if (BVar3 == 0x0) {
                        ctx.PTR_LOOP_1050_0310 = 0x6d2;
                        return;
                    }
                    BVar3 = read_file_1008_7dee(
                        uVar9,
                        u_var10,
                        &i_var9.field_0x1a2,
                        0x0,
                        uVar8,
                        0x54,
                        0x1008,
                    );
                    if (BVar3 == 0x0) {
                        ctx.PTR_LOOP_1050_0310 = 0x6d2;
                        return;
                    }
                }
                read_file_1030_33f0(i_var9.field_0x1f6, param_2);
                puVar6 = local_408;
                read_file_1008_7c6e(uVar9, u_var10, CONCAT22(param_5, puVar6), 0x1008);
                if (puVar6 != 0x0) {
                    u_var4 = str_op_1008_60e8(CONCAT22(param_5, local_408), param_4);
                    i_var9.field_0x1fa = u_var4;
                    i_var9.field_0x1fc = param_4;
                    BVar3 = read_file_1008_7dee(
                        uVar9,
                        u_var10,
                        &i_var9.field_0x1fe,
                        0x0,
                        uVar8,
                        0x2,
                        0x1008,
                    );
                    if (((((BVar3 != 0x0)
                        && (
                            BVar3 = read_file_1008_7dee(
                                uVar9,
                                u_var10,
                                CONCAT11((param_1 >> 0x8) + '\x02', param_1),
                                0x0,
                                uVar8,
                                0x4,
                                0x1008,
                            ),
                            BVar3 != 0x0,
                        ))
                        && (
                            BVar3 = read_file_1008_7dee(
                                uVar9,
                                u_var10,
                                &i_var9.field_0x204,
                                0x0,
                                uVar8,
                                0x2,
                                0x1008,
                            ),
                            BVar3 != 0x0,
                        ))
                        && ((
                            BVar3 = read_file_1008_7dee(
                                uVar9,
                                u_var10,
                                &i_var9.field_0x206,
                                0x0,
                                uVar8,
                                0x2,
                                0x1008,
                            ),
                            BVar3 != 0x0
                                && (
                                    BVar3 = read_file_1008_7dee(
                                        uVar9,
                                        u_var10,
                                        &i_var9.field_0x208,
                                        0x0,
                                        uVar8,
                                        0x2,
                                        0x1008,
                                    ),
                                    BVar3 != 0x0,
                                ),
                        ) && ((
                            BVar3 = read_file_1008_7dee(
                                uVar9,
                                u_var10,
                                &i_var9.field_0x20a,
                                0x0,
                                uVar8,
                                0x2,
                                0x1008,
                            ),
                            BVar3 != 0x0
                                && ((
                                    BVar3 = read_file_1008_7dee(
                                        uVar9,
                                        u_var10,
                                        &i_var9.field_0x20c,
                                        0x0,
                                        uVar8,
                                        0x2,
                                        0x1008,
                                    ),
                                    BVar3 != 0x0
                                        && (
                                            BVar3 = read_file_1008_7dee(
                                                uVar9,
                                                u_var10,
                                                &i_var9.field_0x20e,
                                                0x0,
                                                uVar8,
                                                0x2,
                                                0x1008,
                                            ),
                                            BVar3 != 0x0,
                                        ),
                                )),
                        ))))
                        && (ctx.PTR_LOOP_1050_0312 < 0x2
                            || ((
                                BVar3 = read_file_1008_7dee(
                                    uVar9,
                                    u_var10,
                                    &i_var9.field_0x214,
                                    0x0,
                                    uVar8,
                                    0x2,
                                    0x1008,
                                ),
                                BVar3 != 0x0
                                    && (
                                        BVar3 = read_file_1008_7dee(
                                            uVar9,
                                            u_var10,
                                            &i_var9.field_0x216,
                                            0x0,
                                            uVar8,
                                            0x4,
                                            0x1008,
                                        ),
                                        BVar3 != 0x0,
                                    ),
                            ))))
                    {
                        return;
                    }
                    ctx.PTR_LOOP_1050_0310 = 0x6d0;
                    return;
                }
            }
        }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
    return;
}

pub fn pass1_1038_64de(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_33f8(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn struct_1038_6520(param_1: U32Ptr) {
    let i_var1: &mut Struct308;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    i_var1.field_0x4 = 0x0;
    i_var1.field_0x8 = 0x0;
    i_var1.field_0xc = 0x0;
    i_var1.field_0xe = 0x0;
    i_var1.field_0x12 = 0x0;
    i_var1.field_0x14 = 0x0;
    i_var1.field_0x16 = 0x0;
    clear_struct_1008_3e38((param_1 & 0xffff0000 | &i_var1.field_0x1a));
    i_var1.field_0x20 = 0x0;
    i_var1.field_0x24 = 0x0;
    i_var1.field_0x26 = 0x0;
    i_var1.field_0x28 = 0x0;
    *param_1 = 0x78de;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_6590(
    param_1: U32Ptr,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u32,
) {
    let u_var1: u16;
    let i_var2: i16;
    let i_var3: &mut Struct410;
    let u_var3: u16;
    let unaff_SS: u16;
    let puVar4: U32Ptr;
    let u_var5: u32;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    *param_1 = 0x389a;
    i_var3.field_0x2 = 0x1008;
    &i_var3.field_0x4 = 0x0;
    i_var3.field_0x8 = param_6;
    i_var3.field_0xc = param_4;
    i_var3.field_0xe = 0x0;
    i_var3.field_0x12 = 0x0;
    i_var3.field_0x14 = 0x0;
    i_var3.field_0x16 = param_2;
    i_var3.field_0x18 = param_3;
    puVar4 = clear_struct_1008_3e38((param_1 & 0xffff0000 | &i_var3.field_0x1a));
    // u_var1 = (puVar4 >> 0x10);
    &i_var3.field_0x20 = 0x0;
    i_var3.field_0x24 = 0x0;
    i_var3.field_0x26 = param_5;
    i_var3.field_0x28 = 0x0;
    *param_1 = 0x78de;
    i_var3.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_6);
    u_var5 = pass1_1030_6d4e(CONCAT22(u_var1, param_5), param_5, u_var1, unaff_SS);
    // i_var2 = (u_var5 >> 0x10);
    i_var3.field_0x4 = u_var5;
    i_var3.field_0x6 = i_var2;
    puVar4 = (param_1 & 0xffff0000 | &i_var3.field_0x1a);
    pass1_1008_3f62(puVar4, CONCAT22(u_var1, param_5 + 0xc));
    u_var1 = puVar4;
    pass1_1010_8fba(&i_var3.field_0x4, u_var1);
    i_var3.field_0x20 = u_var1;
    i_var3.field_0x22 = i_var2;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_666e(param_1: U32Ptr, param_2: &i32, param_3: u16, param_4: u32) {
    let u_var1: u16;
    let u_var2: u16;
    let i_var3: &mut Struct420;
    let u_var3: u16;
    let unaff_SS: u16;
    let puVar4: U32Ptr;
    let u_var5: u32;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    *param_1 = 0x389a;
    i_var3.field_0x2 = 0x1008;
    i_var3.field_0x4 = 0x0;
    i_var3.field_0x8 = param_4;
    i_var3.field_0xc = 0x0;
    i_var3.field_0xe = param_2;
    i_var3.field_0x12 = 0x0;
    i_var3.field_0x14 = 0x0;
    i_var3.field_0x18 = 0x0;
    i_var3.field_0x16 = 0x0;
    puVar4 = clear_struct_1008_3e38((param_1 & 0xffff0000 | &i_var3.field_0x1a));
    // u_var1 = (puVar4 >> 0x10);
    &i_var3.field_0x20 = 0x0;
    i_var3.field_0x24 = 0x0;
    i_var3.field_0x26 = param_3;
    i_var3.field_0x28 = 0x0;
    *param_1 = 0x78de;
    i_var3.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_4);
    u_var5 = pass1_1030_6d4e(CONCAT22(u_var1, param_3), param_3, u_var1, unaff_SS);
    // u_var2 = (u_var5 >> 0x10);
    &i_var3.field_0x4 = u_var5;
    (&i_var3.field_0x4 + 0x2) = u_var2;
    puVar4 = (param_1 & 0xffff0000 | &i_var3.field_0x1a);
    pass1_1008_3f62(puVar4, CONCAT22(u_var1, param_3 + 0xc));
    u_var1 = puVar4;
    pass1_1010_8fba(i_var3.field_0x4, u_var1);
    i_var3.field_0x20 = u_var1;
    i_var3.field_0x22 = u_var2;
    pass1_1020_ba94(param_2);
    i_var3.field_0x16 = u_var1;
    i_var3.field_0x18 = u_var2;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_675c(param_1: U32Ptr, param_2: u32, param_3: u16, param_4: u16, param_5: u32) {
    let u_var1: u16;
    let u_var2: u16;
    let i_var3: &mut Struct414;
    let u_var3: u16;
    let unaff_SS: u16;
    let puVar4: U32Ptr;
    let u_var5: u32;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    *param_1 = 0x389a;
    i_var3.field_0x2 = 0x1008;
    i_var3.field_0x4 = 0x0;
    i_var3.field_0x8 = param_5;
    i_var3.field_0xc = 0x0;
    i_var3.field_0xe = 0x0;
    i_var3.field_0x12 = param_3;
    i_var3.field_0x14 = 0x0;
    i_var3.field_0x16 = param_2;
    puVar4 = clear_struct_1008_3e38((param_1 & 0xffff0000 | &i_var3.field_0x1a));
    // u_var1 = (puVar4 >> 0x10);
    &i_var3.field_0x20 = 0x0;
    i_var3.field_0x24 = 0x0;
    i_var3.field_0x26 = param_4;
    i_var3.field_0x28 = 0x0;
    *param_1 = 0x78de;
    i_var3.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_5);
    u_var5 = pass1_1030_6d4e(CONCAT22(u_var1, param_4), param_4, u_var1, unaff_SS);
    // u_var2 = (u_var5 >> 0x10);
    &i_var3.field_0x4 = u_var5;
    (&i_var3.field_0x4 + 0x2) = u_var2;
    puVar4 = (param_1 & 0xffff0000 | &i_var3.field_0x1a);
    pass1_1008_3f62(puVar4, CONCAT22(u_var1, param_4 + 0xc));
    u_var1 = puVar4;
    pass1_1010_8fba(i_var3.field_0x4, u_var1);
    i_var3.field_0x20 = u_var1;
    i_var3.field_0x22 = u_var2;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_6838(param_1: U32Ptr, param_2: u32, param_3: u16, param_4: u16, param_5: u32) {
    let u_var1: u16;
    let u_var2: u16;
    let i_var3: &mut Struct415;
    let u_var3: u16;
    let unaff_SS: u16;
    let puVar4: U32Ptr;
    let u_var5: u32;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    *param_1 = 0x389a;
    i_var3.field_0x2 = 0x1008;
    i_var3.field_0x4 = 0x0;
    i_var3.field_0x8 = param_5;
    i_var3.field_0xc = 0x0;
    i_var3.field_0xe = 0x0;
    i_var3.field_0x12 = 0x0;
    i_var3.field_0x14 = param_3;
    i_var3.field_0x16 = param_2;
    puVar4 = clear_struct_1008_3e38((param_1 & 0xffff0000 | &i_var3.field_0x1a));
    // u_var1 = (puVar4 >> 0x10);
    &i_var3.field_0x20 = 0x0;
    i_var3.field_0x24 = 0x0;
    i_var3.field_0x26 = param_4;
    i_var3.field_0x28 = 0x0;
    *param_1 = 0x78de;
    i_var3.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_5);
    u_var5 = pass1_1030_6d4e(CONCAT22(u_var1, param_4), param_4, u_var1, unaff_SS);
    // u_var2 = (u_var5 >> 0x10);
    &i_var3.field_0x4 = u_var5;
    (&i_var3.field_0x4 + 0x2) = u_var2;
    puVar4 = (param_1 & 0xffff0000 | &i_var3.field_0x1a);
    pass1_1008_3f62(puVar4, CONCAT22(u_var1, param_4 + 0xc));
    u_var1 = puVar4;
    pass1_1010_8fba(i_var3.field_0x4, u_var1);
    i_var3.field_0x20 = u_var1;
    i_var3.field_0x22 = u_var2;
    return;
}

pub fn pass1_1038_6912(param_1: U32Ptr) {
    let u_var1: u16;
    let u_var2: u16;
    let ppc_var3: u32;
    let puVar4: u32;
    let iVar5: i16;
    let u_var6: u16;
    let paStack10: &mut Struct18;

    // u_var6 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = 0x78de;
    (iVar5 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    u_var1 = (iVar5 + 0x6);
    puVar4 = (iVar5 + 0x4);
    if ((u_var1 | puVar4) != 0x0) {
        ppc_var3 = *puVar4;
        (**ppc_var3)();
    }
    u_var1 = (iVar5 + 0xe);
    u_var2 = (iVar5 + 0x10);
    paStack10 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0x0) {
        fn_ptr_1020_ba7e(CONCAT22(u_var2, u_var1));
        fn_ptr_1000_17ce(ctx, paStack10, 0x1000);
    }
    *param_1 = 0x389a;
    (iVar5 + 0x2) = 0x1008;
    return;
}

pub fn pass1_1038_6984(param_1: u32) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0xc) != 0x0) {
        pass1_1020_c3ae();
        return;
    }
    if ((i_var1 + 0xe) != 0x0) {
        pass1_1020_ba94((i_var1 + 0xe));
        return;
    }
    if ((i_var1 + 0x12) == 0x0) {
        if ((i_var1 + 0x14) == 0x0) {
            return;
        }
        pass1_1020_c42e((i_var1 + 0x14));
    } else {
        switch_1020_c3b4((i_var1 + 0x12));
    }
    return;
}

pub fn pass1_1038_69fe(param_1: u32) {
    (param_1 + 0x28) = 0x0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_6a0e(
    param_1: u32,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u16;
    let Bvar4: bool;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let puVar10: U32Ptr;
    let u_var11: u32;
    let uStack22: u32;
    let local_10: [u8; 4];
    let local_c: [u8; 6];
    let uStack6: u32;

    // uVar9 = (param_1 >> 0x10);
    uVar8 = param_1;
    if ((uVar8 + 0x28) == 0x0) {
        u_var2 = (uVar8 + 0x20);
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
        uStack6 = CONCAT22(param_3, param_2);
        pi_var1 = (uVar8 + 0x24);
        *pi_var1 = *pi_var1 + 0x3c;
        puVar10 = clear_struct_1008_3e38(CONCAT22(param_6, local_c));
        // u_var6 = (puVar10 >> 0x10);
        loop {
            u_var3 = pass1_1038_6d24(
                param_1,
                CONCAT22(param_6, local_10),
                CONCAT22(param_6, local_c),
                uStack6,
                (uStack6 >> 0x10),
                param_6,
            );
            if (u_var3 == 0x0) {
                pass1_1010_8fba((uVar8 + 0x4), 0x0);
                uStack22 = CONCAT22(u_var6, u_var3);
                uVar7 = u_var6 | u_var3;
                if (uVar7 == 0x0) {
                    u_var2 = (uVar8 + 0x20);
                    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var2);
                    pass1_1038_7356(param_1, CONCAT22(uVar7, u_var3), param_6, param_4, param_5);
                    return;
                }
                u_var11 = struct_op_1030_73a8(uStack6);
                BVar4 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (u_var11 + 0xc), 0x40);
                if (BVar4 != 0x0) {
                    (uVar8 + 0x28) = 0x1;
                    (uVar8 + 0x20) = uStack22;
                    return;
                }
                (uVar8 + 0x20) = uStack22;
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, (uVar8 + 0x20));
                uStack6 = uStack22 & 0xffff | u_var6 << 0x10;
            }
            u_var5 = pass1_1038_6e1a(uVar8, uVar9, CONCAT22(param_6, local_10));
            if ((uVar8 + 0x24) < u_var5) {
                break;
            }
            pi_var1 = (uVar8 + 0x24);
            *pi_var1 = *pi_var1 - u_var5;
            pass1_1008_3f62(
                (param_1 & 0xffff0000 | (uVar8 + 0x1a)),
                CONCAT22(param_6, local_c),
            );
        }
    }
    return;
}

pub fn pass1_1038_6b3c(param_1: u32) -> u16 {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if (((((i_var1 + 0xc) == 0x0) && ((i_var1 + 0x12) == 0x0)) && ((i_var1 + 0x14) == 0x0))
        && ((i_var1 + 0xe) == 0x0 && ((i_var1 + 0x16) != 0x0)))
    {
        (i_var1 + 0x16) = 0x0;
    }
    if ((i_var1 + 0x16) == 0x0) {
        return 0x1;
    }
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_6b88(
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: U32Ptr,
    param_5: U32Ptr,
    param_6: i16,
    param_7: u16,
) {
    let pu_var1: u32;
    let u_var2: u16;
    let local_12: [u32; 0x2];
    let lStack10: i32;
    let puStack6: U32Ptr;

    puStack6 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_7, param_5, param_6);
    // u_var2 = (puStack6 >> 0x10);
    lStack10 = (puStack6 + 0x20);
    pu_var1 = local_12;
    pass1_1030_64ce(
        param_7,
        pu_var1,
        u_var2,
        ctx.PTR_LOOP_1050_5740,
        param_3,
        lStack10,
        CONCAT22(param_7, pu_var1),
    );
    *param_4 = *pu_var1;
    return;
}

pub fn pass1_1038_6bd4(
    param_1: u32,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: i16,
    param_5: U32Ptr,
    param_6: i16,
    param_7: u16,
) {
    let uStack4: u16;

    pass1_1008_3f62(param_2, (param_1 & 0xffff0000 | (param_1 + 0x1a)));
    if (param_4 < 0x0) {
        uStack4 = *param_2 - 0x1;
    } else {
        uStack4 = *param_2 + 0x1;
    }
    *param_2 = uStack4;
    pass1_1038_6b88(
        param_1,
        (param_1 >> 0x10),
        param_2,
        param_3,
        param_5,
        param_6,
        param_7,
    );
    return;
}

pub fn pass1_1038_6c1c(
    param_1: u32,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: i16,
    param_5: U32Ptr,
    param_6: i16,
    param_7: u16,
) {
    let u_var1: u16;
    let i_stack4: i16;

    pass1_1008_3f62(param_2, (param_1 & 0xffff0000 | (param_1 + 0x1a)));
    // u_var1 = (param_2 >> 0x10);
    i_stack4 = (param_2 + 0x2);
    if (param_4 < 0x0) {
        i_stack4 += -0x1;
    } else {
        i_stack4 += 0x1;
    }
    (param_2 + 0x2) = i_stack4;
    pass1_1038_6b88(
        param_1,
        (param_1 >> 0x10),
        param_2,
        param_3,
        param_5,
        param_6,
        param_7,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_6c68(
    param_1: u32,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: i16,
    param_5: U32Ptr,
    param_6: i16,
    param_7: u16,
) {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let puVar9: U32Ptr;
    let u_var10: u32;
    let iStack30: i16;

    u_var2 = param_1;
    pass1_1008_3f62(param_2, (param_1 & 0xffff0000 | (u_var2 + 0x1a)));
    puVar9 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_7, param_5, param_6);
    // u_var5 = (puVar9 >> 0x10);
    puVar4 = (param_1 & 0xffff0000 | (u_var2 + 0x1a));
    pass1_1030_627e(
        param_7,
        u_var2 + 0x1a,
        u_var5,
        ctx.PTR_LOOP_1050_5740,
        puVar4,
        (puVar9 + 0x20),
    );
    u_var3 = puVar4;
    u_var6 = u_var5 | u_var3;
    if (u_var6 != 0x0) {
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var3);
        u_var10 = struct_op_1030_73a8(CONCAT22(u_var6, u_var3));
        // puVar7 = (u_var10 >> 0x10);
        i_var1 = (u_var10 + 0xc);
        if ((i_var1 == 0x47) || (i_var1 == 0x6a)) {
            // uVar8 = (param_1 >> 0x10);
            iStack30 = (u_var2 + 0x1e);
            if (param_4 < 0x0) {
                iStack30 += -0x1;
            } else {
                iStack30 += 0x1;
            }
            (param_2 + 0x4) = iStack30;
            pass1_1038_6b88(u_var2, uVar8, param_2, param_3, puVar7, param_6, param_7);
        }
    }
    return;
}

pub fn pass1_1038_6d24(
    param_1: u32,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: i16,
    param_5: u16,
    param_6: u16,
) -> i16 {
    let local_14: i16;
    let local_12: i16;
    let local_10: i16;
    let local_e: i16;
    let local_c: i16;
    let local_a: i16;
    let local_8: u32;
    let uStack4: u16;

    *param_2 = 0x0;
    local_8 = (param_4 + 0xc);
    uStack4 = (param_4 + 0x10);
    pass1_1008_3eb4(
        CONCAT22(param_6, &local_8),
        CONCAT22(param_6, &local_e),
        CONCAT22(param_6, &local_c),
        CONCAT22(param_6, &local_a),
    );
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | (param_1 + 0x1a)),
        CONCAT22(param_6, &local_14),
        CONCAT22(param_6, &local_12),
        CONCAT22(param_6, &local_10),
    );
    local_c -= local_12;
    local_e -= local_14;
    local_a -= local_10;
    if (((local_a == 0x0) && (local_c == 0x0)) && (local_e == 0x0)) {
        return 0x0;
    }
    if ((local_c != 0x0) || (local_a == 0x0)) {
        if ((local_a == 0x0) && (local_c != 0x0)) {
            pass1_1038_6c1c(
                param_1,
                param_3,
                param_2,
                local_c,
                0x0,
                &stack0xfffe,
                param_6,
            );
            return 0x1;
        }
        if (((local_a == 0x0) && (local_c == 0x0)) && (local_e != 0x0)) {
            pass1_1038_6c68(
                param_1,
                param_3,
                param_2,
                local_e,
                0x0,
                &stack0xfffe,
                param_6,
            );
            if (local_c != 0x0) {
                return 0x1;
            }
            return local_c;
        }
    }
    pass1_1038_6bd4(
        param_1,
        param_3,
        param_2,
        local_a,
        local_a,
        &stack0xfffe,
        param_6,
    );
    return 0x1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_6e1a(param_1: u16, param_2: u16, param_3: &i32) -> u16 {
    let u_var1: u16;
    let BVar2: bool;
    let u_var3: u16;
    let u_var4: u16;
    let bStack21: u8;
    let uStack4: u16;

    uStack4 = 0x0;
    if ((*param_3 == 0x0) && (param_3 == 0x0)) {
        return 0x1;
    }
    u_var4 = (param_3 + 0x2);
    bStack21 = (u_var4 >> 0x8);
    u_var1 = bStack21;
    if (bStack21 == 0x0) {
        uStack4 = param_3;
        //     TODO: goto switchD_1038_6eab_caseD_9;
    }
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, *param_3);
    u_var3 = pass1_1030_6fa0(CONCAT22(u_var4, u_var1));
    if (u_var3 < 0xa) {
        match (u_var3) {
            0x1 => {
                uStack4 = 0x1;
            }
            0x2 | 0x6 => {
                uStack4 = 0x2;
            }
            0x3 | 0x7 => {
                uStack4 = 0x3;
            }
            0x4 | 0x8 => {
                uStack4 = 0x4;
            }
            0x5 | 0x9 => {} //       TODO: goto switchD_1038_6eab_caseD_5;
        }
    } else {
        BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var3, 0x41);
        if (BVar2 != 0x0) {
            uStack4 = 0xa;
            //       TODO: goto switchD_1038_6eab_caseD_9;
        }
        BVar2 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, u_var3, 0x42);
        if ((BVar2 != 0x0) || (u_var3 == 0x3f)) {
            uStack4 = 0xb;
            //       TODO: goto switchD_1038_6eab_caseD_9;
        }
        // switchD_1038_6eab_caseD_5:
        uStack4 = 0x5;
    }
    // switchD_1038_6eab_caseD_9:
    match (uStack4) {
        0x1 => {
            return 0x14;
        }
        0x2 | 0x7 => {
            return 0x3c;
        }
        0x3 | 0x8 => {
            return 0x78;
        }
        0x4 | 0x9 => {
            return 0xf0;
        }
        0x5 | 0x6 => {
            return 0xf;
        }
        0xa => {
            u_var3 = 0xc;
        }
        0xb => {
            u_var3 = 0xa;
        }
        _ => {
            u_var3 = 0xffff;
        }
    }
    return u_var3;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_6f5a(
    param_1: u32,
    param_2: u32,
    param_3: u16,
    param_4: U32Ptr,
    param_5: u16,
    param_6: u16,
    param_7: u16,
) {
    let u_var1: u32;
    let lVar2: i32;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let u_var5: u16;
    let iVar6: i16;
    let iVar7: i16;
    let uVar8: u16;
    let uVar9: u16;
    let paStack16: &mut Struct99;
    let uStack12: u16;
    let local_a: u16;
    let uStack8: u16;
    let local_6: u16;
    let uStack4: u16;
    let u_var3: &mut Struct623;

    // uVar8 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0xe) == 0x0) {
        if ((iVar6 + 0xc) != 0x0) {
            pass1_1030_7ddc(
                param_2,
                (iVar6 + 0x16),
                (iVar6 + 0xc),
                param_3,
                param_4,
                param_5,
                param_6,
                param_7,
            );
            return;
        }
        if ((iVar6 + 0x12) != 0x0) {
            pass1_1030_7c50(param_2, (iVar6 + 0x16), (iVar6 + 0x12), param_3, param_4);
            return;
        }
        paStack16 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
        // u_var5 = (paStack16 >> 0x10);
        u_var3 = paStack16;
        if ((u_var5 | u_var3) == 0x0) {
            paStack16 = 0x0;
        } else {
            paStack16.field_0x0 = 0x389a;
            u_var3.field_0x2 = 0x1008;
            u_var3.field_0x4 = 0x0;
            u_var3.field_0x6 = 0x0;
            u_var3.field_0x8 = 0x0;
            u_var3.field_0xa = 0x0;
            u_var3.field_0xc = 0x0;
            paStack16.field_0x0 = 0x56ce;
            u_var3.field_0x2 = 0x1018;
        }
        // uVar9 = (paStack16 >> 0x10);
        iVar7 = paStack16;
        (iVar7 + 0x8) = (iVar6 + 0x14);
        (iVar7 + 0xa) = (iVar6 + 0x16);
        u_var4 = pass1_1020_c42e((iVar6 + 0x14));
        lVar2 = u_var4 * (iVar7 + 0xa);
        u_var5 = lVar2;
        (iVar7 + 0xc) = u_var5;
        pass1_1030_6a2c(param_2, paStack16, u_var5, (lVar2 >> 0x10), param_7);
    } else {
        u_var1 = (iVar6 + 0xe);
        uStack4 = (u_var1 + 0x4);
        // for (uStack12 = 0x0; uStack12 < uStack4; uStack12 += 0x1) {
        //   pu_var3 = &local_6;
        //   pass1_1020_bb16(*(u32 **)(iVar6 + 0xe),CONCAT22(param_7,&local_a),
        //                   CONCAT22(param_7,pu_var3),uStack12);
        //   if (CONCAT22(uStack8,local_a) != 0x0) {
        //     pass1_1030_7ddc(param_2,CONCAT22(uStack8,local_a),local_6,pu_var3,param_4,
        //                     param_5,param_6,param_7);
        //   }
        // }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_709c(param_1: u32, param_2: u32, param_3: U32Ptr, param_4: u16) {
    let pu_var1: u32;
    let lVar2: i32;
    let uVar7: u16;
    let uVar8: u16;
    let puVar9: U32Ptr;
    let i_var8: &mut Struct618;
    let iVar10: i16;
    let u_var11: u16;
    let uVar12: u16;
    let paStack40: &mut Struct99;
    let paStack16: &mut Struct99;
    let uStack12: u16;
    let local_a: i32;
    let local_6: u16;
    let uStack4: u16;
    let u_var3: &mut Struct617;
    let u_var4: &mut Struct619;
    let u_var5: &mut Struct620;
    let u_var6: &mut Struct621;

    // u_var11 = (param_1 >> 0x10);
    i_var8 = param_1;
    if (((&i_var8.field_0xe + 0x2) | &i_var8.field_0xe) == 0x0) {
        if (i_var8.field_0xc == 0x0) {
            if (i_var8.field_0x12 == 0x0) {
                if (i_var8.field_0x14 == 0x0) {
                    return;
                }
                paStack40 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
                // uVar8 = (paStack40 >> 0x10);
                u_var3 = paStack40;
                if ((uVar8 | u_var3) == 0x0) {
                    paStack40 = 0x0;
                } else {
                    paStack40.field_0x0 = 0x389a;
                    u_var3.field_0x2 = 0x1008;
                    u_var3.field_0x4 = 0x0;
                    u_var3.field_0x6 = 0x0;
                    u_var3.field_0x8 = 0x0;
                    u_var3.field_0xa = 0x0;
                    u_var3.field_0xc = 0x0;
                    paStack40.field_0x0 = 0x56ce;
                    u_var3.field_0x2 = 0x1018;
                }
                // uVar12 = (paStack40 >> 0x10);
                (paStack40 + 0x8) = i_var8.field_0x14;
                (paStack40 + 0xa) = &i_var8.field_0x16;
                uVar8 = pass1_1020_c42e(i_var8.field_0x14);
            } else {
                pass1_1030_7c50(param_2, i_var8.field_0x16, i_var8.field_0x12, 0x0, param_3);
                paStack40 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
                // uVar8 = (paStack40 >> 0x10);
                u_var4 = paStack40;
                if ((uVar8 | u_var4) == 0x0) {
                    paStack40 = 0x0;
                } else {
                    paStack40.field_0x0 = 0x389a;
                    u_var4.field_0x2 = 0x1008;
                    u_var4.field_0x4 = 0x0;
                    u_var4.field_0x6 = 0x0;
                    u_var4.field_0x8 = 0x0;
                    u_var4.field_0xa = 0x0;
                    u_var4.field_0xc = 0x0;
                    paStack40.field_0x0 = 0x56ce;
                    u_var4.field_0x2 = 0x1018;
                }
                // uVar12 = (paStack40 >> 0x10);
                (paStack40 + 0x6) = i_var8.field_0x12;
                (paStack40 + 0xa) = &i_var8.field_0x16;
                uVar8 = switch_1020_c3b4(i_var8.field_0x12);
            }
            // uVar12 = (paStack40 >> 0x10);
            iVar10 = paStack40;
            lVar2 = uVar8 * (iVar10 + 0xa);
            // puVar9 = (lVar2 >> 0x10);
            uVar8 = lVar2;
        } else {
            paStack40 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
            // uVar8 = (paStack40 >> 0x10);
            u_var5 = paStack40;
            puVar9 = (uVar8 | u_var5);
            if (puVar9 == 0x0) {
                paStack40 = 0x0;
            } else {
                paStack40.field_0x0 = 0x389a;
                u_var5.field_0x2 = 0x1008;
                u_var5.field_0x4 = 0x0;
                u_var5.field_0x6 = 0x0;
                u_var5.field_0x8 = 0x0;
                u_var5.field_0xa = 0x0;
                u_var5.field_0xc = 0x0;
                paStack40.field_0x0 = 0x56ce;
                u_var5.field_0x2 = 0x1018;
            }
            // uVar12 = (paStack40 >> 0x10);
            iVar10 = paStack40;
            (iVar10 + 0x4) = i_var8.field_0xc;
            uVar8 = &i_var8.field_0x16;
            (iVar10 + 0xa) = uVar8;
        }
        (iVar10 + 0xc) = uVar8;
        pass1_1030_6a2c(param_2, CONCAT22(uVar12, iVar10), uVar8, puVar9, param_4);
    } else {
        pu_var1 = i_var8.field_0xe;
        uStack4 = (pu_var1 + 0x4);
        // for (uStack12 = 0x0; uStack12 < uStack4; uStack12 += 0x1) {
        //   pass1_1020_bb16(i_var8.field_0xe,CONCAT22(param_4,&local_a),
        //                   CONCAT22(param_4,&local_6),uStack12);
        //   if (local_a != 0x0) {
        //     paStack16 = pass1_1000_07fc(0x1000,PTR_LOOP_1050_68a2);
        //     uVar8 = (paStack16 >> 0x10);
        //     u_var6 = paStack16;
        //     if ((uVar8 | u_var6) == 0x0) {
        //       paStack16 = 0x0;
        //     }
        //     else {
        //       paStack16.field_0x0 = 0x389a;
        //       u_var6.field_0x2 = 0x1008;
        //       u_var6.field_0x4 = 0x0;
        //       u_var6.field_0x6 = 0x0;
        //       u_var6.field_0x8 = 0x0;
        //       u_var6.field_0xa = 0x0;
        //       u_var6.field_0xc = 0x0;
        //       paStack16.field_0x0 = 0x56ce;
        //       u_var6.field_0x2 = 0x1018;
        //     }
        //     uVar12 = (paStack16 >> 0x10);
        //     iVar10 = paStack16;
        //     (iVar10 + 0x4) = local_6;
        //     (iVar10 + 0xa) = local_a;
        //     uVar7 = pass1_1020_c3ae();
        //     lVar2 = uVar7 * (iVar10 + 0xa);
        //     uVar8 = lVar2;
        //     (iVar10 + 0xc) = uVar8;
        //     pass1_1030_6a2c(param_2,paStack16,uVar8,(lVar2 >> 0x10),
        //                     param_4);
        //   }
        // }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_7356(param_1: u32, param_2: u32, param_3: u16, param_4: u16, param_5: u16) {
    uchar * *ppuVar1;
    let pu_var2: U32Ptr;
    let u_var3: u32;
    let paVar4: &mut Struct18;
    let lVar5: i32;
    let BVar6: bool;
    let uVar7: u16;
    let uVar9: u16;
    let puVar10: U32Ptr;
    let puVar11: U32Ptr;
    let i_var9: &mut Struct615;
    let iVar12: i16;
    let uVar13: u16;
    let uVar14: u16;
    let bVar15: bool;
    let uVar16: u32;
    let uVar17: u32;
    let paStack50: &mut Struct99;
    let paStack26: &mut Struct99;
    let uVar8: &mut Struct616;
    let u_var10: &mut Struct622;

    uVar16 = struct_op_1030_73a8(param_2);
    // puVar10 = (uVar16 >> 0x10);
    uVar7 = uVar16;
    puVar11 = puVar10;
    BVar6 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (uVar7 + 0xc), 0x4);
    i_var9 = param_1;
    // uVar13 = (param_1 >> 0x10);
    if (BVar6 == 0x0) {
        uVar9 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (uVar7 + 0xc), 0x3);
        if (uVar9 == 0x0) {
            // code_r0x10387545:
            pass1_1038_6f5a(param_1, param_2, uVar9, puVar11, param_4, param_5, param_3);
            //       TODO: goto LAB_1038_7549;
        }
        if ((i_var9.field_0xc != 0x0) || (&i_var9.field_0xe != 0x0)) {
            uVar16 = pass1_1028_45e2(uVar16, uVar7, puVar11, param_3);
            // puVar11 = (uVar16 >> 0x10);
            uVar9 = uVar16;
            ppuVar1 = &i_var9.field_0x18;
            bVar15 = *ppuVar1 < puVar11;
            if ((bVar15 || *ppuVar1 == puVar11)
                && (bVar15
                    || (
                        pu_var2 = &i_var9.field_0x16,
                        *pu_var2 < uVar9 || *pu_var2 == uVar9,
                    )))
            {}
            //       TODO: goto code_r0x10387545;
        }
    } else {
        uVar17 = pass1_1028_62c8(uVar16, param_3);
        // puVar11 = (uVar17 >> 0x10);
        uVar9 = uVar17;
        ppuVar1 = &i_var9.field_0x18;
        bVar15 = *ppuVar1 < puVar11;
        if ((bVar15 || *ppuVar1 == puVar11)
            && (bVar15
                || (
                    pu_var2 = &i_var9.field_0x16,
                    *pu_var2 < uVar9 || *pu_var2 == uVar9,
                )))
        {
            if (i_var9.field_0x12 == 0x0) {
                if (i_var9.field_0x14 == 0x0) {
                    // goto
                    // LAB_1038_74e0;
                }
                paStack50 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
                // uVar7 = (paStack50 >> 0x10);
                u_var10 = paStack50;
                if ((uVar7 | u_var10) == 0x0) {
                    paStack50 = 0x0;
                } else {
                    paStack50.field_0x0 = 0x389a;
                    u_var10.field_0x2 = 0x1008;
                    u_var10.field_0x4 = 0x0;
                    u_var10.field_0x6 = 0x0;
                    u_var10.field_0x8 = 0x0;
                    u_var10.field_0xa = 0x0;
                    u_var10.field_0xc = 0x0;
                    paStack50.field_0x0 = 0x56ce;
                    u_var10.field_0x2 = 0x1018;
                }
                // uVar14 = (paStack50 >> 0x10);
                iVar12 = paStack50;
                (iVar12 + 0x8) = i_var9.field_0x14;
                (iVar12 + 0xa) = i_var9.field_0x16;
                uVar7 = pass1_1020_c42e(i_var9.field_0x14);
            } else {
                paStack26 = pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
                // uVar7 = (paStack26 >> 0x10);
                uVar8 = paStack26;
                if ((uVar7 | uVar8) == 0x0) {
                    paStack26 = 0x0;
                } else {
                    paStack26.field_0x0 = 0x389a;
                    uVar8.field_0x2 = 0x1008;
                    uVar8.field_0x4 = 0x0;
                    uVar8.field_0x6 = 0x0;
                    uVar8.field_0x8 = 0x0;
                    uVar8.field_0xa = 0x0;
                    uVar8.field_0xc = 0x0;
                    paStack26.field_0x0 = 0x56ce;
                    uVar8.field_0x2 = 0x1018;
                }
                // uVar14 = (paStack26 >> 0x10);
                iVar12 = paStack26;
                (iVar12 + 0x6) = i_var9.field_0x12;
                (iVar12 + 0xa) = i_var9.field_0x16;
                uVar7 = switch_1020_c3b4(i_var9.field_0x12);
            }
            lVar5 = uVar7 * (iVar12 + 0xa);
            // puVar11 = (lVar5 >> 0x10);
            uVar9 = lVar5;
            (iVar12 + 0xc) = uVar9;
            pass1_1028_6408(uVar16, CONCAT22(uVar14, iVar12), param_3);
            //       TODO: goto LAB_1038_7549;
        }
    }
    //LAB_1038_74e0:
    pass1_1038_709c(param_1, param_2, puVar11, param_3);
    //LAB_1038_7549:
    u_var3 = i_var9.field_0x8;
    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var3);
    pass1_1030_6c4c(CONCAT22(puVar11, uVar9), (uVar9 + 0x34) + i_var9.field_0x26);
    i_var9.field_0xc = 0x0;
    i_var9.field_0x12 = 0x0;
    i_var9.field_0x14 = 0x0;
    &i_var9.field_0x16 = 0x0;
    paVar4 = &i_var9.field_0xe;
    uVar7 = i_var9.field_0x10;
    if ((uVar7 | paVar4) != 0x0) {
        fn_ptr_1020_ba7e((paVar4 & 0xffff | uVar7 << 0x10));
        fn_ptr_1000_17ce(ctx, paVar4, 0x1000);
    }
    &i_var9.field_0xe = 0x0;
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1038_75ca(param_1: u32, param_2: u32, param_3: i16, param_4: u16) {
    let b_var1: bool;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let local_10: [u32; 0x2];
    let local_8: u32;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    pass1_1008_79f0(param_2, (i_var3 + 0x4), 0x1008, param_4);
    if (param_3 != 0x0) {
        local_10[0] = (i_var3 + 0x8);
        u_var5 = param_2;
        // u_var6 = (param_2 >> 0x10);
        b_var1 = write_to_file_1008_7e1c(u_var5, u_var6, local_10, param_4, 0x4, 0x1008);
        if (b_var1 != 0x0) {
            write_to_file_1008_7a22(param_2, (i_var3 + 0xe), 0x1008, param_4);
            if (b_var1 != 0x0) {
                local_8._0_2_ = (i_var3 + 0xc);
                b_var1 = write_to_file_1008_7e1c(u_var5, u_var6, &local_8, param_4, 0x2, 0x1008);
                if (b_var1 != 0x0) {
                    local_8._0_2_ = (i_var3 + 0x12);
                    b_var1 =
                        write_to_file_1008_7e1c(u_var5, u_var6, &local_8, param_4, 0x2, 0x1008);
                    if (b_var1 != 0x0) {
                        local_8 = CONCAT22(local_8._2_2_, (i_var3 + 0x14));
                        b_var1 =
                            write_to_file_1008_7e1c(u_var5, u_var6, &local_8, param_4, 0x2, 0x1008);
                        if (b_var1 != 0x0) {
                            local_8 = (i_var3 + 0x16);
                            b_var1 = write_to_file_1008_7e1c(
                                u_var5, u_var6, &local_8, param_4, 0x4, 0x1008,
                            );
                            if (b_var1 != 0x0) {
                                i_var2 = write_to_file_1008_7b4c(
                                    param_2,
                                    param_1 & 0xffff0000 | (i_var3 + 0x1a),
                                    0x1008,
                                    param_4,
                                );
                                if (i_var2 != 0x0) {
                                    local_8 = (i_var3 + 0x20);
                                    b_var1 = write_to_file_1008_7e1c(
                                        u_var5, u_var6, &local_8, param_4, 0x4, 0x1008,
                                    );
                                    if (b_var1 != 0x0) {
                                        local_8 = local_8 & 0xffff0000 | (i_var3 + 0x24);
                                        b_var1 = write_to_file_1008_7e1c(
                                            u_var5, u_var6, &local_8, param_4, 0x2, 0x1008,
                                        );
                                        if (b_var1 != 0x0) {
                                            local_8 = local_8 & 0xffff0000 | (i_var3 + 0x26);
                                            b_var1 = write_to_file_1008_7e1c(
                                                u_var5, u_var6, &local_8, param_4, 0x2, 0x1008,
                                            );
                                            if (b_var1 != 0x0) {
                                                local_8 = local_8 & 0xffff0000 | (i_var3 + 0x28);
                                                b_var1 = write_to_file_1008_7e1c(
                                                    u_var5, u_var6, &local_8, param_4, 0x2, 0x1008,
                                                );
                                                if (b_var1 != 0x0) {
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
    return;
}

pub fn pass1_1038_78b8(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_6912(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_78e2(param_1: U32Ptr, param_2: U32Ptr) {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: u16;
    let u_var3: u16;
    let i_var4: &mut Struct431;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var1 = 0x0;
    *param_1 = 0x0;
    &i_var4.field_0x4 = 0x0;
    ctx.PTR_LOOP_1050_5a64 = param_1;
    mem_op_1000_179c(0xc, param_2, 0x1000);
    pu_var2 = (param_2 | u_var1);
    if (pu_var2 == 0x0) {
        *param_1 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(param_2, u_var1));
        param_1 = u_var1;
        i_var4.field_0x2 = extraout_dx;
        pu_var2 = extraout_dx;
    }
    mem_op_1000_179c(0xc, pu_var2, 0x1000);
    if ((pu_var2 | u_var1) == 0x0) {
        u_var1 = 0x0;
        u_var3 = 0x0;
    } else {
        set_struct_1008_574a(CONCAT22(pu_var2, u_var1));
        u_var3 = extraout_DX_00;
    }
    i_var4.field_0x4 = u_var1;
    i_var4.field_0x6 = u_var3;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_7964(param_1: U32Ptr) {
    let u_var1: u16;
    let pu_var2: u32;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;

    ctx.PTR_LOOP_1050_5a64 = 0x0;
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var1 = (i_var4 + 0x2);
    if ((u_var1 | *param_1) != 0x0) {
        ppc_var3 = *param_1;
        (**ppc_var3)();
    }
    pu_var2 = (i_var4 + 0x4);
    u_var1 = (i_var4 + 0x6);
    if ((u_var1 | pu_var2) != 0x0) {
        ppc_var3 = *pu_var2;
        (**ppc_var3)();
    }
    return;
}

pub fn pass1_1038_79b2(param_1: u32, param_2: u32, param_3: u16, param_4: U32Ptr) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;

    u_var4 = 0x1000;
    mem_op_1000_179c(0x14, param_4, 0x1000);
    u_var2 = param_4 | param_3;
    if (u_var2 == 0x0) {
        param_3 = 0x0;
        u_var2 = 0x0;
    } else {
        u_var4 = 0x1030;
        pass1_1030_aefa(CONCAT22(param_4, param_3), param_2);
    }
    // u_var3 = (param_1 >> 0x10);
    ppcVar1 = ((param_1 + 0x4) + 0x4);
    (**ppcVar1)(u_var4, (param_1 + 0x4), param_3, u_var2);
    return;
}

pub fn pass1_1038_79f2(param_1: u32, param_2: u32, param_3: u16) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let extraout_dx: u16;
    let i_var3: i16;
    let u_var4: u16;
    let local_e: [u8; 8];
    let lStack6: i32;

    lStack6 = (param_2 + 0x4);
    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    pass1_1008_5784(CONCAT22(param_3, local_e), (i_var3 + 0x4));
    loop {
        pu_var2 = local_e;
        pass1_1008_5b12(pu_var2, param_3);
        if ((extraout_dx | pu_var2) == 0x0) {
            return;
        }
        if ((pu_var2 + 0x4) != lStack6) == false {
            break;
        }
    }
    ppcVar1 = ((i_var3 + 0x4) + 0xc);
    (**ppcVar1)(0x1008, (i_var3 + 0x4), pu_var2, extraout_dx);
    return;
}

pub fn pass1_1038_7a5a(param_1: U32Ptr) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x4);
    (**ppcVar1)();
    return;
}

pub fn pass1_1038_7a76(param_1: U32Ptr, param_2: u16, param_3: i16, param_4: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let u_var3: u32;
    let local_a: [u8; 4];
    let uStack6: u32;

    pass1_1008_5784(CONCAT22(param_4, local_a), *param_1);
    loop {
        u_var3 = pass1_1008_5b12(local_a, param_4);
        if (u_var3 == 0x0) {
            break;
        }
        pass1_1038_6a0e(
            u_var3,
            u_var3,
            (u_var3 >> 0x10) | u_var3,
            param_2,
            param_3,
            param_4,
        );
    }
    loop {
        uStack6 = 0x0;
        loop {
            u_var3 = pass1_1008_5b12(local_a, param_4);
            if (u_var3 == 0x0) {
                pass1_1008_5784(CONCAT22(param_4, local_a), (param_1 + 0x4));
                loop {
                    u_var3 = pass1_1008_5b12(local_a, param_4);
                    if (u_var3 == 0x0) {
                        break;
                    }
                    pass1_1030_affc(u_var3, param_3, param_4);
                }
                return;
            }
            u_var2 = pass1_1038_6b3c(u_var3);
            if (u_var2 == 0x0) == false {
                break;
            }
        }
        ppcVar1 = (*param_1 + 0xc);
        (**ppcVar1)(0x1008);
    }
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1038_7b20(param_1: U32Ptr, param_2: u32, param_3: u16) -> u16 {
    let u_var1: u32;
    let BVar2: bool;
    let u_var3: u16;
    let u_var4: u32;
    let u_var5: u16;
    let local_1c: u16;
    let uStack26: u16;
    let uStack24: u16;
    let uStack16: u32;
    let local_c: [u8; 8];
    let local_4: u16;

    BVar2 = write_to_file_1008_7cac(param_2, param_3);
    if (BVar2 != 0x0) {
        local_1c = (*param_1 + 0x8);
        // u_var5 = (param_2 >> 0x10);
        local_4 = local_1c;
        BVar2 = write_to_file_1008_7e1c(param_2, u_var5, &local_1c, param_3, 0x2, 0x1008);
        if (BVar2 != 0x0) {
            pass1_1008_5784(CONCAT22(param_3, local_c), *param_1);
            loop {
                uStack16 = pass1_1008_5b12(local_c, param_3);
                if (uStack16 == 0x0) {
                    // u_var3 = (param_1 >> 0x10);
                    u_var1 = (param_1 + 0x4);
                    local_1c = (u_var1 + 0x8);
                    local_4 = local_1c;
                    BVar2 =
                        write_to_file_1008_7e1c(param_2, u_var5, &local_4, param_3, 0x2, 0x1008);
                    if (BVar2 == 0x0) {
                        return 0x0;
                    }
                    pass1_1008_5784(CONCAT22(param_3, local_c), (param_1 + 0x4));
                    loop {
                        u_var4 = pass1_1008_5b12(local_c, param_3);
                        uStack26 = u_var4;
                        if (u_var4 == 0x0) {
                            return 0x1;
                        }
                        pass1_1030_b768(u_var4, param_2, param_3);
                        // uStack24 = (u_var4 >> 0x10);
                        if (u_var4 != 0x0) == false {
                            break;
                        }
                    }
                    return 0x0;
                }
                pass1_1038_75ca(uStack16, param_2, uStack16, param_3);
                uStack16._2_2_ = (uStack16 >> 0x10);
                if (uStack16 != 0x0) == false {
                    break;
                }
            }
        }
    }
    return 0x0;
}

pub fn pass1_1038_7d10(
    param_1: &mut Struct57,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i16,
    param_5: u16,
) -> Struct57 {
    let i_var1: &mut Struct703;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x1853));
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    &i_var1.field_0x94 = 0x0;
    param_1 = 0x8876;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x40, param_5, param_3, param_4);
    i_var1.field_0x94 = pu_var2;
    i_var1.field_0x96 = (pu_var2 >> 0x10);
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_7d5c(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x8876;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    unk_draw_op_1040_b0f8(param_1);
    return;
}

pub fn pass1_1038_7dac(param_1: u32, param_2: u16) -> LRESULT {
    let LVar1: LRESULT;

    pass1_1040_78de(param_1);
    LVar1 = send_dlg_item_msg_1038_844a(param_1, &ctx.PTR_LOOP_1050_1040, param_2);
    return LVar1;
}

pub fn pass1_1038_7dc6(
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: u16,
    param_7: u16,
    param_8: u16,
) {
    let bVar1: bool;

    bVar1 = false;
    if (param_4._2_2_ == 0x1854) {
        if (param_4 != 0x1) {
            // goto
            // LAB_1038_7e8c;
        }
        send_dlg_item_msg_1038_8618(CONCAT22(param_2, param_1), param_8);
    } else {
        if (param_4 < 0x18550000) {
            if (param_4._2_2_ == 0xeb) {
                send_dlg_item_msg_1038_844a(CONCAT22(param_2, param_1), param_7, param_8);
            } else {
                if (param_4._2_2_ == 0xfb) {
                    send_dlg_item_msg_1038_7eac(CONCAT22(param_2, param_1));
                } else {
                    if (param_4._2_2_ != (s_vrpal_bmp_1050_183a + 0x7)) {
                        //LAB_1038_7e77:
                        pass1_1040_b54a(
                            param_1,
                            param_2,
                            param_3,
                            param_4,
                            param_5,
                            &ctx.PTR_LOOP_1050_1040,
                            param_8,
                        );
                        return;
                    }
                    msg_box_op_1038_81be(CONCAT22(param_2, param_1), 0x0, param_5, param_8);
                }
            }
            //       TODO: goto LAB_1038_7e8c;
        }
        if (param_4._2_2_ == 0x1855) {
            if (param_4 != 0x1) {
                // goto
                // LAB_1038_7e8c;
            }
            send_dlg_item_msg_1038_87b2(CONCAT22(param_2, param_1), param_7, param_8);
        } else {
            if (param_4._2_2_ == 0x1856) {
                if (param_4 != 0x1) {
                    // goto
                    // LAB_1038_7e8c;
                }
                pass1_1038_8810(CONCAT22(param_2, param_1), param_7, param_8);
            } else {
                if (param_4._2_2_ == 0x1858) {
                    send_dlg_item_msg_1038_7fae(CONCAT22(param_2, param_1));
                } else {
                    if (param_4._2_2_ != 0x1859) {
                        // goto
                        // LAB_1038_7e77;
                    }
                    pass1_1038_801a(CONCAT22(param_2, param_1), param_5, param_6, param_8);
                }
            }
        }
    }
    bVar1 = true;
    //LAB_1038_7e8c:
    if (bVar1) {
        set_win_text_1038_8358(CONCAT22(param_2, param_1), param_7, param_8);
        enable_win_1038_806a(CONCAT22(param_2, param_1), param_7);
    }
    return;
}

pub fn pass1_1038_801a(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) -> u32 {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let puVar4: U32Ptr;
    let mut pcVar5: String;
    let u_var6: u32;

    puVar4 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x30, param_4, param_2, param_3);
    // u_var3 = (param_1 >> 0x10);
    u_var2 = param_1;
    pcVar5 = pass1_1008_b340((u_var2 + 0x94));
    u_var1 = (pcVar5 >> 0x10) | pcVar5;
    u_var6 = pcVar5 & 0xffff | u_var1 << 0x10;
    if (pcVar5 != 0x0) {
        pass1_1010_3770(puVar4, &mut pcVar5, u_var1);
        u_var6 = pass1_1038_af40(
            ctx.PTR_LOOP_1050_5b7c,
            (u_var2 + 0x6),
            0x3,
            u_var1,
            u_var2,
            0x1010,
            param_4,
        );
    }
    return u_var6;
}

pub fn pass1_1038_8810(param_1: u32, param_2: u16, param_3: u16) {
    let u_var1: u16;
    let u_var2: u16;
    let local_102: [u8; 100];

    // u_var2 = (param_1 >> 0x10);
    u_var1 = send_dlg_item_msg_1038_8164(
        param_1,
        u_var2,
        CONCAT22(param_3, local_102),
        0x1856,
        param_2,
    );
    if (u_var1 != 0x0) {
        pass1_1008_b63a((param_1 + 0x94), CONCAT22(param_3, local_102));
    }
    return;
}

pub fn pass1_1038_8848() {
    return;
}

pub fn pass1_1038_884c() {
    return;
}

pub fn pass1_1038_8850(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_7d5c(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_88f2(param_1: &mut Struct57, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x184c));
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x94) = ctx.PTR_LOOP_1050_5a68;
    (i_var1 + 0x98) = 0x0;
    (i_var1 + 0x9a) = 0x0;
    (i_var1 + 0x9c) = 0x0;
    (i_var1 + 0x9e) = 0x0;
    param_1 = 0x8c2e;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_893a(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x8c2e;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    unk_draw_op_1040_b0f8(param_1);
    return;
}

pub fn pass1_1038_8966(
    param_1: u32,
    param_2: u16,
    param_3: u16,
    param_4: i16,
    param_5: HWND16,
) -> u16 {
    let pi_var1: U32Ptr;
    let bVar2: bool;
    let i_var3: i16;
    let u_var4: u16;

    bVar2 = false;
    i_var3 = param_1;
    // u_var4 = (param_1 >> 0x10);
    if (param_4 == 0x0) {
        if ((i_var3 + 0x98) < 0x1) {
            // goto
            // LAB_1038_89af;
        }
        pi_var1 = (i_var3 + 0x9a);
        *pi_var1 = *pi_var1 + 0x1;
        pi_var1 = (i_var3 + 0x98);
        *pi_var1 = *pi_var1 + -0x1;
    } else {
        if (param_4 != 0x1) {
            // goto
            // LAB_1038_89af;
        }
        if ((i_var3 + 0x9a) < 0x1) {
            // goto
            // LAB_1038_89af;
        }
        pi_var1 = (i_var3 + 0x9a);
        *pi_var1 = *pi_var1 + -0x1;
        pi_var1 = (i_var3 + 0x98);
        *pi_var1 = *pi_var1 + 0x1;
    }
    bVar2 = true;
    //LAB_1038_89af:
    if (bVar2) {
        SetDlgItemInt16(
            param_5,
            0x0,
            (i_var3 + 0x9a),
            (s_dibtext_bmp_1050_1844 + 0x9),
        );
        SetDlgItemInt16(
            ctx.s_tile2_bmp_1050_1538,
            0x0,
            (i_var3 + 0x98),
            (s_dibtext_bmp_1050_1844 + 0xb),
        );
    }
    return 0x0;
}

pub fn pass1_1038_89e8(param_1: u32, param_2: u16) {
    send_dlg_item_msg_1038_8b58(param_1, param_2);
    return;
}

pub fn pass1_1038_89f8(
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: u16,
) {
    if (param_4._2_2_ == 0xeb) {
        send_dlg_item_msg_1038_8b58(CONCAT22(param_2, param_1), param_6);
    } else {
        if (param_4._2_2_ != (s_vrpal_bmp_1050_183a + 0x7)) {
            pass1_1040_b54a(
                param_1,
                param_2,
                param_3,
                param_4,
                param_5,
                &ctx.PTR_LOOP_1050_1040,
                param_6,
            );
            return;
        }
        msg_box_ui_op_1038_8a3a(CONCAT22(param_2, param_1), 0x0, param_5, param_6);
    }
    return;
}

pub fn pass1_1038_8c08(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_893a(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_8caa(
    param_1: &mut Struct57,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i16,
    param_5: u16,
) -> Struct57 {
    let i_var1: &mut Struct704;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x185a));
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    &i_var1.field_0x94 = 0x0;
    param_1 = 0x90c8;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3f, param_5, param_3, param_4);
    i_var1.field_0x94 = pu_var2;
    i_var1.field_0x96 = (pu_var2 >> 0x10);
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_8cf6(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x90c8;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    unk_draw_op_1040_b0f8(param_1);
    return;
}

pub fn pass1_1038_8d7e(param_1: u32, param_2: u16) -> LRESULT {
    let LVar1: LRESULT;

    pass1_1040_78de(param_1);
    LVar1 = send_dlg_item_msg_1038_8f74(param_1, &ctx.PTR_LOOP_1050_1040, param_2);
    return LVar1;
}

pub fn pass1_1038_8d98(
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: U32Ptr,
    param_6: u16,
    param_7: u16,
) {
    if (param_4._2_2_ == 0xeb) {
        send_dlg_item_msg_1038_8f74(CONCAT22(param_2, param_1), param_6, param_7);
    } else {
        if (param_4._2_2_ != (s_vrpal_bmp_1050_183a + 0x7)) {
            pass1_1040_b54a(
                param_1,
                param_2,
                param_3,
                param_4,
                param_5,
                &ctx.PTR_LOOP_1050_1040,
                param_7,
            );
            return;
        }
        msg_box_op_1038_8dda(CONCAT22(param_2, param_1), 0x0, param_5, param_7);
    }
    return;
}

pub fn pass1_1038_90a2(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_8cf6(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_9144(param_1: U32Ptr, param_2: u16, param_3: u16) {
    let u_var1: u32;
    let u_var2: u16;
    let in_DX: U32Ptr;
    let pu_var3: U32Ptr;
    let puVar4: U32Ptr;
    let iVar5: i16;
    let iVar6: i16;
    let unaff_DI: i16;
    let uVar7: u16;
    let uVar8: u16;
    let puVar9: U32Ptr;
    let piStack8: U32Ptr;
    let i_var8: &mut Struct432;

    struct_1040_b082(param_1, CONCAT22(param_2, 0xfaa));
    // uVar7 = (param_1 >> 0x10);
    iVar5 = param_1;
    (iVar5 + 0x94) = 0x0;
    (iVar5 + 0x96) = 0x0;
    (iVar5 + 0x98) = 0x0;
    *param_1 = 0x99a2;
    (iVar5 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    (iVar5 + 0x8a) = 0x27;
    puVar9 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x28, param_3, in_DX, unaff_DI);
    // pu_var3 = (puVar9 >> 0x10);
    u_var2 = puVar9;
    (iVar5 + 0x98) = u_var2;
    (iVar5 + 0x9a) = pu_var3;
    mem_op_1000_179c(0x18, pu_var3, 0x1000);
    puVar4 = (pu_var3 | u_var2);
    if (puVar4 == 0x0) {
        (iVar5 + 0x90) = 0x0;
    } else {
        struct_1040_a598(CONCAT22(pu_var3, u_var2));
        (iVar5 + 0x90) = u_var2;
        (iVar5 + 0x92) = puVar4;
    }
    (iVar5 + 0x90) = 0x11;
    iVar6 = *(iVar5 + 0x90);
    u_var2 = iVar6 * 0xa + 0x2;
    mem_op_1000_179c(u_var2, puVar4, 0x1000);
    piStack8 = CONCAT22(puVar4, u_var2);
    if ((puVar4 | u_var2) == 0x0) {
        u_var1 = (iVar5 + 0x90);
        (u_var1 + 0x2) = 0x0;
    } else {
        *piStack8 = iVar6;
        pass1_1000_5586(
            0xa564,
            &ctx.PTR_LOOP_1050_1040,
            iVar6,
            0xa,
            u_var2 + 0x2,
            puVar4,
        );
        u_var1 = (iVar5 + 0x90);
        // uVar8 = (u_var1 >> 0x10);
        iVar6 = u_var1;
        (iVar6 + 0x2) = u_var2 + 0x2;
        (iVar6 + 0x4) = puVar4;
    }
    u_var1 = (iVar5 + 0x90);
    (u_var1 + 0xa) = 0x18;
    u_var1 = (iVar5 + 0x90);
    (u_var1 + 0x12) = (iVar5 + 0xa);
    return;
}

pub fn pass1_1038_927c(param_1: U32Ptr) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x74);
    (**ppcVar1)();
    return;
}

pub fn pass1_1038_993a(param_1: u16, param_2: u16, param_3: i16) -> i16 {
    let i_stack6: i16;

    i_stack6 = 0x0;
    loop {
        if (0xe < i_stack6) {
            return -0x1;
        }
        if ((i_stack6 * 0xe + 0x5a70) == param_3) {
            break;
        }
        i_stack6 += 0x1;
    }
    return i_stack6;
}

pub fn pass1_1038_997c(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    unk_draw_op_1040_b0f8(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_9a1e(param_1: i16, param_2: u16, param_3: u16, param_4: u32) -> u16 {
    pass1_1040_b040(
        CONCAT22(param_2, param_1),
        CONCAT22(param_4, param_3),
        (param_4 >> 0x10),
    );
    CONCAT22(param_2, param_1) = 0x9af6;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1038_9a48(param_1: &mut Struct18) {
    param_1.field_0x0 = 0x9af6;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    unk_draw_op_1040_b0f8(param_1);
    return;
}

pub fn pass1_1038_9ad0(param_1: i32, param_2: u8) -> u32 {
    pass1_1038_9a48(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_9b72(param_1: i16, param_2: u16, param_3: u16, param_4: u32) -> u32 {
    let i_stack4: i16;

    pass1_1040_b040(
        CONCAT22(param_2, param_1),
        CONCAT22(param_4, param_3),
        (param_4 >> 0x10),
    );
    (param_1 + 0x128) = 0x0;
    CONCAT22(param_2, param_1) = 0x9efa;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    i_stack4 = 0x0;
    loop {
        (param_1 + i_stack4 * 0x2 + 0x94) = 0x0;
        i_stack4 += 0x1;
        if (i_stack4 < 0x4a) == false {
            break;
        }
    }
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1038_9ed4(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    unk_draw_op_1040_b0f8(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_9f76(
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) -> Struct57 {
    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfba, param_5);
    param_1 = 0xa0b6;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_9fa4(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa0b6;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_a090(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_9fa4(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_a122(
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: u32,
) -> u16 {
    get_sys_metrics_1040_7728(
        CONCAT22(param_2, param_1),
        param_3,
        param_4,
        param_5,
        (param_5 >> 0x10),
    );
    (param_1 + 0x8e) = 0x0;
    CONCAT22(param_2, param_1) = 0xa2d0;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1038_a156(param_1: &mut Struct18) {
    param_1.field_0x0 = 0xa2d0;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_a174(param_1: u32, param_2: i16) {
    if (param_2 == 0x1) {
        (param_1 + 0x8e) = 0x0;
    }
    return;
}

pub fn pass1_1038_a2aa(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_a156(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_a33c(param_1: U32Ptr, param_2: u16) -> u16 {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, u_var1, 0x1, 0x0, CONCAT22(param_2, 0xfc7));
    *param_1 = 0xa428;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_a36a(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa428;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    pass1_1038_a156(param_1);
    return;
}

pub fn pass1_1038_a402(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_a36a(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_a494(param_1: U32Ptr, param_2: u16) -> u16 {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, u_var1, 0x1, 0x0, CONCAT22(param_2, 0xfc8));
    *param_1 = 0xa62e;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_a4c2(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa62e;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    pass1_1038_a156(param_1);
    return;
}

pub fn pass1_1038_a608(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_a4c2(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_a69a(param_1: U32Ptr, param_2: u16) -> u16 {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, u_var1, 0x1, 0x0, CONCAT22(param_2, 0xfc9));
    *param_1 = 0xa832;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_a6c8(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa832;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    pass1_1038_a156(param_1);
    return;
}

pub fn pass1_1038_a80c(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_a6c8(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_a89e(param_1: U32Ptr, param_2: u16) -> u16 {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, u_var1, 0x1, 0x0, CONCAT22(param_2, 0xfca));
    *param_1 = 0xab16;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_a8cc(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xab16;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    pass1_1038_a156(param_1);
    return;
}

pub fn pass1_1038_aaf0(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_a8cc(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_ab82(param_1: &mut Struct57, param_2: u16) -> Struct57 {
    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfd3, param_2);
    param_1 = 0xad72;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_abb0(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xad72;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_ad4c(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_abb0(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_adde(param_1: i16, param_2: u16, param_3: u16, param_4: u32) -> u16 {
    pass1_1038_9b72(param_1, param_2, param_3, param_4);
    CONCAT22(param_2, param_1) = 0xae4e;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1038_ae08(param_1: &mut Struct18) {
    param_1.field_0x0 = 0xae4e;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    unk_draw_op_1040_b0f8(param_1);
    return;
}

pub fn pass1_1038_ae28(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_ae08(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_aeca(param_1: &mut Struct20, param_2: u16) -> Struct20 {
    let u_var1: u16;
    let local_b6: u16;
    let uStack180: u16;
    let local_5c: [u8; 0x5a];

    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0xac) = 0x0;
    (param_1 + 0xae) = 0x0;
    if (ctx.PTR_LOOP_1050_5b7c == 0x0) {
        ctx.PTR_LOOP_1050_5b7c = param_1;
    }
    pass1_1000_4906(param_1, 0x0, 0xac);
    unk_draw_op_1008_80ee(CONCAT22(param_2, local_5c), param_2);
    unk_win_ui_op_1040_9854(CONCAT22(param_2, &local_b6), param_2);
    local_b6 = 0x389a;
    uStack180 = 0x1008;
    pass1_1008_8168(CONCAT22(param_2, local_5c));
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_af34() {
    ctx.PTR_LOOP_1050_5b7c = 0x0;
    return;
}

pub fn pass1_1038_af40(
    param_1: u32,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
) -> u32 {
    let ppcVar1: u32;
    let u_var2: u32;
    let pu_var3: U32Ptr;
    let puVar4: U32Ptr;
    let u_var5: u16;
    let iVar6: i16;
    let unaff_DI: i16;
    let uVar7: u16;
    let paVar8: &mut Struct57;

    pu_var3 = bring_win_to_top_1038_b72e(param_1, param_3, param_6);
    iVar6 = param_1;
    // uVar7 = (param_1 >> 0x10);
    if (pu_var3 != 0x0) {
        //goto LAB_1038_b61f;
    }
    ctx.PTR_LOOP_1050_5b82 = pu_var3;
    if (true) {
        param_6 = &ctx.PTR_LOOP_1050_1038;
        match (param_3) {
            0x1 => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x8e, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    //LAB_1038_afa0:
                    param_6 = 0x1000;
                    paVar8 = 0x0;
                } else {
                    paVar8 = pass1_1038_9f76(CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2);
                }
            }

            0x2 => {
                mem_op_1000_179c(0x96, param_4, 0x1000);
                u_var5 = param_4 | param_5;
                if (u_var5 == 0x0) {
                    //goto LAB_1038_afa0;
                    // }
                    param_6 = &ctx.PTR_LOOP_1050_1040;
                    pass1_1040_181c(
                        CONCAT22(param_4, param_5),
                        0x0,
                        0x0,
                        0x0,
                        param_2,
                        u_var5,
                        param_7,
                    );
                    paVar8 = CONCAT22(u_var5, param_5);
                }
            }

            0x3 => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x92, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_e99a(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    (param_4 | param_5),
                    param_7,
                );
            }

            0x4 => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x92, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    //goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_c7b8(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    (param_4 | param_5),
                    param_7,
                );
            }

            0x5 => {
                mem_op_1000_179c(0x96, param_4, 0x1000);
                u_var5 = param_4 | param_5;
                if (u_var5 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                pass1_1040_23ea(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    param_7,
                    u_var5,
                );
                paVar8 = CONCAT22(u_var5, param_5);
            }

            0x6 => {
                mem_op_1000_179c(0x92, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                paVar8 = pass1_1040_06e8(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    (param_4 | param_5),
                    param_7,
                );
            }

            0x7 => {
                mem_op_1000_179c(0x9c, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                pass1_1040_4068(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x8 => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x9a, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                pass1_1038_b772(
                    CONCAT22(param_4, param_5),
                    puVar4,
                    unaff_DI,
                    param_7,
                    param_2,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x9 => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x8e, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_e140(CONCAT22(param_4, param_5), 0x0, 0x0, 0x0, param_2);
            }

            0xa => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x90, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_a33c(CONCAT22(param_4, param_5), param_2);
            }

            0xb => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x90, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    //goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_a494(CONCAT22(param_4, param_5), param_2);
            }

            0xc => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x90, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    //goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_a69a(CONCAT22(param_4, param_5), param_2);
            }

            0xd => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x90, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_a89e(CONCAT22(param_4, param_5), param_2);
            }

            0xe => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x94, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                pass1_1038_e69a(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0xf => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x94, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                pass1_1038_cd06(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x10 => {
                mem_op_1000_179c(0x92, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                paVar8 = pass1_1040_0bfc(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    (param_4 | param_5),
                    unaff_DI,
                    param_7,
                );
            }

            0x11 => {
                mem_op_1000_179c(0x9a, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                pass1_1040_0e1c(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x12 => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x9a, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_d756(
                    CONCAT22(param_4, param_5),
                    param_2,
                    (param_4 | param_5),
                    unaff_DI,
                    param_7,
                );
            }

            0x13 => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x92, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_cad8(
                    CONCAT22(param_4, param_5),
                    param_2,
                    (param_4 | param_5),
                    unaff_DI,
                    param_7,
                );
            }

            0x14 => {
                mem_op_1000_179c(0xaa, param_4, 0x1000);
                u_var5 = param_4 | param_5;
                if (u_var5 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                pass1_1040_1f5a(CONCAT22(param_4, param_5), param_2, unaff_DI, param_7);
                paVar8 = CONCAT22(u_var5, param_5);
            }

            0x15 => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x8e, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_d242(CONCAT22(param_4, param_5), param_2);
            }

            0x16 => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x9a, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                pass1_1038_eeda(
                    CONCAT22(param_4, param_5),
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x17 => {
                mem_op_1000_179c(0x96, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = 0x1018;
                paVar8 = pass1_1018_5e26(CONCAT22(param_4, param_5), param_2);
            }

            _ => {}
            //       TODO: goto switchD_1038_b581_caseD_18;
            0x19 => {
                mem_op_1000_179c(0x96, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                pass1_1040_1cb4(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x1a => {
                mem_op_1000_179c(0x92, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                paVar8 = pass1_1040_123e(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    (param_4 | param_5),
                    unaff_DI,
                    param_7,
                );
            }

            0x1b => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x8e, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_ab82(CONCAT22(param_4, param_5), param_2);
            }

            0x1c => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x92, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_e2d0(CONCAT22(param_4, param_5), param_2);
            }

            0x1d => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x92, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_eb9e(CONCAT22(param_4, param_5), param_2);
            }

            0x1e => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x29e, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                pass1_1038_bddc(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x1f => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x9a, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                pass1_1038_c4a2(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x20 => {
                mem_op_1000_179c(0x29a, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                pass1_1040_2ea2(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x21 => {
                mem_op_1000_179c(0xa6, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                pass1_1040_3966(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x22 => {
                mem_op_1000_179c(0x9a, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                pass1_1040_34a2(
                    CONCAT22(param_4, param_5),
                    0x0,
                    0x0,
                    0x0,
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x23 => {
                mem_op_1000_179c(0x9c, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                pass1_1040_ac84(
                    CONCAT22(param_4, param_5),
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x25 => {
                mem_op_1000_179c(0xa0, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                pass1_1040_ca16(
                    CONCAT22(param_4, param_5),
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x26 => {
                mem_op_1000_179c(0xa2, param_4, 0x1000);
                u_var5 = param_4 | param_5;
                if (u_var5 == 0x0) {}
                //goto LAB_1038_afa0;
                param_6 = &ctx.PTR_LOOP_1050_1040;
                pass1_1040_d0f8(CONCAT22(param_4, param_5), param_2);
                paVar8 = CONCAT22(u_var5, param_5);
            }

            0x27 => {
                param_6 = 0x1000;
                mem_op_1000_179c(0xa0, param_4, 0x1000);
                u_var5 = param_4 | param_5;
                if (u_var5 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                pass1_1038_88f2(CONCAT22(param_4, param_5), param_2);
                paVar8 = CONCAT22(u_var5, param_5);
            }

            0x28 => {
                mem_op_1000_179c(0x96, param_4, 0x1000);
                puVar4 = (param_4 | param_5);
                if (puVar4 == 0x0) {
                    // goto LAB_1038_afa0;
                }
                param_6 = &ctx.PTR_LOOP_1050_1040;
                pass1_1040_6402(
                    CONCAT22(param_4, param_5),
                    param_2,
                    puVar4,
                    unaff_DI,
                    param_7,
                );
                paVar8 = CONCAT22(puVar4, param_5);
            }

            0x29 => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x98, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_7d10(
                    CONCAT22(param_4, param_5),
                    param_2,
                    param_4 | param_5,
                    unaff_DI,
                    param_7,
                );
            }

            0x2a => {
                param_6 = 0x1000;
                mem_op_1000_179c(0x98, param_4, 0x1000);
                if ((param_4 | param_5) == 0x0) {
                    // goto LAB_1038_afa0;
                }
                paVar8 = pass1_1038_8caa(
                    CONCAT22(param_4, param_5),
                    param_2,
                    (param_4 | param_5),
                    unaff_DI,
                    param_7,
                );

                (param_3 * 0x4 + iVar6) = paVar8;
                (param_3 * 0x4 + iVar6 + 0x2) = (paVar8 >> 0x10);
            }
        }
        // switchD_1038_b581_caseD_18:
        if ((param_3 * 0x4 + iVar6) != 0x0) {
            if ((iVar6 + 0xae) != 0x0) {
                u_var2 = (param_3 * 0x4 + iVar6);
                (u_var2 + 0x6e) = (iVar6 + 0xae);
            }
            (iVar6 + 0xae) = 0x0;
            u_var2 = (param_3 * 0x4 + iVar6);
            ppcVar1 = ((param_3 * 0x4 + iVar6) + 0x8);
            (**ppcVar1)(param_6, u_var2, (u_var2 >> 0x10));
        }
    }
    //LAB_1038_b61f:
    return CONCAT22((param_3 * 0x4 + iVar6 + 0x2), (param_3 * 0x4 + iVar6));
}

pub fn pass1_1038_b6e0(param_1: u32, param_2: i16) {
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;
    let uStack4: u16;

    uStack4 = 0x1;
    loop {
        if (0x2a < uStack4) {
            return;
        }
        // u_var3 = (param_1 >> 0x10);
        i_var2 = param_1;
        if ((((uStack4 * 0x4 + i_var2 + 0x2) | (uStack4 * 0x4 + i_var2)) != 0x0)
            && (u_var1 = (uStack4 * 0x4 + i_var2), (u_var1 + 0x6) == param_2))
        {
            break;
        }
        uStack4 += 0x1;
    }
    (uStack4 * 0x4 + i_var2) = 0x0;
    return;
}

pub fn pass1_1038_b772(
    param_1: &mut Struct57,
    param_2: U32Ptr,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) {
    let pu_var1: U32Ptr;
    let i_var2: &mut Struct705;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x9a, 0x0, 0xfbf, param_5);
    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    &i_var2.field_0x8e = 0x0;
    &i_var2.field_0x92 = 0x0;
    i_var2.field_0x96 = 0x1;
    i_var2.field_0x98 = 0x0;
    param_1 = 0xbd70;
    i_var2.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x36, param_4, param_2, param_3);
    // pu_var1 = (pu_var3 >> 0x10);
    i_var2.field_0x8e = pu_var3;
    i_var2.field_0x90 = pu_var1;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x6, param_4, pu_var1, param_3);
    i_var2.field_0x92 = pu_var3;
    i_var2.field_0x94 = (pu_var3 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_b7f0(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xbd70;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_bca8(param_1: u32) {
    let u_var1: u16;
    let ppcVar2: u32;
    let u_var3: u32;
    let puVar4: u32;
    let pu_var5: u32;
    let extraout_dx: U32Ptr;
    let puVar6: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let puVar7: U32Ptr;
    let i_var8: i16;
    let uVar9: u16;

    // uVar9 = (param_1 >> 0x10);
    i_var8 = param_1;
    u_var3 = (i_var8 + 0x8e);
    pu_var5 = (u_var3 + 0xa);
    ppcVar2 = (*pu_var5 + 0x14);
    (**ppcVar2)();
    puVar4 = pu_var5;
    puVar6 = extraout_dx;
    if ((i_var8 + 0x70) != 0x0) {
        puVar4 = (i_var8 + 0x70);
        u_var1 = (i_var8 + 0x72);
        puVar6 = (u_var1 | puVar4);
        if (puVar6 != 0x0) {
            ppcVar2 = *puVar4;
            (**ppcVar2)();
            puVar6 = extraout_DX_00;
        }
    }
    mem_op_1000_179c(0x14, puVar6, 0x1000);
    puVar7 = (puVar6 | puVar4);
    if (puVar7 == 0x0) {
        puVar4 = 0x0;
        puVar7 = 0x0;
    } else {
        struct_1008_4c58(CONCAT22(puVar6, puVar4));
    }
    (i_var8 + 0x70) = puVar4;
    (i_var8 + 0x72) = puVar7;
    pass1_1008_4d84(
        (i_var8 + 0x70),
        pu_var5 & 0xffff | ZEXT24(extraout_dx) << 0x10,
        puVar7,
    );
    return;
}

pub fn pass1_1038_bd4a(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_b7f0(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_bddc(
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: U32Ptr,
    param_7: i16,
    param_8: u16,
) {
    let i_var1: &mut Struct706;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x176, param_5);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    &i_var1.field_0x8e = 0x0;
    i_var1.field_0x92 = 0x0;
    i_var1.field_0x94 = 0x0;
    i_var1.field_0x96 = 0x0;
    i_var1.field_0x98 = 0x0;
    i_var1.field_0x9a = 0x0;
    i_var1.field_0x9c = 0x0;
    param_1 = 0xc436;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3b, param_8, param_6, param_7);
    i_var1.field_0x8e = pu_var2;
    i_var1.field_0x90 = (pu_var2 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_be4a(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xc436;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_be76(param_1: u16, param_2: u32, param_3: U32Ptr, param_4: i16, param_5: u16) {
    let pu_var1: U32Ptr;
    let i_var2: i16;

    if (param_2._2_2_ == 0x0) {
        i_var2 = 0x0;
        pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
        pass1_1010_038e(pu_var1, i_var2, param_5);
    }
    destroy_win_1040_7b98(CONCAT22(param_2, param_1), &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_c410(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_be4a(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_c4a2(
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: U32Ptr,
    param_7: i16,
    param_8: u16,
) {
    let i_var1: &mut Struct708;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0x17c, param_5);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    &i_var1.field_0x8e = 0x0;
    i_var1.field_0x92 = 0x0;
    i_var1.field_0x96 = 0x0;
    param_1 = 0xc74c;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3b, param_8, param_6, param_7);
    i_var1.field_0x8e = pu_var2;
    i_var1.field_0x90 = (pu_var2 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_c4fe(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xc74c;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_c52a(param_1: u16, param_2: u32, param_3: U32Ptr, param_4: i16, param_5: u16) {
    let pu_var1: U32Ptr;
    let i_var2: i16;

    if (param_2._2_2_ == 0x0) {
        i_var2 = 0x0;
        pu_var1 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
        pass1_1010_038e(pu_var1, i_var2, param_5);
    }
    destroy_win_1040_7b98(CONCAT22(param_2, param_1), &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_c726(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_c4fe(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_c7b8(
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: U32Ptr,
    param_7: u16,
) -> Struct57 {
    let i_var1: &mut Struct435;
    let unaff_DI: i16;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfb8, param_5);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    &i_var1.field_0x8e = 0x0;
    param_1 = 0xca6c;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x5, param_7, param_6, unaff_DI);
    i_var1.field_0x8e = pu_var2;
    i_var1.field_0x90 = (pu_var2 >> 0x10);
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_c80a(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xca6c;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_ca46(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_c80a(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_cad8(
    param_1: &mut Struct57,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i16,
    param_5: u16,
) -> Struct57 {
    let i_var1: &mut Struct709;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1cb, param_2);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    &i_var1.field_0x8e = 0x0;
    param_1 = 0xcc9a;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2c, param_5, param_3, param_4);
    i_var1.field_0x8e = pu_var2;
    i_var1.field_0x90 = (pu_var2 >> 0x10);
    i_var1.field_0x74 = 0x0;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_cb30(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xcc9a;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_cc74(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_cb30(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_cd06(
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: U32Ptr,
    param_7: i16,
    param_8: u16,
) {
    let i_var1: &mut Struct710;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfcc, param_5);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    &i_var1.field_0x8e = 0x0;
    i_var1.field_0x92 = 0x0;
    param_1 = 0xcf00;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x42, param_8, param_6, param_7);
    i_var1.field_0x8e = pu_var2;
    i_var1.field_0x90 = (pu_var2 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_cd5c(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xcf00;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_ceda(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_cd5c(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_d218(param_1: &mut Struct18, param_2: u8, param_3: u16) -> Struct18 {
    free_proc_inst_1038_cfda(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_d242(param_1: &mut Struct57, param_2: u16) -> Struct57 {
    let u_var1: u16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x13e, param_2);
    // u_var1 = (param_1 >> 0x10);
    param_1 = 0xd6ea;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    (param_1 + 0x74) = 0x1;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_d276(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xd6ea;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_d6c4(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_d276(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_d756(
    param_1: &mut Struct57,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i16,
    param_5: u16,
) -> Struct57 {
    let ppcVar1: u32;
    let i_var2: &mut Struct711;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x11b, param_2);
    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    i_var2.field_0x8e = 0x0;
    i_var2.field_0x90 = 0x0;
    i_var2.field_0x92 = 0x0;
    i_var2.field_0x96 = 0x0;
    param_1 = 0xe0d4;
    i_var2.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    &i_var2.field_0x92 = pu_var3;
    (&i_var2.field_0x92 + 0x2) = (pu_var3 >> 0x10);
    ppcVar1 = (*i_var2.field_0x92 + 0x4);
    (**ppcVar1)();
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_d7d0(param_1: &mut Struct18, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    param_1.field_0x0 = 0xe0d4;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    if ((i_var1 + 0x90) != 0x0) {
        pass1_1010_1ea6(ctx.PTR_LOOP_1050_02a0, param_1, param_2);
    }
    if ((i_var1 + 0x92) != 0x0) {
        pass1_1010_1ea6((i_var1 + 0x92), param_1, param_2);
    }
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (i_var1 + 0x6));
    fn_ptr_1000_17ce(ctx, (i_var1 + 0x96), 0x1000);
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_de20(
    param_1: u32,
    param_2: u16,
    param_3: u16,
    param_4: i16,
    param_5: U32Ptr,
    param_6: u16,
    param_7: u16,
) {
    let ppcVar1: u32;
    let i_var2: i16;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let local_12: [u8; 4];
    let uStack14: u16;
    let puStack12: U32Ptr;
    let puStack10: u32;
    let uStack6: u16;
    let i_stack4: i16;

    i_stack4 = 0x644;
    uStack6 = 0x0;
    uStack14 = param_4 - 0x11c;
    if (true) {
        uStack14 = param_6;
        match (param_4 - 0x11c) {
            0x0 => {
                i_stack4 = 0x635;
                uStack6 = 0x3a;
            }

            0x1 => {
                i_stack4 = 0x636;
                uStack6 = 0x3b;
            }

            0x2 => {
                i_stack4 = 0x637;
                uStack6 = 0x3c;
            }

            0x4 => {
                i_stack4 = 0x639;
                uStack6 = 0x3e;
            }

            0x5 => {
                i_stack4 = 0x63a;
                uStack6 = 0x3f;
            }

            0x6 => {
                i_stack4 = 0x63b;
                uStack6 = 0x40;
            }

            0x7 => {
                i_stack4 = 0x640;
                uStack6 = 0x45;
            }

            0x9 => {
                i_stack4 = 0x642;
                uStack6 = 0x47;
            }

            0xa => {
                i_stack4 = 0x641;
                uStack6 = 0x46;
            }

            0xb => {
                i_stack4 = 0x63f;
                uStack6 = 0x44;
            }
        }
    }
    if (i_stack4 != 0x0) {
        u_var4 = 0x1000;
        mem_op_1000_179c(0xb4, param_5, 0x1000);
        pu_var3 = (param_5 | uStack14);
        puStack12 = param_5;
        if (pu_var3 == 0x0) {
            i_var2 = 0x0;
            pu_var3 = 0x0;
        } else {
            u_var4 = SUB42(&ctx.PTR_LOOP_1050_1040, 0x0);
            i_var2 = string_1040_8520(
                CONCAT22(param_5, uStack14),
                (param_1 + 0x6),
                0x0,
                0x2,
                0x634,
                i_stack4,
                pu_var3,
                param_7,
            );
        }
        puStack10 = CONCAT22(pu_var3, i_var2);
        if (uStack6 == 0x0) {
            ppcVar1 = (*puStack10 + 0x74);
            (**ppcVar1)(u_var4, i_var2, pu_var3);
        } else {
            pass1_1008_941a(CONCAT22(param_7, local_12), 0x1, uStack6);
            ppcVar1 = (*puStack10 + 0x6c);
            (**ppcVar1)(0x1008, puStack10, (puStack10 >> 0x10), local_12, param_7);
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_df5c(param_1: u32, param_2: u16, param_3: u16) -> u32 {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u32;

    // u_var2 = (param_1 >> 0x10);
    u_var1 = param_1;
    pass1_1010_038e((u_var1 + 0x92), 0x1, param_3);
    u_var3 = pass1_1038_af40(
        ctx.PTR_LOOP_1050_5b7c,
        (u_var1 + 0x8),
        0x20,
        param_2,
        u_var1,
        0x1010,
        param_3,
    );
    return u_var3;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_df86(param_1: u32, param_2: U32Ptr, param_3: i16, param_4: u16) {
    let mut pcVar1: String;
    let ppcVar2: u32;
    let BVar3: bool;
    let u_var4: u16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u8;
    let puVar11: U32Ptr;
    let mut pcVar12: String;
    let paVar13: &mut Struct57;
    let puStack22: u32;

    puVar11 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_4, param_2, param_3);
    // u_var5 = (puVar11 >> 0x10);
    pcVar1 = (puVar11 + 0x68);
    // uVar9 = (param_1 >> 0x10);
    uVar8 = param_1;
    BVar3 = pass1_1010_041a();
    if (BVar3 != 0x0) {
        pass1_1010_038e((uVar8 + 0x92), 0x1, param_4);
        pass1_1038_af40(
            ctx.PTR_LOOP_1050_5b7c,
            (uVar8 + 0x8),
            0x1e,
            u_var5,
            uVar8,
            0x1010,
            param_4,
        );
        return;
    }
    pcVar12 = load_string_1010_847e(
        ctx.PTR_LOOP_1050_14cc,
        (ctx.PTR_LOOP_1050_14cc >> 0x10),
        0x1010,
    );
    // puVar6 = (pcVar12 >> 0x10);
    u_var4 = pcVar12;
    u_var10 = 0x0;
    mem_op_1000_179c(0xb4, puVar6, 0x1000);
    if ((puVar6 | u_var4) == 0x0) {
        uVar9 = 0x0;
        uVar7 = 0x0;
    } else {
        u_var10 = 0x40;
        paVar13 = pass1_1040_8478(
            CONCAT22(puVar6, u_var4),
            0x20,
            pcVar1,
            pcVar12,
            (uVar8 + 0x6),
            puVar6 | u_var4,
        );
        // uVar7 = (paVar13 >> 0x10);
        uVar9 = SUB42(paVar13, 0x0);
    }
    puStack22 = CONCAT22(uVar7, uVar9);
    ppcVar2 = (*puStack22 + 0x74);
    (**ppcVar2)(u_var10, uVar9, uVar7);
    return;
}

pub fn pass1_1038_e03e(param_1: u32) {
    let u_var1: u32;
    let u_var2: u16;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let i_stack6: i16;

    // u_var4 = (param_1 >> 0x10);
    u_var2 = pass1_1010_0886();
    // for (i_stack6 = 0x1; i_stack6 <= u_var2; i_stack6 += 0x1) {
    //   u_var1 = (param_1 + 0x92);
    //   u_var6 = pass1_1010_08e2(u_var1,(u_var1 >> 0x10),i_stack6);
    //   u_var1 = (param_1 + 0x96);
    //   u_var5 = (u_var1 >> 0x10);
    //   i_var3 = u_var1;
    //   if ((i_var3 + i_stack6 * 0x4) != 0x0) {
    //     enable_win_1040_9234
    //               ((i_var3 + i_stack6 * 0x4),*(u_var6 + 0x6),
    //                &ctx.PTR_LOOP_1050_1040);
    //   }
    // }
    return;
}

pub fn pass1_1038_e0ae(param_1: &mut Struct18, param_2: u8, param_3: u16) -> Struct18 {
    pass1_1038_d7d0(param_1, param_3);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_e140(
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) -> Struct57 {
    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfc2, param_5);
    param_1 = 0xe264;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_e16e(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xe264;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_e23e(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_e16e(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_e2d0(param_1: &mut Struct57, param_2: u16) -> Struct57 {
    let u_var1: u16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1c3, param_2);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x8e) = 0x0;
    param_1 = 0xe62e;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_e308(param_1: &mut Struct18) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    param_1.field_0x0 = 0xe62e;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (i_var1 + 0x6));
    fn_ptr_1000_17ce(ctx, (i_var1 + 0x8e), 0x1000);
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_e4bc(
    param_1: u16,
    param_2: u32,
    param_3: u32,
    param_4: U32Ptr,
    param_5: i16,
    param_6: u16,
) {
    let ppcVar1: u32;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let puVar6: U32Ptr;
    let extraout_dx: u16;
    let extraout_DX_00: U32Ptr;
    let puVar7: U32Ptr;
    let ppcVar8: u32;
    let puVar9: u32;
    let puVar10: U32Ptr;
    let u_var11: u16;
    let uVar12: u8;
    let uVar13: u8;
    let uVar14: u16;
    let uVar15: u16;
    let uVar16: u16;
    let puStack22: u32;

    if (param_3._2_2_ == 0x1c4) {
        puVar10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_6, param_4, param_5);
        // uVar14 = (puVar10 >> 0x10);
        u_var4 = (puVar10 + 0x24);
        u_var5 = (puVar10 + 0x26);
        u_var3 = u_var5 | u_var4;
        if (u_var3 != 0x0) {
            pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var4);
            if ((u_var5 | u_var3) != 0x0) {
                puVar9 = pass1_1008_c6fa(ctx.PTR_LOOP_1050_06e0, 0x20);
                // puVar6 = (puVar9 >> 0x10);
                u_var4 = puVar9;
                pass1_1038_4e78(u_var4, puVar6, CONCAT22(u_var5, u_var3), puVar9);
                puStack22 = CONCAT22(puVar6, u_var4);
                u_var2 = *puStack22;
                ppcVar8 = u_var2;
                ppcVar1 = ppcVar8 + 0x8;
                u_var5 = u_var4;
                (**ppcVar1)(0x1008, u_var4, puVar6);
                if ((extraout_dx | u_var5) == 0x0) {
                    if (puStack22 != 0x0) {
                        ppcVar1 = ppcVar8;
                        (**ppcVar1)(0x1008, u_var4, puVar6, 0x1);
                    }
                } else {
                    ppcVar1 = (*puStack22 + 0x4);
                    (**ppcVar1)(0x8, u_var4, puVar6, 0x0, 0x0);
                    puVar7 = extraout_DX_00;
                    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var5);
                    puVar10 = mixed_1010_20ba(
                        ctx.PTR_LOOP_1050_0ed0,
                        0x32,
                        param_6,
                        puVar7,
                        (u_var2 >> 0x10),
                    );
                    pass1_1010_71d6(
                        puVar10,
                        0x1,
                        ((ZEXT24(puVar7) & 0xff00) << 0x10 | CONCAT12(puVar7, u_var5 + 0xc)),
                        u_var5 + 0xc,
                        (puVar10 >> 0x10),
                        param_6,
                    );
                    if (puStack22 != 0x0) {
                        ppcVar1 = *puStack22;
                        (**ppcVar1)(0x1010, u_var4, puVar6, 0x1);
                    }
                }
            }
        }
    } else {
        if (param_3._2_2_ == 0x1c5) {
            uVar14 = 0xe;
        } else {
            if (param_3._2_2_ != 0x1c6) {
                post_win_msg_1040_7b3c(
                    CONCAT13((param_2 >> 0x8), CONCAT12(param_2, param_1)),
                    (param_2 >> 0x10),
                    param_3,
                    param_3._2_2_,
                    &ctx.PTR_LOOP_1050_1040,
                );
                return;
            }
            uVar14 = 0xd;
        }
        uVar16 = 0x0;
        uVar15 = 0x0;
        u_var11 = 0x0;
        uVar12 = 0x0;
        uVar13 = 0x0;
        puVar10 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x32, param_6, param_4, param_5);
        unk_win_op_1010_7300(
            puVar10,
            CONCAT13(uVar13, CONCAT12(uVar12, u_var11)),
            uVar14,
            CONCAT22(uVar16, uVar15),
        );
    }
    return;
}

pub fn pass1_1038_e608(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_e308(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_e69a(
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: U32Ptr,
    param_7: i16,
    param_8: u16,
) {
    let i_var1: &mut Struct713;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfcb, param_5);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    &i_var1.field_0x8e = 0x0;
    i_var1.field_0x92 = 0x0;
    param_1 = 0xe92e;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x43, param_8, param_6, param_7);
    i_var1.field_0x8e = pu_var2;
    i_var1.field_0x90 = (pu_var2 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_e6f0(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xe92e;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_e908(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_e6f0(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_e99a(
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: U32Ptr,
    param_7: u16,
) -> Struct57 {
    let i_var1: &mut Struct434;
    let unaff_DI: i16;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfb9, param_5);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    &i_var1.field_0x8e = 0x0;
    param_1 = 0xeb32;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1038;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x30, param_7, param_6, unaff_DI);
    i_var1.field_0x8e = pu_var2;
    i_var1.field_0x90 = (pu_var2 >> 0x10);
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_e9ec(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xeb32;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_eb0c(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_e9ec(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

pub fn pass1_1038_eb9e(param_1: &mut Struct57, param_2: u16) -> Struct57 {
    let u_var1: u16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1c7, param_2);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x8e) = 0x0;
    param_1 = 0xee6e;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_ebd6(param_1: &mut Struct18) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    param_1.field_0x0 = 0xee6e;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (i_var1 + 0x6));
    fn_ptr_1000_17ce(ctx, (i_var1 + 0x8e), 0x1000);
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn pass1_1038_ee48(param_1: &mut Struct18, param_2: u8) -> Struct18 {
    pass1_1038_ebd6(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_eeda(
    param_1: &mut Struct57,
    param_2: u16,
    param_3: U32Ptr,
    param_4: i16,
    param_5: u16,
) {
    let i_var1: &mut Struct714;
    let u_var1: u16;
    let pu_var2: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x166, param_2);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    &i_var1.field_0x8e = 0x0;
    i_var1.field_0x92 = 0x0;
    i_var1.field_0x94 = 0x0;
    param_1 = 0x67c;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x9, param_5, param_3, param_4);
    i_var1.field_0x8e = pu_var2;
    i_var1.field_0x90 = (pu_var2 >> 0x10);
    i_var1.field_0x74 = 0x1;
    return;
}

pub fn pass1_1040_0656(param_1: &mut Struct31, param_2: u8) -> Struct31 {
    destroy_win_1038_ef3a(param_1, &ctx.PTR_LOOP_1050_1038);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_06e8(
    param_1: &mut Struct57,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: U32Ptr,
    param_7: u16,
) -> Struct57 {
    let i_var1: i16;
    let unaff_DI: i16;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    get_sys_metrics_1040_7728(param_1, 0x1, param_2, 0xfbc, param_5);
    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x8e) = 0x0;
    param_1 = 0xb90;
    (i_var1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x7, param_7, param_6, unaff_DI);
    (i_var1 + 0x8e) = pu_var3;
    (i_var1 + 0x90) = (pu_var3 >> 0x10);
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1040_073a(param_1: &mut Struct18) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xb90;
    (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1038);
    return;
}

pub fn pass1_1038_4d28(a: u16) -> String {
    unimplemented!()
}
