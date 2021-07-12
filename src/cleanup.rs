use crate::{
    defines::{Struct11, Struct18, Struct27, Struct29, Struct65, StructB},
    fn_ptr::{
        fn_ptr_1000::fn_ptr_1000_17ce,
        util::{get_fn_ptr_1, get_fn_ptr_2},
    },
    global::AppContext,
    pass::{
        pass_1008::pass1_1008_57c4,
        pass_1010::{pass1_1010_1dda, pass1_1010_1ea6},
        pass_1040::pass1_1040_c60e,
    },
    ui::ui_1008::set_sys_color_1008_357e,
    util::CONCAT22,
    win_struct::{HDC16, HICON16, HMENU16, HWND16, PAINTSTRUCT16},
    winapi::{
        DeleteDC16, DeleteObject16, DestroyIcon16, DestroyMenu16, DestroyWindow16, GetWindowWord16,
        IsDlgButtonChecked, IsWindow16, PostMessage16, SelectPalette16, SendMessage16,
        ShowWindow16,
    },
};

pub fn cleanup_ui_op_1008_0618(
    ctx: &mut AppContext,
    param_1: &mut StructB,
    unaff_CS: u16,
    unaff_SS: u16,
) {
    let pu_var1: u32;
    let u_var2: u16;
    // let paVar3: &mut Struct18;
    let mut pa_var3: *mut Struct18;
    // let ppcVar4: u32;
    let mut ppc_var4: u32;
    let i_var5: i16;
    let u_var6: u16;
    // let unaff_CS: u16;
    // HICON16 h_icon;
    let mut h_icon: HICON16;
    // let unaff_SS: u16;
    let u_var7: u16;
    let u_var8: u16;

    // u_var6 = (param_1 >> 0x10);
    // i_var5 = param_1;
    param_1.field_0x0 = 0x389e;
    param_1.field_0x2 = 0x1008;
    set_sys_color_1008_357e(param_1, 0x0, unaff_CS, unaff_SS);
    pa_var3 = &mut param_1.field_0xf8; // (i_var5 + 0xf8);
                                       // u_var8 = (pa_var3 >> 0x10);
    h_icon = 0x1000;
    fn_ptr_1000_17ce(ctx, pa_var3, 0x1000);
    if (i_var5 + 0xec) != 0x0 {
        u_var8 = i_var5 + 0xec;
        h_icon = ctx.s_tile2_bmp_1050_1538;
        DestroyMenu16(0x1000);
    }
    u_var7 = i_var5 + 0xc2;
    DestroyIcon16(h_icon);
    (i_var5 + 0xc2) = 0x0;
    pu_var1 = i_var5 + 0xe0;
    u_var2 = i_var5 + 0xe2;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var4 = *pu_var1;
        (**ppc_var4)(
            ctx.s_tile2_bmp_1050_1538,
            pu_var1,
            u_var2,
            0x1,
            u_var7,
            pa_var3,
        );
    }
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var5 + 0xd2)));
    *param_1 = 0x380a;
    (i_var5 + 0x2) = 0x1008;
    *param_1 = 0x389a;
    (i_var5 + 0x2) = 0x1008;
    return;
}

pub fn destroy_win_1008_628e(param_1: i32, param_2: HWND16) {
    let mut ppcVar1: u32;
    ppcVar1 = ((param_1 + 0xd2) + 0x14);
    let fn_ptr = get_fn_ptr_2(ppcVar1);
    fn_ptr(param_2, (param_1 + 0xd2), param_1._2_2_);
    DestroyWindow16(param_2);
    (param_1 + 0x8) = 0x0;
    return;
}

pub fn destroy_win_1008_9698(param_1: HWND16) {
    DestroyWindow16(param_1);
    return;
}

