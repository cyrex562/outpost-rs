
char *pass1_1000_472c(param_1: u32, char param_2)

{
     let mut pcVar1: String;
     let mut uVar2: u16;
     let mut pcVar3: String;
     let mut pcVar4: String;
     let mut uVar5: u16;
    bool  bVar6;

    uVar5  = (param_1 >> 0x10);
    pcVar3 = param_1;
    bVar6  = true;
    uVar2  = 0xffff;
    pcVar4 = pcVar3;
    do
    {
        if(uVar2 == 0x0)
            break;
        uVar2  = uVar2 - 0x1;
        pcVar1 = pcVar4;
        pcVar4 = pcVar4 + 0x1;
        bVar6  = *pcVar1 == '\0';
    } while(!bVar6);
    uVar2 = ~uVar2;
    do
    {
        if(uVar2 == 0x0)
            break;
        uVar2  = uVar2 - 0x1;
        pcVar1 = pcVar3;
        pcVar3 = pcVar3 + 0x1;
        bVar6  = param_2 == *pcVar1;
    } while(!bVar6);
    if(!bVar6)
    {
        if(param_2 != '\0')
        {
            return 0x0;
        }
        pcVar3 = pcVar3 + 0x1;
    }
    return pcVar3 + -0x1;
}


i16 pass1_1000_475e(param_1: u32, param_2: u32)

{
     let mut pcVar1: String;
     let mut cVar2: u8;
     let mut cVar3: u8;
     let mut bVar4: u8;
    astruct_235 *bVar3;
    astruct_236 *bVar5;
     let mut pcVar5: String;
     let mut pcVar6: String;

    pcVar6 = param_2;
    pcVar5 = param_1;
    bVar5  = (astruct_236 *)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x20);
    do
    {
        do
        {
            cVar3 = bVar5;
            if(cVar3 == '\0')
                goto LAB_1000_479d;
            pcVar1 = pcVar6;
            pcVar6 = pcVar6 + 0x1;
            cVar3  = *pcVar1;
            cVar2  = *pcVar5;
            bVar5  = (astruct_236 *)CONCAT11(cVar2, cVar3);
            pcVar5 = pcVar5 + 0x1;
        } while(cVar2 == cVar3);
        bVar4       = cVar3 + 0xbfU + (-((u8)(cVar3 + 0xbfU) < 0x1a) & 0x20U) + 0x41;
        bVar3._0_1_ = cVar2 + 0xbf;
        bVar5._0_1_ = (u8)bVar3 + (-((u8)bVar3 < 0x1a) & 0x20U) + 0x41;
        bVar5       = (astruct_236 *)CONCAT11(bVar4, (u8)bVar5);
    } while((u8)bVar5 == bVar4);
    cVar3 = ((u8)bVar5 < bVar4) * -0x2 + '\x01';
LAB_1000_479d:
    return cVar3;
}


u16 pass1_1000_47a4(param_1: u32, param_2: u32, param_3: u16)

{
     let mut pbVar1: *mut u8;
     let mut bVar2: u8;
    u16 *puVar3;
     let mut pbVar4: *mut u8;
    i16  iVar5;
     let mut pbVar6: *mut u8;
    u16 *puVar7;
     let mut uVar8: u16;
    u16  local_22[0x10];

    puVar7 = local_22;
    for(iVar5 = 0x10; iVar5 != 0x0; iVar5 = iVar5 + -0x1)
    {
        puVar3  = puVar7;
        puVar7  = puVar7 + 0x1;
        *puVar3 = 0x0;
    }
    pbVar6 = (u8 *)param_2;
    while(true)
    {
        pbVar1 = pbVar6;
        pbVar6 = pbVar6 + 0x1;
        bVar2  = *pbVar1;
        if(bVar2 == 0x0)
            break;
        pbVar1  = (u8 *)(local_22 + (bVar2 >> 0x3));
        *pbVar1 = *pbVar1 | '\x01' << (bVar2 & 0x7);
    }
    pbVar1 = (u8 *)param_1;
    if(param_1 == 0x0)
    {
        pbVar1 = pbRam105061e4;
    }
    do
    {
        pbRam105061e4 = pbVar1;
        uVar8         = (pbRam105061e4 >> 0x10);
        pbVar6        = (u8 *)(pbRam105061e4 + 0x1);
        bVar2         = *pbRam105061e4;
        if(bVar2 == 0x0)
        {
            return 0x0;
        }
        pbVar1 = (u8 *)(pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar6));
    } while(('\x01' << (bVar2 & 0x7) & *(u8 *)(local_22 + (bVar2 >> 0x3))) != 0x0);
    do
    {
        pbVar4 = pbVar6;
        bVar2  = *pbVar4;
        if(bVar2 == 0x0)
            goto LAB_1000_483c;
        pbVar6 = pbVar4 + 0x1;
    } while(('\x01' << (bVar2 & 0x7) & *(u8 *)(local_22 + (bVar2 >> 0x3))) == 0x0);
    *pbVar4 = 0x0;
    pbVar4  = pbVar4 + 0x1;
LAB_1000_483c:
    pbRam105061e4 = (u8 *)(pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar4));
    return pbRam105061e4;
}


u16 pass1_1000_484c(param_1: u32, param_2: u32, param_3: u16)

{
     let mut pbVar1: *mut u8;
     let mut pbVar2: *mut u8;
    i16  iVar3;
     let mut uVar4: u16;
     let mut uVar5: u16;
     let mut pbVar6: *mut u8;
     let mut pbVar7: *mut u8;
    i16  iVar8;
    bool bVar9;
    bool bVar10;

    if(param_3 == 0x0)
    {
        return 0x0;
    }
    do
    {
        iVar8   = (param_2 >> 0x10);
        pbVar7  = (u8 *)param_2;
        iVar3   = (param_1 >> 0x10);
        pbVar6  = (u8 *)param_1;
        uVar4   = ~pbVar7;
        uVar4   = ((param_3 - 0x1) - uVar4 & -(param_3 - 0x1 < uVar4)) + uVar4;
        uVar5   = ~pbVar6;
        uVar4   = (uVar4 - uVar5 & -(uVar4 < uVar5)) + uVar5 + 0x1;
        bVar9   = param_3 < uVar4;
        param_3 = param_3 - uVar4;
        bVar10  = param_3 == 0x0;
        do
        {
            if(uVar4 == 0x0)
                break;
            uVar4  = uVar4 - 0x1;
            pbVar2 = pbVar7;
            pbVar7 = pbVar7 + 0x1;
            pbVar1 = pbVar6;
            pbVar6 = pbVar6 + 0x1;
            bVar9  = *pbVar1 < *pbVar2;
            bVar10 = *pbVar1 == *pbVar2;
        } while(bVar10);
        param_2 = param_2 & 0xffff0000 | ZEXT24(pbVar7);
        if(!bVar10)
        {
            return (0x1 - bVar9) - (bVar9 != 0x0);
        }
        if(param_3 == 0x0)
        {
            return uVar4;
        }
        if(pbVar6 == (u8 *)0x0)
        {
            iVar3 = iVar3 + 0x6c;
        }
        param_1 = CONCAT22(iVar3, pbVar6);
        if(pbVar7 == (u8 *)0x0)
        {
            param_2 = (iVar8 + 0x6c) << 0x10;
            param_1 = CONCAT22(iVar3, pbVar6);
        }
    } while(true);
}


u16 pass1_1000_48a8(param_1: u32, param_2: u32, param_3: i16)

{
    u16 *puVar1;
    u16 *puVar2;
    i16  iVar3;
     let mut uVar4: u16;
     let mut uVar5: u16;
    u16 *puVar6;
    u16 *puVar7;
    i16  iVar8;

    if(param_3 != 0x0)
    {
        while(true)
        {
            iVar3   = (param_2 >> 0x10);
            puVar6  = param_2;
            iVar8   = (param_1 >> 0x10);
            puVar7  = param_1;
            uVar4   = ~puVar7;
            uVar4   = ((param_3 - 0x1U) - uVar4 & -(param_3 - 0x1U < uVar4)) + uVar4;
            uVar5   = ~puVar6;
            uVar4   = (uVar4 - uVar5 & -(uVar4 < uVar5)) + uVar5 + 0x1;
            param_3 = param_3 - uVar4;
            for(uVar5 = uVar4 >> 0x1; uVar5 != 0x0; uVar5 = uVar5 - 0x1)
            {
                puVar2  = puVar7;
                puVar7  = puVar7 + 0x1;
                puVar1  = puVar6;
                puVar6  = puVar6 + 0x1;
                *puVar2 = *puVar1;
            }
            for(uVar4 = ((uVar4 & 0x1) != 0x0); uVar4 != 0x0; uVar4 = uVar4 - 0x1)
            {
                puVar2  = puVar7;
                puVar7  = (puVar7 + 0x1);
                puVar1  = puVar6;
                puVar6  = (puVar6 + 0x1);
                *puVar2 = *puVar1;
            }
            if(param_3 == 0x0)
                break;
            if(puVar6 == 0x0)
            {
                iVar3 = iVar3 + 0x6c;
            }
            param_1 = param_1 & 0xffff0000 | ZEXT24(puVar7);
            param_2 = CONCAT22(iVar3, puVar6);
            if(puVar7 == 0x0)
            {
                param_1 = (iVar8 + 0x6c) << 0x10;
                param_2 = CONCAT22(iVar3, puVar6);
            }
        }
    }
    return param_1;
}


