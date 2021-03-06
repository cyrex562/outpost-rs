use crate::win_platform::types::{HGDIOBJ16, HWND16};
use crate::win_platform::winapi::DeleteObject16;

pub fn realize_palette_1020_2992(param_1: u32, param_2: i16)

{
    code      **ppcVar1;
    u32 *puVar2;

    if(param_2 != 0x0)
    {
        puVar2  = pass1_1018_0a50(*(param_1 + 0xf2));
        ppcVar1 = (*puVar2 + 0x18);
        (**ppcVar1)(0x1018, puVar2, (puVar2 >> 0x10));
        UnrealizeObject16(0x1018);
        GetDC16((HWND16)s_tile2_bmp_1050_1538);
        RealizePalette16(s_tile2_bmp_1050_1538);
    }
    return;
}

pub fn invalidate_rect_1020_2ae4(u32 *param_1, param_2: u16, HWND16 param_3, param_4: u16)

{
    code      **ppcVar1;
     let mut cVar2: u8;
    i16         iVar3;
     let mut in_DX: *mut u8;
     let mut uVar4: u16;
     let mut uVar5: u16;
    i16         unaff_DI;
    u16        *puVar6;
    u32         uVar7;
    astruct_43 *paVar8;
     let mut uVar9: u16;
     let mut uVar10: u16;

    if(param_2 != 0x129)
    {
        uVar5 = param_1;
        uVar9 = (param_1 >> 0x10);
        if(0x129 < param_2)
        {
            if(param_2 == 0x12a)
            {
                uVar9 = 0xf012;
            }
            else
            {
                if(param_2 == 0x12b)
                {
                    return;
                }
                if(param_2 == 0x12c)
                {
                    uVar9 = 0xf020;
                }
                else
                {
                    if(param_2 == 0x12d)
                    {
                        return;
                    }
                    if(param_2 != 0x12e)
                    {
                        return;
                    }
                    uVar9 = 0xf060;
                }
            }
            PostMessage16(param_3, 0x0, 0x0, CONCAT22(0x112, uVar9));
            return;
        }
        if(param_2 == 0xfb)
        {
            puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x30, param_4, in_DX, unaff_DI);
            pass1_1010_375e(puVar6);
            ppcVar1 = (*param_1 + 0x14);
            (**ppcVar1)();
            uVar7 = pass1_1010_375e(puVar6);
            uVar4 = (uVar7 >> 0x10);
            pass1_1018_181c(*(uVar5 + 0xf2), (uVar7 & 0xffff | uVar4 << 0x10), (u8)(uVar7 & 0xffff), uVar4);
            return;
        }
        if(param_2 < 0xfc)
        {
            cVar2 = param_2;
            if(cVar2 == 'o')
            {
                paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1f8, param_4);
                WinHelp16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0xa), 0x0, CONCAT22(paVar8, 0x1));
                return;
            }
            if(cVar2 == 'r')
            {
                iVar3  = uVar5 + 0xa;
                uVar10 = uVar9;
                puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x30, param_4, in_DX, unaff_DI);
                uVar4  = (puVar6 >> 0x10);
                pass1_1010_3770(puVar6, CONCAT22(uVar10, iVar3), uVar4);
                pass1_1038_af40(_PTR_LOOP_1050_5b7c, (uVar5 + 0x8), 0x3, uVar4, uVar5, &PTR_LOOP_1050_1038, param_4);
                return;
            }
            if(cVar2 == 'u')
            {
                pass1_1018_0a76(*(uVar5 + 0xf2), param_4);
                InvalidateRect16(0x1018, 0x0, 0x0);
                return;
            }
        }
    }
    return;
}

pub fn pass1_1020_0a52(param_1: u32, param_2: u16, param_3: u16, param_4: u16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
    u32 uVar3;

    uVar2 = (param_1 >> 0x10);
    uVar1 = param_1;
    unk_draw_op_1020_0c3e((uVar1 + 0xe2), param_3);
    if((uVar1 + 0xe6) == 0x0)
    {
        (uVar1 + 0xe6)               = 0x1;
        (_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
        uVar3                        = pass1_1038_af40(_PTR_LOOP_1050_5b7c, (uVar1 + 0x8), 0x6, param_2, uVar1, &PTR_LOOP_1050_1038, param_4);
        (uVar1 + 0xe8)               = uVar3;
        (uVar1 + 0xea)               = (uVar3 >> 0x10);
    }
    return;
}

pub fn unk_draw_op_1020_0c3e(param_1: u32, HWND16 param_2)

{
    u32   *puVar1;
    code        **ppcVar2;
    u32    uVar3;
    HDC16        *b_force_background;
    i16           iVar4;
    i16           iVar5;
     let mut uVar6: u16;
     let mut uVar7: u16;
     let mut uStack40: u16;
    HDC16         local_24;
    PAINTSTRUCT16 local_22;

    uVar6    = (param_1 >> 0x10);
    iVar4    = param_1;
    local_24 = BeginPai1616(param_2, &local_22);
    uVar3    = (iVar4 + 0x6);
    uVar7    = (uVar3 >> 0x10);
    iVar5    = uVar3;
    puVar1   = (iVar5 + 0xa);
    uStack40 = puVar1;
    if((*(iVar5 + 0xc) | uStack40) != 0x0)
    {
        b_force_background = &local_24;
        uVar3              = *puVar1;
        ppcVar2            = (uVar3 + 0x8);
        (**ppcVar2)(s_tile2_bmp_1050_1538, uStack40, (puVar1 >> 0x10), b_force_background);
        ppcVar2 = (uVar3 + 0x4);
        (**ppcVar2)(s_tile2_bmp_1050_1538, puVar1, (iVar4 + 0xc), (iVar4 + 0xa), &local_24);
        SelectPalette16(s_tile2_bmp_1050_1538, 0x0, (BOOL16)b_force_background);
        DeleteObject16(s_tile2_bmp_1050_1538);
    }
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}

pub fn realize_palette_1020_0e46(param_1: u32, param_2: i16, HGDIOBJ16 param_3)

{
    u32 *puVar1;
    code      **ppcVar2;
    u32  uVar3;
    i16         iVar4;
     let mut uVar5: u16;

    if(param_2 != 0x0)
    {
        uVar3   = (param_1 + 0xf2);
        uVar5   = (uVar3 >> 0x10);
        iVar4   = uVar3;
        puVar1  = (iVar4 + 0x66);
        ppcVar2 = (*puVar1 + 0x18);
        (**ppcVar2)(param_3, puVar1, (iVar4 + 0x68));
        UnrealizeObject16(param_3);
        RealizePalette16(s_tile2_bmp_1050_1538);
    }
    return;
}


pub fn pass1_1020_1022(param_1: u32, param_2: u16)

{
    draw_op_1020_15de((param_1 + 0xf6), param_2);
    return;
}

pub fn cleanup_ui_op_1020_1038(param_1: u32)

{
    u32 *puVar1;
     let mut uVar2: u16;
    code      **ppcVar3;
    i16         iVar4;
     let mut uVar5: u16;
    HICON16     unaff_CS;
     let mut uVar6: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar6 = (iVar4 + 0xc2);
    DestroyIcon16(unaff_CS);
    (iVar4 + 0xc2) = 0x0;
    (iVar4 + 0x8)  = 0x0;
    puVar1         = *(iVar4 + 0xf6);
    uVar2          = *(iVar4 + 0xf8);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)(s_tile2_bmp_1050_1538, puVar1, uVar2, 0x1, uVar6);
    }
    (iVar4 + 0xf6) = 0x0;
    pass1_1010_1dda(*(iVar4 + 0xf2));
    (iVar4 + 0xf2) = 0x0;
    return;
}

pub fn invalidate_rect_1020_157c(param_1: u32, param_2: i16, HWND16 param_3)

{
    BOOL16 BVar1;
    RECT16 local_a;
     let mut uStack4: u16;

    if(param_2 == 0x1)
    {
        (param_1 + 0x14) = 0x0;
        return;
    }
    if(param_2 == 0x2)
    {
        BVar1 = IsIconic16(param_3);
        if(BVar1 == 0x0)
        {
            GetClientRect16((HWND16)s_tile2_bmp_1050_1538, &local_a);
            uStack4 = 0x9a;
            InvalidateRect16((HWND16)s_tile2_bmp_1050_1538, 0x0, (BOOL16)&local_a);
            return;
        }
    }
    return;
}


pub fn draw_op_1020_15de(param_1: u32, HWND16 in_win_handle_2)

{
    u32           uVar1;
    code        **ppcVar2;
    BOOL16        BVar3;
     let mut uVar4: u16;
    i16           iVar5;
     let mut uVar6: u16;
    HWND16        hwnd;
     let mut unaff_SS: u16;
    u32           uVar7;
     let mut uVar8: u16;
     let mut uVar9: u16;
    HDC16         local_24;
    PAINTSTRUCT16 local_22;

    uVar6    = (param_1 >> 0x10);
    iVar5    = param_1;
    uVar9    = (iVar5 + 0x4);
    local_24 = BeginPai1616(in_win_handle_2, &local_22);
    uVar8    = (iVar5 + 0x4);
    hwnd     = (HWND16)s_tile2_bmp_1050_1538;
    BVar3    = IsIconic16((HWND16)s_tile2_bmp_1050_1538);
    if(BVar3 == 0x0)
    {
        hwnd  = 0x1010;
        uVar7 = pass1_1010_454a(*(iVar5 + 0x14));
        uVar4 = (uVar7 >> 0x10);
        if((uVar4 | uVar7) != 0x0)
        {
            uVar1 = *(iVar5 + 0x14);
            hwnd  = 0x1008;
            pass1_1008_4480(*(iVar5 + 0x18), (uVar1 & 0xffff0000 | (uVar1 + 0x76)), (astruct_76 *)(uVar7 & 0xffff | uVar4 << 0x10), unaff_SS);
        }
        ppcVar2 = ((iVar5 + 0x18) + 0x4);
        (**ppcVar2)(hwnd, (iVar5 + 0x18), 0x0, &local_24, unaff_SS, uVar8, uVar9);
    }
    else
    {
        draw_op_1020_1674(param_1, s_tile2_bmp_1050_1538);
    }
    EndPaint16(hwnd, &local_22);
    return;
}


pub fn draw_op_1020_1674(param_1: u32, param_2: u16)

{
    code  **ppcVar1;
     let mut uVar2: u16;
     let mut local_1a: u16;
     let mut uStack24: u16;
    i16     iStack22;
    i16     iStack20;
    i16     iStack18;
    i16     iStack16;
    RECT16  local_e;
    i16     iStack10;
    i16     iStack8;
    RECT16 *pRStack6;
    i16     iStack4;

    if(PTR_LOOP_1050_0010 == 0x0)
    {
        uVar2   = (param_1 >> 0x10);
        ppcVar1 = ((param_1 + 0x14) + 0x2c);
        iStack4 = (**ppcVar1)(param_2, (param_1 + 0x14));
        if(iStack4 != 0x0)
        {
            pRStack6 = GetStockObject16(param_2);
            GetClientRect16((HWND16)s_tile2_bmp_1050_1538, &local_e);
            local_1a = 0x0;
            uStack24 = 0x0;
            iStack22 = (iStack10 - local_e.x) + -0x1;
            iStack20 = (iStack8 - local_e.y) + -0x1;
            iStack18 = iStack20;
            iStack16 = iStack22;
            FillRect16(s_tile2_bmp_1050_1538, pRStack6, &local_1a);
            DrawIcon16(s_tile2_bmp_1050_1538, iStack4, 0x2, 0x2);
        }
    }
    return;
}

pub fn pass1_1018_e5dc(param_1: u16, Struct20 *param_2, param_3: u16, param_4: u16)