pub unsafe fn unk_destroy_win_op_1010_2fa0(ctx: &mut AppContext, param_1: i32, param_2: HWND16) {
    let pi_var1: *mut i16;
    let u_var2: u32;
    let i_var3: i16;
    let u_var4: u16;
    // let HVar5: HWND16;
    let mut h_var5: HWND16;
    let i_stack4: i16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0x28) = 0x0;
    i_stack4 = 0x0;
    loop {
        pi_var1 = i_var3 + 0x16;
        if *pi_var1 == i_stack4 || *pi_var1 < i_stack4 {
            break;
        }
        DestroyWindow16(param_2);
        i_stack4 += 0x1;
        param_2 = ctx.s_tile2_bmp_1050_1538;
    }
    (i_var3 + 0x16) = 0x0;
    if ((i_var3 + 0x54) | (i_var3 + 0x52)) != 0x0 {
        i_stack4 = 0x0;
        loop {
            u_var2 = i_var3 + 0x52;
            h_var5 = param_2;
            if (u_var2 + i_stack4 * 0x4) != 0x0 {
                h_var5 = ctx.s_tile2_bmp_1050_1538;
                DestroyWindow16(param_2);
                u_var2 = i_var3 + 0x52;
                (u_var2 + i_stack4 * 0x4) = 0x0;
            }
            i_stack4 += 0x1;
            param_2 = h_var5;
            if i_stack4 >= 0xa {
                break;
            }
        }
        fn_ptr_1000_17ce(ctx, i_var3 + 0x52, 0x1000);
        (i_var3 + 0x52) = 0x0;
    }
    return;
}

pub unsafe fn unk_destroy_win_op_1010_305a(
    ctx: &mut AppContext,
    param_1: &mut Struct27,
    param_2: i16,
    param_3: &mut Struct65,
    param_4: u16,
    unaff_SS: u16,
) {
    let pi_var1: *mut i16;
    let u_var2: i32;
    let l_var3: i32;
    let b_var4: bool;
    let u_var5: u16;
    // Struct27 *iVar4;
    let mut i_var4: &mut Struct27;
    let i_var6: i16;
    // Struct27 *uVar7;
    let mut u_var7: &mut Struct27;
    let u_var8: u16;
    // let hwnd: HWND16;
    let mut hwnd: HWND16;
    // let hwnd_00: HWND16;
    let mut hwnd_00: HWND16;
    // let unaff_SS: u16;
    let u_stack10: i16;
    let i_stack8: i16;
    let i_stack6: i16;

    hwnd = ctx.PTR_LOOP_1050_1040;
    u_var5 = pass1_1040_c60e(param_3);
    // uVar7 = (Struct27 *)(param_1 >> 0x10);
    i_var4 = param_1;
    i_var4.field_0x12 = u_var5;
    i_var4.field_0x14 = 0x0;
    i_stack6 = 0x0;
    b_var4 = false;
    i_var4.field_0x28 = 0x0;
    i_stack8 = 0x0;
    loop {
        pi_var1 = &i_var4.field_0x16;
        if (*pi_var1 == i_stack8 || *pi_var1 < i_stack8) {
            //LAB_1010_30ad:
            i_var6 = i_stack6;
            if (bVar4) {
                while (
                    i_stack8 = i_var6 + 0x1,
                    pi_var1 = &i_var4.field_0x16,
                    *pi_var1 != i_stack8 && i_stack8 <= *pi_var1,
                ) {
                    DestroyWindow16(hwnd);
                    (&i_var4.field_0x2e)[i_var6] = 0x0;
                    hwnd = ctx.s_tile2_bmp_1050_1538;
                    i_var6 = i_stack8;
                }
                i_var4.field_0x16 = i_stack6 + 0x1;
                pass1_1010_1f62(unaff_SS, param_1, 0x9);
            } else {
                i_var6 = i_var4.field_0x16;
                (&i_var4.field_0x2a)[i_var6 * 0x2] = param_3;
                (&i_var4.field_0x2c)[i_var6 * 0x2] = (param_3 >> 0x10);
                i_stack10 = 0xa;
                pi_var1 = &i_var4.field_0x16;
                *pi_var1 = *pi_var1 + 0x1;
                if (0x1 < i_var4.field_0x16) {
                    u_var2 = (&i_var4.field_0x22)[i_var4.field_0x16];
                    i_var6 = u_var2;
                   // u_var8 = (u_var2 >> 0x10);
                    i_stack10 = (i_var6 + 0x20) + (i_var6 + 0x24) + 0x8;
                }
                hwnd = &ctx.PTR_LOOP_1050_1040;
                mov_update_win_1040_93aa(
                    param_3,
                    i_stack10,
                    i_var4.field_0x1a,
                    &ctx.PTR_LOOP_1050_1040,
                );
            }
            if (!b_var4) {
                pass1_1010_1f62(unaff_SS, param_1, 0xa);
            }
            if (param_2 == 0x0) {
                if (i_var4.field_0x52 != 0x0) {
                    i_stack8 = 0x0;
                    hwnd_00 = hwnd;
                    loop {
                        l_var3 = i_var4.field_0x52;
                       // u_var8 = (l_var3 >> 0x10);
                        i_var6 = l_var3;
                        hwnd = hwnd_00;
                        if ((i_var6 + i_stack8 * 0x4) != 0x0)
                            && ((iVar6 + iStack8 * 0x4) != param_3)
                        {
                            hwnd = ctx.s_tile2_bmp_1050_1538;
                            DestroyWindow16(hwnd_00);
                        }
                        lVar3 = iVar4.field_0x52;
                        (lVar3 + iStack8 * 0x4) = 0x0;
                        iStack8 += 0x1;
                        hwnd_00 = hwnd;
                        if i_stack8 >= 0xa {
                            break;
                        }
                    }
                }
                pass1_1010_32da(param_1, param_3, hwnd, unaff_SS);
                pass1_1010_1f62(unaff_SS, param_1, 0x8);
            }
            return;
        }
        if (&i_var4.field_0x2a + iStack8 * 0x2) == param_3 {
            bVar4 = true;
            i_stack6 = i_stack8;
            // goto LAB_1010_30ad;
        }
        iStack8 += 0x1;
    }
}

