pub fn string_1010_1722(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u32,
    extraout_dx: u16,
) {
    // let extraout_dx: u16;
    let mut u_var1 = 0u16;
    let mut str_1: String;
    let local_52: [u8; 50] = [0; 50];

    pass1_1028_b58e(param_5);
    if (extraout_dx | param_2) == 0x0 {
        str_1 = load_string_1010_847e(
            ctx.PTR_LOOP_1050_14cc as i16,
            ((ctx.PTR_LOOP_1050_14cc >> 0x10) as i16),
            ctx.USHORT_1050_1028,
        );
        // u_var1 = (pcVar2 >> 0x10);
        string_1000_3d3e(
            &mut read_string_from_addr(CONCAT22(param_1, local_52[0] as u16)),
            &mut str_1,
        );
        str_1 = read_string_from_addr(CONCAT22(u_var1, local_52[0] as u16));
    } else {
        str_1 = load_string_1038_4d28(((param_2 + 0x2e) as u32));
        // param_1 = (pcVar2 >> 0x10);
    }
    str_op_1008_60e8(ctx, (str_1 & 0xffff | param_1 << 0x10), (str_1 >> 0x10));
    return;
}

use crate::global::AppContext;
use crate::mem_1000::mem_op_1000_179c;
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1010::pass1_1010_e964;
use crate::pass::pass_1028::{pass1_1028_b58e, pass1_1028_e1ec};
use crate::pass::pass_1038::load_string_1038_4d28;
use crate::string::string_1000::string_1000_3cea;
use crate::string::string_1000::{str_op_1000_3da4, string_1000_3d3e};
use crate::string::string_1008::str_op_1008_60e8;
use crate::struct_ops::struct_1010::struct_1010_dd5e;
use crate::struct_ops::struct_1030::struct_op_1030_73a8;
use crate::util::{read_string_from_addr, CONCAT22, ZEXT24};
use crate::win_struct::HINSTANCE16;
use crate::winapi::LoadString16;
use std::default::default;

pub fn unk_load_str_op_1010_2c34() -> u16 {
    let pUVar1: U32Ptr;
    let in_DX: U32Ptr;
    let in_buf_len_5: i16;
    let unaff_DI: i16;
    let unaff_SS: u16;
    let pu_var2: U32Ptr;

    pu_var2 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x3, unaff_SS, in_DX, unaff_DI);
    mem_op_1000_179c(0x80, (pu_var2 >> 0x10), 0x1000);
    // in_buf_len_5 = (pu_var2 >> 0x10);
    load_string_1010_84e0(0x1000, ctx.PTR_LOOP_1050_14cc, 0x80, pu_var2, in_buf_len_5);
    pUVar1 = string_1000_3cea(pu_var2, ctx.s__1050_11c8);
    pass1_1010_e964(in_buf_len_5, unaff_SS, unaff_DI);
    string_1000_3cea(pu_var2, CONCAT22(in_buf_len_5, pUVar1));
    return pu_var2;
}

pub fn string_1010_5286(
    param_1: u16,
    param_2: u16,
    param_3: u32,
    param_4: &mut String,
    param_5: u16,
) -> u32 {
    let mut string_1: String;
    let in_buf_len_5: U32Ptr;
    let mut string_2: String;

    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, param_3);
    in_buf_len_5 = (param_5 | param_4);
    if in_buf_len_5.is_null() {
        return 0x0;
    }
    string_1 = param_4.clone();
    mem_op_1000_179c(ctx, 0x80, in_buf_len_5, 0x1000);
    load_string_1010_84e0(
        0x1000,
        ctx.PTR_LOOP_1050_14cc,
        0x80,
        &mut string_1,
        in_buf_len_5,
    );
    string_1000_3cea(CONCAT22(in_buf_len_5, string_1), 0x105013ac);
    string_2 = load_string_1038_4d28(CONCAT22(param_5, param_4));
    string_1000_3cea(CONCAT22(in_buf_len_5, string_1), &mut string_2);
    return CONCAT22(in_buf_len_5, string_1);
}

pub fn load_string_1010_847e(param_1: i16, buf_len: i16, h_inst: HINSTANCE16) -> String {
    LoadString16(
        h_inst,
        0x3ff,
        &mut read_string_from_addr((param_1 + 0x682) as u32),
        buf_len as u16,
    );
    return read_string_from_addr(CONCAT22(buf_len as u16, ((param_1 + 0x682) as u16)));
}

