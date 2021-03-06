use crate::defines::{Struct20, Struct87};
use crate::win_struct::WNDCLASS16;

pub fn set_window_placement_1010_0070(
    param_1: u32,
    param_2: i16,
    param_3: u16,
    param_4: HWND16,
    param_5: u16,
) {
    let ppcVar1: u32;
    let u_var2: u16;
    let pu_var3: u32;
    let lVar4: i32;
    let local_18: [u8; 6];
    let IStack18: i16;
    let iStack16: i16;
    let IStack14: i16;
    let IStack12: i16;
    let IStack10: i16;
    let IStack8: i16;
    let uStack6: u16;
    let uStack4: u16;

    local_18._0_2_ = 0x16;
    local_18._2_4_ = 0x0;
    IStack18 = 0x0;
    iStack16 = 0x0;
    IStack14 = 0x0;
    IStack12 = 0x0;
    IStack10 = 0x0;
    IStack8 = 0x0;
    uStack6 = 0x0;
    uStack4 = 0x0;
    GetWindowPlacement16(param_4, local_18);
    if ((iStack16 == -0x1) || (param_2 != 0x0)) {
        local_18._2_4_ = 0x50001;
        lVar4 = GetWindowLong16(ctx.s_tile2_bmp_1050_1538, 0x0);
        // u_var2 = (lVar4 >> 0x10);
        pu_var3 = (lVar4 + 0xe0);
        ppcVar1 = (*pu_var3 + 0x38);
        (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, pu_var3, (lVar4 + 0xe2), param_3);
        pass1_1010_01f8(param_1, CONCAT22(param_5, local_18), pu_var3);
        SetWindowPlacement16(ctx.s_tile2_bmp_1050_1538, local_18);
    }
    return;
}

pub fn set_win_placement_1010_010e(param_1: u16, param_2: u16, param_3: u16, param_4: HWND16) {
    let ppcVar1: u32;
    let i_var2: i16;
    let pi_var3: U32Ptr;
    let u_var4: u16;
    let pu_var5: u32;
    let extraout_dx: u16;
    let lVar6: i32;
    let WINDOWPLACEMENT16: local_18;
    let i_stack6: i16;
    let i_stack4: i16;

    local_18.length = 0x16;
    local_18.flags = 0x0;
    local_18.show_cmd = 0x0;
    local_18.pt_min_position.x = 0x0;
    local_18.pt_min_position.y = 0x0;
    local_18.pt_max_position.x = 0x0;
    local_18.pt_max_position.y = 0x0;
    local_18.rc_normal_position.x = 0x0;
    local_18.rc_normal_position.y = 0x0;
    i_stack6 = 0x0;
    i_stack4 = 0x0;
    GetWindowPlacement16(param_4, &local_18);
    if (local_18.rc_normal_position.x == -0x1) {
        lVar6 = GetWindowLong16(ctx.s_tile2_bmp_1050_1538, 0x0);
        // u_var4 = (lVar6 >> 0x10);
        pu_var5 = (lVar6 + 0xe0);
        ppcVar1 = (*pu_var5 + 0x1c);
        (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, pu_var5, (lVar6 + 0xe2), param_3);
        i_var2 = pu_var5;
        pi_var3 = (pu_var5 & 0xffff | extraout_dx << 0x10);
        local_18.show_cmd = 0x9;
        local_18.rc_normal_position.x = *pi_var3;
        local_18.rc_normal_position.y = (i_var2 + 0x2);
        i_stack6 = (i_var2 + 0x4) + *pi_var3;
        i_stack4 = (i_var2 + 0x2) + (i_var2 + 0x6);
        SetWindowPlacement16(ctx.s_tile2_bmp_1050_1538, &local_18);
    }
    return;
}

pub fn enum_child_windows_1010_01be(param_1: U32Ptr) {
    let pvVar1: U32Ptr;

    if (ctx.PTR_LOOP_1050_0010 == 0x0) {
        pvVar1 = MakeProcInstance16(param_1, ctx.PTR_LOOP_1050_038c);
        EnumChildWindows1(ctx.s_tile2_bmp_1050_1538, 0x0, ZEXT24(pvVar1) << 0x10);
        FreeProcInstance16(ctx.s_tile2_bmp_1050_1538);
    }
    return;
}