{
     let mut extraout_DX: *mut u8;
     let mut uVar1: u16;
    Struct20 *iVar2;
    i16         unaff_DI;
     let mut uVar2: u16;
    u16        *puVar3;

    unk_draw_op_1020_7f7a(param_2, 0x9, CONCAT22(param_4, param_3));
    uVar2                                    = (param_2 >> 0x10);
    iVar2                                    = (Struct20 *)param_2;
    &iVar2[0x1].field_0xc                    = 0x0;
    iVar2[0x1].field_0x10                    = 0x0;
    param_2->field_0x0                       = 0xe790;
    iVar2->field_0x2                         = 0x1018;
    ((Struct20 *)(iVar2 + 0x1))->field_0x0 = 0xe82c;
    iVar2[0x1].field_0x2                     = 0x1018;
    puVar3                                   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0xa, param_1, extraout_DX, unaff_DI);
    uVar1                                    = (puVar3 >> 0x10);
    &iVar2[0x1].field_0x10                   = puVar3;
    (&iVar2[0x1].field_0x10 + 0x2)           = uVar1;
    &iVar2[0x1].field_0x4                    = &iVar2[0x1].field_0x10;
    (&iVar2[0x1].field_0x4 + 0x2)            = uVar1;
    return;
}

pub fn pass1_1018_e834(astruct_660 *param_1, param_2: u16, param_3: u16, param_4: i16, param_5: u16)

{
    code     **ppcVar1;
    u32 uVar2;
    i16        iVar3;
     let mut extraout_DX: *mut u8;
     let mut uVar4: u16;
    u16       *puVar5;

    set_struct_op_1020_921c(CONCAT22(param_2, param_1), param_3);
    &param_1->field_0x14       = 0x0;
    CONCAT22(param_2, param_1) = 0xe912;
    param_1->field_0x2         = 0x1018;
    puVar5                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0xa, param_5, extraout_DX, param_4);
    uVar4                      = (puVar5 >> 0x10);
    param_1->field_0x14        = puVar5;
    param_1->field_0x16        = uVar4;
    param_1->field_0x6         = param_1->field_0x14;
    param_1->field_0x8         = uVar4;
    uVar2                      = &param_1->field_0x14;
    iVar3                      = &param_1->field_0xa;
    ppcVar1                    = ((uVar2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_1->field_0x12 = iVar3;
    draw_op_1020_9364(CONCAT22(param_2, param_1), 0x1020, param_5);
    return;
}

pub fn pass1_1018_e8bc(mut param_1: *mut u16)

{
    astruct_577 *iVar1;
     let mut uVar1: u16;

    uVar1            = (param_1 >> 0x10);
    iVar1            = (astruct_577 *)param_1;
    *param_1         = 0xe912;
    iVar1->field_0x2 = 0x1018;
    if(iVar1->field_0x14 != 0x0)
    {
        pass1_1010_1dda(iVar1->field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

pub fn pass1_1018_e91e(Struct20 *param_1, param_2: u16, param_3: u16, param_4: u16)

{
    u32         *puVar1;
    code       **ppcVar2;
    u16         *puVar3;
     let mut extraout_DX: *mut u8;
     let mut puVar4: *mut u8;
     let mut uVar5: u16;
    i16          unaff_DI;
    u16         *puVar6;
     let mut uVar7: u16;
    astruct_127 *iVar7;

    iVar7 = (astruct_127 *)param_1;
    uVar7 = (param_1 >> 0x10);
    unk_draw_op_1020_7f7a(param_1, 0x3, CONCAT22(param_3, param_2));
    iVar7->field_0xee          = 0x0;
    &iVar7->field_0xf2         = 0x0;
    iVar7->field_0xf6          = 0x0;
    param_1->field_0x0         = 0xebd0;
    iVar7->field_0x2           = 0x1018;
    iVar7->field_0xe2          = 0xec6c;
    iVar7->field_0xe4          = 0x1018;
    puVar6                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x28, param_4, extraout_DX, unaff_DI);
    puVar4                     = (puVar6 >> 0x10);
    iVar7->field_0xf2          = puVar6;
    iVar7->field_0xf4          = puVar4;
    iVar7->field_0xe6          = iVar7->field_0xf2;
    iVar7->field_0xe8          = puVar4;
    puVar6                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_4, puVar4, unaff_DI);
    &iVar7->field_0xf6         = puVar6;
    (&iVar7->field_0xf6 + 0x2) = (puVar6 >> 0x10);
    if(param_1 == (Struct20 *)0x0)
    {
        puVar3 = 0x0;
        uVar5  = 0x0;
    }
    else
    {
        puVar3 = &iVar7->field_0xe2;
        uVar5  = uVar7;
    }
    puVar1  = iVar7->field_0xf6;
    ppcVar2 = (*iVar7->field_0xf6 + 0x4);
    (**ppcVar2)(0x1010, puVar1, (puVar1 >> 0x10), 0xb, puVar3, uVar5);
    return;
}

pub fn pass1_1018_ec74(astruct_661 *param_1, param_2: i16, param_3: u16, param_4: u16)

{
    u32  *puVar1;
    code       **ppcVar2;
    u32   uVar3;
     let mut uVar4: u16;
     let mut extraout_DX: *mut u8;
     let mut puVar5: *mut u8;
    u16         *puVar6;
    u32          uVar7;
     let mut uVar8: u16;
     let mut uVar9: u16;
    astruct_661 *paVar10;
    i16          iVar11;

    set_struct_op_1020_921c(CONCAT22(param_2, param_1), param_3);
    param_1->field_0x14 = 0x0;
    pass1_1008_3e38(CONCAT22(param_2, &param_1->field_0x18));
    puVar6                       = pass1_1008_3e38(CONCAT22(param_2, &param_1->field_0x1e));
    &param_1->field_0x24         = 0x0;
    CONCAT22(param_2, param_1)   = 0x1cc;
    param_1->field_0x2           = 0x1020;
    puVar6                       = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x28, param_4, (puVar6 >> 0x10), param_2);
    uVar4                        = (puVar6 >> 0x10);
    &param_1->field_0x14         = puVar6;
    (&param_1->field_0x14 + 0x2) = uVar4;
    uVar9                        = 0x0;
    uVar8                        = &param_1->field_0x14;
    ppcVar2                      = (*param_1->field_0x14 + 0x4);
    paVar10                      = param_1;
    iVar11                       = param_2;
    (**ppcVar2)();
    param_1->field_0x6 = param_1->field_0x14;
    puVar1             = param_1->field_0x14;
    pass1_1010_2b50(puVar1, (puVar1 >> 0x10), CONCAT22(param_2, &param_1->field_0x18));
    uVar7               = pass1_1010_2b66(param_1->field_0x14);
    param_1->field_0x24 = uVar7;
    param_1->field_0x26 = (uVar7 >> 0x10);
    puVar1              = param_1->field_0x14;
    puVar1              = (puVar1 + 0xa);
    uVar3               = CONCAT22(param_2, &param_1->field_0xa);
    ppcVar2             = (*puVar1 + 0x8);
    (**ppcVar2)(0x1010, puVar1, (puVar1 >> 0x10), uVar3, uVar8, uVar4, uVar9, paVar10, iVar11);
    param_1->field_0x12 = uVar3;
    puVar5              = extraout_DX;
    draw_op_1020_9364(CONCAT22(param_2, param_1), 0x1020, param_4);
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, puVar5, param_2);
    pass1_1008_3f62(CONCAT22(param_2, &param_1->field_0x1e), (puVar6 & 0xffff0000 | (puVar6 + 0xe)));
    pass1_1008_3f32(CONCAT22(param_2, &param_1->field_0x18), CONCAT22(param_2, &param_1->field_0x1e));
    return;
}

pub fn pass1_1018_ed98(mut param_1: *mut u16, param_2: u16)