u16 *pass1_1000_4906(Struct20 *param_1, WNDCLASS16 *in_wnd_class, param_3: u16)

{
    u16 *puVar1;
     let mut uVar2: u8;
     let mut uVar3: u16;
     let mut uVar4: u16;
     let mut uVar5: u16;
     let mut uVar6: u16;
    u16 *puVar7;
    i16  iVar8;

    if(param_3 != 0x0)
    {
        iVar8 = (param_1 >> 0x10);
        uVar5 = -param_1;
        uVar6 = param_3;
        if(uVar5 != 0x0)
        {
            uVar6 = (uVar5 - param_3 & -(uVar5 < param_3)) + param_3;
            uVar5 = param_3 - uVar6;
        }
        uVar3  = in_wnd_class & 0xff | in_wnd_class << 0x8;
        puVar7 = param_1;
        for(uVar4 = uVar6 >> 0x1; uVar4 != 0x0; uVar4 = uVar4 - 0x1)
        {
            puVar1  = puVar7;
            puVar7  = puVar7 + 0x1;
            *puVar1 = uVar3;
        }
        for(uVar6 = ((uVar6 & 0x1) != 0x0); uVar2 = (undefined)(in_wnd_class & 0xff), uVar6 != 0x0; uVar6 = uVar6 - 0x1)
        {
            puVar1  = puVar7;
            puVar7  = (puVar7 + 0x1);
            *puVar1 = uVar2;
        }
        if(uVar5 != 0x0)
        {
            for(uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 = uVar6 - 0x1)
            {
                puVar1  = puVar7;
                puVar7  = puVar7 + 0x1;
                *puVar1 = uVar3;
            }
            for(uVar6 = ((uVar5 & 0x1) != 0x0); uVar6 != 0x0; uVar6 = uVar6 - 0x1)
            {
                puVar1  = puVar7;
                puVar7  = (puVar7 + 0x1);
                *puVar1 = uVar2;
            }
        }
    }
    return param_1;
}


i16 pass1_1000_49b2(param_1: u16)

{
    return (param_1 ^ param_1 >> 0xf) - (param_1 >> 0xf);
}


u16 pass1_1000_49c6(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16, param_6: u16, mut param_7: *mut u8, param_8: i16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
     let mut uVar3: u16;
     let mut uVar4: u16;
    i16  iVar5;
    i16  iVar6;
    long lVar7;
     let mut uStack20: u16;
     let mut uStack18: u16;
     let mut uStack8: u16;
     let mut uStack6: u16;

    uStack20 = param_3;
    uStack18 = param_4;
    lVar7    = pass1_1000_52be(param_5 - 0x1, -(param_5 == 0x0), param_6, 0x0);
    uStack8  = (lVar7 + 0x8);
    uStack6  = ((lVar7 + 0x8) >> 0x10) * 0x100 + param_4;
    while(true)
    {
        if(uStack6 < uStack18)
        {
            return 0x0;
        }
        if((uStack6 <= uStack18) && (uStack8 < uStack20))
        {
            return 0x0;
        }
        uVar1 = param_5 >> 0x1;
        if(uVar1 == 0x0)
        {
            if((param_5 != 0x0) && (iVar5 = (*(fn_ptr_1)param_7)(), iVar5 == 0x0))
            {
                return uStack20;
            }
            return 0x0;
        }
        uVar2 = uVar1;
        if((param_5 & 0x1) == 0x0)
        {
            uVar2 = uVar1 - 0x1;
        }
        uVar3 = (uVar2 * param_6);
        uVar4 = uVar3 + uStack20;
        iVar6 = ((uVar2 * param_6 >> 0x10) + CARRY2(uVar3, uStack20)) * 0x100 + uStack18;
        iVar5 = (*(fn_ptr_1)param_7)();
        if(iVar5 == 0x0)
            break;
        if(iVar5 < 0x0)
        {
            uStack8 = -param_6 + uVar4;
            uStack6 = (CARRY2(-param_6, uVar4) - (param_6 != 0x0)) * 0x100 + iVar6;
            uVar2   = param_5 & 0x1;
            param_5 = uVar1;
            if(uVar2 == 0x0)
            {
                param_5 = uVar1 - 0x1;
            }
        }
        else
        {
            uStack20 = param_6 + uVar4;
            uStack18 = CARRY2(param_6, uVar4) * 0x100 + iVar6;
            param_5  = uVar1;
        }
    }
    return uVar4;
}


void pass1_1000_4ceb(param_1: u16, param_2: i16, param_3: i16, param_4: u16)

{
     let mut puVar1: *mut u8;
    u16 *puVar2;
     let mut uVar3: u8;
     let mut uVar4: u16;

    if((param_1 & 0x1) != 0x0)
    {
        param_1              = param_1 - 0x1;
        puVar1               = (param_1 + param_3);
        uVar3                = *puVar1;
        *puVar1              = *(param_1 + param_2);
        *(param_1 + param_2) = uVar3;
        if(param_1 == 0x0)
        {
            return;
        }
    }
    do
    {
        param_1             = param_1 - 0x2;
        puVar2              = (param_1 + param_3);
        uVar4               = *puVar2;
        *puVar2             = (param_1 + param_2);
        (param_1 + param_2) = uVar4;
    } while(param_1 != 0x0);
    return;
}


void pass1_1000_4d0c(param_1: u16)

{
    DAT_1050_61e8               = param_1;
    globals.PTR_LOOP_1050_61ea = 0x0;
    return;
}


u16 pass1_1000_4d24(void)

{
    long lVar1;

    lVar1                       = pass1_1000_52be(DAT_1050_61e8, globals.PTR_LOOP_1050_61ea, s_TPPOPMENU_1050_43fa + 0x3, 0x3);
    globals.PTR_LOOP_1050_61ea = ((lVar1 + 0x269ec3) >> 0x10);
    DAT_1050_61e8               = (lVar1 + 0x269ec3);
    return globals.PTR_LOOP_1050_61ea & 0x7fff;
}


u16 pass1_1000_4f2e(param_1: u16)

{
    code *pcVar1;
     let mut uVar2: u16;
    bool  bVar3;

    bVar3  = false;
    pcVar1 = (fn_ptr_1)swi(0x21);
    uVar2  = (*pcVar1)(&USHORT_1050_1050, param_1 + 0x1);
    if(bVar3)
    {
        pass1_1000_29b5(uVar2);
        return 0xffff;
    }
    return 0x0;
}


void pass1_1000_5008(param_1: u16, param_2: u16, param_3: u16, param_4: i16)

{
     let mut unaff_CS: u16;
     let mut unaff_SS: u16;
    i16 iStack2;

    iStack2 = param_4 + 0x1;
    pass1_1000_5026(0x0, param_1, param_2, param_3, &iStack2, unaff_CS, unaff_SS);
    return;
}


i16 pass1_1000_3d7a(param_1: u32, param_2: u32)

{
     let mut pbVar1: *mut u8;
     let mut pcVar2: String;
     let mut pbVar3: *mut u8;
    i16   iVar4;
     let mut uVar5: u16;
     let mut pcVar6: String;
     let mut pbVar7: *mut u8;
     let mut pcVar8: String;
     let mut pbVar9: *mut u8;
     let mut uVar10: u16;
    bool  bVar11;
    bool  bVar12;

    pbVar7 = (u8 *)param_1;
    uVar10 = (param_2 >> 0x10);
    pcVar8 = param_2;
    iVar4  = 0x0;
    uVar5  = 0xffff;
    do
    {
        if(uVar5 == 0x0)
            break;
        uVar5  = uVar5 - 0x1;
        pcVar2 = pcVar8;
        pcVar8 = pcVar8 + 0x1;
    } while(*pcVar2 != '\0');
    pcVar6 = ~uVar5;
    bVar11 = pcVar8 < pcVar6;
    pbVar9 = (u8 *)(pcVar8 + -pcVar6);
    bVar12 = pbVar9 == (u8 *)0x0;
    do
    {
        if(pcVar6 == 0x0)
            break;
        pcVar6 = pcVar6 + -0x1;
        pbVar3 = pbVar9;
        pbVar9 = pbVar9 + 0x1;
        pbVar1 = pbVar7;
        pbVar7 = pbVar7 + 0x1;
        bVar11 = *pbVar1 < *pbVar3;
        bVar12 = *pbVar1 == *pbVar3;
    } while(bVar12);
    if(!bVar12)
    {
        iVar4 = (0x1 - bVar11) - (bVar11 != 0x0);
    }
    return iVar4;
}


u16 pass1_1000_3de8(param_1: &mut String, param_2: &mut String, param_3: u16, param_4: u16, param_5: u16)

