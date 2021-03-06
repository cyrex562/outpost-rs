#include "ui_ops_1.h"
#include "types.h"
#include "structs/structs_11.h"
#include "globals.h"
#include "utils.h"
#include "unk/unk_18.h"
#include "string_consts.h"
#include "winapi.h"
#include "fn_ptr_ops/fn_ptr_ops_7.h"
#include "unk/unk_12.h"

void unk_draw_op_1040_b0f8(Globals *globals, Struct18 *param_1)

{
    u16         uVar1;
    u16         uVar2;
    u8         *in_DX;
//    Struct18 *iVar3;
    i16         unaff_DI;
//    u16         uVar3;
    u16         uVar4;
    u16         unaff_SS;
    u16        *puVar5;
    Struct18 *paStack10;

//    uVar3              = (param_1 >> 0x10);
//    iVar3              = (Struct18 *)param_1;
    param_1->field_0x0 = 0xb772;
    param_1->field_0x2   = &globals->PTR_LOOP_1050_1040;
    puVar5             = mixed_1010_20ba(globals->_PTR_LOOP_1050_0ed0, 0x32, unaff_SS, in_DX, unaff_DI);
    uVar4              = 0x1010;
    pass1_1010_7b8c(puVar5, param_1->field_0x6, unaff_SS);
    if(param_1->field_0x8e != 0x0)
    {
        uVar4 = SUB42(s_tile2_bmp_1050_1538, 0x0);
        DeleteObject16(0x1010);
        param_1->field_0x8e = 0x0;
    }
    uVar1     = param_1->field_0x90;
    uVar2     = param_1->field_0x92;
    paStack10 = (Struct18 *)CONCAT22(uVar2, uVar1);
    if((uVar2 | uVar1) != 0x0)
    {
        pass1_1040_a5d0(CONCAT22(uVar2, uVar1));
        uVar4 = 0x1000;
        fn_ptr_1000_17ce(paStack10, 0x1000);
    }
    ui_cleanup_op_1040_782c(param_1, uVar4);
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_ae04(u32 param_1, WORD *param_2)

{
    bool   bVar1;
    i16    iVar2;
    char  *id;
    u8    *in_DX;
    u16    uVar3;
    SEGPTR lp_string;
    i16    iVar4;
    u16    uVar5;
    i16    unaff_DI;
    u16    uVar6;
    u16    uVar7;
    u8     in_AF;
    u16   *puVar8;
    u32    uVar9;
    u32    uVar10;
    char  *pcVar11;
    i16    iStack280;
    CHAR   local_102[0x100];

    puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_2, in_DX, unaff_DI);
    uVar3  = (puVar8 >> 0x10);
    uVar6  = (param_1 >> 0x10);
    iVar4  = param_1;
    pass1_1010_c3c2(puVar8, uVar3, CONCAT22(param_2, local_102), *(iVar4 + 0x94), uVar3, in_AF, param_2);
    SetDlgItemText16(0x1010, (u16)local_102, (SEGPTR)param_2);
    uVar9     = struct_op_1030_73a8(*(iVar4 + 0x94));
    iVar2     = uVar9 + 0x20;
    uVar10    = pass1_1030_8326();
    lp_string = (SEGPTR)(uVar10 >> 0x10);
    bVar1     = false;
    for(iStack280 = 0x0; iStack280 < 0xa; iStack280 = iStack280 + 0x1)
    {
        uVar7 = ((uVar9 & 0xffff0000) >> 0x10);
        if(((iStack280 * 0xc + iVar2 + 0x2) | (iStack280 * 0xc + iVar2)) != 0x0)
        {
            uVar5 = iStack280 * 0xc + iVar2;
            id    = string_op_1020_c222((uVar5 + 0x4));
            SetDlgItemText16(0x1020, (u16)id, lp_string);
            wspri16f16(s_tile2_bmp_1050_1538, local_102, param_2);
            SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538, (u16)local_102, (SEGPTR)param_2);
            uVar10    = unk_load_str_op_1010_8c96(*(iVar4 + 0x98), CONCAT22(param_2, local_102), uVar9 & 0xffff0000 | uVar5, 0x1010, param_2);
            lp_string = (SEGPTR)uVar10;
            SetDlgItemText16(0x1010, (u16)local_102, (SEGPTR)param_2);
            bVar1 = true;
        }
    }
    if(!bVar1)
    {
        pcVar11 = load_string_1010_847e(_PTR_LOOP_1050_14cc, (u16)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        SetDlgItemText16(0x1010, (u16)pcVar11, (SEGPTR)(pcVar11 >> 0x10));
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void msg_box_ui_op_1040_ad66(u32 param_1, char *param_2, u8 *param_3, u16 param_4)

{
    char local_206[0x102];
    char local_104[0x102];

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_206, param_4);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    fn_ptr_1000_17ce((Struct18 *)CONCAT22(param_3, param_2), 0x1000);
    return;
}


void pass1_1040_ad24(i16 param_1, u16 param_2, u16 param_3, u32 param_4, u8 *param_5, u16 param_6, u16 param_7)

{
    if(param_4._2_2_ == 0xeb)
    {
        win_ui_op_1040_ae04(CONCAT22(param_2, param_1), param_7);
    }
    else
    {
        if(param_4._2_2_ != 0x1f0)
        {
            pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, param_6, param_7);
            return;
        }
        msg_box_ui_op_1040_ad66(CONCAT22(param_2, param_1), 0x0, param_5, param_7);
    }
    return;
}


void pass1_1040_ad14(u32 param_1, u16 param_2)