{
    astruct_579 *iVar1;
     let mut uVar1: u16;

    uVar1            = (param_1 >> 0x10);
    iVar1            = (astruct_579 *)param_1;
    *param_1         = 0x1cc;
    iVar1->field_0x2 = 0x1020;
    if(iVar1->field_0x14 != 0x0)
    {
        pass1_1010_1ea6(iVar1->field_0x14, param_1 & 0xffff | uVar1 << 0x10, param_2);
        pass1_1010_1dda(iVar1->field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

pub fn invalidate_rect_1018_edd8(param_1: u32, param_2: i16, param_3: u16)

{
    i16 iVar1;
     let mut uVar2: u16;
    u32 uVar3;
    i16 local_16;
    i16 iStack20;
    i16 iStack18;
    i16 iStack16;
    u32 uStack14;
     let mut uStack10: u16;
     let mut uStack8: u16;
    i16 local_6;
    i16 local_4;

    iVar1 = param_1;
    uVar2 = (param_1 >> 0x10);
    if(param_2 == 0x1)
    {
        (iVar1 + 0x14) = 0x0;
        return;
    }
    if(param_2 != 0xc)
    {
        return;
    }
    pass1_1008_3e94((param_1 & 0xffff0000 | (iVar1 + 0x18)), CONCAT22(param_3, &local_6), CONCAT22(param_3, &local_4));
    uVar3    = pass1_1010_2b66(*(iVar1 + 0x14));
    uStack8  = (uVar3 >> 0x10);
    uStack10 = uVar3;
    uStack14 = pass1_1008_4772((astruct_76 *)(uVar3 & 0xffff | uStack8 << 0x10));
    uVar2    = (uStack14 >> 0x10);
    local_16 = local_4;
    iStack20 = local_6;
    iStack18 = local_4 + (uStack14 + 0x4);
    iStack16 = local_6 + (uStack14 + 0x8);
    InvalidateRect16(0x1008, 0x0, (BOOL16)&local_16);
    return;
}

pub fn unk_draw_op_1020_0000(param_1: u32, HWND16 param_2, param_3: u16)

{
    code        **ppcVar1;
    u32    uVar2;
    i16           iVar3;
    i16           iVar4;
    i16           iVar5;
     let mut uVar6: u16;
    HWND16        hwnd;
     let mut uVar7: u16;
    u8            local_c4[0x6];
    u8            local_be[0x2];
    i16           local_b4;
    i16           iStack178;
    i16           aiStack176[0x3c];
    i16           iStack56;
    i16           iStack48;
    astruct_76   *paStack46;
    i16           local_2a;
    i16           local_28;
    u32   *puStack38;
    PAINTSTRUCT16 local_22;

    // Segment:    5
    // Offset:     00033420
    // Length:     efba
    // Min Alloc:  efba
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    uVar7 = (iVar5 + 0x4);
    BeginPai1616(param_2, &local_22);
    uVar2     = (iVar5 + 0x14);
    puStack38 = (uVar2 + 0xa);
    pass1_1008_3e94((param_1 & 0xffff0000 | (iVar5 + 0x18U)), CONCAT22(param_3, &local_2a), CONCAT22(param_3, &local_28));
    hwnd = 0x1008;
    pass1_1008_4480(puStack38, (param_1 & 0xffff0000 | (iVar5 + 0x18U)), *(astruct_76 **)(iVar5 + 0x24), param_3);
    paStack46 = (astruct_76 *)0x0;
    for(iStack48 = 0x0; iStack48 < 0x6; iStack48 = iStack48 + 0x1)
    {
        uVar2 = (iVar5 + 0x14);
        hwnd  = 0x1010;
        pass1_1010_2b78(uVar2, (uVar2 >> 0x10), iStack48, CONCAT22(param_3, &local_b4));
        if(local_b4 == 0x0)
        {
            for(iStack56 = 0x0; iVar4 = iStack56, iStack56 <= iStack178; iStack56 = iStack56 + 0x1)
            {
                iVar3 = iStack56 * 0x3;
                if(aiStack176[iStack56 * 0x3 + 0x2] != 0x0)
                {
                    paStack46 = (astruct_76 *)pass1_1010_2b98(*(iVar5 + 0x14), aiStack176[iStack56 * 0x3 + 0x2]);
                    pass1_1008_3e54(CONCAT22(param_3, local_be), 0x0, aiStack176[iVar4 * 0x3 + 0x1] + local_2a, aiStack176[iVar3] + local_28);
                    hwnd = 0x1008;
                    pass1_1008_4480(puStack38, CONCAT22(param_3, local_be), paStack46, param_3);
                }
            }
        }
        else
        {
            _local_be = CONCAT22(param_3, aiStack176 + iStack178 * 0x3);
            if(aiStack176[iStack178 * 0x3 + 0x2] != 0x0)
            {
                paStack46 = (astruct_76 *)pass1_1010_2b98(*(iVar5 + 0x14), aiStack176[iStack178 * 0x3 + 0x2]);
                pass1_1008_3e54(CONCAT22(param_3, local_c4), 0x0, (_local_be + 0x2) + local_2a, *_local_be + local_28);
                hwnd = 0x1008;
                pass1_1008_4480(puStack38, CONCAT22(param_3, local_c4), paStack46, param_3);
            }
        }
    }
    ppcVar1 = (*puStack38 + 0x4);
    (**ppcVar1)(hwnd, puStack38, (puStack38 >> 0x10), 0x0, 0x0, iVar5 + 0xa, uVar6, uVar7);
    EndPaint16(hwnd, &local_22);
    return;
}

pub fn pass1_1020_01d8(Struct20 *param_1, param_2: u16, param_3: u16, param_4: u16, param_5: u16, param_6: u16, param_7: u16, param_8: u32, param_9: u16)

{
    unk_draw_op_1008_61b2((Struct20 *)CONCAT22(param_2, param_1), param_3, param_7, param_8, param_9);
    (param_1 + 0x1)            = 0x0;
    param_1[0x1].field_0x4     = 0x0;
    param_1[0x1].field_0x8     = param_6;
    param_1[0x1].field_0xa     = param_5;
    param_1[0x1].field_0xc     = param_4;
    CONCAT22(param_2, param_1) = 0x45a;
    param_1->field_0x2         = 0x1020;
    return;
}

pub fn draw_op_1020_041e(param_1: u32, param_2: u16)

{
    fill_rect_1020_065e((param_1 + 0xe6), param_2);
    return;
}

pub fn fill_rect_1020_065e(param_1: u32, HWND16 in_win_handle_2)

{
    code        **ppcVar1;
    u32    uVar2;
    i16           iVar3;
     let mut uVar4: u16;
     let mut local_brush_handle: u16;
     let mut uStack50: u16;
    i16           iStack48;
    i16           iStack46;
    RECT16       *local_rect_1;
    HDC16        *pHStack42;
    u32   *puStack40;
    HDC16         local_24;
    PAINTSTRUCT16 local_22;

    uVar4    = (param_1 >> 0x10);
    iVar3    = param_1;
    local_24 = BeginPai1616(in_win_handle_2, &local_22);
    if(0x280 < (iVar3 + 0xa))
    {
        local_rect_1       = CreateSolidBrush16(s_tile2_bmp_1050_1538);
        local_brush_handle = 0x0;
        uStack50           = 0x0;
        iStack48           = (iVar3 + 0xa) + -0x1;
        iStack46           = (iVar3 + 0xc) + -0x1;
        FillRect16(s_tile2_bmp_1050_1538, local_rect_1, &local_brush_handle);
        DeleteObject16(s_tile2_bmp_1050_1538);
    }
    uVar2     = (iVar3 + 0x6);
    puStack40 = (uVar2 + 0xe);
    pHStack42 = &local_24;
    uVar2     = *puStack40;
    ppcVar1   = (uVar2 + 0x8);
    (**ppcVar1)(s_tile2_bmp_1050_1538, puStack40, (puStack40 >> 0x10), pHStack42);
    ppcVar1 = (uVar2 + 0x4);
    (**ppcVar1)(s_tile2_bmp_1050_1538, puStack40, (iVar3 + 0x10), (iVar3 + 0xe), &local_24);
    pHStack42 = (HDC16 *)SelectPalette16(s_tile2_bmp_1050_1538, 0x0, (BOOL16)pHStack42);
    DeleteObject16(s_tile2_bmp_1050_1538);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}

pub fn pass1_1020_07aa(param_1: u32, param_2: u16, param_3: i16, param_4: u16, param_5: u16)

{
    i16 iVar1;
     let mut uVar2: u16;
    u8  local_16[0x14];

    draw_op_1020_041e(param_1, param_4);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if((iVar1 + 0xf0) == 0x0)
    {
        (iVar1 + 0xf0) = 0x1;
        pass1_1008_5bdc((astruct_79 *)CONCAT22(param_5, local_16), param_3, param_5);
        win_1008_5c9e(CONCAT22(param_5, local_16), (param_1 & 0xffff0000 | (iVar1 + 0xf2)), local_16, param_2, param_5);
        pass1_1008_5c34(CONCAT22(param_5, local_16));
    }
    return;
}

pub fn pass1_1018_dfd4(param_1: u32, mut param_2: *mut u8, param_3: i16, param_4: u16, param_5: u16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
    u16 *puVar3;

    uVar2 = (param_1 >> 0x10);
    uVar1 = param_1;
    delete_palette_1018_e16c(*(uVar1 + 0xe2), param_4);
    if((uVar1 + 0xe6) == 0x0)
    {
        (uVar1 + 0xe6) = 0x1;
        puVar3         = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x36, param_5, param_2, param_3);
        pass1_1038_af40(_PTR_LOOP_1050_5b7c, (uVar1 + 0x8), 0x8, (puVar3 >> 0x10), uVar1, &PTR_LOOP_1050_1038, param_5);
    }
    return;
}

pub fn delete_palette_1018_e16c(param_1: u32, HWND16 param_2)

{
    u32   *puVar1;
    code        **ppcVar2;
    u32    uVar3;
    HDC16        *b_force_background;
    HDC16         local_24;
    PAINTSTRUCT16 local_22;

    local_24           = BeginPai1616(param_2, &local_22);
    uVar3              = (param_1 + 0x6);
    puVar1             = (uVar3 + 0xa);
    b_force_background = &local_24;
    uVar3              = *puVar1;
    ppcVar2            = (uVar3 + 0x8);
    (**ppcVar2)(s_tile2_bmp_1050_1538, puVar1, (puVar1 >> 0x10), b_force_background);
    ppcVar2 = (uVar3 + 0x4);
    (**ppcVar2)(s_tile2_bmp_1050_1538, puVar1, 0x0, &local_24);
    SelectPalette16(s_tile2_bmp_1050_1538, 0x0, (BOOL16)b_force_background);
    DeleteObject16(s_tile2_bmp_1050_1538);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}

pub fn pass1_1018_e230(param_1: u16, Struct20 *param_2, param_3: u16, param_4: u16)

{
     let mut extraout_DX: *mut u8;
     let mut uVar1: u16;
    astruct_128 *iVar2;
    i16          unaff_DI;
     let mut uVar2: u16;
    u16         *puVar3;

    unk_draw_op_1020_7f7a(param_2, 0x4, CONCAT22(param_4, param_3));
    uVar2              = (param_2 >> 0x10);
    iVar2              = (astruct_128 *)param_2;
    iVar2->field_0xee  = 0x0;
    &iVar2->field_0xf2 = 0x0;
    param_2->field_0x0 = 0xe44e;
    iVar2->field_0x2   = 0x1018;
    iVar2->field_0xe2  = 0xe4ea;
    iVar2->field_0xe4  = 0x1018;
    puVar3             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x26, param_1, extraout_DX, unaff_DI);
    uVar1              = (puVar3 >> 0x10);
    iVar2->field_0xf2  = puVar3;
    iVar2->field_0xf4  = uVar1;
    iVar2->field_0xe6  = iVar2->field_0xf2;
    iVar2->field_0xe8  = uVar1;
    return;
}

pub fn pass1_1018_e4f2(astruct_659 *param_1, param_2: u16, param_3: u16, param_4: i16, param_5: u16)

{
    code     **ppcVar1;
    u32 uVar2;
    i16        iVar3;
     let mut extraout_DX: *mut u8;
     let mut uVar4: u16;
    u16       *puVar5;

    set_struct_op_1020_921c(CONCAT22(param_2, param_1), param_3);
    &param_1->field_0x14       = 0x0;
    CONCAT22(param_2, param_1) = 0xe5d0;
    param_1->field_0x2         = 0x1018;
    puVar5                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x26, param_5, extraout_DX, param_4);
    uVar4                      = (puVar5 >> 0x10);
    param_1->field_0x14        = puVar5;
    param_1->field_0x16        = uVar4;
    param_1->field_0x6         = param_1->field_0x14;
    param_1->field_0x8         = uVar4;
    uVar2                      = &param_1->field_0x14;
    iVar3                      = &param_1->field_0xa;
    ppcVar1                    = ((uVar2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_1->field_0x12 = iVar3;
    draw_op_1020_9364(CONCAT22(param_2, param_1), 0x1020, param_5);
    return;
}


pub fn pass1_1018_e57a(mut param_1: *mut u16)

{
    astruct_575 *iVar1;
     let mut uVar1: u16;

    uVar1            = (param_1 >> 0x10);
    iVar1            = (astruct_575 *)param_1;
    *param_1         = 0xe5d0;
    iVar1->field_0x2 = 0x1018;
    if(iVar1->field_0x14 != 0x0)
    {
        pass1_1010_1dda(iVar1->field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

pub fn unk_draw_op_1018_c578(astruct_36 *param_1, param_2: u16)

{
     let mut uVar1: u16;
    i16           iVar2;
    i16           iVar3;
    astruct_76   *paVar4;
    code        **ppcVar5;
    u32    uVar6;
     let mut uVar7: u16;
    HDC16        *b_force_background;
    i16           iVar8;
     let mut in_DX: *mut u8;
     let mut uVar9: u16;
     let mut uVar10: u16;
     let mut extraout_DX: u16;
    i16           iVar11;
     let mut uVar12: u16;
    i16           unaff_DI;
     let mut uVar13: u16;
     let mut uVar14: u16;
    HWND16        hwnd;
    u32           uVar15;
    HDC16        *pHVar16;
    RECT16       *pRVar18;
    u32    uVar17;
    HDC16         HVar19;
    u32    local_34;
    i16           iStack48;
    i16           iStack46;
    RECT16       *pRStack44;
    HDC16         local_2a;
     let mut uStack40: u16;
    u16          *puStack38;
    PAINTSTRUCT16 local_22;

    hwnd      = 0x1010;
    puStack38 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    uVar9     = (puStack38 >> 0x10);
    uVar7     = (puStack38 + 0x20);
    iVar11    = param_1;
    uVar13    = (param_1 >> 0x10);
    uStack40  = uVar7;
    if(uVar7 == 0x0)
    {
        BeginPai1616(0x1010, &local_22);
        EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
        PostMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, CONCAT22(0x111, (iVar11 + 0xea)));
        return;
    }
    if(((iVar11 + 0xf0) == 0x0) && ((iVar11 + 0xf4) != 0x0))
    {
        (iVar11 + 0xf0) = 0x1;
        uVar1           = iVar11 + 0xf2;
        hwnd            = 0x1008;
        win_1008_5c9e(_PTR_LOOP_1050_02a0, (param_1 & 0xffff0000 | uVar1), uVar1, uVar9, param_2);
        uVar7 = uVar1;
    }
    if((_PTR_LOOP_1050_02a0 + 0x12) == 0x0)
    {
        hwnd = 0x1008;
        win_1008_5c5c(param_2, uVar7, uVar9, globals._PTR_LOOP_1050_02a0, 0x1d3);
    }
    local_2a  = BeginPai1616(hwnd, &local_22);
    pRStack44 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
    local_34  = 0x0;
    iStack48  = (iVar11 + 0xf6) + -0x1;
    iStack46  = (iVar11 + 0xf8) + -0x1;
    uVar7     = param_2;
    HVar19    = local_2a;
    FillRect16(s_tile2_bmp_1050_1538, pRStack44, &local_34);
    pRVar18 = pRStack44;
    DeleteObject16(s_tile2_bmp_1050_1538);
    uVar6              = (iVar11 + 0xe2);
    paVar4             = *(astruct_76 **)(uVar6 + 0xe);
    b_force_background = &local_2a;
    uVar17             = CONCAT22(pRVar18, param_2);
    uVar14             = (paVar4 >> 0x10);
    uVar12             = SUB42(paVar4, 0x0);
    uVar6              = paVar4;
    ppcVar5            = (uVar6 + 0x8);
    pHVar16            = b_force_background;
    (**ppcVar5)();
    uVar15           = pass1_1008_4772(paVar4);
    uVar10           = (uVar15 >> 0x10);
    iVar2            = (uVar15 + 0x4);
    iVar3            = (uVar15 + 0x8);
    iVar8            = (0x1e0 - iVar3) / 0x2;
    (iVar11 + 0x10c) = iVar8 + iVar3 + (iVar11 + 0x110);
    ppcVar5          = (uVar6 + 0x4);
    (**ppcVar5)(0x1008, paVar4, (iVar11 + 0xfc) + (iVar11 + 0xfe) + iVar8, (iVar11 + 0xfa) + (0x280 - iVar2) / 0x2, &local_2a, param_2, uVar12, uVar14, pHVar16, uVar17, uVar7, HVar19);
    draw_text_1018_c742(param_1, 0x1008, param_2, extraout_DX);
    SelectPalette16(0x1008, 0x0, (BOOL16)b_force_background);
    DeleteObject16(s_tile2_bmp_1050_1538);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    return;
}


pub fn draw_text_1018_c742(astruct_36 *param_1, param_2: HDC16, RECT16 *param_3, param_4: u16)

{
     let mut piVar1: *mut i16;
    COLORREF    CVar2;
    i16         iVar3;
    astruct_36 *iVar4;
    astruct_36 *uVar4;
     let mut local_1a: u16;
     let mut uStack24: u16;
    i16         iStack22;
    i16         iStack20;
    i16         local_12;
    i16         iStack16;
    i16         iStack14;
    i16         iStack12;
    COLORREF    CStack10;
     let mut uStack8: u16;
    u32  uStack6;

    uVar4 = (astruct_36 *)(param_1 >> 0x10);
    iVar4 = (astruct_36 *)param_1;
    if((iVar4->field_0x108 != 0x0) && (*iVar4->field_0x108 != '\0'))
    {
        CVar2    = SetTextColor16(param_2, 0x8000);
        uStack6  = CONCAT22(param_4, CVar2);
        CStack10 = SetBkColor16(s_tile2_bmp_1050_1538, 0x0);
        uStack8  = param_4;
        if(iVar4->field_0x106 == 0x0)
        {
            local_1a = 0x0;
            uStack24 = 0x0;
            iStack22 = iVar4->field_0x10e;
            iStack20 = 0x0;
            DrawText16(s_tile2_bmp_1050_1538, 0x410, &local_1a, param_3, 0xffff);
            iVar4->field_0x100 = (0x280 - iStack22) / 0x2;
            iVar4->field_0x102 = iVar4->field_0x10c;
            iVar4->field_0x104 = iVar4->field_0x100 + iStack22;
            iVar3              = iVar4->field_0x102 + iStack20;
            iVar4->field_0x106 = iVar3;
            piVar1             = &iVar4->field_0xf8;
            if(*piVar1 == iVar3 || *piVar1 < iVar3)
            {
                iVar3   = iVar3 - iVar4->field_0xf8;
                piVar1  = &iVar4->field_0x102;
                *piVar1 = *piVar1 - iVar3;
                piVar1  = &iVar4->field_0x106;
                *piVar1 = *piVar1 - iVar3;
            }
        }
        local_12 = iVar4->field_0xfa + iVar4->field_0x100;
        iStack16 = iVar4->field_0xfc + iVar4->field_0x102;
        iStack14 = iVar4->field_0xfa + iVar4->field_0x104;
        iStack12 = iVar4->field_0xfc + iVar4->field_0x106;
        DrawText16(s_tile2_bmp_1050_1538, &PTR_LOOP_1050_0010, &local_12, param_3, 0xffff);
        SetTextColor16(s_tile2_bmp_1050_1538, uStack6);
        SetBkColor16(s_tile2_bmp_1050_1538, CStack10);
    }
    return;
}

pub fn pass1_1018_5b06(astruct_132 *param_1, param_2: u16, param_3: u16, param_4: u16)

{
    u32  *puVar1;
    code       **ppcVar2;
    u32         *puVar3;
     let mut uVar5: u16;
    u32   uVar6;
     let mut puVar7: *mut u8;
     let mut uVar8: u16;
     let mut puVar9: *mut u8;
     let mut uVar10: u16;
    i16          unaff_DI;
    u16         *puVar11;
    astruct_76  *paVar12;
    u32          uVar13;
     let mut uVar14: u16;
     let mut uVar15: u16;
    astruct_132 *paVar16;
     let mut uVar17: u16;
    u8           local_c[0x6];
    u16         *puStack6;
     let mut uVar4: u16;

    set_struct_op_1020_921c(CONCAT22(param_2, param_1), param_3);
    param_1->field_0x14        = 0x0;
    param_1->field_0x18        = 0x0;
    puVar11                    = pass1_1008_3e38(CONCAT22(param_2, &param_1->field_0x1c_addr_base));
    CONCAT22(param_2, param_1) = &PTR_LOOP_1050_5e1a;
    param_1->field_0x2         = 0x1018;
    puStack6                   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, (puVar11 >> 0x10), unaff_DI);
    puVar11                    = pass1_1008_3e38(CONCAT22(param_4, local_c));
    puVar7                     = (puVar11 >> 0x10);
    pass1_1008_3f62(CONCAT22(param_4, local_c), (puStack6 & 0xffff0000 | (puStack6 + 0xe)));
    puVar11                      = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x27, param_4, puVar7, unaff_DI);
    uVar8                        = (puVar11 >> 0x10);
    &param_1->field_0x14         = puVar11;
    (&param_1->field_0x14 + 0x2) = uVar8;
    uVar15                       = 0x0;
    uVar14                       = &param_1->field_0x14;
    ppcVar2                      = (*param_1->field_0x14 + 0x4);
    paVar16                      = param_1;
    uVar17                       = param_2;
    (**ppcVar2)();
    param_1->field_0x6 = param_1->field_0x14;
    puVar3             = param_1->field_0x14;
    puVar1             = *(puVar3 + 0xa);
    uVar6              = CONCAT22(param_2, &param_1->field_0xa);
    ppcVar2            = (*puVar1 + 0x8);
    (**ppcVar2)(0x1010, puVar1, (puVar1 >> 0x10), uVar6, uVar14, uVar8, uVar15, paVar16, uVar17);
    param_1->field_0x12 = uVar6;
    draw_op_1020_9364(CONCAT22(param_2, param_1), 0x1020, param_4);
    puVar3 = param_1->field_0x14;
    pass1_1008_3f62(CONCAT22(param_2, &param_1->field_0x1c_addr_base), (puVar3 & 0xffff0000 | (puVar3 + 0x52)));
    pass1_1008_3f32(CONCAT22(param_2, &param_1->field_0x1c_addr_base), CONCAT22(param_4, local_c));
    paVar12 = (astruct_76 *)pass1_1008_9f48(param_1->field_0x14);
    uVar13  = pass1_1008_4772(paVar12);
    puVar9  = (uVar13 >> 0x10);
    uVar4   = uVar13;
    puVar7  = puVar9;
    uVar5   = uVar4;
    mem_op_1000_179c(0x14, puVar9, 0x1000);
    uVar10 = puVar7 | uVar5;
    if(uVar10 == 0x0)
    {
        param_1->field_0x18 = 0x0;
    }
    else
    {
        pass1_1008_50c2((astruct_110 *)CONCAT22(puVar7, uVar5), *(uVar4 + 0x8), *(uVar4 + 0x4), CONCAT22(param_2, &param_1->field_0x1c_addr_base), puVar1);
        *&param_1->field_0x18         = uVar5;
        *(&param_1->field_0x18 + 0x2) = uVar10;
    }
    pass1_1008_5134(param_1->field_0x18);
    param_1->field_0x22 = param_1->field_0x1c_addr_base;
    param_1->field_0x24 = param_1->field_0x1e;
    param_1->field_0x26 = (uVar4 + 0x4) + param_1->field_0x22 + 0x1;
    param_1->field_0x28 = (uVar4 + 0x8) + param_1->field_0x24 + 0x1;
    return;
}

pub fn pass1_1018_5cc8(mut param_1: *mut u16, param_2: u16)

{
     let mut uVar1: u16;
    Struct18  *paVar2;
    astruct_508 *iVar3;
     let mut uVar3: u16;

    uVar3            = (param_1 >> 0x10);
    iVar3            = (astruct_508 *)param_1;
    *param_1         = &PTR_LOOP_1050_5e1a;
    iVar3->field_0x2 = 0x1018;
    paVar2           = &iVar3->field_0x18;
    uVar1            = iVar3->field_0x1a_addr_offset;
    if((uVar1 | paVar2) != 0x0)
    {
        pass1_1008_5118(paVar2 & 0xffff | uVar1 << 0x10);
        fn_ptr_1000_17ce(paVar2, 0x1000);
    }
    if(iVar3->field_0x14 != 0x0)
    {
        pass1_1010_1ea6(iVar3->field_0x14, param_1 & 0xffff | uVar3 << 0x10, param_2);
        pass1_1010_1dda(iVar3->field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}


pub fn invalidate_rect_1018_5d32(param_1: u32, param_2: i16, HWND16 param_3)

{
    if(param_2 == 0x1)
    {
        (param_1 + 0x14) = 0x0;
        return;
    }
    if(param_2 != 0x2)
    {
        return;
    }
    InvalidateRect16(param_3, 0x0, param_1 + 0x22);
    return;
}


pub fn misc_draw_op_1018_5d6c(param_1: u32, HWND16 param_2)

{
    u32   *puVar1;
    code        **ppcVar2;
    u32    uVar3;
    i16           iVar4;
     let mut uVar5: u16;
     let mut unaff_SS: u16;
    astruct_76   *paVar6;
     let mut uVar7: u16;
    PAINTSTRUCT16 local_22;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar7 = (iVar4 + 0x4);
    BeginPai1616(param_2, &local_22);
    uVar3  = (iVar4 + 0x14);
    puVar1 = *(uVar3 + 0xa);
    paVar6 = (astruct_76 *)pass1_1008_9f48(*(iVar4 + 0x14));
    pass1_1008_5236(*(iVar4 + 0x18));
    pass1_1008_4480(puVar1, (param_1 & 0xffff0000 | (iVar4 + 0x1c)), paVar6, unaff_SS);
    ppcVar2 = (*puVar1 + 0x4);
    (**ppcVar2)(0x1008, puVar1, (puVar1 >> 0x10), 0x0, param_1 & 0xffff0000 | (iVar4 + 0xa), uVar7);
    EndPaint16(0x1008, &local_22);
    return;
}

pub fn set_window_text_1018_6066(u1616 param_1, u1616 param_2, SEGPTR in_win_text_3, u1616 param_4, u16 dialog_id_5, HWND16 in_hwnd_6)

{
    GetDlgItem16(in_hwnd_6, dialog_id_5);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, in_win_text_3);
    return;
}


pub fn set_window_text_1018_6086(param_1: u32, LPSTR param_2, WORD *param_3)

{
    wspri16f16(param_2, &stack0xfff4, param_3);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1be);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, (SEGPTR)&stack0xfff4);
    wspri16f16(s_tile2_bmp_1050_1538, &stack0xfff4, param_3);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1bf);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, (SEGPTR)&stack0xfff4);
    return;
}

