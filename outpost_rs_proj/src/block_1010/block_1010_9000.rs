
pub fn pass1_1010_9044(mut param_1: u32)

{
  code **ppcVar1;

  ppcVar1 = (code **)((param_1 + 0x4) + 0x10);
  (**ppcVar1)();
  return;
}
pub fn fn_ptr_1010_905e(param_1: *mut astruct_169,mut param_2: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_169 *iVar4;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_169 *)param_1;
  puVar1 = (u32*)&iVar4->field4_0x4;
  uVar2 = (&iVar4->field4_0x4 + 0x2);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  iVar4->field4_0x4 = param_2;
  return;
}
pub fn pass1_1010_9092(mut param_1: u16 ,mut param_2: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_EDX: *mut Struct57;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut in_stack_0000fe7c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut in_stack_0000ffa6: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffd4: u16;
  let mut uVar9: u32;
  let mut uStack14: u32;
  let mut uStack6: u32;
  let mut paVar6: *mut Struct57;

  uVar8 = (param_2 >> 0x10);
  iVar7 = param_2;
  uVar9 = (iVar7 + 0x4);
  ppcVar1 = (code **)((iVar7 + 0x4) + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(in_EDX,param_1);
  mem_op_1000_179c(0xc,in_EDX);
  uVar3 = in_EDX | param_1;
  paVar6 = (astruct_57 *)(in_EDX & 0xffff0000 | uVar3);
  if (uVar3 == 0x0) {
    param_1 = 0x0;
    paVar6 = NULL;
  }
  else {
    pass1_1010_8ef2(paVar6,(astruct_170 *)CONCAT22(in_EDX,param_1),in_stack_0000ffd4,in_stack_0000fe7c,
                    in_stack_0000ffa0,in_stack_0000ffa6,in_stack_0000ffaa);
  }
  uVar4 = SUB42(paVar6,0x0);
  uStack14 = 0x0;
  while( true ) {
    uVar3 = paVar6;
    if (uStack6 <= uStack14) break;
    ppcVar1 = (code **)((iVar7 + 0x4) + 0x4);
    uVar2 = uStack6;
    (**ppcVar1)(0x1000,(iVar7 + 0x4),uStack14,uVar9);
    uVar5 = uVar3 | uVar2;
    paVar6 = (astruct_57 *)uVar5;
    if (uVar5 != 0x0) {
      ppcVar1 = (code **)((param_1 + 0x4) + 0xc);
      (**ppcVar1)(0x1000,(param_1 + 0x4),uVar2,uVar3);
    }
    uStack14 += 0x1;
  }
  return;
}
pub fn pass1_1010_9130(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,u8 *param_4)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_3 >> 0x10);
  pass1_1030_1d58((param_3 + 0x4));
  if ((param_2 | param_1) != NULL) {
    uVar1 = (param_3 + 0x8);
    pass1_1010_c3c2((param_2 | param_1),uVar1,(uVar1 >> 0x10),param_4,
                    CONCAT22(param_2,param_1));
    return;
  }
  *param_4 = '\0';
  return;
}
pub fn struct_1010_9172(param_1: *mut astruct_249,mut param_2: u32)