{
    win_ui_op_1040_ae04(param_1, param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_dlg_op_1040_a94a(u32 param_1, u16 param_2)

{
    i16       *piVar1;
    u32        uVar2;
    u32 uVar3;
    u16        uVar4;
    u8        *value;
    char      *pcVar5;
    u16        uVar6;
    u8        *in_DX;
    u8        *puVar8;
    u8        *puVar9;
    u16        lp_string;
    i16        iVar10;
    i16        iVar11;
    i16        unaff_DI;
    u16        uVar12;
    u16        uVar13;
    HWND16     HVar14;
    u8         in_AF;
    bool       bVar15;
    u16       *puVar16;
    long       lVar17;
    u16        uStack288;
    u16        uStack286;
    BOOL16     BStack278;
    i16        iStack276;
    u8         local_102[0x100];
    u32        uVar7;

    puVar16 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_2, in_DX, unaff_DI);
    puVar8  = (puVar16 >> 0x10);
    uVar4   = puVar16;
    uVar12  = (param_1 >> 0x10);
    iVar10  = param_1;
    puVar9  = puVar8;
    pass1_1010_c3c2(uVar4, puVar8, CONCAT22(param_2, local_102), *(iVar10 + 0x94), puVar8, in_AF, param_2);
    SetDlgItemText16(0x1010, (u16)local_102, param_2);
    pass1_1010_c320(uVar4, puVar8, CONCAT22(param_2, local_102), *(iVar10 + 0x94), puVar9);
    SetDlgItemText16(0x1010, (u16)local_102, param_2);
    string_op_1010_c446(param_2, in_AF, puVar9, puVar16, CONCAT22(param_2, local_102), (iVar10 + 0x94));
    value = local_102;
    SetDlgItemText16(0x1010, (u16)value, param_2);
    pass1_1030_6ddc(*(iVar10 + 0x94));
    SetDlgItemi1616(0x1030, 0x0, value, 0x1f5);
    pass1_1030_6e14(*(iVar10 + 0x94));
    SetDlgItemi1616(0x1030, 0x0, value, 0x1f6);
    if((iVar10 + 0x98) != 0x0)
    {
        HVar14 = 0x1010;
        struct_1010_dd5e(uVar4, puVar8, *(iVar10 + 0x94));
        if((puVar9 | value) != 0x0)
        {
            uVar3           = (iVar10 + 0x94);
            uVar13          = (uVar3 >> 0x10);
            iVar11          = uVar3;
            uVar2           = *(iVar11 + 0x26);
            lp_string       = (iVar11 + 0x28);
            iStack276       = 0x1798;
            BStack278       = 0x17c3;
            (iVar10 + 0xea) = 0x0;
            uVar7           = uVar2;
            for(uStack288 = 0x1; uStack288 < 0x25; uStack288 = uStack288 + 0x1)
            {
                if(uVar2 == 0x0)
                {
                    lVar17 = 0x0;
                }
                else
                {
                    HVar14 = 0x1020;
                    lVar17 = pass1_1020_bae6(uVar2, CONCAT22(uStack288, (uVar2 >> 0x10)), uVar7, lp_string, param_2);
                }
                uVar6     = (lVar17 >> 0x10);
                bVar15    = (value + uStack288 * 0x4) != 0x0;
                lp_string = uVar6;
                if(bVar15)
                {
                    pcVar5 = string_1020_c0d8(uStack288);
                    SetDlgItemText16(0x1020, (u16)pcVar5, lp_string);
                    HVar14 = (HWND16)s_tile2_bmp_1050_1538;
                    SetDlgItemi1616((HWND16)s_tile2_bmp_1050_1538, 0x0, (value + uStack288 * 0x4), BStack278);
                }
                uStack286 = lVar17;
                uVar6     = uVar6 | uStack286;
                if(lVar17 != 0x0)
                {
                    if(!bVar15)
                    {
                        pcVar5 = string_1020_c0d8(uStack288);
                        HVar14 = (HWND16)s_tile2_bmp_1050_1538;
                        SetDlgItemText16(0x1020, (u16)pcVar5, lp_string);
                    }
                    SetDlgItemi1616(HVar14, 0x0, uStack286, BStack278);
                    iVar11                                    = (iVar10 + 0xea);
                    HVar14                                    = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, iStack276);
                    *(HWND16 *)(iVar10 + iVar11 * 0x2 + 0x9a) = HVar14;
                    piVar1                                    = (iVar10 + 0xea);
                    *piVar1                                   = *piVar1 + 0x1;
                    iVar11                                    = (iVar10 + 0xea);
                    HVar14                                    = (HWND16)s_tile2_bmp_1050_1538;
                    uVar6                                     = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, BStack278);
                    *(HWND16 *)(iVar10 + iVar11 * 0x2 + 0x9a) = uVar6;
                    piVar1                                    = (iVar10 + 0xea);
                    *piVar1                                   = *piVar1 + 0x1;
                    bVar15                                    = true;
                }
                uVar7 = uVar6;
                if(bVar15)
                {
                    iStack276 = iStack276 + 0x1;
                    BStack278 = BStack278 + 0x1;
                }
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void msg_box_op_1040_a85a(u32 param_1, char *param_2, u8 *param_3, u16 param_4)

{
    char local_206[0x102];
    char local_104[0x102];

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_206, param_4);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    fn_ptr_1000_17ce((Struct18 *)CONCAT22(param_3, param_2), 0x1000);
    return;
}


void pass1_1040_a84a(u32 param_1, u16 param_2)

{
    win_ui_dlg_op_1040_a94a(param_1, param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1040_a784(i16 param_1, i16 param_2, u16 param_3, u32 param_4, u16 param_5, u16 param_6, u16 param_7)

{
    i16 iVar1;

    if(param_4._2_2_ != 0xeb)
    {
        if(param_4._2_2_ == 0x1f4)
        {
            msg_box_op_1040_a85a(CONCAT22(param_2, param_1), 0x0, param_5, param_7);
            return;
        }
        if(param_4._2_2_ == 0x1f7)
        {
            globals->_PTR_LOOP_1050_5ef0 = (param_1 + 0x94);
            pass1_1038_af40(_PTR_LOOP_1050_5b7c, (param_1 + 0x8), 0x23, param_5, param_1, &PTR_LOOP_1050_1038, param_7);
            PostMessage16((HWND16)&PTR_LOOP_1050_1038, 0x0, 0x0, 0x1110002);
            return;
        }
        if(param_4._2_2_ != 0x17d8)
        {
            pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, param_6, param_7);
            return;
        }
        iVar1 = (param_1 + 0x6);
        SetWindowPos16(param_6, 0x6, 0xed, 0x237, 0x0, 0x0, 0x0);
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x17d8);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
        (param_1 + 0x98) = 0x1;
        param_1          = param_2;
        param_2          = iVar1;
    }
    win_ui_dlg_op_1040_a94a(CONCAT22(param_2, param_1), param_7);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_win_ui_op_1040_b230(astruct_1 *param_1, u16 param_2, u16 param_3)

{
    code     **ppcVar1;
    u16      IVar2;
    u8        *in_DX;
    i16        unaff_DI;
    u16        uVar3;
    u16       *puVar4;
    u16       *puVar5;
    u16        uVar7;
    u32 uVar6;
    RECT16     local_1a;
    i16        iStack22;
    i16        iStack20;
    i16        iStack18;
    i16        iStack16;
    i16        iStack14;
    i16        iStack12;
    u16       *puStack10;
    i16        local_6;
    i16        local_4;

    dialog_ui_fn_1040_78e2(param_1, param_2);
    if(PTR_LOOP_1050_5ef8 == (&DAT_1050_0004 + 0x1))
    {
        globals->PTR_LOOP_1050_5ef8 = 0x0;
    }
    puVar5    = CONCAT22(param_3, &local_4);
    puVar4    = CONCAT22(param_3, &local_6);
    puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_3, in_DX, unaff_DI);
    pass1_1008_3e94((puStack10 & 0xffff0000 | (puStack10 + 0xe)), puVar4, puVar5);
    uVar3                       = (puStack10 >> 0x10);
    iStack12                    = (puStack10 + 0xa);
    iStack14                    = (puStack10 + 0xc);
    uVar7                       = 0x4;
    IVar2                       = GetSystemMetrics16(0x1008);
    iStack16                    = IVar2 * globals->PTR_LOOP_1050_5ef8 + 0xa;
    globals->PTR_LOOP_1050_5ef8 = globals->PTR_LOOP_1050_5ef8 + 0x1;
    iStack18                    = iStack16 + local_6;
    iStack16                    = iStack16 + local_4;
    uVar3                       = (param_1 >> 0x10);
    uVar6                       = CONCAT22(uVar7, (param_1 + 0x6));
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538, &local_1a);
    if(iStack14 < (iStack20 - local_1a.y) + iStack18)
    {
        iStack18 = -0x2 - ((iStack20 - local_1a.y) - iStack14);
    }
    if(iStack12 < (iStack22 - local_1a.x) + iStack16)
    {
        iStack16 = -0x2 - ((iStack22 - local_1a.x) - iStack12);
    }
    uVar3 = (param_1 + 0x6);
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538, 0x1, 0x0, 0x0, iStack18, iStack16, 0x0);
    ppcVar1 = (param_1->field_0x0 + 0x6c);
    (**ppcVar1)(s_tile2_bmp_1050_1538, param_1, uVar3, uVar6);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_b54a(i16 param_1, u16 param_2, u16 param_3, u32 param_4, u8 *param_5, u16 param_6, u16 param_7)

{
    Struct18  *paVar1;
    code       **ppcVar2;
    u32   uVar3;
    i16          iVar4;
    i16          iVar5;
    Struct18  *paVar6;
    u16          uVar7;
    astruct_515 *iVar6;
    i16          unaff_DI;
    u16          uVar8;
    u16         *puVar9;
    u8           uVar10;
    u8           uVar11;
    u16          uVar12;
    u16          uVar13;
    u16          uVar14;

    if(param_4._2_2_ == 0xea)
    {
        ppcVar2 = (CONCAT22(param_2, param_1) + 0x5c);
        (**ppcVar2)(param_6, param_1, param_2);
    }
    else
    {
        if(param_4._2_2_ == 0xeb)
        {
            puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_7, param_5, unaff_DI);
            uVar7  = (puVar9 >> 0x10);
            paVar1 = (param_1 + 0x90);
            if(paVar1 != (Struct18 *)0x0)
            {
                uVar8  = (paVar1 >> 0x10);
                uVar12 = 0x1010;
                paVar6 = paVar1;
                pass1_1010_ad64(puVar9, CONCAT22((paVar1 + 0xa), uVar7), *(paVar1 + 0x6), paVar1, uVar7);
                (param_1 + 0x90) = paVar6;
                (param_1 + 0x92) = uVar7;
                if((uVar7 | (param_1 + 0x90)) == 0x0)
                {
                    (param_1 + 0x90) = paVar1;
                }
                else
                {
                    if(paVar1 != (Struct18 *)0x0)
                    {
                        pass1_1040_a5d0(paVar1);
                        uVar12 = 0x1000;
                        fn_ptr_1000_17ce(paVar1, 0x1000);
                    }
                    ppcVar2 = (CONCAT22(param_2, param_1) + 0x70);
                    (**ppcVar2)(uVar12, param_1, param_2);
                }
            }
        }
        else
        {
            if(param_4._2_2_ == 0x1790)
            {
                puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_7, param_5, unaff_DI);
                uVar3  = (param_1 + 0x90);
                uVar3  = (uVar3 + 0x6);
                iVar4  = pass1_1010_7d38(puVar9, (puVar9 >> 0x10), uVar3, (uVar3 >> 0x10), param_7);
                iVar5  = iVar4;
                ui_op_1010_79aa(puVar9, 0xfab, 0x0, param_7);
                if(iVar5 != 0x0)
                {
                    return;
                }
                if(iVar4 == 0x0)
                {
                    uVar3  = (param_1 + 0x90);
                    uVar8  = (uVar3 >> 0x10);
                    iVar6  = (astruct_515 *)uVar3;
                    uVar3  = iVar6->field_0x6;
                    uVar13 = uVar3;
                    uVar14 = (uVar3 >> 0x10);
                    uVar12 = 0x14;
                }
                else
                {
                    uVar3  = (param_1 + 0x90);
                    uVar8  = (uVar3 >> 0x10);
                    iVar6  = (astruct_515 *)uVar3;
                    uVar3  = iVar6->field_0x6;
                    uVar13 = uVar3;
                    uVar14 = (uVar3 >> 0x10);
                    uVar12 = 0x9;
                }
                uVar10 = (undefined)uVar8;
                uVar11 = (undefined)(uVar8 >> 0x8);
            }
            else
            {
                if(param_4._2_2_ == 0x1824)
                {
                    puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_7, param_5, unaff_DI);
                    iVar6  = (astruct_515 *)puVar9;
                    uVar3  = (param_1 + 0x90);
                    ui_op_1010_79aa(puVar9, 0xfc5, (uVar3 + 0x6), param_7);
                    if(iVar6 != (astruct_515 *)0x0)
                    {
                        return;
                    }
                    uVar3  = (param_1 + 0x90);
                    uVar3  = (uVar3 + 0x6);
                    uVar13 = uVar3;
                    uVar14 = (uVar3 >> 0x10);
                    uVar12 = 0x12;
                    uVar10 = 0x0;
                    uVar11 = 0x0;
                }
                else
                {
                    if(param_4._2_2_ != 0x1830)
                    {
                        post_win_msg_1040_7b3c(CONCAT13((param_2 >> 0x8), CONCAT12(param_2, param_1)), param_3, param_4, param_4._2_2_, param_6);
                        return;
                    }
                    puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_7, param_5, unaff_DI);
                    iVar6  = (astruct_515 *)puVar9;
                    uVar3  = (param_1 + 0x90);
                    ui_op_1010_79aa(puVar9, 0xfb6, (uVar3 + 0x6), param_7);
                    if(iVar6 != (astruct_515 *)0x0)
                    {
                        return;
                    }
                    uVar3  = (param_1 + 0x90);
                    uVar3  = (uVar3 + 0x6);
                    uVar13 = uVar3;
                    uVar14 = (uVar3 >> 0x10);
                    uVar12 = 0xc;
                    uVar10 = 0x0;
                    uVar11 = 0x0;
                }
            }
            unk_win_op_1010_7300(puVar9, CONCAT13(uVar11, CONCAT12(uVar10, iVar6)), uVar12, CONCAT22(uVar14, uVar13));
        }
    }
    return;
}


void destroy_window_1040_b726(u32 *param_1, i16 param_2, HWND16 in_win_handle_3)

{
    fn_ptr_1 *ppcVar1;

    if(param_2 != 0x0)
    {
        ppcVar1 = (*param_1 + 0x78);
        (**ppcVar1)(in_win_handle_3, param_1);
    }
    DestroyWindow16(in_win_handle_3);
    return;
}


void pass1_1040_c5ac(u16 *param_1)