pub fn destroy_window_1010_7b26(param_1: u32, param_2: i32, param_3: u16, param_4: u16) -> u32 {
    let u_var1: u16;
    let pu_var2: *mut u8;
    let extraout_DX: u16;
    let i_var2: i16;
    let u_var4: u16;
    let local_a: [u8; 0x8];

   // u_var4 = (param_1 >> 0x10);
    i_var2 = param_1;
    u_var1 = (i_var2 + 0x1e) | (i_var2 + 0x1c);
    if (u_var1 != 0x0) {
        pass1_1008_5784(CONCAT22(param_3, local_a), (i_var2 + 0x1c));
        loop {
            pu_var2 = local_a;
            pass1_1008_5b12(pu_var2, param_3);
            param_4 = extraout_DX | pu_var2;
            if (param_4 == 0x0) {
                break;
            }
            if (pu_var2 + 0x4) == param_2 {
                break;
            }
        }
        u_var1 = extraout_DX | pu_var2;
        if (u_var1 != 0x0) {
            u_var1 = DestroyWindow16(0x1008);
        }
    }
    return CONCAT22(u_var1, param_4);
}

pub unsafe fn clenaup_win_ui_1018_4d22(
    ctx: &mut AppContext,
    in_struct_1: &mut Struct11,
    in_hdc_2: HDC16,
    unaff_SS: u16,
) {
    let u_var1: u16;
    let ppc_var23: u32;
    let local_struct_1: Struct11;
    let u_var4: &mut Struct11;
    let unaff_SS: u16;
    let pu_var2: *mut libc::c_ulong;
    let pu_var1: *mut libc::c_ulong;

   // u_var4 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1 = (ctx.s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12);
    local_struct_1.field_0x2 = 0x1018;
    if (local_struct.field_0x12 != 0x0) {
        SelectPalette16(in_hdc_2, 0x0, local_struct_1.field_0x1a);
        DeleteObject16(ctx.s_tile2_bmp_1050_1538);
        in_hdc_2 = ctx.s_tile2_bmp_1050_1538;
        DeleteDC16(ctx.s_tile2_bmp_1050_1538);
    }
    pu_var1 = local_struct_1.field_0xa;
    u_var1 = local_struct_1.field_0xc;
    if ((u_var1 | pu_var1) != 0x0) {
        ppc_var23 = *pu_var1;
        (**ppc_var23)(in_hdc_2, pu_var1, u_var1, 0x1);
    }
    pu_var2 = local_struct_1.field_0xe;
    u_var1 = local_struct_1.field_0x10;
    if ((u_var1 | pu_var2) != 0x0) {
        ppc_var23 = *pu_var2;
        (**ppc_var23)(in_hdc_2, pu_var2, u_var1, 0x1);
    }
    ctx._PTR_LOOP_1050_4230 = 0x0;
    pass1_1010_1d80(in_struct_1, unaff_SS);
    return;
}

