

void pass1_1010_0f76(u16 *param_1, u16 param_2)

{
    u16 uVar1;

    uVar1           = (param_1 >> 0x10);
    *param_1        = s_648_bmp_1050_1919 + 0x1;
    (param_1 + 0x2) = 0x1010;
    pass1_1010_17c0(param_1 & 0xffff | uVar1 << 0x10);
    pass1_1010_2db2(param_1, param_2);
    return;
}

void pass1_1010_1146(u32 param_1, u16 param_2, i16 param_3, u16 param_4)

{
    u32 uVar1;
    u16        uVar2;

    DAT_1050_0ecc = param_2;
    uVar2         = (param_1 >> 0x10);
    uVar1         = (param_1 + 0x66);
    pass1_1000_4aea(*(param_1 + 0x64), uVar1, (uVar1 >> 0x10), 0x4, (s_dibtext_bmp_1050_1844 + 0x6), &stack0xfffe, param_3, uVar2, 0x1000, param_4);
    return;
}

void pass1_1010_116c(u32 *param_1, i16 param_2, u16 param_3)

{
    code     **ppcVar1;
    i16        iVar2;
    u16        uVar3;
    i16        iVar4;
    u16        uVar5;
    u32 uVar6;
    u16        uStack4;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if((iVar4 + 0x56) != 0x0)
    {
        ppcVar1 = (*param_1 + 0x34);
        (**ppcVar1)();
    }
    ppcVar1 = (*param_1 + 0x28);
    uVar6   = (**ppcVar1)();
    uVar3   = (uVar6 >> 0x10);
    if(uVar6 != 0x0)
    {
        uStack4 = DAT_1050_0ecc;
        iVar2   = DAT_1050_0ecc + 0x1;
        if(iVar2 == 0x0)
        {
            uStack4 = 0x0;
        }
        pass1_1010_1146(param_1, uStack4, param_2, param_3);
        pass1_1010_11c6(param_1, iVar2, uVar3);
        (iVar4 + 0x56) = iVar2;
        (iVar4 + 0x58) = uVar3;
    }
    return;
}

void pass1_1008_e852(u16 param_1, u16 param_2, u32 param_3, u16 param_4, u16 param_5)

{
    u8   *puVar1;
    i16   iVar2;
    char *pcVar3;
    u8    local_14[0x12];

    pass1_1028_dc52((astruct_92 *)CONCAT22(param_4, local_14), 0x1, 0x0, 0x400);
    do
    {
        puVar1 = local_14;
        pass1_1028_e4ec(CONCAT22(param_4, puVar1));
        if((param_5 | puVar1) == 0x0)
        {
            return;
        }
        pcVar3  = pass1_1038_4d28(CONCAT22(param_5, puVar1));
        param_5 = (pcVar3 >> 0x10);
        iVar2   = pass1_1000_3d7a(param_3, pcVar3 & 0xffff | param_5 << 0x10);
    } while(iVar2 != 0x0);
    return;
}

long pass1_1008_e8cc(u16 param_1, u32 param_2, u32 param_3, u32 param_4)

{
    u32 uVar1;
    u16        uVar2;
    u16        uVar3;
    i16        iVar4;
    u16        uVar5;
    u16        uVar6;
    long       lVar7;
    char      *pcVar8;
    char      *pcVar9;
    u32        uStack22;
    u32        uStack18;
    u8         local_a[0x8];

    pass1_1008_5784(CONCAT22(param_1, local_a), *(param_2 + 0xa));
    while(true)
    {
        lVar7 = pass1_1008_5b12(local_a, param_1);
        uVar5 = (lVar7 >> 0x10);
        uVar2 = lVar7;
        uVar6 = uVar5 | uVar2;
        if(lVar7 == 0x0)
        {
            return 0x0;
        }
        uVar1 = (uVar2 + 0x4);
        uVar3 = uVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        uStack18 = CONCAT22(uVar6, uVar3);
        uVar1    = (uVar2 + 0x8);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
        uStack22 = CONCAT22(uVar6, uVar3);
        pcVar8   = pass1_1038_4d28(uStack18);
        pcVar9   = pass1_1038_4d28(uStack22);
        iVar4    = pass1_1000_3d7a(param_4, pcVar8);
        if((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_3, pcVar9), iVar4 == 0x0))
            break;
        iVar4 = pass1_1000_3d7a(param_3, pcVar8);
        if((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_4, pcVar9), iVar4 == 0x0))
        {
            return lVar7;
        }
    }
    return lVar7;
}


u32 pass1_1008_eb5c(u16 param_1, u16 param_2, i16 param_3)

{
    return CONCAT22(0x1050, param_3 * 0x10 + 0xd0e);
}


u16 pass1_1008_eb6e(void)

{
    return 0x5;
}

void pass1_1008_ec94(u16 *param_1)

{
    *param_1        = 0xefc4;
    (param_1 + 0x2) = 0x1008;
    pass1_1010_3880(param_1);
    return;
}

void pass1_1008_ed00(u16 *param_1, u16 param_2)

{
    *param_1        = 0xef9c;
    (param_1 + 0x2) = 0x1008;
    pass1_1010_2db2(param_1, param_2);
    return;
}


void pass1_1008_ed62(u32 param_1, i16 param_2)

{
    i16 iVar1;
    u16 uVar2;

    uVar2          = (param_1 >> 0x10);
    iVar1          = param_1;
    (iVar1 + 0x16) = param_2 * 0x8 + 0xd5e;
    (iVar1 + 0x18) = &USHORT_1050_1050;
    (iVar1 + 0x12) = (param_2 * 0x8 + 0xd64);
    return;
}

void pass1_1008_ee72(u16 param_1, u16 param_2, i16 param_3)

{
    fn_ptr_1 *ppcVar1;
    u32       uVar2;

    if((param_1 + 0x56) == 0x0)
    {
        ppcVar1 = (CONCAT22(param_2, param_1) + 0x10);
        (**ppcVar1)();
    }
    uVar2 = pass1_1010_2e02(CONCAT22(param_2, param_1), param_3);
    pass1_1010_2e5c(param_1, param_2, uVar2);
    return;
}


u16 pass1_1008_eea6(void)

{
    return 0x0;
}

bool pass1_1008_eeac(u16 param_1, u16 param_2, u32 param_3, u8 *param_4, i16 param_5, u16 param_6)

{
    u16  uVar1;
    char cVar2;
    u16  uVar3;
    u16  uVar4;
    u16  uVar5;
    u16 *puVar6;
    u16  uVar7;

    uVar7  = *(param_3 + 0x12);
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, param_4, param_5);
    uVar4  = (puVar6 >> 0x10);
    uVar1  = puVar6;
    uVar5  = uVar4;
    if(uVar7 == 0x7d)
    {
        pass1_1010_a5ca(uVar1, uVar4, 0x7c, 0x7d, uVar4);
        if(uVar7 != 0x0)
        {
            return false;
        }
        pass1_1010_a5ca(uVar1, uVar4, 0x7d, 0x0, uVar5);
        if(uVar7 != 0x0)
        {
            return false;
        }
        uVar3 = uVar7;
        uVar7 = 0x78;
    }
    else
    {
        uVar3 = uVar7;
        if(uVar7 < 0x7e)
        {
            cVar2 = uVar7;
            uVar3 = uVar7 & 0xff00;
            if((u8)(cVar2 + 0x8dU) == 0x0)
            {
                uVar7 = 0x9;
                uVar3 = uVar3 | (u8)(cVar2 + 0x8dU);
            }
            else
            {
                if((u8)(cVar2 + 0x89U) == 0x0)
                {
                    uVar7 = 0x2e;
                    uVar3 = uVar3 | (u8)(cVar2 + 0x89U);
                }
                else
                {
                    uVar3 = uVar3 | (u8)(cVar2 + 0x87U);
                    if((u8)(cVar2 + 0x87U) == 0x0)
                    {
                        uVar7 = 0x5b;
                    }
                }
            }
        }
    }
    pass1_1010_a5ca(uVar1, uVar4, uVar7, uVar3, uVar5);
    return uVar3 == 0x0;
}