{
    u32 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    i16         iVar4;
    u16         uVar5;

    uVar5                       = (param_1 >> 0x10);
    iVar4                       = param_1;
    *param_1                    = 0xc9f2;
    (iVar4 + 0x2)               = &PTR_LOOP_1050_1040;
    globals->PTR_LOOP_1050_5f02 = globals->PTR_LOOP_1050_5f02 + -0x1;
    if(PTR_LOOP_1050_5f02 == 0x0)
    {
        puVar1 = (iVar4 + 0x8);
        uVar2  = (iVar4 + 0xa);
        if((uVar2 | puVar1) != 0x0)
        {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
        puVar1 = (iVar4 + 0xc);
        uVar2  = (iVar4 + 0xe);
        if((uVar2 | puVar1) != 0x0)
        {
            ppcVar3 = *puVar1;
            (**ppcVar3)();
        }
    }
    mix_win_ui_op_1040_911e(param_1);
    return;
}


void win_ui_op_1040_bbe2(i16 param_1, u16 param_2, u16 param_3, u32 param_4, HWND16 param_5, u16 param_6)

{
    code     **ppcVar1;
    BOOL16     BVar2;
    i16        iVar3;
    u16        uVar4;
    u8        *in_DX;
    u16        uVar5;
    u16        uVar6;
    u16        uVar7;
    i16        unaff_DI;
    u16       *puVar8;
    u16       *puVar9;
    u32 uVar10;
    u16        uVar11;
    u16        uVar12;
    u16        uVar13;
    u16        uStack30;
    RECT16     local_a;
    i16        iStack6;
    i16        iStack4;

    if(param_4._2_2_ != 0x10c)
    {
        if(param_4._2_2_ < 0x10d)
        {
            if(param_4._2_2_ == 0xfa)
            {
                uVar10  = (param_1 + 0x98);
                ppcVar1 = ((param_1 + 0x98) + 0x18);
                (**ppcVar1)(param_5, uVar10, (uVar10 >> 0x10), 0x0, globals->_PTR_LOOP_1050_5efe, (_PTR_LOOP_1050_5efe >> 0x10));
                return;
            }
            if(param_4._2_2_ == 0x10a)
            {
                GetClientRect16(param_5, &local_a);
                uVar10    = (param_1 + 0x98);
                local_a.y = local_a.y + 0x3;
                local_a.x = (uVar10 + 0x1a) + -0x9;
                iStack6   = iStack6 + -0x3;
                iStack4   = iStack4 + -0x3;
                InvalidateRect16((HWND16)s_tile2_bmp_1050_1538, (RECT16 *)(&PTR_LOOP_1050_0000 + 0x1), (BOOL16)&local_a);
                unk_destroy_win_op_1010_2fa0(*(param_1 + 0x98), 0x1010);
                pass1_1010_32c0(*(param_1 + 0x98), 0x0);
                pass1_1010_2ee2((param_1 + 0x98), param_6, 0x1010);
                return;
            }
            if(param_4._2_2_ != 0x10b)
                goto LAB_1040_be78;
            uVar10 = (param_1 + 0x98);
            uVar11 = (uVar10 + 0x12);
            uVar6  = uVar11;
            puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, in_DX, unaff_DI);
            uVar5  = (puVar8 >> 0x10);
            puVar9 = puVar8;
            pass1_1010_a5ca(puVar8, uVar5, uVar6, puVar8, uVar5);
            uVar6 = (puVar9 >> 0x10);
            if((uVar11 != 0x70) && (puVar9 == 0x0))
            {
                return;
            }
            uVar10 = (param_1 + 0xb0);
            uVar12 = uVar10;
            uVar13 = (uVar10 >> 0x10);
            uVar10 = (param_1 + 0x98);
            uVar11 = (uVar10 + 0x12);
        }
        else
        {
            if(param_4._2_2_ != 0x10d)
            {
                if(param_4._2_2_ == 0x10e)
                {
                    puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_6, in_DX, unaff_DI);
                    iVar3  = puVar8;
                    ui_op_1010_79aa(puVar8, 0xfc6, (param_1 + 0xb0), param_6);
                    if(iVar3 != 0x0)
                    {
                        return;
                    }
                    unk_win_op_1010_7300(puVar8, 0x0, 0x13, *(param_1 + 0xb0));
                    return;
                }
                if(param_4._2_2_ == 0xbbb)
                {
                    if((param_1 + 0xb6) != 0x0)
                    {
                        BVar2   = IsWindow16(param_5);
                        param_5 = (HWND16)s_tile2_bmp_1050_1538;
                        if(BVar2 != 0x0)
                            goto LAB_1040_bd39;
                    }
                    uVar10           = pass1_1038_af40(_PTR_LOOP_1050_5b7c, (param_1 + 0x6), 0x1b, in_DX, param_1, &PTR_LOOP_1050_1038, param_6);
                    (param_1 + 0xb6) = (uVar10 + 0x6);
                    ShowWindow16((HWND16)&PTR_LOOP_1050_1038, 0x1);
                    return;
                }
                if(param_4._2_2_ == 0xbbc)
                {
                    puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, in_DX, unaff_DI);
                    uVar7  = (puVar8 >> 0x10);
                    uVar6  = puVar8;
                    uVar5  = uVar7;
                    uVar4  = pass1_1010_a5ac(uVar6, uVar7, *(param_1 + 0xb0));
                    uVar11 = uVar4;
                    pass1_1010_a58a(uVar6, uVar7, uVar4, uVar4, uVar5);
                    if(uVar11 == 0x0)
                    {
                        pass1_1010_a568(uVar6, uVar7, uVar4, 0x0, uVar5);
                    }
                    GetDlgItem16(0x1010, 0xbbc);
                    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
                    return;
                }
            LAB_1040_be78:
                pass1_1040_b54a(param_1, param_2, param_3, param_4, in_DX, param_5, param_6);
                return;
            }
            puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, in_DX, unaff_DI);
            uVar6  = (puVar8 >> 0x10);
            uVar10 = (param_1 + 0xb0);
            uVar12 = uVar10;
            uVar13 = (uVar10 >> 0x10);
            uVar11 = 0x71;
            uVar5  = uVar6;
        }
        uStack30 = puVar8;
        param_5  = 0x1010;
        pass1_1010_a5ec(uStack30, uVar5, uVar11, CONCAT22(uVar13, uVar12), uVar6);
        if((param_1 + 0xb4) != 0x0)
        {
            param_5 = (HWND16)s_tile2_bmp_1050_1538;
            BVar2   = IsWindow16(0x1010);
            if(BVar2 != 0x0)
            {
                param_5 = (HWND16)s_tile2_bmp_1050_1538;
                SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x11100eb);
            }
        }
    }
LAB_1040_bd39:
    DestroyWindow16(param_5);
    return;
}


void destroy_win_1040_bb78(astruct_35 *param_1, HWND16 param_2)

{
    u32 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    BOOL16      BVar4;
    astruct_35 *iVar5;
    u16         uVar5;
    HWND16      HVar6;

    uVar5 = (param_1 >> 0x10);
    iVar5 = (astruct_35 *)param_1;
    HVar6 = param_2;
    if(iVar5->field_0xb6 != 0x0)
    {
        HVar6 = (HWND16)s_tile2_bmp_1050_1538;
        BVar4 = IsWindow16(param_2);
        if(BVar4 != 0x0)
        {
            HVar6 = (HWND16)s_tile2_bmp_1050_1538;
            DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
        }
    }
    iVar5->field_0xb6 = 0x0;
    puVar1            = iVar5->field_0x94;
    uVar2             = iVar5->field_0x96;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)(HVar6, puVar1, uVar2, 0x1);
    }
    &iVar5->field_0x94 = 0x0;
    iVar5->field_0x98  = 0x0;
    return;
}


void win_ui_1040_b8d2(astruct_1 *param_1, u16 param_2, u16 param_3, u16 param_4)

{
    u32         *puVar1;
    u16          uVar2;
    u32          uVar3;
    astruct_160 *paVar4;
    u16          uVar5;
    u16          uVar6;
    i16          iVar7;
    u8          *puVar8;
    u8          *puVar9;
    u16          uVar10;
    u16          uVar11;
    astruct_167 *iVar10;
    i16          unaff_DI;
    u16          uVar12;
    u16         *puVar13;

    dialog_ui_fn_1040_78e2(param_1, param_3);
    puVar13                              = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x31, param_4, param_2, unaff_DI);
    puVar9                               = (puVar13 >> 0x10);
    paVar4                               = (astruct_160 *)puVar13;
    uVar12                               = (param_1 >> 0x10);
    iVar10                               = (astruct_167 *)param_1;
    *(astruct_160 **)&iVar10->field_0x98 = paVar4;
    (&iVar10->field_0x98 + 0x2)          = puVar9;
    mem_op_1000_179c(0xa, puVar9, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if(puVar8 == 0x0)
    {
        iVar10->field_0x94 = 0x0;
    }
    else
    {
        puVar13                              = struct_1040_bf3e(CONCAT22(puVar9, paVar4), iVar10->field_0x6);
        puVar8                               = (puVar13 >> 0x10);
        paVar4                               = (astruct_160 *)puVar13;
        *(astruct_160 **)&iVar10->field_0x94 = paVar4;
        (&iVar10->field_0x94 + 0x2)          = puVar8;
    }
    pass1_1040_bfde(iVar10->field_0x94, iVar10->field_0x98, param_4);
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if(puVar9 != 0x0)
    {
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa000a, 0x0, 0x800081, CONCAT22(iVar10->field_0x6, 0x10a), puVar9, param_4);
    }
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if(puVar8 != 0x0)
    {
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa0028, 0x0, 0x840085, CONCAT22(iVar10->field_0x6, 0x10b), puVar8, param_4);
    }
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if(puVar9 != 0x0)
    {
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa0046, 0x0, 0x860087, CONCAT22(iVar10->field_0x6, 0x10d), puVar9, param_4);
    }
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if(puVar8 != 0x0)
    {
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa0064, 0x0, 0x880089, CONCAT22(iVar10->field_0x6, 0x10e), puVar8, param_4);
    }
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if(puVar9 != 0x0)
    {
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa0082, 0x0, 0x820083, CONCAT22(iVar10->field_0x6, 0x10c), puVar9, param_4);
    }
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if(puVar8 != 0x0)
    {
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa00d2, 0x0, 0x8a008b, CONCAT22(iVar10->field_0x6, 0xbbb), puVar8, param_4);
    }
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if(puVar9 == 0x0)
    {
        paVar4 = (astruct_160 *)0x0;
        puVar9 = 0x0;
    }
    else
    {
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa00a0, 0x8e, 0x8c008d, CONCAT22(iVar10->field_0x6, 0xbbc), puVar9, param_4);
    }
    puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_4, puVar9, unaff_DI);
    uVar10  = (puVar13 >> 0x10);
    uVar2   = puVar13;
    uVar11  = uVar10;
    uVar5   = pass1_1010_a5ac(uVar2, uVar10, iVar10->field_0xb0);
    uVar6   = pass1_1010_ac62(uVar2, uVar10, 0x1e, uVar5, uVar11);
    if(uVar6 != 0x0)
    {
        pass1_1010_a5ca(uVar2, uVar10, uVar5, uVar6, uVar11);
        if(0x0 < uVar6)
        {
            pass1_1010_a58a(uVar2, uVar10, uVar5, uVar6, uVar11);
            if(uVar6 == 0x0)
                goto LAB_1040_bb26;
        }
    }
    enable_win_1040_9234(CONCAT22(puVar9, paVar4), 0x0, 0x1010);
LAB_1040_bb26:
    puVar1 = iVar10->field_0x98;
    iVar7  = puVar1;
    uVar3  = puVar1 & 0xffff0000;
    uVar12 = (uVar3 >> 0x10);
    SetWindowPos16(0x1010, 0x40, (iVar7 + 0x10), (iVar7 + 0xe), (iVar7 + 0xc), (uVar3 | iVar7 + 0xa), 0x0);
    return;
}


u16 pass1_1040_b864(u32 *param_1, i16 *param_2, u16 param_3, u16 param_4, i16 param_5, u16 param_6)

{
    fn_ptr_1 *ppcVar1;
    u16       uVar2;

    if(param_5 == 0x2b)
    {
        if(*param_2 == 0x4)
        {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), param_6);
        }
    }
    else
    {
        if(param_5 != 0x111)
        {
            uVar2 = pass1_1040_b316(param_1, param_2, param_3, param_4, param_5);
            return uVar2;
        }
        ppcVar1 = (*param_1 + 0x7c);
        (**ppcVar1)(param_6, param_1, param_2, CONCAT22(param_4, param_3));
    }
    return 0x1;
}


void show_win_1040_b43c(u32 *param_1, HWND16 param_2)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = (*param_1 + 0x70);
    (**ppcVar1)(param_2, param_1);
    ShowWindow16(param_2, 0x5);
    return;
}


void pass1_1040_b45e(u32 param_1, HWND16 param_2)

{
    u32 uVar1;
    i16       *piVar2;
    i16        iVar3;
    u16        uVar4;
    i16        iStack8;
    u16     *pIStack6;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if((iVar3 + 0x90) != 0x0)
    {
        uVar1          = (iVar3 + 0x90);
        (uVar1 + 0x14) = (iVar3 + 0x6);
        uVar1          = (iVar3 + 0x90);
        pIStack6       = (uVar1 + 0x2);
        for(iStack8 = 0x0; piVar2 = *(i16 **)(iVar3 + 0x90), *piVar2 != iStack8 && iStack8 <= *piVar2; iStack8 = iStack8 + 0x1)
        {
            SetDlgItemText16(param_2, *pIStack6, (SEGPTR) * (pIStack6 + 0x2));
            pIStack6 = (u16 *)(pIStack6 & 0xffff0000 | (pIStack6 + 0xa));
            param_2  = (HWND16)s_tile2_bmp_1050_1538;
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1040_b4c8(u32 param_1, u8 *param_2, i16 param_3, u16 param_4)

{
    i16        iVar1;
    u32 uVar2;
    i16        iVar3;
    u16        uVar4;
    u16       *puVar5;

    uVar4 = (param_1 >> 0x10);
    if((param_1 + 0x90) != 0x0)
    {
        puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_4, param_2, param_3);
        uVar2  = (param_1 + 0x90);
        iVar1  = (uVar2 + 0xa);
        iVar3  = iVar1 + -0x4;
        if(iVar3 == 0x0)
        {
            ui_op_1010_79aa(puVar5, 0xfd9, 0x0, param_4);
            if(iVar3 == 0x0)
            {
                uVar4 = 0xe;
            LAB_1040_b50f:
                unk_win_op_1010_7300(puVar5, CONCAT22(iVar3, iVar3), uVar4, CONCAT22(iVar3, iVar3));
                return;
            }
        }
        else
        {
            if(((0x0 < iVar1 + -0x5) && (!SBORROW2(iVar1 + -0x5, 0x1))) && (iVar3 = iVar1 + -0x7, iVar3 == 0x0 || iVar1 + -0x6 < 0x1))
            {
                ui_op_1010_79aa(puVar5, 0xfda, 0x0, param_4);
                if(iVar3 == 0x0)
                {
                    uVar4 = 0xd;
                    goto LAB_1040_b50f;
                }
            }
        }
    }
    return;
}


void send_dlg_item_msg_1040_d20c(u32 param_1, long param_2, u16 param_3, u16 param_4)

{
    u16  in_AX;
    u8  *in_DX;
    u16  uVar1;
    i16  unaff_DI;
    u16  uVar2;
    u8   in_AF;
    u16 *puVar3;
    u8  *puVar4;
    u16  uVar5;
    u8   local_106[0x104];

    if(param_2 == 0x0)
    {
        enable_win_1040_d60e(param_1, param_3);
        return;
    }
    uVar2 = (param_1 >> 0x10);
    if((param_1 + 0xa0) != 0x0)
    {
        pass1_1010_9210(*(param_1 + 0x9c));
        enable_win_1040_d60e(param_1, 0x1010);
        pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), param_2);
        puVar4 = local_106;
        uVar5  = param_4;
        puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_4, in_DX, unaff_DI);
        uVar1  = (puVar3 >> 0x10);
        pass1_1010_c3c2(puVar3, uVar1, CONCAT22(uVar5, puVar4), CONCAT22(in_DX, in_AX), uVar1, in_AF, param_4);
        SendDlgItemMessage16(0x1010, (u16)local_106, param_4, 0x0, 0x18470401);
    }
    return;
}


