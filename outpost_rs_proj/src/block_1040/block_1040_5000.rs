
u16 pass1_1040_5238(mut param_1: u32)

{
  code **ppcVar1;

  ppcVar1 = (code **)((param_1 + 0x94) + 0x8);
  (**ppcVar1)();
  return 0x0;
}
pub fn destroy_win_1040_5256(param_1: *mut astruct_34)

{
  let mut pUVar1: *mut u32;
  let mut bool3: bool;
  astruct_34 *pstruct34_5;
  astruct_34 *pstruct34_hi;
  let mut unaff_CS: u16;
  let mut uVar2: u16;
  code **fn_ptr_1;

  pstruct34_hi = (astruct_34 *)(param_1 >> 0x10);
  pstruct34_5 = (astruct_34 *)param_1;
  if (pstruct34_5->hwnd_0xb6 != 0x0) {
    // 0x1538
    unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
    bool3 = IsWindow16(pstruct34_5->hwnd_0xb6);
    if (bool3 != 0x0) {
    // 0x1538
      unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
      DestroyWindow16(pstruct34_5->hwnd_0xb6);
    }
  }
  pstruct34_5->hwnd_0xb6 = 0x0;
  pUVar1 = pstruct34_5->field148_0x94;
  uVar2 = pstruct34_5->field149_0x96;
  if ((uVar2 | pUVar1) != 0x0) {
    fn_ptr_1 = (code **)*pUVar1;
    (**fn_ptr_1)(unaff_CS,pUVar1,uVar2,0x1);
  }
  &pstruct34_5->field148_0x94 = 0x0;
  pstruct34_5->field150_0x98 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_52c0(u8 *param_1,param_2: *mut astruct_894,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  code **ppcVar1;
  let mut puVar2: *mut u32;
  let mut is_window: bool;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut uVar9: u32;
  let mut puVar10: *mut u32;
  astruct_940 *paVar11;
  let mut in_stack_0000fe84: u16;
  let mut in_stack_0000fe86: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb4: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut in_stack_0000ffde: u16;
  let mut uVar15: u16;
  let mut uStack30: u16;
  let mut local_a: RECT16;
  let mut iStack6: i16;
  let mut iStack4: i16;
  let mut hwnd_8: HWND16;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_5 != 0x10c) {
    if (param_5 < 0x10d) {
      if (param_5 == 0xfa) {
        ppcVar1 = (code **)(*param_2->field148_0x98 + 0x18);
        (**ppcVar1)();
        return;
      }
      if (param_5 == 0x10a) {
        GetClientRect16(&local_a,(HWND16)&DAT_1050_1050);
        puVar2 = param_2->field148_0x98;
        local_a.y += 0x3;
        local_a.x = (puVar2 + 0x1a) + -0x9;
        iStack6 += -0x3;
        iStack4 += -0x3;
        InvalidateRect16(0x1,&local_a,(HWND16)&DAT_1050_1050);
        unk_destroy_win_op_1010_2fa0((astruct_873 *)param_2->field148_0x98);
        pass1_1010_32c0(param_2->field148_0x98,0x0);
        pass1_1010_2ee2(param_2->field148_0x98);
        return;
      }
      if (param_5 != 0x10b) {//
LAB_1040_5560:
        pass1_1040_b54a(param_1,(astruct_903 *)CONCAT22(param_3,param_2),param_4,param_5);
        return;
      }
      puVar2 = param_2->field148_0x98;
      uVar12 = (puVar2 + 0x12);
      uVar5 = uVar12;
      puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(uVar12,0x3),in_stack_0000fe84,
                                in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
      uVar7 = (puVar10 >> 0x10);
      uStack30 = puVar10;
      uVar4 = uStack30;
      uVar6 = uVar7;
      pass1_1010_a5ca(uStack30,uVar7,uStack30,uVar7,uVar12);
      if ((uVar5 != 0x70) && (uVar4 == 0x0)) {
        return;
      }
      uVar9 = param_2->field169_0xb0;
      uVar13 = uVar9;
      uVar14 = (uVar9 >> 0x10);
      puVar2 = param_2->field148_0x98;
      uVar12 = (puVar2 + 0x12);
    }
    else {
      if (param_5 != 0x10d) {
        if (param_5 == 0x10e) {
          puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffde,0x32),
                                    in_stack_0000fe86,in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
          uVar9 = paVar8 & 0xffff0000 | puVar10 >> 0x10;
          uVar3 = puVar10;
          uVar15 = uVar3;
          ui_op_1010_79aa(puVar10,0xfc6,param_2->field169_0xb0);
          if (uVar3 != 0x0) {
            return;
          }
          unk_win_op_1010_7300
                    (uVar9,(astruct_57 *)(puVar10 & 0xffff0000 | uVar15),0x0,0x13,param_2->field169_0xb0);
          return;
        }
        if (param_5 != 0xbbb) {
          if (param_5 == 0xbbc) {
            puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffde,0x3),
                                      in_stack_0000fe86,in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
            uVar12 = (puVar10 >> 0x10);
            uVar4 = puVar10;
            uVar7 = uVar12;
            uVar5 = pass1_1010_a5ac(uVar4,uVar12,param_2->field169_0xb0);
            uVar6 = uVar5;
            pass1_1010_a58a(uVar5,uVar7,uVar4,uVar12,uVar5);
            if (uVar6 == 0x0) {
              pass1_1010_a568(0x0,uVar7,uVar4,uVar12,uVar5);
            }
            hwnd_8 = GetDlgItem16(0xbbc,param_2->hwnd_0x6);
            EnableWindow16(0x0,hwnd_8);
            return;
          }
          goto LAB_1040_5560;
        }
        if ((param_2->field171_0xb6 == 0x0) || (is_window = IsWindow16(param_2->field171_0xb6), is_window == 0x0)) {
          paVar11 = (astruct_940 *)
                    pass1_1038_af40(param_2,paVar8,_PTR_LOOP_1050_5b7c,param_2->hwnd_0x6,0x1b);
          param_2->field171_0xb6 = *(HWND16 *)(paVar11 + 0x6);
          set_win_pos_1038_abdc(paVar11);
          ShowWindow16(0x1,param_2->field171_0xb6);
          return;
        }
        hwnd_8 = param_2->field171_0xb6;
        goto LAB_1040_5417;
      }
      puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffde,0x3),in_stack_0000fe86,
                                in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
      uVar6 = (puVar10 >> 0x10);
      uStack30 = puVar10;
      uVar9 = param_2->field169_0xb0;
      uVar13 = uVar9;
      uVar14 = (uVar9 >> 0x10);
      uVar12 = 0x71;
      uVar7 = uVar6;
    }
    pass1_1010_a5ec(uVar6,uStack30,uVar7,uVar12,CONCAT22(uVar14,uVar13));
    if ((param_2->hwnd_0xb4 != 0x0) && (is_window = IsWindow16(param_2->hwnd_0xb4), is_window != 0x0)) {
      SendMessage16(0x0,0xeb,0x111,param_2->hwnd_0xb4);
    }
  }
  hwnd_8 = param_2->hwnd_0x6;//