pub fn unk_draw_op_1018_623e(param_1: u32, HWND16 param_2, param_3: u16)

{
     let mut piVar1: *mut i16;
    code        **ppcVar2;
    u32    uVar3;
    u32   *puVar4;
     let mut uVar5: u16;
    HDC16        *pHVar6;
    i16           iVar7;
    HPEN16        handle;
    HGDIOBJ16     HVar8;
    HBRUSH16      handle_00;
     let mut puVar9: *mut u8;
     let mut uVar10: u16;
    i16           iVar11;
    i16           iVar12;
     let mut puVar13: *mut u8;
     let mut uVar14: u16;
     let mut uVar15: u16;
    u32           uVar16;
    i16           iVar17;
     let mut uVar18: u8;
     let mut uVar19: u8;
    u8            local_38[0x6];
     let mut local_32: u16;
     let mut uStack48: u16;
    u32    uStack46;
     let mut uStack42: u16;
    u32   *puStack40;
    HDC16         local_24;
    PAINTSTRUCT16 local_22;

    puVar13   = &stack0xfffe;
    uVar14    = (param_1 >> 0x10);
    iVar11    = param_1;
    uVar15    = (iVar11 + 0x4);
    local_24  = BeginPai1616(param_2, &local_22);
    puStack40 = pass1_1010_4c2c(*(iVar11 + 0x6));
    pHVar6    = &local_24;
    ppcVar2   = (*puStack40 + 0x8);
    (**ppcVar2)(0x1010, puStack40, (puStack40 >> 0x10), pHVar6, param_3, uVar15);
    *(HDC16 **)(iVar11 + 0x10) = pHVar6;
    uVar3                      = (iVar11 + 0x6);
    uStack42                   = (uVar3 + 0x30);
    uVar3                      = (iVar11 + 0x6);
    uStack46                   = (uVar3 + 0x12);
    uStack48                   = 0x14;
    local_32                   = 0x0;
    pass1_1008_3e38(CONCAT22(param_3, local_38));
    while((puVar13 + -0x38) < (puVar13 + -0x28))
    {
        iVar11            = (puVar13 + -0x38) * 0x4;
        uVar3             = (puVar13 + -0x2c);
        uVar16            = pass1_1008_4772(*(astruct_76 **)(iVar11 + uVar3));
        puVar9            = (uVar16 >> 0x10);
        (puVar13 + -0x44) = uVar16;
        (puVar13 + -0x42) = puVar9;
        uVar3             = (puVar13 + 0x6);
        pass1_1018_642e(uVar3, (uVar3 >> 0x10), CONCAT13((param_3 >> 0x8), CONCAT12(param_3, puVar13 + -0x30)), (uVar16 + 0x8));
        uVar3 = (puVar13 + -0x30);
        pass1_1008_3e76(CONCAT22(param_3, puVar13 + -0x36), 0x0, uVar3, (uVar3 >> 0x10));
        uVar3 = (puVar13 + -0x2c);
        pass1_1008_4480(*(puVar13 + -0x26), CONCAT22(param_3, puVar13 + -0x36), *(astruct_76 **)(uVar3 + iVar11), param_3);
        iVar11 = (puVar13 + -0x38);
        uVar3  = (puVar13 + -0x30);
        uVar14 = uVar3;
        uVar18 = (undefined)(uVar3 >> 0x10);
        uVar19 = (undefined)(uVar3 >> 0x18);
        uVar3  = (puVar13 + -0x44);
        uVar15 = (uVar3 >> 0x10);
        iVar12 = uVar3;
        iVar7  = (iVar12 + 0x4) + (puVar13 + -0x2e);
        iVar12 = (iVar12 + 0x8) + (puVar13 + -0x30);
        uVar3  = (puVar13 + 0x6);
        uVar3  = (uVar3 + 0x6);
        iVar17 = uVar3;
        uVar15 = (uVar3 >> 0x10);
        if((iVar17 + 0x1a) == 0x0)
        {
            uVar5 = (iVar17 + 0x30) << 0x3;
            mem_op_1000_179c(uVar5, puVar9, 0x1000);
            (iVar17 + 0x1a) = uVar5;
            (iVar17 + 0x1c) = puVar9;
        }
        uVar3                  = (iVar17 + 0x1a);
        iVar11                 = iVar11 * 0x8;
        (uVar3 + iVar11)       = CONCAT11(uVar19, uVar18);
        uVar3                  = (iVar17 + 0x1a);
        (uVar3 + iVar11 + 0x2) = uVar14;
        uVar3                  = (iVar17 + 0x1a);
        (uVar3 + iVar11 + 0x4) = iVar7;
        uVar3                  = (iVar17 + 0x1a);
        (uVar3 + iVar11 + 0x6) = iVar12;
        uVar3                  = (puVar13 + -0x44);
        piVar1                 = (puVar13 + -0x2e);
        *piVar1                = *piVar1 + (-((puVar13 + -0x38) == 0x0) & 0x5) + 0x14 + (uVar3 + 0x4);
        piVar1                 = (puVar13 + -0x38);
        *piVar1                = *piVar1 + 0x1;
    }
    puVar4  = (puVar13 + -0x26);
    ppcVar2 = (*puVar4 + 0x4);
    (**ppcVar2)(0x1008, puVar4, (puVar4 >> 0x10), 0x0, 0x0, puVar13 + -0x22, param_3);
    handle                          = CreatePen16(0x1008, 0x25, 0x100);
    *(HPEN16 *)(puVar13 + -0x3a)    = handle;
    HVar8                           = SelectObject16(s_tile2_bmp_1050_1538, handle);
    *(HGDIOBJ16 *)(puVar13 + -0x3c) = HVar8;
    handle_00                       = CreateSolidBrush16(s_tile2_bmp_1050_1538);
    *(HBRUSH16 *)(puVar13 + -0x3e)  = handle_00;
    HVar8                           = SelectObject16(s_tile2_bmp_1050_1538, handle_00);
    *(HGDIOBJ16 *)(puVar13 + -0x40) = HVar8;
    draw_line_1018_6444(*(puVar13 + 0x6), s_tile2_bmp_1050_1538);
    uVar3  = (puVar13 + 0x6);
    uVar16 = pass1_1010_4dc8(*(uVar3 + 0x6));
    uVar10 = (uVar16 >> 0x10);
    uVar5  = uVar16;
    draw_op_1018_6544(*(puVar13 + 0x6), (uVar16 & 0xffff | uVar10 << 0x10), param_3);
    pass1_1018_6630(*(puVar13 + 0x6), uVar5, uVar10);
    uVar3 = (puVar13 + 0x6);
    SelectPalette16(0x1010, 0x0, (uVar3 + 0x10));
    DeleteObject16(s_tile2_bmp_1050_1538);
    SelectObject16(s_tile2_bmp_1050_1538, *(HGDIOBJ16 *)(puVar13 + -0x3c));
    DeleteObject16(s_tile2_bmp_1050_1538);
    SelectObject16(s_tile2_bmp_1050_1538, *(HGDIOBJ16 *)(puVar13 + -0x40));
    DeleteObject16(s_tile2_bmp_1050_1538);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, (PAINTSTRUCT16 *)(puVar13 + -0x20));
    return;
}