void win_ui_op_1040_d2ac(i16 param_1, u16 param_2, u16 param_3, u32 param_4, u16 param_5, u16 param_6, u16 param_7)

{
    LRESULT LVar1;

    if(param_4._2_2_ == s_dibtext_bmp_1050_1844 + 0x4U)
    {
        SendDlgItemMessage16(param_6, 0x0, 0x0, 0x0, 0x18470405);
        struct_1010_9172(*(param_1 + 0x9c));
    }
    else
    {
        if(s_dibtext_bmp_1050_1844 + 0x4U < param_4._2_2_)
        {
            if(param_4._2_2_ == s_dibtext_bmp_1050_1844 + 0x5U)
            {
                LVar1 = SendDlgItemMessage16(param_6, 0x0, 0x0, 0x0, 0x1847040c);
                if((LVar1 != -0x1) || ((LVar1 >> 0x10) != -0x1))
                {
                    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, LVar1 - 0x1, 0x18470403);
                    pass1_1010_91cc(*(param_1 + 0x9c));
                }
            }
            else
            {
                if(param_4._2_2_ == s_dibtext_bmp_1050_1844 + 0x6U)
                {
                    enable_win_1040_d6be(CONCAT22(param_2, param_1), param_6);
                    pass1_1018_57d2(*(param_1 + 0x94), CONCAT22(param_2, param_1));
                    PostMessage16(0x1018, 0x0, 0x0, 0x1110203);
                }
                else
                {
                    if(param_4._2_2_ != s_dibtext_bmp_1050_1844 + 0x7U)
                        goto LAB_1040_d3b3;
                    globals->_PTR_LOOP_1050_5a68 = (param_1 + 0x98);
                    pass1_1038_af40(_PTR_LOOP_1050_5b7c, (param_1 + 0x6), 0x27, param_5, param_1, &PTR_LOOP_1050_1038, param_7);
                }
            }
        }
        else
        {
            if(param_4._2_2_ == 0xeb)
            {
                send_ldg_item_msg_1040_d79c(CONCAT22(param_2, param_1), param_7);
            }
            else
            {
                if(param_4._2_2_ != s_vrpal_bmp_1050_183a + 0x7U)
                {
                LAB_1040_d3b3:
                    pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, param_6, param_7);
                    return;
                }
                msg_box_op_1040_d3d0(CONCAT22(param_2, param_1), 0x0, param_5, param_7);
            }
        }
    }
    return;
}


void msg_box_op_1040_d3d0(u32 param_1, char *param_2, u8 *param_3, u16 param_4)

{
    char local_206[0x102];
    char local_104[0x102];

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_206, param_4);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    fn_ptr_1000_17ce((Struct18 *)CONCAT22(param_3, param_2), 0x1000);
    return;
}


void enable_win_1040_d60e(u32 param_1, HWND16 param_2)

{
    GetDlgItem16(param_2, 0x1);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x2);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, s_vrpal_bmp_1050_183a + 0x7);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, s_dibtext_bmp_1050_1844 + 0x4);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, s_dibtext_bmp_1050_1844 + 0x5);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, s_dibtext_bmp_1050_1844 + 0x6);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, s_dibtext_bmp_1050_1844 + 0x7);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
    (param_1 + 0xa0) = 0x0;
    return;
}


void enable_win_1040_d6be(u32 param_1, HWND16 param_2)

{
    GetDlgItem16(param_2, 0x1);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x2);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, s_vrpal_bmp_1050_183a + 0x7);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, s_dibtext_bmp_1050_1844 + 0x4);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, s_dibtext_bmp_1050_1844 + 0x5);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, s_dibtext_bmp_1050_1844 + 0x6);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, s_dibtext_bmp_1050_1844 + 0x7);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0);
    (param_1 + 0xa0) = 0x1;
    return;
}


void send_ldg_item_msg_1040_d79c(u32 param_1, u16 param_2)

{
    u16     uVar1;
    u8     *in_DX;
    u16     uVar2;
    u16     uVar3;
    i16     iVar4;
    i16     unaff_DI;
    u16     uVar5;
    HWND16  hwnd;
    u8      in_AF;
    LRESULT LVar6;
    u32     uStack270;
    u32     uStack266;
    char    local_106[0x100];
    u16    *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_2, in_DX, unaff_DI);
    uVar2    = (puStack6 >> 0x10);
    uVar5    = (param_1 >> 0x10);
    iVar4    = param_1;
    pass1_1010_c3c2(puStack6, uVar2, CONCAT22(param_2, local_106), *(iVar4 + 0x98), uVar2, in_AF, param_2);
    SendDlgItemMessage16(0x1010, (u16)local_106, param_2, 0x0, 0x1846000c);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x1847000b);
    LVar6 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x18470405);
    uVar3 = (LVar6 >> 0x10);
    uVar1 = LVar6;
    hwnd  = 0x1010;
    pass1_1010_9044(*(iVar4 + 0x9c));
    uStack266 = CONCAT22(uVar3, uVar1);
    for(uStack270 = 0x0; uStack270 < uStack266; uStack270 = uStack270 + 0x1)
    {
        hwnd = 0x1010;
        pass1_1010_9130(*(iVar4 + 0x9c), CONCAT22(param_2, local_106), local_106, uVar3, param_2, in_AF);
        if(local_106[0] != '\0')
        {
            hwnd  = (HWND16)s_tile2_bmp_1050_1538;
            LVar6 = SendDlgItemMessage16(0x1010, (u16)local_106, param_2, 0x0, 0x18470401);
            uVar3 = (LVar6 >> 0x10);
        }
    }
    SendDlgItemMessage16(hwnd, 0x0, 0x0, 0x1, 0x1847000b);
    return;
}

void pass1_1040_d29c(u32 param_1, u16 param_2)

{
    send_ldg_item_msg_1040_d79c(param_1, param_2);
    return;
}


LRESULT send_dlg_msg_1040_cf1c(u32 param_1, u16 param_2)

{
    u8        *in_DX;
    u16        uVar1;
    u16        uVar2;
    i16        unaff_DI;
    u16        uVar3;
    u8         in_AF;
    LRESULT    LVar4;
    BOOL16     enable;
    char       local_50c[0x402];
    u32 uStack266;
    u8         local_106[0x100];
    u16       *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_2, in_DX, unaff_DI);
    uVar1    = (puStack6 >> 0x10);
    uVar3    = (param_1 >> 0x10);
    uVar2    = param_1;
    pass1_1010_c3c2(puStack6, uVar1, CONCAT22(param_2, local_106), *(uVar2 + 0x94), uVar1, in_AF, param_2);
    SendDlgItemMessage16(0x1010, (u16)local_106, param_2, 0x0, 0x1846000c);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x1842000b);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x18420405);
    uVar1     = s_vrpal_bmp_1050_183a + 0x8;
    uStack266 = pass1_1018_526a(*(uVar2 + 0x98), *(uVar2 + 0x94), param_2);
    send_dlg_item_msg_1040_ce12(uVar2, uVar3, uStack266, uVar1, param_2);
    LVar4 = SendDlgItemMessage16(0x1018, 0x0, 0x0, 0x0, 0x1842040c);
    if(((LVar4 >> 0x10) != 0x0 && -0x1 < LVar4) || ((-0x1 < LVar4 && (LVar4 != 0x0))))
    {
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        enable = 0x1;
    }
    else
    {
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_50c, param_2);
        SendDlgItemMessage16(0x1010, (u16)local_50c, param_2, 0x0, 0x18420401);
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        enable = 0x0;
    }
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, enable);
    LVar4 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1, 0x1842000b);
    return LVar4;
}


void send_dlg_item_1040_ce76(u32 param_1, HWND16 param_2, u16 param_3)

{
    i16      iVar1;
    u16      uVar2;
    LRESULT  LVar3;
    u32      uVar4;
    u8       local_106[0x100];
    WPARAM16 WStack6;
    i16      iStack4;

    uVar2   = (param_1 >> 0x10);
    iVar1   = param_1;
    LVar3   = SendDlgItemMessage16(param_2, 0x0, 0x0, 0x0, 0x18420409);
    WStack6 = (WPARAM16)LVar3;
    iStack4 = WStack6 >> 0xf;
    if(WStack6 != 0xffff)
    {
        SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, (u16)local_106, param_3, WStack6, 0x1842040a);
        uVar4 = pass1_1018_5206(*(iVar1 + 0x98), CONCAT22(param_3, local_106), param_3);
        if(uVar4 != 0x0)
        {
            (iVar1 + 0x9c) = (uVar4 + 0x8);
            (iVar1 + 0x9e) = 0x0;
            SetDlgItemi1616(0x1018, 0x0, 0x0, 0x18e);
            SetDlgItemi1616((HWND16)s_tile2_bmp_1050_1538, 0x0, (iVar1 + 0x9c), 0x191);
        }
    }
    return;
}

void send_dlg_item_msg_1040_ce12(u16 param_1, u16 param_2, u32 param_3, u16 param_4, WORD *param_5)

{
    long lVar1;
    CHAR local_10a[0x100];
    u8   local_a[0x8];

    pass1_1008_5784(CONCAT22(param_5, local_a), param_3);
    while(true)
    {
        lVar1 = pass1_1008_5b12(local_a, param_5);
        if(lVar1 == 0x0)
            break;
        wspri16f16(0x1008, local_10a, param_5);
        SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538, (u16)local_10a, param_5, 0x0, CONCAT22(param_4, 0x401));
    }
    return;
}


u16 pass1_1040_cdac(u32 param_1, u16 param_2, u16 param_3, i16 param_4, HWND16 param_5)

{
    i16 *piVar1;
    i16  iVar2;
    bool bVar3;
    i16  iVar4;
    u16  uVar5;

    bVar3 = false;
    iVar4 = param_1;
    uVar5 = (param_1 >> 0x10);
    if(param_4 == 0x0)
    {
        iVar2  = (iVar4 + 0x9e);
        piVar1 = (iVar4 + 0x9c);
        if(*piVar1 == iVar2 || *piVar1 < iVar2)
            goto LAB_1040_cdef;
        piVar1  = (iVar4 + 0x9e);
        *piVar1 = *piVar1 + 0x1;
    }
    else
    {
        if(param_4 != 0x1)
            goto LAB_1040_cdef;
        if((iVar4 + 0x9e) < 0x1)
            goto LAB_1040_cdef;
        piVar1  = (iVar4 + 0x9e);
        *piVar1 = *piVar1 + -0x1;
    }
    bVar3 = true;
LAB_1040_cdef:
    if(bVar3)
    {
        SetDlgItemi1616(param_5, 0x0, (iVar4 + 0x9e), 0x18e);
    }
    return 0x0;
}


