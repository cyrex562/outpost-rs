#include "types.h"
#include "unk_10.h"
#include "structs_1.h"

typedef struct struct_1040_d76e_1 {
   let mut field_0x94: u16;
  u16 *field_0x98;
   let mut field_0x9c: u16;
} struct_1040_d76e_1;

void  pass1_1040_d76e(struct_1040_d76e_1 *param_1)

{
    u32 uVar1;
//    u8*        param_1_lo;
//    u8*        param_1_hi;

//    param_1_hi = (param_1 >> 0x10);
//    param_1_lo = param_1;
    uVar1 = (param_1->field_0x94);
    pass1_1018_5742(uVar1, (uVar1 >> 0x10), (param_1->field_0x9c), *(param_1->field_0x98));
    (param_1->field_0x9c) = 0x0;
}


void  pass1_1040_d0f8(Struct57 *param_1, param_2: u16)

{
     let mut uVar1: u16;
     let mut in_DX: *mut u8;
     let mut uVar2: u16;
     let mut puVar3: *mut u8;
     let mut puVar4: *mut u8;
    astruct_438 *iVar5;
    i16          unaff_DI;
     let mut uVar5: u16;
     let mut unaff_SS: u16;
    u16         *puVar6;
    u32          uVar7;
    astruct_392 *iVar8;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x1845));
    uVar5              = (param_1 >> 0x10);
    iVar5              = (astruct_438 *)param_1;
    &iVar5->field_0x94 = 0x0;
    iVar5->field_0x98  = globals._PTR_LOOP_1050_5f16;
    &iVar5->field_0x9c = 0x0;
    iVar5->field_0xa0  = 0x0;
    param_1            = 0xd8c4;
    iVar5->field_0x2   = &PTR_LOOP_1050_1040;
    puVar6             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x47, unaff_SS, in_DX, unaff_DI);
    uVar2              = (puVar6 >> 0x10);
    iVar5->field_0x94  = puVar6;
    iVar5->field_0x96  = uVar2;
    uVar7              = pass1_1018_5732(iVar5->field_0x94, uVar2, iVar5->field_0x98, puVar6, uVar2, unaff_SS);
    puVar3             = (uVar7 >> 0x10);
    iVar5->field_0x9c  = uVar7;
    iVar5->field_0x9e  = puVar3;
    uVar1              = puVar3 | iVar5->field_0x9c;
    if(uVar1 == 0x0)
    {
        mem_op_1000_179c(0xc, puVar3, 0x1000);
        puVar4 = (puVar3 | uVar1);
        if(puVar4 == 0x0)
        {
            &iVar5->field_0x9c = 0x0;
        }
        else
        {
            pass1_1010_8ef2(CONCAT22(puVar3, uVar1), puVar4, unaff_SS);
            iVar5->field_0x9c = uVar1;
            iVar5->field_0x9e = puVar4;
        }
    }
    return;
}


void  pass1_1040_ca16(Struct57 *param_1, param_2: u16, mut param_3: *mut u8, param_4: i16, param_5: u16)

{
    astruct_727 *iVar1;
     let mut uVar1: u16;
    u16         *puVar2;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x1840));
    uVar1              = (param_1 >> 0x10);
    iVar1              = (astruct_727 *)param_1;
    iVar1->field_0x94  = globals._PTR_LOOP_1050_5f0c;
    &iVar1->field_0x98 = 0x0;
    iVar1->field_0x9c  = 0x0;
    iVar1->field_0x9e  = 0x0;
    param_1            = 0xd07c;
    iVar1->field_0x2   = &PTR_LOOP_1050_1040;
    puVar2             = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3e, param_5, param_3, param_4);
    iVar1->field_0x98  = puVar2;
    iVar1->field_0x9a  = (puVar2 >> 0x10);
    return;
}


u16 * pass1_1040_c9cc(mut param_1: *mut u16, param_2: u8)