pub fn draw_line_1018_6444(param_1: u32, param_2: HDC16)

{
    i16        iVar1;
    u16     *pIVar2;
    u32 uVar3;
    i16        iVar4;
    i16        iVar5;
     let mut piVar6: *mut i16;
     let mut uVar7: u16;
    i16        iStack10;

    uVar7  = (param_1 >> 0x10);
    uVar3  = (param_1 + 0x6);
    iVar1  = (uVar3 + 0x30);
    uVar3  = (param_1 + 0x6);
    pIVar2 = (uVar3 + 0x1a);
    MoveTo16(param_2, 0x5, *pIVar2);
    uVar7 = (pIVar2 >> 0x10);
    iVar5 = pIVar2;
    LineTo16(s_tile2_bmp_1050_1538, 0x5, (iVar5 + iVar1 * 0x8 + -0x4));
    for(iStack10 = 0x0; iStack10 < iVar1; iStack10 = iStack10 + 0x1)
    {
        piVar6 = (iStack10 * 0x8 + iVar5);
        iVar4  = (piVar6[0x2] - *piVar6 >> 0x1) + *piVar6;
        MoveTo16(s_tile2_bmp_1050_1538, 0x5, iVar4);
        LineTo16(s_tile2_bmp_1050_1538, 0xa, iVar4);
    }
    MoveTo16(s_tile2_bmp_1050_1538, 0x5f, *pIVar2);
    LineTo16(s_tile2_bmp_1050_1538, 0x5f, (iVar5 + iVar1 * 0x8 + -0x4));
    for(iStack10 = 0x0; iStack10 < iVar1; iStack10 = iStack10 + 0x1)
    {
        piVar6 = (iStack10 * 0x8 + iVar5);
        iVar4  = (piVar6[0x2] - *piVar6 >> 0x1) + *piVar6;
        MoveTo16(s_tile2_bmp_1050_1538, 0x5f, iVar4);
        LineTo16(s_tile2_bmp_1050_1538, 0x5a, iVar4);
    }
    return;
}


pub fn draw_op_1018_6544(param_1: u32, i16 *param_2, param_3: u16)

{
    u16 *puVar1;
    u32  uVar2;
     let mut uVar3: u16;
    u8   local_a[0x6];
     let mut uStack4: u16;

    if(param_2 != 0x0)
    {
        uStack4 = ((param_2 + 0x4) - *param_2 >> 0x1) + *param_2;
        puVar1  = pass1_1008_3e54(CONCAT22(param_3, local_a), 0x0, 0x57, uStack4);
        uVar3   = (param_1 >> 0x10);
        uVar2   = pass1_1018_659a(param_1, uVar3, CONCAT22(param_3, local_a), (puVar1 >> 0x10), param_3);
        draw_polygon_1018_661c(param_1, uVar3, CONCAT22(uVar2, 0x3), 0x1008);
    }
    return;
}

pub fn draw_polygon_1018_661c(param_1: u16, param_2: u16, param_3: u32, param_4: HDC16)

{
    Polygon16(param_4, (POINT16 *)param_3, (param_3 >> 0x10));
    return;
}

pub fn struct_1018_66cc(Struct20 *param_1, param_2: u16, param_3: u16, param_4: u16)

{
     let mut extraout_DX: *mut u8;
     let mut uVar1: u16;
    Struct20 *iVar2;
    i16         unaff_DI;
    Struct20 *uVar2;
    u16        *puVar2;

    unk_draw_op_1020_7f7a(param_1, 0xa, CONCAT22(param_3, param_2));
    uVar2                                    = (Struct20 *)(param_1 >> 0x10);
    iVar2                                    = (Struct20 *)param_1;
    &iVar2[0x1].field_0xc                    = 0x0;
    iVar2[0x1].field_0x10                    = 0x0;
    param_1->field_0x0                       = 0x6880;
    iVar2->field_0x2                         = 0x1018;
    ((Struct20 *)(iVar2 + 0x1))->field_0x0 = 0x691c;
    iVar2[0x1].field_0x2                     = 0x1018;
    puVar2                                   = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0xb, param_4, extraout_DX, unaff_DI);
    uVar1                                    = (puVar2 >> 0x10);
    &iVar2[0x1].field_0x10                   = puVar2;
    (&iVar2[0x1].field_0x10 + 0x2)           = uVar1;
    &iVar2[0x1].field_0x4                    = &iVar2[0x1].field_0x10;
    (&iVar2[0x1].field_0x4 + 0x2)            = uVar1;
    return;
}

pub fn pass1_1018_6924(astruct_658 *param_1, param_2: u16, param_3: u16, param_4: i16, param_5: u16)

{
    code     **ppcVar1;
    u32 uVar2;
    i16        iVar3;
     let mut extraout_DX: *mut u8;
     let mut uVar4: u16;
    u16       *puVar5;

    set_struct_op_1020_921c(CONCAT22(param_2, param_1), param_3);
    &param_1->field_0x14       = 0x0;
    CONCAT22(param_2, param_1) = 0x6a02;
    param_1->field_0x2         = 0x1018;
    puVar5                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0xb, param_5, extraout_DX, param_4);
    uVar4                      = (puVar5 >> 0x10);
    param_1->field_0x14        = puVar5;
    param_1->field_0x16        = uVar4;
    param_1->field_0x6         = param_1->field_0x14;
    param_1->field_0x8         = uVar4;
    uVar2                      = &param_1->field_0x14;
    iVar3                      = &param_1->field_0xa;
    ppcVar1                    = ((uVar2 + 0xa) + 0x8);
    (**ppcVar1)();
    param_1->field_0x12 = iVar3;
    draw_op_1020_9364(CONCAT22(param_2, param_1), 0x1020, param_5);
    return;
}


pub fn pass1_1018_69ac(mut param_1: *mut u16)

{
    astruct_511 *iVar1;
     let mut uVar1: u16;

    uVar1            = (param_1 >> 0x10);
    iVar1            = (astruct_511 *)param_1;
    *param_1         = 0x6a02;
    iVar1->field_0x2 = 0x1018;
    if(iVar1->field_0x14 != 0x0)
    {
        pass1_1010_1dda(iVar1->field_0x14);
    }
    palette_op_1020_92c4(param_1, 0x1020);
    return;
}

pub fn mixed_draw_op_1018_6a7a(astruct_28 *param_1, param_2: u16)