void msg_box_op_1040_cce4(u32 param_1, char *param_2, u8 *param_3, u16 param_4)

{
    u32  uStack522;
    char local_206[0x102];
    char local_104[0x102];
    u16  uVar2;
    u16  uVar3;

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_206, param_4);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    fn_ptr_1000_17ce((Struct18 *)CONCAT22(param_3, param_2), 0x1000);
    return;
}

void pass1_1040_cc8c(i16 param_1, u16 param_2, u16 param_3, u32 param_4, u8 *param_5, u16 param_6, u16 param_7)

{
    if(param_4._2_2_ == 0xeb)
    {
        send_dlg_msg_1040_cf1c(CONCAT22(param_2, param_1), param_7);
    }
    else
    {
        if(param_4._2_2_ == s_vrpal_bmp_1050_183a + 0x7)
        {
            msg_box_op_1040_cce4(CONCAT22(param_2, param_1), 0x0, param_5, param_7);
        }
        else
        {
            if(param_4._2_2_ != s_vrpal_bmp_1050_183a + 0x8)
            {
                pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, param_6, param_7);
                return;
            }
            if(param_4 == 0x1)
            {
                send_dlg_item_1040_ce76(CONCAT22(param_2, param_1), param_6, param_7);
            }
        }
    }
    return;
}

LRESULT pass1_1040_cc66(u32 param_1, u16 param_2)

{
    code  **ppcVar1;
    LRESULT LVar2;

    ppcVar1 = ((param_1 + 0x98) + 0x10);
    (**ppcVar1)();
    LVar2 = send_dlg_msg_1040_cf1c(param_1, param_2);
    return LVar2;
}


void win_ui_op_1040_cace(u32 param_1, HWND16 param_2, u16 param_3)

{
    u32 uVar1;
    bool       bVar2;
    i16        iVar3;
    u16      IVar4;
    u16        in_DX;
    u16        uVar5;
    u16        uVar6;
    bool       bVar7;
    u16        uVar8;
    char       local_208[0x100];
    char       local_108[0x100];
    u16        UStack8;
    u16        uStack6;
    BOOL16     local_4;

    uVar6   = (param_1 >> 0x10);
    uVar5   = param_1;
    uStack6 = GetDlgItemi1616(param_2, 0x0, &local_4, param_3);
    UStack8 = GetDlgItemi1616((HWND16)s_tile2_bmp_1050_1538, 0x0, &local_4, param_3);
    if(uStack6 == 0x0)
    {
        return;
    }
    pass1_1018_50ea(*(uVar5 + 0x98), uStack6, *(uVar5 + 0x94));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_208, param_3);
    uVar1 = (uVar5 + 0x94);
    uVar8 = (_PTR_LOOP_1050_14cc >> 0x10);
    if((uVar1 + 0x36) == 0x0)
    {
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, uVar8, 0x3ff, local_108, param_3);
        iVar3 = MessageBox16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0x14), local_208, param_3);
    }
    else
    {
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, uVar8, 0x3ff, local_108, param_3);
        iVar3 = MessageBox16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0x14), local_208, param_3);
    }
    bVar2 = iVar3 == 0x6;
    bVar7 = false;
    if((!bVar2) && (uVar1 = (uVar5 + 0x94), (uVar1 + 0x34) < 0x1))
    {
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_108, param_3);
        IVar4 = MessageBox16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0x14), local_208, param_3);
        bVar7 = IVar4 == 0x6;
        bVar2 = false;
    }
    if(bVar2)
    {
        globals->_PTR_LOOP_1050_5f16 = (uVar5 + 0x94);
        iVar3                        = 0x26;
    }
    else
    {
        if(!bVar7)
        {
            return;
        }
        globals->_PTR_LOOP_1050_5a68 = (uVar5 + 0x94);
        iVar3                        = 0x27;
    }
    pass1_1038_af40(_PTR_LOOP_1050_5b7c, (uVar5 + 0x8), iVar3, in_DX, uVar5, &PTR_LOOP_1050_1038, param_3);
    return;
}


void pass1_1040_caa6(u16 param_1, u32 param_2, u8 *param_3, i16 param_4, u16 param_5)

{
    u16 *puVar1;
    i16  iVar2;

    iVar2  = 0x0;
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    pass1_1010_038e(puVar1, iVar2, param_5);
    destroy_window_1040_b726(CONCAT22(param_2, param_1), (param_2 >> 0x10), 0x1010);
    return;
}


void get_dlg_item_1040_a3d0(u32 param_1, HWND16 param_2)

{
    long        lVar1;
    astruct_49 *iVar3;
    i16         unaff_DI;
    u16         uVar2;
    u16         unaff_SS;

    uVar2 = (param_1 >> 0x10);
    iVar3 = (astruct_49 *)param_1;
    if(iVar3->field_0x90 != 0x0)
    {
        lVar1          = iVar3->field_0x90;
        (lVar1 + 0x14) = iVar3->field_0x6;
        GetDlgItem16(param_2, 0x1826);
        win_msg_1040_a308(param_1, unaff_DI, (HWND16)s_tile2_bmp_1050_1538, unaff_SS);
    }
    return;
}


void enable_win_1040_86dc(HWND16 param_1)

{
    HWND16 HVar1;

    HVar1 = GetDlgItem16(param_1, 0x1);
    if(HVar1 != 0x0)
    {
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x2);
        if(HVar1 != 0x0)
        {
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538, 0x1);
        }
    }
    return;
}


void destroy_win_1040_8b7e(HWND16 param_1)

{
    DestroyWindow16(param_1);
    return;
}


void load_icon_1040_8b92(u32 param_1, HINSTANCE16 param_2)

{
    u8      bVar1;
    HICON16 HVar2;
    u16     uVar3;
    LPCSTR  name;

    uVar3 = (param_1 >> 0x10);
    bVar1 = *(u8 *)(param_1 + 0x98) & 0xf0;
    if(bVar1 == 0x30)
    {
        name = 0x7f03;
    }
    else
    {
        if((bVar1 == 0x10) || (bVar1 == 0x10))
        {
            name = 0x7f01;
        }
        else
        {
            if((bVar1 == 0x40) || (bVar1 == 0x40))
            {
                name = 0x7f04;
            }
            else
            {
                if(bVar1 != 0x20)
                {
                    return;
                }
                name = 0x7f02;
            }
        }
    }
    HVar2                        = LoadIcon16(param_2, name);
    *(HICON16 *)(param_1 + 0x8e) = HVar2;
    return;
}


void get_sys_metrics_1040_8c66(astruct_37 *param_1, HWND16 param_2)

{
    i16  *piVar1;
    u8    bVar2;
    HDC16 hdc;
    u16 IVar3;
    i16   iVar4;
    u16   uVar5;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    hdc   = GetDC16(param_2);
    draw_text_1040_8d14(param_1, s_tile2_bmp_1050_1538);
    (iVar4 + 0xa6) = (iVar4 + 0x9e);
    (iVar4 + 0xaa) = (iVar4 + 0xa2);
    IVar3          = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
    piVar1         = (iVar4 + 0xac);
    *piVar1        = *piVar1 + IVar3;
    bVar2          = *(u8 *)(iVar4 + 0x98) & 0xf0;
    if((((bVar2 == 0x30) || (bVar2 == 0x10)) || (bVar2 == 0x40)) || (bVar2 == 0x20))
    {
        IVar3 = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
        if((iVar4 + 0xac) < IVar3)
        {
            IVar3          = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
            (iVar4 + 0xac) = IVar3;
        }
    }
    piVar1         = (iVar4 + 0xaa);
    *piVar1        = *piVar1 + 0x14;
    piVar1         = (iVar4 + 0xac);
    *piVar1        = *piVar1 + 0xa;
    (iVar4 + 0xb0) = (iVar4 + 0xac);
    piVar1         = (iVar4 + 0xac);
    *piVar1        = *piVar1 + 0x30;
    ReleaseDC16((HWND16)s_tile2_bmp_1050_1538, hdc);
    return;
}


void draw_text_1040_8d14(astruct_37 *param_1, HWND16 param_2)

{
    u8          bVar1;
    u16       IVar2;
    HANDLE16    handle;
    astruct_37 *iVar3;
    RECT16     *rect;
    HGDIOBJ16   HStack8;

    rect  = (RECT16 *)(param_1 >> 0x10);
    iVar3 = (astruct_37 *)param_1;
    bVar1 = iVar3->field_0x98 & 0xf0;
    if((((bVar1 == 0x30) || (bVar1 == 0x10)) || (bVar1 == 0x40)) || (bVar1 == 0x20))
    {
        iVar3->field_0xa0 = 0xa;
        IVar2             = GetSystemMetrics16(param_2);
        iVar3->field_0x9e = IVar2 + 0x28;
        param_2           = (HWND16)s_tile2_bmp_1050_1538;
    }
    else
    {
        iVar3->field_0xa0 = 0xa;
        iVar3->field_0x9e = 0x14;
    }
    HStack8 = 0x0;
    handle  = GetProp16(param_2, 0x5e0f);
    if(handle != 0x0)
    {
        HStack8 = SelectObject16((HDC16)s_tile2_bmp_1050_1538, handle);
    }
    DrawText16((HDC16)s_tile2_bmp_1050_1538, 0x410, (u16)&iVar3->field_0x9e, rect, 0xffff);
    if(HStack8 != 0x0)
    {
        SelectObject16((HDC16)s_tile2_bmp_1050_1538, HStack8);
    }
    return;
}


void enable_window_1040_8ea0(u16 param_1, u16 param_2, u16 param_3, u32 param_4, u16 param_5)

{
    HWND16 enable;
    u8    *in_DX;
    u16    unaff_SS;

    if(param_4._2_2_ == 0xf8)
    {
        GetDlgItem16(param_5, 0x17d8);
        enable = 0x1;
    }
    else
    {
        if(param_4._2_2_ != 0x17d8)
        {
            pass1_1040_b54a(param_1, param_2, param_3, param_4, in_DX, param_5, unaff_SS);
            return;
        }
        SetWindowPos16(param_5, 0x6, 0xf6, 0x269, 0x0, 0x0, 0x0);
        enable = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x17d8);
    }
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538, enable);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void create_window_1040_92dc(u32 param_1, u16 param_2)

{
    HWND16 HVar1;
    LPCSTR str;
    LPCSTR str_00;
    LPCSTR str_01;
    long   lVar2;

    str_01 = (param_1 >> 0x10);
    str_00 = param_1;
    if((str_00 + 0x18) == 0x0)
    {
        HVar1 = CreateWindow16(param_2, 0x0, ZEXT24(globals->PTR_LOOP_1050_038c) << 0x10, (str_00 + 0x1c), (str_00 + 0x1a), 0x0, 0x0, *(HWND16 *)(str_00 + 0x20), *(HMENU16 *)(str_00 + 0x1e), 0xb, 0x4000);
        *(HWND16 *)(str_00 + 0x18) = HVar1;
        lVar2                      = SetWindowLong16((HWND16)s_tile2_bmp_1050_1538, (u16)_PTR_LOOP_1050_5e18, CONCAT22(0xfffc, (_PTR_LOOP_1050_5e18 >> 0x10)));
        str                        = (lVar2 >> 0x10);
        (str_00 + 0x14)            = lVar2;
        *(LPCSTR *)(str_00 + 0x16) = str;
        SetProp16((HWND16)s_tile2_bmp_1050_1538, str, s_procHi_1050_5e46);
        SetProp16((HWND16)s_tile2_bmp_1050_1538, *(LPCSTR *)(str_00 + 0x14), s_procLo_1050_5e4d);
        SetProp16((HWND16)s_tile2_bmp_1050_1538, str_01, s_thisHi_1050_5e54);
        SetProp16((HWND16)s_tile2_bmp_1050_1538, str_00, s_thisLo_1050_5e5b);
        if((str_00 + 0x40) != 0x0)
        {
            SetProp16((HWND16)s_tile2_bmp_1050_1538, (&PTR_LOOP_1050_0000 + 0x1), 0x5e62);
        }
        ShowWindow16((HWND16)s_tile2_bmp_1050_1538, 0x5);
    }
    return;
}