pub fn win_ui_op_1010_0240(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: HWND16,
    param_5: u16,
) -> bool {
    let ppcVar1: u32;
    let BVar2: bool;
    let WVar3: u16;
    let in_DX: U32Ptr;
    let unaff_DI: i16;
    let puVar4: u32;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;

    uVar7 = SUB42(ctx.data_seg, 0x0);
    u_var6 = param_3;
    BVar2 = IsWindow16(param_4);
    if (BVar2 != 0x0) {
        WVar3 = GetWindowWord16(ctx.s_tile2_bmp_1050_1538, -0x6);
        if (WVar3 == &ctx.PTR_LOOP_1050_038c) {
            u_var5 = param_3;
            BVar2 = IsIconic16(ctx.s_tile2_bmp_1050_1538);
            if (BVar2 != 0x0) {
                puVar4 = mixed_1010_20ba(&ctx.PTR_LOOP_1050_0ed0, 0x45, param_5, in_DX, unaff_DI);
                ppcVar1 = (*puVar4 + 0x10);
                (**ppcVar1)(
                    ctx.s_tile2_bmp_1050_1538,
                    puVar4,
                    0x1,
                    param_3,
                    u_var5,
                    u_var6,
                    uVar7,
                );
            }
        }
    }
    return 0x1;
}

pub fn win_ui_op_1010_3202(param_1: u32, param_2: i16, param_3: HWND16) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let hwnd: HWND16;
    let unaff_SS: u16;
    let i_stack4: i16;

    i_var3 = param_1;
    // u_var4 = (param_1 >> 0x10);
    if (param_2 == 0x0) {
        pi_var1 = (i_var3 + 0x28);
        *pi_var1 = *pi_var1 + -0xa;
        if (*pi_var1 < 0x0) {
            (i_var3 + 0x28) = 0x0;
        }
    } else {
        pi_var1 = (i_var3 + 0x28);
        *pi_var1 = *pi_var1 + (i_var3 + 0x18);
    }
    if ((i_var3 + 0x52) != 0x0) {
        i_stack4 = 0x0;
        hwnd = param_3;
        loop {
            u_var2 = (i_var3 + 0x52);
            param_3 = hwnd;
            if ((u_var2 + i_stack4 * 0x4) != 0x0) {
                param_3 = ctx.s_tile2_bmp_1050_1538;
                DestroyWindow16(hwnd);
                u_var2 = (i_var3 + 0x52);
                (u_var2 + i_stack4 * 0x4) = 0x0;
            }
            i_stack4 += 0x1;
            hwnd = param_3;
            if (i_stack4 < 0xa) == false {
                break;
            }
        }
    }
    if ((i_var3 + 0x16) == 0x0) {
        pass1_1010_32f4(param_1, (i_var3 + 0x56), unaff_SS, param_3);
    } else {
        pass1_1010_32da(
            param_1,
            (i_var3 + (i_var3 + 0x16) * 0x4 + 0x26),
            param_3,
            unaff_SS,
        );
    }
    pass1_1010_1f62(unaff_SS, param_1, 0x8);
    return;
}

pub fn ui_op_1010_79aa(
    param_1: &mut Struct20,
    param_2: i16,
    param_3: i32,
    param_4: &mut WNDCLASS16,
) {
    let u_var1: u32;
    let pu_var2: U32Ptr;
    let extraout_dx: u16;
    let u_var3: u16;
    let lStack18: i32;
    let lStack14: i32;
    let local_a: [u8; 8];

    // u_var3 = (param_1 >> 0x10);
    if (((param_1 + 0x1c) != 0x0) && (param_3 != 0x0 || (param_2 != 0x0))) {
        pass1_1008_5784(CONCAT22(param_4, local_a), (param_1 + 0x1c));
        lStack18 = 0x0;
        loop {
            pu_var2 = local_a;
            pass1_1008_5b12(pu_var2, param_4);
            lStack14 = CONCAT22(extraout_dx, pu_var2);
            if ((extraout_dx | pu_var2) == 0x0) {
                // goto
                // LAB_1010_7a49;
            }
            if (((param_2 == 0x0) && ((pu_var2 + 0x4) == param_3))
                || (param_3 == 0x0 && (u_var1 = (pu_var2 + 0x8), (u_var1 + 0xa) == param_2)))
            {
                break;
            }
            if (((pu_var2 + 0x4) != param_3)
                || (u_var1 = (pu_var2 + 0x8), (u_var1 + 0xa) != param_2))
                == false
            {
                break;
            }
        }
        lStack18 = lStack14;
        //LAB_1010_7a49:
        if (lStack18 != 0x0) {
            SetFocus16(0x1008);
            BringWindowToTop16(ctx.s_tile2_bmp_1050_1538);
            return;
        }
    }
    return;
}