pub fn load_string_1010_84ac(param_1: U32Ptr, param_3: HINSTANCE16) {
    let u_var1: u16;

    u_var1 = param_2;
    LoadString16(param_3, 0x3ff, (param_1 + 0x682), param_2);
    str_op_1008_60e8(ctx, CONCAT22(param_2, (param_1 + 0x682)), u_var1);
    return;
}

pub fn load_string_1010_84e0(
    in_hinstance_5: HINSTANCE16,
    param_2: U32Ptr,
    in_resc_id_3: u16,
    in_buffer_4: &mut String,
    in_buf_len_5: i16,
) {
    LoadString16(in_hinstance_5, in_resc_id_3, in_buffer_4, in_buf_len_5);
    return;
}

pub fn unk_load_str_op_1010_8c96(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u16,
    param_5: u16,
) -> u32 {
    let u_var1: u32;
    let IVar2: i16;
    let pu_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let in_DX: U32Ptr;
    let iVar6: i16;
    let uVar7: u16;
    let uVar8: u16;
    let in_AF: u8;
    let uVar9: u32;
    let mut pcVar10: String;
    let mut spec: String;
    let valist: U32Ptr;
    let uStack46: u32;
    let local_10: u32;
    let iStack12: i16;
    let uStack10: i16;
    let puStack8: U32Ptr;
    let uStack6: u16;
    let uStack4: u16;

    // uVar7 = (param_3 >> 0x10);
    iVar6 = param_3;
    u_var5 = (iVar6 + 0x6);
    uVar9 = u_var5;
    spec = param_2;
    // valist = (param_2 >> 0x10);
    if (u_var5 != 0x0) {
        // uVar8 = (param_1 >> 0x10);
        if (u_var5 == 0x1) {
            u_var5 = (iVar6 + 0x4) - 0x1;
            uVar9 = u_var5;
            if (false) {
                // goto switchD_1010_8e11_caseD_4;
            }
            uVar9 = param_3 & 0xffff;
            i_var4 = uVar9;
            param_4 = 0x1010;
            match (u_var5) {
                0x0 => {}
                0x1 => {
                    u_var1 = (iVar6 + 0x8);
                    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
                    local_10 = (i_var4 + 0xc);
                    iStack12 = (i_var4 + 0x10);
                    iStack10 = i_var4;
                    if (0x0 < iStack12) {
                        pcVar10 = load_string_1010_847e(
                            ctx.PTR_LOOP_1050_14cc,
                            (ctx.PTR_LOOP_1050_14cc >> 0x10),
                            &USHORT_1050_1028,
                        );
                        // uStack4 = (pcVar10 >> 0x10);
                        uStack6 = SUB42(pcVar10, 0x0);
                        IVar2 = wsprintf16(&USHORT_1050_1028, spec, valist);
                        return CONCAT22(IVar2, uStack4);
                    }
                }
                0x2 => {
                    u_var1 = (iVar6 + 0x8);
                    pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
                    local_10 = (i_var4 + 0xc);
                    iStack12 = (i_var4 + 0x10);
                    if (0x0 < iStack12) {
                        iStack12 = 0x0;
                        uVar9 = struct_op_1030_73a8(CONCAT22(in_DX, i_var4));
                        uVar9 = pass1_1028_bb24(uVar9);
                        // in_DX = (uVar9 >> 0x10);
                        iStack10 = uVar9;
                        pu_var3 = &local_10;
                        puStack8 = in_DX;
                        pass1_1030_627e(
                            param_5,
                            pu_var3,
                            in_DX,
                            ctx.PTR_LOOP_1050_5740,
                            CONCAT22(param_5, pu_var3),
                            uVar9,
                        );
                        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, pu_var3, in_DX);
                        u_var1 = (param_1 + 0xa);
                        pass1_1010_c3c2(
                            u_var1,
                            (u_var1 >> 0x10),
                            0x0,
                            CONCAT22(in_DX, pu_var3),
                            in_DX,
                            in_AF,
                            param_5,
                        );
                        uStack46 = CONCAT22(in_DX, pu_var3);
                        pcVar10 = load_string_1010_847e(
                            ctx.PTR_LOOP_1050_14cc,
                            (ctx.PTR_LOOP_1050_14cc >> 0x10),
                            &USHORT_1050_1028,
                        );
                        // uStack4 = (pcVar10 >> 0x10);
                        uStack6 = SUB42(pcVar10, 0x0);
                        wsprintf16(&USHORT_1050_1028, spec, valist);
                        //           TODO: goto LAB_1010_8def;
                    }
                }
                _ => {}
                //         TODO: goto switchD_1010_8e11_caseD_4;
                0x4 => {}
                0x7 => {}
                0x8 => {}
                0xa => {} //         TODO: goto LAB_1010_8ea5;
            }
            uVar9 = ZEXT24(&local_10);
            param_4 = &USHORT_1050_1028;
        } else {
            uVar9 = (u_var5 - 0x2);
            if (u_var5 - 0x2 == 0x0) {
                i_var4 = (iVar6 + 0x4);
                u_var5 = i_var4 - 0x4;
                if (u_var5 != 0x0) {
                    u_var5 = i_var4 - 0xc;
                    uVar9 = u_var5;
                    if (u_var5 != 0x0) {
                        // goto LAB_1010_8ea5;
                    }
                }
                u_var1 = (iVar6 + 0x8);
                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
                u_var1 = (param_1 + 0xa);
                pass1_1010_c3c2(
                    u_var1,
                    (u_var1 >> 0x10),
                    0x0,
                    CONCAT22(in_DX, u_var5),
                    in_DX,
                    in_AF,
                    param_5,
                );
                uStack46 = CONCAT22(in_DX, u_var5);
                pcVar10 = load_string_1010_847e(
                    ctx.PTR_LOOP_1050_14cc,
                    (ctx.PTR_LOOP_1050_14cc >> 0x10),
                    &USHORT_1050_1028,
                );
                // uStack4 = (pcVar10 >> 0x10);
                uStack6 = SUB42(pcVar10, 0x0);
                wsprintf16(&USHORT_1050_1028, spec, valist);
                //LAB_1010_8def:
                fn_ptr_1000_17ce((uStack46 & 0xffff | ZEXT24(in_DX) << 0x10), 0x1000);
                return CONCAT22(uStack46, in_DX);
            }
        }
    }
    //LAB_1010_8ea5:
    load_string_1010_84e0(
        param_4,
        ctx.PTR_LOOP_1050_14cc,
        (ctx.PTR_LOOP_1050_14cc >> 0x10),
        0x3ff,
        spec,
        valist,
    );
    //switchD_1010_8e11_caseD_4:
    return CONCAT22(uVar9, in_DX);
}

