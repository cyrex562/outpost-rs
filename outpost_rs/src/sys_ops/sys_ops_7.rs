
void  pass1_1010_d5ae(mut param_1: *mut u8, param_2: u32, mut param_3: *mut u16, mut param_4: *mut u8, param_5: u8, param_6: i16)

{
    u16       *puVar1;
    u32 uVar2;
    u32        uVar3;
     let mut puVar4: *mut u8;
    u16       *puVar5;
     let mut pcVar6: String;
    i16        iVar7;
     let mut uVar8: u16;
    i16        iVar9;
     let mut uVar10: u16;
    u32        uVar11;
     let mut uVar12: u16;
     let mut local_40c: u16;
     let mut uStack1034: u16;
     let mut uStack1032: u16;
    u32        uStack1030;
    u8         local_402[0x400];

    uVar10     = (param_3 >> 0x10);
    iVar9      = param_3;
    uStack1030 = struct_op_1030_73a8(*(iVar9 + 0x6));
    uStack1034 = uStack1030;
    uStack1032 = (uStack1030 >> 0x10) | uStack1034;
    if(uStack1032 != 0x0)
    {
        pass1_1028_45fe(uStack1030, uStack1034, param_4);
        if((uStack1032 | uStack1034) != 0x0)
        {
            local_40c = 0x0;
            puVar5    = &local_40c;
            uVar12    = (param_1 >> 0x10);
            pass1_1010_d984(param_1, uVar12, CONCAT22(param_4, puVar5), 0x3, CONCAT22(uStack1032, uStack1034), &PTR_DAT_1050_1805_1050_3706, param_3, param_4, param_5);
            puVar1         = *(u16 **)(iVar9 + 0x2);
            uVar8          = (iVar9 + 0x4);
            (puVar1 + 0x4) = globals.PTR_DAT_1050_1805_1050_3706;
            uVar2          = (iVar9 + 0x6);
            pcVar6         = pass1_1010_b038(param_1, uVar2, (uVar2 >> 0x10), (puVar1 + 0x4), param_6);
            unk_str_op_1000_3d3e(CONCAT22(param_4, local_402), CONCAT22(uVar8, pcVar6));
            string_1040_a626(puVar1, CONCAT22(param_4, local_402), uVar8);
            uVar3         = *(iVar9 + 0x2);
            uVar8         = (iVar9 + 0x4);
            iVar7         = uVar3;
            (iVar7 + 0xe) = globals.PTR_DAT_1050_1822_1050_3708;
            sys_1000_3f9c(local_402, param_4, 0x3926, &USHORT_1050_1050, local_40c, &stack0xfffe, uVar8, 0x1000, param_4, param_5);
            string_1040_a626((uVar3 & 0xffff0000 | (iVar7 + 0xa)), CONCAT22(param_4, local_402), uVar8);
            puVar4         = globals.PTR_DAT_1050_1823_1050_370a;
            uVar3          = *(iVar9 + 0x2);
            iVar9          = (iVar9 + 0x4);
            iVar7          = uVar3;
            (iVar7 + 0x18) = globals.PTR_DAT_1050_1823_1050_370a;
            uVar11         = pass1_1028_45e2(uStack1030, puVar4, iVar9, param_4);
            uVar8          = (uVar11 >> 0x10);
            sys_1000_3f9c(local_402, param_4, 0x3929, &USHORT_1050_1050, uVar11, &stack0xfffe, iVar9, 0x1000, param_4, param_5);
            string_1040_a626((uVar3 & 0xffff0000 | (iVar7 + 0x14)), CONCAT22(param_4, local_402), uVar8);
            pass1_1010_dc36(param_1, uVar12, puVar5, param_2, param_3, param_4);
        }
    }
    return;
}


void  pass1_1010_d710(param_1: u32, param_2: u32, mut param_3: *mut u16, mut param_4: *mut u8, param_5: u8)

{
    u32          uVar1;
    long         lVar2;
    u16         *puVar3;
     let mut pcVar4: String;
    i16          iVar5;
     let mut uVar6: u16;
     let mut in_DX: u16;
     let mut uVar7: u16;
    i16          unaff_SI;
    i16          iVar8;
    astruct_496 *iVar9;
     let mut uVar9: u16;
     let mut uVar10: u16;
     let mut uVar11: u16;
    u32   uVar12;
     let mut uVar13: u16;
     let mut uVar14: u16;
     let mut uStack322: u16;
    i16          iStack316;
    i16          iStack314;
    i16          iStack312;
    u16          local_136[0x4a];
    u32   local_a2;
    i16          iStack14;
    u32   uStack12;
    u16         *puStack8;
    i16          iStack4;

    iStack4 = 0x0;
    do
    {
        uVar9                          = (param_2 >> 0x10);
        iVar8                          = param_2;
        uVar10                         = (param_3 >> 0x10);
        iVar9                          = (astruct_496 *)param_3;
        puVar3                         = iVar9->field_0x2;
        (iStack4 * 0xa + puVar3 + 0x4) = (iStack4 * 0x2 + iVar8);
        iStack4                        = iStack4 + 0x1;
    } while(iStack4 < 0x4);
    puStack8 = iVar9->field_0x2;
    iStack4  = 0x0;
    do
    {
        uVar1  = iVar9->field_0x6;
        pcVar4 = pass1_1010_b038(param_1, uVar1, (uVar1 >> 0x10), (puStack8 + 0x4), unaff_SI);
        string_1040_a626(puStack8, CONCAT22(in_DX, pcVar4), in_DX);
        iStack4  = iStack4 + 0x1;
        puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
    } while(iStack4 < 0x4);
    uVar13 = param_1;
    uVar14 = (param_1 >> 0x10);
    struct_1010_dd5e(uVar13, uVar14, iVar9->field_0x6);
    uStack12 = CONCAT22(in_DX, pcVar4);
    in_DX    = in_DX | pcVar4;
    if(in_DX != 0x0)
    {
        iStack14 = 0x0;
        pass1_1000_4906((Struct20 *)CONCAT22(param_4, &local_a2), 0x0, 0x94);
        pass1_1000_4906((Struct20 *)CONCAT22(param_4, local_136), 0x0, 0x94);
        iStack314 = 0x0;
        iStack312 = 0x0;
        iStack316 = 0x0;
        uVar1     = iVar9->field_0x6;
        lVar2     = (uVar1 + 0x26);
        for(uStack322 = 0x1; uStack322 < 0x25; uStack322 = uStack322 + 0x1)
        {
            if((uStack322 * 0x4 + uStack12) != 0x0)
            {
                if(iStack14 == 0x0)
                {
                    iStack14 = 0x1;
                }
                pcVar4 = string_1020_c0d8(uStack322);
                uVar7  = in_DX | pcVar4;
                if(uVar7 == 0x0)
                {
                    unk_str_op_1000_3d3e((&local_a2)[iStack312], s_Null_Ptr_1050_392c);
                }
                else
                {
                    uVar6                               = str_op_1008_60e8(CONCAT22(in_DX, pcVar4), uVar7);
                    (&local_a2 + iStack312)             = uVar6;
                    (&local_a2 + iStack312 * 0x4 + 0x2) = uVar7;
                }
                uVar11                           = (uStack12 >> 0x10);
                uVar7                            = (uStack322 * 0x4 + uStack12);
                in_DX                            = (uStack322 * 0x4 + uStack12 + 0x2);
                local_136[iStack312 * 0x2]       = uVar7;
                local_136[iStack312 * 0x2 + 0x1] = in_DX;
                iStack312                        = iStack312 + 0x1;
                if(lVar2 == 0x0)
                {
                    iVar5 = 0x0;
                }
                else
                {
                    uVar12 = pass1_1020_bae6(lVar2, CONCAT22(uStack322, (lVar2 >> 0x10)), uVar7, in_DX, param_4);
                    in_DX  = (uVar12 >> 0x10);
                    iVar5  = uVar12;
                }
                if(iVar5 == 0x0)
                {
                    iStack316 = iStack316 + 0x2;
                }
                else
                {
                    (uVar13 + iStack314 * 0x2 + 0xa4)         = (iVar8 + iStack316 * 0x2 + 0x8);
                    (uVar13 + (iStack314 + 0x1) * 0x2 + 0xa4) = (iVar8 + (iStack316 + 0x1) * 0x2 + 0x8);
                    iStack316                                 = iStack316 + 0x2;
                    iStack314                                 = iStack314 + 0x2;
                }
            }
        }
        uVar7 = pass1_1010_db2e(uVar13, uVar14, 0x4, CONCAT22(param_4, &local_a2), CONCAT22(param_4, local_136), param_2, param_3, param_4, param_5);
        if(iStack14 != 0x0)
        {
            iVar9->field_0x16 = 0x1;
        }
        while(iStack312 != 0x0)
        {
            fn_ptr_1000_17ce((Struct18 *)(&local_a2)[iStack312 + -0x1], 0x1000);
            iStack312 = iStack312 + -0x1;
        }
        pass1_1010_dc36(uVar13, uVar14, uVar7, param_2, param_3, param_4);
    }
    return;
}


void  pass1_1010_d984(param_1: u16, param_2: u16, i16 *param_3, param_4: i16, param_5: u32, param_6: u32, param_7: u32, mut param_8: *mut u8, param_9: u8)