{
     let mut pbVar1: *mut u8;
     let mut pcVar2: String;
     let mut pcVar3: String;
     let mut bVar4: u8;
     let mut uVar5: u16;
    i16   iVar6;
     let mut pcVar7: String;
     let mut pcVar8: String;
     let mut uVar9: u16;
     let mut uVar10: u16;
    bool  bVar11;

    if(param_3 != 0x0)
    {
        uVar9  = (param_1 >> 0x10);
        pcVar8 = param_1;
        uVar5  = param_3;
        pcVar7 = pcVar8;
        do
        {
            if(uVar5 == 0x0)
                break;
            uVar5  = uVar5 - 0x1;
            pcVar2 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
        } while(*pcVar2 != '\0');
        iVar6  = param_3 - uVar5;
        uVar10 = (param_2 >> 0x10);
        pcVar7 = param_2;
        do
        {
            if(iVar6 == 0x0)
                break;
            iVar6  = iVar6 + -0x1;
            pcVar3 = pcVar8;
            pcVar8 = pcVar8 + 0x1;
            pcVar2 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
        } while(*pcVar2 == *pcVar3);
        bVar4  = pcVar7[-0x1];
        uVar5  = 0x0;
        pbVar1 = (u8 *)(pcVar8 + -0x1);
        bVar11 = bVar4 == *pbVar1;
        if(bVar4 < *pbVar1 || bVar11)
        {
            if(bVar11)
            {
                return 0x0;
            }
            uVar5 = 0xfffe;
        }
        param_3 = ~uVar5;
    }
    return param_3;
}


i16 pass1_1000_3e2c(param_1: u32)

{
     let mut pbVar1: *mut u8;
     let mut bVar2: u8;
     let mut bVar3: u8;
    i16 iVar4;
     let mut pbVar5: *mut u8;
     let mut uVar6: u16;

    uVar6  = (param_1 >> 0x10);
    pbVar5 = (u8 *)param_1;
    iVar4  = 0x0;
    do
    {
        do
        {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 0x1;
            bVar2  = *pbVar1;
        } while(bVar2 == 0x20);
    } while(bVar2 == 0x9);
    if((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b))
        goto LAB_1000_3e4d;
    while(true)
    {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 0x1;
        bVar3  = *pbVar1;
    LAB_1000_3e4d:
        if((0x39 < bVar3) || (bVar3 < 0x30))
            break;
        iVar4 = iVar4 * 0xa + (u8)(bVar3 - 0x30);
    }
    if(bVar2 == 0x2d)
    {
        iVar4 = -iVar4;
    }
    return iVar4;
}


i16 pass1_1000_3e2c(param_1: u32)

{
     let mut pbVar1: *mut u8;
     let mut bVar2: u8;
     let mut bVar3: u8;
    i16 iVar4;
     let mut pbVar5: *mut u8;
     let mut uVar6: u16;

    uVar6  = (param_1 >> 0x10);
    pbVar5 = (u8 *)param_1;
    iVar4  = 0x0;
    do
    {
        do
        {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 0x1;
            bVar2  = *pbVar1;
        } while(bVar2 == 0x20);
    } while(bVar2 == 0x9);
    if((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b))
        goto LAB_1000_3e4d;
    while(true)
    {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 0x1;
        bVar3  = *pbVar1;
    LAB_1000_3e4d:
        if((0x39 < bVar3) || (bVar3 < 0x30))
            break;
        iVar4 = iVar4 * 0xa + (u8)(bVar3 - 0x30);
    }
    if(bVar2 == 0x2d)
    {
        iVar4 = -iVar4;
    }
    return iVar4;
}


i16 pass1_1000_3e2c(param_1: u32)

{
     let mut pbVar1: *mut u8;
     let mut bVar2: u8;
     let mut bVar3: u8;
    i16 iVar4;
     let mut pbVar5: *mut u8;
     let mut uVar6: u16;

    uVar6  = (param_1 >> 0x10);
    pbVar5 = (u8 *)param_1;
    iVar4  = 0x0;
    do
    {
        do
        {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 0x1;
            bVar2  = *pbVar1;
        } while(bVar2 == 0x20);
    } while(bVar2 == 0x9);
    if((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b))
        goto LAB_1000_3e4d;
    while(true)
    {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 0x1;
        bVar3  = *pbVar1;
    LAB_1000_3e4d:
        if((0x39 < bVar3) || (bVar3 < 0x30))
            break;
        iVar4 = iVar4 * 0xa + (u8)(bVar3 - 0x30);
    }
    if(bVar2 == 0x2d)
    {
        iVar4 = -iVar4;
    }
    return iVar4;
}


u8 *pass1_1000_3e82(param_1: u16, mut param_2: *mut u8, param_3: u16)

{
     let mut pbVar1: *mut u8;
    u32  uVar2;
     let mut bVar3: u8;
     let mut uVar5: u16;
     let mut uVar6: u16;
     let mut uVar7: u16;
     let mut pbVar8: *mut u8;
     let mut pbVar9: *mut u8;
     let mut pbVar10: *mut u8;
     let mut pbVar11: *mut u8;
     let mut uVar12: u16;
    bool bVar13;
     let mut cVar4: u8;

    uVar6 = 0x0;
    if(param_3 == 0xa)
    {
        uVar6 = param_1 >> 0xf;
    }
    uVar12  = (param_2 >> 0x10);
    pbVar9  = (u8 *)param_2;
    pbVar10 = pbVar9;
    pbVar8  = pbVar9;
    if((param_3 == 0xa) && (uVar6 < 0x0))
    {
        pbVar10  = pbVar9 + 0x1;
        *param_2 = '-';
        bVar13   = param_1 != 0x0;
        param_1  = -param_1;
        uVar6    = -(uVar6 + bVar13);
        pbVar8   = pbVar10;
    }
    do
    {
        uVar7 = 0x0;
        uVar5 = uVar6;
        if(uVar6 != 0x0)
        {
            uVar5 = uVar6 / param_3;
            uVar7 = uVar6 % param_3;
        }
        uVar2   = CONCAT22(uVar7, param_1);
        param_1 = (uVar2 / param_3);
        cVar4   = (uVar2 % param_3);
        bVar3   = cVar4 + 0x30;
        if(0x39 < bVar3)
        {
            bVar3 = cVar4 + 0x57;
        }
        pbVar11  = pbVar10 + 0x1;
        *pbVar10 = bVar3;
        uVar6    = uVar5;
        pbVar10  = pbVar11;
    } while((uVar5 | param_1) != 0x0);
    *pbVar11 = 0x0;
    do
    {
        pbVar11 = pbVar11 + -0x1;
        pbVar1  = pbVar11;
        bVar3   = *pbVar1;
        *pbVar1 = *pbVar8;
        *pbVar8 = bVar3;
        pbVar10 = pbVar8 + 0x2;
        pbVar8  = pbVar8 + 0x1;
    } while(pbVar10 < pbVar11);
    return pbVar9;
}


i16 pass1_1000_3f5c(param_1: i16, param_2: u16, param_3: u16, param_4: u16, param_5: u8)

{
     let mut uVar1: u16;
    u16 *puVar2;
    i16  iVar3;
    i16  iStack2;

    iStack2 = param_1 + 0x1;
    iVar3   = 0x0;
    if(PTR_LOOP_1050_61ec == 0x0)
    {
        puVar2 = &PTR_LOOP_1050_6210;
    }
    else
    {
        puVar2 = 0x6234;
    }
    for(; puVar2 <= globals.PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0x6)
    {
        uVar1 = pass1_1000_2a00(puVar2, &iStack2, param_2, param_3, param_4, param_5);
        if(uVar1 != 0xffff)
        {
            iVar3 = iVar3 + 0x1;
        }
    }
    return iVar3;
}

u16 pass1_1000_41e0(param_1: i16)

{
     let mut piStack6: *mut i16;

    piStack6 = CONCAT22(PTR_LOOP_1050_6192, globals.PTR_LOOP_1050_6190);
    while(true)
    {
        if(PTR_LOOP_1050_6190 + (PTR_LOOP_1050_6194 & 0xfffc) <= piStack6)
        {
            return 0x0;
        }
        if(*piStack6 == param_1)
            break;
        piStack6 = (piStack6 & 0xffff0000 | ZEXT24(piStack6 + 0x4));
    }
    *piStack6 = 0x0;
    return (piStack6 + 0x2);
}


i16 pass1_1000_422a(param_1: i16, param_2: u16, param_3: u16, param_4: u16)

{
     let mut puVar1: *mut u8;
     let mut puVar2: *mut u8;
     let mut puVar3: *mut u8;
     let mut puVar4: *mut u8;
     let mut piStack6: *mut i16;

    piStack6 = CONCAT22(PTR_LOOP_1050_6192, globals.PTR_LOOP_1050_6190);
    while(true)
    {
        if(PTR_LOOP_1050_6190 + (PTR_LOOP_1050_6194 & 0xfffc) <= piStack6)
        {
            puVar2 = globals.PTR_LOOP_1050_6194 + 0x28;
            puVar4 = globals.PTR_LOOP_1050_6192;
            puVar3 = pass1_1000_16aa(PTR_LOOP_1050_6190, globals.PTR_LOOP_1050_6192, puVar2, globals.PTR_LOOP_1050_6192, param_3, param_4);
            if((puVar4 | puVar3) == 0x0)
            {
                param_1 = 0x0;
            }
            else
            {
                puVar1                      = puVar3 + (PTR_LOOP_1050_6194 & 0xfffc);
                piStack6                    = CONCAT22(puVar4, puVar1);
                globals.PTR_LOOP_1050_6190 = puVar3;
                globals.PTR_LOOP_1050_6192 = puVar4;
                *piStack6                   = param_1;
                (puVar1 + 0x2)              = param_2;
                globals.PTR_LOOP_1050_6194 = puVar2;
                pass1_1000_4906((Struct20 *)CONCAT22(puVar4, puVar1 + 0x4), 0x0, 0x24);
            }
            return param_1;
        }
        if(*piStack6 == 0x0)
            break;
        piStack6 = (piStack6 & 0xffff0000 | ZEXT24(piStack6 + 0x4));
    }
    (piStack6 + 0x2) = param_2;
    *piStack6        = param_1;
    return param_1;
}