void mov_update_win_1040_93aa(astruct_65 *param_1, u16 param_2, u16 param_3, HWND16 param_4)

{
    astruct_65 *iVar1;
    u16         uVar1;

    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_65 *)param_1;
    iVar1->field_0x1e = param_3;
    iVar1->field_0x20 = param_2;
    MoveWindow16(param_4, 0x1, iVar1->field_0x24, iVar1->field_0x22, param_2, iVar1->field_0x1e);
    UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
    return;
}


i16 string_1040_8520(Struct57 *param_1, u16 param_2, u16 param_3, i16 param_4, u16 param_5, u16 param_6, u8 *param_7, u16 param_8)

{
    u32          UVar1;
    u16          uVar2;
    u16          uVar3;
    astruct_293 *iVar5;
    u16          uVar4;
    u16          uVar5;
    char        *pcVar6;
    i16          iStack22;
    i16          iStack16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfc3, param_2);
    uVar4             = (param_1 >> 0x10);
    iVar5             = (astruct_293 *)param_1;
    iVar5->field_0x8e = 0x0;
    iVar5->field_0x98 = param_3;
    iVar5->field_0x9a = 0x0;
    iVar5->field_0xb2 = 0x0;
    param_1           = 0x8ddc;
    iVar5->field_0x2  = &PTR_LOOP_1050_1040;
    iVar5->field_0x9e = 0x0;
    iVar5->field_0xa2 = 0x12c;
    iStack16          = param_4;
    if(param_4 != 0x0)
    {
        load_string_1010_84ac(_PTR_LOOP_1050_14cc, (u16)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        iVar5->field_0x94 = param_5;
        iVar5->field_0x96 = param_7;
        iStack16          = param_4 + -0x1;
    }
    iStack22 = 0x0;
    while(iStack16 != 0x0)
    {
        pcVar6   = load_string_1010_847e(_PTR_LOOP_1050_14cc, (u16)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        param_7  = (pcVar6 >> 0x10);
        uVar2    = str_op_1000_3da4(pcVar6);
        iStack22 = iStack22 + uVar2;
        iStack16 = iStack16 + -0x1;
    }
    uVar3 = iStack22 + 0x1;
    uVar5 = 0x1000;
    mem_op_1000_179c(uVar3, param_7, 0x1000);
    &iVar5->field_0x90         = uVar3;
    (&iVar5->field_0x90 + 0x2) = param_7;
    iStack16                   = param_4 + -0x1;
    if(param_4 + -0x1 != 0x0)
    {
        UVar1 = iVar5->field_0x90;
        uVar5 = 0x1010;
        load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, UVar1, (short)(UVar1 >> 0x10));
        iStack16 = param_4 + -0x2;
    }
    while(iStack16 != 0x0)
    {
        pcVar6 = load_string_1010_847e(_PTR_LOOP_1050_14cc, (u16)(_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        uVar5  = 0x1000;
        pass1_1000_3cea(iVar5->field_0x90, pcVar6);
        iStack16 = iStack16 + -0x1;
    }
    load_icon_1040_8b92(param_1, uVar5);
    globals->PTR_LOOP_1050_5df8 = 0x0;
    return iVar5;
}


Struct18 *pass1_1040_83e6(Struct18 *param_1, u8 param_2, u16 param_3)

{
    ui_cleanup_op_1040_782c(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void move_win_1040_826c(astruct_1 *param_1, u16 param_2, BOOL16 param_3)

{
    u16  IVar1;
    HWND16 unaff_CS;
    RECT16 local_e;
    i16    iStack10;
    i16    iStack8;
    u16  IStack6;
    BOOL16 BStack4;

    GetWindowRect16(unaff_CS, &local_e);
    if((param_3 == 0xffff) || (param_2 == -0x1))
    {
        IVar1   = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
        BStack4 = (IVar1 - (iStack10 - local_e.x)) / 0x2;
        IVar1   = GetSystemMetrics16((u16)s_tile2_bmp_1050_1538);
        param_2 = (IVar1 - (iStack8 - local_e.y)) / 0x2;
    }
    else
    {
        BStack4 = param_3;
    }
    IStack6 = param_2;
    MoveWindow16((HWND16)s_tile2_bmp_1050_1538, 0x0, iStack8 - local_e.y, iStack10 - local_e.x, param_2, BStack4);
    return;
}


void destroy_win_1040_8212(u32 param_1, HWND16 param_2)

{
    BOOL16 is_window;
    u16    uVar1;

    uVar1 = (param_1 >> 0x10);
    if((param_1 + 0x8c) != 0x0)
    {
        is_window = IsWindow16(param_2);
        if(is_window != 0x0)
        {
            DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
            (param_1 + 0x8c) = 0x0;
        }
    }
    return;
}


void win_ui_op_1040_81fe(HWND16 param_1)

{
    SetSysModalWindow(param_1);
    return;
}


void menu_ui_op_1040_7f86(u32 param_1, HWND16 param_2, RECT16 *param_3)

{
    HMENU16 HVar1;
    i16     iVar2;
    u16     uVar3;
    HWND16  unaff_CS;
    POi1616 local_6;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if(((iVar2 + 0x6a) != 0x0) && ((iVar2 + 0x68) == 0x0))
    {
        HVar1                      = LoadMenu16(unaff_CS,  * (iVar2 + 0x6a));
        *(HMENU16 *)(iVar2 + 0x68) = HVar1;
        if(HVar1 == 0x0)
        {
            return;
        }
        unaff_CS                   = (HWND16)s_tile2_bmp_1050_1538;
        HVar1                      = GetSubMenu16((HMENU16)s_tile2_bmp_1050_1538, 0x0);
        *(HMENU16 *)(iVar2 + 0x68) = HVar1;
        if(HVar1 == 0x0)
        {
            return;
        }
    }
    local_6.x = (u16)param_3;
    local_6.y = param_2;
    ClientToScreen16(unaff_CS, &local_6);
    TrackPopupMenu16((HMENU16)s_tile2_bmp_1050_1538, 0x0, 0x0, (iVar2 + 0x6), 0x0, local_6.y, (RECT16 *)local_6.x);
    return;
}


u16 pass1_1040_79c0(u32 *param_1, i16 *param_2, u16 param_3, u16 param_4, u16 param_5)

{
    code **ppcVar1;
    char   cVar2;
    u16    uVar3;
    u16    unaff_CS;

    if(param_5 == 0xa1)
    {
        ppcVar1 = (*param_1 + 0x38);
        uVar3   = (**ppcVar1)();
        return uVar3;
    }
    if(param_5 < 0xa2)
    {
        if(param_5 == 0x85)
        {
            ppcVar1 = (*param_1 + 0x1c);
            (**ppcVar1)();
            return 0x1;
        }
        if(param_5 < 0x86)
        {
            cVar2 = param_5;
            if(cVar2 == '\x02')
            {
                ppcVar1 = (*param_1 + 0x24);
                (**ppcVar1)();
                return 0x1;
            }
            if(cVar2 == '\f')
            {
                ppcVar1 = (*param_1 + 0x18);
                (**ppcVar1)();
                return 0x1;
            }
            if(cVar2 == '\x0f')
            {
                ppcVar1 = (*param_1 + 0x60);
                uVar3   = (**ppcVar1)();
                return uVar3;
            }
            if(cVar2 == '+')
            {
                if(*param_2 != 0x4)
                {
                    return 0x1;
                }
                win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), unaff_CS);
                return 0x1;
            }
        }
    }
    else
    {
        if(param_5 == 0x114)
        {
            ppcVar1 = (*param_1 + 0x58);
            uVar3   = (**ppcVar1)();
            return uVar3;
        }
        if(param_5 < 0x115)
        {
            if(param_5 == 0x104)
            {
                ppcVar1 = (*param_1 + 0x30);
                uVar3   = (**ppcVar1)();
                return uVar3;
            }
            if(param_5 == 0x111)
            {
                ppcVar1 = (*param_1 + 0x10);
                uVar3   = (**ppcVar1)();
                return uVar3;
            }
        }
        else
        {
            if(param_5 == 0x115)
            {
                ppcVar1 = (*param_1 + 0x54);
                uVar3   = (**ppcVar1)();
                return uVar3;
            }
            if(param_5 == 0x201)
            {
                ppcVar1 = (*param_1 + 0x44);
                (**ppcVar1)();
                return 0x1;
            }
            if(param_5 == 0x204)
            {
                ppcVar1 = (*param_1 + 0x28);
                (**ppcVar1)();
                return 0x1;
            }
        }
    }
    return 0x0;
}


void dialog_ui_fn_1040_78e2(astruct_1 *in_struct_1, HINSTANCE16 in_instance_handle)

{
    code     **ppcVar1;
    LPCSTR     dlg_template;
    HWND16     dialog_handle;
    astruct_1 *local_struct_1;
    astruct_1 *local_string_1;
    u16        uVar2;
    long       lVar3;
    u16        uVar4;
    u16        uVar5;
    u16        uVar6;
    u16        uVar7;
    u16        uVar8;
    u16        uVar9;
    u16        uVar10;
    LPCSTR     local_string_2;
    LPCSTR     pCStack8;

    local_string_1 = (astruct_1 *)(in_struct_1 >> 0x10);
    local_struct_1 = (astruct_1 *)in_struct_1;
    if(&local_struct_1->field_0xc == 0x0)
    {
        uVar2         = (_PTR_LOOP_1050_5bc8 >> 0x10);
        dlg_template  = *(LPCSTR *)(_PTR_LOOP_1050_5bc8 + 0x4);
        dialog_handle = *(HWND16 *)(_PTR_LOOP_1050_5bc8 + 0x6);
    }
    else
    {
        dlg_template  = *(LPCSTR *)&local_struct_1->field_0xc;
        dialog_handle = *(HWND16 *)&local_struct_1->field_0xe;
    }
    dialog_handle                         = CreateDialog16(in_instance_handle, dlg_template, dialog_handle, local_struct_1->lpvoid_field_0x8);
    *(HWND16 *)&local_struct_1->field_0x6 = dialog_handle;
    GetWindowText16((HWND16)s_tile2_bmp_1050_1538, 0x50, (u16)&local_struct_1->max_count_field_0x10);
    lVar3 = GetWindowLong16((HWND16)s_tile2_bmp_1050_1538, -0x4);
    SetWindowLong16((HWND16)s_tile2_bmp_1050_1538, (u16)_PTR_LOOP_1050_5bcc, CONCAT22(0xfffc, (_PTR_LOOP_1050_5bcc >> 0x10)));
    uVar2  = &local_struct_1->field_0x6;
    uVar10 = SUB42(&USHORT_1050_1050, 0x0);
    SetProp16((HWND16)s_tile2_bmp_1050_1538, local_struct_1, s_thisLo_1050_5dcd);
    uVar9 = &local_struct_1->field_0x6;
    uVar8 = SUB42(&USHORT_1050_1050, 0x0);
    SetProp16((HWND16)s_tile2_bmp_1050_1538, local_string_1, s_thisHi_1050_5dd4);
    local_string_2 = lVar3;
    uVar7          = &local_struct_1->field_0x6;
    uVar6          = SUB42(&USHORT_1050_1050, 0x0);
    SetProp16((HWND16)s_tile2_bmp_1050_1538, local_string_2, s_procLo_1050_5ddb);
    pCStack8 = (lVar3 >> 0x10);
    uVar5    = &local_struct_1->field_0x6;
    uVar4    = SUB42(&USHORT_1050_1050, 0x0);
    SetProp16((HWND16)s_tile2_bmp_1050_1538, pCStack8, s_procHi_1050_5de2);
    ppcVar1 = (in_struct_1->field_0x0 + 0x50);
    (**ppcVar1)(s_tile2_bmp_1050_1538, in_struct_1, uVar4, uVar5, uVar6, uVar7, uVar8, uVar9, uVar10, uVar2);
    return;
}


void win_cleanup_op_1040_748c(i16 param_1, u16 param_2, u16 param_3, u32 param_4)

{
    code     **ppcVar1;
    u32 uVar2;
    u8        *in_DX;
    u16        unaff_SS;
    i16        iVar3;
    RECT16     local_a;
    i16        iStack6;
    i16        iStack4;

    switch(param_4._2_2_)
    {
    case 0xfa:
        ppcVar1 = ((param_1 + 0x94) + 0x18);
        (**ppcVar1)();
        break;
    default:
        pass1_1040_b54a(param_1, param_2, param_3, param_4, in_DX, &PTR_LOOP_1050_1040, unaff_SS);
        return;
    case 0xfd:
        if(DAT_1050_0ecc == 0x0)
        {
            return;
        }
        DAT_1050_0ecc = 0x0;
        goto LAB_1040_755d;
    case 0xfe:
        if(DAT_1050_0ecc == 0x1)
        {
            return;
        }
        DAT_1050_0ecc = 0x1;
        goto LAB_1040_755d;
    case 0xff:
        if(DAT_1050_0ecc == 0x2)
        {
            return;
        }
        DAT_1050_0ecc = 0x2;
    LAB_1040_755d:
        uVar2   = (param_1 + 0x94);
        ppcVar1 = ((param_1 + 0x94) + 0x10);
        (**ppcVar1)(&PTR_LOOP_1050_1040, uVar2, (uVar2 >> 0x10));
        pass1_1010_2ee2((param_1 + 0x94), unaff_SS, 0x1010);
        PostMessage16(0x1010, 0x0, 0x0, 0x111010a);
        break;
    case 0x107:
        iVar3 = 0x0;
        goto LAB_1040_75ba;
    case 0x108:
        iVar3 = 0x1;
    LAB_1040_75ba:
        win_ui_op_1010_3202(*(param_1 + 0x94), iVar3, 0x1010);
        break;
    case 0x10a:
        GetClientRect16((HWND16)&PTR_LOOP_1050_1040, &local_a);
        uVar2     = (param_1 + 0x94);
        local_a.y = local_a.y + 0x3;
        local_a.x = (uVar2 + 0x1a) + -0x9;
        iStack6   = iStack6 + -0x3;
        iStack4   = iStack4 + -0x3;
        InvalidateRect16((HWND16)s_tile2_bmp_1050_1538, (RECT16 *)(&PTR_LOOP_1050_0000 + 0x1), (BOOL16)&local_a);
        unk_destroy_win_op_1010_2fa0(*(param_1 + 0x94), 0x1010);
        pass1_1010_32c0(*(param_1 + 0x94), 0x0);
        pass1_1010_2ee2((param_1 + 0x94), unaff_SS, 0x1010);
        break;
    case 0x10c:
        DestroyWindow16((HWND16)&PTR_LOOP_1050_1040);
    }
    return;
}


void msg_box_ui_op_1040_64ca(u32 param_1, char *param_2, u8 *param_3, u16 param_4)

{
    char local_206[0x102];
    char local_104[0x102];

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x100, local_206, param_4);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, local_104, param_4);
    pass1_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    fn_ptr_1000_17ce((Struct18 *)CONCAT22(param_3, param_2), 0x1000);
    return;
}