{
     let mut puVar1: *mut u8;
     let mut pcVar2: String;
    i16        iVar3;
     let mut extraout_DX: u16;
     let mut uVar4: u16;
     let mut uVar5: u16;
    i16        iVar6;
    i16        iVar7;
     let mut uVar8: u16;
     let mut uVar9: u16;
    u16       *puStack1052;
    u8         local_418[0x400];
     let mut uStack24: u16;
     let mut pcStack22: String;
     let mut uStack18: u16;
    u32 uStack16;
    u8         local_c[0x8];
    i16        iStack4;

    iStack4 = param_4;
    pass1_1008_5784(CONCAT22(param_8, local_c), param_5);
    do
    {
        puVar1 = local_c;
        pass1_1008_5b12(puVar1, param_8);
        uStack16 = CONCAT22(extraout_DX, puVar1);
        uVar4    = extraout_DX | puVar1;
        if(uVar4 == 0x0)
        {
            return;
        }
        uStack18  = (puVar1 + 0xa);
        pcStack22 = 0x0;
        if((puVar1 + 0x4) == 0x0)
        {
            if((puVar1 + 0x6) == 0x0)
            {
                if((puVar1 + 0x8) == 0x0)
                {
                    return;
                }
                pcVar2 = string_op_1020_c2f8((puVar1 + 0x8));
            }
            else
            {
                pcVar2 = string_op_1020_c222((puVar1 + 0x6));
            }
        }
        else
        {
            pcVar2 = string_1020_c0d8((puVar1 + 0x4));
        }
        pcStack22     = CONCAT22(uVar4, pcVar2);
        uStack24      = (uStack16 + 0xc);
        *param_3      = *param_3 + uStack24;
        uVar8         = (param_7 >> 0x10);
        iVar6         = param_7;
        uVar5         = (iVar6 + 0x4);
        iVar3         = (iVar6 + 0x2) + iStack4 * 0xa;
        puStack1052   = CONCAT22(uVar5, iVar3);
        uVar9         = (param_6 >> 0x10);
        iVar7         = param_6;
        (iVar3 + 0x4) = (iStack4 * 0x2 + iVar7);
        sys_1000_3f9c(local_418, param_8, 0x3935, &USHORT_1050_1050, uStack18, &stack0xfffe, uVar5, 0x1000, param_8, param_9);
        string_1040_a626(puStack1052, CONCAT22(param_8, local_418), uVar5);
        uVar5         = (iVar6 + 0x4);
        iStack4       = iStack4 + 0x1;
        iVar3         = (iVar6 + 0x2) + iStack4 * 0xa;
        puStack1052   = CONCAT22(uVar5, iVar3);
        (iVar3 + 0x4) = (iStack4 * 0x2 + iVar7);
        unk_str_op_1000_3d3e(CONCAT22(param_8, local_418), pcStack22);
        string_1040_a626(puStack1052, CONCAT22(param_8, local_418), uVar5);
        iVar3         = (iStack4 + 0x1) * 0xa + (iVar6 + 0x2);
        uVar5         = (iVar6 + 0x4);
        puStack1052   = CONCAT22(uVar5, iVar3);
        (iVar3 + 0x4) = ((iStack4 + 0x1) * 0x2 + iVar7);
        iStack4       = iStack4 + 0x2;
        sys_1000_3f9c(local_418, param_8, 0x3938, &USHORT_1050_1050, uStack24, &stack0xfffe, uVar5, 0x1000, param_8, param_9);
        string_1040_a626(puStack1052, CONCAT22(param_8, local_418), uVar5);
    } while(true);
}

u16  pass1_1010_db2e(param_1: u16, param_2: u16, param_3: u16, param_4: u32, param_5: u32, param_6: u32, i16 *param_7, mut param_8: *mut u8, param_9: u8)

{
     let mut uVar1: u16;
    astruct_493 *iVar2;
    i16          iVar3;
     let mut uVar4: u16;
    astruct_492 *iVar4;
     let mut uVar5: u16;
     let mut uVar6: u16;
     let mut uVar7: u16;
     let mut uStack94: u16;
    i16          iStack92;
     let mut uStack90: u16;
    u16         *puStack88;
    u8           local_54[0x52];

    uStack94 = param_3;
    uStack90 = param_3;
    iStack92 = 0x0;
    while(true)
    {
        uVar7 = (param_7 >> 0x10);
        iVar4 = (astruct_492 *)param_7;
        if(*param_7 - 0x1U < uStack94)
        {
            return uStack94;
        }
        uVar1 = iVar4->field_0x4;
        iVar2 = (astruct_493 *)(iVar4->field_0x2 + uStack94 * 0xa);
        uVar5 = (param_5 >> 0x10);
        uVar6 = (param_4 >> 0x10);
        if(((iStack92 * 0x4 + param_5) == 0x0) && ((iStack92 * 0x4 + param_4) == 0x0))
            break;
        uVar4 = uVar1;
        unk_str_op_1000_3d3e(CONCAT22(param_8, local_54), (iStack92 * 0x4 + param_4));
        uVar6            = (param_6 >> 0x10);
        iVar2->field_0x4 = (uStack90 * 0x2 + param_6);
        string_1040_a626(CONCAT22(uVar1, iVar2), CONCAT22(param_8, local_54), uVar4);
        sys_1000_3f9c(local_54, param_8, 0x393b, &USHORT_1050_1050, *(param_5 + iStack92 * 0x4), &stack0xfffe, uVar5, 0x1000, param_8, param_9);
        uVar1         = iVar4->field_0x4;
        iVar3         = iVar4->field_0x2 + (uStack94 + 0x1) * 0xa;
        puStack88     = CONCAT22(uVar1, iVar3);
        (iVar3 + 0x4) = ((uStack90 + 0x1) * 0x2 + param_6);
        string_1040_a626(puStack88, CONCAT22(param_8, local_54), uVar1);
        uStack94 = uStack94 + 0x2;
        uStack90 = uStack90 + 0x2;
        iStack92 = iStack92 + 0x1;
    }
    return uStack94;
}

i16  string_1010_dcac(param_1: u16, param_2: u16, param_3: u16, param_4: i16, param_5: u32, astruct_104 *param_6)

{
    u32          uVar1;
    astruct_105 *iVar2;
     let mut uVar2: u16;
     let mut uVar3: u16;
    astruct_104 *iVar5;
     let mut uVar6: u16;
     let mut uVar7: u16;
     let mut pcVar4: String;

    pcVar4           = load_string_1010_847e(_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), param_1);
    uVar6            = (param_6 >> 0x10);
    iVar5            = (astruct_104 *)param_6;
    uVar2            = (&iVar5->field_0x2 + 0x2);
    iVar2            = (astruct_105 *)(&iVar5->field_0x2 + param_4 * 0xa);
    uVar7            = (param_5 >> 0x10);
    iVar2->field_0x4 = (param_4 * 0x2 + param_5);
    string_1040_a626(CONCAT22(uVar2, iVar2), pcVar4, uVar2);
    unk_str_op_1000_3d3e(pcVar4, 0x10503941);
    uVar2         = param_4 + 0x1;
    uVar1         = iVar5->field_0x2;
    uVar3         = uVar1 + uVar2 * 0xa;
    (uVar3 + 0x4) = (uVar2 * 0x2 + param_5);
    string_1040_a626((uVar1 & 0xffff0000 | uVar3), pcVar4, uVar2);
    return uVar2;
}

void  pass1_1010_debe(param_1: u32, param_2: u16, mut param_3: *mut u16, u32 *param_4, param_5: u32, param_6: u16)

{
     let mut bVar1: u8;
     let mut uVar2: u16;
     let mut uVar3: u16;
    i16  iVar4;
     let mut uVar5: u16;
     let mut puVar6: *mut u8;
    i16  iVar7;
    i16  unaff_DI;
     let mut uVar8: u16;
    u32  uVar9;
    u16 *puVar10;
     let mut uVar11: u16;
    i16  iStack34;
     let mut uStack30: u16;
    i16  iStack26;
    i16  iStack24;
    i16  iStack22;
    i16  iStack20;

    *param_4 = 0x0;
    *param_3 = 0x0;
    uVar9    = struct_op_1030_73a8(param_5);
    puVar6   = (uVar9 >> 0x10);
    iVar4    = (uVar9 + 0x12);
    uVar5    = param_1;
    uVar11   = (param_1 >> 0x10);
    uVar2    = pass1_1010_b028(uVar5, uVar11, uVar9);
    puVar10  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x35, param_6, puVar6, unaff_DI);
    puVar6   = (puVar10 >> 0x10);
    iVar7    = param_4;
    uVar8    = (param_4 >> 0x10);
    if(param_2 == 0x13)
    {
        iStack34 = 0x0;
        while(iStack34 = iStack34 + 0x1, iStack34 < 0x43)
        {
            param_2 = pass1_1010_ac62(uVar5, uVar11, iStack34, param_2, puVar6);
            if(param_2 != 0x0)
            {
                *param_3 = *param_3 + 0x1;
            }
        }
        uVar2 = *param_3 * 0x2;
        mem_op_1000_179c(uVar2, puVar6, 0x1000);
        param_4       = uVar2;
        (iVar7 + 0x2) = puVar6;
        if((puVar6 | param_4) != 0x0)
        {
            iStack34 = 0x0;
            for(uStack30 = 0x0; uVar2 = uStack30, *param_3 != uStack30 && uStack30 <= *param_3; uStack30 = uStack30 + 0x1)
            {
                do
                {
                    iStack34 = iStack34 + 0x1;
                    if(0x42 < iStack34)
                        goto LAB_1010_e0d4;
                    uVar2 = pass1_1010_ac62(uVar5, uVar11, iStack34, uVar2, puVar6);
                } while(uVar2 == 0x0);
                (uStack30 * 0x2 + *param_4) = iStack34;
            LAB_1010_e0d4:
            }
        }
    }
    else
    {
        if(param_2 < 0x14)
        {
            if(param_2 == '\x06')
            {
                if(((iVar4 == 0x5) || (iVar4 == 0x6)) || (iVar4 == 0x8))
                {
                    uVar3 = puVar10 + 0x11e;
                    if(uVar2 == 0xf)
                    {
                        iStack20 = 0xf;
                        iStack22 = 0x13;
                    }
                    else
                    {
                        if(uVar2 == 0xe)
                        {
                            iStack22 = 0x4;
                            iStack20 = 0x1;
                        }
                        else
                        {
                            iStack22 = 0xe;
                            iStack20 = 0x1;
                        }
                    }
                    iVar4    = pass1_1010_e128(uVar5, uVar11, iStack22, iStack20, puVar10 & 0xffff0000 | uVar3);
                    *param_3 = iVar4 + 0x1U;
                    if(iVar4 + 0x1U != 0x0)
                    {
                        uVar2 = *param_3 * 0x2;
                        mem_op_1000_179c(uVar2, puVar6, 0x1000);
                        param_4       = uVar2;
                        (iVar7 + 0x2) = puVar6;
                        iStack24      = 0x0;
                        for(iStack26 = iStack20; iStack26 <= iStack22; iStack26 = iStack26 + 0x1)
                        {
                            if((iStack26 * 0x2 + uVar3) != 0x0)
                            {
                                (*param_4 + iStack24 * 0x2) = iStack26;
                                iStack24                    = iStack24 + 0x1;
                            }
                        }
                        (*param_4 + iStack24 * 0x2) = 0x14;
                        return;
                    }
                }
            }
            else
            {
                bVar1 = param_2 - 0x7;
                if((bVar1 == 0x0) && (((iVar4 == 0x5 || (iVar4 == 0x6)) || (iVar4 == 0x8))))
                {
                    uVar5    = pass1_1010_ac62(uVar5, uVar11, 0x7, param_2 & 0xff00 | bVar1, puVar6);
                    uVar2    = -(uVar5 == 0x0) + 0x10;
                    *param_3 = uVar2;
                    uVar2    = uVar2 * 0x2;
                    mem_op_1000_179c(uVar2, puVar6, 0x1000);
                    param_4       = uVar2;
                    (iVar7 + 0x2) = puVar6;
                    if((puVar6 | param_4) == 0x0)
                    {
                        *param_3 = 0x0;
                        return;
                    }
                    for(iStack26 = 0x0; iStack26 < (-(uVar5 == 0x0) + 0xf); iStack26 = iStack26 + 0x1)
                    {
                        (iStack26 * 0x2 + *param_4) = iStack26 + 0x1;
                    }
                    (iStack26 * 0x2 + *param_4) = 0x10;
                    return;
                }
            }
        }
    }
    return;
}