void pass1_1000_43f0(param_1: u16, param_2: u16)

{
    if(PTR_LOOP_1050_68b4 == 0x0)
    {
        pass1_1000_440c(param_2);
        globals.PTR_LOOP_1050_68b4 = globals.PTR_LOOP_1050_68b4 + 0x1;
    }
    return;
}


void pass1_1000_440c(param_1: u16)

{
     let mut cVar1: u8;
     let mut pcVar2: String;
     let mut uVar3: u16;
    i16   iVar4;
     let mut uVar5: u16;
    long  lVar6;
     let mut uVar7: u16;
     let mut uVar8: u16;
     let mut pcStack8: String;

    uVar3    = pass1_1000_3ec0(0x61ca, &USHORT_1050_1050);
    pcStack8 = CONCAT22(param_1, uVar3);
    if(((param_1 | uVar3) != 0x0) && (_DAT_1050_61ce = CONCAT22(PTR_LOOP_1050_61d0, DAT_1050_61ce), *pcStack8 != '\0'))
    {
        str_op_1000_3dbe(CONCAT13((PTR_USHORT_1050_1050_1050_61de >> 0x8), CONCAT12(PTR_USHORT_1050_1050_1050_61de, globals.PTR_PTR_DAT_1050_5350_1050_61d4_1050_61dc)), CONCAT22(param_1, uVar3), 0x3);
        pcStack8 = CONCAT22(param_1, uVar3 + 0x3);
        cVar1    = *pcStack8;
        if(cVar1 == '-')
        {
            pcStack8 = CONCAT22(param_1, uVar3 + 0x4);
        }
        uVar5          = 0x0;
        uVar8          = 0x0;
        uVar7          = 0xe10;
        iVar4          = pass1_1000_3e2c(pcStack8 & 0xffff | param_1 << 0x10);
        _DAT_1050_61ce = pass1_1000_52be(iVar4, uVar5, uVar7, uVar8);
        for(; (pcVar2 = pcStack8, *pcStack8 == '+' || (('/' < *pcStack8 && (*pcStack8 < ':')))); pcStack8 = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1)))
        {
        }
        if(*pcStack8 == ':')
        {
            uVar5          = 0x0;
            uVar8          = 0x0;
            uVar7          = 0x3c;
            pcStack8       = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1));
            iVar4          = pass1_1000_3e2c(pcVar2 & 0xffff0000 | (pcStack8 + 0x1));
            lVar6          = pass1_1000_52be(iVar4, uVar5, uVar7, uVar8);
            uVar5          = (lVar6 >> 0x10);
            _DAT_1050_61ce = _DAT_1050_61ce + lVar6;
            for(; (pcVar2 = pcStack8, '/' < *pcStack8 && (*pcStack8 < ':')); pcStack8 = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1)))
            {
            }
            if(*pcStack8 == ':')
            {
                pcStack8       = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1));
                iVar4          = pass1_1000_3e2c(pcVar2 & 0xffff0000 | (pcStack8 + 0x1));
                _DAT_1050_61ce = _DAT_1050_61ce + CONCAT22(uVar5, iVar4);
                for(; ('/' < *pcStack8 && (*pcStack8 < ':')); pcStack8 = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1)))
                {
                }
            }
        }
        globals.PTR_LOOP_1050_61d0 = (_DAT_1050_61ce >> 0x10);
        if(cVar1 == '-')
        {
            _DAT_1050_61ce = CONCAT22(-(PTR_LOOP_1050_61d0 + (DAT_1050_61ce != 0x0)), -DAT_1050_61ce);
        }
        DAT_1050_61d2 = *pcStack8;
        if(DAT_1050_61d2 == 0x0)
        {
            *_PTR_PTR_1050_61e0 = '\0';
        }
        else
        {
            str_op_1000_3dbe(_PTR_PTR_1050_61e0, pcStack8, 0x3);
        }
    }
    globals.PTR_LOOP_1050_61d0 = (_DAT_1050_61ce >> 0x10);
    return;
}


u16 pass1_1000_455a(u1632 param_1, param_2: u16)

{
     let mut piVar1: *mut i16;
    i16  iVar2;
     let mut uVar3: u16;
    i16  iVar4;
     let mut UVar5: u16;
    u32  uVar6;
    i16  iStack6;

    if((((param_1 + 0xa) < 0x43) || ((param_1 + 0x8) < 0x3)) || (0x9 < (param_1 + 0x8)))
        goto LAB_1000_4623;
    if(((param_1 + 0x8) < 0x4) || (0x8 < (param_1 + 0x8)))
    {
        uVar3 = (param_1 + 0xa);
        if((uVar3 < 0x57) || ((param_1 + 0x8) != 0x3))
        {
            iStack6 = ((param_1 + 0x8) * 0x2 + 0x61b2);
        }
        else
        {
            iStack6 = ((param_1 + 0x8) * 0x2 + 0x61b0) + 0x7;
        }
        if((uVar3 & 0x3) == 0x0)
        {
            iStack6 = iStack6 + 0x1;
        }
        uVar3   = (uVar3 - 0x46) * 0x16d + ((uVar3 - 0x1) >> 0x2) + iStack6;
        uVar6   = pass1_1000_52f0(uVar3 - 0xd, (uVar3 >> 0xf) - (uVar3 < 0xd), 0x7, 0x0);
        iStack6 = uVar6 - iStack6;
        iVar4   = -iStack6;
        if((param_1 + 0x8) == 0x3)
        {
            iVar2 = (param_1 + 0xe);
            if((iVar4 < iVar2) || ((-iVar2 == iStack6 && (0x1 < (param_1 + 0x4)))))
                goto LAB_1000_460e;
        }
        else
        {
            piVar1 = (param_1 + 0xe);
            iVar2  = *piVar1;
            if((SBORROW2(*piVar1, iVar4) != iVar2 + iStack6 < 0x0) || ((iVar2 == iVar4 && ((param_1 + 0x4) < 0x1))))
                goto LAB_1000_460e;
        }
    LAB_1000_4623:
        UVar5 = 0x0;
    }
    else
    {
    LAB_1000_460e:
        UVar5 = 0x1;
    }
    return UVar5;
}


i16 pass1_1000_462e(param_1: u16, param_2: i16, param_3: u16, param_4: u16, param_5: u16, param_6: i16, param_7: i16, param_8: u16, param_9: u16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
     let mut uVar3: u16;
    i16        iVar4;
     let mut UVar5: u16;
     let mut uVar6: u16;
     let mut uVar7: u16;
     let mut uVar8: u16;
    u32 uVar9;
    i16        iStack26;
    u8         local_16[0x4];
     let mut uStack18: u16;
    i16        iStack14;
    i16        iStack12;
    i16        iStack8;
     let mut local_4: u16;
    i16        iStack2;
     let mut uVar10: u16;
     let mut uVar11: u16;
     let mut uVar12: u16;
     let mut uVar13: u16;

    iStack2 = param_7 + 0x1;
    local_4 = &USHORT_1050_1050;
    uVar8   = (param_2 * 0x2 + 0x61ae);
    if(((param_1 & 0x3) == 0x0) && (0x2 < param_2))
    {
        uVar8 = uVar8 + 0x1;
    }
    pass1_1000_43f0(&iStack2, param_9);
    uVar13 = 0x0;
    uVar12 = 0x3c;
    uVar11 = 0x0;
    uVar10 = 0x3c;
    uVar1  = ((long)param_1 * 0x16d);
    uVar2  = (param_1 + 0x3) >> 0x2;
    uVar3  = uVar2 + param_3;
    uVar6  = uVar1 + uVar3;
    uVar7  = uVar6 + uVar8;
    uVar9
      = pass1_1000_52be(uVar7 + 0xe44, (((long)param_1 * 0x16d) >> 0x10) + ((param_1 + 0x3) >> 0xf) + (param_3 >> 0xf) + CARRY2(uVar2, param_3) + CARRY2(uVar1, uVar3) + (uVar8 >> 0xf) + CARRY2(uVar6, uVar8) + (0xf1bb < uVar7), 0x18, 0x0);
    uVar9    = pass1_1000_52be(uVar9 + param_4, (uVar9 >> 0x10) + (param_4 >> 0xf) + CARRY2(uVar9, param_4), uVar10, uVar11);
    iVar4    = pass1_1000_52be(uVar9 + param_5, (uVar9 >> 0x10) + (param_5 >> 0xf) + CARRY2(uVar9, param_5), uVar12, uVar13);
    iStack26 = iVar4 + param_6 + DAT_1050_61ce;
    iStack8  = param_3 + uVar8;
    iStack12 = param_1 + 0x50;
    iStack14 = param_2 + -0x1;
    uStack18 = param_4;
    if(DAT_1050_61d2 != 0x0)
    {
        UVar5 = pass1_1000_455a((u1632)local_16, param_8);
        if(UVar5 != 0x0)
        {
            iStack26 = iStack26 + -0xe10;
        }
    }
    return iStack26;
}