pub fn load_string_1010_9432(param_1: HINSTANCE16) -> U32Ptr {
    let mut pcVar1: String;

    pcVar1 = load_string_1010_847e(
        ctx.PTR_LOOP_1050_14cc,
        (ctx.PTR_LOOP_1050_14cc >> 0x10),
        param_1,
    );
    return pcVar1;
}

pub fn load_string_1010_ac92(
    param_1: HINSTANCE16,
    param_2: u16,
    param_3: u16,
    param_4: i16,
) -> U32Ptr {
    let mut pcVar1: String;

    if ((0x0 < param_4) && (param_4 < 0x43)) {
        pcVar1 = load_string_1010_847e(
            ctx.PTR_LOOP_1050_14cc,
            (ctx.PTR_LOOP_1050_14cc >> 0x10),
            param_1,
        );
        return pcVar1;
    }
    return 0x0;
}

pub fn string_op_1010_ada6(
    param_1: HINSTANCE16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
    param_6: i16,
) -> String {
    let mut pcVar1: String;
    let mut pcStack6: String;

    pcStack6 = 0x0;
    if (param_6 == 0x6) {
        if (param_5 == 0x0) {
            // goto LAB_1010_adee;
        }
        pcVar1 = string_op_1020_c222(param_5);
    } else {
        if (param_6 != 0x7) {
            return 0x0;
        }
        if (param_5 == 0x0) {
            // goto LAB_1010_adee;
        }
        pcVar1 = string_op_1020_c2f8(param_5);
    }
    param_1 = 0x1020;
    pcStack6 = CONCAT22(param_2, pcVar1);
    //LAB_1010_adee:
    if (pcStack6 == 0x0) {
        pcStack6 = load_string_1010_847e(
            ctx.PTR_LOOP_1050_14cc,
            (ctx.PTR_LOOP_1050_14cc >> 0x10),
            param_1,
        );
    }
    return pcStack6;
}

