
#include "unk_5.h"
#include "types.h"
#include "globals.h"
#include "structs/structs_45.h"
#include "utils.h"
#include "unk_6.h"
#include "struct_ops_5.h"

void  bad_1030_8cd2(void)

{
    return;
}


void  pass1_1030_8d08(param_1: u32, param_2: u16)

{
     let mut piVar1: *mut i16;
    u32 uVar2;
     let mut uVar3: u16;
     let mut uVar4: u16;
    u32        uVar5;
    u32        uStack16;
    i16        iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar4  = (param_1 >> 0x10);
        piVar1 = (param_1 + 0x1e);
        if(*piVar1 == iStack4 || *piVar1 < iStack4)
            break;
        uVar3                 = iStack4 * 0x6;
        uVar2                 = (param_1 + 0x1a);
        (uVar2 + uVar3 + 0x4) = 0x0;
        pass1_1028_e2ac(_PTR_LOOP_1050_65e2, 0x500);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, param_2);
        uStack16 = CONCAT22(param_2, uVar3);
        uVar5    = pass1_1028_e2e0(_PTR_LOOP_1050_65e2, param_2, 0x7);
        param_2  = (uVar5 >> 0x10);
        pass1_1030_7e5a(uStack16, uVar5 & 0xffff | param_2 << 0x10, param_2);
        iStack4 = iStack4 + 0x1;
    }
    return;
}


void  pass1_1030_8d9e(param_1: u32, param_2: u16)

{
    u8 local_c[0x2];
    u8 local_a[0x2];
    u8 local_8[0x6];

    pass1_1008_3e38(CONCAT22(param_2, local_8));
    pass1_1008_6d64((param_1 & 0xffff0000 | (param_1 + 0x28)), CONCAT22(param_2, local_8));
    pass1_1008_3e94(CONCAT22(param_2, local_8), CONCAT22(param_2, local_c), CONCAT22(param_2, local_a));
    return;
}


Struct18 * pass1_1030_8e12(Struct18 *param_1, param_2: u8)

{
    pass1_1030_8a2c(&param_1->field_0x0);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1030_8f04(param_1: u16, param_2: u16, param_3: u32, param_4: u32, param_5: u16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
     let mut uVar3: u16;
     let mut uVar4: u16;
    u32 uVar5;
     let mut uVar6: u16;
    i16 iStack8;
    u32 uStack6;

    pass1_1038_53ba(param_3, 0x1);
    if((((param_5 != 0x0) || (0x1 < param_4)) && ((pass1_1038_53ba(param_3, 0x2), param_5 != 0x0 || (0x1 < param_4)))) && ((pass1_1038_53ba(param_3, 0x3), param_5 != 0x0 || (0x1 < param_4))))
    {
        pass1_1038_53ba(param_3, 0x4);
        uVar5 = param_5;
        if((param_5 != 0x0) || (0x1 < param_4))
        {
            empty_1038_540a();
            uStack6 = param_4 & 0xffff | uVar5 << 0x10;
            iStack8 = 0x0;
            do
            {
                uVar3 = uVar5;
                uVar2 = param_4;
                if(0x0 < (iStack8 * 0x2 + globals._PTR_LOOP_1050_580e))
                {
                    empty_1038_540a();
                    uVar6   = (_PTR_LOOP_1050_580e >> 0x10);
                    uVar1   = (iStack8 * 0x2 + globals._PTR_LOOP_1050_580e);
                    param_4 = uVar1;
                    uVar4   = uVar1 >> 0xf;
                    uVar5   = uVar4;
                    if((uVar3 <= uVar4) && ((uVar3 < uVar4 || (uVar2 < uVar1))))
                    {
                        if(0x1c < iStack8)
                        {
                            return;
                        }
                        uVar2   = (iStack8 * 0x2 + globals._PTR_LOOP_1050_580e);
                        param_4 = SEXT24(uVar2);
                        uVar5   = param_4 >> 0x10;
                        if((long)uStack6 < (long)param_4)
                        {
                            return;
                        }
                        uStack6 = CONCAT22(((uStack6 >> 0x10) - (uVar2 >> 0xf)) - (uStack6 < uVar2), uStack6 - uVar2);
                    }
                }
                iStack8 = iStack8 + 0x1;
                if(0x24 < iStack8)
                {
                    return;
                }
            } while(true);
        }
    }
    return;
}


u32  pass1_1030_7c28(param_1: u32, param_2: u16, param_3: u16, param_4: u16, param_5: u16)

{
     let mut uVar1: u16;
    u32 uVar2;

    uVar1 = (param_1 >> 0x10);
    if((param_1 + 0x22) == 0x0)
    {
        return 0x0;
    }
    uVar2 = (param_1 + 0x22);
    uVar2 = pass1_1020_bae6(uVar2, CONCAT22(param_2, (uVar2 >> 0x10)), param_3, param_4, param_5);
    return uVar2;
}


void  pass1_1030_7c50(param_1: u32, long param_2, param_3: i16, param_4: u16, mut param_5: *mut u8)

{
     let mut piVar1: *mut i16;
    code       **ppcVar2;
     let mut uVar3: u16;
    u32          uVar4;
     let mut uVar5: u16;
     let mut puVar6: *mut u8;
     let mut extraout_DX: *mut u8;
     let mut extraout_DX_00: u16;
     let mut uVar7: u16;
     let mut extraout_DX_01: *mut u8;
    astruct_305 *iVar8;
     let mut uVar8: u16;
    u32          uVar9;
    u32  *puVar10;
    u32  *puStack18;

    uVar8  = (param_1 >> 0x10);
    iVar8  = (astruct_305 *)param_1;
    puVar6 = param_5;
    if(iVar8->field_0x1e == 0x0)
    {
        mem_op_1000_179c(0x18, param_5, 0x1000);
        puVar6 = (param_5 | param_4);
        if(puVar6 == 0x0)
        {
            iVar8->field_0x1e = 0x0;
        }
        else
        {
            struct_op_1030_1cd8(CONCAT22(param_5, param_4), 0x5, 0x5);
            &iVar8->field_0x1e         = param_4;
            (&iVar8->field_0x1e + 0x2) = extraout_DX;
            puVar6                     = extraout_DX;
        }
    }
    if(param_3 == 0x4)
    {
        piVar1  = &iVar8->field_0x34;
        *piVar1 = *piVar1 + param_2;
    }
    while(param_2 != 0x0)
    {
        uVar9   = pass1_1028_e2e0(_PTR_LOOP_1050_65e2, puVar6, 0x6);
        uVar3   = uVar9;
        uVar4   = uVar9 >> 0x10;
        puVar10 = iVar8->field_0x1e;
        ppcVar2 = (*iVar8->field_0x1e + 0xc);
        uVar5   = uVar3;
        (**ppcVar2)();
        uVar7 = extraout_DX_00;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, uVar4);
        puStack18 = CONCAT22(uVar7, uVar5);
        ppcVar2   = (*puStack18 + 0x14);
        (**ppcVar2)(&USHORT_1050_1028, uVar5, uVar7, param_1, puVar10, uVar9);
        puVar6  = extraout_DX_01;
        param_2 = param_2 + -0x1;
    }
    return;
}


BOOL16  pass1_1030_7ea0(param_1: u32)

{
    u32 uVar1;
     let mut uVar2: u16;
    BOOL16     BVar3;

    uVar2 = pass1_1030_6fa0(param_1);
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0xb);
    if(BVar3 != 0x0)
    {
        uVar1 = (param_1 + 0x1a);
        if((uVar1 + 0x12) == 0x5)
        {
            return 0x1;
        }
        BVar3 = 0x0;
    }
    return BVar3;
}


u32  pass1_1030_8086(param_1: u32)

{
     let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x18), (param_1 + 0x16)) & 0xffffff;
}


u16 * pass1_1030_80ee(mut param_1: *mut u16, param_2: u8, param_3: u16)