{
     let mut in_DX: *mut u8;
    i16           unaff_DI;
     let mut unaff_SS: u16;
    u16          *puVar1;
    PAINTSTRUCT16 local_22;

    BeginPai1616(param_2, &local_22);
    EndPaint16((HWND16)s_tile2_bmp_1050_1538, &local_22);
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, unaff_SS, in_DX, unaff_DI);
    if((puVar1 + 0x20) == 0x0)
    {
        unk_destroy_window_op_1018_6bb6(param_1, 0x1010);
        return;
    }
    mix_ui_op_1018_6adc(param_1);
    return;
}

pub fn clenaup_win_ui_1018_4d22(astruct_11 *in_struct_1, HDC16 in_hdc_2)

{
     let mut uVar1: u16;
    code      **ppcVar2;
    astruct_11 *local_struct_1;
    astruct_11 *uVar4;
     let mut unaff_SS: u16;
    u32        *puVar2;
    u32        *puVar1;

    uVar4                     = (astruct_11 *)(in_struct_1 >> 0x10);
    local_struct_1            = (astruct_11 *)in_struct_1;
    in_struct_1               = s_SCi16ernalPutBldg_site_0x_08lx__b_1050_5046 + 0x12;
    local_struct_1->field_0x2 = 0x1018;
    if(local_struct_1->field_0x12 != 0x0)
    {
        SelectPalette16(in_hdc_2, 0x0, local_struct_1->field_0x1a_addr_offset);
        DeleteObject16(s_tile2_bmp_1050_1538);
        in_hdc_2 = s_tile2_bmp_1050_1538;
        DeleteDC16(s_tile2_bmp_1050_1538);
    }
    puVar1 = local_struct_1->field_0xa;
    uVar1  = local_struct_1->field_0xc;
    if((uVar1 | puVar1) != 0x0)
    {
        ppcVar2 = *puVar1;
        (**ppcVar2)(in_hdc_2, puVar1, uVar1, 0x1);
    }
    puVar2 = local_struct_1->field_0xe;
    uVar1  = local_struct_1->field_0x10;
    if((uVar1 | puVar2) != 0x0)
    {
        ppcVar2 = *puVar2;
        (**ppcVar2)(in_hdc_2, puVar2, uVar1, 0x1);
    }
    globals._PTR_LOOP_1050_4230 = 0x0;
    pass1_1010_1d80(in_struct_1, unaff_SS);
    return;
}


pub fn get_dc_1018_4db0(param_1: u32, param_2: u16, HWND16 param_3)

{
    HDC16 HVar1;
     let mut uVar2: u16;

    uVar2                      = (param_1 >> 0x10);
    (param_1 + 0x16)           = param_2;
    HVar1                      = GetDC16(param_3);
    *(HDC16 *)(param_1 + 0x14) = HVar1;
    return;
}

pub fn create_dc_1018_4e04(astruct_8 **param_1, u1616 param_2, param_3: i16, param_4: i16, LPCSTR in_string_5, u16 in_string_6)

{
    code     **ppcVar1;
    astruct_8 *paVar2;
    astruct_9 *iVar4;
     let mut uVar3: u16;
    u32        uVar4;
    i16        iStack16;

    uVar3   = (param_1 >> 0x10);
    iVar4   = (astruct_9 *)param_1;
    ppcVar1 = (*param_1 + 0x14);
    (**ppcVar1)();
    uVar4 = pass1_1008_4772((astruct_76 *)iVar4->field_0xa);
    pass1_1008_43cc(iVar4->field_0xa);
    paVar2            = (astruct_8 *)CreateDC16(0x1008, uVar4, (uVar4 >> 0x10), (DEVMODEA *)0x0);
    iVar4->field_0x12 = paVar2;
    paVar2            = (astruct_8 *)&iVar4->field_0x12;
    ppcVar1           = (*iVar4->field_0xa + 0x8);
    (**ppcVar1)();
    iVar4->field_0x1a_addr_offset = paVar2;
    if((DAT_1050_422e != 0x0) && (0x280 < param_4))
    {
        for(iStack16 = 0x0; iStack16 < DAT_1050_4216 + 0x1; iStack16 = iStack16 + 0x1)
        {
            (&PTR_DAT_1050_0009_1050_4172 + iStack16 * 0x2) = (((long)(&PTR_DAT_1050_0009_1050_4172 + iStack16 * 0x2) * ((long)param_4 + 0x1)) / 0x280);
        }
        for(iStack16 = 0x0; iStack16 < DAT_1050_422c + 0x1; iStack16 = iStack16 + 0x1)
        {
            (&DAT_1050_419a + iStack16 * 0x2) = (((long)(&DAT_1050_419a + iStack16 * 0x2) * ((long)param_3 + 0x1)) / 0x1e0);
        }
    }
    DAT_1050_422e = 0x0;
    return;
}

pub fn struct_1018_5840(Struct20 *param_1, param_2: u16, param_3: u16, param_4: u16)

{
     let mut extraout_DX: *mut u8;
     let mut uVar1: u16;
    astruct_130 *iVar2;
    i16          unaff_DI;
     let mut uVar2: u16;
    u16         *puVar3;

    unk_draw_op_1020_7f7a(param_1, 0x6, CONCAT22(param_3, param_2));
    uVar2              = (param_1 >> 0x10);
    iVar2              = (astruct_130 *)param_1;
    iVar2->field_0xee  = 0x0;
    &iVar2->field_0xf2 = 0x0;
    iVar2->field_0xf6  = 0x0;
    param_1->field_0x0 = s_Alloc__s_1050_5a5b + 0x7;
    iVar2->field_0x2   = 0x1018;
    iVar2->field_0xe2  = 0x5afe;
    iVar2->field_0xe4  = 0x1018;
    puVar3             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x27, param_4, extraout_DX, unaff_DI);
    uVar1              = (puVar3 >> 0x10);
    iVar2->field_0xf2  = puVar3;
    iVar2->field_0xf4  = uVar1;
    iVar2->field_0xe6  = iVar2->field_0xf2;
    iVar2->field_0xe8  = uVar1;
    return;
}

pub fn invalidate_rect_1018_58e2(astruct_58 *param_1, param_2: i16, HWND16 param_3)

{
     let mut piVar1: *mut i16;
    astruct_58 *iVar2;
     let mut uVar2: u16;

    if(param_2 == 0x105)
    {
        uVar2   = (param_1 >> 0x10);
        iVar2   = (astruct_58 *)param_1;
        piVar1  = &iVar2->field_0xf6;
        *piVar1 = *piVar1 + 0x1;
        if(PTR_DAT_1050_0004_1050_4240 <= iVar2->field_0xf6)
        {
            PostMessage16(param_3, 0x0, 0x0, 0x11100ca);
            return;
        }
        iVar2->field_0xea = 0x0;
        InvalidateRect16(param_3, 0x0, 0x0);
    }
    return;
}

pub fn pass1_1010_4674(param_1: u32, param_2: i16, param_3: u16, u1616 param_4)

{
     let mut piVar1: *mut i16;
    u1632 UVar2;
    u1616 UVar3;

    UVar2 = (u1632)param_1;
    UVar3 = (u1616)(param_1 >> 0x10);
    if(param_2 == 0x1)
    {
        piVar1  = (UVar2 + 0x24);
        *piVar1 = *piVar1 + 0x1;
        if(0xf < (UVar2 + 0x24))
        {
            (UVar2 + 0x24) = 0x0;
        }
    LAB_1010_469a:
        draw_op_1010_47d0(UVar2, UVar3, *(UVar2 + 0x24), param_3, param_4);
    }
    else
    {
        if(param_2 != 0x2)
        {
            if(param_2 != 0x3)
            {
                if(((UVar2 + 0x6a) != 0x0) && ((UVar2 + 0x6a) != 0x4))
                {
                    pass1_1010_459e(param_1);
                }
                goto LAB_1010_46e8;
            }
            piVar1  = (UVar2 + 0x24);
            *piVar1 = *piVar1 + -0x1;
            if(*piVar1 < 0x0)
            {
                (UVar2 + 0x24) = 0xf;
            }
            goto LAB_1010_469a;
        }
    }
    pass1_1010_1f62(param_4, param_1, 0x2);
    pass1_1010_45d6(param_1, param_3);
LAB_1010_46e8:
    (UVar2 + 0x6a) = param_2;
    return;
}

pub fn draw_1010_47ae(param_1: u32, param_2: u16, u1616 param_3)

{
    u1616 UStack4;

    UStack4 = 0x0;
    do
    {
        draw_op_1010_47d0((u1632)param_1, (u1616)(param_1 >> 0x10), UStack4, param_2, param_3);
        UStack4 = UStack4 + 0x1;
    } while(UStack4 < 0x10);
    return;
}

pub fn draw_op_1010_47d0(u1632 param_1, u1616 param_2, u1616 param_3, u16 in_style_3, u1616 param_5)

{
     let mut piVar1: *mut i16;
    u32 *puVar2;
    code      **ppcVar3;
    i16         iVar4;
    HPALETTE16  b_force_background;
    HGDIOBJ16   handle;
    HGDIOBJ16   handle_00;
     let mut uVar5: u16;
     let mut extraout_DX: *mut u8;
     let mut puVar6: *mut u8;
    LPCSTR      output;
    astruct_5  *iVar6;
    i16         iVar7;
    astruct_4  *iVar9;
     let mut uVar8: u16;
    HDC16       hdc;
    u32         uVar9;
    DEVMODEA   *init_data;
    u32  uVar10;
    i16         iStack32;
    HDC16       local_14;
    LPCSTR      pCStack18;
    LPCSTR      pCStack16;
     let mut local_e: u16;
     let mut uStack12: u16;
     let mut uStack10: u16;
     let mut uStack8: u16;
    HGDIOBJ16   stock_obj_handle;
    HPEN16      pen_handle;

    uVar10           = 0x1;
    pen_handle       = CreatePen16(in_style_3, -0x2805, 0x77);
    uVar8            = 0x5;
    stock_obj_handle = GetStockObject16(s_tile2_bmp_1050_1538);
    local_e          = 0x0;
    uStack12         = 0x0;
    uStack10         = 0x1;
    uStack8          = 0x1;
    puVar2           = *(param_1 + 0x26 + param_3 * 0x4);
    puVar6           = (param_1 + 0x26 + param_3 * 0x4 + 0x2);
    if((puVar6 | puVar2) != 0x0)
    {
        ppcVar3 = *puVar2;
        (**ppcVar3)(s_tile2_bmp_1050_1538, puVar2, puVar6, 0x1, uVar8, uVar10);
        puVar6 = extraout_DX;
    }
    iVar4 = param_3 + 0x105;
    pass1_1010_8170(_PTR_LOOP_1050_14cc, iVar4, puVar6, s_tile2_bmp_1050_1538);
    iVar7                    = param_3 * 0x4;
    (param_1 + iVar7 + 0x26) = iVar4;
    (param_1 + iVar7 + 0x28) = puVar6;
    init_data                = (DEVMODEA *)0x0;
    uVar9                    = pass1_1008_4772(*(astruct_76 **)(param_1 + 0x26 + iVar7));
    output                   = (uVar9 >> 0x10);
    pCStack18                = uVar9;
    pCStack16                = output;
    local_14                 = CreateDC16(0x1008, pCStack18, output, init_data);
    b_force_background       = palette_op_1008_4e08(*(astruct_13 **)(_PTR_LOOP_1050_4230 + 0xe), &local_14, output, 0x1008);
    handle                   = SelectObject16(0x1008, pen_handle);
    hdc                      = s_tile2_bmp_1050_1538;
    handle_00                = SelectObject16(s_tile2_bmp_1050_1538, stock_obj_handle);
    iStack32                 = 0x0;
    while(true)
    {
        piVar1 = (param_1 + 0x74);
        if(*piVar1 == iStack32 || *piVar1 < iStack32)
            break;
        iVar4 = (iStack32 * 0x10 + param_3) * 0x8;
        hdc   = 0x1000;
        uVar5 = pass1_1000_484c(CONCAT22(param_5, &local_e), CONCAT22((param_1 + 0x72), iVar4 + (param_1 + 0x70)), 0x8);
        if(uVar5 != 0x0)
        {
            uVar10 = (param_1 + 0x70);
            uVar8  = (uVar10 >> 0x10);
            iVar7  = uVar10;
            iVar9  = (astruct_4 *)(iVar4 + iVar7);
            hdc    = s_tile2_bmp_1050_1538;
            Rectangle16(0x1000, iVar9->field_0x6, iVar9->field_0x4, iVar9->field_0x2, (iVar7 + iVar4));
        }
        iStack32 = iStack32 + 0x1;
    }
    SelectPalette16(hdc, 0x0, b_force_background);
    DeleteObject16(s_tile2_bmp_1050_1538);
    SelectObject16(s_tile2_bmp_1050_1538, handle);
    SelectObject16(s_tile2_bmp_1050_1538, handle_00);
    DeleteDC16(s_tile2_bmp_1050_1538);
    DeleteObject16(s_tile2_bmp_1050_1538);
    return;
}