void  pass1_1010_e58a(param_1: u32, param_2: u32, mut param_3: *mut u8, param_4: i16, param_5: u16)

{
    code      **ppcVar1;
     let mut uVar2: u16;
    BOOL16      BVar3;
    u32 *puVar4;
     let mut extraout_DX: u16;
     let mut uVar5: u16;
     let mut extraout_DX_00: u16;
     let mut extraout_DX_01: u16;
    i16         iVar6;
    short       in_buf_len_5;
     let mut uVar7: u16;
    u32 *puVar8;

    in_buf_len_5     = (short)(param_1 >> 0x10);
    iVar6            = param_1;
    *(iVar6 + 0x13c) = 0x0;
    uVar7            = (param_2 >> 0x10);
    puVar4           = (param_2 + 0x20);
    uVar7            = (param_2 + 0xc);
    BVar3            = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar7, 0x11);
    if(BVar3 == 0x0)
    {
        BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar7, 0x12);
        if(BVar3 == 0x0)
        {
            return;
        }
        puVar8  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x31, param_5, param_3, param_4);
        ppcVar1 = (*puVar8 + 0x14);
        (**ppcVar1)(0x1008, puVar8, (puVar8 >> 0x10), puVar4, puVar4 >> 0xf);
        uVar5 = extraout_DX_01 | puVar4;
        uVar2 = extraout_DX_01;
    }
    else
    {
        puVar8  = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x41, param_5, param_3, param_4);
        ppcVar1 = (*puVar8 + 0x14);
        (**ppcVar1)(0x1008, puVar8, (puVar8 >> 0x10), puVar4, puVar4 >> 0xf);
        uVar5 = extraout_DX | puVar4;
        uVar2 = extraout_DX;
    }
    if(uVar5 == 0x0)
    {
        load_string_1010_84e0(0x1008, globals._PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, (iVar6 + 0x13c), in_buf_len_5);
    }
    else
    {
        ppcVar1 = (*puVar4 + 0x14);
        (**ppcVar1)(0x1008, puVar4, uVar2);
        unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (iVar6 + 0x13c)), CONCAT22(extraout_DX_00, puVar4));
    }
    return;
}

void  pass1_1010_e682(param_1: u32, param_2: u32, param_3: u16, param_4: u8)

{
     let mut uVar1: u16;
    BOOL16     BVar2;
    u32        uVar3;
     let mut uVar4: u16;
     let mut uVar5: u16;
     let mut in_buf_len_5: *mut u8;
     let mut uVar6: u16;
    u32 uVar7;
     let mut uVar8: u16;
     let mut uVar9: u16;
     let mut local_1e: u16;
     let mut uStack28: u16;
     let mut local_1a: u16;
     let mut uStack24: u16;
     let mut uStack22: u16;
    u32 uStack20;
    u32        uStack16;
    u32        uStack12;
     let mut uStack8: u16;
    u32        uStack6;

    in_buf_len_5     = (param_1 >> 0x10);
    uVar5            = param_1;
    *(uVar5 + 0x13c) = 0x0;
    uStack6          = struct_op_1030_73a8(param_2);
    uVar6            = (uStack6 >> 0x10);
    uStack8          = (uStack6 + 0xc);
    uVar4            = uVar6;
    uVar1            = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0x1);
    if(((uVar1 == 0x0) && (uVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0x13), uVar1 == 0x0)) && (uVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0x2), uVar1 == 0x0))
    {
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0xe);
        if(BVar2 != 0x0)
        {
            uVar7    = (uVar5 + 0x138);
            uVar3    = *(uVar7 + 0x24);
            uStack16 = uVar3;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, (uVar3 >> 0x10));
            uStack12 = uVar3 & 0xffff | uVar4 << 0x10;
            uStack20 = (uVar3 + 0x1f6);
            uVar6    = (uStack20 >> 0x10);
            uVar8    = (uStack20 + 0x1a8);
            uVar9    = 0x3947;
            uStack22 = uVar8;
        LAB_1010_e76d:
            sys_1000_3f9c((uVar5 + 0x13c), in_buf_len_5, uVar9, &USHORT_1050_1050, uVar8, &stack0xfffe, uVar6, 0x1000, param_3, param_4);
            return;
        }
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0x5);
        if((BVar2 == 0x0) && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0x6), BVar2 == 0x0))
        {
            BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0x10);
            if(BVar2 == 0x0)
            {
                local_1e = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0xc);
                if((local_1e == 0x0) && (local_1e = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0x14), local_1e == 0x0))
                {
                    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0xa);
                    if(BVar2 == 0x0)
                    {
                        uVar8 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uStack8, 0x1e);
                        if(uVar8 == 0x0)
                        {
                            load_string_1010_84e0(0x1008, globals._PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, (uVar5 + 0x13c), (short)in_buf_len_5);
                            return;
                        }
                        pass1_1030_6ddc(param_2);
                        uVar9    = 0x395c;
                        local_1e = uVar8;
                        goto LAB_1010_e76d;
                    }
                    uVar7    = pass1_1030_7c28(param_2, 0x21, BVar2, uVar4, param_3);
                    uStack28 = (uVar7 >> 0x10);
                    uVar1    = uVar7;
                    uVar8    = 0x3958;
                    local_1e = uVar1;
                }
                else
                {
                    pass1_1010_e8f6(uVar5, in_buf_len_5, param_2, param_3);
                    uStack28 = uVar4;
                    uVar7    = pass1_1030_7c28(CONCAT22(uVar4, local_1e), 0x23, local_1e, uVar4, param_3);
                    uStack24 = (uVar7 >> 0x10);
                    uVar1    = uVar7;
                    uVar8    = 0x3954;
                    local_1a = uVar1;
                }
            }
            else
            {
                uVar7    = pass1_1030_7c28(param_2, 0x1e, BVar2, uVar4, param_3);
                uStack28 = (uVar7 >> 0x10);
                uVar1    = uVar7;
                uVar8    = 0x3950;
                local_1e = uVar1;
            }
        }
        else
        {
            local_1e = 0x0;
            local_1a = 0x0;
            pass1_1010_e8d0(uVar5, in_buf_len_5, CONCAT22(param_3, &local_1a), CONCAT22(param_3, &local_1e), param_2, &local_1e);
            uVar8 = 0x394a;
            uVar1 = local_1e;
        }
    }
    else
    {
        pass1_1010_e8f6(uVar5, in_buf_len_5, param_2, param_3);
        uStack12 = CONCAT22(uVar4, uVar1);
        pass1_1030_70f4(CONCAT22(uVar4, uVar1));
        uStack16 = CONCAT22(uVar4, uVar1);
        uVar8    = 0x3943;
    }
    sys_1000_3f9c((uVar5 + 0x13c), in_buf_len_5, uVar8, &USHORT_1050_1050, uVar1, &stack0xfffe, uVar6, 0x1000, param_3, param_4);
    return;
}

void  pass1_1010_bffa(param_1: u32, param_2: i16, mut param_3: *mut u8, param_4: u16)

{
    u16         *puVar1;
    u32   uVar2;
     let mut uVar3: u16;
     let mut uVar4: u16;
    astruct_257 *puVar5;
    astruct_254 *iVar6;
    astruct_255 *iVar7;
    astruct_256 *iVar8;
    i16          iVar5;
     let mut uVar6: u16;
     let mut uVar7: u16;
    astruct_43  *paVar8;
     let mut uStack34: u16;
    i16          iStack30;
    u8           local_18[0x16];

    mem_op_1000_179c(0xa, param_3, 0x1000);
    local_18._18_4_ = CONCAT22(param_3, param_2);
    bad_1010_bf08(param_1, (param_1 >> 0x10), 0x2000000);
    uVar6            = (local_18._18_4_ >> 0x10);
    iVar6            = (astruct_254 *)local_18._18_4_;
    iVar6->field_0x8 = param_2;
    if(param_2 == 0x0)
    {
        iVar6->field_0x8 = 0x1;
    }
    uVar3 = iVar6->field_0x8 << 0x2;
    mem_op_1000_179c(uVar3, param_3, 0x1000);
    uVar6            = (local_18._18_4_ >> 0x10);
    iVar7            = (astruct_255 *)local_18._18_4_;
    local_18._18_4_  = uVar3;
    iVar7->field_0x2 = param_3;
    if((param_3 | local_18._18_4_) == 0x0)
    {
        iVar7->field_0x8 = 0x0;
    }
    else
    {
        uVar4 = iVar7->field_0x8 * 0x2;
        mem_op_1000_179c(uVar4, param_3, 0x1000);
        uVar6            = (local_18._18_4_ >> 0x10);
        iVar8            = (astruct_256 *)local_18._18_4_;
        iVar8->field_0x4 = uVar4;
        iVar8->field_0x6 = param_3;
        if((param_3 | iVar8->field_0x4) != 0x0)
        {
            paVar8                  = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x1b4, param_4);
            uVar3                   = (paVar8 >> 0x10);
            puVar1                  = *local_18._18_4_;
            *puVar1                 = paVar8;
            (puVar1 + 0x2)          = uVar3;
            (local_18._18_4_ + 0x4) = 0x0;
            pass1_1028_dc52((astruct_92 *)CONCAT22(param_4, local_18), 0x1, 0x0, 0x200);
            iStack30 = 0x1;
            while(true)
            {
                puVar5 = (astruct_257 *)local_18;
                pass1_1028_e4ec(CONCAT22(param_4, puVar5));
                if((uVar3 | puVar5) == 0x0)
                    break;
                uVar6    = *puVar5->field_0x10;
                uStack34 = 0x0;
                switch(uVar6)
                {
                case 0x1:
                    uStack34 = 0x1b5;
                    break;
                case 0x2:
                    uStack34 = 0x1b6;
                    break;
                case 0x3:
                    uStack34 = 0x1b7;
                    break;
                case 0x4:
                    uStack34 = 0x1b8;
                    break;
                case 0x5:
                    uStack34 = 0x1b9;
                    break;
                case 0x6:
                    uStack34 = 0x1ba;
                    break;
                case 0x7:
                    uStack34 = 0x1bb;
                    break;
                case 0x8:
                    uStack34 = 0x1bc;
                    break;
                case 0x9:
                    uStack34 = 0x1bd;
                    break;
                case 0xa:
                    uStack34 = 0x1be;
                }
                paVar8                         = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, uStack34, param_4);
                uVar3                          = (paVar8 >> 0x10);
                uVar7                          = (*local_18._18_4_ >> 0x10);
                iVar5                          = *local_18._18_4_;
                (iVar5 + iStack30 * 0x4)       = paVar8;
                (iVar5 + iStack30 * 0x4 + 0x2) = uVar3;
                uVar2                          = (local_18._18_4_ + 0x4);
                (uVar2 + iStack30 * 0x2)       = uVar6;
                iStack30                       = iStack30 + 0x1;
            }
            return;
        }
    }
    return;
}