void pass1_1000_3552(param_1: i16, param_2: i16, param_3: u16)

{
     let mut piVar1: *mut i16;
     let mut uVar2: u16;
     let mut in_DX: u16;
     let mut uVar3: u16;
     let mut unaff_CS: u16;
     let mut in_AF: u8;

    if(param_1 != 0x0)
    {
        piVar1  = (param_2 + -0xa);
        *piVar1 = *piVar1 + param_1;
        uVar3   = 0x0;
        do
        {
            uVar2   = pass1_1000_3503(in_DX, in_DX, param_2, unaff_CS, param_3, in_AF);
            uVar3   = uVar3 | uVar2;
            param_1 = param_1 + -0x1;
        } while(param_1 != 0x0);
        if(uVar3 != 0x0)
        {
            (param_2 + -0xa) = 0xffff;
        }
    }
    return;
}


void pass1_1000_356e(param_1: u16, param_2: u16, param_3: u16, param_4: i16, param_5: i16, mut param_6: *mut u8, param_7: u16, param_8: u16)

{
     let mut pbVar1: *mut u8;
    u32 uVar2;
     let mut bVar3: u8;

    while(((0x0 < param_5 || (param_1 != 0x0)) || (param_3 != 0x0)))
    {
        uVar2   = param_3;
        param_3 = param_3 / param_2;
        uVar2   = uVar2 % param_2 << 0x10 | param_1;
        param_1 = (uVar2 / param_2);
        bVar3   = (uVar2 % param_2) + 0x30;
        if(0x39 < bVar3)
        {
            bVar3 = bVar3 + *(param_4 + -0x3);
        }
        pbVar1  = param_6;
        param_6 = param_6 + -0x1;
        *pbVar1 = bVar3;
        param_5 = param_5 + -0x1;
    }
    return;
}


u16 *pass1_1000_35aa(void)

{
    u16 *puVar1;

    puVar1 = &PTR_LOOP_1050_6210;
    while(true)
    {
        if(PTR_LOOP_1050_5ff0 < puVar1)
        {
            return 0x0;
        }
        if((*(u8 *)(puVar1 + 0x5) & 0x83) == 0x0)
            break;
        puVar1 = puVar1 + 0x6;
    }
    *(puVar1 + 0x5) = 0x0;
    puVar1[0x2]     = 0x0;
    puVar1[0x4]     = 0x0;
    puVar1[0x3]     = 0x0;
    puVar1[0x1]     = 0x0;
    *puVar1         = 0x0;
    *(puVar1 + 0xb) = 0xff;
    return puVar1;
}


void pass1_1000_39e1(void)

{
    return;
}

i16 pass1_1000_3bac(void)

{
    i16 iVar1;

    if(PTR_LOOP_1050_5f48 < &stack0x0004)
    {
        iVar1 = -(PTR_LOOP_1050_5f48 - &stack0x0004);
    }
    else
    {
        iVar1 = 0x0;
    }
    return iVar1;
}

void pass1_1000_3cb7(param_1: i16)

{
     let mut uVar1: u16;
    u16 *puVar2;

    puVar2 = *(u16 **)(param_1 + 0xa);
    if(puVar2 == *(u16 **)(param_1 + 0xc))
    {
        puVar2 = *(u16 **)(param_1 + 0x8);
    }
    while(true)
    {
        uVar1 = *puVar2;
        if(uVar1 == 0xfffe)
            break;
        puVar2 = (puVar2 + (uVar1 & 0xfffe) + 0x2);
    }
    return;
}


u16 *pass1_1000_3cea(param_1: u32, param_2: u32)

{
    u16  *pUVar1;
     let mut pcVar2: String;
    u16  *pUVar3;
    i16   iVar4;
     let mut uVar5: u16;
     let mut uVar6: u16;
    u16  *pUVar7;
     let mut pcVar8: String;
    u16  *pUVar9;
    u16  *pUVar10;
     let mut uVar11: u16;
     let mut uVar12: u16;
    bool  bVar13;

    uVar11 = (param_1 >> 0x10);
    bVar13 = true;
    iVar4  = -0x1;
    pUVar7 = param_1;
    do
    {
        if(iVar4 == 0x0)
            break;
        iVar4  = iVar4 + -0x1;
        pUVar1 = pUVar7;
        pUVar7 = (pUVar7 + 0x1);
        bVar13 = *pUVar1 == '\0';
    } while(!bVar13);
    pUVar10 = (pUVar7 + -0x1);
    uVar12  = (param_2 >> 0x10);
    pcVar8  = param_2;
    uVar5   = 0xffff;
    do
    {
        if(uVar5 == 0x0)
            break;
        uVar5  = uVar5 - 0x1;
        pcVar2 = pcVar8;
        pcVar8 = pcVar8 + 0x1;
        bVar13 = *pcVar2 == '\0';
    } while(!bVar13);
    uVar5 = ~uVar5;
    if(!bVar13)
    {
        pcVar8 = pcVar8 + -uVar5;
        uVar5  = uVar5 + 0x1;
    }
    pUVar9 = (pcVar8 + -uVar5);
    if(uVar5 == 0x0)
    {
        pUVar1   = pUVar9;
        pUVar9   = pUVar9 + 0x1;
        *pUVar10 = *pUVar1;
        uVar5    = 0xfffe;
        pUVar10  = (pUVar7 + 0x1);
    }
    else
    {
        if((pUVar9 & 0x1) != 0x0)
        {
            pUVar1   = pUVar9;
            pUVar9   = (pUVar9 + 0x1);
            *pUVar10 = *pUVar1;
            uVar5    = uVar5 - 0x1;
            pUVar10  = pUVar7;
        }
    }
    for(uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 = uVar6 - 0x1)
    {
        pUVar3  = pUVar10;
        pUVar10 = pUVar10 + 0x1;
        pUVar1  = pUVar9;
        pUVar9  = pUVar9 + 0x1;
        *pUVar3 = *pUVar1;
    }
    for(uVar5 = ((uVar5 & 0x1) != 0x0); uVar5 != 0x0; uVar5 = uVar5 - 0x1)
    {
        pUVar3  = pUVar10;
        pUVar10 = (pUVar10 + 0x1);
        pUVar1  = pUVar9;
        pUVar9  = (pUVar9 + 0x1);
        *pUVar3 = *pUVar1;
    }
    return param_1;
}


void unk_str_op_1000_3d3e(param_1: &mut String, char *in_string_2)

