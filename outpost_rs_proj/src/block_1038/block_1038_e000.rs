//
// Created by cyrex on 2022-05-22.
//

// WARNING: Globals starting with '_' overlap smaller symbols at the same address








pub fn pass1_1038_e03e(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut iStack6: i16;

  uVar4 = (param_1 >> 0x10);
  uVar2 = pass1_1010_0886();
  for (iStack6 = 0x1; iStack6 <= (int)uVar2; iStack6 += 0x1) {
    uVar1 = (u32)((int)param_1 + 0x92);
    uVar6 = pass1_1010_08e2(uVar1,((u32)uVar1 >> 0x10),iStack6);
    uVar1 = (u32)((int)param_1 + 0x96);
    uVar5 = ((u32)uVar1 >> 0x10);
    iVar3 = (int)uVar1;
    if (*(i32 *)(iVar3 + iStack6 * 0x4) != 0x0) {
      enable_win_1040_9234((u32)(iVar3 + iStack6 * 0x4),*(BOOL16 *)((int)uVar6 + 0x6));
    }
  }
  return;
}



StructD * pass1_1038_e0ae(StructD *param_1,param_2: u8)

{
  pass1_1038_d7d0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_e140(param_1: *mut astruct_57,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfc2,param_5);
  param_1->field0_0x0 = 0xe264;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_e16e(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xe264;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn check_radio_btn_show_win_1038_e19a(StructB *param_1)

{
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  CheckRadioButton16(0x1807,0x1807,0x1807,*(HWND16 *)((int)param_1 + 0x6));
  move_win_1040_826c(param_1,0xc8,0xc8);
  ShowWindow16(0x5,*(HWND16 *)((int)param_1 + 0x6));
  return;
}
pub fn destroy_win_1038_e1dc(param_1: *mut astruct_886,mut param_2: u16 ,mut param_3: i16)

{
  let mut UVar1: u16;
  let mut uVar2: u32;

  if (param_3 != 0x0) {
    UVar1 = IsDlgButtonChecked(0x1807,param_1->field6_0x6);
    if (UVar1 == 0x0) {
      UVar1 = IsDlgButtonChecked(0x1806,param_1->field6_0x6);
      if (UVar1 == 0x0) goto LAB_1038_e229;
      uVar2 = 0x1110130;
    }
    else {
      uVar2 = 0x111012f;
    }
    SendMessage16(0x0,(WPARAM16)uVar2,((u32)uVar2 >> 0x10),HWND16_1050_0396);
  }//
LAB_1038_e229:
  DestroyWindow16(param_1->field6_0x6);
  return;
}
pub fn FUN_1038_e23a(void)

{
  return;
}



StructD * pass1_1038_e23e(StructD *param_1,param_2: u8)

{
  pass1_1038_e16e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_e2d0(param_1: *mut astruct_57,mut param_2: u16 )

{
  astruct_57 *uVar1;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c3,param_2);
  uVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  (u32)((int)param_1 + 0x8e) = 0x0;
  param_1->field0_0x0 = 0xe62e;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_e308(StructD *param_1)

{
  StructD *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xe62e;
  iVar1->address_offset_field_0x2 = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1->field_0x6);
  fn_ptr_1000_17ce(*(char **)&iVar1->field_0x8e);
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_e348(StructB *param_1,param_2: u8,StructD *param_3,mut param_4: u16 )

{
  LPVOID pvVar1;
  let mut uVar3: u32;
  let mut uVar5: u16;
  let mut uVar4: u16;
  astruct_57 *rect;
  let mut uVar7: u16;
  StructD *uVar6;
  let mut uVar11: u16;
  StructB *struct_b_5;
  let mut iVar12: i16;
  let mut unaff_SI: u16;
  let mut uVar8: u16;
  let mut uVar10: u16;
  let mut uVar9: u16;
  let mut in_stack_0000fe2a: u16;
  let mut in_stack_0000fe2e: u16;
  let mut in_stack_0000fe7e: u16;
  let mut in_stack_0000ff54: u16;
  let mut in_stack_0000ff58: u16;
  let mut in_stack_0000ff5c: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffac: u16;
  let mut local_22: u16;
  let mut uStack32: u16;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut puStack26: *mut u16;
  let mut iStack10: i16;
  let mut uStack8: u16;
  u32 *puStack6;
  let mut uVar2: u32;
  astruct_57 *paVar8;

  dialog_ui_fn_1040_78e2(param_1);
  uVar6 = param_3;
  puStack6 = mixed_1010_20ba((astruct_57 *)param_3,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2b),
                             in_stack_0000fe7e,in_stack_0000ffa2,in_stack_0000ffa8,in_stack_0000ffac);
  uVar6 = (StructD *)((u32)uVar6 & 0xffff0000 | (u32)puStack6 >> 0x10);
  uStack8 = pass1_1010_088c();
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(uVar6);
  }
  else {
    uVar6 = (StructD *)((u32)uVar6 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
  }
  puStack26 = (u16 *)CONCAT22(uVar6,PTR_LOOP_1050_5f2c);
  uVar4 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,uVar6);
  uVar8 = ((u32)param_1 >> 0x10);
  struct_b_5 = (StructB *)param_1;
  struct_b_5[0x7].field1_0x2 = uVar4;
  struct_b_5[0x7].hwnd_0x6 = (HWND16)uVar6;
  for (iStack10 = 0x1; uVar11 = ((u32)uVar6 >> 0x10), iStack10 <= (int)uStack8; iStack10 += 0x1) {
    puStack26 = (u16 *)pass1_1010_091e(puStack6,((u32)puStack6 >> 0x10),iStack10);
    uVar5 = ((u32)puStack26 >> 0x10);
    paVar8 = (astruct_57 *)CONCAT22(uVar11,uVar5);
    local_22 = *puStack26;
    uStack32 = ((int)puStack26 + 0x2);
    uStack30 = 0x1;
    uStack28 = 0x1;
    rect = (astruct_57 *)&local_22;
    MapDialogRect16((RECT16 *)rect,(HWND16)&DAT_1050_1050);
    mem_op_1000_179c(0x42,paVar8);
    uVar7 = (astruct_57 *)paVar8 | rect;
    uVar6 = (StructD *)((u32)paVar8 & 0xffff0000 | (u32)uVar7);
    if (uVar7 == 0x0) {
      uVar3 = (u32)&struct_b_5[0x7].field1_0x2;
      (u32)((int)uVar3 + iStack10 * 0x4) = 0x0;
    }
    else {
      pvVar1 = struct_b_5->lpvoid_field_0x8;
      pass1_1008_3bd6((u32)uVar6,rect,(astruct_57 *)paVar8,0x0,CONCAT22(local_22,uStack32),0x101,0xff0100,
                      CONCAT13((char)(pvVar1 >> 0x8),CONCAT12((char)pvVar1,((int)puStack26 + 0x4)))
                      ,param_4,in_stack_0000fe2a,in_stack_0000fe2e,in_stack_0000ff54,in_stack_0000ff58,in_stack_0000ff5c
                     );
      uVar2 = (u32)&struct_b_5[0x7].field1_0x2;
      uVar9 = ((u32)uVar2 >> 0x10);
      iVar12 = (int)uVar2;
      *(astruct_57 **)(iVar12 + iStack10 * 0x4) = rect;
      (iVar12 + iStack10 * 0x4 + 0x2) = (int)uVar6;
    }
    uVar3 = (u32)&struct_b_5[0x7].field1_0x2;
    uVar10 = ((u32)uVar3 >> 0x10);
    iVar12 = (int)uVar3;
    if (*(i32 *)(iVar12 + iStack10 * 0x4) != 0x0) {
      enable_win_1040_9234((u32)(iVar12 + iStack10 * 0x4),*(BOOL16 *)((int)puStack26 + 0x6));
    }
  }
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(0x5,(HWND16)struct_b_5->lpvoid_field_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_e4bc(u8 *param_1,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  let mut uVar7: u32;
  let mut uVar8: u32;
  let mut unaff_SI: u16;
  u32 *puVar9;
  u32 *puVar10;
  astruct_57 *paVar11;
  let mut in_stack_0000fe66: u16;
  let mut in_stack_0000fe68: u16;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000ff8a: u16;
  let mut in_stack_0000ff8c: u16;
  let mut in_stack_0000ff90: u16;
  let mut in_stack_0000ff92: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut uVar12: u16;
  u8 uVar13;
  u8 uVar14;
  let mut uVar15: u16;
  let mut uVar16: u16;
  let mut uVar17: u16;
  u32 *puStack22;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_4 == 0x1c4) {
    puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fe72,
                             in_stack_0000ff96,in_stack_0000ff9c,in_stack_0000ffa0);
    uVar15 = ((u32)puVar9 >> 0x10);
    uVar4 = ((int)puVar9 + 0x24);
    uVar5 = ((int)puVar9 + 0x26);
    uVar7 = (u32)paVar6 & 0xffff0000 | (u32)uVar5;
    uVar3 = uVar5 | uVar4;
    if (uVar3 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar5,uVar4));
      uVar8 = uVar7 & 0xffff0000;
      if ((uVar7 | uVar3) != 0x0) {
        puVar10 = pass1_1008_c6fa(_u16_1050_06e0,0x20);
        paVar11 = (astruct_57 *)(uVar8 & 0xffff0000 | (u32)puVar10 >> 0x10);
        uVar4 = puVar10;
        pass1_1038_4e78(uVar4,paVar11,CONCAT22(uVar7,uVar3),puVar10);
        uVar15 = SUB42(paVar11,0x0);
        puStack22 = (u32 *)CONCAT22(uVar15,uVar4);
        uVar2 = *puStack22;
        ppcVar1 = (code **)uVar2 + 0x8;
        paVar6 = paVar11;
        uVar5 = uVar4;
        (**ppcVar1)(0x1008,uVar4,uVar15);
        uVar3 = paVar6 | uVar5;
        paVar6 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)uVar3);
        if (uVar3 == 0x0) {
          if (puStack22 != NULL) {
            ppcVar1 = (code **)uVar2;
            (**ppcVar1)(0x1008,uVar4,(char)paVar11,0x1);
          }
        }
        else {
          ppcVar1 = (code **)((int)*puStack22 + 0x4);
          uVar3 = uVar4;
          (**ppcVar1)(0x8,uVar4,uVar15,0x0,0x0);
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT13((char)((u32)paVar6 >> 0x8),CONCAT12((char)paVar6,uVar5)));
          puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(uVar3,0x32),in_stack_0000fe66,
                                   in_stack_0000ff8a,in_stack_0000ff90,in_stack_0000ff94);
          pass1_1010_71d6(uVar5 + 0xc,((u32)puVar9 >> 0x10),(u32)puVar9,0x1,
                          (u16 *)(((u32)paVar6 & 0xff00) << 0x10 | (u32)CONCAT12((char)paVar6,uVar5 + 0xc)),
                          &DAT_1050_1050);
          if (puStack22 != NULL) {
            ppcVar1 = (code **)*puStack22;
            (**ppcVar1)(0x1010,uVar4,(char)paVar11,0x1);
          }
        }
      }
    }
  }
  else {
    if (param_4 == 0x1c5) {
      uVar15 = 0xe;
    }
    else {
      if (param_4 != 0x1c6) {
        post_win_msg_1040_7b3c
                  ((StructC *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,param_2)),(param_3 >> 0x10)
                   ,param_4,param_4);
        return;
      }
      uVar15 = 0xd;
    }
    uVar17 = 0x0;
    uVar16 = 0x0;
    uVar12 = 0x0;
    uVar13 = 0x0;
    uVar14 = 0x0;
    paVar11 = (astruct_57 *)
              mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)0x32,in_stack_0000fe68,in_stack_0000ff8c,
                              in_stack_0000ff92,in_stack_0000ff96);
    unk_win_op_1010_7300
              ((u32)paVar6 & 0xffff0000 | (u32)paVar11 >> 0x10,paVar11,CONCAT13(uVar14,CONCAT12(uVar13,uVar12)),
               uVar15,CONCAT22(uVar17,uVar16));
  }
  return;
}