{
    pass1_1040_c5ac(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1040_c71e(param_1: u32, param_2: u16)

{
    i16 iVar1;
     let mut uVar2: u16;

    pass1_1040_9252(param_1, param_2);
    uVar2          = (param_1 >> 0x10);
    iVar1          = param_1;
    (iVar1 + 0x28) = (iVar1 + 0x24) / 0x2 - (iVar1 + 0x2c) / 0x2;
    return;
}


void  pass1_1040_c630(u32 *param_1, param_2: u16, param_3: u16)

{
    i16          iVar1;
    code       **ppcVar2;
    u32   uVar3;
    u32          uVar4;
    astruct_165 *iVar4;
     let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = (astruct_165 *)param_1;
    uVar3 = iVar4->field_0x42;
    if((uVar3 + 0x12) != 0x71)
    {
        iVar4->field_0x36 = 0x5;
        iVar4->field_0x26 = 0x5;
        iVar4->field_0x28 = 0x5;
        iVar1             = iVar4->field_0x36;
        iVar4->field_0x30 = iVar1;
        iVar4->field_0x2e = iVar1;
        if(PTR_LOOP_1050_5f02 == 0x0)
        {
            globals._PTR_LOOP_1050_5f04 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0xff, param_3);
            param_2             = 0x1010;
            globals._PTR_LOOP_1050_5f08 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, 0x100, param_3);
        }
        globals.PTR_LOOP_1050_5f02 = globals.PTR_LOOP_1050_5f02 + 0x1;
        iVar4->field_0x8   = globals._PTR_LOOP_1050_5f04;
        iVar4->field_0xc   = globals._PTR_LOOP_1050_5f08;
        pass1_1040_9618(param_1);
        iVar4->field_0x20 = 0x0;
        iVar4->field_0x1e = 0xc8;
        iVar4->field_0x22 = 0xa0;
        iVar4->field_0x24 = iVar4->field_0x2c + iVar4->field_0x36;
        iVar4->field_0x2e = iVar4->field_0x36 * 0x3 + iVar4->field_0x2a;
        iVar4->field_0x30 = iVar4->field_0x36;
        iVar4->field_0x32 = iVar4->field_0x22 - iVar4->field_0x36;
        iVar4->field_0x3c = 0x25;
        uVar4             = *param_1;
        ppcVar2           = (uVar4 + 0x4);
        (**ppcVar2)(param_2, param_1);
        ppcVar2 = (uVar4 + 0x8);
        (**ppcVar2)(param_2, param_1, uVar5);
    }
    return;
}


u16  pass1_1040_c60e(param_1: u32)

{
    u32 uVar1;
     let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if((param_1 + 0x42) != 0x0)
    {
        uVar1 = (param_1 + 0x42);
        return (uVar1 + 0x12);
    }
    return 0x0;
}


u32  pass1_1040_c518(param_1: u32, param_2: u8, param_3: u16)