pub fn unk_destroy_window_op_1018_6bb6(
    ctx: &mut AppContext,
    param_1: &mut Struct28,
    param_2: HWND16,
) {
    let b_var1: bool;
    let i_var2: &mut Struct28;
    let u_var2: u16;
    // let hwnd: HWND16;
    let hwnd: HWND16;

   // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    hwnd = param_2;
    if (i_var2.field_0xea != 0x0) {
        hwnd = ctx.s_tile2_bmp_1050_1538;
        PostMessage16(param_2, 0x0, 0x0, CONCAT22(0x111, i_var2.field_0xea));
    }
    PostMessage16(hwnd, 0x0, 0x0, 0x1110079);
    if (i_var2.field_0xf0 != 0x0) {
        b_var1 = IsWindow16(ctx.s_tile2_bmp_1050_1538);
        if (b_var1 != 0x0) {
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
            i_var2.field_0xf0 = 0x0;
        }
    }
    return;
}

pub fn destroy_window_1018_c518(ctx: &mut AppContext, param_1: &mut Struct29) {
    let b_var1: bool;
    let i_var2: &mut Struct29;
    let u_var3: u16;

   // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    param_1 = 0xc8bc;
    i_var2.field_0x2 = 0x1018;
    fn_ptr_1000_17ce(ctx, &i_var2.field_0x108, 0x1000);
    if (i_var2.field_0x112 != 0x0) {
        b_var1 = IsWindow16(0x1000);
        if (b_var1 != 0x0) {
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
            i_var2.field_0x112 = 0x0;
        }
    }
    pass1_1020_022c(param_1);
    return;
}

pub fn delete_palette_1018_e16c(param_1: u32, param_2: HWND16) {
    let pu_var1: u32;
    let ppc_var2: u32;
    let u_var3: u32;
    let b_force_background: *mut HDC16;
    let local_24: HDC16;
    let local_22: PAINTSTRUCT16;

    local_24 = BeginPaint16(param_2, &local_22);
    u_var3 = (param_1 + 0x6);
    pu_var1 = (u_var3 + 0xa);
    b_force_background = &local_24;
    u_var3 = *pu_var1;
    ppc_var2 = (u_var3 + 0x8);
    (**ppc_var2)(
        ctx.s_tile2_bmp_1050_1538,
        pu_var1,
        (pu_var1 >> 0x10),
        b_force_background,
    );
    ppc_var2 = (u_var3 + 0x4);
    (**ppc_var2)(ctx.s_tile2_bmp_1050_1538, pu_var1, 0x0, &local_24);
    SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x0, b_force_background);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    EndPaint16(ctx.s_tile2_bmp_1050_1538, &local_22);
    return;
}

pub fn cleanup_ui_op_1020_1038(ctx: &mut AppContext, param_1: u32, unaff_CS: HICON16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    // HICON16 unaff_CS;
    let u_var6: u16;

   // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var6 = (i_var4 + 0xc2);
    DestroyIcon16(unaff_CS);
    (i_var4 + 0xc2) = 0x0;
    (i_var4 + 0x8) = 0x0;
    pu_var1 = (i_var4 + 0xf6);
    u_var2 = (i_var4 + 0xf8);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(ctx.s_tile2_bmp_1050_1538, pu_var1, u_var2, 0x1, u_var6);
    }
    (i_var4 + 0xf6) = 0x0;
    pass1_1010_1dda((i_var4 + 0xf2));
    (i_var4 + 0xf2) = 0x0;
    return;
}

pub fn destroy_window_1020_1d4a(ctx: &mut AppContext, param1: i32, param_2: i16, param_3: HWND16) {
    let b_var1: bool;
    let hwnd: HWND16;

    if (param_2 != 0x0) {
        b_var1 = post_win_msg_1020_1ca4(param_1);
        if (b_var1 != 0x0) {
            hwnd = param_3;
            if ((param_1 + 0x96) != 0x0) {
                hwnd = ctx.s_tile2_bmp_1050_1538;
                PostMessage16(param_3, 0x0, 0x0, 0x11100ee);
            }
            DestroyWindow16(hwnd);
        }
    }
    return;
}

pub fn destroy_win_1020_1dea(ctx: &mut AppContext, param_1: HWND16) -> bool {
    let b_var1: bool;
    let w_var2: u16;

    b_var1 = IsWindow16(param_1);
    if (b_var1 != 0x0) {
        w_var2 = GetWindowWord16(ctx.s_tile2_bmp_1050_1538, -0xc);
        if (w_var2 == 0x175) {
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
            return 0x0;
        }
    }
    return 0x1;
}

pub fn destroy_win_1020_1e1e(ctx: &mut AppContext, param_1: HWND16) -> u16 {
    let b_var1: bool;
    let w_var2: u16;

    b_var1 = IsWindow16(param_1);
    if (b_var1 != 0x0) {
        w_var2 = GetWindowWord16(ctx.s_tile2_bmp_1050_1538, -0xc);
        if ((w_var2 != 0x1) && (w_var2 != 0x175)) {
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
        }
    }
    return 0x1;
}