StructD * pass1_1038_e608(StructD *param_1,param_2: u8)

{
  pass1_1038_e308(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_e69a(StructD *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  astruct_57 *uVar1;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfcb,param_6);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  iVar1[0x1].field2_0x4 = 0x0;
  param_2->field0_0x0 = 0xe92e;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x43),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_e6f0(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xe92e;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn unk_win_ui_op_1038_e71c(mut param_1: u16 ,StructB *param_2)

{
  let mut extraout_DX: u16;
  StructB *struct_1;
  let mut struct_1_lo: u16;
  char *pcStack6;

  dialog_ui_fn_1040_78e2(param_2);
  struct_1_lo = ((u32)param_2 >> 0x10);
  struct_1 = (StructB *)param_2;
  unk_load_str_op_1010_2c34((u32 *)(u32)&struct_1[0x7].field1_0x2);
  pcStack6 = (char *)CONCAT22(extraout_DX,param_1);
  unk_str_op_1000_3d3e
            ((char *)((u32)param_2 & 0xffff0000 | ZEXT24(&struct_1->field8_0x10)),
             (char *)CONCAT22(extraout_DX,param_1));
  fn_ptr_1000_17ce(pcStack6);
  move_win_1040_826c(param_2,-0x1,0xffff);
  ShowWindow16(0x5,(HWND16)struct_1->lpvoid_field_0x8);
  struct_1[0x7].lpvoid_field_0x8 = (LPVOID)((int)&PTR_LOOP_1050_0000 + 0x1);
  unk_win_msg_op_1008_9510(((u32)param_2 & 0xffff0000 | ZEXT24(&struct_1[0x7].lpvoid_field_0x8)));
  DestroyWindow16((HWND16)struct_1->lpvoid_field_0x8);
  return;
}
pub fn chk_is_dlg_btn_checked_1038_e7a0(param_1: *mut astruct_62,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut UVar2: u16;
  astruct_62 *iVar3;
  let mut uVar3: u16;

  iVar3 = (astruct_62 *)param_1;
  uVar3 = ((u32)param_1 >> 0x10);
  if (param_2 == 0x0) {
    uVar1 = iVar3->field142_0x8e;
    ((int)uVar1 + 0x10) = 0x1;
    uVar1 = iVar3->field142_0x8e;
    ((int)uVar1 + 0xa) = 0x0;
    uVar1 = iVar3->field142_0x8e;
    ((int)uVar1 + 0xc) = 0x0;
    uVar1 = iVar3->field142_0x8e;
    ((int)uVar1 + 0xe) = 0x0;
  }
  else {
    UVar2 = IsDlgButtonChecked(0x1827,*(HWND16 *)&iVar3->field_0x6);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked(0x1828,*(HWND16 *)&iVar3->field_0x6);
      if (UVar2 == 0x0) {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xa) = 0x0;
      }
      else {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = iVar3->field142_0x8e;
      ((int)uVar1 + 0xa) = 0x1;
    }
    UVar2 = IsDlgButtonChecked(s_vrpal_bmp_1050_183a,*(HWND16 *)&iVar3->field_0x6);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked((int)s_vrpal_bmp_1050_183a + 0x1,*(HWND16 *)&iVar3->field_0x6);
      if (UVar2 == 0x0) {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xc) = 0x0;
      }
      else {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xc) = 0x2;
      }
    }
    else {
      uVar1 = iVar3->field142_0x8e;
      ((int)uVar1 + 0xc) = 0x1;
    }
    UVar2 = IsDlgButtonChecked((int)s_vrpal_bmp_1050_183a + 0x2,*(HWND16 *)&iVar3->field_0x6);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked((int)s_vrpal_bmp_1050_183a + 0x3,*(HWND16 *)&iVar3->field_0x6);
      if (UVar2 == 0x0) {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xe) = 0x0;
      }
      else {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xe) = 0x2;
      }
    }
    else {
      uVar1 = iVar3->field142_0x8e;
      ((int)uVar1 + 0xe) = 0x1;
    }
    uVar1 = iVar3->field142_0x8e;
    ((int)uVar1 + 0x10) = 0x0;
  }
  iVar3->field143_0x92 = 0x0;
  return;
}
pub fn FUN_1038_e904(void)