{
    pass1_1040_bf92(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1040_bf92(mut param_1: *mut u16, param_2: u16)

{
    astruct_514 *iVar1;
     let mut uVar1: u16;

    uVar1            = (param_1 >> 0x10);
    iVar1            = (astruct_514 *)param_1;
    *param_1         = 0xc53e;
    iVar1->field_0x2 = &PTR_LOOP_1050_1040;
    pass1_1010_1ea6(iVar1->field_0x6, param_1 & 0xffff | uVar1 << 0x10, param_2);
    unk_destroy_win_op_1010_2fa0(iVar1->field_0x6, 0x1010);
    *param_1         = 0x3ab0;
    iVar1->field_0x2 = 0x1008;
    *param_1         = 0x389a;
    iVar1->field_0x2 = 0x1008;
    return;
}


void  pass1_1040_bfde(param_1: u32, u32 *param_2, param_3: u16)

{
    code     **ppcVar1;
    u32 uVar2;
    i16        iVar3;
     let mut uVar4: u16;

    uVar4                  = (param_1 >> 0x10);
    iVar3                  = param_1;
    (iVar3 + 0x6) = param_2;
    ppcVar1                = (*param_2 + 0x4);
    (**ppcVar1)();
    uVar2          = (iVar3 + 0x6);
    (uVar2 + 0x22) = (iVar3 + 0x4);
    pass1_1010_2ee2((iVar3 + 0x6), param_3, 0x1010);
    return;
}


u16  pass1_1040_bb5a(param_1: u32)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = ((param_1 + 0x94) + 0x8);
    (**ppcVar1)();
    return 0x0;
}


void  pass1_1040_b8be(u32 *param_1)

{
    fn_ptr_1 *ppcVar1;

    ppcVar1 = (*param_1 + 0x80);
    (**ppcVar1)();
    return;
}


u16  pass1_1040_b316(u32 *param_1, param_2: u16, param_3: u16, param_4: u16, param_5: i16)

{
    fn_ptr_1 *ppcVar1;
     let mut uStack4: u16;

    if(param_5 == 0xf)
    {
        ppcVar1 = (*param_1 + 0x60);
        uStack4 = (**ppcVar1)();
    }
    else
    {
        if(param_5 == 0x111)
        {
            ppcVar1 = (*param_1 + 0x10);
            (**ppcVar1)();
            uStack4 = 0x1;
        }
        else
        {
            uStack4 = pass1_1040_79c0(param_1, param_2, param_3, param_4, param_5);
        }
    }
    return uStack4;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1040_b17c(param_1: u32, param_2: u32, mut param_3: *mut u8, param_4: i16, param_5: i16, param_6: u16)

{
     let mut piVar1: *mut i16;
    u32 uVar2;
     let mut pcVar3: String;
     let mut uVar4: u16;
    i16        iVar5;
     let mut uVar6: u16;
     let mut puVar7: *mut u8;
    u16       *puVar8;
    u16       *puStack12;
    i16        iStack4;

    iStack4 = 0x0;
    while(true)
    {
        uVar6  = (param_1 >> 0x10);
        iVar5  = param_1;
        piVar1 = *(i16 **)(iVar5 + 0x90);
        puVar7 = (piVar1 >> 0x10);
        if(*piVar1 == iStack4 || *piVar1 < iStack4)
            break;
        param_5                       = (iStack4 * 0x2 + param_2);
        uVar2                         = (piVar1 + 0x2);
        (iStack4 * 0xa + uVar2 + 0x4) = param_5;
        iStack4                       = iStack4 + 0x1;
        param_3                       = puVar7;
    }
    puVar8    = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3, param_6, param_3, param_5);
    uVar4     = (puVar8 >> 0x10);
    uVar2     = (iVar5 + 0x90);
    puStack12 = *(u16 **)(uVar2 + 0x2);
    for(iStack4 = 0x0; piVar1 = *(i16 **)(iVar5 + 0x90), *piVar1 != iStack4 && iStack4 <= *piVar1; iStack4 = iStack4 + 0x1)
    {
        uVar2  = (iVar5 + 0x90);
        uVar2  = (uVar2 + 0x6);
        pcVar3 = pass1_1010_b038(puVar8, uVar2, (uVar2 >> 0x10), (puStack12 + 0x4), param_4);
        string_1040_a626(puStack12, CONCAT22(uVar4, pcVar3), uVar4);
        puStack12 = (puStack12 & 0xffff0000 | (puStack12 + 0xa));
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  pass1_1040_ac84(Struct57 *param_1, param_2: u16, mut param_3: *mut u8, param_4: i16, param_5: u16)

{
    astruct_726 *iVar1;
     let mut uVar1: u16;
    u16         *puVar2;

    struct_1040_b082(param_1, CONCAT22(param_2, 0x1f3));
    uVar1               = (param_1 >> 0x10);
    iVar1               = (astruct_726 *)param_1;
    iVar1->field_0x94   = 0x0;
    &iVar1->field_0x98  = 0x0;
    param_1             = 0xafc4;
    iVar1->field_0x2    = &PTR_LOOP_1050_1040;
    iVar1->field_0x94   = globals._PTR_LOOP_1050_5ef0;
    globals._PTR_LOOP_1050_5ef0 = 0x0;
    puVar2              = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3d, param_5, param_3, param_4);
    iVar1->field_0x98   = puVar2;
    iVar1->field_0x9a   = (puVar2 >> 0x10);
    return;
}


u16 * pass1_1040_a204(mut param_1: *mut u16, param_2: u8)

{
    *param_1        = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((Struct18 *)param_1, 0x1000);
    }
    return param_1;
}


u32  pass1_1040_a2cc(param_1: i16, param_2: u32, param_3: u32, param_4: u16, mut param_5: *mut u8, param_6: u16, param_7: u16)

{
     let mut uVar1: u16;

    if(param_3._2_2_ == 0x1826)
    {
        if((param_3 == 0x1) || ((0x1 < param_3 - 0x1U && (param_3 - 0x3U < 0x2))))
        {
            uVar1 = 0x1;
        }
        else
        {
            uVar1 = 0x0;
        }
        return uVar1;
    }
    pass1_1040_b54a(param_1, param_2, (param_2 >> 0x10), param_3, param_5, param_6, param_7);
    return CONCAT22(param_5, param_4);
}


void  pass1_1040_8b3c(param_1: u16, param_2: u32, param_3: u32, param_4: u16)

{
    if((param_3._2_2_ != 0x0) && ((param_3._2_2_ == (&PTR_LOOP_1050_0000 + 0x1) || param_3._2_2_ == &PTR_LOOP_1050_0002 || (((&PTR_LOOP_1050_0002 + 0x1U) < param_3._2_2_ + -0x2 && (param_3._2_2_ + -0x6 < &PTR_LOOP_1050_0002))))))
    {
        globals.PTR_LOOP_1050_5df4 = 0x0;
        globals.PTR_LOOP_1050_5df8 = param_3._2_2_;
        return;
    }
    post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), (param_2 >> 0x10), param_3, param_3._2_2_, param_4);
    return;
}