u16 pass1_1008_ef38(u32 param_1)

{
    u32 uVar1;

    uVar1 = (param_1 + 0x16);
    return (uVar1 + 0x2);
}


u16 pass1_1008_ef4a(void)

{
    return 0x41;
}

u32 pass1_1010_0000(astruct_645 *param_1, u16 param_2, u16 param_3, u16 param_4)

{
    i16         unaff_DI;
    astruct_79 *paVar1;
    u16        *puVar2;
    u16        *puVar3;
    u16         uVar4;
    u16        *puVar5;
    u16         uVar6;

    // Segment:    3
    // Offset:     00015420
    // Length:     ee9f
    // Min Alloc:  ee9f
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    paVar1                     = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2, param_1), param_3);
    param_1->field_0xa         = 0x0;
    param_1->field_0xc         = 0x0;
    CONCAT22(param_2, param_1) = 0x2c8;
    param_1->field_0x2         = 0x1010;
    puVar5                     = &param_1->field_0xa;
    puVar3                     = &param_1->field_0xc;
    uVar4                      = param_2;
    uVar6                      = param_2;
    puVar2                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x48, param_4, (paVar1 >> 0x10), unaff_DI);
    pass1_1008_3e94((puVar2 & 0xffff0000 | (puVar2 + 0xe)), CONCAT22(uVar4, puVar3), CONCAT22(uVar6, puVar5));
    return CONCAT22(param_2, param_1);
}


void pass1_1010_0052(u16 *param_1, u16 param_2)

{
    *param_1        = 0x2c8;
    (param_1 + 0x2) = 0x1010;
    pass1_1010_1d80(param_1, param_2);
    return;
}

void pass1_1010_01f8(u32 param_1, u32 param_2, i16 param_3)

{
    i16 iVar1;
    i16 iVar2;
    u16 uVar3;
    u16 uVar4;

    iVar2           = (param_3 * 0x4 + 0xe02) * 0x4;
    iVar1           = (iVar2 + 0xdfc);
    uVar3           = (param_1 >> 0x10);
    uVar4           = (param_2 >> 0x10);
    (param_2 + 0x6) = (param_3 * 0x4 + 0xe04) * 0x28 + (iVar2 + 0xdfa) + (param_1 + 0xa);
    (param_2 + 0x8) = (param_1 + 0xc) + iVar1;
    return;
}

void pass1_1010_0350(u16 *param_1, u16 param_2)

{
    u32  *puVar1;
    u16          uVar2;
    code       **ppcVar3;
    astruct_474 *iVar4;
    u16          uVar4;

    uVar4            = (param_1 >> 0x10);
    iVar4            = (astruct_474 *)param_1;
    *param_1         = 0xe98;
    iVar4->field_0x2 = 0x1010;
    puVar1           = iVar4->field_0xa;
    uVar2            = iVar4->field_0xc;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}

void pass1_1008_d7da(u16 *param_1, u16 param_2)

{
    u32 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    i16         iVar4;
    u16         uVar5;

    uVar5         = (param_1 >> 0x10);
    iVar4         = param_1;
    *param_1      = 0xd98e;
    (iVar4 + 0x2) = 0x1008;
    puVar1        = *(iVar4 + 0xa);
    uVar2         = *(iVar4 + 0xc);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}


void  pass1_1008_dc80(u16 param_1, u16 *param_2, u32 param_3, u32 param_4, u16 param_5, u8 param_6, i16 param_7, i16 param_8, u8 param_9, u16 param_10)

{
    char *pcVar1;
    u16   uVar2;
    u16   uVar3;
    code *pcVar4;
    u16   uVar5;
    char  cVar6;
    char  extraout_DL;
    u8  bVar7;
    i16   iVar8;
    u16   uVar9;
    u8  bVar10;

    bVar7  = (u8)(param_10 >> 0x8);
    bVar10 = (u8)param_10 + bVar7;
    cVar6  = bVar10 + param_9;
    uVar2  = (CARRY1((u8)param_10, bVar7) || CARRY1(bVar10, param_9));
    uVar3  = param_5 + 0xeff0;
    bVar10 = param_5 < 0x1010 || uVar3 < uVar2;
    uVar5  = uVar3 - uVar2;
    pcVar4 = (fn_ptr_1)swi(0x4);
    if(SBORROW2(param_5, 0x1010) != SBORROW2(uVar3, uVar2))
    {
        (*pcVar4)();
        cVar6 = extraout_DL;
    }
    pcVar1         = (param_7 + param_8);
    *pcVar1        = *pcVar1 + cVar6 + (uVar5 < 0x1010 || uVar5 + 0xeff0 < bVar10);
    uVar9          = (param_2 >> 0x10);
    iVar8          = param_2;
    *param_2       = 0x389a;
    (iVar8 + 0x2)  = 0x1008;
    *(iVar8 + 0x4) = param_4;
    *(iVar8 + 0x8) = param_3;
    (iVar8 + 0xc)  = 0x0;
    (iVar8 + 0xe)  = 0x0;
    (iVar8 + 0x12) = 0x0;
    *param_2       = 0xdd4a;
    (iVar8 + 0x2)  = 0x1008;
    return;
}


void pass1_1008_df4a(u32 param_1, i16 param_2, u16 param_3)

{
    u16 uVar1;
    u16 uVar2;
    u32 uVar3;
    u8  local_a[0x8];

    uVar2 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(param_3, local_a), *(param_1 + 0xa));
    while(true)
    {
        uVar3 = pass1_1008_5b12(local_a, param_3);
        uVar1 = (uVar3 >> 0x10);
        if(uVar3 == 0x0)
            break;
        if(((uVar3 + 0xc) == 0x2) || ((uVar3 + 0xc) == 0x3))
        {
            pass1_1008_e9a4(param_1, uVar2, uVar3, param_2, param_3);
        }
    }
    return;
}

void pass1_1008_dfa6(u32 param_1, long param_2, long param_3, u16 param_4)

{
    u8 *puVar1;
    u16 extraout_DX;
    u8  local_a[0x8];

    pass1_1008_5784(CONCAT22(param_4, local_a), *(param_1 + 0xa));
    do
    {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, param_4);
        if((extraout_DX | puVar1) == 0x0)
        {
            return;
        }
    } while((((puVar1 + 0x4) != param_3) || ((puVar1 + 0x8) != param_2)) && (((puVar1 + 0x8) != param_3 || ((puVar1 + 0x4) != param_2))));
    if((puVar1 + 0xc) != 0x1)
    {
        return;
    }
    return;
}


void pass1_1008_e01c(u32 param_1, u32 param_2, u32 param_3)

{
    u16 uVar1;

    uVar1             = (param_1 >> 0x10);
    *(param_1 + 0x16) = param_3;
    *(param_1 + 0x1a) = param_2;
    return;
}


void pass1_1008_e038(u32 param_1, u32 *param_2, u32 *param_3)

{
    u16 uVar1;

    uVar1    = (param_1 >> 0x10);
    *param_3 = *(param_1 + 0x16);
    *param_2 = *(param_1 + 0x1a);
    return;
}


i16 pass1_1008_e10c(u32 param_1, u32 param_2, u32 param_3, i16 param_4, u16 param_5)