{
  return;
}



StructD * pass1_1038_e908(StructD *param_1,param_2: u8)

{
  pass1_1038_e6f0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 *
pass1_1038_e99a(u8 *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  astruct_57 *uVar2;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfb9,param_6);
  uVar2 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  param_2->field0_0x0 = 0xeb32;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x30),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_e9ec(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xeb32;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn win_ui_op_1038_ea18(StructB *param_1)

{
  HWND16 hwnd;
  INT16 IVar1;
  StructB *iVar2;
  let mut uVar2: u16;
  let mut lparam: u32;
  let mut in_stack_0000fff0: bool;
  let mut iStack14: i16;

  dialog_ui_fn_1040_78e2(param_1);
  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (StructB *)param_1;
  lparam = pass1_1010_375e((u32)&iVar2[0x7].field1_0x2);
  hwnd = GetDlgItem16(0xfa5,(HWND16)iVar2->lpvoid_field_0x8);
  SendMessage16(lparam,0x0,0xc,hwnd);
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&stack0xfff0),iVar2->max_count_field_0x10);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  move_win_1040_826c(param_1,IVar1 + iStack14 + 0x5,in_stack_0000fff0);
  ShowWindow16(0x5,(HWND16)iVar2->lpvoid_field_0x8);
  return;
}
pub fn win_ui_op_1038_eaa2(param_1: *mut astruct_888,mut param_2: i16)