void show_win_1040_65ba(astruct_1 *param_1, u16 param_2)

{
    u32   uVar1;
    u16          uVar2;
    astruct_160 *rect;
    u8          *in_DX;
    u8          *puVar3;
    i16          iVar4;
    i16          iVar5;
    i16          unaff_DI;
    u16          uVar6;
    u16          uVar7;
    HWND16       hwnd;
    u16          unaff_SS;
    u16          local_22;
    u16          uStack32;
    u16          uStack30;
    u16          uStack28;
    u16         *puStack26;
    i16          iStack10;
    u16          uStack8;
    u16         *puStack6;

    dialog_ui_fn_1040_78e2(param_1, param_2);
    puStack6                    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, unaff_SS, in_DX, unaff_DI);
    globals->PTR_LOOP_1050_5f2e = (puStack6 >> 0x10);
    uStack8                     = pass1_1010_0898();
    if(_PTR_LOOP_1050_5f2c == 0x0)
    {
        globals->PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e, 0x1000);
    }
    else
    {
    }
    puStack26      = CONCAT22(PTR_LOOP_1050_5f2e, globals->PTR_LOOP_1050_5f2c);
    hwnd           = 0x1000;
    uVar2          = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4, 0x0, 0x1, globals->PTR_LOOP_1050_5f2c, globals->PTR_LOOP_1050_5f2e, 0x1000);
    uVar6          = (param_1 >> 0x10);
    iVar4          = param_1;
    (iVar4 + 0x8e) = uVar2;
    (iVar4 + 0x90) = globals->PTR_LOOP_1050_5f2e;
    for(iStack10 = 0x1; iStack10 <= uStack8; iStack10 = iStack10 + 0x1)
    {
        puStack26 = pass1_1010_0946(puStack6, (puStack6 >> 0x10), iStack10, globals->PTR_LOOP_1050_5f2e, unaff_DI, unaff_SS);
        puVar3    = (puStack26 >> 0x10);
        local_22  = *puStack26;
        uStack32  = (puStack26 + 0x2);
        uStack30  = 0x1;
        uStack28  = 0x1;
        rect      = (astruct_160 *)&local_22;
        MapDialogRect16(0x1010, (RECT16 *)rect);
        hwnd = 0x1000;
        mem_op_1000_179c(0x42, puVar3, 0x1000);
        globals->PTR_LOOP_1050_5f2e = (puVar3 | rect);
        if(PTR_LOOP_1050_5f2e == 0x0)
        {
            uVar1                    = (iVar4 + 0x8e);
            (uVar1 + iStack10 * 0x4) = 0x0;
        }
        else
        {
            hwnd = 0x1008;
            pass1_1008_3bd6(rect, puVar3, 0x0, CONCAT22(local_22, uStack32), 0x101, 0xff0100, CONCAT22((iVar4 + 0x6), (puStack26 + 0x4)), globals->PTR_LOOP_1050_5f2e, unaff_SS);
            uVar1                                     = (iVar4 + 0x8e);
            uVar7                                     = (uVar1 >> 0x10);
            iVar5                                     = uVar1;
            *(astruct_160 **)(iVar5 + iStack10 * 0x4) = rect;
            (iVar5 + iStack10 * 0x4 + 0x2)            = globals->PTR_LOOP_1050_5f2e;
        }
        uVar1 = (iVar4 + 0x8e);
        uVar7 = (uVar1 >> 0x10);
        iVar5 = uVar1;
        if((iVar5 + iStack10 * 0x4) != 0x0)
        {
            unaff_DI = puStack26;
            enable_win_1040_9234(*(iVar5 + iStack10 * 0x4), *(BOOL16 *)(unaff_DI + 0x6), hwnd);
        }
    }
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(hwnd, 0x5);
    return;
}


void post_win_msg_1040_672e(i16 param_1, u16 param_2, u16 param_3, u32 param_4, u16 param_5, u16 param_6)

{
    u16 unaff_CS;
    i16 iVar1;

    if(param_4._2_2_ == s_vrpal_bmp_1050_183a + 0x7)
    {
        msg_box_ui_op_1040_64ca(CONCAT22(param_2, param_1), 0x0, param_5, param_6);
    }
    else
    {
        if(param_4._2_2_ == 0x1851)
        {
            iVar1 = 0x2a;
        }
        else
        {
            if(param_4._2_2_ != 0x1852)
            {
                post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, param_4._2_2_, unaff_CS);
                return;
            }
            iVar1 = 0x29;
        }
        pass1_1038_af40(_PTR_LOOP_1050_5b7c, (param_1 + 0x8), iVar1, param_5, param_1, &PTR_LOOP_1050_1038, param_6);
        PostMessage16((HWND16)&PTR_LOOP_1050_1038, 0x0, 0x0, 0x1110002);
    }
    return;
}


void enable_win_1040_6880(u32 param_1, i16 param_2, HWND16 param_3)

{
    u32 uVar1;
    u16        uVar2;

    if(param_2 == 0x8)
    {
        uVar2 = (param_1 >> 0x10);
        GetDlgItem16(param_3, 0x107);
        uVar1 = (param_1 + 0x94);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, *(BOOL16 *)(uVar1 + 0x24));
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x108);
        uVar1 = (param_1 + 0x94);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, *(BOOL16 *)(uVar1 + 0x26));
    }
    return;
}


void mixed_win_ui_op_1040_6942(astruct_1 *param_1, u16 param_2, u16 param_3)