{
    i16 iVar1;
    i16 iVar2;
    u32 uVar3;

    uVar3 = pass1_1008_e8cc(param_5, param_1, param_2, param_3);
    if(uVar3 == 0x0)
    {
        return 0x1;
    }
    iVar1 = (uVar3 + 0xc);
    if(-0x1 < iVar1)
    {
        if(iVar1 < 0x2)
        {
            return 0x1;
        }
        if((0x0 < iVar1 + -0x1) && (iVar2 = iVar1 + -0x3, iVar2 == 0x0 || iVar1 + -0x2 < 0x1))
        {
            pass1_1008_e9a4(param_1, (param_1 >> 0x10), uVar3, param_4, param_5);
            return iVar2;
        }
    }
    return 0x0;
}


BOOL16 pass1_1008_c6ae(u32 param_1, i16 param_2, i16 param_3)

{
    i16 *piVar1;
    u32 *puVar2;
    i16  iStack8;

    puVar2  = pass1_1008_c6fa(param_1, param_3);
    iStack8 = 0x0;
    while(true)
    {
        piVar1 = (puVar2 + 0x4);
        if(*piVar1 == iStack8 || *piVar1 < iStack8)
        {
            return 0x0;
        }
        if((*puVar2 + iStack8 * 0x2) == param_2)
            break;
        iStack8 = iStack8 + 0x1;
    }
    return 0x1;
}


u32 *pass1_1008_c6fa(i16 *param_1, i16 param_2)

{
    if((0x0 < param_2) && (param_2 < 0x47))
    {
        return CONCAT22((param_1 + 0x2), param_2 * 0x6 + *param_1);
    }
    return 0x0;
}


void pass1_1008_c75c(u16 *param_1, u16 param_2)

{
    u32  *puVar1;
    u16          uVar2;
    code       **ppcVar3;
    astruct_469 *iVar4;
    u16          uVar4;

    uVar4            = (param_1 >> 0x10);
    iVar4            = (astruct_469 *)param_1;
    *param_1         = 0xca4a;
    iVar4->field_0x2 = 0x1008;
    puVar1           = iVar4->field_0xa;
    uVar2            = iVar4->field_0xc;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1008_c79a(u32 param_1, u32 param_2, i16 param_3, u16 param_4, u8 param_5)

{
    u8        *puVar1;
    i16        iVar2;
    u32        uVar3;
    u16        extraout_DX;
    u8        *puVar4;
    u16        uVar5;
    u16        uVar6;
    u16       *puVar7;
    u8         local_12[0x4];
    u32 uStack14;
    u8         local_a[0x8];

    uVar6 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(param_4, local_a), *(param_1 + 0xa));
    while(true)
    {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, param_4);
        uStack14 = CONCAT22(extraout_DX, puVar1);
        puVar4   = (extraout_DX | puVar1);
        if(puVar4 == 0x0)
            break;
        iVar2 = pass1_1000_3d7a(*(puVar1 + 0x4), param_2);
        if(iVar2 == 0x0)
        {
            puVar7 = pass1_1020_a43e(param_4, puVar4, CONCAT22(param_4, local_12));
            uVar5  = (puVar7 >> 0x10);
            pass1_1020_a6ee(CONCAT22(param_4, local_12), (uStack14 + 0x12), local_12, uVar5, param_3, param_4, param_5);
            uVar3 = *(_PTR_LOOP_1050_65e2 + 0x52);
            pass1_1030_4bbe(param_4, uVar5, uVar3, (uStack14 + 0x12));
            (param_1 + 0xe) = (long)(uVar3 + 0x94) + *_PTR_LOOP_1050_65e2;
        }
    }
    return;
}

void pass1_1008_c83a(u32 param_1)

{
    if(*_PTR_LOOP_1050_65e2 <= *(param_1 + 0xe))
    {
        return;
    }
    return;
}


u32 pass1_1008_c85e(u32 param_1, u16 param_2)

{
    i16 iVar1;
    u16 uVar2;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if((iVar1 + 0xa) == 0x0)
    {
        pass1_1008_c882(param_1 & 0xffff | uVar2 << 0x10, param_2);
    }
    return CONCAT22((iVar1 + 0xc), (iVar1 + 0xa));
}


void pass1_1008_caa0(u16 *param_1, u16 param_2)

{
    u16 uVar1;

    uVar1           = (param_1 >> 0x10);
    *param_1        = 0xd71a;
    (param_1 + 0x2) = 0x1008;
    pass1_1008_cac6(param_1 & 0xffff | uVar1 << 0x10);
    pass1_1010_1d80(param_1, param_2);
    return;
}


void pass1_1008_cac6(u32 param_1)

{
    u32  *puVar1;
    u16          uVar2;
    code       **ppcVar3;
    astruct_470 *iVar4;
    u16          uVar4;

    uVar4  = (param_1 >> 0x10);
    iVar4  = (astruct_470 *)param_1;
    puVar1 = iVar4->field_0xa;
    uVar2  = iVar4->field_0xc;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    &iVar4->field_0xa = 0x0;
    puVar1            = iVar4->field_0xe;
    uVar2             = iVar4->field_0x10;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    &iVar4->field_0xe = 0x0;
    puVar1            = iVar4->field_0x12;
    uVar2             = iVar4->field_0x14;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    &iVar4->field_0x12 = 0x0;
    puVar1             = iVar4->field_0x16;
    uVar2              = iVar4->field_0x18;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    &iVar4->field_0x16 = 0x0;
    puVar1             = iVar4->field_0x1a_addr_offset;
    uVar2              = iVar4->field_0x1c_addr_base;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    &iVar4->field_0x1a_addr_offset = 0x0;
    puVar1             = iVar4->field_0x1e;
    uVar2              = iVar4->field_0x20;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    &iVar4->field_0x1e = 0x0;
    return;
}


u32 pass1_1008_b340(u32 param_1)

{
    u32 uVar1;
    i16        iVar2;
    u16        uVar3;

    uVar3 = (param_1 >> 0x10);
    if((param_1 + 0x16) != 0x0)
    {
        uVar1 = (param_1 + 0x16);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + 0x6), (iVar2 + 0x4));
    }
    return 0x0;
}


u32 pass1_1008_b366(u32 param_1)

{
    u32 uVar1;
    i16        iVar2;
    u16        uVar3;

    uVar3 = (param_1 >> 0x10);
    if((param_1 + 0x1a) != 0x0)
    {
        uVar1 = (param_1 + 0x1a);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + 0x6), (iVar2 + 0x4));
    }
    return 0x0;
}


u32 pass1_1008_b47a(u32 param_1)

{
    u32 uVar1;
    i16        iVar2;
    u16        uVar3;

    uVar3 = (param_1 >> 0x10);
    if((param_1 + 0x1e) != 0x0)
    {
        uVar1 = (param_1 + 0x1e);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + 0x6), (iVar2 + 0x4));
    }
    return 0x0;
}


void pass1_1008_b4a0(u32 param_1, long param_2, u16 param_3, u8 *param_4, u16 param_5)