{
    pass1_1030_68dc(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        pass1_1000_093a(param_1, (param_1 >> 0x10), 0x1000);
    }
    return param_1;
}


void  pass1_1030_82f0(param_1: u16, param_2: u32, param_3: u32)

{
    pass1_1028_d078(param_1, *(param_2 + 0x4), param_3);
    return;
}


void  pass1_1030_8308(param_1: u16, param_2: u16, mut param_3: *mut u16, mut param_4: *mut u16, param_5: u32, param_6: u16, param_7: u16)

{
    pass1_1028_e198(_PTR_LOOP_1050_65e2, param_3, param_4, param_5, param_6, param_7);
    return;
}


u32  pass1_1030_8326(void)

{
    return CONCAT22((_PTR_LOOP_1050_65e2 + 0x2), *_PTR_LOOP_1050_65e2);
}


void  pass1_1030_8334(void)

{
    *_PTR_LOOP_1050_65e2 = 0x0;
    return;
}


void  pass1_1030_8344(param_1: u16, param_2: u16, param_3: u32)

{
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_3, (param_3 >> 0x10));
    return;
}


void  pass1_1030_8372(u32 **param_1, param_2: u32, u32 *param_3)

{
    pass1_1028_d52c(*param_1, param_2, param_3);
    return;
}


void vspri16f_op_1030_840a(param_1: u32, LPSTR param_2, WORD *param_3, param_4: u16)

{
    LPCSTR pCVar1;
     let mut unaff_ES: u16;
     let mut in_AF: u8;
    WORD  *args;

    if(PTR_LOOP_1050_574c != 0x0)
    {
        args = param_3;
        if(PTR_LOOP_1050_5750 == 0x0)
        {
            param_2 = &globals.PTR_LOOP_1050_1000;
            pCVar1  = &stack0x0008;
            pass1_1000_2b3c(s_simres_out_1050_5758, &USHORT_1050_1050, 0x5756, &USHORT_1050_1050, param_4, &stack0xfffe);
            globals._PTR_LOOP_1050_5752 = CONCAT22(param_4, pCVar1);
            globals.PTR_LOOP_1050_5750  = (&PTR_LOOP_1050_0000 + 0x1);
        }
        wvspri16f16(param_2, &stack0x0008, args);
        pass1_1000_2b5c(_PTR_LOOP_1050_5752, (_PTR_LOOP_1050_5752 >> 0x10), 0x5763, &USHORT_1050_1050, unaff_ES, &stack0xfffe, 0x1000, param_3);
        pass1_1000_2f48(_PTR_LOOP_1050_5752, &stack0xfffe, unaff_ES, 0x1000, param_3, in_AF);
    }
    return;
}


void  pass1_1030_861a(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16, param_6: u16)

{
    u32 *puStack6;

    pass1_1030_8854(param_1, param_2, param_3, param_6);
    puStack6 = CONCAT22(param_5, param_4);
    if((param_5 | param_4) == 0x0)
    {
        (param_1 + 0xa) = 0x0;
    }
    else
    {
        (param_1 + 0xa) = *puStack6;
    }
    return;
}


void  pass1_1030_8660(param_1: u32, u32 *param_2, param_3: u16, param_4: u16, param_5: u16, param_6: u16, param_7: i16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
     let mut uVar3: u16;
    u32 *puStack6;

    uVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    pass1_1030_8854(uVar2, uVar3, param_3, param_6);
    puStack6 = CONCAT22(param_5, param_4);
    uVar1    = param_5 | param_4;
    if(uVar1 == 0x0)
    {
        pass1_1030_8854(uVar2, uVar3, 0x0, param_6);
        puStack6 = CONCAT22(uVar1, param_4);
        uVar1    = uVar1 | param_4;
        if(uVar1 == 0x0)
        {
            pass1_1030_878c((long *)param_1, param_7, param_6);
            pass1_1030_8854(uVar2, uVar3, 0x0, param_6);
            puStack6 = CONCAT22(uVar1, param_4);
            if((uVar1 | param_4) == 0x0)
            {
                return;
            }
        }
        (puStack6 + 0x4) = param_3;
        *puStack6        = *param_2;
        pass1_1030_8834((u16 *)param_1, param_7, param_6);
    }
    else
    {
        *puStack6 = *param_2;
    }
    return;
}


void  pass1_1030_871e(long *param_1, u32 *param_2, param_3: u16, param_4: i16, param_5: u16)

{
     let mut piVar1: *mut i16;
    astruct_681 *iVar2;
     let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = (astruct_681 *)param_1;
    if(*param_1 == 0x0)
    {
        pass1_1030_878c((long *)(param_1 & 0xffff | uVar2 << 0x10), param_4, param_5);
    }
    piVar1                                    = &iVar2->field_0xe;
    *piVar1                                   = *piVar1 + 0x1;
    (*param_1 + iVar2->field_0xe * 0x6 + 0x4) = param_3;
    *(iVar2->field_0xe * 0x6 + *param_1)      = *param_2;
    return;
}


void  pass1_1030_877c(mut param_1: *mut u16, param_2: i16, param_3: u16)

{
    pass1_1030_8834(param_1, param_2, param_3);
    return;
}


void  pass1_1030_8834(mut param_1: *mut u16, param_2: i16, param_3: u16)

{
    u32 uVar1;
     let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x2);
    pass1_1000_4aea(*param_1, uVar1, (uVar1 >> 0x10), 0x6, 0x888e, &stack0xfffe, param_2, uVar2, 0x1000, param_3);
    return;
}


void  pass1_1030_8854(param_1: u16, param_2: u16, param_3: u16, param_4: u16)

{
    u32 uVar1;
    u32 local_c;
     let mut uStack8: u16;

    uStack8 = param_3;
    local_c = 0x0;
    uVar1   = (param_1 + 0x2);
    pass1_1000_49c6(&local_c, param_4, *_param_1, uVar1, (uVar1 >> 0x10), 0x6, 0x888e, &stack0xfffe);
    return;
}


u16 pass1_1030_888e(param_1: u32, param_2: u32)

{
     let mut piVar1: *mut i16;
    i16  iVar2;
     let mut uVar3: u16;
     let mut uVar4: u16;

    uVar3  = (param_1 >> 0x10);
    iVar2  = (param_1 + 0x4);
    uVar4  = (param_2 >> 0x10);
    piVar1 = (param_2 + 0x4);
    if(*piVar1 != iVar2 && iVar2 <= *piVar1)
    {
        return 0xffff;
    }
    if((param_2 + 0x4) < (param_1 + 0x4))
    {
        return 0x1;
    }
    return 0x0;
}


void  pass1_1030_88ce(mut param_1: *mut u16, param_2: u32, param_3: u32, param_4: u16)