LAB_1040_5417:
  DestroyWindow16(hwnd_8);
  return;
}



StructD * pass1_1040_557c(StructD *param_1,param_2: u8)

{
  pass1_1040_4f0a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar1
pub fn pass1_1040_5626(param_1: *mut astruct_57,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 )

{
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut iVar9: *mut Struct57;
  let mut iVar6: i16;
  let mut uVar11: *mut Struct57;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut piStack12: *mut i16;
  let mut uVar1: u32;
  let mut paVar5: *mut Struct57;

  struct_1040_b082(param_2,CONCAT22(param_4,0xfa3));
  uVar11 = (astruct_57 *)(param_2 >> 0x10);
  iVar9 = (astruct_57 *)param_2;
  uVar3 = 0x0;
  iVar9[0x1].field3_0x6 = 0x0;
  iVar9[0x1].field4_0x8 = 0x0;
  iVar9[0x1].field5_0xa = 0x0;
  iVar9[0x1].field7_0xe = 0x0;
  param_2->field0_0x0 = 0x6386;
  iVar9->field1_0x2 = &PTR_LOOP_1050_1040;
  mem_op_1000_179c(0x18,param_1);
  uVar4 = param_1 | uVar3;
  paVar5 = (astruct_57 *)(param_1 & 0xffff0000 | uVar4);
  if (uVar4 == 0x0) {
    &iVar9[0x1].field1_0x2 = 0x0;
  }
  else {
    struct_1040_a598((astruct_259 *)CONCAT22(param_1,uVar3));
    iVar9[0x1].field1_0x2 = uVar3;
    iVar9[0x1].field2_0x4 = paVar5;
  }
  *(u16*)&iVar9[0x1].field1_0x2 = 0x6;
  iVar6 = *&iVar9[0x1].field1_0x2;
  uVar3 = iVar6 * 0xa + 0x2;
  mem_op_1000_179c(uVar3,paVar5);
  uVar4 = paVar5;
  piStack12 = CONCAT22(uVar4,uVar3);
  if ((uVar4 | uVar3) == 0x0) {
    uVar2 = &iVar9[0x1].field1_0x2;
    (uVar2 + 0x2) = 0x0;
  }
  else {
    *piStack12 = iVar6;
    pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iVar6,0xa,uVar3 + 0x2,uVar4);
    uVar2 = &iVar9[0x1].field1_0x2;
    uVar7 = (uVar2 >> 0x10);
    iVar6 = uVar2;
    (iVar6 + 0x2) = uVar3 + 0x2;
    (iVar6 + 0x4) = uVar4;
  }
  uVar1 = &iVar9[0x1].field1_0x2;
  (uVar1 + 0x6) = param_3;
  uVar2 = &iVar9[0x1].field1_0x2;
  (uVar2 + 0xa) = 0x4;
  uVar2 = &iVar9[0x1].field1_0x2;
  (uVar2 + 0x12) = iVar9->field5_0xa;
  uVar8 = pass1_1040_5d12(param_2);
  uVar3 = (uVar8 >> 0x10);
  if ((uVar3 | uVar8) == 0x0) {
    iVar9[0x1].field6_0xc = 0x0;
  }
  else {
    iVar9[0x1].field6_0xc = (uVar8 + 0x20);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn enable_win_1040_5780(u32 *param_1)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut hwnd: HWND16;
  let mut in_EDX: *mut Struct57;
  astruct_945 *iVar4;
  let mut uVar4: u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  astruct_945 *paVar6;

  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_945 *)param_1;
  ppcVar1 = (code **)(*param_1 + 0x74);
  paVar6 = iVar4;
  (**ppcVar1)();
  puVar5 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(paVar6,0x3),in_stack_0000fe98,in_stack_0000ffbc,
                           in_stack_0000ffc2,in_stack_0000ffc6);
  uVar2 = iVar4->field143_0x90;
  uVar3 = pass1_1010_acc0(puVar5,(puVar5 >> 0x10),(uVar2 + 0x6));
  if (uVar3 != 0x0) {
    hwnd = GetDlgItem16(0x1790,iVar4->field6_0x6);
    EnableWindow16(0x1,hwnd);
  }
  return;
}
pub fn pass1_1040_57d4(u8 *param_1,StructB *param_2)