u16 * pass1_1040_8e58(param_1: i16, param_2: u16, param_3: u16, param_4: u32)

{
    pass1_1040_b040(CONCAT22(param_2, param_1), CONCAT22(param_4, param_3), (param_4 >> 0x10));
    CONCAT22(param_2, param_1) = 0x8f3c;
    (param_1 + 0x2)            = &PTR_LOOP_1050_1040;
    return CONCAT22(param_2, param_1);
}


void  pass1_1040_9422(u32 *param_1)

{
    code **ppcVar1;
     let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if((param_1 + 0x8) != 0x0)
    {
        ppcVar1 = (*param_1 + 0x10);
        (**ppcVar1)();
    }
    if((param_1 + 0x4) != 0x0)
    {
        ppcVar1 = (*param_1 + 0x14);
        (**ppcVar1)();
    }
    return;
}


void  pass1_1040_9618(param_1: u32)

{
     let mut uVar1: u16;
    astruct_162 *iVar2;
     let mut uVar2: u16;
    u32          uVar3;

    uVar2             = (param_1 >> 0x10);
    iVar2             = (astruct_162 *)param_1;
    uVar3             = pass1_1008_4772(iVar2->field_0x8);
    uVar1             = (uVar3 >> 0x10);
    iVar2->field_0x2a = (uVar3 + 0x4);
    iVar2->field_0x2c = (uVar3 + 0x8);
    return;
}


u16  pass1_1040_824a(param_1: u32, param_2: i16)

{
    if((param_1 + 0x6) != param_2)
    {
        return 0x1;
    }
    return 0x0;
}


void  pass1_1040_807e(param_1: u32, param_2: u16, param_3: u16)

{
     let mut uVar1: u16;
    code       **ppcVar2;
    u32  *puVar3;
    u32  *puVar4;
     let mut in_DX: *mut u8;
     let mut uVar5: u16;
     let mut extraout_DX: *mut u8;
     let mut puVar6: *mut u8;
     let mut extraout_DX_00: *mut u8;
     let mut puVar7: *mut u8;
    astruct_395 *iVar9;
     let mut uVar8: u16;
    astruct_43  *paVar9;
    u32          uStack10;
    astruct_393 *iVar8;

    if(param_2 == 0x1)
    {
        pass1_1040_805a(in_DX);
        return;
    }
    paVar9 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc, param_2, param_3);
    uVar5  = (paVar9 >> 0x10);
    puVar3 = paVar9;
    if((uVar5 | puVar3) != 0x0)
    {
        ppcVar2 = (paVar9 + 0x14);
        puVar4  = puVar3;
        (**ppcVar2)(0x1010, puVar3, uVar5);
        uStack10 = CONCAT22(extraout_DX, puVar4);
        uVar8    = (param_1 >> 0x10);
        iVar9    = (astruct_395 *)param_1;
        puVar6   = extraout_DX;
        if(iVar9->field_0x70 != (astruct_90 *)0x0)
        {
            puVar4 = &iVar9->field_0x70;
            uVar1  = (&iVar9->field_0x70 + 0x2);
            puVar6 = (uVar1 | puVar4);
            if(puVar6 != 0x0)
            {
                ppcVar2 = *puVar4;
                (**ppcVar2)();
                puVar6 = extraout_DX_00;
            }
        }
        mem_op_1000_179c(0x14, puVar6, 0x1000);
        puVar7 = (puVar6 | puVar4);
        if(puVar7 == 0x0)
        {
            puVar4 = 0x0;
            puVar7 = 0x0;
        }
        else
        {
            struct_1008_4c58(CONCAT22(puVar6, puVar4));
        }
        &iVar9->field_0x70 = puVar4;
        (&iVar9->field_0x70 + 0x2)         = puVar7;
        pass1_1008_4d84(iVar9->field_0x70, uStack10, puVar7);
        if(paVar9 != (astruct_43 *)0x0)
        {
            ppcVar2 = paVar9;
            (**ppcVar2)(0x1008, puVar3, uVar5, 0x1);
        }
        return;
    }
    return;
}