void  pass1_1010_c28a(mut param_1: *mut u8, param_2: i16, param_3: u16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
     let mut uVar3: u16;
     let mut uVar4: u16;
    u16 *puVar5;

    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, param_3, param_1, param_2);
    uVar2  = (puVar5 >> 0x10);
    uVar1  = (puVar5 + 0x24);
    uVar4  = (puVar5 + 0x26);
    uVar3  = uVar4 | uVar1;
    if(uVar3 != 0x0)
    {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, uVar4);
        if((uVar4 | uVar3) != 0x0)
        {
            return;
        }
    }
    return;
}
lobals starting with '_' overlap smaller symbols at the same address

  void
  pass1_1010_c320(param_1: u16, param_2: u16, mut param_3: *mut u8, param_4: u32, mut param_5: *mut u8)

{
    u32 uVar1;
     let mut puStack6: *mut u8;

    puStack6 = param_3;
    if(param_3 == 0x0)
    {
        mem_op_1000_179c(0x100, param_5, 0x1000);
        puStack6 = (param_3 & 0xffff | ZEXT24(param_5) << 0x10);
    }
    uVar1 = struct_op_1030_73a8(param_4);
    switch((uVar1 + 0x12))
    {
    case 0x1:
    case 0x2:
    case 0x4:
        break;
    case 0x3:
    case 0x5:
        break;
    case 0x6:
        break;
    case 0x7:
        break;
    case 0x8:
        break;
    case 0x9:
        break;
    default:
        *puStack6 = '\0';
        return;
    }
    load_string_1010_84e0(0x1010, globals._PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, puStack6, (short)(puStack6 >> 0x10));
    return;
}


void  pass1_1010_c3c2(param_1: u16, param_2: u16, param_3: u32, param_4: u32, mut param_5: *mut u8, param_6: u8, param_7: u16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
     let mut uVar3: u16;
    u8  local_40c[0x400];
     let mut uStack12: u16;
    u32 uStack10;
    u32 uStack6;

    uStack6 = param_3;
    if(param_3 == 0x0)
    {
        mem_op_1000_179c(0x100, param_5, 0x1000);
        uStack6 = param_3 & 0xffff | ZEXT24(param_5) << 0x10;
    }
    uStack10 = struct_op_1030_73a8(param_4);
    uVar2    = (uStack10 >> 0x10);
    uStack12 = (uStack10 + 0xc);
    uVar3    = uVar2;
    uVar1    = pass1_1020_bd80(uStack12);
    unk_str_op_1000_3d3e(CONCAT22(param_7, local_40c), CONCAT22(uVar3, uVar1));
    pass1_1030_8086(param_4);
    sys_1000_3f9c(uStack6, (uStack6 >> 0x10), 0x38c5, &USHORT_1050_1050, local_40c, &stack0xfffe, uVar2, 0x1000, param_7, param_6);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  string_op_1010_c446(param_1: u16, param_2: u8, mut param_3: *mut u8, param_4: u32, param_5: &mut String, param_6: u32)

{
    i16   iVar1;
     let mut uVar2: u16;
     let mut uVar3: u16;
    u32   uVar4;
     let mut pcVar5: String;
     let mut uVar6: u16;
     let mut uVar7: u16;
     let mut in_buffer_4: String;
     let mut in_buf_len_5: *mut u8;
     let mut uStack22: u16;
     let mut pcStack6: String;

    pcStack6 = param_5;
    if(param_5 == 0x0)
    {
        mem_op_1000_179c(0x100, param_3, 0x1000);
        pcStack6 = (param_5 & 0xffff | ZEXT24(param_3) << 0x10);
    }
    uVar4 = struct_op_1030_73a8(param_6);
    uVar2 = (uVar4 >> 0x10);
    uVar3 = uVar2;
    struct_1010_dd5e(param_4, (param_4 >> 0x10), param_6);
    iVar1 = (uVar4 + 0x12);
    if(0x6 < iVar1 - 0x3U)
    {
        return;
    }
    in_buffer_4  = pcStack6;
    in_buf_len_5 = (pcStack6 >> 0x10);
    uVar7        = globals._PTR_LOOP_1050_14cc;
    uVar6        = (_PTR_LOOP_1050_14cc >> 0x10);
    switch(iVar1)
    {
    default:
        break;
    case 0x6:
        load_string_1010_84e0(0x1010, uVar7, uVar6, 0x3ff, in_buffer_4, (short)in_buf_len_5);
        uStack22 = str_op_1000_3da4(pcStack6);
        pcVar5   = load_string_1010_847e(_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x1000);
        uVar7    = pcVar5;
        uVar6    = s_____s__lu_1050_38d7;
        goto LAB_1010_c4f9;
    case 0x7:
    case 0x9:
        break;
    case 0x8:
        load_string_1010_84e0(0x1010, uVar7, uVar6, 0x3ff, in_buffer_4, (short)in_buf_len_5);
        uStack22 = str_op_1000_3da4(pcStack6);
        pcVar5   = load_string_1010_847e(_PTR_LOOP_1050_14cc, (_PTR_LOOP_1050_14cc >> 0x10), 0x1000);
        uVar7    = pcVar5;
        uVar6    = s_____s__lu_1050_38cd;
    LAB_1010_c4f9:
        sys_1000_3f9c((in_buffer_4 + uStack22), in_buf_len_5, uVar6, &USHORT_1050_1050, uVar7, &stack0xfffe, uVar3, 0x1000, param_1, param_2);
        return;
    }
    load_string_1010_84e0(0x1010, uVar7, uVar6, 0x3ff, in_buffer_4, (short)in_buf_len_5);
    return;
}


void  pass1_1010_c58as(param_1: u16, param_2: u16, param_3: u16, param_4: u32, param_5: u16, param_6: u16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
    u32  uVar3;
     let mut puVar4: *mut u8;
     let mut puVar5: *mut u8;
     let mut unaff_SS: u16;
     let mut piStack26: *mut i16;
     let mut uStack18: u16;
    i16  iStack16;
    u8 **ppuStack14;
     let mut piStack6: *mut i16;

    uVar3 = param_5;
    mem_op_1000_179c(0x18, param_6, 0x1000);
    uVar1  = uVar3;
    puVar4 = (param_6 | uVar1);
    if(puVar4 == 0x0)
    {
        uVar1  = 0x0;
        puVar4 = 0x0;
    }
    else
    {
        struct_1040_a598((uVar3 & 0xffff | param_6 << 0x10));
    }
    piStack6 = CONCAT22(puVar4, uVar1);
    puVar5   = (puVar4 | uVar1);
    if(puVar5 == 0x0)
    {
        return;
    }
    ppuStack14 = (u8 **)0x0;
    uStack18   = 0x0;
    iStack16   = 0x0;
    switch(param_3)
    {
    case 0x5:
        ppuStack14 = (u8 **)&USHORT_1050_352c;
        uStack18   = 0xfa4;
        iStack16   = 0x30;
        break;
    default:
        if(piStack6 == 0x0)
        {
            return;
        }
        pass1_1040_a5d0(CONCAT22(puVar4, uVar1));
        fn_ptr_1000_17ce((Struct18 *)CONCAT22(puVar4, uVar1), 0x1000);
        return;
    case 0xa:
        ppuStack14 = (u8 **)&USHORT_1050_358c;
        uStack18   = 0xfb3;
        iStack16   = 0x51;
        break;
    case 0xb:
        ppuStack14 = (u8 **)&USHORT_1050_358c;
        uStack18   = 0xfb4;
        iStack16   = 0x51;
        break;
    case 0xc:
        ppuStack14 = (u8 **)&USHORT_1050_362e;
        uStack18   = 0xfb6;
        iStack16   = 0x30;
        break;
    case 0x10:
        ppuStack14 = &PTR_DAT_1050_1805_1050_368e;
        uStack18   = 0xfc4;
        iStack16   = 0x3c;
        break;
    case 0x11:
        ppuStack14 = &PTR_DAT_1050_1805_1050_3706;
        uStack18   = 0xfc1;
        iStack16   = 0x4b;
        break;
    case 0x12:
        ppuStack14 = (u8 **)&USHORT_1050_379c;
        uStack18   = 0xfc5;
        iStack16   = 0x8;
        break;
    case 0x13:
        puVar5 = puVar4;
        pass1_1010_debe(CONCAT22(param_2, param_1), param_3, CONCAT22(puVar4, uVar1 + 0x10), CONCAT22(puVar4, uVar1 + 0xc), param_4, unaff_SS);
        ppuStack14 = (u8 **)&USHORT_1050_37ac;
        uStack18   = 0xfc6;
        iStack16   = 0x1;
        break;
    case 0x15:
        *(uVar1 + 0x6) = param_4;
        (uVar1 + 0xa)  = param_3;
        break;
    case 0x16:
        ppuStack14 = (u8 **)&USHORT_1050_37ae;
        uStack18   = 0x157;
        iStack16   = 0x4;
        break;
    case 0x17:
        ppuStack14 = (u8 **)&USHORT_1050_37b6;
        iStack16   = 0x2c;
        uStack18   = 0xfd8;
    }
    if(ppuStack14 != (u8 **)0x0)
    {
        *piStack6 = iStack16;
        uVar2     = iStack16 * 0xa + 0x2;
        mem_op_1000_179c(uVar2, puVar5, 0x1000);
        piStack26 = CONCAT22(puVar5, uVar2);
        if((puVar5 | uVar2) == 0x0)
        {
            (uVar1 + 0x2) = 0x0;
        }
        else
        {
            *piStack26 = iStack16;
            pass1_1000_5586(0xa564, &PTR_LOOP_1050_1040, iStack16, 0xa, uVar2 + 0x2, puVar5);
            (uVar1 + 0x2) = uVar2 + 0x2;
            (uVar1 + 0x4) = puVar5;
        }
        *(uVar1 + 0x6) = param_4;
        (uVar1 + 0xa)  = param_3;
        (uVar1 + 0x12) = uStack18;
        pass1_1010_a50c((Struct20 *)CONCAT22(param_2, param_1), ppuStack14, CONCAT22(puVar4, uVar1));
    }
    return;
}


void  pass1_1010_c7e2(param_1: u32, param_2: u32, i16 *param_3)

{
    u32 uVar1;
     let mut pcVar2: String;
     let mut in_DX: u16;
    i16        iVar3;
    i16        unaff_SI;
     let mut uVar4: u16;
    u16       *puStack8;
    i16        iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar4 = (param_3 >> 0x10);
        iVar3 = param_3;
        if(*param_3 == iStack4 || *param_3 < iStack4)
            break;
        uVar1                         = (iVar3 + 0x2);
        (iStack4 * 0xa + uVar1 + 0x4) = (iStack4 * 0x2 + param_2);
        iStack4                       = iStack4 + 0x1;
    }
    puStack8 = *(u16 **)(iVar3 + 0x2);
    for(iStack4 = 0x0; *param_3 != iStack4 && iStack4 <= *param_3; iStack4 = iStack4 + 0x1)
    {
        uVar1  = (iVar3 + 0x6);
        pcVar2 = pass1_1010_b038(param_1, uVar1, (uVar1 >> 0x10), (puStack8 + 0x4), unaff_SI);
        string_1040_a626(puStack8, CONCAT22(in_DX, pcVar2), in_DX);
        puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
    }
    return;
}