pub fn destroy_icon_1020_2c88(ctx: &mut AppContext, param_1: u32, param_2: HICON16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;

   // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var6 = (i_var4 + 0xc2);
    DestroyIcon16(param_2);
    (i_var4 + 0xc2) = 0x0;
    (i_var4 + 0x8) = 0x0;
    pu_var1 = (i_var4 + 0xf6);
    u_var2 = (i_var4 + 0xf8);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(ctx.s_tile2_bmp_1050_1538, pu_var1, u_var2, 0x1, u_var6);
    }
    (i_var4 + 0xf6) = 0x0;
    pass1_1010_1dda((i_var4 + 0xf2));
    (i_var4 + 0xf2) = 0x0;
    return;
}

pub fn cleanup_win_ui_1020_2fea(
    ctx: &mut AppContext,
    in_struct_1: &mut Struct12,
    in_dc_handle_2: HDC16,
    unaff_SS: u16,
) {
    let i_var1: &mut Struct12;
    let var2: u16;
    // let unaff_SS: u16;

   // var2 = (in_struct_1 >> 0x10);
    i_var1 = in_struct_1;
    in_struct_1.offset_field_0x0 = 0x363c;
    i_var1.offset_field_0x2 = 0x1020;
    if (i_var1.field_0x14 != 0x0) {
        in_dc_handle_2 = 0x1010;
        pass1_1010_1ea6(
            i_var1.field_0x14,
            in_struct_1 & 0xffff | var2 << 0x10,
            unaff_SS,
        );
    }
    SelectObject16(in_dc_handle_2, i_var1.field_0x1c);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, i_var1.field_0x1e);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x0, i_var1.field_0x20);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    DeleteDC16(ctx.s_tile2_bmp_1050_1538);
    in_struct_1.offset_field_0x0 = 0x3ab0;
    i_var1.offset_field_0x2 = 0x1008;
    in_struct_1.offset_field_0x0 = 0x389a;
    i_var1.offset_field_0x2 = 0x1008;
    return;
}

pub fn destroy_window_1020_3b3e(param_1: &mut Struct30, param_2: HWND16) {
    let puVar1: u32;
    let ppcVar2: u32;
    let uVar3: u16;
    let paVar4: &mut Struct30;
    let uVar5: &mut Struct30;
    let uVar6: &mut Struct30;
    let HVar5: HWND16;
    let unaff_SS: u16;

   // uVar6 = (param_1 >> 0x10);
    uVar5 = param_1;
    uVar5.field_0x10e = 0x0;
    HVar5 = param_2;
    if (uVar5.field_0x10a != 0x0) {
        HVar5 = ctx.s_tile2_bmp_1050_1538;
        DestroyWindow16(param_2);
    }
    puVar1 = uVar5.field_0xf6;
    uVar3 = uVar5.field_0xf8;
    if ((uVar3 | puVar1) != 0x0) {
        ppcVar2 = *puVar1;
        (**ppcVar2)(HVar5, puVar1);
    }
    &uVar5.field_0xf6 = 0x0;
    if (uVar5.field_0xfa != 0x0) {
        uVar3 = uVar6 | uVar5;
        if (param_1 == 0x0) {
            paVar4 = 0x0;
        } else {
            uVar3 = &uVar5.field_0xf2;
            paVar4 = uVar6;
        }
        pass1_1010_1ea6(uVar5.field_0xfa, CONCAT22(paVar4, uVar3), unaff_SS);
    }
    uVar5.field_0xfa = 0x0;
    return;
}

pub unsafe fn destroy_cursor_1020_42f4(param_1: *mut u16, param_2: u16) {
    let iVar1: i16;
    let uVar2: u16;
    let h_cursor: HMENU16;

   // uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0x623c;
    (iVar1 + 0x2) = 0x1020;
    (iVar1 + 0xe2) = 0x62d8;
    (iVar1 + 0xe4) = 0x1020;
    h_cursor = param_2;
    if ((iVar1 + 0x106) != 0x0) {
        h_cursor = ctx.s_tile2_bmp_1050_1538;
        DestroyMenu16(param_2);
    }
    DestroyCursor16(h_cursor);
    DestroyCursor16(ctx.s_tile2_bmp_1050_1538);
    pass1_1020_808e(param_1);
    return;
}