{
  code **ppcVar1;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut paVar4: *mut Struct57;
  astruct_249 *iVar4;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (astruct_249 *)param_1;
  puVar2 = iVar4->field4_0x4;
  uVar3 = iVar4->field5_0x6;
  paVar4 = (astruct_57 *)(param_2 & 0xffff0000 | uVar3);
  if ((uVar3 | puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    puVar2 = (**ppcVar1)();
  }
  mem_op_1000_179c(0x18,paVar4);
  uVar3 = paVar4 | puVar2;
  if (uVar3 == 0x0) {
    puVar2 = NULL;
    uVar3 = 0x0;
  }
  else {
    puVar2 = struct_op_1030_1cd8((astruct_75 *)CONCAT22(paVar4,puVar2),0x5,0x5);
  }
  iVar4->field4_0x4 = puVar2;
  iVar4->field5_0x6 = uVar3;
  return;
}
pub fn pass1_1010_91cc(mut param_1: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  i32 lVar3;

  uVar2 = (param_1 >> 0x10);
  ppcVar1 = (code **)((param_1 + 0x4) + 0x10);
  lVar3 = (**ppcVar1)();
  if (lVar3 != 0x0) {
    ppcVar1 = (code **)((param_1 + 0x4) + 0x8);
    (**ppcVar1)();
  }
  return;
}
pub fn pass1_1010_9210(mut param_1: u32)

{
  code **ppcVar1;

  ppcVar1 = (code **)((param_1 + 0x4) + 0xc);
  (**ppcVar1)();
  return;
}



StructD * pass1_1010_922e(StructD *param_1,param_2: u8)

{
  pass1_1010_8f78(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_223 * pass1_1010_9258(param_1: *mut astruct_223)

{
  struct_1010_383a(param_1);
  param_1->field0_0x0 = 0x958e;
  (param_1 + 0x2) = 0x1010;
  return param_1;
}
pub fn pass1_1010_927a(param_1: *mut u16)

{
  *param_1 = 0x958e;
  (param_1 + 0x2) = 0x1010;
  pass1_1010_3880((StructD *)param_1);
  return;
}



pub fn pass1_1010_9298(StructD *param_1,param_2: *mut astruct_19,mut param_3: u16 ) -> u32

{
  let mut uVar1: u16;
  let mut paVar2: *mut Struct57;

  uVar1 = (param_1 >> 0x10);
  paVar2 = (astruct_57 *)(param_1 & 0xffff0000 | param_1 & 0xffff);
  struct_1010_2cd2(param_2,param_3);
    //        1010:9566  40  95  10  10      addr         pass1_1010_9540
  param_2->offset_0x0 = 0x9566;
  (param_2 + 0x2) = 0x1010;
  mem_op_1000_179c(0x20c,paVar2);
  (param_2 + 0x5c) = uVar1;
  (param_2 + 0x5e) = paVar2;
  pass1_1000_4906((StructD *)CONCAT22(paVar2,(param_2 + 0x5c)),NULL,0x20c);
  return param_2;
}
pub fn pass1_1010_92e6(param_1: *mut u16)

{
  *param_1 = 0x9566;
  (param_1 + 0x2) = 0x1010;
  pass1_1010_2db2((astruct_455 *)param_1);
  return;
}
pub fn pass1_1010_9304(mut param_1: u16 ,u8 *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  if (param_5 != 0x0) {
    mem_op_1000_179c(param_5 << 0x2,paVar1);
    return;
  }
  mem_op_1000_179c(0x1a,paVar1);
  if ((paVar1 | param_1) != 0x0) {
    pass1_1010_9258((astruct_223 *)CONCAT22(paVar1,param_1));
    return;
  }
  return;
}
pub fn pass1_1010_9348(mut param_1: u32,mut param_2: i16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  (param_2 * 0x8 + 0x319e) = param_2;
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x16) = param_2 * 0x8 + 0x3198;
  (iVar1 + 0x18) = &DAT_1050_1050;
  (iVar1 + 0x12) = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_9372(u32 *param_1,mut param_2: u16 ,mut param_3: i16,mut param_4: i16,mut param_5: i16)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  char cVar4;
  let mut in_EDX: *mut Struct57;
  let mut uVar5: u16;
  let mut unaff_SI: u16;
  let mut uVar6: u16;
  let mut bVar7: bool;
  let mut uVar8: u32;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;

  if (0x0 < param_4) {
    if (_PTR_LOOP_1050_3528 == NULL) {
      ppcVar1 = (code **)(*param_1 + 0x18);
      uVar3 = (**ppcVar1)();
      _PTR_LOOP_1050_3528 =
           mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,uVar3),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
    }
    uVar2 = (param_1 + 0xc);
    uVar8 = pass1_1010_2e02(_PTR_LOOP_1050_3528,(uVar2 + 0x12));
    uVar5 = param_2 + 0x1;
    uVar6 = param_3 + (0xfffe < param_2);
    for (cVar4 = ((char)param_4 + -0x1) * '\x04'; cVar4 != '\0'; cVar4 += -0x1) {
      bVar7 = CARRY2(uVar5,uVar5);
      uVar5 *= 0x2;
      uVar6 = uVar6 * 0x2 + bVar7;
    }
    pass1_1010_2e30(_PTR_LOOP_1050_3528,uVar5 | uVar8,uVar6 | (uVar8 >> 0x10),param_5);
  }
  return;
}
pub fn pass1_1010_93f0(mut param_1: u32)

{
  let mut puVar1: *mut u8,
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  astruct_223 *paVar5;
  u8 local_1c [0x1a];

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (*(i32 *)(iVar3 + 0x56) == 0x0) {
    paVar5 = pass1_1010_9258((astruct_223 *)CONCAT22(0x1050,local_1c));
    uVar2 = (paVar5 >> 0x10);
    puVar1 = local_1c;
    pass1_1010_398e(puVar1,CONCAT22(0x1050,puVar1),0x0,0x0,0x0);
    (iVar3 + 0x56) = puVar1;
    (iVar3 + 0x58) = uVar2;
    pass1_1010_927a(CONCAT22(0x1050,local_1c));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * load_string_1010_9432(void)

{
  let mut pcVar1: *mut c_char;
  let mut in_stack_00000004: u32;

  pcVar1 = load_string_1010_847e(_u16_1050_14cc,*(u16*)(in_stack_00000004 + 0x16));
  return pcVar1;
}
pub fn pass1_1010_944e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16)

{
  code **ppcVar1;
  let mut uVar2: u32;

  if (*(i32 *)(param_1 + 0x56) == 0x0) {
    ppcVar1 = (code **)(CONCAT22(param_2,param_1) + 0x10);
    (**ppcVar1)();
  }
  uVar2 = pass1_1010_2e02(CONCAT22(param_2,param_1),param_3);
  pass1_1010_2e5c(param_1,param_2,uVar2);
  return;
}



u16 FUN_1010_9482(void)

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool pass1_1010_9488(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut uVar6: u16;
  let mut in_stack_0000ffee: u32;

  uVar6 = (param_4 + 0x12);
  puVar5 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22((in_stack_0000ffee >> 0x10),0x3),in_stack_0000fe98,
                           in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
  uVar1 = (puVar5 >> 0x10);
  uVar2 = puVar5;
  uVar3 = uVar6 - 0x32;
  uVar4 = uVar1;
  if (uVar3 == 0x0) {
    pass1_1010_a5ca(0x0,uVar1,uVar2,uVar1,0x32);
    if (uVar3 != 0x0) {
      return false;
    }
    uVar6 = 0x4d;
  }
  else {
    uVar3 = uVar6 - 0x3f;
    if (uVar3 == 0x0) {
      pass1_1010_a5ca(0x0,uVar1,uVar2,uVar1,0x3f);
      if (uVar3 != 0x0) {
        return false;
      }
      uVar6 = 0x4e;
    }
  }
  pass1_1010_a5ca(uVar3,uVar4,uVar2,uVar1,uVar6);
  return uVar3 == 0x0;
}



u16 pass1_1010_9502(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 0x16);
  return (uVar1 + 0x2);
}



u16 pass1_1010_9514(void)

{
  return 0x31;
}



StructD * pass1_1010_951a(StructD *param_1,param_2: u8)

{
  pass1_1010_927a(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1010_9540(param_1: *mut u16,param_2: u8)

{
  pass1_1010_92e6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1010_95aa(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0x0;
  (param_1 + 0xe) = 0x0;
  (param_1 + 0x12) = 0x0;
  (param_1 + 0x16) = 0x0;
  (param_1 + 0x18) = 0x0;
  (param_1 + 0x1a) = 0x0;
  (param_1 + 0x1c) = 0xa;
  (param_1 + 0x1e) = 0x0;
  param_1->offset_0x0 = 0xa1c8;
  (param_1 + 0x2) = 0x1010;
  return;
}
pub fn pass1_1010_95f8(param_1: *mut astruct_455)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  code **ppcVar4;
  astruct_455 *iVar4;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1->field0_0x0 = 0xa1c8;
  iVar4->field1_0x2 = 0x1010;
  puVar1 = iVar4[0x1].field1_0x2;
  puVar2 = iVar4[0x1].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  puVar1 = iVar4[0x1].field3_0x6;
  uVar3 = (iVar4 + 0x2)->field0_0x0;
  if ((uVar3 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  puVar1 = iVar4[0x2].field1_0x2;
  puVar2 = iVar4[0x2].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  pass1_1010_1d80((StructD *)param_1);
  return;
}
pub fn pass1_1010_9674(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0x12);
  uVar2 = (iVar4 + 0x14);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0x12) = 0x0;
  return;
}
pub fn pass1_1010_96a8(mut param_1: u32,mut param_2: i16)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  piVar1 = (param_1 + 0x1e);
  *piVar1 = *piVar1 - param_2;
  if (*piVar1 < 0x0) {
    (param_1 + 0x1e) = 0x0;
  }
  return;
}



u16 pass1_1010_96c2(mut param_1: u32)

{
  return (param_1 + 0x1e);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 pass1_1010_96d0(param_1: *mut astruct_690)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  astruct_690 *iVar3;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut iStack8: i16;

  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_690 *)param_1;
  if (iVar3->field26_0x1a != 0x0) {
    if (0x0 < iVar3->field27_0x1c) {
      piVar1 = &iVar3->field27_0x1c;
      *piVar1 = *piVar1 + -0x1;
    }
    if ((iVar3->field27_0x1c == 0x0) && (iVar3->field28_0x1e != 0x0)) {
      iStack8 = 0x1;
      uVar4 = pass1_1030_8326();
      iVar2 = (uVar4 >> 0x10);
      if ((iVar2 != 0x0) || (0x32 < uVar4)) {
        iStack8 = 0x5;
      }
      if ((iVar2 != 0x0) || (0x3c < uVar4)) {
        iStack8 = 0xa;
      }
      if (iVar3->field28_0x1e < iStack8) {
        iStack8 = iVar3->field28_0x1e;
      }
      piVar1 = &iVar3->field28_0x1e;
      *piVar1 = *piVar1 - iStack8;
      if (*piVar1 < 0x0) {
        iVar3->field28_0x1e = 0x0;
      }
      if (0x0 < iVar3->field28_0x1e) {
        return iStack8;
      }
      return -0x1;
    }
  }
  return 0x0;
}
pub fn pass1_1010_9766(mut param_1: u16 ,mut param_2: u32)

{
  let mut in_AX: i16;
  let mut uVar1: u16;

  uVar1 = (param_2 >> 0x10);
  (param_2 + 0x1a) = 0x1;
  pass1_1010_a0a0(param_1,(astruct_252 *)param_2);
  pass1_1010_9f8c(param_2,0x80);
  (param_2 + 0x1e) = in_AX * 0x32;
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_9794(param_1: *mut astruct_250)

{
  let mut iVar1: i16;
  code **ppcVar2;
  let mut pcVar3: *mut c_char;
  let mut uVar4: u16;
  let mut pchar_5: *mut c_char;
  let mut puVar5: *mut u32;
  let mut uVar6: u16;
  let mut in_EDX: u32;
  let mut uVar8: u32;
  astruct_250 *iVar9;
  let mut uVar9: u16;
  let mut pcVar10: *mut c_char;
  char local_a [0x8];
  let mut paVar7: *mut Struct57;

  uVar9 = (param_1 >> 0x10);
  iVar9 = (astruct_250 *)param_1;
  if (iVar9->field18_0x18 == 0x0) {
    iVar9->field18_0x18 = 0x1;
    puVar5 = iVar9->field11_0xe;
    uVar4 = (&iVar9->field11_0xe + 0x2);
    uVar6 = uVar4 | puVar5;
    paVar7 = (astruct_57 *)(in_EDX & 0xffff0000 | uVar6);
    if (uVar6 != 0x0) {
      ppcVar2 = (code **)puVar5;
      (**ppcVar2)();
    }
    mem_op_1000_179c(0xc,paVar7);
    uVar4 = puVar5;
    uVar6 = paVar7 | uVar4;
    uVar8 = uVar6;
    if (uVar6 == 0x0) {
      uVar4 = 0x0;
      uVar8 = 0x0;
    }
    else {
      set_struct_1008_574a((astruct_57 *)(puVar5 & 0xffff | (long)paVar7 << 0x10));
    }
    &iVar9->field11_0xe = uVar4;
    (&iVar9->field11_0xe + 0x2) = uVar8;
    pass1_1008_5784(CONCAT22(0x1050,local_a),iVar9->field10_0xa);
    while( true ) {
      uVar4 = uVar8;
      pchar_5 = local_a;
      pass1_1008_5b12(CONCAT22(0x1050,pchar_5));
      uVar8 = (uVar4 | pchar_5);
      if ((uVar4 | pchar_5) == 0x0) break;
      iVar1 = (pchar_5 + 0x4);
      if ((iVar1 == 0x3e) || (iVar1 == 0x41)) {
        pcVar10 = iVar9->field10_0xa;
        (pcVar10 + 0xa) = 0x0;
        pcVar10 = iVar9->field10_0xa;
        ppcVar2 = (code **)(iVar9->field10_0xa + 0xc);
        (**ppcVar2)();
        pcVar3 = iVar9->field10_0xa;
        (pcVar3 + 0xa) = 0x1;
        local_a._4_4_ = 0x0;
        ppcVar2 = (code **)(*iVar9->field11_0xe + 0x4);
        (**ppcVar2)(0x1008,iVar9->field11_0xe,CONCAT22(uVar4,pchar_5),pcVar10);
      }
    }
  }
  return;
}
pub fn pass1_1010_988c(mut param_1: u32,mut param_2: i16)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  i32 lVar8;
  u8 local_a [0x8];

  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  pass1_1008_5784(CONCAT22(0x1050,local_a),(iVar6 + 0xe));
  do {
    lVar8 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    uVar5 = (lVar8 >> 0x10);
    iVar3 = lVar8;
    if (lVar8 == 0x0) {
      return;
    }
  } while ((iVar3 + 0x4) != param_2);
  iVar4 = (iVar3 + 0x6) + -0x1;
  (iVar3 + 0x6) = iVar4;
  if ((iVar4 < 0x1) &&
     (ppcVar1 = (code **)((iVar6 + 0xe) + 0xc),
     (**ppcVar1)(0x1008,(iVar6 + 0xe),lVar8), uVar2 = (iVar6 + 0xe),
     (uVar2 + 0x8) == 0x0)) {
    (iVar6 + 0x16) = 0x1;
  }
  return;
}



u16 FUN_1010_9900(mut param_1: u16 ,mut param_2: u32,u8 *param_3)

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  i32 lVar6;
  HFILE16 in_stack_0000ffc0;
  let mut uStack36: u16;
  u16 local_1c [0x3];
  u16 local_16 [0x3];
  let mut local_10: u32;
  u8 local_c [0x8];
  let mut local_4: u16;

  BVar2 = write_to_file_1008_7cac(param_3);
  if (BVar2 == 0x0) {
    return 0x0;
  }
  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  if (*(i32 *)(iVar4 + 0xa) == 0x0) {
    local_4 = 0x0;
  }
  else {
    uVar1 = (iVar4 + 0xa);
    local_4 = (uVar1 + 0x8);
  }
  local_1c[0] = local_4;
  BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_1c),0x2,in_stack_0000ffc0);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  pass1_1008_5784(CONCAT22(0x1050,local_c),(iVar4 + 0xa));
  do {
    local_10 = pass1_1008_5b12(CONCAT22(0x1050,local_c));
    if (local_10 == 0x0) {
      if (*(i32 *)(iVar4 + 0xe) == 0x0) {
        uStack36 = 0x0;
      }
      else {
        uVar1 = (iVar4 + 0xe);
        uStack36 = (uVar1 + 0x8);
      }
      local_16[0] = uStack36;
      BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_16),0x2,in_stack_0000ffc0);
      if (BVar2 == 0x0) {
        u16_1050_0310 = 0x6d0;
        return 0x0;
      }
      pass1_1008_5784(CONCAT22(0x1050,local_c),(iVar4 + 0xe));
      do {
        lVar6 = pass1_1008_5b12(CONCAT22(0x1050,local_c));
        uVar3 = (lVar6 >> 0x10);
        if (lVar6 == 0x0) {
          if (*(i32 *)(iVar4 + 0x12) == 0x0) {
            uStack36 = 0x0;
          }
          else {
            uVar1 = (iVar4 + 0x12);
            uStack36 = (uVar1 + 0x8);
          }
          local_16[0] = uStack36;
          BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_16),0x2,in_stack_0000ffc0);
          if (BVar2 == 0x0) {
            u16_1050_0310 = 0x6d0;
            return 0x0;
          }
          pass1_1008_5784(CONCAT22(0x1050,local_c),(iVar4 + 0x12));
          do {
            lVar6 = pass1_1008_5b12(CONCAT22(0x1050,local_c));
            uVar3 = (lVar6 >> 0x10);
            if (lVar6 == 0x0) {
              local_1c[0] = (iVar4 + 0x1a);
              BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_1c),0x2,in_stack_0000ffc0);
              if (BVar2 == 0x0) {
                u16_1050_0310 = 0x6d0;
                return 0x0;
              }
              local_1c[0] = (iVar4 + 0x1c);
              BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_1c),0x2,in_stack_0000ffc0);
              if (BVar2 == 0x0) {
                u16_1050_0310 = 0x6d0;
                return 0x0;
              }
              local_1c[0] = (iVar4 + 0x1e);
              BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_1c),0x2,in_stack_0000ffc0);
              if (BVar2 == 0x0) {
                u16_1050_0310 = 0x6d0;
                return 0x0;
              }
              return 0x1;
            }
            local_10 = local_10 & 0xffff0000 | (lVar6 + 0x4);
            BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_10),0x2,in_stack_0000ffc0);
            if (BVar2 == 0x0) {
              u16_1050_0310 = 0x6d0;
              return 0x0;
            }
            local_4 = (lVar6 + 0x6);
            BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_4),0x2,in_stack_0000ffc0);
          } while (BVar2 != 0x0);
          u16_1050_0310 = 0x6d0;
          return 0x0;
        }
        local_10 = local_10 & 0xffff0000 | (lVar6 + 0x4);
        BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_10),0x2,in_stack_0000ffc0);
        if (BVar2 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return 0x0;
        }
        local_4 = (lVar6 + 0x6);
        BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_4),0x2,in_stack_0000ffc0);
      } while (BVar2 != 0x0);
      u16_1050_0310 = 0x6d0;
      return 0x0;
    }
    local_16[0] = (local_10 + 0x4);
    BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_16),0x2,in_stack_0000ffc0);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return 0x0;
    }
    local_16[0] = (local_10 + 0x6);
    BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_16),0x2,in_stack_0000ffc0);
  } while (BVar2 != 0x0);
  u16_1050_0310 = 0x6d0;
  return 0x0;
}
pub fn FUN_1010_9b72(mut param_1: u16 ,mut param_2: u32,HFILE16 *param_3,mut param_4: i16,mut param_5: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut iVar7: i16;
  let mut puVar8: *mut u32;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  u16 local_36 [0x4];
  let mut puStack46: *mut u16;
  u16 local_2a [0x2];
  let mut puStack38: *mut u16;
  u16 auStack34 [0x2];
  let mut puStack30: *mut u16;
  i16 local_1a [0x2];
  let mut puStack22: *mut u16;
  let mut uStack18: u16;
  let mut puStack14: *mut u16;
  i16 local_a [0x3];
  let mut local_4: u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_5);
  uVar10 = param_3;
  uVar11 = (param_3 >> 0x10);
  read_file_1008_7cfe(uVar10,uVar11,0x1);
  if (param_4 != 0x0) {
    uVar2 = read_file_1008_7dee(param_3,CONCAT22(0x1050,&local_4),0x2);
    if (uVar2 != 0x0) {
      iVar7 = param_2;
      uVar9 = (param_2 >> 0x10);
      if (local_4 != 0x0) {
        if (*(i32 *)(iVar7 + 0xa) == 0x0) {
          mem_op_1000_179c(0xc,paVar6);
          uVar5 = paVar6;
          puStack22 = CONCAT22(uVar5,uVar2);
          paVar6 = (astruct_57 *)(paVar6 & 0xffff0000 | (uVar5 | uVar2));
          if ((uVar5 | uVar2) == 0x0) {
            (iVar7 + 0xa) = 0x0;
          }
          else {
            set_struct_1008_574a((astruct_57 *)CONCAT22(uVar5,uVar2));
            (iVar7 + 0xa) = uVar2;
            (iVar7 + 0xc) = paVar6;
          }
        }
        for (uStack18 = 0x0; uStack18 < local_4; uStack18 += 0x1) {
          uVar2 = local_4;
          mem_op_1000_179c(0x8,paVar6);
          uVar5 = paVar6;
          puStack22 = CONCAT22(uVar5,uVar2);
          paVar6 = (astruct_57 *)(paVar6 & 0xffff0000 | (uVar5 | uVar2));
          if ((uVar5 | uVar2) == 0x0) {
            puStack14 = NULL;
          }
          else {
            *puStack22 = 0x389a;
            (uVar2 + 0x2) = 0x1008;
            *puStack22 = 0xa1c4;
            (uVar2 + 0x2) = 0x1010;
            puStack14 = puStack22;
          }
          BVar3 = read_file_1008_7dee(param_3,CONCAT22(0x1050,local_a),0x2);
          if (BVar3 == 0x0) {//
LAB_1010_9c32:
            puStack22 = puStack14;
            if (puStack14 == NULL) goto LAB_1010_9ba1;
            uVar9 = (puStack14 >> 0x10);
            puVar8 = puStack14;
            goto LAB_1010_9e9e;
          }
          BVar3 = read_file_1008_7dee(param_3,(puStack14 & 0xffff0000 | (puStack14 + 0x6)),0x2)
          ;
          if (BVar3 == 0x0) goto LAB_1010_9c32;
          iVar4 = switch_1008_73ea(uVar10,uVar11,local_a[0]);
          (puStack14 + 0x4) = iVar4;
          ppcVar1 = (code **)((iVar7 + 0xa) + 0x4);
          (**ppcVar1)();
        }
      }
      uVar2 = read_file_1008_7dee(param_3,CONCAT22(0x1050,local_2a),0x2);
      if (uVar2 != 0x0) {
        if (local_2a[0] != 0x0) {
          if (*(i32 *)(iVar7 + 0xe) == 0x0) {
            mem_op_1000_179c(0xc,paVar6);
            uVar5 = paVar6;
            puStack46 = CONCAT22(uVar5,uVar2);
            paVar6 = (astruct_57 *)(paVar6 & 0xffff0000 | (uVar5 | uVar2));
            if ((uVar5 | uVar2) == 0x0) {
              (iVar7 + 0xe) = 0x0;
            }
            else {
              set_struct_1008_574a((astruct_57 *)CONCAT22(uVar5,uVar2));
              (iVar7 + 0xe) = uVar2;
              (iVar7 + 0x10) = paVar6;
            }
          }
          for (auStack34[0] = 0x0; auStack34[0] < local_2a[0]; auStack34[0] += 0x1) {
            uVar2 = local_2a[0];
            mem_op_1000_179c(0x8,paVar6);
            uVar5 = paVar6;
            puStack22 = CONCAT22(uVar5,uVar2);
            paVar6 = (astruct_57 *)(paVar6 & 0xffff0000 | (uVar5 | uVar2));
            if ((uVar5 | uVar2) == 0x0) {
              puStack30 = NULL;
            }
            else {
              *puStack22 = 0x389a;
              (uVar2 + 0x2) = 0x1008;
              *puStack22 = 0xa1c4;
              (uVar2 + 0x2) = 0x1010;
              puStack30 = puStack22;
            }
            BVar3 = read_file_1008_7dee(param_3,CONCAT22(0x1050,local_1a),0x2);
            if (BVar3 == 0x0) {//
LAB_1010_9d5c:
              puStack22 = puStack30;
              if (puStack30 == NULL) goto LAB_1010_9ba1;
              uVar9 = (puStack30 >> 0x10);
              puVar8 = puStack30;
              goto LAB_1010_9e9e;
            }
            BVar3 = read_file_1008_7dee(param_3,(puStack30 & 0xffff0000 | (puStack30 + 0x6)),
                                        0x2);
            if (BVar3 == 0x0) goto LAB_1010_9d5c;
            iVar4 = switch_1008_73ea(uVar10,uVar11,local_1a[0]);
            (puStack30 + 0x4) = iVar4;
            ppcVar1 = (code **)((iVar7 + 0xe) + 0x4);
            (**ppcVar1)();
          }
        }
        uVar2 = read_file_1008_7dee(param_3,CONCAT22(0x1050,local_36),0x2);
        if (uVar2 != 0x0) {
          if (local_36[0] != 0x0) {
            if (*(i32 *)(iVar7 + 0x12) == 0x0) {
              mem_op_1000_179c(0xc,paVar6);
              uVar5 = paVar6;
              puStack22 = CONCAT22(uVar5,uVar2);
              paVar6 = (astruct_57 *)(paVar6 & 0xffff0000 | (uVar5 | uVar2));
              if ((uVar5 | uVar2) == 0x0) {
                (iVar7 + 0x12) = 0x0;
              }
              else {
                set_struct_1008_574a((astruct_57 *)CONCAT22(uVar5,uVar2));
                (iVar7 + 0x12) = uVar2;
                (iVar7 + 0x14) = paVar6;
              }
            }
            for (local_2a[0] = 0x0; local_2a[0] < local_36[0]; local_2a[0] += 0x1) {
              uVar2 = local_36[0];
              mem_op_1000_179c(0x8,paVar6);
              uVar5 = paVar6;
              puStack46 = CONCAT22(uVar5,uVar2);
              paVar6 = (astruct_57 *)(paVar6 & 0xffff0000 | (uVar5 | uVar2));
              if ((uVar5 | uVar2) == 0x0) {
                puStack38 = NULL;
              }
              else {
                *puStack46 = 0x389a;
                (uVar2 + 0x2) = 0x1008;
                *puStack46 = 0xa1c4;
                (uVar2 + 0x2) = 0x1010;
                puStack38 = puStack46;
              }
              BVar3 = read_file_1008_7dee(param_3,CONCAT22(0x1050,auStack34),0x2);
              if (BVar3 == 0x0) {//
LAB_1010_9e86:
                puStack22 = puStack38;
                if (puStack38 != NULL) {
                  uVar9 = (puStack38 >> 0x10);
                  puVar8 = puStack38;//
LAB_1010_9e9e:
                  ppcVar1 = (code **)*puVar8;
                  puStack46 = puStack22;
                  (**ppcVar1)(0x1008,puStack22,uVar9,0x1);
                }
                goto LAB_1010_9ba1;
              }
              BVar3 = read_file_1008_7dee(param_3,(puStack38 & 0xffff0000 | (puStack38 + 0x6)),
                                          0x2);
              if (BVar3 == 0x0) goto LAB_1010_9e86;
              iVar4 = switch_1008_73ea(uVar10,uVar11,auStack34[0]);
              (puStack38 + 0x4) = iVar4;
              ppcVar1 = (code **)((iVar7 + 0x12) + 0x4);
              (**ppcVar1)();
            }
          }
          BVar3 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | (iVar7 + 0x1a)),0x2);
          if (BVar3 != 0x0) {
            BVar3 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | (iVar7 + 0x1c)),0x2);
            if (BVar3 != 0x0) {
              BVar3 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | (iVar7 + 0x1e)),0x2);
              if (BVar3 != 0x0) {
                return;
              }
            }
          }
        }
      }
    }//