u32  pass1_1040_805a(mut param_1: *mut u8)

{
    i16 unaff_DI;
     let mut uVar1: u16;
     let mut unaff_SS: u16;

    if(_PTR_LOOP_1050_4230 == 0x0)
    {
        mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x28, unaff_SS, param_1, unaff_DI);
    }
    uVar1 = (_PTR_LOOP_1050_4230 >> 0x10);
    return CONCAT22((_PTR_LOOP_1050_4230 + 0x10), (_PTR_LOOP_1050_4230 + 0xe));
}


u16  pass1_1040_8054(void)

{
    return 0x0;
}


void  pass1_1040_78de(void)

{
    return;
}


void  pass1_1040_741e(param_1: u32, param_2: u16)

{
    u32 *puVar1;
     let mut uVar2: u16;
    code      **ppcVar3;
    i16         iVar4;
     let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1010_1ea6(*(iVar4 + 0x94), param_1 & 0xffff | uVar5 << 0x10, param_2);
    puVar1 = (iVar4 + 0x98);
    uVar2  = (iVar4 + 0x9a);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1010, puVar1, uVar2, 0x1);
    }
    (iVar4 + 0x98) = 0x0;
    (iVar4 + 0x94) = 0x0;
    return;
}


i16  pass1_1040_5eaa(param_1: u32)