void  pass1_1010_c864(param_1: u32, mut param_2: *mut u16, astruct_104 *param_3, mut param_4: *mut u8, mut param_5: *mut u8, param_6: u8)

{
    long       *plVar1;
    long        lVar2;
    code      **ppcVar3;
    u32  uVar4;
     let mut pcVar5: String;
     let mut uVar6: u16;
     let mut uVar7: u16;
    u32         uVar8;
     let mut uVar9: u16;
     let mut extraout_DX: u16;
     let mut extraout_DX_00: u16;
     let mut uVar10: u16;
    i16         iVar11;
    i16         iVar12;
     let mut uVar13: u16;
     let mut uVar14: u16;
     let mut uVar15: u16;
     let mut uVar16: u16;
     let mut uVar17: u16;
    u32  local_1f0;
    Struct18 *paStack412;
    u32  uStack408;
    u32  uStack404;
     let mut uStack400: u16;
    u32  uStack398;
    u32  local_18a;
    u32  local_f6;
    u16        *puStack98;
    i16         iStack94;
    u32         uStack92;
    i16         iStack88;
     let mut uStack86: u16;
    u8          local_54[0x52];

    uStack86 = 0x0;
    uVar13   = (param_3 >> 0x10);
    iVar11   = param_3;
    iStack88 = param_3;
    uVar14   = 0x0;
    uStack92 = 0x0;
    uVar16   = param_1;
    uVar17   = (param_1 >> 0x10);
    pass1_1010_c320(uVar16, uVar17, 0x0, *(iVar11 + 0x6), param_4);
    unk_str_op_1000_3d3e(CONCAT22(param_5, local_54), CONCAT22(param_4, uVar14));
    puStack98         = *(u16 **)(iVar11 + 0x2);
    uStack404._2_2_   = (iVar11 + 0x4);
    (puStack98 + 0x4) = *param_2;
    string_1040_a626(puStack98, CONCAT22(param_5, local_54), uStack404._2_2_);
    iStack94 = 0x0;
    pass1_1000_4906((Struct20 *)CONCAT22(param_5, &local_f6), 0x0, 0x94);
    uStack404._0_2_ = pass1_1000_4906((Struct20 *)CONCAT22(param_5, &local_18a), 0x0, 0x94);
    uStack398       = 0x0;
    for(uStack400 = 0x1; uStack400 < 0x25; uStack400 = uStack400 + 0x1)
    {
        uStack404       = (Struct18 *)pass1_1030_7c28(*(iVar11 + 0x6), uStack400, uStack404, uStack404._2_2_, param_5);
        uStack404._2_2_ = (uStack404 >> 0x10) | uStack404;
        if(uStack404 != (Struct18 *)0x0)
        {
            pcVar5    = string_1020_c0d8(uStack400);
            uStack408 = CONCAT22(uStack404._2_2_, pcVar5);
            uVar9     = uStack404._2_2_ | pcVar5;
            if(uVar9 == 0x0)
            {
                unk_str_op_1000_3d3e((&local_f6)[uStack398], s_Null_Ptr_1050_38e1);
            }
            else
            {
                uVar6                               = str_op_1008_60e8(CONCAT22(uStack404._2_2_, pcVar5), uVar9);
                (&local_f6 + uStack398)             = uVar6;
                (&local_f6 + uStack398 * 0x4 + 0x2) = uVar9;
            }
            *(u16 **)(&local_18a + uStack398)    = uStack404;
            (&local_18a + uStack398 * 0x4 + 0x2) = uStack404._2_2_;
            uStack398                            = uStack398 + 0x1;
        }
    }
    uStack92 = uStack398;
    if(0x13 < uStack398)
    {
        iStack94 = 0x1;
    }
    uStack86 = pass1_1010_db2e(uVar16, uVar17, 0x1, CONCAT22(param_5, &local_f6), CONCAT22(param_5, &local_18a), param_2, param_3, param_5, param_6);
    while(uVar8 = uStack398 - 0x1, uStack398 != 0x0)
    {
        uStack398._0_2_ = uVar8;
        paStack412      = (Struct18 *)(&local_f6)[uStack398];
        uStack404       = paStack412;
        uStack398       = uVar8;
        fn_ptr_1000_17ce(paStack412, 0x1000);
    }
    uVar15    = 0x1000;
    uStack398 = uVar8;
    pass1_1000_4906((Struct20 *)CONCAT22(param_5, &local_18a), 0x0, 0x54);
    uVar4     = (iVar11 + 0x6);
    uVar14    = (uVar4 >> 0x10);
    iVar12    = uVar4;
    uStack404 = (Struct18 *)(iVar12 + 0x1e);
    uVar9     = (iVar12 + 0x20) | uStack404;
    uVar8     = uVar9;
    if(uVar9 != 0x0)
    {
        uStack398 = 0x0;
        while(true)
        {
            uVar4   = uStack404;
            ppcVar3 = (uVar4 + 0x10);
            (**ppcVar3)(uVar15, uStack404, (uStack404 >> 0x10));
            if((extraout_DX < uStack398._2_2_) || ((extraout_DX <= uStack398._2_2_ && (uVar8 <= uStack398))))
                break;
            ppcVar3 = (uVar4 + 0x4);
            (**ppcVar3)(uVar15, uStack404, uStack398, uStack398._2_2_);
            uVar9  = uVar8;
            uVar10 = extraout_DX_00 | uVar9;
            if(uVar10 != 0x0)
            {
                uVar15 = SUB42(&USHORT_1050_1028, 0x0);
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar9, extraout_DX_00);
                uStack408 = CONCAT22(uVar10, uVar9);
                if((uVar10 | uVar9) == 0x0)
                {
                    return;
                }
                iVar12 = (uVar9 + 0xc);
                uVar8  = (iVar12 - 0x1U);
                if(((0x0 < iVar12) && (!SBORROW2(iVar12, 0x1))) && (uVar8 = (iVar12 - 0x6U), iVar12 - 0x6U == 0x0 || (iVar12 - 0x1U) < 0x5))
                {
                    plVar1  = &local_18a + iVar12;
                    *plVar1 = *plVar1 + 0x1;
                }
            }
            uStack398 = uStack398 + 0x1;
        }
        uVar9 = extraout_DX;
        pass1_1000_4906((Struct20 *)CONCAT22(param_5, &local_f6), 0x0, 0x54);
        uVar6 = 0x1000;
        pass1_1000_4906((Struct20 *)CONCAT22(param_5, &local_1f0), 0x0, 0x54);
        uStack398 = 0x0;
        for(uStack400 = 0x1; uStack400 < 0x15; uStack400 = uStack400 + 0x1)
        {
            if((&local_18a)[uStack400] != 0x0)
            {
                pcVar5 = string_op_1020_c222(uStack400);
                uVar10 = uVar9 | pcVar5;
                if(uVar10 == 0x0)
                {
                    uVar6 = 0x1000;
                    unk_str_op_1000_3d3e((&local_f6)[uStack398], s_Null_Ptr_1050_38ea);
                }
                else
                {
                    uVar6                               = 0x1008;
                    uVar7                               = str_op_1008_60e8(CONCAT22(uVar9, pcVar5), uVar10);
                    (&local_f6 + uStack398)             = uVar7;
                    (&local_f6 + uStack398 * 0x4 + 0x2) = uVar10;
                }
                uVar9                                = (&local_18a + uStack400 * 0x4 + 0x2);
                (&local_1f0 + uStack398)             = (&local_18a + uStack400);
                (&local_1f0 + uStack398 * 0x4 + 0x2) = uVar9;
                uStack398                            = uStack398 + 0x1;
            }
        }
        if(iStack94 == 0x0)
        {
            iVar12   = (uStack92 >> 0x10) + CARRY2(uStack92, uStack398);
            uStack92 = CONCAT22(iVar12, uStack92 + uStack398);
            if((iVar12 != 0x0) || (0x13 < uStack92 + uStack398))
            {
                iStack94 = 0x1;
            }
        }
        if((uStack86 < iStack88 - 0x2U) && (local_1f0 != 0x0))
        {
            iVar12   = string_1010_dcac(uVar6, uVar16, uVar17, uStack86, param_2, param_3);
            uStack86 = iVar12 + 0x1;
            uStack86 = pass1_1010_db2e(uVar16, uVar17, uStack86, CONCAT22(param_5, &local_f6), CONCAT22(param_5, &local_1f0), param_2, param_3, param_5, param_6);
        }
        while(lVar2 = uStack398 + -0x1, uStack398 != 0x0)
        {
            uStack398._0_2_ = lVar2;
            paStack412      = (Struct18 *)(&local_f6)[uStack398];
            uStack398       = lVar2;
            fn_ptr_1000_17ce(paStack412, 0x1000);
        }
        if(iStack94 != 0x0)
        {
            (iVar11 + 0x16) = 0x1;
        }
        uStack398 = lVar2;
        pass1_1010_dc36(uVar16, uVar17, uStack86, param_2, param_3, param_5);
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1010_cc56(param_1: u32, param_2: u32, astruct_104 *param_3, param_4: u16, mut param_5: *mut u8, param_6: u8)

{
    long      *plVar1;
    u32 uVar2;
     let mut pcVar3: String;
     let mut uVar4: u16;
     let mut uVar5: u16;
    i16        iVar6;
    u32        uVar7;
     let mut uVar8: u16;
     let mut uVar9: u16;
     let mut uVar10: u16;
    u32 local_1a0;
    u32 uStack332;
     let mut uStack328: u16;
     let mut uStack326: u16;
    u32 uStack324;
     let mut uStack320: u16;
    u32 local_13e;
    u32 local_aa;
     let mut uStack22: u16;
    i16        iStack18;
     let mut uStack16: u16;
    i16        iStack14;
     let mut uStack12: u16;
    u32        uStack10;
    u32        uStack6;

    uVar10  = (param_1 >> 0x10);
    uVar9   = param_1;
    uVar2   = (uVar9 + 0x138);
    uVar7   = *(uVar2 + 0x24);
    uStack6 = uVar7;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar7, (uVar7 >> 0x10));
    uStack10        = uVar7 & 0xffff | param_4 << 0x10;
    uStack324._2_2_ = param_4 | uVar7;
    if(uStack324._2_2_ != 0x0)
    {
        iStack14 = param_3;
        iStack18 = 0x0;
        pass1_1000_4906((Struct20 *)CONCAT22(param_5, &local_aa), 0x0, 0x94);
        pass1_1000_4906((Struct20 *)CONCAT22(param_5, &local_13e), 0x0, 0x94);
        uStack12 = 0x0;
        uStack16 = 0x0;
        uStack22 = 0x0;
        for(uStack320 = 0x1; uStack320 < 0x25; uStack320 = uStack320 + 0x1)
        {
            uStack324 = (uStack10 + 0x26 + uStack320 * 0x4);
            if(uStack324 != 0x0)
            {
                pcVar3    = string_1020_c0d8(uStack320);
                uStack332 = uStack332 & 0xffff | ZEXT24(pcVar3) << 0x10;
                uVar8     = uStack324._2_2_ | pcVar3;
                uStack328 = uStack324._2_2_;
                if(uVar8 == 0x0)
                {
                    unk_str_op_1000_3d3e((&local_aa)[uStack22], s_Null_Ptr_1050_38f3);
                }
                else
                {
                    uVar4                              = str_op_1008_60e8(CONCAT22(uStack324._2_2_, pcVar3), uVar8);
                    (&local_aa + uStack22)             = uVar4;
                    (&local_aa + uStack22 * 0x4 + 0x2) = uVar8;
                }
                (&local_13e + uStack22)             = uStack324;
                (&local_13e + uStack22 * 0x4 + 0x2) = uStack324._2_2_;
                uStack22                            = uStack22 + 0x1;
            }
        }
        uStack16 = uStack22;
        if(0x13 < uStack22)
        {
            iStack18 = 0x1;
        }
        uStack12 = pass1_1010_db2e(uVar9, uVar10, 0x1, CONCAT22(param_5, &local_aa), CONCAT22(param_5, &local_13e), param_2, param_3, param_5, param_6);
        pass1_1000_4906((Struct20 *)CONCAT22(param_5, &local_13e), 0x0, 0x54);
        for(uStack332._0_2_ = 0x1; uStack332 < 0x15; uStack332._0_2_ = uStack332 + 0x1)
        {
            uStack326 = uStack332;
            if((uStack10 + 0x14e + uStack332 * 0x4) != 0x0)
            {
                if(((0x0 < uStack332) && (!SBORROW2(uStack332, 0x1))) && ((uStack332 - 0x1) < 0x6))
                {
                    plVar1  = &local_13e + uStack332;
                    *plVar1 = *plVar1 + 0x1;
                }
            }
        }
        pass1_1000_4906((Struct20 *)CONCAT22(param_5, &local_aa), 0x0, 0x54);
        uVar4 = 0x1000;
        pass1_1000_4906((Struct20 *)CONCAT22(param_5, &local_1a0), 0x0, 0x54);
        for(uStack332 = 0x10000; uStack332._2_2_ < 0x15; uStack332 = uStack332 & 0xffff | (uStack332._2_2_ + 0x1) << 0x10)
        {
            if((&local_13e)[uStack332._2_2_] != 0x0)
            {
                pcVar3    = string_op_1020_c222(uStack332._2_2_);
                uStack324 = CONCAT22(uStack324._2_2_, pcVar3);
                uVar8     = uStack324._2_2_ | pcVar3;
                if(uVar8 == 0x0)
                {
                    uVar4 = 0x1000;
                    unk_str_op_1000_3d3e((&local_aa)[uStack332], s_Null_Ptr_1050_38fc);
                }
                else
                {
                    uVar4                               = 0x1008;
                    uVar5                               = str_op_1008_60e8(CONCAT22(uStack324._2_2_, pcVar3), uVar8);
                    (&local_aa + uStack332)             = uVar5;
                    (&local_aa + uStack332 * 0x4 + 0x2) = uVar8;
                }
                uStack324._2_2_                      = (&local_13e + uStack332._2_2_ * 0x4 + 0x2);
                (&local_1a0 + uStack332)             = (&local_13e + uStack332._2_2_);
                (&local_1a0 + uStack332 * 0x4 + 0x2) = uStack324._2_2_;
                uStack332                            = uStack332 & 0xffff0000 | (uStack332 + 0x1);
            }
        }
        if(iStack18 == 0x0)
        {
            uStack16 = uStack16 + uStack332;
            if(0x13 < uStack16)
            {
                iStack18 = 0x1;
            }
        }
        if((uStack12 < iStack14 - 0x2U) && (local_1a0 != 0x0))
        {
            iVar6    = string_1010_dcac(uVar4, uVar9, uVar10, uStack12, param_2, param_3);
            uStack12 = iVar6 + 0x1;
            uStack12 = pass1_1010_db2e(uVar9, uVar10, uStack12, CONCAT22(param_5, &local_aa), CONCAT22(param_5, &local_1a0), param_2, param_3, param_5, param_6);
        }
        if(iStack18 != 0x0)
        {
            (param_3 + 0x16) = 0x1;
        }
        pass1_1010_dc36(uVar9, uVar10, uStack12, param_2, param_3, param_5);
    }
    return;
}