pub fn pt_in_rect_1010_4e08(param_1: u32, param_2: u16, param_3: u16, RECT16 *param_4)

{
     let mut piVar1: *mut i16;
    bool    bVar2;
    BOOL16  BVar3;
    i16     iVar4;
     let mut uVar5: u16;
    i16     iStack12;
    i16     iStack10;
    POINT16 PStack8;

    PStack8        = (POINT16)CONCAT22(param_2, param_3);
    uVar5          = (param_1 >> 0x10);
    iVar4          = param_1;
    (iVar4 + 0x22) = (iVar4 + 0x20);
    bVar2          = false;
    (iVar4 + 0x24) = 0x0;
    iStack12       = 0x0;
    iStack10       = 0x0;
    do
    {
        piVar1 = (iVar4 + 0x30);
        if(*piVar1 == iStack12 || *piVar1 < iStack12)
        {
        LAB_1010_4e67:
            if(iStack10 != 0x0)
            {
                (iVar4 + 0x20) = iStack10;
            }
            if(bVar2)
            {
                return;
            }
            return;
        }
        BVar3 = PtInRect16(param_4, PStack8);
        if(BVar3 != 0x0)
        {
            iStack10 = iStack12;
            bVar2    = true;
            goto LAB_1010_4e67;
        }
        iStack12 = iStack12 + 0x1;
        param_4  = s_tile2_bmp_1050_1538;
    } while(true);
}

i16 pt_in_rect_1010_40f8(param_1: u32, POINT16 *param_2, RECT16 *param_3)

{
     let mut piVar1: *mut i16;
    code      **ppcVar2;
    BOOL16      BVar3;
     let mut uVar4: u16;
     let mut uVar5: u16;
    i16         iVar6;
     let mut in_DX: *mut u8;
     let mut puVar7: *mut u8;
     let mut puVar8: *mut u8;
    i16         unaff_DI;
     let mut uVar9: u16;
     let mut unaff_SS: u16;
    u16        *puVar10;
    u32 *puStack16;
    i16         iStack6;
     let mut uStack4: u16;

    iStack6 = 0x0;
    uStack4 = 0x0;
    do
    {
        uVar9  = (param_1 >> 0x10);
        piVar1 = (param_1 + 0x74);
        if(*piVar1 == iStack6 || *piVar1 < iStack6)
        {
        LAB_1010_413e:
            if((uStack4 != 0x0) && (0x3 < globals.PTR_LOOP_1050_3960))
            {
                puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, iStack6 + 0xcU, unaff_SS, in_DX, unaff_DI);
                puVar7  = (puVar10 >> 0x10);
                uVar4   = pass1_1018_0afa(puVar10);
                if(uVar4 == 0x0)
                {
                    uVar9 = 0x1000;
                    uVar5 = uVar4;
                    mem_op_1000_179c(0xb4, puVar7, 0x1000);
                    puVar8 = (puVar7 | uVar5);
                    if(puVar8 == 0x0)
                    {
                        iVar6  = 0x0;
                        puVar8 = 0x0;
                    }
                    else
                    {
                        uVar9 = SUB42(&PTR_LOOP_1050_1040, 0x0);
                        iVar6 = string_1040_8520(CONCAT22(puVar7, uVar5), globals.PTR_LOOP_1050_0396, 0x30, 0x2, 0x643, 0x645, puVar8, unaff_SS);
                    }
                    puStack16 = CONCAT22(puVar8, iVar6);
                    ppcVar2   = (*puStack16 + 0x74);
                    (**ppcVar2)(uVar9, iVar6, puVar8);
                    pass1_1010_209e(_PTR_LOOP_1050_0ed0, iStack6 + 0xcU);
                    uStack4 = uVar4;
                }
            }
            if(uStack4 != 0x0)
            {
                return iStack6;
            }
            return -0x1;
        }
        in_DX = (param_1 + 0x72);
        BVar3 = PtInRect16(param_3, *param_2);
        if(BVar3 != 0x0)
        {
            uStack4 = 0x1;
            goto LAB_1010_413e;
        }
        iStack6 = iStack6 + 0x1;
        param_3 = s_tile2_bmp_1050_1538;
    } while(true);
}

u16 draw_fn_1010_2a32(param_1: u16, param_2: u16, u1616 *__return_storage_ptr__, param_4: i16, param_5: u16, param_6: u32, param_7: u16, param_8: u16, param_9: u16, param_10: u16)