{
    u32          uVar1;
    u32   uVar2;
    code       **ppcVar3;
    astruct_160 *paVar4;
    LPCSTR       pCVar5;
    u32  *puVar6;
    i16          iVar7;
    u8          *in_DX;
    u8          *extraout_DX;
    u8          *puVar8;
    u8          *puVar9;
    u16          uVar10;
    i16          iVar11;
    i16          unaff_DI;
    u16          uVar12;
    u16          uVar13;
    u16          uVar14;
    HWND16       hwnd;
    u16         *puVar15;
    DWORD        DVar16;
    char        *pcVar17;
    BOOL16       BVar18;
    u32   local_64;
    u32   uStack96;
    u16        IStack92;
    u16        IStack90;
    char         local_58[0x50];
    HDC16        HStack8;
    astruct_160 *paStack6;
    u8          *puStack4;

    dialog_ui_fn_1040_78e2(param_1, param_3);
    puVar15                          = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x33, param_2, in_DX, unaff_DI);
    uVar14                           = (puVar15 >> 0x10);
    paVar4                           = (astruct_160 *)puVar15;
    uVar12                           = (param_1 >> 0x10);
    iVar11                           = param_1;
    *(astruct_160 **)(iVar11 + 0x94) = paVar4;
    (iVar11 + 0x96)                  = uVar14;
    ppcVar3                          = ((iVar11 + 0x94) + 0x4);
    (**ppcVar3)(0x1010, (iVar11 + 0x94), uVar14, 0x0, param_1);
    puVar9 = extraout_DX;
    mem_op_1000_179c(0xa, extraout_DX, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if(puVar8 == 0x0)
    {
        (iVar11 + 0x98) = 0x0;
    }
    else
    {
        puVar15                          = struct_1040_bf3e(CONCAT22(puVar9, paVar4), (iVar11 + 0x6));
        puVar8                           = (puVar15 >> 0x10);
        paVar4                           = (astruct_160 *)puVar15;
        *(astruct_160 **)(iVar11 + 0x98) = paVar4;
        (iVar11 + 0x9a)                  = puVar8;
    }
    pass1_1040_bfde(*(iVar11 + 0x98), (iVar11 + 0x94), param_2);
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if(puVar9 != 0x0)
    {
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa000a, 0x0, 0x800081, CONCAT22((iVar11 + 0x6), 0x10a), puVar9, param_2);
    }
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if(puVar8 != 0x0)
    {
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa0028, 0x0, 0x820083, CONCAT22((iVar11 + 0x6), 0x10c), puVar8, param_2);
    }
    BVar18 = 0x42;
    uVar14 = 0x1000;
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if(puVar9 == 0x0)
    {
        paVar4 = (astruct_160 *)0x0;
        puVar9 = 0x0;
    }
    else
    {
        uVar14 = 0x1008;
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa00aa, 0x101, 0xff0100, CONCAT22((iVar11 + 0x6), 0x107), puVar9, param_2);
    }
    paStack6 = paVar4;
    puStack4 = puVar9;
    enable_win_1040_9234(CONCAT22(puVar9, paVar4), BVar18, uVar14);
    BVar18 = 0x42;
    hwnd   = 0x1000;
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    uVar10 = puVar9 | paVar4;
    if(uVar10 == 0x0)
    {
        paVar4 = (astruct_160 *)0x0;
        uVar10 = 0x0;
    }
    else
    {
        hwnd = 0x1008;
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa00c2, 0x101, 0xff0100, CONCAT22((iVar11 + 0x6), 0x108), uVar10, param_2);
    }
    paStack6 = paVar4;
    puStack4 = uVar10;
    enable_win_1040_9234(CONCAT22(uVar10, paVar4), BVar18, hwnd);
    HStack8 = GetDC16(hwnd);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x50, local_58, param_2);
    pcVar17  = local_58;
    pCVar5   = str_op_1000_3da4(CONCAT22(param_2, pcVar17));
    DVar16   = GetTextExtent16(0x1000, pCVar5, (u16)pcVar17);
    IStack90 = (u16)(DVar16 >> 0x10);
    IStack92 = (u16)DVar16;
    CreateWindow16(s_tile2_bmp_1050_1538, 0x0, ZEXT24(globals->PTR_LOOP_1050_038c) << 0x10, 0x7cd, (iVar11 + 0x6), IStack90, IStack92, 0xad, 0x22, 0x0, (s_Rebel_1050_4ffc + 0x4));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x50, local_58, param_2);
    pcVar17  = local_58;
    pCVar5   = str_op_1000_3da4(CONCAT22(param_2, pcVar17));
    DVar16   = GetTextExtent16(0x1000, pCVar5, (u16)pcVar17);
    IStack90 = (u16)(DVar16 >> 0x10);
    IStack92 = (u16)DVar16;
    ReleaseDC16((HWND16)s_tile2_bmp_1050_1538, HStack8);
    CreateWindow16(s_tile2_bmp_1050_1538, 0x0, ZEXT24(globals->PTR_LOOP_1050_038c) << 0x10, 0x7ce, (iVar11 + 0x6), IStack90, IStack92, 0xc5, 0x22, 0x0, (s_Rebel_1050_4ffc + 0x4));
    local_64 = 0x5a000a;
    uStack96 = 0x140050;
    puVar6   = &local_64;
    create_window_1040_6eae();
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_window_1040_6eae();
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_window_1040_6eae();
    SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4010001);
    uVar1  = *(iVar11 + 0x94);
    iVar7  = uVar1;
    uVar1  = uVar1 & 0xffff0000;
    uVar14 = (iVar11 + 0x6);
    uVar13 = (uVar1 >> 0x10);
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538, 0x40, (iVar7 + 0x10), (iVar7 + 0xe), (iVar7 + 0xc), (uVar1 | iVar7 + 0xa), 0x0);
    DAT_1050_0ecc = 0x0;
    uVar2         = (iVar11 + 0x94);
    ppcVar3       = ((iVar11 + 0x94) + 0x10);
    (**ppcVar3)(s_tile2_bmp_1050_1538, uVar2, (uVar2 >> 0x10), uVar14, puVar6);
    pass1_1010_2ee2((iVar11 + 0x94), param_2, 0x1010);
    PostMessage16(0x1010, 0x0, 0x0, 0x111010a);
    return;
}


void enable_win_1040_6ff2(u32 param_1, i16 param_2, HWND16 param_3)

{
    u32 uVar1;
    u16        uVar2;

    if(param_2 == 0x8)
    {
        uVar2 = (param_1 >> 0x10);
        GetDlgItem16(param_3, 0x107);
        uVar1 = (param_1 + 0x94);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, *(BOOL16 *)(uVar1 + 0x24));
        GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x108);
        uVar1 = (param_1 + 0x94);
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538, *(BOOL16 *)(uVar1 + 0x26));
    }
    return;
}


u16 pass1_1040_7044(u32 *param_1, i16 *param_2, u16 param_3, u16 param_4, i16 param_5, u16 param_6)

{
    fn_ptr_1 *ppcVar1;
    u16       uVar2;

    if(param_5 == 0x2b)
    {
        if(*param_2 == 0x4)
        {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2), param_6);
        }
    }
    else
    {
        if(param_5 != 0x111)
        {
            uVar2 = pass1_1040_b316(param_1, param_2, param_3, param_4, param_5);
            return uVar2;
        }
        ppcVar1 = (*param_1 + 0x80);
        (**ppcVar1)(param_6, param_1, param_2, CONCAT22(param_4, param_3));
    }
    return 0x1;
}


void mixed_win_ui_op_1040_70b4(astruct_1 *param_1, u16 param_2, u16 param_3)

{
    u32          uVar1;
    u32   uVar2;
    code       **ppcVar3;
    astruct_160 *paVar4;
    LPCSTR       pCVar5;
    u32  *puVar6;
    i16          iVar7;
    u8          *in_DX;
    u8          *extraout_DX;
    u8          *puVar8;
    u8          *puVar9;
    u16          uVar10;
    i16          iVar11;
    i16          unaff_DI;
    u16          uVar12;
    u16          uVar13;
    u16          uVar14;
    HWND16       hwnd;
    u16         *puVar15;
    DWORD        DVar16;
    char        *pcVar17;
    BOOL16       BVar18;
    u32   local_64;
    u32   uStack96;
    u16        IStack92;
    u16        IStack90;
    char         local_58[0x50];
    HDC16        HStack8;
    astruct_160 *paStack6;
    u8          *puStack4;

    dialog_ui_fn_1040_78e2(param_1, param_2);
    puVar15                          = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x34, param_3, in_DX, unaff_DI);
    uVar14                           = (puVar15 >> 0x10);
    paVar4                           = (astruct_160 *)puVar15;
    uVar12                           = (param_1 >> 0x10);
    iVar11                           = param_1;
    *(astruct_160 **)(iVar11 + 0x94) = paVar4;
    (iVar11 + 0x96)                  = uVar14;
    ppcVar3                          = ((iVar11 + 0x94) + 0x4);
    (**ppcVar3)(0x1010, (iVar11 + 0x94), uVar14, 0x0, param_1);
    puVar9 = extraout_DX;
    mem_op_1000_179c(0xa, extraout_DX, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if(puVar8 == 0x0)
    {
        (iVar11 + 0x98) = 0x0;
    }
    else
    {
        puVar15                          = struct_1040_bf3e(CONCAT22(puVar9, paVar4), (iVar11 + 0x6));
        puVar8                           = (puVar15 >> 0x10);
        paVar4                           = (astruct_160 *)puVar15;
        *(astruct_160 **)(iVar11 + 0x98) = paVar4;
        (iVar11 + 0x9a)                  = puVar8;
    }
    pass1_1040_bfde(*(iVar11 + 0x98), (iVar11 + 0x94), param_3);
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if(puVar9 != 0x0)
    {
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa000a, 0x0, 0x800081, CONCAT22((iVar11 + 0x6), 0x10a), puVar9, param_3);
    }
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if(puVar8 != 0x0)
    {
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa0028, 0x0, 0x820083, CONCAT22((iVar11 + 0x6), 0x10c), puVar8, param_3);
    }
    BVar18 = 0x42;
    uVar14 = 0x1000;
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if(puVar9 == 0x0)
    {
        paVar4 = (astruct_160 *)0x0;
        puVar9 = 0x0;
    }
    else
    {
        uVar14 = 0x1008;
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa00aa, 0x101, 0xff0100, CONCAT22((iVar11 + 0x6), 0x107), puVar9, param_3);
    }
    paStack6 = paVar4;
    puStack4 = puVar9;
    enable_win_1040_9234(CONCAT22(puVar9, paVar4), BVar18, uVar14);
    BVar18 = 0x42;
    hwnd   = 0x1000;
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    uVar10 = puVar9 | paVar4;
    if(uVar10 == 0x0)
    {
        paVar4 = (astruct_160 *)0x0;
        uVar10 = 0x0;
    }
    else
    {
        hwnd = 0x1008;
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa00c2, 0x101, 0xff0100, CONCAT22((iVar11 + 0x6), 0x108), uVar10, param_3);
    }
    paStack6 = paVar4;
    puStack4 = uVar10;
    enable_win_1040_9234(CONCAT22(uVar10, paVar4), BVar18, hwnd);
    HStack8 = GetDC16(hwnd);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x50, local_58, param_3);
    pcVar17  = local_58;
    pCVar5   = str_op_1000_3da4(CONCAT22(param_3, pcVar17));
    DVar16   = GetTextExtent16(0x1000, pCVar5, (u16)pcVar17);
    IStack90 = (u16)(DVar16 >> 0x10);
    IStack92 = (u16)DVar16;
    CreateWindow16(s_tile2_bmp_1050_1538, 0x0, ZEXT24(globals->PTR_LOOP_1050_038c) << 0x10, 0x7cd, (iVar11 + 0x6), IStack90, IStack92, 0xad, 0x22, 0x0, (s_Rebel_1050_4ffc + 0x4));
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x50, local_58, param_3);
    pcVar17  = local_58;
    pCVar5   = str_op_1000_3da4(CONCAT22(param_3, pcVar17));
    DVar16   = GetTextExtent16(0x1000, pCVar5, (u16)pcVar17);
    IStack90 = (u16)(DVar16 >> 0x10);
    IStack92 = (u16)DVar16;
    ReleaseDC16((HWND16)s_tile2_bmp_1050_1538, HStack8);
    CreateWindow16(s_tile2_bmp_1050_1538, 0x0, ZEXT24(globals->PTR_LOOP_1050_038c) << 0x10, 0x7ce, (iVar11 + 0x6), IStack90, IStack92, 0xc5, 0x22, 0x0, (s_Rebel_1050_4ffc + 0x4));
    local_64 = 0x5a000a;
    uStack96 = 0x140050;
    puVar6   = &local_64;
    create_window_1040_7620();
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_window_1040_7620();
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_window_1040_7620();
    SendMessage16((HWND16)s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4010001);
    uVar1  = *(iVar11 + 0x94);
    iVar7  = uVar1;
    uVar1  = uVar1 & 0xffff0000;
    uVar14 = (iVar11 + 0x6);
    uVar13 = (uVar1 >> 0x10);
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538, 0x40, (iVar7 + 0x10), (iVar7 + 0xe), (iVar7 + 0xc), (uVar1 | iVar7 + 0xa), 0x0);
    DAT_1050_0ecc = 0x0;
    uVar2         = (iVar11 + 0x94);
    ppcVar3       = ((iVar11 + 0x94) + 0x10);
    (**ppcVar3)(s_tile2_bmp_1050_1538, uVar2, (uVar2 >> 0x10), uVar14, puVar6);
    pass1_1010_2ee2((iVar11 + 0x94), param_3, 0x1010);
    PostMessage16(0x1010, 0x0, 0x0, 0x111010a);
    return;
}


void pass1_1040_57d4(astruct_1 *param_1, u8 *param_2, i16 param_3, u16 param_4, u16 param_5)

{
    pass1_1040_5d42(param_1);
    pass1_1040_5eaa(param_1);
    pass1_1040_5dc4(param_1, param_2, param_3, param_5);
    unk_win_ui_op_1040_b230(param_1, param_4, param_5);
    return;
}