pub fn unk_destroy_win_op_1020_694c(
    param_1: i32,
    param_2: u16,
    param_3: HWND16,
    param_4: u16,
) -> u16 {
    let uVar1: u32;
    let uVar2: u16;
    let HVar3: HWND16;
    let iVar4: i16;
    let paVar5: &mut Struct43;
    let uVar6: u16;

    uVar2 = param_2;
    if (param_2 != 0x12b) {
        iVar4 = param_1;
       // uVar6 = (param_1 >> 0x10);
        if (param_2 < 0x12c) {
            if (param_2 == 0x6f) {
                paVar5 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc, 0x1f8, param_4);
                uVar2 = WinHelp16(
                    0x1010,
                    (s_New_failed_in_Op__Op_1050_0020 + 0x9),
                    0x0,
                    CONCAT22(paVar5, 0x1),
                );
                return uVar2;
            }
            if (param_2 == 0xeb) {
                uVar2 = GetDlgItem16(param_3, 0x1797);
                if (uVar2 != 0x0) {
                    //LAB_1020_6a6f:
                    win_ui_fn_1020_6e98(param_1, ctx.s_tile2_bmp_1050_1538, param_4);
                    return uVar2;
                }
            } else {
                uVar2 = param_2 - 0xef;
                if (uVar2 == 0x0) {
                    pass1_1018_2e28((iVar4 + 0xf2));
                    pass1_1008_3e0e(param_1);
                } else {
                    uVar2 = param_2 - 0x129;
                    if ((uVar2 != 0x0) && (uVar2 = param_2 - 0x12a, uVar2 == 0x0)) {
                        uVar6 = 0xf012;
                        //LAB_1020_69c3:
                        uVar2 = PostMessage16(param_3, 0x0, 0x0, CONCAT22(0x112, uVar6));
                        return uVar2;
                    }
                }
            }
        } else {
            if (param_2 == 0xbb8) {
                HVar3 = GetDlgItem16(param_3, 0x1797);
                if (HVar3 != 0x0) {
                    DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
                }
                uVar2 = pass1_1018_31d0((iVar4 + 0xf2));
                if (uVar2 != 0x0) {
                    uVar1 = (iVar4 + 0xf2);
                    uVar2 = pass1_1018_2d9a(uVar1, (uVar1 >> 0x10));
                    //LAB_1020_6a0b:
                    invalidate_rect_1020_735a((iVar4 + 0xf6), 0x1018);
                    return uVar2;
                }
            } else {
                if (param_2 < 0xbb9) {
                    if (param_2 == 0x12c) {
                        uVar6 = 0xf020;
                        //             TODO: goto LAB_1020_69c3;
                    }
                    uVar2 = param_2 - 0x12d;
                    if (param_2 != 0x12c) {
                        uVar2 = param_2 - 0x12e;
                    }
                } else {
                    if (param_2 == 0xbb9) {
                        HVar3 = GetDlgItem16(param_3, 0x1797);
                        if (HVar3 != 0x0) {
                            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
                        }
                        uVar2 = pass1_1018_31d0((iVar4 + 0xf2));
                        if (uVar2 != 0x0) {
                            uVar1 = (iVar4 + 0xf2);
                            uVar2 = pass1_1018_2dde(uVar1, (uVar1 >> 0x10));
                            //               TODO: goto LAB_1020_6a0b;
                        }
                    } else {
                        uVar2 = param_2 - 0xbba;
                        if (uVar2 == 0x0) {
                            uVar2 = GetDlgItem16(param_3, 0x1797);
                            if (uVar2 != 0x0) {
                                uVar2 = DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
                                return uVar2;
                            }
                            //               TODO: goto LAB_1020_6a6f;
                        }
                    }
                }
            }
        }
    }
    return uVar2;
}

pub fn destroy_icon_1020_6bd2(param_1: u32, param_2: u8, param_3: HICON16) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let iVar4: i16;
    let uVar5: u16;
    let uVar6: u16;

   // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar6 = (iVar4 + 0xc2);
    DestroyIcon16(param_3);
    (iVar4 + 0xc2) = 0x0;
    (iVar4 + 0x8) = 0x0;
    puVar1 = (iVar4 + 0xf6);
    uVar2 = (iVar4 + 0xf8);
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(ctx.s_tile2_bmp_1050_1538, puVar1, uVar2, 0x1, uVar6);
    }
    (iVar4 + 0xf6) = 0x0;
    pass1_1010_1dda((iVar4 + 0xf2));
    (iVar4 + 0xf2) = 0x0;
    return;
}