void  pass1_1010_cf36(param_1: u32, param_2: u32, mut param_3: *mut u16, param_4: u8, mut param_5: *mut u8)

{
    u32          uVar1;
    u16         *puVar2;
     let mut pcVar3: String;
     let mut uVar4: u16;
     let mut uVar5: u16;
     let mut in_DX: u16;
     let mut uVar6: u16;
    i16          unaff_SI;
    i16          iVar7;
    astruct_494 *iVar9;
     let mut uVar8: u16;
     let mut uVar9: u16;
     let mut uVar10: u16;
    u32   uVar11;
    u32          uVar12;
     let mut uVar13: u16;
     let mut uVar14: u16;
     let mut uVar15: u16;
     let mut uStack326: u16;
    i16          iStack316;
     let mut uStack314: u16;
    i16          iStack312;
    u16          local_136[0x4a];
    u32   local_a2;
    i16          iStack14;
    u32   uStack12;
    u16         *puStack8;
    i16          iStack4;

    iStack4 = 0x0;
    do
    {
        uVar8                          = (param_2 >> 0x10);
        iVar7                          = param_2;
        uVar9                          = (param_3 >> 0x10);
        iVar9                          = (astruct_494 *)param_3;
        puVar2                         = iVar9->field_0x2;
        (iStack4 * 0xa + puVar2 + 0x4) = (iStack4 * 0x2 + iVar7);
        iStack4                        = iStack4 + 0x1;
    } while(iStack4 < 0x8);
    puStack8 = iVar9->field_0x2;
    iStack4  = 0x0;
    do
    {
        uVar1  = iVar9->field_0x6;
        pcVar3 = pass1_1010_b038(param_1, uVar1, (uVar1 >> 0x10), (puStack8 + 0x4), unaff_SI);
        string_1040_a626(puStack8, CONCAT22(in_DX, pcVar3), in_DX);
        iStack4  = iStack4 + 0x1;
        puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
    } while(iStack4 < 0x8);
    uVar13 = param_1;
    uVar14 = (param_1 >> 0x10);
    struct_1010_dd5e(uVar13, uVar14, iVar9->field_0x6);
    uStack12 = CONCAT22(in_DX, pcVar3);
    in_DX    = in_DX | pcVar3;
    if(in_DX != 0x0)
    {
        iStack14 = 0x0;
        pass1_1000_4906((Struct20 *)CONCAT22(param_5, &local_a2), 0x0, 0x94);
        pass1_1000_4906((Struct20 *)CONCAT22(param_5, local_136), 0x0, 0x94);
        uStack314 = 0x0;
        iStack312 = 0x0;
        iStack316 = 0x0;
        uVar1     = iVar9->field_0x6;
        uVar1     = *(uVar1 + 0x26);
        uVar12    = uVar1;
        for(uStack326 = 0x1; uStack326 < 0x25; uStack326 = uStack326 + 0x1)
        {
            uVar15 = (uVar1 >> 0x10);
            if((uStack326 * 0x4 + uStack12) == 0x0)
            {
                if(uVar1 != 0x0)
                {
                    uVar12 = pass1_1020_bae6(uVar1, CONCAT22(uStack326, uVar15), uVar12, in_DX, param_5);
                    uVar6  = (uVar12 >> 0x10);
                    uVar12 = uVar12 & 0xffff;
                    in_DX  = uVar6 | uVar12;
                    if(in_DX != 0x0)
                    {
                        if(iStack14 == 0x0)
                        {
                            iStack14 = 0x1;
                        }
                        pcVar3 = string_1020_c0d8(uStack326);
                        uVar4  = in_DX | pcVar3;
                        if(uVar4 == 0x0)
                        {
                            unk_str_op_1000_3d3e((&local_a2)[iStack312], s_Null_Ptr_1050_390e);
                        }
                        else
                        {
                            uVar5                               = str_op_1008_60e8(CONCAT22(in_DX, pcVar3), uVar4);
                            (&local_a2 + iStack312)             = uVar5;
                            (&local_a2 + iStack312 * 0x4 + 0x2) = uVar4;
                        }
                        local_136[iStack312 * 0x2]       = uVar12;
                        local_136[iStack312 * 0x2 + 0x1] = uVar6;
                        goto LAB_1010_d11d;
                    }
                }
            }
            else
            {
                if(iStack14 == 0x0)
                {
                    iStack14 = 0x1;
                }
                pcVar3 = string_1020_c0d8(uStack326);
                uVar6  = in_DX | pcVar3;
                if(uVar6 == 0x0)
                {
                    unk_str_op_1000_3d3e((&local_a2)[iStack312], s_Null_Ptr_1050_3905);
                }
                else
                {
                    uVar5                               = str_op_1008_60e8(CONCAT22(in_DX, pcVar3), uVar6);
                    (&local_a2 + iStack312)             = uVar5;
                    (&local_a2 + iStack312 * 0x4 + 0x2) = uVar6;
                }
                uVar10                           = (uStack12 >> 0x10);
                uVar4                            = (uStack326 * 0x4 + uStack12);
                uVar6                            = (uStack326 * 0x4 + uStack12 + 0x2);
                local_136[iStack312 * 0x2]       = uVar4;
                local_136[iStack312 * 0x2 + 0x1] = uVar6;
                if(uVar1 == 0x0)
                {
                    uVar4 = 0x0;
                }
                else
                {
                    uVar11 = pass1_1020_bae6(uVar1, CONCAT22(uStack326, uVar15), uVar4, uVar6, param_5);
                    uVar6  = (uVar11 >> 0x10);
                    uVar4  = uVar11;
                }
                uVar12 = uVar4;
                if(uVar4 == 0x0)
                {
                    iStack316 = iStack316 + 0x2;
                    in_DX     = uVar6;
                    iStack312 = iStack312 + 0x1;
                }
                else
                {
                LAB_1010_d11d:
                    iStack312                                 = iStack312 + 0x1;
                    (uVar13 + uStack314 * 0x2 + 0xa4)         = (iVar7 + iStack316 * 0x2 + 0x10);
                    (uVar13 + (uStack314 + 0x1) * 0x2 + 0xa4) = (iVar7 + (iStack316 + 0x1) * 0x2 + 0x10);
                    iStack316                                 = iStack316 + 0x2;
                    uStack314                                 = uStack314 + 0x2;
                    uVar12                                    = uStack314;
                    in_DX                                     = uVar6;
                }
            }
        }
        uVar6 = pass1_1010_db2e(uVar13, uVar14, 0x8, CONCAT22(param_5, &local_a2), CONCAT22(param_5, local_136), param_2, param_3, param_5, param_4);
        if(iStack14 != 0x0)
        {
            iVar9->field_0x16 = 0x1;
        }
        while(iStack312 != 0x0)
        {
            fn_ptr_1000_17ce((Struct18 *)(&local_a2)[iStack312 + -0x1], 0x1000);
            iStack312 = iStack312 + -0x1;
        }
        pass1_1010_dc36(uVar13, uVar14, uVar6, param_2, param_3, param_5);
    }
    return;
}