{
     let mut puVar1: *mut u8;
     let mut puVar2: *mut u8;
    astruct_354 *iVar4;
     let mut uVar3: u16;
    u32          uVar4;
    u16         *puStack38;
    i16          iStack34;
    u8           local_20[0x2];
    i16          local_1e;
    i16          local_1c;
    u8           local_1a[0x6];
    u8           local_14[0x6];
    u32   uStack14;
    u32   uStack10;
    i16          iStack6;
     let mut uStack4: u16;

    uVar3            = (param_1 >> 0x10);
    iVar4            = (astruct_354 *)param_1;
    *param_1         = 0x389a;
    iVar4->field_0x2 = 0x1008;
    pass1_1030_84ae(param_1 & 0xffff0000 | &iVar4->field_0x4);
    iVar4->field_0x24 = param_3;
    puStack38         = (param_1 & 0xffff0000 | &iVar4->field_0x28);
    pass1_1008_6c90((param_1 & 0xffff0000 | &iVar4->field_0x28));
    &iVar4->field_0x34 = 0x0;
    *param_1           = 0x8e38;
    iVar4->field_0x2   = 0x1030;
    struct_1030_8544((param_1 & 0xffff0000 | &iVar4->field_0x4), param_2);
    uVar4    = pass1_1008_4772(iVar4->field_0x12);
    uStack4  = (uVar4 >> 0x10);
    iStack6  = uVar4;
    uStack10 = (iStack6 + 0x4);
    uStack14 = (iStack6 + 0x8);
    pass1_1008_3e54(CONCAT22(param_4, local_14), 0x0, uStack14 - 0x1, uStack10 - 0x1);
    pass1_1008_3e54(CONCAT22(param_4, local_1a), 0x0, 0x0, 0x0);
    pass1_1008_6d18(puStack38, CONCAT22(param_4, local_14), CONCAT22(param_4, local_1a));
    pass1_1008_6d64(puStack38, CONCAT22(param_4, local_1a));
    pass1_1008_3eb4(CONCAT22(param_4, local_1a), CONCAT22(param_4, local_20), CONCAT22(param_4, &local_1e), CONCAT22(param_4, &local_1c));
    puVar1            = (((long)local_1e * (long)local_1c) >> 0x10);
    uVar4             = (long)local_1e * (long)local_1c & 0xffff;
    iVar4->field_0x34 = uVar4;
    iVar4->field_0x36 = puVar1;
    for(iStack34 = 0x0; iStack34 < 0x5; iStack34 = iStack34 + 0x1)
    {
        mem_op_1000_179c(0x10, puVar1, 0x1000);
        puVar2 = (puVar1 | uVar4);
        if(puVar2 == 0x0)
        {
            (&iVar4[0x1].field_0x0 + iStack34 * 0x4) = 0x0;
        }
        else
        {
            pass1_1030_85be((long *)(uVar4 & 0xffff | ZEXT24(puVar1) << 0x10), 0x19, 0x64, uVar3, param_4);
            (&iVar4[0x1].field_0x0 + iStack34 * 0x4) = uVar4;
            (&iVar4[0x1].field_0x2)[iStack34 * 0x2]  = puVar2;
        }
        puVar1 = puVar2;
    }
    return;
}


void  pass1_1030_6b86(param_1: u32, param_2: u16, param_3: u16)

{
    code **ppcVar1;
    u32    uVar2;
     let mut extraout_DX: u16;
     let mut uVar3: u16;
     let mut extraout_DX_00: u16;
    i16    iVar4;
     let mut uVar5: u16;
    u32    uStack12;
    u32    uStack8;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if((iVar4 + 0x1e) == 0x0)
    {
        param_2 = 0x0;
        uVar3   = 0x0;
    }
    else
    {
        ppcVar1 = ((iVar4 + 0x1e) + 0x10);
        (**ppcVar1)();
        uVar3 = extraout_DX;
    }
    uStack8 = CONCAT22(uVar3, param_2);
    for(uStack12 = 0x0; uStack12 < uStack8; uStack12 = uStack12 + 0x1)
    {
        ppcVar1 = ((iVar4 + 0x1e) + 0x4);
        uVar2   = uStack8;
        (**ppcVar1)(param_3, (iVar4 + 0x1e));
        if((extraout_DX_00 | uVar2) != 0x0)
        {
            param_3 = SUB42(&USHORT_1050_1028, 0x0);
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, extraout_DX_00);
        }
    }
    return;
}


void  pass1_1030_6c1a(param_1: u32, param_2: i16)

{
     let mut piVar1: *mut i16;
    i16  iVar2;
    i16  iVar3;
     let mut uVar4: u16;

    uVar4          = (param_1 >> 0x10);
    iVar3          = param_1;
    iVar2          = (iVar3 + 0x32);
    (iVar3 + 0x32) = param_2;
    piVar1         = (iVar3 + 0x34);
    *piVar1        = *piVar1 + (param_2 - iVar2);
    iVar2          = (iVar3 + 0x32);
    if(iVar2 < 0x0)
    {
        iVar2 = 0x0;
    }
    (iVar3 + 0x32) = iVar2;
    return;
}


void  pass1_1030_6c4c(param_1: u32, param_2: i16)

{
    i16 iVar1;
     let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = (param_1 + 0x32);
    if(param_2 < iVar1)
    {
        iVar1 = param_2;
    }
    (param_1 + 0x34) = iVar1;
    return;
}


u32  pass1_1030_6d4e(param_1: u32, param_2: u16, param_3: u16, param_4: u16)

{
     let mut uVar1: u16;
     let mut uStack6: u16;
     let mut uStack4: u16;

    uStack6 = 0x0;
    uStack4 = 0x0;
    uVar1   = (param_1 >> 0x10);
    if((param_1 + 0x36) != 0x0)
    {
        pass1_1010_9092(*(param_1 + 0x36), param_2, param_4);
        uStack6 = param_2;
        uStack4 = param_3;
    }
    return CONCAT22(uStack4, uStack6);
}


void  pass1_1030_6d80(param_1: u32, param_2: u32)

{
    u32  *puVar1;
     let mut uVar2: u16;
    code       **ppcVar3;
    astruct_299 *iVar4;
     let mut uVar4: u16;

    uVar4  = (param_1 >> 0x10);
    iVar4  = (astruct_299 *)param_1;
    puVar1 = &iVar4->field_0x36;
    uVar2  = (&iVar4->field_0x36 + 0x2);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    iVar4->field_0x36 = param_2;
    return;
}


void  pass1_1030_6ddc(param_1: u32)

{
     let mut uVar1: u16;
    BOOL16 BVar2;

    uVar1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x1e);
    if(BVar2 != 0x0)
    {
        pass1_1030_d0c6(*(param_1 + 0x1a));
        return;
    }
    return;
}


void  pass1_1030_6e14(param_1: u32)

{
    u32 uVar1;
     let mut uVar2: u16;
    BOOL16     BVar3;

    uVar2 = pass1_1030_6fa0(param_1);
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar2, 0x1e);
    if(BVar3 != 0x0)
    {
        uVar1 = (param_1 + 0x1a);
        pass1_1030_d102(uVar1, (uVar1 >> 0x10));
        return;
    }
    return;
}


void  pass1_1030_6e9c(param_1: u32, long param_2, param_3: i16)

{
    code       **ppcVar1;
     let mut uVar2: u16;
     let mut uVar3: u16;
    u32          uVar4;
     let mut extraout_DX: u16;
     let mut extraout_DX_00: u16;
     let mut uVar5: u16;
    astruct_301 *iVar6;
     let mut uVar6: u16;
     let mut unaff_SS: u16;
    u32          uStack10;
    u32          uStack6;

    uVar6 = (param_1 >> 0x10);
    iVar6 = (astruct_301 *)param_1;
    uVar2 = (&iVar6->field_0x1e + 0x2) | &iVar6->field_0x1e;
    if(uVar2 != 0x0)
    {
        ppcVar1 = (*iVar6->field_0x1e + 0x10);
        (**ppcVar1)();
        uStack6 = CONCAT22(extraout_DX, uVar2);
        for(uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1)
        {
            ppcVar1 = (*iVar6->field_0x1e + 0x4);
            uVar4   = uStack6;
            (**ppcVar1)();
            uVar2 = uVar4;
            uVar5 = extraout_DX_00 | uVar2;
            if(uVar5 != 0x0)
            {
                uVar3 = uVar2;
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2, extraout_DX_00);
                if((uVar3 + 0xc) == param_3)
                {
                    param_2 = param_2 + -0x1;
                    pass1_1028_e332(_PTR_LOOP_1050_65e2, uVar2, extraout_DX_00, unaff_SS);
                    ppcVar1 = (*iVar6->field_0x1e + 0x8);
                    (**ppcVar1)(&USHORT_1050_1028, iVar6->field_0x1e, 0x0, uStack10);
                }
                if((param_2._2_2_ | param_2) == 0x0)
                {
                    return;
                }
            }
        }
    }
    return;
}