pub fn cleanup_menu_ui_op_1020_795c(in_struct_1: &mut Struct3, in_menu_handle_2: HMENU16) {
    let local_struct_1: &mut Struct3;
    let uVar1: &mut Struct3;

   // uVar1 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.address_offset_field_0x0 = 0x7b86;
    local_struct_1.address_offset_field_0x2 = 0x1020;
    if (local_struct_1.field_0xec != 0x0) {
        DestroyMenu16(in_menu_handle_2);
    }
    pass1_1008_57c4((in_struct_1 & 0xffff0000 | &local_struct_1.field_0xd2));
    in_struct_1.address_offset_field_0x0 = 0x380a;
    local_struct_1.address_offset_field_0x2 = 0x1008;
    in_struct_1.address_offset_field_0x0 = 0x389a;
    local_struct_1.address_offset_field_0x2 = 0x1008;
    return;
}

pub fn destroy_window_1020_8250(param_1: u32, param_2: HWND16) {
    let BVar1: bool;
    let uVar2: u16;

   // uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0xec) != 0x0) {
        BVar1 = IsWindow16(param_2);
        if (BVar1 != 0x0) {
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
            (param_1 + 0xec) = 0x0;
        }
    }
    return;
}

pub fn destroy_window_1038_7d88(param_1: i32, param_2: u16) {
    let in_DX: u16;

    pass1_1008_b544((param_1 + 0x94), param_2, in_DX, 0x1008);
    DestroyWindow16(0x1008);
    return;
}

pub fn destroy_window_1038_a072(param_1: u16, param_2: u16, param_3: i16, param_4: HWND16) {
    if (param_3 != 0x0) {
        DestroyWindow16(param_4);
    }
    return;
}

pub fn destroy_win_1038_a3d2(param_1: i32, param_2: HWND16) {
    GetWindowWord16(param_2, -0x8);
    PostMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1110105);
    destroy_win_1040_7b98(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn destroy_window_1038_cc00(param_1: i16, param_2: u16, param_3: u16, param_4: i32) {
    let uVar1: u16;
    let in_DX: *mut u8;
    let unaff_DI: i16;
    WNDCLASS16 * unaff_SS;
    let i_var2: i16;

    uVar1 = param_4._2_2_ - 0x1cd;
    if (uVar1 == 0x0) {
        i_var2 = 0x1;
    } else {
        uVar1 = param_4._2_2_ - 0x1ce;
        if (uVar1 == 0x0) {
            i_var2 = 0x2;
        } else {
            uVar1 = param_4._2_2_ - 0x1cf;
            if (uVar1 == 0x0) {
                i_var2 = 0x3;
            } else {
                uVar1 = param_4._2_2_ - 0x1d0;
                if (uVar1 == 0x0) {
                    i_var2 = 0x4;
                } else {
                    uVar1 = param_4._2_2_ - 0x1d1;
                    if (uVar1 != 0x0) {
                        post_win_msg_1040_7b3c(
                            CONCAT22(param_2, param_1),
                            param_3,
                            param_4,
                            param_4._2_2_,
                            &ctx.PTR_LOOP_1050_1040,
                        );
                        return;
                    }
                    i_var2 = 0x5;
                }
            }
        }
    }
    pass1_1008_eb74((param_1 + 0x8e), i_var2, in_DX, unaff_DI, unaff_SS);
    if (uVar1 != 0x0) {
        win_1008_5c7c(
            ctx._PTR_LOOP_1050_02a0,
            CONCAT22(uVar1, 0x1),
            unaff_SS,
            uVar1,
            in_DX,
        );
        DestroyWindow16(0x1008);
    }
    return;
}

pub fn destroy_window_1038_cd88(param_1: &mut Struct1) {
    let unaff_SS: u16;

    dialog_ui_fn_1040_78e2(param_1, &ctx.PTR_LOOP_1050_1040);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(&ctx.PTR_LOOP_1050_1040, 0x5);
    (param_1 + 0x92) = 0x1;
    unk_win_msg_op_1008_9510((param_1 & 0xffff0000 | (param_1 + 0x92)), 0x1008, unaff_SS);
    DestroyWindow16(0x1008);
    return;
}

pub fn destroy_win_1038_e1dc(param_1: u16, param_2: u16, param_3: i16, param_4: HWND16) {
    let UVar1: u16;
    let lparam: LPARAM;

    if (param_3 != 0x0) {
        UVar1 = IsDlgButtonChecked(param_4, 0x1807);
        if (UVar1 == 0x0) {
            param_4 = ctx.s_tile2_bmp_1050_1538;
            UVar1 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538, 0x1806);
            if (UVar1 == 0x0) {
                // TODO: goto LAB_1038_e229;
            }
            lparam = 0x1110130;
        } else {
            lparam = 0x111012f;
        }
        param_4 = ctx.s_tile2_bmp_1050_1538;
        SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, lparam);
    }
    //LAB_1038_e229:
    DestroyWindow16(param_4);
    return;
}