void  pass1_1010_d24a(param_1: u32, param_2: u32, mut param_3: *mut u16, mut param_4: *mut u8, param_5: u8)

{
    u32          uVar1;
    u16         *puVar2;
     let mut pcVar3: String;
    u16         *puVar4;
     let mut uVar5: u16;
     let mut in_DX: u16;
    u16         *puVar6;
    u16         *puVar7;
     let mut uVar8: u16;
    i16          unaff_SI;
    astruct_495 *iVar9;
     let mut uVar9: u16;
    long         lVar10;
     let mut uVar11: u16;
     let mut uVar12: u16;
     let mut uStack320: u16;
    long         lStack318;
    u16         *local_13a[0x4a];
    u32   local_a6;
    i16          iStack18;
    u32          uStack16;
     let mut pcStack12: String;
    u16         *puStack8;
    i16          iStack4;

    iStack4 = 0x0;
    do
    {
        uVar9                          = (param_3 >> 0x10);
        iVar9                          = (astruct_495 *)param_3;
        puVar2                         = iVar9->field_0x2;
        (iStack4 * 0xa + puVar2 + 0x4) = (iStack4 * 0x2 + param_2);
        iStack4                        = iStack4 + 0x1;
    } while(iStack4 < 0x8);
    puStack8 = iVar9->field_0x2;
    iStack4  = 0x0;
    do
    {
        uVar1  = iVar9->field_0x6;
        pcVar3 = pass1_1010_b038(param_1, uVar1, (uVar1 >> 0x10), (puStack8 + 0x4), unaff_SI);
        string_1040_a626(puStack8, CONCAT22(in_DX, pcVar3), in_DX);
        iStack4  = iStack4 + 0x1;
        puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
    } while(iStack4 < 0x8);
    uVar11 = param_1;
    uVar12 = (param_1 >> 0x10);
    struct_1010_dd5e(uVar11, uVar12, iVar9->field_0x6);
    puVar6 = (in_DX | pcVar3);
    if(puVar6 != 0x0)
    {
        pcStack12 = pcVar3;
        pass1_1010_e8f6(uVar11, uVar12, iVar9->field_0x6, param_4);
        uStack16 = CONCAT22(puVar6, pcVar3);
        iStack18 = 0x0;
        pass1_1000_4906((Struct20 *)CONCAT22(param_4, &local_a6), 0x0, 0x94);
        puVar4    = pass1_1000_4906((Struct20 *)CONCAT22(param_4, local_13a), 0x0, 0x94);
        lStack318 = 0x0;
        for(uStack320 = 0x1; uStack320 < 0x25; uStack320 = uStack320 + 0x1)
        {
            lVar10 = pass1_1030_7c28(uStack16, uStack320, puVar4, puVar6, param_4);
            puVar7 = (lVar10 >> 0x10);
            puVar4 = lVar10;
            puVar6 = (puVar7 | puVar4);
            if(lVar10 != 0x0)
            {
                if(iStack18 == 0x0)
                {
                    iStack18 = 0x1;
                }
                pcVar3 = string_1020_c0d8(uStack320);
                uVar8  = puVar6 | pcVar3;
                if(uVar8 == 0x0)
                {
                    unk_str_op_1000_3d3e((&local_a6)[lStack318], s_Null_Ptr_1050_3917);
                }
                else
                {
                    uVar5                               = str_op_1008_60e8(CONCAT22(puVar6, pcVar3), uVar8);
                    (&local_a6 + lStack318)             = uVar5;
                    (&local_a6 + lStack318 * 0x4 + 0x2) = uVar8;
                }
                local_13a[lStack318 * 0x2]       = puVar4;
                local_13a[lStack318 * 0x2 + 0x1] = puVar7;
                lStack318                        = lStack318 + 0x1;
                puVar6                           = puVar7;
            }
        }
        uVar8 = pass1_1010_db2e(uVar11, uVar12, 0x8, CONCAT22(param_4, &local_a6), CONCAT22(param_4, local_13a), param_2, param_3, param_4, param_5);
        if(iStack18 != 0x0)
        {
            iVar9->field_0x16 = 0x1;
        }
        while(lStack318 != 0x0)
        {
            lStack318._0_2_ = (lStack318 + -0x1);
            fn_ptr_1000_17ce((Struct18 *)(&local_a6)[lStack318], 0x1000);
            lStack318 = lStack318 + -0x1;
        }
        pass1_1010_dc36(uVar11, uVar12, uVar8, param_2, param_3, param_4);
    }
    return;
}

void  pass1_1010_9fee(param_1: u32, param_2: u16, param_3: u16, param_4: u16, mut param_5: *mut u8)