{
    i16 iVar1;
     let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    switch((iVar1 + 0x9a))
    {
    case 0x0:
    case 0x70:
    case 0x71:
        (iVar1 + 0x98) = 0x0;
        return iVar1;
    case 0x1:
    case 0x2:
        (iVar1 + 0x98) = 0xd;
        return iVar1;
    case 0x3:
        (iVar1 + 0x98) = 0xe;
        return iVar1;
    case 0x4:
    case 0x4b:
        (iVar1 + 0x98) = 0xf;
        break;
    case 0x5:
        (iVar1 + 0x98) = 0x10;
        return iVar1;
    case 0x6:
        (iVar1 + 0x98) = 0x11;
        return iVar1;
    case 0x7:
        (iVar1 + 0x98) = 0x12;
        break;
    case 0x8:
        (iVar1 + 0x98) = 0x13;
        break;
    case 0x9:
    case 0xa:
    case 0xb:
        (iVar1 + 0x98) = 0x14;
        break;
    case 0xc:
        (iVar1 + 0x98) = 0x18;
        break;
    case 0xd:
        (iVar1 + 0x98) = 0x19;
        break;
    case 0xe:
    case 0x76:
        (iVar1 + 0x98) = 0x17;
        break;
    case 0xf:
    case 0x10:
    case 0x11:
        (iVar1 + 0x98) = 0x1a;
        break;
    case 0x12:
        (iVar1 + 0x98) = 0x1b;
        break;
    case 0x13:
        (iVar1 + 0x98) = 0x1c;
        break;
    case 0x14:
        (iVar1 + 0x98) = 0x1d;
        break;
    case 0x15:
    case 0x16:
    case 0x17:
    case 0x18:
    case 0x19:
        (iVar1 + 0x98) = 0x1e;
        break;
    case 0x1a:
        (iVar1 + 0x98) = 0x1f;
        break;
    case 0x1b:
        (iVar1 + 0x98) = 0x20;
        break;
    case 0x1c:
    case 0x1d:
    case 0x1e:
        (iVar1 + 0x98) = 0x21;
        break;
    case 0x1f:
        (iVar1 + 0x98) = 0x22;
        break;
    case 0x20:
        (iVar1 + 0x98) = 0x23;
        break;
    case 0x21:
        (iVar1 + 0x98) = 0x24;
        break;
    case 0x22:
        (iVar1 + 0x98) = 0x25;
        break;
    case 0x23:
    case 0x24:
    case 0x25:
    case 0x26:
    case 0x27:
    case 0x28:
    case 0x29:
    case 0x2a:
    case 0x2b:
        (iVar1 + 0x98) = 0x26;
        break;
    case 0x2c:
        (iVar1 + 0x98) = 0x27;
        break;
    case 0x2d:
        (iVar1 + 0x98) = 0x28;
        break;
    case 0x2e:
    case 0x2f:
    case 0x30:
    case 0x31:
        (iVar1 + 0x98) = 0x29;
        break;
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x4d:
        (iVar1 + 0x98) = 0x2a;
        break;
    case 0x36:
        (iVar1 + 0x98) = 0x2b;
        break;
    case 0x37:
    case 0x38:
    case 0x39:
        (iVar1 + 0x98) = 0x2c;
        break;
    case 0x3a:
        (iVar1 + 0x98) = 0x2d;
        break;
    case 0x3b:
    case 0x3c:
        (iVar1 + 0x98) = 0x2e;
        break;
    case 0x3d:
        (iVar1 + 0x98) = 0x2f;
        break;
    case 0x3e:
        (iVar1 + 0x98) = 0x30;
        break;
    case 0x3f:
        (iVar1 + 0x98) = 0x31;
        break;
    case 0x40:
        (iVar1 + 0x98) = 0x32;
        break;
    case 0x41:
        (iVar1 + 0x98) = 0x33;
        break;
    case 0x42:
        (iVar1 + 0x98) = 0x34;
        break;
    case 0x43:
        (iVar1 + 0x98) = 0x35;
        break;
    case 0x44:
        (iVar1 + 0x98) = 0x36;
        break;
    case 0x45:
        (iVar1 + 0x98) = 0x37;
        break;
    case 0x46:
        (iVar1 + 0x98) = 0x38;
        break;
    case 0x47:
        (iVar1 + 0x98) = 0x39;
        break;
    case 0x48:
    case 0x49:
    case 0x4a:
        (iVar1 + 0x98) = 0x3a;
        break;
    case 0x4c:
        (iVar1 + 0x98) = 0x3b;
        break;
    case 0x4e:
        (iVar1 + 0x98) = 0x3c;
        break;
    case 0x4f:
    case 0x50:
        (iVar1 + 0x98) = 0x3d;
        break;
    case 0x51:
    case 0x52:
    case 0x53:
    case 0x54:
    case 0x55:
        (iVar1 + 0x98) = 0x3e;
        break;
    case 0x56:
    case 0x57:
    case 0x58:
    case 0x59:
    case 0x5a:
        (iVar1 + 0x98) = 0x3f;
        break;
    case 0x5b:
        (iVar1 + 0x98) = 0x40;
        break;
    case 0x5c:
    case 0x5d:
    case 0x5e:
        (iVar1 + 0x98) = 0x41;
        break;
    case 0x5f:
    case 0x60:
    case 0x61:
        (iVar1 + 0x98) = 0x42;
        break;
    case 0x62:
    case 0x63:
    case 0x64:
    case 0x65:
    case 0x66:
        (iVar1 + 0x98) = 0x43;
        break;
    case 0x67:
    case 0x68:
        (iVar1 + 0x98) = 0x44;
        break;
    case 0x69:
        (iVar1 + 0x98) = 0x45;
        break;
    case 0x6a:
        (iVar1 + 0x98) = 0x46;
        break;
    case 0x6b:
        (iVar1 + 0x98) = 0x47;
        break;
    case 0x6c:
        (iVar1 + 0x98) = 0x48;
        break;
    case 0x6d:
        (iVar1 + 0x98) = 0x49;
        break;
    case 0x6e:
        (iVar1 + 0x98) = 0x4a;
        break;
    case 0x6f:
        (iVar1 + 0x98) = 0x4b;
        break;
    case 0x74:
        (iVar1 + 0x98) = 0x15;
        break;
    case 0x75:
        (iVar1 + 0x98) = 0x16;
        break;
    case 0x78:
    case 0x7a:
    case 0x7b:
    case 0x7c:
    case 0x7d:
    case 0x7e:
    case 0x7f:
    case 0x80:
    case 0x81:
    case 0x82:
        (iVar1 + 0x98) = 0x4c;
    }
    return iVar1;
}


void  pass1_1040_6402(Struct57 *param_1, param_2: u16, mut param_3: *mut u8, param_4: i16, param_5: u16)