{
    u16  *puVar4;
    u16  *puVar5;
     let mut uVar6: u16;
     let mut uVar7: u16;
     let mut l_string_2: String;
     let mut puVar6: String;
     let mut puVar7: String;
     let mut uVar8: u16;
     let mut l_string_1: String;
    bool  l_b_var8;
     let mut puVar3: String;
     let mut puVar2: String;
     let mut puVar1: String;

    l_string_1 = (in_string_2 >> 0x10);
    l_string_2 = in_string_2;
    l_b_var8   = true;
    uVar6      = 0xffff;
    puVar6     = l_string_2;
    do
    {
        if(uVar6 == 0x0)
            break;
        uVar6    = uVar6 - 0x1;
        puVar1   = puVar6;
        puVar6   = puVar6 + 0x1;
        l_b_var8 = *puVar1 == '\0';
    } while(!l_b_var8);
    uVar6  = ~uVar6;
    uVar8  = (param_1 >> 0x10);
    puVar7 = param_1;
    if(l_b_var8)
    {
        if((param_1 & 0x1) != 0x0)
        {
            puVar7     = puVar7 + 0x1;
            l_string_2 = l_string_2 + 0x1;
            *param_1   = *in_string_2;
            uVar6      = uVar6 - 0x1;
        }
    }
    else
    {
        puVar7     = puVar7 + 0x2;
        l_string_2 = l_string_2 + 0x2;
        param_1    = in_string_2;
        uVar6      = uVar6 - 0x1;
    }
    for(uVar7 = uVar6 >> 0x1; uVar7 != 0x0; uVar7 = uVar7 - 0x1)
    {
        puVar5     = puVar7;
        puVar7     = (puVar7 + 0x2);
        puVar4     = l_string_2;
        l_string_2 = (l_string_2 + 0x2);
        *puVar5    = *puVar4;
    }
    for(uVar6 = ((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 = uVar6 - 0x1)
    {
        puVar5     = puVar7;
        puVar7     = (puVar7 + 0x1);
        puVar4     = l_string_2;
        l_string_2 = (l_string_2 + 0x1);
        *puVar5    = *puVar4;
    }
    return;
}
void pass1_1000_2f00(param_1: i16, i16 *param_2, param_3: u16, param_4: u16, param_5: u16, param_6: u8)

{
    if(((*(u8 *)(param_2 + 0x78) & 0x10) != 0x0) && ((*(u8 *)(*(u8 *)(param_2 + 0xb) + 0x5f90) & 0x40) != 0x0))
    {
        pass1_1000_2fa4(param_2, param_3, param_4, param_5, param_6);
        if(param_1 != 0x0)
        {
            *(u8 *)(param_2 + 0x78) = 0x0;
            param_2[0x79]           = 0x0;
            *param_2                = 0x0;
            param_2[0x1]            = 0x0;
            param_2[0x3]            = 0x0;
            param_2[0x4]            = 0x0;
        }
    }
    return;
}


u16 pass1_1000_2f48(long param_1, param_2: i16, param_3: u16, param_4: u16, param_5: u16, param_6: u8)

{
     let mut uVar1: u16;
     let mut puVar2: *mut u8;
    i16 iStack2;

    iStack2 = param_2 + 0x1;
    if(param_1 == 0x0)
    {
        uVar1 = pass1_1000_3038(0x0, param_3, param_4, param_5, param_6);
    }
    else
    {
        uVar1 = pass1_1000_2fa4(param_1, param_3, param_4, param_5, param_6);
        if(uVar1 == 0x0)
        {
            if((*(u8 *)(param_1 + 0x78) & 0x40) != 0x0)
            {
                puVar2 = pass1_1000_400a(*(u8 *)(param_1 + 0xb), &iStack2);
                uVar1  = -(puVar2 != 0x0);
            }
        }
        else
        {
            uVar1 = 0xffff;
        }
    }
    return uVar1;
}


u16 pass1_1000_2fa4(i16 *param_1, param_2: u16, param_3: u16, param_4: u16, param_5: u8)

{
     let mut piVar1: *mut i16;
     let mut bVar2: u8;
    i16  iVar3;
     let mut puVar4: *mut u8;
     let mut puVar5: *mut u8;
     let mut uVar6: u16;

    uVar6 = 0x0;
    bVar2 = *(u8 *)(param_1 + 0x5);
    if(((bVar2 & 0x3) == 0x2) && (((bVar2 & 0x8) != 0x0 || ((*(u8 *)(param_1 + 0x78) & 0x1) != 0x0))))
    {
        puVar4 = (*param_1 - param_1[0x3]);
        if(0x0 < puVar4)
        {
            puVar5 = mixed_dos3_call_1000_39f2(*(u8 *)(param_1 + 0xb), CONCAT22(param_1[0x4], param_1[0x3]), puVar4, param_2, param_3, param_4, param_5);
            if(puVar5 == puVar4)
            {
                if((*(u8 *)(param_1 + 0x5) & 0x80) != 0x0)
                {
                    piVar1        = param_1 + 0x5;
                    *(u8 *)piVar1 = *(u8 *)piVar1 & 0xfd;
                }
            }
            else
            {
                piVar1        = param_1 + 0x5;
                *(u8 *)piVar1 = *(u8 *)piVar1 | 0x20;
                uVar6         = 0xffff;
            }
        }
    }
    iVar3        = param_1[0x4];
    *param_1     = param_1[0x3];
    param_1[0x1] = iVar3;
    param_1[0x2] = 0x0;
    return uVar6;
}


void pass1_1000_3024(param_1: u16, param_2: u16, param_3: u16, param_4: u8)

{
    pass1_1000_3038(0x1, param_1, param_2, param_3, param_4);
    return;
}


i16 pass1_1000_3038(param_1: i16, param_2: u16, param_3: u16, param_4: u16, param_5: u8)

{
     let mut uVar1: u16;
     let mut puVar2: *mut u8;
    i16 iVar3;
    i16 iStack4;

    iVar3   = 0x0;
    iStack4 = 0x0;
    for(puVar2 = &PTR_LOOP_1050_6210; puVar2 <= globals.PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0xc)
    {
        if((param_1 == 0x1) && ((puVar2[0xa] & 0x83) != 0x0))
        {
            uVar1 = pass1_1000_2f48(CONCAT22(0x1050, puVar2), &stack0xfffe, param_2, param_3, param_4, param_5);
            if(uVar1 != 0xffff)
            {
                iVar3 = iVar3 + 0x1;
            }
        }
        else
        {
            if((param_1 == 0x0) && (((puVar2[0xa] & 0x2) != 0x0 && (uVar1 = pass1_1000_2f48(CONCAT22(0x1050, puVar2), &stack0xfffe, param_2, param_3, param_4, param_5), uVar1 == 0xffff))))
            {
                iStack4 = -0x1;
            }
        }
    }
    if(param_1 == 0x1)
    {
        iStack4 = iVar3;
    }
    return iStack4;
}


u16 pass1_1000_3113(param_1: u16, param_2: u16)

{
     let mut cVar1: u8;
     let mut pcVar2: String;
     let mut bVar3: u8;
     let mut uVar4: u16;

    pass1_1000_3552(0x1, param_1, param_2);
    pcVar2           = (param_1 + 0xa);
    cVar1            = *pcVar2;
    (param_1 + 0xa)  = pcVar2 + 0x1;
    *(param_1 - 0x4) = cVar1;
    if((cVar1 != '\0') && (-0x1 < (param_1 - 0xa)))
    {
        if((u8)(cVar1 - 0x20U) < 0x59)
        {
            bVar3 = *(u8 *)((u8)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
        }
        else
        {
            bVar3 = 0x0;
        }
        bVar3                  = *(u8 *)((u8)(bVar3 * '\b' + *(param_1 - 0x7)) + 0x5ffe) >> 0x4;
        *(u8 *)(param_1 - 0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (**(bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (param_1 - 0xa);
}


u16 pass1_1000_311e(param_1: i16, param_2: u16)

{
     let mut cVar1: u8;
     let mut pcVar2: String;
     let mut bVar3: u8;
     let mut uVar4: u16;

    (param_1 + -0x12) = 0x0;
    (param_1 + -0xc)  = 0x0;
    (param_1 + -0x14) = 0x0;
    (param_1 + -0x6)  = 0x20;
    (param_1 + -0xe)  = 0xffff;
    pcVar2            = (param_1 + 0xa);
    cVar1             = *pcVar2;
    (param_1 + 0xa)   = pcVar2 + 0x1;
    *(param_1 + -0x4) = cVar1;
    if((cVar1 != '\0') && (-0x1 < (param_1 + -0xa)))
    {
        if((u8)(cVar1 - 0x20U) < 0x59)
        {
            bVar3 = *(u8 *)((u8)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
        }
        else
        {
            bVar3 = 0x0;
        }
        bVar3                   = *(u8 *)((u8)(bVar3 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *)(param_1 + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (**(bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (param_1 + -0xa);
}


u16 pass1_1000_3134(param_1: i16, param_2: u16)

{
     let mut pbVar1: *mut u8;
     let mut cVar2: u8;
     let mut pcVar3: String;
     let mut bVar4: u8;
     let mut uVar5: u16;

    cVar2 = *(param_1 + -0x4);
    if(cVar2 == '-')
    {
        pbVar1  = (u8 *)(param_1 + -0x6);
        *pbVar1 = *pbVar1 | 0x4;
    }
    else
    {
        if(cVar2 == '+')
        {
            pbVar1  = (u8 *)(param_1 + -0x6);
            *pbVar1 = *pbVar1 | 0x1;
        }
        else
        {
            if(cVar2 == ' ')
            {
                pbVar1  = (u8 *)(param_1 + -0x6);
                *pbVar1 = *pbVar1 | 0x2;
            }
            else
            {
                if(cVar2 == '#')
                {
                    pbVar1  = (u8 *)(param_1 + -0x6);
                    *pbVar1 = *pbVar1 | 0x80;
                }
                else
                {
                    pbVar1  = (u8 *)(param_1 + -0x6);
                    *pbVar1 = *pbVar1 | 0x8;
                }
            }
        }
    }
    pcVar3            = (param_1 + 0xa);
    cVar2             = *pcVar3;
    (param_1 + 0xa)   = pcVar3 + 0x1;
    *(param_1 + -0x4) = cVar2;
    if((cVar2 != '\0') && (-0x1 < (param_1 + -0xa)))
    {
        if((u8)(cVar2 - 0x20U) < 0x59)
        {
            bVar4 = *(u8 *)((u8)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
        }
        else
        {
            bVar4 = 0x0;
        }
        bVar4                   = *(u8 *)((u8)(bVar4 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *)(param_1 + -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 = (**(bVar4 * 0x2 + 0x30a4))();
        return uVar5;
    }
    return (param_1 + -0xa);
}

u16 pass1_1000_3168(param_1: i16, param_2: u16)

{
     let mut pbVar1: *mut u8;
     let mut cVar2: u8;
     let mut pcVar3: String;
     let mut bVar4: u8;
     let mut uVar5: u16;

    cVar2 = *(param_1 + -0x4);
    if(cVar2 == '*')
    {
        uVar5 = pass1_1000_34cf(param_1, param_2);
        if(uVar5 < 0x0)
        {
            uVar5   = -uVar5;
            pbVar1  = (u8 *)(param_1 + -0x6);
            *pbVar1 = *pbVar1 | 0x4;
        }
    }
    else
    {
        uVar5 = (param_1 + -0xc) * 0xa + (u8)(cVar2 - 0x30);
    }
    (param_1 + -0xc)  = uVar5;
    pcVar3            = (param_1 + 0xa);
    cVar2             = *pcVar3;
    (param_1 + 0xa)   = pcVar3 + 0x1;
    *(param_1 + -0x4) = cVar2;
    if((cVar2 != '\0') && (-0x1 < (param_1 + -0xa)))
    {
        if((u8)(cVar2 - 0x20U) < 0x59)
        {
            bVar4 = *(u8 *)((u8)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
        }
        else
        {
            bVar4 = 0x0;
        }
        bVar4                   = *(u8 *)((u8)(bVar4 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *)(param_1 + -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 = (**(bVar4 * 0x2 + 0x30a4))();
        return uVar5;
    }
    return (param_1 + -0xa);
}


u16 pass1_1000_3194(param_1: i16, param_2: u16)

{
     let mut cVar1: u8;
     let mut pcVar2: String;
     let mut bVar3: u8;
     let mut uVar4: u16;

    (param_1 + -0xe)  = 0x0;
    pcVar2            = (param_1 + 0xa);
    cVar1             = *pcVar2;
    (param_1 + 0xa)   = pcVar2 + 0x1;
    *(param_1 + -0x4) = cVar1;
    if((cVar1 != '\0') && (-0x1 < (param_1 + -0xa)))
    {
        if((u8)(cVar1 - 0x20U) < 0x59)
        {
            bVar3 = *(u8 *)((u8)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
        }
        else
        {
            bVar3 = 0x0;
        }
        bVar3                   = *(u8 *)((u8)(bVar3 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *)(param_1 + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (**(bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (param_1 + -0xa);
}


// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_319c(param_1: i16, param_2: u16)

{
     let mut cVar1: u8;
     let mut pcVar2: String;
     let mut bVar3: u8;
     let mut uVar4: u16;

    cVar1 = *(param_1 + -0x4);
    if(cVar1 == '*')
    {
        uVar4 = pass1_1000_34cf(param_1, param_2);
        if(uVar4 < 0x0)
        {
            uVar4 = 0xffff;
        }
    }
    else
    {
        uVar4 = (param_1 + -0xe) * 0xa + (u8)(cVar1 - 0x30);
    }
    (param_1 + -0xe)  = uVar4;
    pcVar2            = (param_1 + 0xa);
    cVar1             = *pcVar2;
    (param_1 + 0xa)   = pcVar2 + 0x1;
    *(param_1 + -0x4) = cVar1;
    if((cVar1 != '\0') && (-0x1 < (param_1 + -0xa)))
    {
        if((u8)(cVar1 - 0x20U) < 0x59)
        {
            bVar3 = *(u8 *)((u8)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
        }
        else
        {
            bVar3 = 0x0;
        }
        bVar3                   = *(u8 *)((u8)(bVar3 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *)(param_1 + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (**(bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (param_1 + -0xa);
}


u16 pass1_1000_31c5(param_1: i16, param_2: u16)

{
     let mut pbVar1: *mut u8;
     let mut cVar2: u8;
     let mut pcVar3: String;
     let mut bVar4: u8;
     let mut uVar5: u16;

    cVar2 = *(param_1 + -0x4);
    if(cVar2 == 'l')
    {
        pbVar1  = (u8 *)(param_1 + -0x6);
        *pbVar1 = *pbVar1 | 0x10;
    }
    else
    {
        if(cVar2 == 'F')
        {
            pbVar1  = (u8 *)(param_1 + -0x6);
            *pbVar1 = *pbVar1 | 0x20;
        }
        else
        {
            if(cVar2 == 'N')
            {
                pbVar1  = (u8 *)(param_1 + -0x5);
                *pbVar1 = *pbVar1 | 0x10;
            }
            else
            {
                if(cVar2 == 'L')
                {
                    pbVar1  = (u8 *)(param_1 + -0x5);
                    *pbVar1 = *pbVar1 | 0x4;
                }
                else
                {
                    pbVar1  = (u8 *)(param_1 + -0x5);
                    *pbVar1 = *pbVar1 | 0x8;
                }
            }
        }
    }
    pcVar3            = (param_1 + 0xa);
    cVar2             = *pcVar3;
    (param_1 + 0xa)   = pcVar3 + 0x1;
    *(param_1 + -0x4) = cVar2;
    if((cVar2 != '\0') && (-0x1 < (param_1 + -0xa)))
    {
        if((u8)(cVar2 - 0x20U) < 0x59)
        {
            bVar4 = *(u8 *)((u8)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
        }
        else
        {
            bVar4 = 0x0;
        }
        bVar4                   = *(u8 *)((u8)(bVar4 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *)(param_1 + -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 = (**(bVar4 * 0x2 + 0x30a4))();
        return uVar5;
    }
    return (param_1 + -0xa);
}


u16 pass1_1000_31f7(param_1: u16, param_2: i16, mut param_3: *mut u16, param_4: i16, param_5: u16)

{
     let mut piVar1: *mut i16;
     let mut pbVar2: *mut u8;
    u16  *puVar3;
     let mut cVar4: u8;
     let mut pcVar5: String;
     let mut bVar6: u8;
     let mut uVar7: u16;
    i16   iVar8;
    i16   iVar9;
    i16   iVar10;
     let mut pcVar11: String;
     let mut uVar12: u16;
    u16  *puVar13;
     let mut pcVar14: String;
    bool  bVar15;
    u32   uVar16;

    cVar4 = *(param_2 + -0x4);
    if((cVar4 == 'd') || (cVar4 == 'i'))
    {
        pbVar2  = (u8 *)(param_2 + -0x6);
        *pbVar2 = *pbVar2 | 0x40;
    LAB_1000_3399:
        *(param_2 + -0x8) = 0xa;
    LAB_1000_33d4:
        if((*(u8 *)(param_2 + -0x6) & 0x10) == 0x0)
        {
            uVar7 = pass1_1000_34cf(param_2, param_5);
            if((*(u8 *)(param_2 + -0x6) & 0x40) == 0x0)
            {
                uVar16 = uVar7;
            }
            else
            {
                uVar16 = SEXT24(uVar7);
            }
        }
        else
        {
            uVar16 = pass1_1000_34d8(param_2, param_5);
        }
        if(((*(u8 *)(param_2 + -0x6) & 0x40) != 0x0) && ((long)uVar16 < 0x0))
        {
            pbVar2  = (u8 *)(param_2 + -0x5);
            *pbVar2 = *pbVar2 | 0x1;
            uVar16  = CONCAT22(-((uVar16 >> 0x10) + (uVar16 != 0x0)), -uVar16);
        }
        if((param_2 + -0xe) < 0x0)
        {
            (param_2 + -0xe) = 0x1;
        }
        else
        {
            pbVar2  = (u8 *)(param_2 + -0x6);
            *pbVar2 = *pbVar2 & 0xf7;
        }
        if(uVar16 == 0x0)
        {
            (param_2 + -0x12) = 0x0;
        }
        pcVar11 = *(u8 *)(param_2 + -0x8);
        pass1_1000_356e(uVar16, pcVar11, (uVar16 >> 0x10), param_2, (param_2 + -0xe), (u8 *)(param_2 + -0x17), param_5, param_5);
        if(((*(u8 *)(param_2 + -0x5) & 0x2) != 0x0) && ((pcVar11 == 0x0 || (*(u8 *)(param_2 + -0x17) != 0x30))))
        {
            *(param_2 + -0x18) = 0x30;
            pcVar11            = pcVar11 + 0x1;
        }
    }
    else
    {
        if(cVar4 == 'u')
            goto LAB_1000_3399;
        if(cVar4 == 'X')
        {
            *(param_2 + -0x3) = 0x7;
        LAB_1000_33a9:
            if((*(u8 *)(param_2 + -0x6) & 0x80) != 0x0)
            {
                (param_2 + -0x12)  = 0x2;
                *(param_2 + -0x10) = 0x30;
                *(param_2 + -0xf)  = *(param_2 + -0x3) + 'Q';
            }
            *(param_2 + -0x8) = 0x10;
            goto LAB_1000_33d4;
        }
        if(cVar4 == 'x')
        {
            *(param_2 + -0x3) = 0x27;
            goto LAB_1000_33a9;
        }
        if(cVar4 == 'o')
        {
            if((*(u8 *)(param_2 + -0x6) & 0x80) != 0x0)
            {
                pbVar2  = (u8 *)(param_2 + -0x5);
                *pbVar2 = *pbVar2 | 0x2;
            }
            *(param_2 + -0x8) = 0x8;
            goto LAB_1000_33d4;
        }
        if(cVar4 == 'c')
        {
            uVar7               = pass1_1000_34cf(param_2, param_5);
            *(param_2 + -0x216) = uVar7;
            pcVar11             = 0x1;
        }
        else
        {
            if(cVar4 == 's')
            {
                pass1_1000_34e6(param_1, param_2, param_5);
                if((param_3 != 0x0) || (pcVar11 = DAT_1050_605d, param_4 != 0x0))
                {
                    iVar10  = (param_2 + -0xe);
                    puVar13 = param_3;
                    if(iVar10 != 0x0)
                    {
                        bVar15 = true;
                        do
                        {
                            if(iVar10 == 0x0)
                                break;
                            iVar10  = iVar10 + -0x1;
                            puVar3  = puVar13;
                            puVar13 = (puVar13 + 0x1);
                            bVar15  = *puVar3 == '\0';
                        } while(!bVar15);
                        if(bVar15)
                        {
                            puVar13 = (puVar13 + -0x1);
                        }
                    }
                    pcVar11 = (puVar13 - param_3);
                }
            }
            else
            {
                if(cVar4 == 'n')
                {
                    pass1_1000_34e6(param_1, param_2, param_5);
                    *param_3 = (param_2 + -0xa);
                    if((*(u8 *)(param_2 + -0x6) & 0x10) != 0x0)
                    {
                        param_3[0x1] = 0x0;
                    }
                    goto LAB_1000_30cf;
                }
                if(cVar4 == 'p')
                {
                    if((*(u8 *)(param_2 + -0x6) & 0x30) == 0x0)
                    {
                        uVar7  = pass1_1000_34cf(param_2, param_5);
                        uVar16 = uVar7;
                    }
                    else
                    {
                        uVar16 = pass1_1000_34d8(param_2, param_5);
                        uVar12 = (uVar16 >> 0x10);
                        if((*(u8 *)(param_2 + -0x5) & 0x18) == 0x0)
                        {
                            *(param_2 + -0x3) = 0x7;
                            pass1_1000_356e(uVar16, 0x10, 0x0, param_2, 0x4, (u8 *)(param_2 + -0x20e), param_5, param_5);
                            pass1_1000_356e(uVar12, 0x10, 0x0, param_2, 0x4, (u8 *)(param_2 + -0x213), param_5, param_5);
                            *(param_2 + -0x212) = 0x3a;
                            pcVar11             = 0x9;
                            goto LAB_1000_3444;
                        }
                    }
                    *(param_2 + -0x3) = 0x7;
                    pass1_1000_356e(uVar16, 0x10, 0x0, param_2, 0x4, (u8 *)(param_2 + -0x213), param_5, param_5);
                    pcVar11 = 0x4;
                }
                else
                {
                    if((cVar4 == 'E') || (cVar4 == 'G'))
                    {
                        piVar1  = (param_2 + -0x14);
                        *piVar1 = *piVar1 + 0x1;
                    }
                    pbVar2  = (u8 *)(param_2 + -0x6);
                    *pbVar2 = *pbVar2 | 0x40;
                    bVar6   = *(u8 *)(param_2 + -0x4) | 0x20;
                    iVar10  = (param_2 + -0xe);
                    if(iVar10 < 0x1)
                    {
                        if(iVar10 == 0x0)
                        {
                            if(bVar6 == 0x67)
                            {
                                (param_2 + -0xe) = 0x1;
                            }
                        }
                        else
                        {
                            (param_2 + -0xe) = 0x6;
                        }
                    }
                    pcVar11 = (param_2 + -0x216);
                    if((*(u8 *)(param_2 + -0x5) & 0x4) == 0x0)
                    {
                        (*(fn_ptr_1)PTR_s_3_wav_1050_25cc_1050_6068)();
                        piVar1  = (param_2 + 0xe);
                        *piVar1 = *piVar1 + 0x8;
                    }
                    else
                    {
                        (*(fn_ptr_1)PTR_s_3_wav_1050_25cc_1050_607c)();
                        piVar1  = (param_2 + 0xe);
                        *piVar1 = *piVar1 + 0xa;
                    }
                    if(((*(u8 *)(param_2 + -0x6) & 0x80) != 0x0) && ((param_2 + -0xe) == 0x0))
                    {
                        (*(fn_ptr_1)PTR_s_3_wav_1050_25cc_1050_6074)();
                    }
                    if((bVar6 == 0x67) && (((param_2 + -0x6) & 0x80) == 0x0))
                    {
                        (*(fn_ptr_1)PTR_s_3_wav_1050_25cc_1050_6070)();
                    }
                    if(*pcVar11 == '-')
                    {
                        pcVar11 = (param_2 + -0x215);
                        pbVar2  = (u8 *)(param_2 + -0x5);
                        *pbVar2 = *pbVar2 | 0x1;
                    }
                    iVar10  = -0x1;
                    pcVar14 = pcVar11;
                    do
                    {
                        if(iVar10 == 0x0)
                            break;
                        iVar10  = iVar10 + -0x1;
                        pcVar5  = pcVar14;
                        pcVar14 = pcVar14 + 0x1;
                    } while(*pcVar5 != '\0');
                    pcVar11 = pcVar14 + (-0x1 - pcVar11);
                }
            }
        }
    }
LAB_1000_3444:
    if((*(u8 *)(param_2 + -0x6) & 0x40) != 0x0)
    {
        if((*(u8 *)(param_2 + -0x5) & 0x1) == 0x0)
        {
            if((*(u8 *)(param_2 + -0x6) & 0x1) == 0x0)
            {
                if((*(u8 *)(param_2 + -0x6) & 0x2) != 0x0)
                {
                    *(param_2 + -0x10) = 0x20;
                    (param_2 + -0x12)  = 0x1;
                }
            }
            else
            {
                *(param_2 + -0x10) = 0x2b;
                (param_2 + -0x12)  = 0x1;
            }
        }
        else
        {
            *(param_2 + -0x10) = 0x2d;
            (param_2 + -0x12)  = 0x1;
        }
    }
    iVar8  = (param_2 + -0xc) - pcVar11;
    iVar10 = (param_2 + -0x12);
    iVar9  = iVar8 - iVar10;
    if(iVar8 < iVar10)
    {
        iVar9 = 0x0;
    }
    if((*(u8 *)(param_2 + -0x6) & 0xc) == 0x0)
    {
        pass1_1000_3552(iVar9, param_2, param_5);
    }
    pass1_1000_3534((param_2 + -0x12), param_2, param_5);
    if(((*(u8 *)(param_2 + -0x6) & 0x8) != 0x0) && ((*(u8 *)(param_2 + -0x6) & 0x4) == 0x0))
    {
        pass1_1000_3552(iVar9, param_2, param_5);
    }
    pass1_1000_3534(pcVar11, param_2, param_5);
    if((*(u8 *)(param_2 + -0x6) & 0x4) != 0x0)
    {
        pass1_1000_3552(iVar9, param_2, param_5);
    }
LAB_1000_30cf:
    pcVar5            = (param_2 + 0xa);
    cVar4             = *pcVar5;
    (param_2 + 0xa)   = pcVar5 + 0x1;
    *(param_2 + -0x4) = cVar4;
    if((cVar4 != '\0') && (-0x1 < (param_2 + -0xa)))
    {
        if((u8)(cVar4 - 0x20U) < 0x59)
        {
            bVar6 = *(u8 *)((u8)(cVar4 - 0x20U) + 0x5ffe) & 0xf;
        }
        else
        {
            bVar6 = 0x0;
        }
        bVar6                   = *(u8 *)((u8)(bVar6 * '\b' + *(param_2 + -0x7)) + 0x5ffe) >> 0x4;
        *(u8 *)(param_2 + -0x7) = bVar6;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar7 = (**(bVar6 * 0x2 + 0x30a4))();
        return uVar7;
    }
    return (param_2 + -0xa);
}


u16 pass1_1000_34cf(param_1: i16, param_2: u16)

{
     let mut uVar1: u16;
    u16 *puVar2;

    puVar2          = *(u16 **)(param_1 + 0xe);
    uVar1           = *puVar2;
    (param_1 + 0xe) = puVar2 + 0x2;
    return uVar1;
}


u32 pass1_1000_34d8(param_1: i16, param_2: u16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
    u16 *puVar3;

    puVar3          = (param_1 + 0xe);
    uVar1           = *puVar3;
    uVar2           = (puVar3 + 0x2);
    (param_1 + 0xe) = puVar3 + 0x4;
    return CONCAT22(uVar2, uVar1);
}


u32 pass1_1000_34e6(param_1: u16, param_2: i16, param_3: u16)

{
     let mut uVar1: u16;
    u32 uVar2;

    if((*(u8 *)(param_2 + -0x6) & 0x20) != 0x0)
    {
        uVar2 = pass1_1000_34d8(param_2, param_3);
        return uVar2;
    }
    uVar1 = pass1_1000_34cf(param_2, param_3);
    if(uVar1 == 0x0)
    {
        return param_1 << 0x10;
    }
    return CONCAT22(param_1, uVar1);
}


void pass1_1000_3534(param_1: i16, param_2: i16, param_3: u16)

{
     let mut piVar1: *mut i16;
     let mut puVar2: *mut u8;
     let mut uVar3: u16;
     let mut in_DX: u16;
     let mut unaff_DI: *mut u8;
     let mut uVar4: u16;
     let mut unaff_ES: u16;
     let mut unaff_CS: u16;
     let mut in_AF: u8;

    if(param_1 != 0x0)
    {
        piVar1  = (param_2 + -0xa);
        *piVar1 = *piVar1 + param_1;
        uVar4   = 0x0;
        do
        {
            puVar2   = unaff_DI;
            unaff_DI = unaff_DI + 0x1;
            uVar3    = pass1_1000_3503(*puVar2, in_DX, param_2, unaff_CS, param_3, in_AF);
            uVar4    = uVar4 | uVar3;
            param_1  = param_1 + -0x1;
        } while(param_1 != 0x0);
        if(uVar4 != 0x0)
        {
            (param_2 + -0xa) = 0xffff;
        }
    }
    return;
}