{
  pass1_1040_5d42(param_2);
  pass1_1040_5eaa(param_2);
  pass1_1040_5dc4(param_1,param_2);
  unk_win_ui_op_1040_b230(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_5800(u8 *param_1,param_2: *mut astruct_18,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  astruct_18 *paVar5;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut hwnd: HWND16;
  let mut uVar6: u16;
  let mut puVar7: *mut u8,
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut paVar10: *mut Struct57;
  let mut iVar11: i16;
  let mut unaff_SI: u16;
  let mut uVar12: u16;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut piStack24: *mut i16;
  u8 local_14 [0x8];
  let mut iStack12: i16;
  StructD *pSStack10;
  astruct_20 *paStack6;
  StructD *pSVar5;
  let mut paVar9: *mut Struct57;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0xeb) {
    paStack6 = (astruct_20 *)
               mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fe80,
                               in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
    paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | paStack6 >> 0x10);
    pSVar5 = *(StructD **)&param_2->field138_0x90;
    if (pSVar5 != NULL) {
      pSStack10 = pSVar5;
    // 0x0018
      mem_op_1000_179c(0x18,paVar8);
      uVar3 = pSVar5;
      uVar6 = paVar8 | uVar3;
      paVar10 = (astruct_57 *)(paVar8 & 0xffff0000);
      paVar9 = (astruct_57 *)(paVar10 | uVar6);
      if (uVar6 == 0x0) {
        uVar3 = 0x0;
      }
      else {
        struct_1040_a598((astruct_259 *)(pSVar5 & 0xffff | (long)paVar8 << 0x10));
        paVar10 = paVar9;
      }
      param_2->field138_0x90 = uVar3;
      param_2->field139_0x92 = paVar10;
      *(u16*)&param_2->field138_0x90 = 0x6;
      iStack12 = *&param_2->field138_0x90;
      uVar3 = iStack12 * 0xa + 0x2;
      mem_op_1000_179c(uVar3,paVar10);
      uVar6 = paVar10;
      piStack24 = CONCAT22(uVar6,uVar3);
      puVar7 = (uVar6 | uVar3);
      if (puVar7 == NULL) {
        uVar2 = &param_2->field138_0x90;
        (uVar2 + 0x2) = 0x0;
      }
      else {
        *piStack24 = iStack12;
    // &PTR_LOOP_1050_1040 actually 0x1040
        pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iStack12,0xa,uVar3 + 0x2,uVar6);
        uVar2 = &param_2->field138_0x90;
        uVar12 = (uVar2 >> 0x10);
        iVar11 = uVar2;
        (iVar11 + 0x2) = uVar3 + 0x2;
        (iVar11 + 0x4) = uVar6;
      }
      uVar2 = &param_2->field138_0x90;
      (uVar2 + 0x6) = (pSStack10 + 0x6);
      uVar2 = &param_2->field138_0x90;
      (uVar2 + 0xa) = 0x4;
      uVar2 = &param_2->field138_0x90;
      (uVar2 + 0x12) = &param_2->field_0xa;
      uVar12 = 0x1010;
      pass1_1010_a50c(paStack6,(u8 **)&u32_1050_5d78,*(StructD **)&param_2->field138_0x90);
      if (pSStack10 != NULL) {
        pass1_1040_a5d0(pSStack10);
        uVar12 = 0x1000;
        fn_ptr_1000_17ce(pSStack10);
      }
      ppcVar1 = (code **)(CONCAT22(param_3,param_2) + 0x70);
      (**ppcVar1)(uVar12,param_2,param_3);
      uVar4 = pass1_1040_5cd6(CONCAT22(param_3,param_2));
      if (uVar4 != 0x0) {
        pass1_1040_5eaa((StructB *)CONCAT22(param_3,param_2));
        &param_2->field_0x94 = 0x0;
      }
      pass1_1040_5dc4(puVar7,(StructB *)CONCAT22(param_3,param_2));
      GetWindowRect16((RECT16 *)CONCAT13(0x10,CONCAT12(0x50,local_14)),param_2->hwnd_0x6);
      InvalidateRect16(param_2[0x1].base_0x0,NULL,0x0);
      if (param_2[0x1].base_0x0 != 0x0) {
        param_2[0x1].base_0x0 = 0x0;
      }
    }
  }
  else {
    if (param_5 != 0x13b) {
      pass1_1040_b54a(param_1,(astruct_903 *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,param_2)),param_4,
                      param_5);
      return;
    }
    hwnd = GetDlgItem16(0x1790,param_2->hwnd_0x6);
    EnableWindow16(0x1,hwnd);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn draw_op_1040_5a06(mut param_1: u32,astruct_741 *struct741_param_1)