{
    code       **ppcVar1;
    astruct_725 *iVar2;
     let mut uVar2: u16;
    u16         *puVar3;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0x1850, param_2);
    uVar2                      = (param_1 >> 0x10);
    iVar2                      = (astruct_725 *)param_1;
    iVar2->field_0x8e          = 0x0;
    iVar2->field_0x92          = 0x0;
    param_1                    = 0x67ba;
    iVar2->field_0x2           = &PTR_LOOP_1050_1040;
    puVar3                     = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2b, param_5, param_3, param_4);
    &iVar2->field_0x92         = puVar3;
    (&iVar2->field_0x92 + 0x2) = (puVar3 >> 0x10);
    ppcVar1                    = (*iVar2->field_0x92 + 0x4);
    (**ppcVar1)();
    return;
}


void  pass1_1040_6470(Struct18 *param_1, param_2: u16)

{
    Struct18 *iVar1;
     let mut uVar1: u16;

    uVar1              = (param_1 >> 0x10);
    iVar1              = (Struct18 *)param_1;
    param_1->field_0x0 = 0x67ba;
    iVar1->field_0x2   = &PTR_LOOP_1050_1040;
    if(&iVar1->field_0x92 != 0x0)
    {
        pass1_1010_1ea6(*&iVar1->field_0x92, (long)param_1, param_2);
    }
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, iVar1->field_0x6);
    fn_ptr_1000_17ce(&iVar1->field_0x8e, 0x1000);
    ui_cleanup_op_1040_782c(param_1, 0x1000);
    return;
}


Struct18 * pass1_1040_6794(Struct18 *param_1, param_2: u8, param_3: u16)

{
    pass1_1040_6470(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


void  pass1_1040_6826(Struct57 *param_1, param_2: u16)

{
    i16 iVar1;
     let mut uVar2: u16;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_2, 0xfda));
    uVar2          = (param_1 >> 0x10);
    iVar1          = param_1;
    (iVar1 + 0x94) = 0x0;
    (iVar1 + 0x98) = 0x0;
    param_1        = 0x6f32;
    (iVar1 + 0x2)  = &PTR_LOOP_1050_1040;
    return;
}


u16  pass1_1040_68d2(u32 *param_1, i16 *param_2, param_3: u16, param_4: u16, param_5: i16, param_6: u16)

{
    fn_ptr_1 *ppcVar1;
     let mut uVar2: u16;

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


void  pass1_1040_6cac(param_1: u32, param_2: u16)

{
    u32 *puVar1;
     let mut uVar2: u16;
    code      **ppcVar3;
    i16         iVar4;
     let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1010_1ea6(*(iVar4 + 0x94), param_1 & 0xffff | uVar5 << 0x10, param_2);
    puVar1 = (iVar4 + 0x98);
    uVar2  = (iVar4 + 0x9a);
    if((uVar2 | puVar1) != 0x0)
    {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1010, puVar1, uVar2, 0x1);
    }
    (iVar4 + 0x98) = 0x0;
    (iVar4 + 0x94) = 0x0;
    return;
}


void  pass1_1040_6fb6(Struct57 *param_1, param_2: u16)

{
    i16 iVar1;
     let mut uVar2: u16;

    pass1_1040_b0bc(param_1, 0x0, CONCAT22(param_2, 0xfd9));
    uVar2          = (param_1 >> 0x10);
    iVar1          = param_1;
    (iVar1 + 0x94) = 0x0;
    (iVar1 + 0x98) = 0x0;
    param_1        = 0x76a4;
    (iVar1 + 0x2)  = &PTR_LOOP_1050_1040;
    return;
}


void  pass1_1040_4d7e(param_1: u32)

{
    u32 uVar1;
     let mut piVar2: *mut i16;
     let mut uVar3: u16;
    i16        iStack8;
    u32       *puStack6;

    uVar3    = (param_1 >> 0x10);
    uVar1    = (param_1 + 0x90);
    puStack6 = (uVar1 + 0x2);
    iStack8  = 0x0;
    while((piVar2 = *(i16 **)(param_1 + 0x90), *piVar2 != iStack8 && iStack8 <= *piVar2 && ((puStack6 + 0x4) != 0x1770)))
    {
        iStack8  = iStack8 + 0x1;
        puStack6 = (puStack6 & 0xffff0000 | (puStack6 + 0xa));
    }
    pass1_1000_3e2c(*puStack6);
    return;
}