{
  HWND16 hwnd;
  astruct_888 *struct_1;
  let mut struct_1_lo: u16;
  LRESULT LVar1;

  struct_1 = (astruct_888 *)param_1;
  struct_1_lo = ((u32)param_1 >> 0x10);
  if (param_2 != 0x0) {
    hwnd = GetDlgItem16(0xfa5,struct_1->field6_0x6);
    LVar1 = SendMessage16(CONCAT22(0x1050,&stack0xffac),0x50,0xd,hwnd);
    pass1_1010_3770(((u32)LVar1 >> 0x10),(astruct_477 *)struct_1->field140_0x8e,
                    (char *)CONCAT22(0x1050,&stack0xffac));
    PostMessage16(0x0,0xfb,0x111,struct_1->field7_0x8);
  }
  DestroyWindow16(struct_1->field6_0x6);
  return;
}
pub fn FUN_1038_eb08(void)

{
  return;
}



StructD * pass1_1038_eb0c(StructD *param_1,param_2: u8)

{
  pass1_1038_e9ec(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_eb9e(param_1: *mut astruct_57,mut param_2: u16 )

{
  astruct_57 *uVar1;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c7,param_2);
  uVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  (u32)((int)param_1 + 0x8e) = 0x0;
  param_1->field0_0x0 = 0xee6e;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_ebd6(StructD *param_1)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  param_1->address_offset_field_0x0 = 0xee6e;
  (iVar1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(iVar1 + 0x6));
  fn_ptr_1000_17ce(*(char **)(iVar1 + 0x8e));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn FUN_1038_ec16(mut param_1: u16 ,StructB *param_2,param_3: *mut astruct_57,mut param_4: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  astruct_57 *rect;
  let mut uVar3: u16;
  let mut uVar4: u16;
  StructD *pSVar5;
  let mut iVar7: i16;
  let mut iVar8: i16;
  let mut unaff_SI: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut in_stack_0000fe2a: u16;
  let mut in_stack_0000fe2e: u16;
  let mut in_stack_0000fe7e: u16;
  let mut in_stack_0000ff54: u16;
  let mut in_stack_0000ff58: u16;
  let mut in_stack_0000ff5c: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffac: u16;
  let mut local_22: u16;
  let mut uStack32: u16;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut puStack26: *mut u16;
  let mut iStack10: i16;
  let mut uStack8: u16;
  u32 *puStack6;
  astruct_57 *paVar6;

  dialog_ui_fn_1040_78e2(param_2);
  puStack6 = mixed_1010_20ba(param_3,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2b),in_stack_0000fe7e,
                             in_stack_0000ffa2,in_stack_0000ffa8,in_stack_0000ffac);
  pSVar5 = (StructD *)((u32)param_3 & 0xffff0000 | (u32)puStack6 >> 0x10);
  uStack8 = pass1_1010_0892();
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
  }
  else {
    pSVar5 = (StructD *)((u32)pSVar5 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
  }
  puStack26 = (u16 *)CONCAT22(pSVar5,PTR_LOOP_1050_5f2c);
  uVar2 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar5);
  uVar9 = ((u32)param_2 >> 0x10);
  iVar7 = (int)param_2;
  (iVar7 + 0x8e) = uVar2;
  (iVar7 + 0x90) = (int)pSVar5;
  for (iStack10 = 0x1; uVar10 = ((u32)pSVar5 >> 0x10), iStack10 <= (int)uStack8; iStack10 += 0x1) {
    puStack26 = (u16 *)pass1_1010_0932(puStack6,((u32)puStack6 >> 0x10),iStack10);
    uVar3 = ((u32)puStack26 >> 0x10);
    paVar6 = (astruct_57 *)CONCAT22(uVar10,uVar3);
    local_22 = *puStack26;
    uStack32 = ((int)puStack26 + 0x2);
    uStack30 = 0x1;
    uStack28 = 0x1;
    rect = (astruct_57 *)&local_22;
    MapDialogRect16((RECT16 *)rect,(HWND16)&DAT_1050_1050);
    mem_op_1000_179c(0x42,paVar6);
    uVar4 = (astruct_57 *)paVar6 | rect;
    pSVar5 = (StructD *)((u32)paVar6 & 0xffff0000 | (u32)uVar4);
    if (uVar4 == 0x0) {
      uVar1 = (u32)(iVar7 + 0x8e);
      (u32)((int)uVar1 + iStack10 * 0x4) = 0x0;
    }
    else {
      uVar10 = (iVar7 + 0x6);
      pass1_1008_3bd6((u32)pSVar5,rect,(astruct_57 *)paVar6,0x0,CONCAT22(local_22,uStack32),0x101,0xff0100,
                      CONCAT13((char)(uVar10 >> 0x8),CONCAT12((char)uVar10,((int)puStack26 + 0x4)))
                      ,param_4,in_stack_0000fe2a,in_stack_0000fe2e,in_stack_0000ff54,in_stack_0000ff58,in_stack_0000ff5c
                     );
      uVar1 = (u32)(iVar7 + 0x8e);
      uVar10 = ((u32)uVar1 >> 0x10);
      iVar8 = (int)uVar1;
      *(astruct_57 **)(iVar8 + iStack10 * 0x4) = rect;
      (iVar8 + iStack10 * 0x4 + 0x2) = (int)pSVar5;
    }
    uVar1 = (u32)(iVar7 + 0x8e);
    uVar10 = ((u32)uVar1 >> 0x10);
    iVar8 = (int)uVar1;
    if (*(i32 *)(iVar8 + iStack10 * 0x4) != 0x0) {
      enable_win_1040_9234((u32)(iVar8 + iStack10 * 0x4),*(BOOL16 *)((int)puStack26 + 0x6));
    }
  }
  move_win_1040_826c(param_2,-0x1,0xffff);
  ShowWindow16(0x5,*(HWND16 *)(iVar7 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn send_msg_1038_ed8a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  u32 *puVar7;
  let mut uVar8: u32;
  let mut in_stack_0000fe8a: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffb8: u16;
  HWND16 hwnd;
  let mut in_stack_0000ffe2: u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  hwnd = HWND16_1050_0396;
  if (param_4 != 0x1c8) {
    if (param_4 == 0x1c9) {
      puVar7 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe2,0x2f),in_stack_0000fe8a,
                               in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
      uVar2 = ((u32)puVar7 >> 0x10);
      uVar5 = ((int)puVar7 + 0x20);
      uVar1 = ((int)puVar7 + 0x22);
      uVar8 = (u32)paVar6 & 0xffff0000 | (u32)uVar1;
      uVar3 = uVar1 | uVar5;
      if (uVar3 == 0x0) {
        return;
      }
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar1,uVar5));
      uVar5 = uVar8 | uVar3;
      paVar6 = (astruct_57 *)(uVar8 & 0xffff0000 | (u32)uVar5);
      if (uVar5 == 0x0) {
        return;
      }
      iVar4 = pass1_1030_5b00(CONCAT22(uVar8,uVar3));
      puVar7 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe2,iVar4),in_stack_0000fe8a,
                               in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
      if ((((u32)puVar7 >> 0x10) | puVar7) == 0x0) {
        return;
      }
      uVar8 = pass1_1018_0ad4((u32)puVar7);
      uVar5 = (uVar8 >> 0x10);
      if ((uVar5 | uVar8) == 0x0) {
        return;
      }
      param_4 = 0x72;
      hwnd = *(HWND16 *)(uVar8 + 0x8);
    }
    else if (param_4 != 0x1ca) {
      post_win_msg_1040_7b3c
                ((StructC *)CONCAT22((int)param_3,param_2),(param_3 >> 0x10),param_4,param_4);
      return;
    }
  }
  SendMessage16(0x0,param_4,0x111,hwnd);
  return;
}



StructD * pass1_1038_ee48(StructD *param_1,param_2: u8)

{
  pass1_1038_ebd6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_eeda(StructD *param_1,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  astruct_57 *uVar1;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x166,param_3);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  iVar1[0x1].field2_0x4 = 0x0;
  iVar1[0x1].field3_0x6 = 0x0;
  param_2->field0_0x0 = 0x67c;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x9),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  iVar1->field86_0x74 = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn destroy_win_1038_ef3a(StructD *param_1)

{
  let mut uVar2: u32;
  StructD *iVar1;
  StructD *uVar1;

  uVar1 = (StructD *)((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x67c;
  iVar1->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  if (*(i32 *)&iVar1->field_0x96 != 0x0) {
    uVar2 = (u32)&iVar1->field_0x96;
    DestroyWindow16(*(HWND16 *)((int)uVar2 + 0x6));
    (u32)&iVar1->field_0x96 = 0x0;
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1->field_0x6);
  ui_cleanup_op_1040_782c(param_1);
  return;
}