pub fn string_op_1010_c446(
    param_1: u16,
    param_2: u8,
    uparam_3: &mut String,
    param_4: u32,
    param_5: &mut String,
    param_6: u32,
) {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u32;
    let mut pcVar5: String;
    let u_var6: u16;
    let uVar7: u16;
    let mut in_buffer_4: String;
    let in_buf_len_5: U32Ptr;
    let uStack22: u16;
    let mut pcStack6: String;

    pcStack6 = param_5;
    if (param_5 == 0x0) {
        mem_op_1000_179c(0x100, param_3, 0x1000);
        pcStack6 = (param_5 & 0xffff | ZEXT24(param_3) << 0x10);
    }
    u_var4 = struct_op_1030_73a8(param_6);
    // u_var2 = (u_var4 >> 0x10);
    u_var3 = u_var2;
    struct_1010_dd5e(param_4, (param_4 >> 0x10), param_6);
    i_var1 = (u_var4 + 0x12);
    if (0x6 < i_var1 - 0x3) {
        return;
    }
    in_buffer_4 = pcStack6;
    // in_buf_len_5 = (pcStack6 >> 0x10);
    uVar7 = ctx.PTR_LOOP_1050_14cc;
    u_var6 = (ctx.PTR_LOOP_1050_14cc >> 0x10);
    match (i_var1) {
        0x6 => {
            load_string_1010_84e0(0x1010, uVar7, 0x3ff, &mut in_buffer_4, in_buf_len_5);
            uStack22 = str_op_1000_3da4(&mut pcStack6);
            pcVar5 = load_string_1010_847e(
                ctx.PTR_LOOP_1050_14cc,
                (ctx.PTR_LOOP_1050_14cc >> 0x10),
                0x1000,
            );
            uVar7 = pcVar5;
            u_var6 = ctx.s_____s__lu_1050_38d7;
        }
        //     TODO: goto LAB_1010_c4f9;
        0x7 => {}
        0x9 => {}
        0x8 => {
            load_string_1010_84e0(0x1010, uVar7, 0x3ff, &mut in_buffer_4, in_buf_len_5);
            uStack22 = str_op_1000_3da4(&mut pcStack6);
            pcVar5 = load_string_1010_847e(
                ctx.PTR_LOOP_1050_14cc,
                (ctx.PTR_LOOP_1050_14cc >> 0x10),
                0x1000,
            );
            uVar7 = pcVar5;
            u_var6 = ctx.s_____s__lu_1050_38cd;
            //LAB_1010_c4f9:
            sys_1000_3f9c(
                (in_buffer_4 + uStack22),
                in_buf_len_5,
                u_var6,
                ctx.data_seg,
                uVar7,
                &stack0xfffe,
                u_var3,
                0x1000,
                param_1,
                param_2,
            );
            return;
        }
        _ => {}
    }
    load_string_1010_84e0(0x1010, uVar7, 0x3ff, &mut in_buffer_4, in_buf_len_5);
    return;
}

pub fn string_1010_dcac(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: i16,
    param_5: u32,
    param_6: &mut Struct104,
) -> i16 {
    let u_var1: u32;
    let i_var2: &mut Struct105;
    let u_var2: u16;
    let u_var3: u16;
    let iVar5: &mut Struct104;
    let u_var6: u16;
    let uVar7: u16;
    let mut pcVar4: String;

    pcVar4 = load_string_1010_847e(
        ctx.PTR_LOOP_1050_14cc,
        (ctx.PTR_LOOP_1050_14cc >> 0x10),
        param_1,
    );
    // u_var6 = (param_6 >> 0x10);
    iVar5 = param_6;
    u_var2 = (&iVar5.field_0x2 + 0x2);
    i_var2 = (&iVar5.field_0x2 + param_4 * 0xa);
    // uVar7 = (param_5 >> 0x10);
    i_var2.field_0x4 = (param_4 * 0x2 + param_5);
    string_1040_a626(CONCAT22(u_var2, i_var2), pcVar4, u_var2);
    unk_str_op_1000_3d3e(pcVar4, 0x10503941);
    u_var2 = param_4 + 0x1;
    u_var1 = iVar5.field_0x2;
    u_var3 = u_var1 + u_var2 * 0xa;
    (u_var3 + 0x4) = (u_var2 * 0x2 + param_5);
    string_1040_a626((u_var1 & 0xffff0000 | u_var3), pcVar4, u_var2);
    return u_var2;
}

pub fn load_str_1010_ddf6(param_1: u32, param_2: u32) {
    let in_buf_len_5: i16;
    let u_var1: u32;

    // in_buf_len_5 = (param_1 >> 0x10);
    *(param_1 + 0x13c) = 0x0;
    u_var1 = struct_op_1030_73a8(param_2);
    match (u_var1 + 0x12) {
        0x1 => {}
        0x2 => {}
        0x4 => {}
        0x7 => {}
        0x9 => {}
        0x3 => {}
        0x5 => {}
        0x6 => {}
        0x8 => {}
        _ => {} //     TODO: goto switchD_1010_de53_caseD_9;
    }
    load_string_1010_84e0(
        0x1010,
        ctx.PTR_LOOP_1050_14cc,
        0x3ff,
        (param_1 + 0x13c),
        in_buf_len_5,
    );
    // switchD_1010_de53_caseD_9:
    return;
}