{
  let mut uVar1: u16;
  let mut caption_height_px: i16;
  let mut IVar2: i16;
  HPEN16 handle;
  HGDIOBJ16 handle_00;
  let mut IVar3: i16;
  let mut y: i16;
  let mut IVar4: i16;
  let mut y_00: i16;
  let mut x: i16;
  let mut IVar5: i16;
  let mut in_DX: u16;
  let mut uVar6: u16;
  HPALETTE16 palette_handle_7;
  let mut puVar2: *mut u32;
  let mut uVar8: u32;
  astruct_741 *struct_1;
  let mut uVar9: u16;
  let mut base_addr: u16;
  let mut uVar11: u32;
  let mut uVar12: u32;
  u8 uVar10;
  let mut uVar14: u16;
  let mut uStack82: u16;
  let mut iStack72: i16;
  let mut iStack68: i16;
  let mut puStack54: *mut u32;
  HDC16 hdc16_2c;
  u8 paint_struct_2a [0x20];
  u8 rect_array_local_a [0x8];
  let mut uVar13: u16;
  let mut uVar15: u16;
  let mut uVar16: u16;
  let mut uVar4: u32;
  let mut puVar1: *mut u16;
  let mut uVar5: u16;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut uVar7: u32;
  code **fn_ptr_1;

  uVar9 = (struct741_param_1 >> 0x10);
  struct_1 = (astruct_741 *)struct741_param_1;
  GetWindowRect16((RECT16 *)CONCAT13(0x10,CONCAT12(0x50,rect_array_local_a)),struct_1->field6_0x6);
  hdc16_2c = BeginPaint16((PAINTSTRUCT16 *)CONCAT13(0x10,CONCAT12(0x50,paint_struct_2a)),struct_1->field6_0x6);
  base_addr = 0x1008;
  palette_handle_7 =
       palette_op_1008_4e08
                 ((HPALETTE16)&hdc16_2c,param_1,*(astruct_13 **)(_PTR_LOOP_1050_4230 + 0xe),
                  (HDC16 *)CONCAT13(0x10,CONCAT12(0x50,&hdc16_2c)));
  puVar2 = NULL;
  puStack54 = NULL;
  uVar7 = param_1;
  if (struct_1->field149_0x98 != 0x0) {
    uVar1 = FUN_1010_830a(0x0,param_1,0x1008,_u16_1050_14cc,struct_1->field149_0x98);
    uVar14 = param_1;
    puStack54 = CONCAT22(uVar14,uVar1);
    uVar7 = param_1;
    uVar11 = pass1_1008_4772((astruct_76 *)CONCAT22(uVar14,uVar1));
    uVar6 = (uVar11 >> 0x10) | (uVar11 & 0xffff);
    uVar7 = uVar7 & 0xffff0000 | uVar6;
    if (uVar6 == 0x0) {
      puVar2 = (uVar11 & 0xffff);
      if (puStack54 != NULL) {
        puVar2 = puStack54;
        if (puStack54 != NULL) {
          fn_ptr_1 = (code **)*puStack54;
          (**fn_ptr_1)(0x8,uVar1,(char)param_1,0x1,uVar14);
          puVar2 = puStack54;
        }
      }
      uVar1 = FUN_1010_830a(puVar2,uVar7,0x1008,_u16_1050_14cc,0x4d);
      puStack54 = CONCAT22(uVar7,uVar1);
    }
    uVar13 = &DAT_1050_1050;
    uVar10 = SUB21(&hdc16_2c,0x0);
    base_addr = s_tile2_bmp_1050_1538;
    caption_height_px = GetSystemMetrics16(SM_CYCAPTION);
    puVar2 = -(caption_height_px + -0x23);
    fn_ptr_1 = (code **)(*puStack54 + 0x4);
    (**fn_ptr_1)(0x38,(char)puStack54,(char)(puStack54 >> 0x10),-(caption_height_px + -0x23),uVar10,uVar13);
  }
  if (puStack54 != NULL) {
    uVar1 = (puStack54 >> 0x10);
    puVar2 = puStack54;
    if (puStack54 != NULL) {
      fn_ptr_1 = (code **)*puStack54;
      (**fn_ptr_1)((char)base_addr,(char)puStack54,uVar1,0x1,puStack54,uVar1);
      puVar2 = puStack54;
    }
  }
  uVar1 = FUN_1010_830a(puVar2,uVar7,base_addr,_u16_1050_14cc,struct_1->field148_0x96);
  puStack54 = CONCAT22(uVar7,uVar1);
  uVar14 = SUB42(&DAT_1050_1050,0x0);
  uVar10 = SUB21(&hdc16_2c,0x0);
  uVar8 = uVar7;
  IVar2 = GetSystemMetrics16(SM_CYCAPTION);
  uVar3 = *puStack54;
  fn_ptr_1 = (code **)uVar3 + 0x2;
  (**fn_ptr_1)(0x38,(char)uVar1,(char)uVar7,-(IVar2 + -0x23),uVar10,uVar14);
  if (puStack54 != NULL) {
    if (puStack54 != NULL) {
      fn_ptr_1 = (code **)uVar3;
      (**fn_ptr_1)(s_tile2_bmp_1050_1538,uVar1,(char)uVar7,0x1);
    }
  }
  handle = CreatePen16(0x1000025,0x0,0x0);
  handle_00 = SelectObject16(handle,hdc16_2c);
  uVar14 = FUN_1010_830a(handle_00,uVar8,s_tile2_bmp_1050_1538,_u16_1050_14cc,0x4f);
  puStack54 = CONCAT22(uVar8,uVar14);
  uVar12 = pass1_1008_4772((astruct_76 *)CONCAT13((char)(uVar8 >> 0x8),CONCAT12((char)uVar8,uVar14)));
  uVar1 = (uVar12 >> 0x10);
  uVar4 = (uVar12 + 0x4);
  uVar2 = (uVar12 + 0x8);
  IVar3 = GetSystemMetrics16(SM_CYCAPTION);
  y = -(IVar3 + -0xc1);
  IVar4 = GetSystemMetrics16(SM_CYCAPTION);
  iStack72 = (i16)uVar2;
  y_00 = 0xc5 - (IVar4 - iStack72);
  MoveTo16(y,0x82,hdc16_2c);
  iStack68 = (i16)uVar4;
  x = iStack68 * 0xa + 0x85;
  LineTo16(y,x,hdc16_2c);
  LineTo16(y_00,x,hdc16_2c);
  LineTo16(y_00,0x82,hdc16_2c);
  LineTo16(y,0x82,hdc16_2c);
  for (uStack82 = 0x0; puVar1 = &struct_1->field147_0x94, uStack82 <= *puVar1 && *puVar1 != uStack82; uStack82 += 0x1) {
    IVar5 = GetSystemMetrics16(SM_CYCAPTION);
    fn_ptr_1 = (code **)(*puStack54 + 0x4);
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,(char)uVar14,uVar8,-(IVar5 + -0xc4));
  }
  if (puStack54 != NULL) {
    if (puStack54 != NULL) {
      fn_ptr_1 = (code **)*puStack54;
      (**fn_ptr_1)(s_tile2_bmp_1050_1538,uVar14);
    }
  }
  SelectObject16(handle_00,hdc16_2c);
  DeleteObject16(handle);
  palette_handle_7 = SelectPalette16(0x0,palette_handle_7,hdc16_2c);
  DeleteObject16(palette_handle_7);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paint_struct_2a),struct_1->field6_0x6);
  return;
}