{
     let mut piVar1: *mut i16;
     let mut pcVar2: String;
     let mut pbVar3: *mut u8;
    u32  uVar4;
     let mut bVar5: u8;
     let mut uVar6: u16;
     let mut uVar7: u16;
    code      **ppcVar8;
    code       *pcVar9;
    u1616      *puVar10;
     let mut uVar11: u16;
    HPALETTE16  b_force_background;
    HGDIOBJ16   handle;
     let mut uVar12: u16;
     let mut uVar13: u16;
    BOOL16      BVar14;
    i16         iVar15;
     let mut bVar16: u8;
     let mut extraout_DX: *mut u8;
     let mut extraout_DX_00: *mut u8;
     let mut puVar17: *mut u8;
     let mut extraout_DX_01: *mut u8;
     let mut extraout_DX_02: *mut u8;
     let mut puVar18: *mut u8;
    i16         unaff_SI;
    i16         iVar19;
    i16         iVar20;
     let mut unaff_DI: u16;
     let mut uVar21: u16;
    HDC16       hdc;
     let mut unaff_SS: u16;
     let mut bVar22: u8;
    bool        bVar23;
    undefined1  in_AF;
    u32         uVar24;
     let mut uStack54: u16;
    u32 *puStack46;
     let mut uStack42: u16;
    u32 *puStack38;
    i16         local_22;
    i16         iStack32;
    HGDIOBJ16   HStack30;
     let mut uVar25: u8;
    HGDIOBJ16   handle_00;
     let mut uVar26: u8;
     let mut uVar27: u8;
     let mut uVar28: u8;

    puVar10 = __return_storage_ptr__;
    uVar27  = (undefined)param_9;
    uVar28  = (undefined)(param_9 >> 0x8);
    bVar22  = 0x0;
    uVar26  = 0x0;
    uVar25  = (undefined)(param_4 >> 0x8);
    if((param_6 + 0xec76 & 0x3) != 0x0)
        goto LAB_1010_2ad8;
    uVar11 = param_6 + 0xec76 >> 0x1;
    if(0x1c < uVar11)
        goto LAB_1010_2ad8;
    switch(uVar11)
    {
    default:
        goto switchD_1010_2ab5_caseD_0;
    case 0x1:
    case 0x3:
    case 0xb:
        *(uVar11 + 0xa) = param_8;
    case 0x9:
    case 0xf:
    case 0x15:
    case 0x1b:
        *(uVar11 + 0xa)  = param_8;
        *(uVar11 + 0x10) = param_8;
        *(uVar11 + 0xc)  = param_8;
        return param_10;
    case 0x5:
        BVar14 = write_to_file_1008_7e1c(param_5, param_6, param_8, 0x0, CONCAT13(param_1._1_1_, CONCAT12((undefined)param_1, param_9)), 0x1008);
        if(BVar14 != 0x0)
        {
            return param_7;
        }
        globals.PTR_LOOP_1050_0310 = 0x6d0;
        return param_7;
    case 0x6:
        bVar22 = 0x0;
        goto LAB_1010_2ad8;
    case 0x7:
        ppcVar8 = param_8;
        (**ppcVar8)();
        iVar15  = param_5 + 0x105;
        puVar17 = extraout_DX;
        pass1_1010_8170(_PTR_LOOP_1050_14cc, iVar15, extraout_DX, 0x1010);
        iVar19                                   = param_5 * 0x4;
        (__return_storage_ptr__ + iVar19 + 0x26) = iVar15;
        (__return_storage_ptr__ + iVar19 + 0x28) = puVar17;
        handle_00                                = &USHORT_1050_1050;
        uVar24                                   = pass1_1008_4772(*(astruct_76 **)(__return_storage_ptr__ + iVar19 + 0x26));
        puVar17                                  = (uVar24 >> 0x10);
        CreateDC16(0x1008, uVar24, puVar17, (DEVMODEA *)puVar17);
        b_force_background = palette_op_1008_4e08(*(astruct_13 **)(_PTR_LOOP_1050_4230 + 0xe), &stack0xffec, puVar17, 0x1008);
        handle             = SelectObject16(0x1008, CONCAT11(uVar26, bVar22));
        hdc                = s_tile2_bmp_1050_1538;
        HStack30           = SelectObject16(s_tile2_bmp_1050_1538, handle_00);
        for(iStack32 = 0x0; piVar1 = (__return_storage_ptr__ + 0x74), *piVar1 != iStack32 && iStack32 <= *piVar1; iStack32 = iStack32 + 0x1)
        {
            iVar15             = (iStack32 * 0x10 + param_5) * 0x8;
            puVar17            = (__return_storage_ptr__ + 0x72);
            hdc                = 0x1000;
            b_force_background = 0x48e1;
            uVar11             = pass1_1000_484c(CONCAT13((unaff_SS >> 0x8), CONCAT12(unaff_SS, &stack0xfff2)), CONCAT13((puVar17 >> 0x8), CONCAT12(puVar17, iVar15 + (__return_storage_ptr__ + 0x7))), 0x8);
            if(uVar11 != 0x0)
            {
                uVar4              = (__return_storage_ptr__ + 0x7);
                uVar21             = (uVar4 >> 0x10);
                iVar19             = uVar4;
                iVar20             = iVar15 + iVar19;
                hdc                = s_tile2_bmp_1050_1538;
                b_force_background = (HPALETTE16)&PTR_LOOP_1050_4908;
                Rectangle16(0x1000, (iVar20 + 0x6), (iVar20 + 0x4), (iVar20 + 0x2), (iVar19 + iVar15));
            }
        }
        SelectPalette16(hdc, 0x0, b_force_background);
        DeleteObject16(s_tile2_bmp_1050_1538);
        SelectObject16(s_tile2_bmp_1050_1538, handle);
        SelectObject16(s_tile2_bmp_1050_1538, HStack30);
        DeleteDC16(s_tile2_bmp_1050_1538);
        DeleteObject16(s_tile2_bmp_1050_1538);
        return puVar17;
    case 0x8:
        bVar22 = 0x3;
        goto LAB_1010_2ad8;
    case 0xd:
        pbVar3  = (u8 *)(uVar11 + unaff_SI);
        bVar22  = *pbVar3;
        bVar5   = *pbVar3 + (u8)param_7;
        *pbVar3 = bVar5 + (uVar11 < 0x1c);
        uVar6   = (CARRY1(bVar22, (u8)param_7) || CARRY1(bVar5, uVar11 < 0x1c));
        uVar7   = param_8 + 0xeff0;
        bVar22  = param_8 < 0x1010 || uVar7 < uVar6;
        uVar12  = uVar7 - uVar6;
        pcVar9  = (fn_ptr_1)swi(0x4);
        if(SBORROW2(param_8, 0x1010) != SBORROW2(uVar7, uVar6))
        {
            (*pcVar9)();
            param_7 = extraout_DX_00;
        }
        bVar23  = uVar12 < 0x1010 || uVar12 + 0xeff0 < bVar22;
        pbVar3  = (u8 *)(uVar11 + unaff_SI);
        bVar22  = *pbVar3;
        bVar16  = (u8)param_7;
        bVar5   = *pbVar3;
        *pbVar3 = bVar5 + bVar16 + bVar23;
        pcVar2  = (uVar11 + unaff_SI);
        *pcVar2 = *pcVar2 + bVar16 + (CARRY1(bVar22, bVar16) || CARRY1(bVar5 + bVar16, bVar23));
        struct_op_1018_4cda(CONCAT11(uVar27, uVar26), CONCAT11((undefined)param_1, uVar28), CONCAT11((undefined)param_2, param_1._1_1_));
        iVar15         = CONCAT11(uVar27, uVar26);
        piVar1         = CONCAT13((undefined)param_1, CONCAT12(uVar28, iVar15));
        *piVar1        = s_SCi16ernalPutBldg2_site_0x_08lx__1050_5099 + 0x1;
        (iVar15 + 0x2) = 0x1010;
        pass1_1018_4dce(CONCAT13((undefined)param_1, CONCAT12(uVar28, iVar15)), 0x1b3, param_7, unaff_SS);
        globals._PTR_LOOP_1050_4230 = CONCAT13((undefined)param_1, CONCAT12(uVar28, CONCAT11(uVar27, uVar26)));
        return CONCAT11((undefined)param_1, uVar28);
    case 0xe:
        (__return_storage_ptr__ + 0x2) = param_5;
        break;
    case 0x11:
        do
        {
            // WARNING: Do nothing block with infinite loop
        } while(true);
    case 0x12:
        globals.PTR_LOOP_1050_10c6 = (0x0 < param_5);
        globals.PTR_LOOP_1050_1142 = (0x2 < param_5);
        break;
    case 0x13:
        iVar15 = __return_storage_ptr__ * 0x8 + param_1;
        if((((iVar15 + 0x22) != 0x0) || ((iVar15 + 0x24) != 0x0)) || (((iVar15 + 0x26) != 0x0 || ((iVar15 + 0x28) != 0x0))))
        {
            uVar4    = (param_1 + 0xe);
            HStack30 = 0x627c;
            sys_1000_3f9c(uVar4, (uVar4 >> 0x10), s__d__d__d__d_1050_14ae, &USHORT_1050_1050, *(__return_storage_ptr__ * 0x8 + param_1 + 0x22), &stack0xfffa, param_2, 0x1000, unaff_SS, in_AF);
            uVar4    = (param_1 + 0xa);
            HStack30 = 0x62a0;
            WritePrivateProfileString16(&globals.PTR_LOOP_1050_1000, uVar4, (uVar4 >> 0x10),  * (param_1 + 0xe));
        }
        return param_7;
    case 0x14:
        (__return_storage_ptr__ + 0x24) = param_5;
        break;
    case 0x17:
        puVar17                                     = (param_7 - 0x1);
        pbVar3                                      = (u8 *)(uVar11 + unaff_SI);
        *pbVar3                                     = *pbVar3 | (u8)puVar17;
        *(__return_storage_ptr__ + 0x12) = param_8;
        (__return_storage_ptr__ + 0x14)             = puVar17;
        uStack42                                    = 0x0;
        while(true)
        {
            if(uStack54 <= uStack42)
            {
                BVar14 = read_file_1008_7dee(param_5, param_6, __return_storage_ptr__ + 0x1a, 0x0, param_4, 0x2, 0x1008);
                if(((BVar14 != 0x0) && (BVar14 = read_file_1008_7dee(param_5, param_6, __return_storage_ptr__ + 0x1c, 0x0, param_4, 0x2, 0x1008), BVar14 != 0x0))
                   && (BVar14 = read_file_1008_7dee(param_5, param_6, __return_storage_ptr__ + 0x1e, 0x0, param_4, 0x2, 0x1008), BVar14 != 0x0))
                {
                    return puVar17;
                }
                globals.PTR_LOOP_1050_0310 = 0x6d2;
                return puVar17;
            }
            uVar11 = uStack54;
            mem_op_1000_179c(0x8, puVar17, 0x1000);
            puStack46 = CONCAT22(puVar17, uVar11);
            puVar18   = (puVar17 | uVar11);
            if(puVar18 == 0x0)
            {
                puStack38 = 0x0;
            }
            else
            {
                puStack46      = 0x389a;
                (uVar11 + 0x2) = 0x1008;
                puStack46      = 0xa1c4;
                (uVar11 + 0x2) = 0x1010;
                puStack38      = puStack46;
            }
            BVar14 = read_file_1008_7dee(param_5, param_6, &local_22, 0x0, unaff_SS, 0x2, 0x1008);
            uVar13 = (puStack38 >> 0x10);
            if((BVar14 == 0x0) || (BVar14 = read_file_1008_7dee(param_5, param_6, puStack38 + 0x6, 0x0, uVar13, 0x2, 0x1008), BVar14 == 0x0))
                break;
            iVar15            = switch_1008_73ea(param_5, param_6, local_22);
            (puStack38 + 0x4) = iVar15;
            ppcVar8           = ((__return_storage_ptr__ + 0x12) + 0x4);
            (**ppcVar8)();
            uStack42 = uStack42 + 0x1;
            puVar17  = extraout_DX_02;
        }
        if(puStack38 == 0x0)
        {
            globals.PTR_LOOP_1050_0310 = 0x6d2;
            return puVar18;
        }
        ppcVar8 = *puStack38;
        (**ppcVar8)();
        globals.PTR_LOOP_1050_0310 = 0x6d2;
        return extraout_DX_01;
    case 0x18:
        bVar22 = 0x2;
        goto LAB_1010_2ad8;
    case 0x19:
        uVar13 = pass1_1010_6ca2(CONCAT13(uVar25, CONCAT12(param_4, __return_storage_ptr__)), 0x8, param_7, unaff_SS);
        if(uVar13 != 0x0)
        {
            __return_storage_ptr__ = (s_version__d__d_1050_0012 + 0x8);
            pass1_1010_715c(CONCAT22(0x1a, puVar10), 0x1a, uVar13, param_7, unaff_DI, unaff_SS);
        }
        if(param_5 == 0x2c)
        {
            pass1_1010_715c(CONCAT22(0x1d, __return_storage_ptr__), 0x1d, uVar13, param_7, unaff_DI, unaff_SS);
        }
        uVar13 = pass1_1010_6ca2(0x5a, 0x2, param_7, unaff_SS);
        if(uVar13 != 0x0)
        {
            pass1_1010_715c(0x1c005a, 0x1c, uVar13, param_7, unaff_DI, unaff_SS);
        }
        return param_7;
    case 0x1a:
        (__return_storage_ptr__ + 0x26) = param_5;
    }
    bVar22 = 0x1;
LAB_1010_2ad8:
    if((bVar22 == 0x1) || (bVar22 == 0x2))
    {
        if(bVar22 == 0x1)
        {
            param_5 = (__return_storage_ptr__ + 0x2) + (__return_storage_ptr__ + 0x22) + (__return_storage_ptr__ + 0x24) + (__return_storage_ptr__ + 0x26);
        }
        if(param_5 != 0x0)
        {
            param_7 = param_5 >> 0xf;
            param_5 = param_5 / 0x2 + 0x1;
            if(0x5 < param_5)
            {
                param_5 = 0x5;
            }
            if(((bVar22 == 0x1) && (PTR_LOOP_1050_10c6 != 0x0)) && (0x4 < param_5))
            {
                param_5 = 0x4;
            }
        }
    }
    (bVar22 * 0x7c + 0xed6) = param_5;
    pass1_1010_1f62(unaff_SS, CONCAT13(uVar25, CONCAT12(param_4, __return_storage_ptr__)), 0xc);
switchD_1010_2ab5_caseD_0:
    return param_7;
}

pub fn unk_draw_op_1008_da12(astruct_19 *param_1, param_2: u16, param_3: u16)

{
     let mut piVar1: *mut i16;
     let mut bVar2: u8;
    u32           uVar3;
    u16          *puVar4;
    HDC16         hdc;
     let mut IVar6: u16;
    i16           iVar7;
     let mut uVar8: u16;
    astruct_80   *IVar5;
     let mut start: u16;
     let mut uVar9: u16;
    PALETTEENTRY *entries;
     let mut count: *mut u8;
    i16           iVar10;
    HWND16        hwnd;
    u16          *puStack32;
    i16           iStack16;
    long          lStack8;

    struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa = 0x0;
    param_1->field_0xc = 0x0;
    pass1_1008_3e38(CONCAT22(param_2, &param_1->field_0xe));
    param_1->field_0x14        = 0x0;
    param_1->field_0x16        = 0x0;
    param_1->field_0x18        = 0x0;
    CONCAT22(param_2, param_1) = 0xdc80;
    param_1->field_0x2         = 0x1008;
    hdc                        = GetDC16(0x1010);
    IVar6                      = GetDeviceCaps16(s_tile2_bmp_1050_1538, 0x8);
    param_1->field_0xa         = IVar6;
    IVar6                      = GetDeviceCaps16(s_tile2_bmp_1050_1538, 0xa);
    param_1->field_0xc         = IVar6;
    iVar7                      = param_1->field_0xc + -0x1e0;
    count                      = (iVar7 >> 0xf);
    pass1_1008_3e76(CONCAT22(param_2, &param_1->field_0xe), 0x0, iVar7 / 0x2, (param_1->field_0xa + -0x280) / 0x2);
    hwnd  = (HWND16)s_tile2_bmp_1050_1538;
    uVar8 = GetDeviceCaps16(s_tile2_bmp_1050_1538, 0x26);
    if((uVar8 & 0x100) != 0x0)
    {
        IVar6               = GetDeviceCaps16(s_tile2_bmp_1050_1538, 0x68);
        param_1->field_0x14 = IVar6;
        IVar5               = (astruct_80 *)GetDeviceCaps16(s_tile2_bmp_1050_1538, 0x6a);
        param_1->field_0x16 = IVar5;
        if(_PTR_LOOP_1050_5f2c == 0x0)
        {
            globals.PTR_LOOP_1050_5f2c = mem_op_1000_160a(count, 0x1000);
        }
        else
        {
            count = globals.PTR_LOOP_1050_5f2e;
        }
        start   = fn_ptr_op_1000_1708((IVar5 + 0x1) * 0x4, 0x0, 0x1, globals.PTR_LOOP_1050_5f2c, count, 0x1000);
        lStack8 = CONCAT22(count, start);
        iVar7   = param_1->field_0x16;
        if(_PTR_LOOP_1050_5f2c == 0x0)
        {
            globals.PTR_LOOP_1050_5f2e = count;
            globals.PTR_LOOP_1050_5f2c = mem_op_1000_160a(count, 0x1000);
        }
        else
        {
        }
        uVar9                        = fn_ptr_op_1000_1708((iVar7 + 0x1) * 0x4, 0x0, 0x1, globals.PTR_LOOP_1050_5f2c, globals.PTR_LOOP_1050_5f2e, 0x1000);
        &param_1->field_0x18         = uVar9;
        (&param_1->field_0x18 + 0x2) = globals.PTR_LOOP_1050_5f2e;
        if(lStack8 != 0x0)
        {
            if(param_1->field_0x18 != 0x0)
            {
                entries = (PALETTEENTRY *)(param_1->field_0x16 / 0x2);
                GetSystemPaletteEntries(0x1000, start, (u1616)count, entries);
                GetSystemPaletteEntries(s_tile2_bmp_1050_1538, entries * 0x4 + start, (u1616)count, entries);
                puStack32 = param_1->field_0x18;
                for(iStack16 = 0x0; puVar4 = puStack32, piVar1 = &param_1->field_0x16, *piVar1 != iStack16 && iStack16 <= *piVar1; iStack16 = iStack16 + 0x1)
                {
                    bVar2           = *(u8 *)(iStack16 * 0x4 + start);
                    iVar7           = iStack16 * 0x4 + start;
                    uVar3           = puStack32 >> 0x10;
                    iVar10          = puStack32;
                    puStack32       = (puStack32 & 0xffff0000 | (iVar10 + 0x4));
                    *puVar4         = CONCAT11(*(iVar7 + 0x1), *(iVar7 + 0x2));
                    *(iVar10 + 0x2) = bVar2;
                }
            }
        }
        hwnd = 0x1000;
        fn_ptr_1000_17ce((Struct18 *)CONCAT22(count, start), 0x1000);
    }
    ReleaseDC16(hwnd, hdc);
    return;
}