{
    u32 uVar1;
    u16        uVar2;
    u16        uVar3;
    i16        iVar4;
    u16        uVar5;
    u32        uVar6;
    long       lVar7;

    iVar4 = param_1;
    uVar5 = (param_1 >> 0x10);
    if(param_2 == 0x0)
    {
        (iVar4 + 0x16) = 0x0;
    }
    else
    {
        pass1_1008_b9ce(param_1, param_2, param_5);
        *(iVar4 + 0x16) = param_3;
        (iVar4 + 0x18)  = param_4;
    }
    uVar1 = (iVar4 + 0x16);
    if((uVar1 + 0x8) != 0x0)
    {
        pass1_1008_b200(param_1, param_5);
        uVar6 = pass1_1008_b38c(param_1, param_3, param_4);
        uVar3 = (uVar6 >> 0x10);
        uVar2 = uVar6;
        uVar1 = (iVar4 + 0x16);
        pass1_1008_b85c(param_1, (uVar1 + 0xa));
        (iVar4 + 0x1a) = uVar2;
        (iVar4 + 0x1c) = uVar3;
        uVar1          = (iVar4 + 0x16);
        lVar7          = pass1_1008_b8ac(param_1, (uVar1 + 0xe), param_5);
        (iVar4 + 0x1e) = lVar7;
        (iVar4 + 0x20) = (lVar7 >> 0x10);
        return;
    }
    (iVar4 + 0x1a) = 0x0;
    (iVar4 + 0x1e) = 0x0;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1008_b544(u32 param_1, i16 param_2, u16 param_3, u16 param_4)

{
    u32 *puVar1;
    u16         uVar2;
    code      **ppcVar3;
    u32  uVar4;
    u32  uVar5;
    u16         uVar6;
    i16         iVar7;
    u16         uVar8;

    iVar7 = param_1;
    uVar8 = (param_1 >> 0x10);
    if(param_2 != 0x0)
    {
        if((iVar7 + 0x1a) != 0x0)
        {
            uVar4         = (iVar7 + 0x16);
            (uVar4 + 0x8) = 0x1;
            uVar4         = (iVar7 + 0x1a);
            uVar5         = (iVar7 + 0x16);
            (uVar5 + 0xa) = (uVar4 + 0x8);
            uVar4         = (iVar7 + 0x1e);
            uVar6         = (uVar4 + 0x8);
            uVar4         = (iVar7 + 0x16);
            (uVar4 + 0xe) = uVar6;
            uVar4         = (iVar7 + 0x16);
            pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), *(uVar4 + 0xa));
            param_4 = &PTR_LOOP_1050_1038;
            pass1_1038_3608(CONCAT22(param_3, uVar6));
        }
    }
    (iVar7 + 0x1e) = 0x0;
    (iVar7 + 0x1a) = 0x0;
    (iVar7 + 0x16) = 0x0;
    puVar1         = *(iVar7 + 0xe);
    uVar2          = *(iVar7 + 0x10);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)(param_4, puVar1, uVar2, 0x1);
    }
    (iVar7 + 0xe) = 0x0;
    puVar1        = *(iVar7 + 0x12);
    uVar2         = *(iVar7 + 0x14);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)(param_4, puVar1, uVar2, 0x1);
    }
    (iVar7 + 0x12) = 0x0;
    return;
}


void pass1_1008_b61a(u32 param_1, u32 param_2, u16 param_3, u16 param_4, u16 param_5)

{
    u16 uVar1;

    pass1_1008_b8fa(param_1, param_2, param_4, param_5);
    uVar1            = (param_1 >> 0x10);
    (param_1 + 0x1a) = param_3;
    (param_1 + 0x1c) = param_4;
    return;
}


void pass1_1008_b63a(u32 param_1, u32 param_2)

{
    u16 in_AX;
    u16 in_DX;
    u16 uVar1;
    u16 unaff_SS;

    pass1_1008_b964(param_1, param_2, unaff_SS);
    uVar1            = (param_1 >> 0x10);
    (param_1 + 0x1e) = in_AX;
    (param_1 + 0x20) = in_DX;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void load_string_1008_b65a(u32 param_1, LPSTR in_string_2, u32 param_3, u1616 param_4)

{
    u16 unaff_SS;

    pass1_1008_b9ce(param_1, CONCAT22(param_4, param_3._2_2_), unaff_SS);
    load_string_1010_84e0(0x1010, globals->_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, in_string_2, (short)param_3);
    return;
}


u32 pass1_1008_b820(u32 param_1, i16 param_2, u16 param_3)

{
    u16 uVar1;

    pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), 0x8000001);
    if((param_2 + 0x152) == 0x0)
    {
        return 0x0;
    }
    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0xc), (param_1 + 0xa));
}


void pass1_1008_b85c(u32 param_1, long param_2)

{
    u8 *puVar1;
    u16 extraout_DX;
    u16 unaff_SS;
    u8  local_a[0x8];

    pass1_1008_5784(CONCAT22(unaff_SS, local_a), *(param_1 + 0xe));
    do
    {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, unaff_SS);
        if((extraout_DX | puVar1) == 0x0)
        {
            return;
        }
    } while((puVar1 + 0x8) != param_2);
    return;
}


long pass1_1008_b8ac(u32 param_1, i16 param_2, u16 param_3)

{
    long lVar1;
    u8   local_a[0x8];

    pass1_1008_5784(CONCAT22(param_3, local_a), *(param_1 + 0x12));
    do
    {
        lVar1 = pass1_1008_5b12(local_a, param_3);
        if(lVar1 == 0x0)
        {
            return 0x0;
        }
    } while((lVar1 + 0x8) != param_2);
    return lVar1;
}


void pass1_1008_b8fa(u32 param_1, u32 param_2, u16 param_3, u16 param_4)

{
    u8 *puVar1;
    i16 iVar2;
    u16 extraout_DX;
    u8  local_a[0x8];

    if(param_2 == 0x0)
    {
        return;
    }
    pass1_1008_5784(CONCAT22(param_4, local_a), *(param_1 + 0xe));
    do
    {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, param_4);
        if((extraout_DX | puVar1) == 0x0)
        {
            return;
        }
        iVar2 = pass1_1000_3d7a(*(puVar1 + 0x4), param_2);
    } while(iVar2 != 0x0);
    return;
}


void pass1_1008_b964(u32 param_1, u32 param_2, u16 param_3)

{
    u8 *puVar1;
    i16 iVar2;
    u16 extraout_DX;
    u8  local_a[0x8];

    if(param_2 == 0x0)
    {
        return;
    }
    pass1_1008_5784(CONCAT22(param_3, local_a), *(param_1 + 0x12));
    do
    {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, param_3);
        if((extraout_DX | puVar1) == 0x0)
        {
            return;
        }
        iVar2 = pass1_1000_3d7a(*(puVar1 + 0x4), param_2);
    } while(iVar2 != 0x0);
    return;
}


void pass1_1008_b9ce(u32 param_1, u32 param_2, u16 param_3)

{
    u8 *puVar1;
    i16 iVar2;
    u16 extraout_DX;
    u8  local_a[0x8];

    if(param_2 == 0x0)
    {
        return;
    }
    pass1_1008_5784(CONCAT22(param_3, local_a), *(param_1 + 0xa));
    do
    {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, param_3);
        if((extraout_DX | puVar1) == 0x0)
        {
            return;
        }
        iVar2 = pass1_1000_3d7a(*(puVar1 + 0x4), param_2);
    } while(iVar2 != 0x0);
    return;
}


u32  pass1_1008_a8f4(u32 param_1, u16 *param_2, u16 *param_3, u16 *param_4, u16 param_5, u16 param_6, u16 param_7, u8 param_8)

{
    i16        iVar1;
    u32 local_6;

    iVar1 = &local_6 + 0x2;
    pass1_1008_a1f0(param_6, param_7, param_8, param_1, param_2, CONCAT22(param_7, &local_6), CONCAT22(param_7, iVar1), param_4);
    pass1_1008_944e(param_3, local_6, (local_6 >> 0x10));
    return CONCAT22(param_5, iVar1);
}


u16 pass1_1008_a9ec(u32 param_1)