u16 pass1_1040_5cd6(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  uVar3 = pass1_1040_5d12(param_1);
  if (uVar3 != 0x0) {
    iVar1 = (uVar3 + 0x20);
    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x9a) != iVar1) {
      (param_1 + 0x9a) = iVar1;
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Unable to use type for symbol uVar3

pub fn pass1_1040_5d12(mut param_1: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar4: u16;
  astruct_440 *iVar4;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut uVar3: u32;

  uVar3 = (param_1 + 0x90);
  uVar5 = (uVar3 >> 0x10);
  iVar4 = (astruct_440 *)uVar3;
  uVar1 = iVar4->field6_0x6;
  uVar2 = iVar4->field7_0x8;
  uVar4 = uVar2 | uVar1;
  if (uVar4 != 0x0) {
    uVar6 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,uVar1),uVar1,uVar4);
    return uVar6;
  }
  return 0x0;
}
pub fn pass1_1040_5d42(StructB *param_1)

{
  let mut uVar1: u16;
  char cVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u32;

  uVar5 = pass1_1040_5d12(param_1);
  if (uVar5 != 0x0) {
    uVar1 = (uVar5 + 0xc);
    iVar3 = param_1;
    uVar4 = (param_1 >> 0x10);
    if (uVar1 == 0x5f) {
      (iVar3 + 0x96) = 0x53;
      return;
    }
    if (uVar1 < 0x60) {
      cVar2 = (char)uVar1;
      if (cVar2 == '(') {
        (iVar3 + 0x96) = 0x54;
        return;
      }
      if (cVar2 == ')') {
        (iVar3 + 0x96) = 0x55;
        return;
      }
      if (cVar2 == ']') {
        (iVar3 + 0x96) = 0x51;
        return;
      }
      if (cVar2 == '^') {
        (iVar3 + 0x96) = 0x52;
        return;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_5dc4(u8 *param_1,StructB *param_2)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
  StructB *iVar7;
  let mut unaff_SI: u16;
  let mut uVar8: u16;
  let mut puVar9: *mut u32;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut uVar10: u16;
  let mut iStack18: i16;

  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puVar9 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fe8c,
                           in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  paVar7 = (astruct_57 *)(paVar7 & 0xffff0000 | puVar9 >> 0x10);
  uVar3 = puVar9;
  uVar8 = (param_2 >> 0x10);
  iVar7 = (StructB *)param_2;
  uVar5 = (puVar9 >> 0x10);
  pass1_1010_a5ca(uVar3,uVar5,uVar3,uVar5,iVar7[0x7].field7_0xe);
  if (uVar3 == 0x0) {
    iVar7[0x7].max_count_field_0x10 = 0x0;
    &iVar7[0x7].field8_0x10 = 0x1;
  }
  if (-0x1 < uVar3) {
    if (iVar7[0x7].field7_0xe < 0x72) {
      uVar10 = 0x31;
    }
    else {
      uVar10 = 0x41;
    }
    puVar9 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,uVar10),in_stack_0000fe8c,
                             in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
    uVar6 = (puVar9 >> 0x10);
    uVar4 = iVar7[0x7].field7_0xe;
    ppcVar1 = (code **)(*puVar9 + 0x14);
    (**ppcVar1)(0x1010,puVar9,uVar6,uVar4,uVar4 >> 0xf);
    if ((uVar6 | uVar4) == 0x0) {
      iStack18 = 0x0;
    }
    else {
      uVar2 = (uVar4 + 0x16);
      iStack18 = (uVar2 + 0x4);
    }
    if ((iStack18 != 0x0) && (uVar3 != 0x0)) {
      uVar4 = ((iStack18 - uVar3) * 0x64) / iStack18;
      uVar6 = uVar4 / 0xa;
      iVar7[0x7].max_count_field_0x10 = uVar6;
      if (0x4 < uVar4 % 0xa) {
        iVar7[0x7].max_count_field_0x10 = uVar6 + 0x1;
      }
    }
  }
  return;
}



i16 pass1_1040_5eaa(StructB *param_1)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  switch((iVar1 + 0x9a)) {
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



StructD * pass1_1040_6360(StructD *param_1,param_2: u8)

{
  let mut in_stack_0000ffda: u16;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