LAB_1010_9ba1:
    u16_1050_0310 = 0x6d2;
  }
  return;
}
pub fn pass1_1010_9f72(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  pass1_1010_9fa6(param_1,uVar1,(param_1 + 0xe),param_2);
  return;
}
pub fn pass1_1010_9f8c(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  pass1_1010_9fa6(param_1,uVar1,(param_1 + 0xa),param_2);
  return;
}



u16 pass1_1010_9fa6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: i16)

{
  let mut uVar1: u16;
  i32 lVar2;
  u8 local_a [0x8];

  if (param_3 != 0x0) {
    pass1_1008_5784(CONCAT22(0x1050,local_a),param_3);
    while( true ) {
      lVar2 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
      uVar1 = (lVar2 >> 0x10);
      if (lVar2 == 0x0) break;
      if ((lVar2 + 0x4) == param_4) {
        return (lVar2 + 0x6);
      }
    }
  }
  return 0x0;
}
pub fn pass1_1010_9fee(StructD *param_1,param_2: *mut astruct_252,mut param_3: u16 ,mut param_4: u16 )

{
  StructD *iVar5;
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut paVar3: *mut Struct57;
  astruct_252 *iVar3;
  astruct_253 *iVar4;
  let mut uVar4: u16;
  let mut uVar5: u16;
  StructD *puStack10;
  StructD *pSStack6;
  code **fn_ptr_1;

  iVar5 = (StructD *)(param_1 >> 0x10);
  paVar3 = (astruct_57 *)(param_1 & 0xffff0000 | param_1 & 0xffff);
  uVar4 = (param_2 >> 0x10);
  iVar3 = (astruct_252 *)param_2;
  if (iVar3->field18_0x12 == NULL) {
    mem_op_1000_179c(0xc,paVar3);
    uVar1 = paVar3;
    uVar2 = uVar1 | iVar5;
    paVar3 = (astruct_57 *)(paVar3 & 0xffff0000 | uVar2);
    if (uVar2 == 0x0) {
      iVar3->field18_0x12 = NULL;
    }
    else {
      set_struct_1008_574a((astruct_57 *)CONCAT22(uVar1,iVar5));
      *(StructD **)&iVar3->field18_0x12 = iVar5;
      (&iVar3->field18_0x12 + 0x2) = paVar3;
    }
  }
  mem_op_1000_179c(0x8,paVar3);
  uVar1 = paVar3;
  puStack10 = (StructD *)CONCAT22(uVar1,iVar5);
  if ((uVar1 | iVar5) == 0x0) {
    pSStack6 = NULL;
  }
  else {
    puStack10->address_offset_field_0x0 = 0x389a;
    iVar5->address_offset_field_0x2 = 0x1008;
    puStack10->address_offset_field_0x0 = 0xa1c4;
    iVar5->address_offset_field_0x2 = 0x1010;
    pSStack6 = puStack10;
  }
  uVar5 = (pSStack6 >> 0x10);
  iVar4 = (astruct_253 *)pSStack6;
  iVar4->field4_0x4 = param_4;
  iVar4->field5_0x6 = param_3;
  fn_ptr_1 = (code **)(*iVar3->field18_0x12 + 0x4);
  (**fn_ptr_1)(0x1000,iVar3->field18_0x12,iVar4,uVar5);
  return;
}