{
    u32  uVar1;
    u16         in_AX;
    i16         iVar2;
    u16         uVar3;
    WNDCLASS16 *unaff_SS;
    u16         uStack4;

    uStack4 = 0x0;
    uVar3   = (param_1 >> 0x10);
    iVar2   = param_1;
    if(((iVar2 + 0x414) == 0x0) && (uVar1 = (iVar2 + 0x410), (uVar1 + 0x8) != 0x0))
    {
        (iVar2 + 0x414) = 0x1;
        pass1_1008_aa28(param_1 & 0xffff | uVar3 << 0x10, in_AX, unaff_SS);
        uStack4 = in_AX;
    }
    return uStack4;
}


u16 pass1_1008_aaa8(u16 param_1, u16 param_2, u16 param_3)

{
    u16 uStack4;

    uStack4 = 0x0;
    switch(param_3)
    {
    case 0x1:
        uStack4 = 0x24;
        break;
    case 0x2:
        uStack4 = 0x16;
        break;
    case 0x3:
        uStack4 = 0x17;
        break;
    case 0x4:
        uStack4 = 0x18;
        break;
    case 0x5:
        uStack4 = 0x1b;
        break;
    case 0x6:
        uStack4 = 0x1c;
        break;
    case 0x7:
        uStack4 = 0x1f;
    }
    return uStack4;
}


u16 pass1_1008_ab12(u16 param_1, u16 param_2, u16 param_3)

{
    if(param_3 == 0x37)
    {
        return 0x22;
    }
    if(param_3 < 0x38)
    {
        if(param_3 == '\r')
        {
            return 0xf;
        }
        if(param_3 == '*')
        {
            return 0x2b;
        }
    }
    return 0x0;
}


u16 pass1_1008_ab54(u32 param_1)

{
    u32 uVar1;
    u16        uVar2;
    u16        uStack4;

    uStack4 = 0x0;
    uVar2   = (param_1 >> 0x10);
    if(((param_1 + 0xa) != 0x0) && (uVar1 = (param_1 + 0xa), (uVar1 + 0x8) != 0x0))
    {
        uStack4 = 0x1;
    }
    return uStack4;
}


u16 pass1_1008_ab80(u16 param_1, u16 param_2, u16 param_3)

{
    u16 uStack4;

    uStack4 = 0x0;
    switch(param_3)
    {
    case 0x8:
        uStack4 = 0x82;
        break;
    case 0x9:
        uStack4 = 0x7f;
        break;
    case 0xa:
        uStack4 = 0x80;
        break;
    case 0xb:
        uStack4 = 0x84;
        break;
    case 0xc:
        uStack4 = 0x89;
        break;
    case 0xd:
        uStack4 = 0x8a;
        break;
    case 0xe:
        uStack4 = 0x8c;
        break;
    case 0xf:
        uStack4 = 0x8e;
        break;
    case 0x10:
        uStack4 = 0x8f;
        break;
    case 0x11:
        uStack4 = 0x90;
        break;
    case 0x12:
        uStack4 = 0x91;
        break;
    case 0x13:
        uStack4 = 0x95;
        break;
    case 0x14:
        uStack4 = 0x96;
        break;
    case 0x16:
        uStack4 = 0x9b;
        break;
    case 0x17:
        uStack4 = 0x9f;
        break;
    case 0x18:
        uStack4 = 0xa2;
        break;
    case 0x19:
        uStack4 = 0xa4;
        break;
    case 0x1b:
    case 0x1c:
        uStack4 = 0xa7;
        break;
    case 0x1d:
        uStack4 = 0xaa;
        break;
    case 0x1e:
        uStack4 = 0xac;
        break;
    case 0x1f:
        uStack4 = 0xad;
        break;
    case 0x20:
        uStack4 = 0xae;
        break;
    case 0x21:
        uStack4 = 0xb1;
        break;
    case 0x22:
        uStack4 = 0xb3;
        break;
    case 0x23:
        uStack4 = 0xb4;
        break;
    case 0x24:
        uStack4 = 0xb5;
        break;
    case 0x25:
        uStack4 = 0xb6;
        break;
    case 0x26:
        uStack4 = 0xb7;
        break;
    case 0x27:
        uStack4 = 0xab;
        break;
    case 0x28:
        uStack4 = 0xb9;
        break;
    case 0x29:
        uStack4 = 0xba;
        break;
    case 0x2a:
        uStack4 = 0xbc;
        break;
    case 0x2b:
        uStack4 = 0xbe;
        break;
    case 0x2c:
        uStack4 = 0xdf;
        break;
    case 0x2d:
        uStack4 = 0xe0;
    }
    return uStack4;
}


u16 *pass1_1008_ad0c(u16 *param_1, u8 param_2)

{
    u16 uVar1;

    uVar1          = (param_1 >> 0x10);
    *param_1       = 0x389a;
    (param_1)[0x1] = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        pass1_1000_093a(param_1, uVar1, 0x1000);
    }
    return param_1;
}

u16 *pass1_1008_ada2(u16 *param_1, i16 param_2)

{
    u16 uVar1;

    uVar1           = (param_1 >> 0x10);
    *param_1        = 0x0;
    (param_1 + 0x2) = 0x0;
    (param_1 + 0x4) = param_2;
    *param_1        = (param_2 * 0x6 + 0x3a4);
    return param_1;
}


void pass1_1008_add2(u16 *param_1)

{
    *param_1 = ((param_1 + 0x4) * 0x6 + 0x3a4);
    return;
}


u16 pass1_1008_adf2(u32 param_1)

{
    return ((param_1 + 0x4) * 0x6 + 0x3a4);
}


u16 pass1_1008_ae0c(u32 param_1)

{
    return ((param_1 + 0x4) * 0x6 + 0x3a6);
}


void pass1_1008_ae26(i16 *param_1)

{
    i16 *piVar1;
    i16  iVar2;
    i16  iVar3;
    u16  uVar4;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    iVar2 = ((iVar3 + 0x4) * 0x6 + 0x3a8);
    if(iVar2 == 0x2)
    {
        if((iVar3 + 0x2) == 0x1)
        {
            *param_1 = *param_1 + -0x1;
            iVar2    = (iVar3 + 0x4) * 0x6;
            piVar1   = (iVar2 + 0x3a4);
            if(*piVar1 != *param_1 && *param_1 <= *piVar1)
            {
                *param_1      = (iVar2 + 0x3a4) + 0x1;
                (iVar3 + 0x2) = 0x0;
                return;
            }
        }
        else
        {
            *param_1 = *param_1 + 0x1;
            iVar2    = (iVar3 + 0x4) * 0x6;
            if((iVar2 + 0x3a6) < *param_1)
            {
                *param_1      = (iVar2 + 0x3a6) + -0x1;
                (iVar3 + 0x2) = 0x1;
                return;
            }
        }
    }
    else
    {
        if((iVar2 != 0x3) && (iVar2 != 0x4))
        {
            *param_1 = *param_1 + 0x1;
            iVar2    = (iVar3 + 0x4) * 0x6;
            if((iVar2 + 0x3a6) < *param_1)
            {
                *param_1 = (iVar2 + 0x3a4);
            }
        }
    }
    return;
}


BOOL16 pass1_1008_aed8(u32 param_1)

{
    if(((param_1 + 0x4) * 0x6 + 0x3a4) != 0x0)
    {
        return 0x1;
    }
    return 0x0;
}

void pass1_1008_afde(u16 *param_1, u16 param_2)

{
    u32  *puVar1;
    u16          uVar2;
    code       **ppcVar3;
    astruct_468 *iVar4;
    u16          uVar4;

    uVar4            = (param_1 >> 0x10);
    iVar4            = (astruct_468 *)param_1;
    *param_1         = 0xbdcc;
    iVar4->field_0x2 = 0x1008;
    puVar1           = iVar4->field_0xa;
    uVar2            = iVar4->field_0xc;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    puVar1 = iVar4->field_0xe;
    uVar2  = iVar4->field_0x10;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    puVar1 = iVar4->field_0x12;
    uVar2  = iVar4->field_0x14;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}