void  pass1_1030_6f5a(param_1: u32, param_2: u16)

{
     let mut uVar1: u16;
    BOOL16 BVar2;

    uVar1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x4);
    if(BVar2 != 0x0)
    {
        pass1_1028_6302(*(param_1 + 0x1a), param_2);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1030_73ee(param_1: u32, param_2: u32, param_3: u16)

{
    astruct_294 *iVar1;
     let mut uVar1: u16;

    uVar1             = (param_1 >> 0x10);
    iVar1             = (astruct_294 *)param_1;
    iVar1->field_0x2a = param_2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_2, (param_2 >> 0x10));
    iVar1->field_0x2e = param_2;
    iVar1->field_0x30 = param_3;
    return;
}


u32  pass1_1030_5b5c(param_1: i16, param_2: u16)

{
    return CONCAT22(param_2, param_1 + 0x14);
}


void  pass1_1030_5bec(param_1: u32)

{
    globals._PTR_LOOP_1050_5736 = param_1;
    pass1_1000_54a0(param_1, 0x0, 0x24);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1030_5c0e(void)

{
    globals._PTR_LOOP_1050_5736 = 0x0;
    return;
}

u16 * pass1_1030_5d0a(mut param_1: *mut u16)

{
     let mut uVar1: u16;

    struct_1030_17ce(param_1, 0x1, 0x4);
    uVar1            = (param_1 >> 0x10);
    (param_1 + 0x10) = 0x0;
    *param_1         = 0x613e;
    (param_1 + 0x2)  = 0x1030;
    return param_1;
}


u16 * pass1_1030_5d3c(mut param_1: *mut u16, param_2: u32, param_3: u16, mut param_4: *mut u8)

{
     let mut uVar1: u16;

    pass1_1030_183c(param_1, 0x1, 0x4, 0x1000000, param_2, param_3, param_4);
    uVar1            = (param_1 >> 0x10);
    (param_1 + 0x10) = 0x0;
    *param_1         = 0x613e;
    (param_1 + 0x2)  = 0x1030;
    return param_1;
}


void  pass1_1030_5d78(mut param_1: *mut u16)

{
     let mut uVar1: u16;
    Struct18 *paVar2;
    i16         iVar3;
     let mut uVar4: u16;

    uVar4         = (param_1 >> 0x10);
    iVar3         = param_1;
    *param_1      = 0x613e;
    (iVar3 + 0x2) = 0x1030;
    paVar2        = (iVar3 + 0x10);
    uVar1         = (iVar3 + 0x12);
    if((uVar1 | paVar2) != 0x0)
    {
        pass1_1030_8480((Struct18 **)(paVar2 & 0xffff | uVar1 << 0x10));
        fn_ptr_1000_17ce(paVar2, 0x1000);
    }
    pass1_1030_18b2(param_1);
    return;
}


void  pass1_1030_5fe2(param_1: u32, param_2: u32)

{
    *(param_1 + 0x10) = param_2;
    return;
}


void  pass1_1030_61b0(mut param_1: *mut u16)

{
     let mut uVar1: u16;
    u32 *puVar2;
    code      **ppcVar3;
    i16         iVar4;
     let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uVar1 = (iVar4 + 0x2);
    if((uVar1 | *param_1) != 0x0)
    {
        ppcVar3 = *param_1;
        (**ppcVar3)();
    }
    puVar2 = (iVar4 + 0x4);
    uVar1  = (iVar4 + 0x6);
    if((uVar1 | puVar2) != 0x0)
    {
        ppcVar3 = *puVar2;
        (**ppcVar3)();
    }
    globals._PTR_LOOP_1050_5740 = 0x0;
    return;
}


void  pass1_1030_61fe(param_1: u32, param_2: u32, param_3: u32, long param_4, param_5: u16, param_6: u16, param_7: u16)

{
    pass1_1030_677a(param_1, param_4, param_7);
    pass1_1030_8aa0(CONCAT22(param_6, param_5), param_2, param_3, param_6, param_7);
    return;
}


void  pass1_1030_627e(param_1: u16, param_2: u16, param_3: u16, param_4: u32, mut param_5: *mut u16, long param_6)

{
    u32 local_12[0x2];
    u32        uStack10;
    u32 uStack6;

    uStack6 = 0x0;
    pass1_1030_677a(param_4, param_6, param_1);
    uStack10 = CONCAT22(param_3, param_2);
    if((param_3 | param_2) != 0x0)
    {
        pass1_1030_8b00(uStack10, param_5, CONCAT22(param_1, local_12), param_1);
    }
    return;
}


void  pass1_1030_64ce(param_1: u16, param_2: u16, param_3: u16, param_4: u32, mut param_5: *mut u16, long param_6, u32 *param_7)

{
    u32 *puVar1;
     let mut uVar2: u16;
    u32  local_e;
    u32  uStack10;
    u32  uStack6;

    uStack6 = 0x0;
    pass1_1030_677a(param_4, param_6, param_1);
    uStack10 = CONCAT22(param_3, param_2);
    uVar2    = param_3 | param_2;
    if(uVar2 != 0x0)
    {
        puVar1 = &local_e;
        pass1_1030_8b00(uStack10, param_5, CONCAT22(param_1, puVar1), param_1);
        uStack6 = *puVar1;
    }
    *param_7 = uStack6;
    return;
}


void  pass1_1030_6522(u32 *param_1, param_2: u32, param_3: u32, param_4: u16)

{
    code      **ppcVar1;
    u32 *puVar2;
     let mut uVar3: u16;
     let mut extraout_DX: *mut u8;
     let mut puVar4: *mut u8;
     let mut extraout_DX_00: u16;
     let mut extraout_DX_01: u16;
     let mut uVar5: u16;
    u8          local_64[0xc];
     let mut uStack88: u16;
    u32  local_40;
    u32  uStack60;
     let mut uStack56: u16;
    u32 *puStack54;
     let mut puStack52: *mut u8;
    u32 *puStack50;
     let mut puStack48: *mut u8;
     let mut uStack46: u16;
    i16         iStack44;
    u8          local_2a[0x2];
    i16         local_28;
    i16         local_26;
     let mut local_24: u16;
    u8          local_22[0x2];
    u8          local_20[0x2];
     let mut local_1e: u16;
     let mut local_1c: u16;
     let mut local_1a: u16;
    u8          local_18[0x6];
    u8          local_12[0x6];
    u8          local_c[0x6];
    u32         uStack6;

    uVar5     = (param_1 >> 0x10);
    puVar2    = param_1;
    puVar4    = (param_1 + 0x2);
    puStack54 = puVar2;
    puStack52 = puVar4;
    puStack50 = puVar2;
    puStack48 = puVar4;
    if((puVar4 | puVar2) != 0x0)
    {
        ppcVar1 = *puVar2;
        (**ppcVar1)();
        puVar4 = extraout_DX;
    }
    mem_op_1000_179c(0x18, puVar4, 0x1000);
    puStack54 = puVar2;
    puStack52 = puVar4;
    if((puVar4 | puVar2) == 0x0)
    {
        puVar2 = 0x0;
        uVar3  = 0x0;
    }
    else
    {
        struct_op_1030_1cd8(CONCAT22(puVar4, puVar2), 0x5, 0x5);
        uVar3 = extraout_DX_00;
    }
    param_1 = puVar2;
    (param_1 + 0x2)         = uVar3;
    pass1_1030_677a(param_1, param_3, param_4);
    uStack6 = CONCAT22(uVar3, puVar2);
    if((uVar3 | puVar2) != 0x0)
    {
        pass1_1008_3e38(CONCAT22(param_4, local_c));
        pass1_1008_3e38(CONCAT22(param_4, local_12));
        pass1_1008_3e38(CONCAT22(param_4, local_18));
        pass1_1008_6d3e(param_2, CONCAT22(param_4, local_12), CONCAT22(param_4, local_c));
        pass1_1008_3eb4(CONCAT22(param_4, local_c), CONCAT22(param_4, &local_1e), CONCAT22(param_4, &local_1c), CONCAT22(param_4, &local_1a));
        pass1_1008_3eb4(CONCAT22(param_4, local_12), CONCAT22(param_4, &local_24), CONCAT22(param_4, local_22), CONCAT22(param_4, local_20));
        pass1_1008_6d64(param_2, CONCAT22(param_4, local_18));
        pass1_1008_3eb4(CONCAT22(param_4, local_18), CONCAT22(param_4, local_2a), CONCAT22(param_4, &local_28), CONCAT22(param_4, &local_26));
        if(local_24 == local_1e)
        {
            iStack44 = 0x0;
            for(uStack46 = local_1c; uVar3 = local_28 + local_1c, uStack46 < uVar3; uStack46 = uStack46 + 0x1)
            {
                for(uStack56 = local_1a; uStack56 < (local_26 + local_1a); uStack56 = uStack56 + 0x1)
                {
                    uStack88 = local_1e;
                    pass1_1008_3e54(CONCAT13((param_4 >> 0x8), CONCAT12(param_4, local_64)), local_1e, uStack46, uStack56);
                    pass1_1030_8b00(uStack6, CONCAT22(param_4, local_64), CONCAT22(param_4, &local_40), param_4);
                    uStack60 = local_40;
                    iStack44 = iStack44 + 0x1;
                    ppcVar1  = (*param_1 + 0x8);
                    (**ppcVar1)();
                }
            }
            ppcVar1 = (*param_1 + 0x10);
            (**ppcVar1)(0x1008, *param_1);
            if((extraout_DX_01 | uVar3) != 0x0)
            {
                return;
            }
        }
    }
    return;
}


void  pass1_1030_66de(param_1: u32, param_2: u32, param_3: u16)

{
    u32 uVar1;
    u8  local_a[0x8];

    pass1_1008_5784(CONCAT22(param_3, local_a), *(param_1 + 0x4));
    while(true)
    {
        uVar1 = pass1_1008_5b12(local_a, param_3);
        if(uVar1 == 0x0)
            break;
        pass1_1030_8bac(uVar1, param_2);
    }
    return;
}


void  pass1_1030_671c(param_1: u32, param_2: u32, mut param_3: *mut u16, long param_4, param_5: u16, param_6: u16, param_7: i16, param_8: u16)

{
    pass1_1030_677a(param_1, param_4, param_8);
    pass1_1030_8bdc(CONCAT22(param_6, param_5), param_2, param_3, param_7, param_8);
    return;
}


void  pass1_1030_6740(param_1: u32, param_2: u16, param_3: i16)

{
    u32 uVar1;
    u8  local_a[0x8];

    pass1_1008_5784(CONCAT22(param_2, local_a), *(param_1 + 0x4));
    while(true)
    {
        uVar1 = pass1_1008_5b12(local_a, param_2);
        if(uVar1 == 0x0)
            break;
        pass1_1030_8c38(uVar1, param_3, param_2);
    }
    return;
}


void  pass1_1030_677a(param_1: u32, long param_2, param_3: u16)

{
     let mut puVar1: *mut u8;
     let mut extraout_DX: u16;
     let mut uVar2: u16;
    u8  local_a[0x8];

    uVar2 = (param_1 >> 0x10);
    if((param_1 + 0x4) == 0x0)
    {
        return;
    }
    pass1_1008_5784(CONCAT22(param_3, local_a), *(param_1 + 0x4));
    do
    {
        puVar1 = local_a;
        pass1_1008_5b12(puVar1, param_3);
        if((extraout_DX | puVar1) == 0x0)
        {
            return;
        }
    } while((puVar1 + 0x24) != param_2);
    return;
}


void  pass1_1030_69cc(param_1: u32, param_2: u16, param_3: u16, param_4: u16)

{
     let mut uVar1: u16;
    BOOL16 BVar2;
    i16    iVar3;
     let mut uVar4: u16;
    u32    uVar5;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if((iVar3 + 0x3e) != 0x0)
    {
        return;
    }
    if(((iVar3 + 0x22) != 0x0) && (pass1_1020_ba94(*(long **)(iVar3 + 0x22)), (param_3 | param_2) != 0x0))
    {
        return;
    }
    uVar1 = pass1_1030_6fa0(param_1);
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x4);
    if((BVar2 != 0x0) && (uVar5 = pass1_1028_67d4(*(iVar3 + 0x1a), param_4), ((uVar5 >> 0x10) | uVar5) != 0x0))
    {
        return;
    }
    return;
}