pub fn show_win_1010_7a76(param_1: u32, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;
    let unaff_SS: u16;
    let lVar3: i32;
    let local_a: [u8; 8];

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x20) == 0x0) {
        (i_var1 + 0x20) = 0x1;
        pass1_1008_5784(CONCAT22(unaff_SS, local_a), (i_var1 + 0x1c));
        loop {
            lVar3 = pass1_1008_5b12(local_a, unaff_SS);
            if (lVar3 == 0x0) {
                break;
            }
            ShowWindow16(0x1008, 0x0);
        }
    }
    return;
}

pub fn show_window_1010_7ace(param_1: u32, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;
    let lVar3: i32;
    let local_a: [u8; 8];

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x20) != 0x0) {
        (i_var1 + 0x20) = 0x0;
        pass1_1008_5784(CONCAT22(param_2, local_a), (i_var1 + 0x1c));
        loop {
            lVar3 = pass1_1008_5b12(local_a, param_2);
            if (lVar3 == 0x0) {
                break;
            }
            ShowWindow16(0x1008, 0x1);
        }
    }
    return;
}

pub fn send_msg_1010_7c42(param_1: u32, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;
    let lVar3: i32;
    let local_a: [u8; 8];

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if (((i_var1 + 0x1e) | (i_var1 + 0x1c)) != 0x0) {
        pass1_1008_5784(CONCAT22(param_2, local_a), (i_var1 + 0x1c));
        loop {
            lVar3 = pass1_1008_5b12(local_a, param_2);
            if (lVar3 == 0x0) {
                break;
            }
            SendMessage16(0x1008, 0x0, 0x0, 0x11100eb);
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn send_msg_1010_7c9e(param_1: u32, param_2: i16, param_3: u16) {
    let b_var1: bool;
    let i_var2: i16;
    let u_var3: u16;
    let lVar4: i32;
    let u_var5: u32;
    let local_a: [u8; 8];

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((((i_var2 + 0x1e) | (i_var2 + 0x1c)) != 0x0) && (param_2 != 0x0)) {
        pass1_1008_5784(CONCAT22(param_3, local_a), (i_var2 + 0x1c));
        loop {
            lVar4 = pass1_1008_5b12(local_a, param_3);
            // u_var3 = (lVar4 >> 0x10);
            if (lVar4 == 0x0) {
                break;
            }
            if ((lVar4 + 0x4) != 0x0) {
                u_var5 = struct_op_1030_73a8((lVar4 + 0x4));
                b_var1 = pass1_1008_c6ae(ctx.PTR_LOOP_1050_06e0, (u_var5 + 0xc), param_2);
                if (b_var1 != 0x0) {
                    SendMessage16(0x1008, 0x0, 0x0, 0x11100eb);
                }
            }
        }
    }
    return;
}

pub fn msg_box_op_1010_8bb4(
    param_1: &mut Struct87,
    param_2: u16,
    param_3: &mut Struct87,
    param_4: HINSTANCE16,
    param_5: u16,
) {
    let mut pcVar1: String;
    let local_402: [u8; 400];

    pcVar1 = load_string_1010_847e(
        ctx.PTR_LOOP_1050_14cc,
        (ctx.PTR_LOOP_1050_14cc >> 0x10),
        param_4,
    );
    unk_str_op_1000_3d3e(CONCAT22(param_5, local_402), pcVar1);
    pass1_1000_3cea(CONCAT22(param_5, local_402), param_3);
    pcVar1 = load_string_1010_847e(
        ctx.PTR_LOOP_1050_14cc,
        (ctx.PTR_LOOP_1050_14cc >> 0x10),
        0x1000,
    );
    MessageBox16(0x1000, 0x1010, pcVar1, (pcVar1 >> 0x10));
    PostMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x11100ee);
    return;
}