u16 *pass1_1008_b05a(u16 *param_1)

{
    astruct_193 *iVar1;
    u16          uVar1;

    uVar1            = (param_1 >> 0x10);
    iVar1            = (astruct_193 *)param_1;
    *param_1         = 0x389a;
    iVar1->field_0x2 = 0x1008;
    iVar1->field_0x4 = 0x0;
    *param_1         = 0xbdc8;
    iVar1->field_0x2 = 0x1008;
    return param_1;
}


void set_stuct_1008_b0bc(astruct_26 *param_1)

{
    astruct_26 *iVar1;
    u16         uVar1;

    pass1_1008_b05a(param_1);
    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_26 *)param_1;
    iVar1->field_0x8  = 0x0;
    iVar1->field_0xa  = 0x0;
    iVar1->field_0xe  = 0x0;
    iVar1->field_0x10 = 0x0;
    param_1           = 0xbdc4;
    iVar1->field_0x2  = 0x1008;
    return;
}


u16 *pass1_1008_b0f2(u16 *param_1)

{
    u16 uVar1;

    pass1_1008_b05a(param_1);
    uVar1           = (param_1 >> 0x10);
    (param_1 + 0x8) = 0x0;
    *param_1        = 0xbdc0;
    (param_1 + 0x2) = 0x1008;
    return param_1;
}


u16 *pass1_1008_b11e(u16 *param_1)

{
    u16 uVar1;

    pass1_1008_b05a(param_1);
    uVar1           = (param_1 >> 0x10);
    (param_1 + 0x8) = 0x0;
    *param_1        = 0xbddc;
    (param_1 + 0x2) = 0x1008;
    return param_1;
}

void pass1_1008_b146(u32 param_1, u16 param_2, u16 param_3)

{
    u32 uVar1;
    i16        iVar2;
    u16        uVar3;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if((iVar2 + 0x16) != 0x0)
    {
        uVar1 = (iVar2 + 0x16);
        pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), *(uVar1 + 0xa));
        pass1_1038_3608(CONCAT22(param_3, param_2));
        uVar1          = (iVar2 + 0x16);
        (uVar1 + 0x8)  = 0x0;
        uVar1          = (iVar2 + 0x16);
        (uVar1 + 0xa)  = 0x0;
        uVar1          = (iVar2 + 0x16);
        (uVar1 + 0xe)  = 0x0;
        uVar1          = (iVar2 + 0x16);
        (uVar1 + 0x10) = 0x0;
    }
    return;
}


void pass1_1008_9628(u32 param_1, u16 param_2)

{
    u16 uVar1;

    uVar1 = (param_1 >> 0x10);
    if((param_1 + 0x8) == 0x0)
    {
        (param_1 + 0x8) = param_2;
    }
    return;
}


LRESULT pass1_1008_9c16(u1616 param_1, u32 param_2, u32 param_3, HWND16 param_4)

{
    LRESULT LVar1;

    LVar1 = make_def_wnd_proc_1008_9ce6(param_1, (u1616)param_2, (u1616)(param_2 >> 0x10), (WPARAM16)param_3, CONCAT22(0x85, (param_3 >> 0x10)), param_4);
    return LVar1;
}


LRESULT pass1_1008_9c30(u1616 param_1, u32 param_2, u32 param_3, HWND16 param_4)

{
    LRESULT LVar1;

    LVar1 = make_def_wnd_proc_1008_9ce6(param_1, (u1616)param_2, (u1616)(param_2 >> 0x10), (WPARAM16)param_3, CONCAT22(0x86, (param_3 >> 0x10)), param_4);
    return LVar1;
}


void pass1_1008_9c4a(void)

{
    return;
}


void pass1_1008_9c4e(void)

{
    return;
}


void pass1_1008_9c52(void)

{
    return;
}


void pass1_1008_9c60(u16 param_1, u16 param_2, u32 *param_3, i16 param_4)

{
    fn_ptr_1 *ppcVar1;

    if((param_4 == 0xc7) && (param_3 != 0x0))
    {
        ppcVar1 = *param_3;
        (**ppcVar1)();
    }
    return;
}


BOOL16 pass1_1008_9cc4(u32 param_1, i16 param_2)

{
    if((param_1 + 0x8) != param_2)
    {
        return 0x1;
    }
    return 0x0;
}


u16 pass1_1008_9ce0(void)

{
    return 0x0;
}


void pass1_1008_9f18(i16 param_1, u16 param_2, i16 param_3, u16 param_4)

{
    if(param_3 == 0x2)
    {
        pass1_1008_9f64(CONCAT22(param_2, param_1 + -0x1c));
        pass1_1010_1f62(param_4, CONCAT22(param_2, param_1 + -0x1c), 0x2);
    }
    return;
}


u32 pass1_1008_9f48(u32 param_1)

{
    astruct_134 *iVar1;
    i16          iVar2;
    u16          uVar3;

    uVar3 = (param_1 >> 0x10);
    iVar1 = (astruct_134 *)param_1;
    iVar2 = iVar1->field_0x20 * 0x4;
    return CONCAT22((&iVar1[0x1].field_0x2 + iVar2), (&iVar1[0x1].field_0x0 + iVar2));
}


void pass1_1008_9f64(u32 param_1)

{
    i16 *piVar1;
    i16  iVar2;
    u16  uVar3;

    uVar3   = (param_1 >> 0x10);
    iVar2   = param_1;
    piVar1  = (iVar2 + 0x20);
    *piVar1 = *piVar1 + 0x1;
    if(0xb < (iVar2 + 0x20))
    {
        (iVar2 + 0x20) = 0x0;
    }
    return;
}

void pass1_1008_a086(u16 *param_1, u16 param_2)

{
    u32  *puVar1;
    u16          uVar2;
    code       **ppcVar3;
    astruct_465 *iVar4;
    u16          uVar4;

    uVar4            = (param_1 >> 0x10);
    iVar4            = (astruct_465 *)param_1;
    *param_1         = 0xad92;
    iVar4->field_0x2 = 0x1008;
    puVar1           = iVar4->field_0xa;
    uVar2            = iVar4->field_0xc;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    puVar1 = iVar4->field_0x410;
    uVar2  = iVar4->field_0x412;
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}

void  pass1_1008_87cc(astruct_86 *param_1, i16 param_2, i16 param_3, u16 param_4, u32 param_5, u32 param_6, u16 param_7)