void  pass1_1030_4bbe(param_1: u16, param_2: u16, param_3: u32, param_4: i16)

{
    u32  *puVar1;
    u32  *puVar2;
     let mut uVar3: u16;
    i16          iVar4;
    astruct_117 *iVar5;
    u32  *puVar5;
    u32  *puVar6;
     let mut uVar7: u16;

    uVar7 = (param_3 >> 0x10);
    iVar5 = (astruct_117 *)param_3;
    if(iVar5->field_0x12 == 0x0)
    {
        pass1_1030_4f5a(param_1, param_2, param_3 & 0xffff | uVar7 << 0x10);
    }
    puVar6 = &iVar5->field_0x16;
    uVar3  = (&iVar5->field_0x12 + 0x2);
    puVar5 = (&iVar5->field_0x12 + param_4 * 0x98);
    for(iVar4 = 0x26; iVar4 != 0x0; iVar4 = iVar4 + -0x1)
    {
        puVar2  = puVar6;
        puVar6  = puVar6 + 0x1;
        puVar1  = puVar5;
        puVar5  = puVar5 + 0x1;
        *puVar2 = *puVar1;
    }
    return;
}


void  pass1_1030_4c06(param_1: u32, param_2: i16, param_3: u16, param_4: u16)

{
    u32 *puVar1;
    u32 *puVar2;
     let mut uVar3: u16;
    u32 *puVar4;
    i16         iVar5;
    u32 *puVar6;
     let mut uVar7: u16;

    uVar7 = (param_1 >> 0x10);
    iVar5 = param_1;
    if((iVar5 + 0x15c) == 0x0)
    {
        pass1_1030_5044(param_1 & 0xffff | uVar7 << 0x10, param_4, param_3);
    }
    puVar4 = (iVar5 + 0xae);
    uVar3  = (iVar5 + 0x15e);
    puVar6 = ((iVar5 + 0x15c) + param_2 * 0xae);
    for(iVar5 = 0x2b; iVar5 != 0x0; iVar5 = iVar5 + -0x1)
    {
        puVar2  = puVar4;
        puVar4  = puVar4 + 0x1;
        puVar1  = puVar6;
        puVar6  = puVar6 + 0x1;
        *puVar2 = *puVar1;
    }
    puVar4 = puVar6;
    return;
}


void  pass1_1030_4c52(param_1: u16, param_2: u16, param_3: u32, param_4: u32, param_5: u16, param_6: u16)