pub fn destroy_win_1038_ef3a(param_1: &mut Struct31, param_2: HWND16) {
    let iVar1: &mut Struct31;
    let uVar1: &mut Struct31;

   // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1 = 0x67c;
    iVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    if (iVar1.field_0x96 != 0x0) {
        DestroyWindow16(param_2);
        iVar1.field_0x96 = 0x0;
    }
    pass1_1038_b6e0(ctx.PTR__LOOP_1050_5b7c, iVar1.field_0x6);
    ui_cleanup_op_1040_782c(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn destroy_win_1040_5256(param_1: &mut Struct34, param_2: HWND16) {
    let pUVar1: &mut u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let Bvar4: bool;
    let iVar5: &mut Struct34;
    let uVar5: u16;
    let HVar6: HWND16;

   // uVar5 = (param_1 >> 0x10);
    iVar5 = param_1;
    HVar6 = param_2;
    if (iVar5.field_0xb6 != 0x0) {
        HVar6 = ctx.s_tile2_bmp_1050_1538;
        BVar4 = IsWindow16(param_2);
        if (BVar4 != 0x0) {
            HVar6 = ctx.s_tile2_bmp_1050_1538;
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
        }
    }
    iVar5.field_0xb6 = 0x0;
    pUVar1 = iVar5.field_0x94;
    uVar2 = iVar5.field_0x96;
    if ((uVar2 | pUVar1) != 0x0) {
        ppcVar3 = *pUVar1;
        (**ppcVar3)(HVar6, pUVar1, uVar2, 0x1);
    }
    &iVar5.field_0x94 = 0x0;
    iVar5.field_0x98 = 0x0;
    return;
}

pub fn destroy_win_1040_7b98(param_1: i32, param_2: HWND16) {
    if ((param_1 + 0x74) == 0x0) {
        DestroyWindow16(param_2);
    }
    return;
}

pub fn destroy_win_1040_8212(param_1: i32, param_2: HWND16) {
    let is_window: bool;
    let uVar1: u16;

   // uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x8c) != 0x0) {
        is_window = IsWindow16(param_2);
        if (is_window != 0x0) {
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
            (param_1 + 0x8c) = 0x0;
        }
    }
    return;
}

pub fn destroy_win_1040_8b7e(param_1: HWND16) {
    DestroyWindow16(param_1);
    return;
}

pub unsafe fn destroy_window_1040_b726(param_1: *mut u32, param_2: i16, in_win_handle_3: HWND16) {
    let ppcVar1: u32;

    if (param_2 != 0x0) {
        ppcVar1 = (*param_1 + 0x78);
        (**ppcVar1)(in_win_handle_3, param_1);
    }
    DestroyWindow16(in_win_handle_3);
    return;
}

pub fn destroy_win_1040_bb78(param_1: &mut Struct35, param_2: HWND16) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let Bvar4: bool;
    let iVar5: &mut Struct35;
    let uVar5: u16;
    let HVar6: HWND16;

   // uVar5 = (param_1 >> 0x10);
    iVar5 = param_1;
    HVar6 = param_2;
    if (iVar5.field_0xb6 != 0x0) {
        HVar6 = ctx.s_tile2_bmp_1050_1538;
        BVar4 = IsWindow16(param_2);
        if (BVar4 != 0x0) {
            HVar6 = ctx.s_tile2_bmp_1050_1538;
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
        }
    }
    iVar5.field_0xb6 = 0x0;
    puVar1 = iVar5.field_0x94;
    uVar2 = iVar5.field_0x96;
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(HVar6, puVar1, uVar2, 0x1);
    }
    &iVar5.field_0x94 = 0x0;
    iVar5.field_0x98 = 0x0;
    return;
}