{
    long        lVar1;
    u16         uVar2;
    BOOL16      BVar3;
    i16        *piVar4;
    u8         *puVar5;
    astruct_86 *iVar5;
    i16         iVar6;
    i16         unaff_DI;
    u16         uVar7;
    u16         uVar8;
    u16        *puVar9;
    i16        *piStack48;
    u32  local_24;
    u16         uStack32;
    u32         uStack30;
    Struct18 *paStack26;
    u32  uStack18;
    i16         iStack14;
    i16         iStack12;
    i16         iStack10;
    i16         iStack8;
    u32         uStack6;

    uVar7              = (param_1 >> 0x10);
    iVar5              = (astruct_86 *)param_1;
    param_1->field_0x0 = 0x389a;
    iVar5->field_0x2   = 0x1008;
    iVar5->field_0x4   = (astruct_76 *)param_5;
    &iVar5->field_0x8  = 0x0;
    iVar5->field_0xc   = param_3;
    iVar5->field_0xe   = param_2;
    iVar5->field_0x10  = 0x0;
    iVar5->field_0x12  = 0x0;
    pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x1c_addr_base)));
    pass1_1008_3e38((param_1 & 0xffff0000 | &iVar5->field_0x22));
    puVar9             = pass1_1008_3e38((param_1 & 0xffff0000 | &iVar5->field_0x28));
    iVar5->field_0x2e  = param_4;
    iVar5->field_0x30  = 0xffff;
    iVar5->field_0x3a  = 0x0;
    iVar5->field_0x3e  = 0x1;
    iVar5->field_0x40  = 0x1;
    iVar5->field_0x42  = param_6;
    param_1->field_0x0 = 0x8e9a;
    iVar5->field_0x2   = 0x1008;
    if(_PTR_LOOP_1050_0382 == 0x0)
    {
        globals->_PTR_LOOP_1050_0382 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2e, param_7, (puVar9 >> 0x10), unaff_DI);
    }
    uStack6           = pass1_1008_4772(iVar5->field_0x4);
    iVar5->field_0x12 = 0x2f - (uStack6 + 0x8);
    uVar8             = (_PTR_LOOP_1050_0382 >> 0x10);
    iVar6             = globals->_PTR_LOOP_1050_0382;
    iStack8           = (iVar6 + 0xa);
    iStack10          = (iVar6 + 0xc);
    iStack12          = (iVar6 + 0xe);
    iStack14          = (iVar6 + 0x10);
    iVar6             = iVar5->field_0xc;
    lVar1             = (long)(iVar6 + iVar5->field_0xe) * (long)iStack14;
    puVar5            = (lVar1 >> 0x10);
    pass1_1008_3e76((param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x1c_addr_base)), 0x0, lVar1 + iVar5->field_0x12 + iStack10, (iVar6 - iVar5->field_0xe) * iStack12 + iVar5->field_0x10 + iStack8);
    iVar5->field_0x14 = iVar5->field_0x1c_addr_base + 0x20;
    iVar5->field_0x16 = (uStack6 + 0x8) + iVar5->field_0x1e + -0x25;
    iVar5->field_0x18 = iVar5->field_0x14 + 0x32;
    uVar2             = iVar5->field_0x16 + 0x19;
    iVar5->field_0x1a_addr_offset = uVar2;
    mem_op_1000_179c(0x6, puVar5, 0x1000);
    paStack26      = (Struct18 *)CONCAT22(puVar5, uVar2);
    uStack18._2_2_ = puVar5 | uVar2;
    if(uStack18._2_2_ == 0x0)
    {
        &iVar5->field_0x8 = 0x0;
    }
    else
    {
        puVar9           = pass1_1008_ada2(CONCAT22(puVar5, uVar2), iVar5->field_0x2e);
        uStack18._2_2_   = (puVar9 >> 0x10);
        iVar5->field_0x8 = puVar9;
        iVar5->field_0xa = uStack18._2_2_;
    }
    BVar3 = pass1_1008_aed8(*&iVar5->field_0x8);
    if(BVar3 == 0x0)
    {
        paStack26 = &iVar5->field_0x8;
        uStack18  = paStack26;
        fn_ptr_1000_17ce(paStack26, 0x1000);
        &iVar5->field_0x8 = 0x0;
    }
    else
    {
        piVar4 = *(i16 **)&iVar5->field_0x8;
        pass1_1018_20ee(_PTR_LOOP_1050_0382, piVar4);
        uStack18._0_2_ = SUB42(piVar4, 0x0);
        pass1_1008_add2(*(u16 **)&iVar5->field_0x8);
        uStack30 = pass1_1008_4772((astruct_76 *)CONCAT22(uStack18._2_2_, uStack18));
        pass1_1018_214e(_PTR_LOOP_1050_0382, (_PTR_LOOP_1050_0382 >> 0x10), (param_1 & 0xffff0000 | &iVar5->field_0x28), iVar5->field_0x2e);
        local_24 = &iVar5->field_0x1c_addr_base;
        uStack32 = iVar5->field_0x20;
        pass1_1008_3f32(CONCAT22(param_7, &local_24), (param_1 & 0xffff0000 | &iVar5->field_0x28));
        piStack48 = (param_1 & 0xffff0000 | &iVar5->field_0x32);
        pass1_1008_3e94(CONCAT22(param_7, &local_24), (param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x34)), (param_1 & 0xffff0000 | &iVar5->field_0x32));
        uVar8             = (uStack30 >> 0x10);
        iVar5->field_0x36 = (uStack30 + 0x4) + *piStack48;
        uVar2             = (uStack30 + 0x8) + iVar5->field_0x34;
        iVar5->field_0x38 = uVar2;
        pass1_1008_612e(0x2, 0x5, uVar2);
        iVar5->field_0x3e = uVar2;
    }
    return;
}
void pass1_1008_8b20(u32 param_1, u16 param_2)

{
    i16         iVar1;
    i16        *piVar2;
    u16         uVar3;
    i16         iVar4;
    u16         uVar5;
    u8          local_a[0x2];
    u8          local_8[0x2];
    astruct_76 *paStack6;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if((iVar4 + 0x8) != 0x0)
    {
        iVar1   = (iVar4 + 0x40);
        piVar2  = (iVar4 + 0x40);
        *piVar2 = *piVar2 + 0x1;
        uVar3   = iVar1 % (iVar4 + 0x3e);
        if(uVar3 == 0x0)
        {
            (iVar4 + 0x40) = 0x1;
            piVar2         = *(i16 **)(iVar4 + 0x8);
            pass1_1018_20ee(_PTR_LOOP_1050_0382, piVar2);
            paStack6 = (astruct_76 *)(piVar2 & 0xffff | uVar3 << 0x10);
            pass1_1008_3e94((param_1 & 0xffff0000 | (iVar4 + 0x28U)), CONCAT22(param_2, local_a), CONCAT22(param_2, local_8));
            pass1_1008_8d8a(param_1 & 0xffff | uVar5 << 0x10, paStack6, *(iVar4 + 0x4));
            pass1_1008_4480(*(iVar4 + 0x4), (param_1 & 0xffff0000 | (iVar4 + 0x28U)), paStack6, param_2);
            return;
        }
    }
    return;
}


void pass1_1008_8bc6(u16 param_1, u16 param_2, u32 param_3)

{
    i16        *piVar1;
    i16         iVar2;
    u16         uVar3;
    u8          local_a[0x2];
    u8          local_8[0x2];
    astruct_76 *paStack6;

    uVar3 = (param_3 >> 0x10);
    iVar2 = param_3;
    if((iVar2 + 0x8) == 0x0)
    {
        return;
    }
    piVar1 = *(i16 **)(iVar2 + 0x8);
    pass1_1018_20ee(_PTR_LOOP_1050_0382, piVar1);
    paStack6 = (astruct_76 *)(piVar1 & 0xffff | param_2 << 0x10);
    pass1_1008_3e94((param_3 & 0xffff0000 | (iVar2 + 0x28U)), CONCAT22(param_1, local_a), CONCAT22(param_1, local_8));
    pass1_1008_8d8a(param_3 & 0xffff | uVar3 << 0x10, paStack6, *(iVar2 + 0x4));
    pass1_1008_4480(*(iVar2 + 0x4), (param_3 & 0xffff0000 | (iVar2 + 0x28U)), paStack6, param_1);
    return;
}


void pass1_1008_8c4e(u32 param_1, u32 param_2, u16 param_3)