{
     let mut uVar1: u16;
    i16   iVar2;
    i16   iVar3;
     let mut uVar4: u16;
     let mut pcStack8: String;
    i16   iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar1    = pass1_1000_47a4(param_4, 0x1050518a, param_6);
        pcStack8 = CONCAT22(param_5, uVar1);
        if((param_5 | uVar1) == 0x0)
            break;
        if(*pcStack8 != '\"')
        {
            iVar2 = pass1_1000_3e2c(CONCAT22(param_5, uVar1));
            iVar3 = param_3;
            uVar4 = (param_3 >> 0x10);
            if(iStack4 < 0x25)
            {
                (iStack4 * 0x4 + iVar3)       = iVar2;
                (iStack4 * 0x4 + iVar3 + 0x2) = param_5;
            }
            else
            {
                if(iStack4 == 0x25)
                {
                    (iVar3 + 0x94) = iVar2;
                }
                else
                {
                    if(iStack4 == 0x26)
                    {
                        (iVar3 + 0x96) = iVar2;
                    }
                    else
                    {
                        if(iStack4 == 0x27)
                        {
                            (iVar3 + 0x98) = iVar2;
                        }
                        else
                        {
                            if(iStack4 == 0x28)
                            {
                                (iVar3 + 0x9a) = iVar2;
                            }
                            else
                            {
                                if(iStack4 == 0x29)
                                {
                                    (iVar3 + 0x9c) = iVar2;
                                }
                                else
                                {
                                    if(iStack4 == 0x2a)
                                    {
                                        (iVar3 + 0x9e) = iVar2;
                                    }
                                    else
                                    {
                                        if(iStack4 == 0x2b)
                                        {
                                            (iVar3 + 0xa0) = iVar2;
                                        }
                                        else
                                        {
                                            if(iStack4 == 0x2c)
                                            {
                                                (iVar3 + 0xa2) = iVar2;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            iStack4 = iStack4 + 0x1;
        }
        param_4 = 0x0;
    }
    return;
}


void  pass1_1030_4d3a(param_1: u16, param_2: u16, param_3: u16, param_4: u32, param_5: u32)

{
     let mut uVar1: u16;
    i16          iVar2;
    astruct_118 *iVar3;
     let mut uVar3: u16;
     let mut unaff_SS: u16;
     let mut pcStack8: String;
    i16          iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar1    = pass1_1000_47a4(param_5, 0x1050518a, unaff_SS);
        pcStack8 = CONCAT22(param_1, uVar1);
        if((param_1 | uVar1) == 0x0)
            break;
        if(*pcStack8 != '\"')
        {
            iVar2 = pass1_1000_3e2c(CONCAT22(param_1, uVar1));
            iVar3 = (astruct_118 *)param_4;
            uVar3 = (param_4 >> 0x10);
            if(iStack4 < 0x25)
            {
                (&iVar3->field_0x0 + iStack4 * 0x4) = iVar2;
                (&iVar3->field_0x2 + iStack4 * 0x4) = param_1;
            }
            else
            {
                if(iStack4 == 0x25)
                {
                    iVar3->field_0x94 = iVar2;
                }
                else
                {
                    if(iStack4 == 0x26)
                    {
                        iVar3->field_0x96 = iVar2;
                    }
                }
            }
            iStack4 = iStack4 + 0x1;
        }
        param_5 = 0x0;
    }
    return;
}


void  pass1_1030_4e34(param_1: u16, param_2: u16, long param_3, param_4: &mut String)

{
    while(param_3 != 0x0)
    {
        if((*param_4 == '\r') || (*param_4 == '\n'))
        {
            *param_4 = '\0';
        }
        param_4 = (param_4 & 0xffff0000 | (param_4 + 0x1));
        param_3 = param_3 + -0x1;
    }
    return;
}


u32  pass1_1030_5164(param_1: u32, param_2: u32, param_3: u16)

{
     let mut uVar1: u16;
     let mut uVar2: u16;
    long lVar3;
    u8   local_a[0x8];

    pass1_1008_5784(CONCAT22(param_3, local_a), *(param_1 + 0x568));
    do
    {
        lVar3 = pass1_1008_5b12(local_a, param_3);
        if(lVar3 == 0x0)
        {
            return param_2;
        }
        uVar1 = param_1 + 0x168;
        unk_str_op_1000_3d3e((param_1 & 0xffff0000 | uVar1), (lVar3 + 0x4));
        pass1_1000_3cea(param_1 & 0xffff0000 | uVar1, param_2);
        uVar2 = dos3_call_1000_51aa(&stack0xfffe);
    } while(uVar2 != 0x0);
    return param_1 & 0xffff0000 | uVar1;
}


void pass1_1030_51eb(void)

{
     let mut unaff_SS: u16;

    pass1_1030_3b28(unaff_SS);
    return;
}


u16  pass1_1030_5260(param_1: u32, param_2: u16, param_3: u16)

{
    u32  uVar1;
    code      **ppcVar2;
    u32 *puStack6;

    uVar1 = (param_1 + 0x108);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    puStack6 = CONCAT22(param_3, param_2);
    ppcVar2  = (*puStack6 + 0x14);
    (**ppcVar2)();
    return 0x1;
}


void  pass1_1030_53f4(param_1: u32, param_2: u16, param_3: u16, param_4: u8)

{
    u32 uVar1;
     let mut uVar2: u16;
    i16        iVar3;
     let mut uVar4: u16;
    u32        uVar5;
     let mut bStack291: u8;
    u8         local_11e[0x10e];
    u32 uStack16;
    u32 uStack12;

    uVar4          = (param_1 >> 0x10);
    iVar3          = param_1;
    uStack12       = (iVar3 + 0x108);
    uStack12._3_1_ = (uStack12 >> 0x18);
    if(uStack12._3_1_ == -0x1)
    {
        uVar5   = pass1_1028_e2e0(_PTR_LOOP_1050_65e2, param_2, (u8)((iVar3 + 0x108) >> 0x18));
        param_2 = (uVar5 >> 0x10);
    }
    else
    {
        uStack16       = (iVar3 + 0x108);
        uStack16._3_1_ = (uStack16 >> 0x18);
        if(uStack16._3_1_ == '\x03')
        {
            pass1_1028_e44a(_PTR_LOOP_1050_65e2, (iVar3 + 0x108), param_3);
        }
        else
        {
            uVar1 = (iVar3 + 0x108);
            pass1_1028_e372(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10), param_3);
        }
    }
    uStack12       = (iVar3 + 0x108);
    uStack12._3_1_ = (uStack12 >> 0x18);
    if(uStack12._3_1_ != '\x03')
    {
        pass1_1030_521c((astruct_100 *)CONCAT13((param_3 >> 0x8), CONCAT12(param_3, local_11e)), *(iVar3 + 0x108), param_3, param_4);
        uStack16 = *_PTR_LOOP_1050_5748;
        fn_ptr_1028_d566(uStack16, CONCAT22(param_3, local_11e));
        bStack291 = (u8)((iVar3 + 0x108) >> 0x18);
        uVar2     = bStack291;
        if(bStack291 == 0x2)
        {
            uVar1 = (iVar3 + 0x108);
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
            pass1_1010_82f8(_PTR_LOOP_1050_14cc, **(u16 **)(uVar2 + 0x10));
        }
    }
    return;
}


void  pass1_1030_5a52(param_1: u32, u32 *param_2, u32 *param_3)

{
    u32 uVar1;
     let mut uVar2: u16;

    uVar2    = (param_1 >> 0x10);
    uVar1    = (param_1 + 0x10);
    *param_3 = *(uVar1 + 0xe);
    uVar1    = (param_1 + 0x10);
    *param_2 = *(uVar1 + 0x12);
    return;
}


void  pass1_1030_5a80(param_1: u32, param_2: u32, param_3: u16)

{
    u32       *puVar1;
     let mut uVar2: u16;
    u32        uVar3;
    u8         local_20[0xc];
    u32        local_14;
    u32 uStack14;
    u32 uStack10;
    i16        iStack6;
     let mut uStack4: u16;

    uVar2             = (param_1 >> 0x10);
    *(param_1 + 0x10) = param_2;
    uVar3             = pass1_1008_4772(*(astruct_76 **)(param_2 + 0xe));
    uStack4           = (uVar3 >> 0x10);
    iStack6           = uVar3;
    uStack10          = (iStack6 + 0x4);
    uStack14          = (iStack6 + 0x8);
    pass1_1008_3e54(CONCAT22(param_3, &local_14), 0x0, uStack14 - 0x1, uStack10 - 0x1);
    puVar1 = (param_1 + 0x14);
    pass1_1008_6cb4(CONCAT22(param_3, local_20), &local_14, param_3, puVar1, uVar2);
    pass1_1008_6d64(CONCAT22(param_3, local_20), (param_1 & 0xffff0000 | ZEXT24(puVar1)));
    return;
}


i16  pass1_1030_5b00(param_1: u32)

{
    return (param_1 + 0x4) + 0xb;
}


void  pass1_1030_5b1c(param_1: u32, mut param_2: *mut u16, mut param_3: *mut u16)

{
     let mut uVar1: u16;

    uVar1    = (param_1 >> 0x10);
    *param_3 = (param_1 + 0x1a);
    *param_2 = (param_1 + 0x1c);
    return;
}


void  pass1_1030_5b3e(param_1: u32, param_2: i16, param_3: u16)

{
    i16 iVar1;
     let mut uVar2: u16;

    uVar2          = (param_1 >> 0x10);
    iVar1          = param_1;
    (iVar1 + 0x1a) = param_3;
    if((iVar1 + 0x1c) < param_2)
    {
        (iVar1 + 0x1c) = param_2;
    }
    return;
}


void  pass1_1030_3006(param_1: u32, param_2: u32)

{
    *(param_1 + 0x10) = param_2;
    return;
}


void  pass1_1030_3258(param_1: u32, param_2: u16)

{
    (param_1 + 0x1ae) = param_2;
    return;
}


void  pass1_1030_326a(param_1: u32, param_2: u32, param_3: u16, param_4: u16)

{
     let mut uVar1: u16;
    u32          uVar2;
     let mut uVar3: u16;
    astruct_692 *iVar4;
     let mut uVar4: u16;
    long         lStack6;

    uVar4 = (param_1 >> 0x10);
    iVar4 = (astruct_692 *)param_1;
    if(iVar4->field_0x1aa == 0x0)
    {
        iVar4->field_0x1aa = 0x1;
    }
    else
    {
        param_2            = iVar4->field_0x1aa * 0x2;
        iVar4->field_0x1aa = param_2;
    }
    uVar1 = param_2;
    pass1_1030_38b8();
    lStack6 = CONCAT22(param_3, uVar1);
    uVar2   = iVar4->field_0x1aa;
    uVar3   = (&iVar4->field_0x1aa + 0x2);
    if(lStack6 < (long)uVar2)
    {
        uVar2 = uVar1;
        uVar3 = param_3;
    }
    &iVar4->field_0x1aa         = uVar2;
    (&iVar4->field_0x1aa + 0x2) = uVar3;
    pass1_1030_375a(param_1 & 0xffff | uVar4 << 0x10, 0x0, uVar2 & 0xffff | uVar3 << 0x10, param_4);
    return;
}


void  pass1_1030_3534(param_1: u32, param_2: u32)

{
    *(param_1 + 0x4) = param_2;
    return;
}


void  pass1_1030_3548(param_1: u32, long param_2)

{
    long *plVar1;

    plVar1  = (long *)(param_1 + 0x4);
    *plVar1 = *plVar1 + param_2;
    return;
}


void  pass1_1030_355c(param_1: u32, param_2: u32)

{
    i16 iVar1;
     let mut uVar2: u16;
    i16 iStack4;

    iStack4 = 0x0;
    do
    {
        iVar1                   = iStack4 * 0x4;
        uVar2                   = (param_1 >> 0x10);
        (param_1 + iVar1 + 0x4) = (iVar1 + param_2) + (param_1 + 0x4 + iVar1);
        iStack4                 = iStack4 + 0x1;
    } while(iStack4 < 0x5b);
    return;
}


void  pass1_1030_35a4(param_1: u32, long param_2, mut param_3: *mut u8, param_4: u16, param_5: u16)

{
    u16       *puVar1;
    u8       **ppuVar2;
     let mut uVar3: u16;
     let mut puVar4: *mut u8;
     let mut uVar5: u16;
     let mut uVar6: u16;
    u32        uVar7;
     let mut puVar8: *mut u8;
     let mut uVar9: u16;
     let mut uVar10: u8;
     let mut uVar11: u8;
    u8         local_c[0x2];
    u32 local_a;
    u32 uStack6;

    vspri16f_op_1030_840a(s_Pop_Leaving__ld_1050_516a, param_4, param_5, param_3);
    if(_PTR_LOOP_1050_5f2c == 0x0)
    {
        globals.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_3, 0x1000);
        globals.PTR_LOOP_1050_5f2e = param_3;
    }
    else
    {
    }
    uVar5   = fn_ptr_op_1000_1708(0x16c, 0x0, 0x1, globals.PTR_LOOP_1050_5f2c, globals.PTR_LOOP_1050_5f2e, 0x1000);
    uStack6 = CONCAT22(PTR_LOOP_1050_5f2e, uVar5);
    uVar10  = (undefined)param_5;
    uVar11  = (undefined)(param_5 >> 0x8);
    pass1_1030_3948(param_1, CONCAT22(param_5, local_c), CONCAT13(uVar11, CONCAT12(uVar10, &local_a)), 0x3);
    uVar7 = (&local_a + 0x2U);
    pass1_1030_3948(param_1, CONCAT22(param_5, &local_a + 0x2U), CONCAT13(uVar11, CONCAT12(uVar10, local_c)), 0x4);
    do
    {
        uVar6 = uVar7;
        if(param_2 < 0x1)
            break;
        pass1_1008_612e(local_a, (local_a >> 0x10), uVar6);
        uVar7 = ZEXT24(&param_2);
        pass1_1030_3a3a(param_1, (long *)CONCAT13(uVar11, CONCAT12(uVar10, &param_2)), uVar6);
        uVar9    = (uStack6 >> 0x10);
        puVar1   = (uVar6 * 0x4 + uStack6);
        uVar3    = *puVar1;
        *puVar1  = *puVar1 + uVar7;
        ppuVar2  = (u8 **)(uVar6 * 0x4 + uStack6 + 0x2);
        *ppuVar2 = globals.PTR_LOOP_1050_5f2e + (*ppuVar2 + CARRY2(uVar3, uVar7));
        pass1_1030_38f2(param_1, 0x3, param_5);
        uVar6  = uVar7;
        puVar8 = globals.PTR_LOOP_1050_5f2e;
        pass1_1030_38f2(param_1, 0x4, param_5);
        puVar4                      = globals.PTR_LOOP_1050_5f2e + puVar8;
        globals.PTR_LOOP_1050_5f2e = puVar8;
    } while(((puVar4 + CARRY2(uVar6, uVar7)) | uVar6 + uVar7) != 0x0);
    pass1_1000_4906((Struct20 *)(param_1 & 0xffff0000 | (param_1 + 0x18c)), 0x0, 0x18);
    return;
}


void  pass1_1030_3694(param_1: u32, param_2: i16, long param_3, mut param_4: *mut u8, param_5: u16, param_6: u16)

{
    u16 *puVar1;
    u8 **ppuVar2;
     let mut uVar3: u16;
     let mut uVar4: u16;
     let mut uVar5: u16;
    u32  uVar6;
     let mut puVar7: *mut u8;

    vspri16f_op_1030_840a(s_Pop_Leaving__ld_1050_517a, param_5, param_6, param_4);
    if(_PTR_LOOP_1050_5f2c == 0x0)
    {
        globals.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_4, 0x1000);
        globals.PTR_LOOP_1050_5f2e = param_4;
    }
    else
    {
    }
    uVar4  = fn_ptr_op_1000_1708(0x16c, 0x0, 0x1, globals.PTR_LOOP_1050_5f2c, globals.PTR_LOOP_1050_5f2e, 0x1000);
    uVar6  = (param_2 - 0x1U);
    puVar7 = globals.PTR_LOOP_1050_5f2e;
    if(((param_2 < 0x1) || (SBORROW2(param_2, 0x1))) || (uVar6 = (param_2 - 0x5U), param_2 - 0x5U != 0x0 && 0x3 < (param_2 - 0x1U)))
    {
        while(uVar5 = uVar6, 0x0 < param_3)
        {
            pass1_1008_612e(0x0, 0x5a, uVar5);
            uVar6 = ZEXT24(&param_3);
            pass1_1030_3a3a(param_1, (long *)CONCAT13((param_6 >> 0x8), CONCAT12(param_6, &param_3)), uVar5);
            puVar1   = (uVar5 * 0x4 + uVar4);
            uVar3    = *puVar1;
            *puVar1  = *puVar1 + uVar6;
            ppuVar2  = (u8 **)(uVar5 * 0x4 + uVar4 + 0x2);
            *ppuVar2 = puVar7 + (*ppuVar2 + CARRY2(uVar3, uVar6));
        }
    }
    else
    {
        pass1_1030_39dc(param_1, (long *)CONCAT22(param_6, &param_3), CONCAT13((PTR_LOOP_1050_5f2e >> 0x8), CONCAT12(PTR_LOOP_1050_5f2e, uVar4)), param_2);
    }
    pass1_1000_4906((Struct20 *)(param_1 & 0xffff0000 | (param_1 + 0x18c)), 0x0, 0x18);
    return;
}


void  pass1_1030_375a(param_1: u32, param_2: i16, long param_3, param_4: u16)

{
    i16        iVar1;
    i16        iVar2;
     let mut uVar3: u16;
    long       lVar4;
    long       lVar5;
    i16        iVar6;
    i16        iVar7;
     let mut uVar8: u16;
    i16        iStack20;
    u32 uStack18;
    i16        local_6;
    i16        local_4;

    iVar6 = param_1;
    if(param_2 == 0x0)
    {
        local_4 = 0x5a;
        while((-0x1 < local_4 && (pass1_1030_3a3a(param_1, (long *)CONCAT22(param_4, &param_3), local_4), param_3 != 0x0)))
        {
            local_4 = local_4 + -0x1;
        }
    }
    else
    {
        pass1_1030_3948(param_1, CONCAT22(param_4, &local_4), CONCAT22(param_4, &local_6), param_2);
        iVar2    = (local_4 - local_6) + 0x1;
        lVar4    = param_3 / (long)iVar2;
        lVar5    = lVar4 * iVar2;
        uVar3    = lVar5;
        uStack18 = CONCAT22(((param_3 >> 0x10) - (lVar5 >> 0x10)) - (param_3 < uVar3), param_3 - uVar3);
        for(iStack20 = local_6; iStack20 <= local_4; iStack20 = iStack20 + 0x1)
        {
            iVar7                 = iStack20 * 0x4;
            uVar8                 = (param_1 >> 0x10);
            (iVar6 + iVar7 + 0x4) = (iVar6 + iVar7 + 0x4) - lVar4;
            iVar2                 = (iVar6 + iVar7 + 0x6);
            if((uStack18._2_2_ | uStack18) != 0x0)
            {
                iVar1                 = (iVar6 + iVar7 + 0x4);
                (iVar6 + iVar7 + 0x4) = iVar1 + -0x1;
                (iVar6 + iVar7 + 0x6) = iVar2 - (iVar1 == 0x0);
                uStack18              = uStack18 + -0x1;
            }
            if((iVar6 + iStack20 * 0x4 + 0x6) < 0x0)
            {
                (iVar6 + iStack20 * 0x4 + 0x4) = 0x0;
            }
        }
    }
    pass1_1000_4906((Struct20 *)(param_1 & 0xffff0000 | (iVar6 + 0x18c)), 0x0, 0x18);
    return;
}


void  pass1_1030_387c(param_1: u32)

{
    i16 iStack4;

    iStack4 = 0x5a;
    do
    {
        (iStack4 * 0x4 + param_1 + 0x4) = (iStack4 * 0x4 + param_1);
        iStack4                         = iStack4 + -0x1;
    } while(0x0 < iStack4);
    (param_1 + 0x4) = 0x0;
    return;
}


void  pass1_1030_38b8(void)

{
    i16 iStack8;

    iStack8 = 0x0;
    do
    {
        iStack8 = iStack8 + 0x1;
    } while(iStack8 < 0x5b);
    return;
}


void  pass1_1030_38f2(param_1: u32, param_2: i16, param_3: u16)

{
    i16        iStack12;
    i16        local_a;
    i16        local_8;
    u32 uStack6;

    uStack6 = 0x0;
    pass1_1030_3948(param_1, CONCAT22(param_3, &local_a), CONCAT22(param_3, &local_8), param_2);
    for(iStack12 = local_8; iStack12 <= local_a; iStack12 = iStack12 + 0x1)
    {
    }
    return;
}


void  pass1_1030_3948(param_1: u32, mut param_2: *mut u16, i16 *param_3, param_4: i16)

{
     let mut uVar1: u16;

    if(param_4 == 0x1)
    {
        *param_3 = 0x0;
        *param_2 = 0x3;
        return;
    }
    uVar1 = (param_1 >> 0x10);
    if(param_4 == 0x2)
    {
        *param_3 = 0x4;
        *param_2 = (param_1 + 0x1ae);
        return;
    }
    if(param_4 == 0x3)
    {
        *param_3 = (param_1 + 0x1ae) + 0x1;
        *param_2 = 0x27;
        return;
    }
    if(param_4 != 0x4)
    {
        if(param_4 == 0x5)
        {
            *param_3 = 0x4c;
        }
        else
        {
            *param_3 = 0x0;
        }
        *param_2 = 0x5a;
        return;
    }
    *param_3 = 0x28;
    *param_2 = 0x4b;
    return;
}


void  pass1_1030_39dc(param_1: u32, long *param_2, param_3: u32, param_4: i16)

{
    i16 iVar1;
     let mut in_DX: u16;
     let mut uVar2: u16;
     let mut unaff_SS: u16;
    i16 iStack8;
    i16 local_6;
    i16 local_4;

    pass1_1030_3948(param_1, CONCAT22(unaff_SS, &local_6), CONCAT22(unaff_SS, &local_4), param_4);
    iStack8 = local_6;
    while(true)
    {
        if(iStack8 < local_4)
        {
            return;
        }
        iVar1 = local_4;
        pass1_1030_3a3a(param_1, param_2, iStack8);
        uVar2                           = (param_3 >> 0x10);
        (iStack8 * 0x4 + param_3)       = iVar1;
        (iStack8 * 0x4 + param_3 + 0x2) = in_DX;
        if(*param_2 == 0x0)
            break;
        iStack8 = iStack8 + -0x1;
    }
    return;
}


void  pass1_1030_3a3a(param_1: u32, long *param_2, param_3: i16)

{
     let mut piVar1: *mut i16;
    i16  iVar2;
    i16  iVar3;
     let mut uVar4: u16;
     let mut uVar5: u16;
    i16  iVar6;
    i16  iVar7;
    i16  iVar8;
     let mut uVar9: u16;

    iVar2  = (param_2 + 0x2);
    uVar9  = (param_1 >> 0x10);
    iVar6  = param_1;
    iVar7  = iVar6 + 0x4;
    iVar8  = param_3 * 0x4;
    piVar1 = (iVar7 + iVar8 + 0x2);
    iVar3  = *piVar1;
    if((iVar3 < iVar2) || ((uVar5 = *param_2, *piVar1 == iVar2 || iVar3 < iVar2 && ((iVar7 + iVar8) < uVar5))))
    {
        *param_2                      = *param_2 - (iVar6 + 0x4 + param_3 * 0x4);
        (iVar6 + param_3 * 0x4 + 0x4) = 0x0;
    }
    else
    {
        uVar4                 = (iVar7 + iVar8);
        iVar3                 = (iVar7 + iVar8 + 0x2);
        (iVar6 + iVar8 + 0x4) = uVar4 - uVar5;
        (iVar6 + iVar8 + 0x6) = (iVar3 - iVar2) - (uVar4 < uVar5);
        *param_2              = 0x0;
    }
    return;
}