{
    code       **ppcVar1;
     let mut puVar2: *mut u8;
     let mut extraout_DX: *mut u8;
    astruct_252 *iVar3;
    astruct_253 *iVar4;
     let mut uVar3: u16;
     let mut uVar4: u16;
     let mut uVar5: u16;
    u16         *puStack10;
    u16         *puStack6;

    uVar3  = (param_1 >> 0x10);
    iVar3  = (astruct_252 *)param_1;
    puVar2 = param_5;
    if(iVar3->field_0x12 == 0x0)
    {
        mem_op_1000_179c(0xc, param_5, 0x1000);
        puVar2 = (param_5 | param_4);
        if(puVar2 == 0x0)
        {
            iVar3->field_0x12 = 0x0;
        }
        else
        {
            set_struct_1008_574a(CONCAT22(param_5, param_4));
            &iVar3->field_0x12         = param_4;
            (&iVar3->field_0x12 + 0x2) = extraout_DX;
            puVar2                     = extraout_DX;
        }
    }
    uVar5 = 0x8;
    mem_op_1000_179c(0x8, puVar2, 0x1000);
    puStack10 = CONCAT22(puVar2, param_4);
    if((puVar2 | param_4) == 0x0)
    {
        puStack6 = 0x0;
    }
    else
    {
        *puStack10      = 0x389a;
        (param_4 + 0x2) = 0x1008;
        *puStack10      = 0xa1c4;
        (param_4 + 0x2) = 0x1010;
        puStack6        = puStack10;
    }
    uVar4            = (puStack6 >> 0x10);
    iVar4            = (astruct_253 *)puStack6;
    iVar4->field_0x4 = param_3;
    iVar4->field_0x6 = param_2;
    ppcVar1          = (*iVar3->field_0x12 + 0x4);
    (**ppcVar1)(0x1000, iVar3->field_0x12, iVar4, uVar4, uVar5);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1010_a0a0(param_1: u32, mut param_2: *mut u8, param_3: i16, param_4: u16)

{
     let mut piVar1: *mut i16;
    i16  iVar2;
    i16  iVar3;
    i16  iVar4;
     let mut uVar5: u16;
     let mut uVar6: u16;
    bool bVar7;
    bool bVar8;
    long lVar9;
    i16  iStack12;
    u8   local_a[0x8];

    pass1_1008_5784(CONCAT22(param_4, local_a), *(param_1 + 0xa));
    iStack12 = 0x4;
    mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, param_4, param_2, param_3);
    if((PTR_LOOP_1050_13ae != &PTR_LOOP_1050_0002) && (PTR_LOOP_1050_13ae != (&PTR_LOOP_1050_0000 + 0x1)))
    {
        iStack12 = 0x2;
    }
    do
    {
        while(true)
        {
            lVar9 = pass1_1008_5b12(local_a, param_4);
            uVar6 = (lVar9 >> 0x10);
            iVar4 = lVar9;
            if(lVar9 == 0x0)
            {
                return;
            }
            iVar2 = (iVar4 + 0x4);
            if(iVar2 != 0x12)
                break;
            piVar1 = (iVar4 + 0x6);
            bVar8  = SBORROW2(*piVar1, 0x2);
            iVar3  = *piVar1 + -0x2;
            bVar7  = iVar3 == 0x0;
        LAB_1010_a151:
            if(!bVar7 && bVar8 == iVar3 < 0x0)
            {
            LAB_1010_a153:
                piVar1  = (iVar4 + 0x6);
                *piVar1 = *piVar1 - (iVar4 + 0x6) / iStack12;
            }
        }
        if(((iVar2 != 0x3e) && (iVar2 != 0x41)) && (iVar2 != 0x80))
        {
            if(iVar2 == 0x83)
            {
                piVar1 = (iVar4 + 0x6);
                bVar8  = SBORROW2(*piVar1, 0x1);
                iVar2  = *piVar1;
                iVar3  = iVar2 + -0x1;
                bVar7  = iVar2 == 0x1;
                goto LAB_1010_a151;
            }
            goto LAB_1010_a153;
        }
        iVar2   = (iVar4 + 0x6);
        uVar5   = iVar2 / 0x2;
        piVar1  = (iVar4 + 0x6);
        *piVar1 = *piVar1 - uVar5;
        if(uVar5 == 0x0)
        {
            uVar5 = 0x1;
        }
        pass1_1010_9fee(param_1, uVar5, (iVar4 + 0x4), uVar5, iVar2 >> 0xf);
    } while(true);
}

void  pass1_1010_a69c(param_1: u32, param_2: u16, param_3: i16, mut param_4: *mut u8, param_5: i16, param_6: u16)

{
    i16         iVar1;
     let mut uVar2: u16;
     let mut puVar3: *mut u8;
     let mut puVar4: *mut u8;
    astruct_25 *paVar5;
    astruct_67 *paVar6;
    u16        *puVar7;
     let mut uVar8: u16;
     let mut uVar9: u16;
     let mut uVar10: u16;
     let mut uStack22: u16;
    i16         iStack20;

    pass1_1030_8344(_PTR_LOOP_1050_5748, (_PTR_LOOP_1050_5748 >> 0x10), 0x8000001);
    if(param_2 == 0x1)
    {
        puVar3 = param_4;
        for(iStack20 = 0x0; iStack20 < 0x83; iStack20 = iStack20 + 0x1)
        {
            iVar1 = pass1_1030_2242(CONCAT22(param_4, param_3), iStack20);
            if(0x19 < iVar1)
            {
                uStack22 = iVar1 - 0x5;
                if(uStack22 < 0x19)
                {
                    uStack22 = 0x19;
                }
                pass1_1030_25d8(CONCAT22(param_4, param_3), uStack22, iStack20);
            }
        }
        goto switchD_1010_aaef_caseD_b;
    }
    puVar3 = param_4;
    pass1_1030_25f0(CONCAT22(param_4, param_3), 0x0, param_2);
    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x35, param_6, puVar3, param_5);
    puVar3 = (puVar7 >> 0x10);
    uVar2  = param_1;
    uVar8  = (param_1 >> 0x10);
    puVar4 = puVar3;
    switch(param_2)
    {
    case 0xa:
    case 0xc:
        iVar1 = 0x1b;
        break;
    default:
        goto switchD_1010_aaef_caseD_b;
    case 0x10:
        pass1_1010_682e(puVar7, 0x1, 0x2d);
        if((param_3 + 0x160) == 0x0)
            goto switchD_1010_aaef_caseD_b;
        iVar1 = 0x2d;
        goto LAB_1010_a91f;
    case 0x12:
        pass1_1010_682e(puVar7, 0x1, 0x16);
        pass1_1010_682e(puVar7, 0x1, 0x17);
        pass1_1010_682e(puVar7, 0x1, 0x18);
        pass1_1010_682e(puVar7, 0x1, 0x40);
        iVar1 = 0x3f;
        goto LAB_1010_a96c;
    case 0x13:
        iVar1 = 0x35;
        goto LAB_1010_a91f;
    case 0x19:
        goto switchD_1010_aaef_caseD_19;
    case 0x1a:
        iVar1 = 0xf;
        goto LAB_1010_a96c;
    case 0x1c:
        iVar1 = 0x11;
        goto LAB_1010_a96c;
    case 0x1d:
    case 0x24:
        pass1_1010_abd2(uVar2, uVar8, 0x1e, puVar3, param_5, param_6);
        iVar1 = 0x5b;
        goto LAB_1010_a91f;
    case 0x1e:
        uVar2  = 0x1;
        iVar1  = 0x2;
        puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_6, puVar3, param_5);
        puVar3 = (puVar7 >> 0x10);
        pass1_1010_08c0(puVar7, uVar2, iVar1, param_6);
        paVar5 = (astruct_25 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x40, param_6, puVar3, param_5);
        puVar3 = (paVar5 >> 0x10);
        load_str_and_spri16f_1008_b69c(paVar5, param_6, puVar3);
        goto switchD_1010_aaef_caseD_b;
    case 0x22:
        iVar1 = 0x8;
        goto LAB_1010_aabe;
    case 0x23:
        iVar1 = 0xc;
        goto LAB_1010_aabe;
    case 0x25:
        pass1_1010_abd2(uVar2, uVar8, 0x14, puVar3, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x1b, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x1e, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x22, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x25, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x28, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x2a, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x2d, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x2f, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x31, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x35, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x38, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x3a, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x3c, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x48, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x4f, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x52, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x54, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x57, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x5b, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x5d, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x62, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x66, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x68, puVar4, param_5, param_6);
        pass1_1010_abd2(uVar2, uVar8, 0x6c, puVar4, param_5, param_6);
        goto switchD_1010_aaef_caseD_19;
    case 0x29:
        iVar1 = 0x25;
        break;
    case 0x2a:
        iVar1 = 0xf;
        goto LAB_1010_aabe;
    case 0x2b:
        iVar1 = 0x6e;
        goto LAB_1010_a96c;
    case 0x30:
        iVar1 = 0x54;
        break;
    case 0x33:
        pass1_1010_abd2(uVar2, uVar8, 0x31, puVar3, param_5, param_6);
        iVar1 = 0x6c;
        goto LAB_1010_a91f;
    case 0x36:
        iVar1 = 0x13;
        goto LAB_1010_aabe;
    case 0x37:
        iVar1 = 0x2c;
    LAB_1010_a96c:
        pass1_1010_682e(puVar7, 0x1, iVar1);
        goto switchD_1010_aaef_caseD_b;
    case 0x38:
        pass1_1010_682e(puVar7, 0x1, 0x28);
        if((param_3 + 0x160) == 0x0)
            goto switchD_1010_aaef_caseD_b;
        iVar1 = 0x28;
        goto LAB_1010_a91f;
    case 0x39:
        iVar1 = 0x10;
        goto LAB_1010_aabe;
    case 0x3a:
        iVar1 = 0x11;
        goto LAB_1010_aabe;
    case 0x3b:
        iVar1 = 0x12;
    LAB_1010_aabe:
        pass1_1010_6814(puVar7, 0x1, iVar1);
        goto switchD_1010_aaef_caseD_b;
    case 0x3c:
        pass1_1010_abd2(uVar2, uVar8, 0x14, puVar3, param_5, param_6);
        iVar1 = 0x62;
        goto LAB_1010_a91f;
    case 0x3d:
        pass1_1010_682e(puVar7, 0x1, 0x66);
        if((param_3 + 0x160) == 0x0)
            goto switchD_1010_aaef_caseD_b;
        iVar1 = 0x66;
    LAB_1010_a91f:
        pass1_1010_abd2(uVar2, uVar8, iVar1, puVar3, param_5, param_6);
        goto switchD_1010_aaef_caseD_b;
    case 0x3e:
        iVar1 = 0x5d;
        break;
    case 0x3f:
        iVar1 = 0x22;
        break;
    case 0x40:
        iVar1 = 0x57;
        break;
    case 0x41:
        iVar1 = 0x4f;
    }
    pass1_1010_abd2(uVar2, uVar8, iVar1, puVar3, param_5, param_6);
switchD_1010_aaef_caseD_b:
    paVar6 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x37, param_6, puVar3, param_5);
    puVar3 = (paVar6 >> 0x10);
    uVar2  = pass1_1008_ab12(paVar6, puVar3, param_2);
    if(uVar2 != 0x0)
    {
        post_win_msg_1008_a0e4(paVar6, 0x0, 0x0, 0x1, 0x0, uVar2, 0x1008, param_6);
    }
    post_win_msg_1008_a0e4(paVar6, 0x0, 0x0, 0x1, 0x0, 0x3d, 0x1008, param_6);
    uVar10 = 0x400;
    iVar1  = 0x6;
    uVar9  = 0x1;
    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_6, puVar3, param_5);
    pass1_1010_043a(puVar7, CONCAT22(uVar10, uVar9), iVar1, param_6);
    return;
switchD_1010_aaef_caseD_19:
    (puVar7 + 0x148) = 0x34;
    puVar3           = puVar4;
    goto switchD_1010_aaef_caseD_b;
}