{
    u16          uVar1;
    u16         *puVar2;
    u8          *puVar3;
    u8          *puVar4;
    u16          uVar5;
    i16          iVar6;
    u16          uVar7;
    u32          uVar8;
    astruct_110 *paStack14;

    uVar7  = (param_1 >> 0x10);
    iVar6  = param_1;
    uVar8  = pass1_1008_4772(*(astruct_76 **)(iVar6 + 0x4));
    puVar3 = (uVar8 >> 0x10);
    uVar5  = 0x0;
    if(((iVar6 + 0xc) == 0x0) || ((iVar6 + 0xe) == 0x0))
    {
        puVar4 = puVar3;
        mem_op_1000_179c(0x14, puVar3, 0x1000);
        paStack14 = (astruct_110 *)CONCAT22(puVar4, uVar5);
        uVar5     = puVar4 | uVar5;
        if(uVar5 == 0x0)
        {
            uVar1 = 0x0;
            uVar5 = 0x0;
        }
        else
        {
            puVar2 = (param_1 & 0xffff0000 | (iVar6 + 0x1c));
            pass1_1008_50c2(paStack14, *(uVar8 + 0x8), *(uVar8 + 0x4), puVar2, param_2);
            uVar1 = SUB42(puVar2, 0x0);
        }
        pass1_1008_5134(CONCAT22(uVar5, uVar1));
    }
    pass1_1008_4480(param_2, (param_1 & 0xffff0000 | (iVar6 + 0x1c)), *(astruct_76 **)(iVar6 + 0x4), param_3);
    return;
}

void pass1_1008_8ce4(u32 param_1, u16 *param_2, u32 param_3, u16 param_4)

{
    u8        *puVar1;
    u8        *puVar2;
    u16        uVar3;
    i16        iVar4;
    u16        uVar5;
    u16        uVar6;
    u16       *puVar7;
    u8         local_10[0x6];
    u32 uStack10;
    u32        uStack6;

    uVar5    = (param_1 >> 0x10);
    iVar4    = param_1;
    uStack6  = pass1_1008_4772(*(astruct_76 **)(iVar4 + 0x4));
    uStack10 = 0x0;
    puVar7   = pass1_1008_3e54(CONCAT22(param_4, local_10), 0x0, (iVar4 + 0x12), (iVar4 + 0x10));
    puVar2   = (puVar7 >> 0x10);
    puVar1   = local_10;
    pass1_1008_3f32(param_2, CONCAT22(param_4, puVar1));
    mem_op_1000_179c(0x14, puVar2, 0x1000);
    uVar3 = puVar2 | puVar1;
    if(uVar3 == 0x0)
    {
        puVar1 = 0x0;
        uVar3  = 0x0;
    }
    else
    {
        uVar6 = (uStack6 >> 0x10);
        pass1_1008_50c2((astruct_110 *)CONCAT22(puVar2, puVar1), *(uStack6 + 0x8), *(uStack6 + 0x4), param_2, param_3);
    }
    uStack10 = CONCAT22(uVar3, puVar1);
    pass1_1008_5134(CONCAT22(uVar3, puVar1));
    pass1_1008_4480(param_3, param_2, *(astruct_76 **)(iVar4 + 0x4), param_4);
    return;
}


void pass1_1008_8faa(u32 param_1, u32 param_2)

{
    u16 uVar1;

    uVar1 = (param_1 >> 0x10);
    pass1_1008_9004(param_1 & 0xffff | uVar1 << 0x10, param_2, (param_2 >> 0x10), *(param_1 + 0xa));
    return;
}


void empty_1008_8fc4(void)

{
    return;
}


void pass1_1008_9004(u32 param_1, u16 param_2, u16 param_3, u32 param_4)

{
    u32         *puVar1;
    u16         *puVar2;
    long         lVar3;
    astruct_107 *iVar4;
    astruct_108 *iVar5;
    u16          uVar4;
    u16          uVar5;
    u16          unaff_SS;
    bool         bVar6;

    uVar4  = (param_1 >> 0x10);
    iVar4  = (astruct_107 *)param_1;
    puVar1 = &iVar4->field_0xa;
    if((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4->field_0x6 == 0x0))
    {
        puVar2 = (&iVar4->field_0x12 + 0x2);
        bVar6  = *puVar2 < param_4._2_2_;
        if((bVar6 || *puVar2 == param_4._2_2_) && ((bVar6 || (puVar1 = &iVar4->field_0x12, *puVar1 < param_4 || *puVar1 == param_4))))
        {
            pass1_1008_909c(param_1 & 0xffff | uVar4 << 0x10, unaff_SS);
        }
        puVar1 = &iVar4->field_0x12;
        if((*puVar1 < param_4 || *puVar1 == param_4) || (iVar4->field_0x6 == 0x0))
        {
            return;
        }
        puVar2 = &iVar4->field_0xc;
        bVar6  = *puVar2 < param_4._2_2_;
        if((bVar6 || *puVar2 == param_4._2_2_) && ((bVar6 || (puVar2 = &iVar4->field_0xa, *puVar2 < param_4 || *puVar2 == param_4))))
        {
            iVar4->field_0xa = (param_4 + 0x1);
            iVar4->field_0xc = (param_4 + 0x1 >> 0x10);
        }
    }
    lVar3                         = iVar4->field_0x6;
    uVar5                         = (lVar3 >> 0x10);
    iVar5                         = (astruct_108 *)lVar3;
    (iVar5 + param_4 * 0x4)       = param_2;
    (iVar5 + param_4 * 0x4 + 0x2) = param_3;
    return;
}


void pass1_1008_92b2(u32 param_1, long param_2, long param_3)

{
    code     **ppcVar1;
    u8        *puVar2;
    u16        extraout_DX;
    u16        unaff_SS;
    u8         local_c[0x4];
    u32 uStack8;
    u16        uStack4;

    uStack4 = 0x0;
    pass1_1008_57a4(CONCAT22(unaff_SS, local_c), param_1 & 0xffff0000 | (param_1 + 0x6));
    while(true)
    {
        puVar2 = local_c;
        pass1_1008_5b12(puVar2, unaff_SS);
        if((extraout_DX | puVar2) == 0x0)
            break;
        if(((puVar2 + 0x4) == param_3) && ((puVar2 + 0x8) == param_2))
        {
            uStack4 = 0x1;
            ppcVar1 = ((param_1 + 0x6) + 0xc);
            (**ppcVar1)();
            uStack8 = 0x0;
        }
    }
    return;
}


void pass1_1008_932a(u32 param_1, u16 param_2)

{
    u16    uVar1;
    code **ppcVar2;
    u8    *puVar3;
    u16    extraout_DX;
    i16    iVar4;
    i16    iVar5;
    u16    uVar6;
    u8     local_a[0x8];

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if((iVar5 + 0x4) == 0x0)
    {
        (iVar5 + 0x4) = 0x1;
        pass1_1008_57a4(CONCAT22(param_2, local_a), param_1 & 0xffff0000 | (iVar5 + 0x6));
        while(true)
        {
            puVar3 = local_a;
            pass1_1008_5b12(puVar3, param_2);
            if((extraout_DX | puVar3) == 0x0)
                break;
            uVar1           = *(puVar3 + 0xc);
            iVar4           = (puVar3 + 0xe) - (uVar1 < 0x37);
            *(puVar3 + 0xc) = uVar1 - 0x37;
            (puVar3 + 0xe)  = iVar4;
            if((iVar4 < 0x1) && (((iVar4 < 0x0 || ((puVar3 + 0xc) == 0x0)) && ((puVar3 + 0x10) == 0x0))))
            {
                ppcVar2 = (**(u32 **)(puVar3 + 0x4) + 0x4);
                (**ppcVar2)();
                (puVar3 + 0xc) = (puVar3 + 0x8);
            }
        }
        (iVar5 + 0x4) = 0x0;
    }
    return;
}
