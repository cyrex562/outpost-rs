
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_109c(u8 *param_1,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut uVar1: u32;
  let mut bVar2: bool;
  let mut iVar3: i16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  let mut uVar5: u32;
  astruct_57 *paVar6;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000fff2: u16;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  bVar2 = false;
  if (param_5 == 0x1c1) {
    (param_2 + 0x96) = 0x2;
    bVar2 = true;
  }
  else if (param_5 == 0x1c2) {
    (param_2 + 0x96) = 0x1;
    bVar2 = true;
  }
  else {
    if (param_5 != 0x1830) {
      post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
      return;
    }
    paVar6 = (astruct_57 *)
             mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0x32),in_stack_0000fe9a,
                             in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
    uVar5 = (u32)paVar4 & 0xffff0000 | (u32)paVar6 >> 0x10;
    iVar3 = (int)paVar6;
    uVar1 = (u32)(param_2 + 0x92);
    ui_op_1010_79aa(paVar6,0xfb6,*(i32 *)((int)uVar1 + 0x6));
    if (iVar3 == 0x0) {
      uVar1 = (u32)(param_2 + 0x92);
      unk_win_op_1010_7300(uVar5,paVar6,0x0,0xc,(u32)((int)uVar1 + 0x6));
    }
  }
  if (bVar2) {
    uVar1 = (u32)(param_2 + 0x8e);
    ((int)uVar1 + 0xa) = (param_2 + 0x96);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_1152(u8 *param_1,mut param_2: i16,mut param_3: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  u32 *puVar6;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000fff4: u16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (*(i32 *)(param_2 + 0x92) != 0x0) {
    uVar2 = (u32)(param_2 + 0x8e);
    uVar1 = ((int)uVar2 + 0xa);
    puVar6 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff4,0x3),in_stack_0000fe9c,
                             in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
    uVar2 = (u32)(param_2 + 0x92);
    uVar5 = ((u32)uVar2 >> 0x10);
    iVar4 = (int)uVar2;
    pass1_1010_ae92((u32)puVar6,uVar1,(iVar4 + 0xa),(u32)(iVar4 + 0x6),
                    (u32)paVar3 & 0xffff0000 | (u32)puVar6 >> 0x10);
  }
  destroy_win_1040_7b98(CONCAT22(param_3,param_2));
  PTR_LOOP_1050_5b80 = NULL;
  return;
}



StructD * pass1_1040_11ac(StructD *param_1,param_2: u8)

{
  pass1_1040_0e86(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 *
pass1_1040_123e(StructD *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfd1,param_6);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  param_2.field0_0x0 = 0x17b0;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x46),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_1290(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x17b0;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1040_12bc(undefined1 param_1,StructB *struct_b_param_1)

{
  let mut uVar1: u32;
  HWND16 HVar2;
  StructB *struct_b_3;
  let mut uVar3: u16;
  char *lparam;
  u8 local_54 [0x52];

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar3 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_3 = (StructB *)struct_b_param_1;
  uVar1 = (u32)&struct_b_3[0x7].field1_0x2;
  sys_1000_3f9c((char *)CONCAT22(0x1050,local_54),s__u_1050_5cd4,((int)uVar1 + 0xa));
  HVar2 = GetDlgItem16(0xfd2,(HWND16)struct_b_3.lpvoid_field_0x8);
  SendMessage16(CONCAT22(0x1050,local_54),0x0,0xc,HVar2);
  SetFocus16(HVar2);
  SendMessage16(-0x10000,0x0,0x401,HVar2);
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  lparam = load_string_1010_847e(_u16_1050_14cc,0x531);
  HVar2 = GetDlgItem16((int)s_vrpal_bmp_1050_183a + 0x5,(HWND16)struct_b_3.lpvoid_field_0x8);
  send_msg_1040_1696(struct_b_param_1,HVar2);
  SendMessage16((LPARAM)lparam,0xffff,0x40d,HVar2);
  HVar2 = GetDlgItem16((int)s_vrpal_bmp_1050_183a + 0x4,(HWND16)struct_b_3.lpvoid_field_0x8);
  send_msg_1040_1696(struct_b_param_1,HVar2);
  SendMessage16((LPARAM)lparam,0xffff,0x40d,HVar2);
  ShowWindow16(0x5,(HWND16)struct_b_3.lpvoid_field_0x8);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_msg_op_1040_13b2(param_1: *mut astruct_892,mut param_2: i16)

{
  HWND16 HVar1;
  let mut uVar4: u16;
  let mut iVar4: i16;
  u8 *puVar5;
  let mut iVar5: i16;
  char *puVar4;
  u8 *puVar6;
  u8 *puVar7;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  astruct_57 *paVar2;
  astruct_892 *struct_7;
  let mut iVar6: i16;
  let mut struct_5_lo: u16;
  let mut uVar6: u16;
  LRESULT lresult_4;
  char *pcVar6;
  u32 *puStack266;
  char local_100 [0x52];
  u8 local_aa [0x52];
  let mut uStack88: u16;
  HWND16 handle_86;
  u8 local_54 [0x52];
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u32;
  code **fn_ptr_1;

  uVar5 = ((u32)in_EDX >> 0x10);
  struct_7 = (astruct_892 *)param_1;
  struct_5_lo = ((u32)param_1 >> 0x10);
  if (param_2 != 0x0) {
    handle_86 = GetDlgItem16(0xfd2,struct_7.hwnd_0x6);
    SendMessage16(CONCAT22(0x1050,local_54),0x51,0xd,handle_86);
    uStack88 = pass1_1000_3e2c(CONCAT22(0x1050,local_54));
    HVar1 = GetDlgItem16((int)s_vrpal_bmp_1050_183a + 0x4,struct_7.hwnd_0x6);
    lresult_4 = SendMessage16(0x0,0x0,0x407,HVar1);
    if ((WPARAM16)lresult_4 != 0xffff) {
      SendMessage16(CONCAT22(0x1050,local_aa),(WPARAM16)lresult_4,0x408,HVar1);
    }
    HVar1 = GetDlgItem16((int)s_vrpal_bmp_1050_183a + 0x5,struct_7.hwnd_0x6);
    lresult_4 = SendMessage16(0x0,0x0,0x407,HVar1);
    if ((WPARAM16)lresult_4 != 0xffff) {
      SendMessage16(CONCAT22(0x1050,local_100),(WPARAM16)lresult_4,0x408,HVar1);
    }
    pcVar6 = load_string_1010_847e(_u16_1050_14cc,0x531);
    paVar2 = (astruct_57 *)CONCAT22(uVar5,local_aa);
    uVar4 = pass1_1000_3d7a((char *)CONCAT22(0x1050,local_aa),(char *)CONCAT22(0x1050,local_100));
    if (uVar4 != 0x0) {
      uVar4 = pass1_1000_3d7a((char *)CONCAT22(0x1050,local_aa),pcVar6);
      if (uVar4 != 0x0) {
        uVar4 = pass1_1000_3d7a((char *)CONCAT22(0x1050,local_100),pcVar6);
        if (uVar4 != 0x0) {
          pass1_1010_531c(local_aa,paVar2,struct_7.field141_0x8e,CONCAT22(0x1050,local_aa));
          puVar5 = local_100;
          pass1_1010_52fc(puVar5,paVar2,struct_7.field141_0x8e,CONCAT22(0x1050,puVar5));
          pass1_1010_5120(puVar5,paVar2,struct_7.field141_0x8e,uStack88);
          if (puVar5 == NULL) {
            mem_op_1000_179c(0xb4,paVar2);
            puVar7 = (paVar2 | puVar5);
            uVar3 = (u32)paVar2 & 0xffff0000 | ZEXT24(puVar7);
            if (puVar7 == NULL) {
              iVar5 = 0x0;
              uVar5 = 0x0;
            }
            else {
              iVar5 = string_1040_8520(uVar3,(astruct_57 *)CONCAT22(paVar2,puVar5),HWND16_1050_0396,0x20030,
                                       0x7d2057b);
              uVar5 = uVar3;
            }
            fn_ptr_1 = (code **)((int)(u32)CONCAT13((char)(uVar5 >> 0x8),CONCAT12((char)uVar5,iVar5)) +
                                0x74);
            (**fn_ptr_1)();
            return;
          }
          uVar1 = struct_7.field141_0x8e;
          uVar2 = struct_7.field141_0x8e;
          uVar6 = (uVar2 >> 0x10);
          iVar6 = (i16)uVar2;
          uVar3 = struct_7.field141_0x8e;
          pass1_1028_8d9e((astruct_97 *)CONCAT22(0x1050,&stack0xfdd2),(u32)((int)uVar3 + 0xa),
                          (u32)((int)uVar1 + 0x12),
                          (u32)(iVar6 + 0x16) & 0xffff | (u32)(iVar6 + 0x18) << 0x10);
          fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&stack0xfdd2));
          pass1_1028_8dec((u16 *)CONCAT22(0x1050,&stack0xfdd2));
          goto LAB_1040_1619;
        }
      }
    }
    mem_op_1000_179c(0xb4,paVar2);
    puVar6 = (paVar2 | uVar4);
    uVar3 = (u32)paVar2 & 0xffff0000 | ZEXT24(puVar6);
    if (puVar6 == NULL) {
      iVar4 = 0x0;
      uVar5 = 0x0;
    }
    else {
      iVar4 = string_1040_8520(uVar3,(astruct_57 *)CONCAT22(paVar2,uVar4),HWND16_1050_0396,0x20030,0x755057b);
      uVar5 = uVar3;
    }
    puStack266 = (u32 *)CONCAT22(uVar5,iVar4);
    fn_ptr_1 = (code **)((int)*puStack266 + 0x74);
    (**fn_ptr_1)();
  }//
LAB_1040_1619:
  DestroyWindow16(struct_7.hwnd_0x6);
  return;
}



pub fn set_win_pos_1040_162a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32) -> u32

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  let mut iStack6: i16;

  if ((param_4 != (int)s_vrpal_bmp_1050_183a + 0x5) && (param_4 != (int)s_vrpal_bmp_1050_183a + 0x4)) {
    BVar2 = post_win_msg_1040_7b3c
                      ((StructC *)CONCAT22((int)param_3,param_2),param_3,param_4,param_4);
    return CONCAT22(param_1,BVar2);
  }
  if (param_4 == 0x7) {
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,&stack0xfff6),param_3);
    SetWindowPos16(0x2,0x50,iStack6 - param_3,0x0,0x0,0x0,param_3);
  }
  else if ((param_4 != 0x9) && (param_4 != 0xa)) {
    uVar1 = 0x0;
    goto LAB_1040_164d;
  }
  uVar1 = 0x1;//
LAB_1040_164d:
  return (u32)uVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn send_msg_1040_1696(StructB *param_1,mut param_2: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut puVar3: *mut u16;
  u8 *puVar4;
  u8 *puVar5;
  let mut uVar6: u16;
  LRESULT LVar7;
  char *pcVar8;
  WPARAM16 WVar9;
  let mut UVar10: u16;
  let mut uVar11: u16;
  let mut uStack18: u16;
  let mut local_4: u16;

  SendMessage16(0x0,0x0,0x40b,param_2);
  LVar7 = SendMessage16(0x0,0x0,0xb,param_2);
  puVar4 = ((u32)LVar7 >> 0x10);
  local_4 = 0x0;
  puVar3 = &local_4;
  uVar6 = ((u32)param_1 >> 0x10);
  pass1_1010_519a(puVar4,(u32)((int)param_1 + 0x8e),(char *)CONCAT22(0x1050,puVar3));
  puVar5 = puVar4;
  for (uStack18 = 0x0; uStack18 < local_4; uStack18 += 0x1) {
    uVar1 = (u32)(puVar3 + uStack18 * 0x2);
    WVar9 = 0x0;
    UVar10 = 0x403;
    uVar2 = (u32)((int)param_1 + 0x8e);
    uVar11 = param_2;
    pcVar8 = (char *)string_1010_5286((char *)uVar1,puVar5,uVar2,((u32)uVar2 >> 0x10),uVar1);
    LVar7 = SendMessage16((LPARAM)pcVar8,WVar9,UVar10,uVar11);
    puVar5 = ((u32)LVar7 >> 0x10);
    fn_ptr_1000_17ce(pcVar8);
  }
  WVar9 = 0x0;
  UVar10 = 0x40a;
  uVar11 = param_2;
  pcVar8 = load_string_1010_847e(_u16_1050_14cc,0x531);
  SendMessage16((LPARAM)pcVar8,WVar9,UVar10,uVar11);
  SendMessage16(0x0,0x1,0xb,param_2);
  return;
}
pub fn FUN_1040_1786(void)

{
  return;
}



StructD * pass1_1040_178a(StructD *param_1,param_2: u8)

{
  pass1_1040_1290(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_181c(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

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
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfbb,param_6);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  iVar1[0x1].field2_0x4 = 0x0;
  iVar1[0x1].field3_0x6 = 0x0;
  param_2.field0_0x0 = 0x1c48;
  iVar1.field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x2),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_1876(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1.address_offset_field_0x0 = 0x1c48;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn show_win_1040_18a2(StructB *struct_b_param_1)

{
  let mut uVar1: u32;
  StructB *struct_b_2;
  let mut uVar2: u16;
  WORD local_304 [0x80];
  char local_204 [0x100];
  char local_104 [0x100];
  let mut uStack4: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  check_dialog_btn_1040_1afe(struct_b_param_1);
  struct_b_2 = (StructB *)struct_b_param_1;
  uVar2 = ((u32)struct_b_param_1 >> 0x10);
  if (PTR_LOOP_1050_13ae != NULL) {
    if (PTR_LOOP_1050_13ae == &u16_1050_0002) {
      uStack4 = 0x621;
    }
    else if (PTR_LOOP_1050_13ae == ((int)&u16_1050_0002 + 0x1)) {
      uStack4 = 0x622;
    }
    else if (PTR_LOOP_1050_13ae == &u32_1050_0004) {
      uStack4 = 0x623;
    }
    else {
      uStack4 = 0x620;
    }
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_204,(short)&DAT_1050_1050);
    wsprintf16(local_304,(char *)0x5cda1050,(char *)CONCAT22(local_204,0x1050),(int)&DAT_1050_1050);
    SetDlgItemText16(CONCAT22(0x1050,local_304),0xfe0,(HWND16)struct_b_2.lpvoid_field_0x8);
    uVar1 = (u32)&struct_b_2[0x7].field1_0x2;
    if (((int)uVar1 + 0x82) == 0x0) {
      uStack4 = 0x627;
    }
    else {
      uStack4 = 0x626;
    }
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_204,(short)&DAT_1050_1050);
    wsprintf16(local_304,(char *)0x5cdf1050,(char *)CONCAT22(local_204,0x1050),(int)&DAT_1050_1050);
    SetDlgItemText16(CONCAT22(0x1050,local_304),0xfdf,(HWND16)struct_b_2.lpvoid_field_0x8);
  }
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  ShowWindow16(0x5,(HWND16)struct_b_2.lpvoid_field_0x8);
  return;
}
pub fn unk_win_ui_op_1040_19ea(param_1: *mut astruct_32,mut param_2: i16,u8 *param_3)

{
  StructD *pSVar1;
  let mut UVar2: u16;
  astruct_32 *pstruct32_1;
  astruct_32 *pstruct_32_hi;

  pstruct32_1 = (astruct_32 *)param_1;
  pstruct_32_hi = (astruct_32 *)((u32)param_1 >> 0x10);
  if (param_2 != 0x0) {
    UVar2 = IsDlgButtonChecked(0xfdb,pstruct32_1.field6_0x6);
    pass1_1010_5d9c(param_3,(u32)pstruct32_1.pstructd_0x8e,UVar2);
    UVar2 = IsDlgButtonChecked(0xfdc,pstruct32_1.field6_0x6);
    pSVar1 = pstruct32_1.pstructd_0x8e;
    ((int)pSVar1 + 0x20) = UVar2;
    UVar2 = IsDlgButtonChecked(0xfdd,pstruct32_1.field6_0x6);
    pSVar1 = pstruct32_1.pstructd_0x8e;
    ((int)pSVar1 + 0x74) = UVar2;
    UVar2 = IsDlgButtonChecked(0xfde,pstruct32_1.field6_0x6);
    pSVar1 = pstruct32_1.pstructd_0x8e;
    ((int)pSVar1 + 0x72) = UVar2;
    if (pstruct32_1.field142_0x92 != 0x0) {
      pSVar1 = pstruct32_1.pstructd_0x8e;
      pass1_1000_4906((StructD *)((u32)pSVar1 & 0xffff0000 | (u32)((int)pSVar1 + 0x22)),NULL,0x40);
    }
    if (pstruct32_1.field143_0x94 != 0x0) {
      pass1_1010_60a0((astruct_19 *)pstruct32_1.pstructd_0x8e);
    }
  }
  DestroyWindow16(pstruct32_1.field6_0x6);
  return;
}



pub fn pass1_1040_1ab0(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32) -> u32

{
  let mut BStack6: bool;
  let mut uStack4: u16;

  BStack6 = 0x0;
  uStack4 = 0x0;
  if (param_5 == 0x1831) {
    (param_2 + 0x92) = 0x1;
    (param_2 + 0x94) = 0x1;
    check_dialog_btn_1040_1b8a((StructC *)CONCAT22(param_3,param_2));
  }
  else {
    BStack6 = post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
    uStack4 = param_1;
  }
  return CONCAT22(uStack4,BStack6);
}
pub fn check_dialog_btn_1040_1afe(StructB *param_1)

{
  let mut check: u16;
  let mut check_00: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;
  StructB *iVar3;
  let mut uVar3: u16;
  let mut check_01: u16;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (StructB *)param_1;
  uVar1 = (u32)&iVar3[0x7].field1_0x2;
  uVar2 = (u32)&iVar3[0x7].field1_0x2;
  check = ((int)uVar2 + 0x20);
  uVar2 = (u32)&iVar3[0x7].field1_0x2;
  check_00 = ((int)uVar2 + 0x74);
  uVar2 = (u32)&iVar3[0x7].field1_0x2;
  check_01 = ((int)uVar2 + 0x72);
  CheckDlgButton16(((int)uVar1 + 0x1e),0xfdb,(HWND16)iVar3.lpvoid_field_0x8);
  CheckDlgButton16(check_00,0xfdd,(HWND16)iVar3.lpvoid_field_0x8);
  CheckDlgButton16(check_01,0xfde,(HWND16)iVar3->lpvoid_field_0x8);
  CheckDlgButton16(check,0xfdc,(HWND16)iVar3->lpvoid_field_0x8);
  return;
}
pub fn check_dialog_btn_1040_1b8a(StructC *param_1)

{
  let mut check: u16;
  let mut check_00: u16;
  let mut check_01: u16;
  StructC *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructC *)param_1;
  check_00 = pass1_1010_60b4();
  pass1_1010_60c6();
  check_01 = pass1_1010_60c0();
  pass1_1010_60ba();
  CheckDlgButton16(check_00,0xfdb,iVar1->field6_0x6);
  CheckDlgButton16(check_01,0xfdd,iVar1->field6_0x6);
  CheckDlgButton16(0xfde,0xfde,iVar1->field6_0x6);
  check = iVar1->field6_0x6;
  CheckDlgButton16(check,0xfdc,check);
  return;
}
pub fn FUN_1040_1c1e(void)

{
  return;
}



StructD * pass1_1040_1c22(StructD *param_1,param_2: u8)

{
  pass1_1040_1876(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_1cb4(StructD *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar2;
  let mut unaff_BP: u16;
  astruct_57 *uVar2;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;
  u8 **ppuVar3;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xe8,param_6);
  uVar2 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar2 = (astruct_57 *)param_2;
  (u32)(iVar2 + 0x1) = 0x0;
  (u32)&iVar2[0x1].field2_0x4 = 0x0;
  param_2->field0_0x0 = 0x1eee;
  iVar2->field1_0x2 = &PTR_LOOP_1050_1040;
  ppuVar3 = (u8 **)CONCAT22(unaff_BP,0x2);
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,ppuVar3,in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  (iVar2 + 0x1)->field0_0x0 = puVar2;
  iVar2[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  puVar2 = mixed_1010_20ba((astruct_57 *)((u32)paVar1 & 0xffff0000 | (u32)puVar2 >> 0x10),_u16_1050_0ed0,
                           (u8 **)CONCAT22((int)((u32)ppuVar3 >> 0x10),0x37),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar2;
  iVar2[0x1].field3_0x6 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_1d24(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x1eee;
  ((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn show_win_1040_1d50(StructB *param_1)

{
  dialog_ui_fn_1040_78e2(param_1);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(0x5,*(HWND16 *)((int)param_1 + 0x6));
  return;
}
pub fn unk_win_ui_op_1040_1d7a(param_1: *mut astruct_33,mut param_2: i16)

{
  let mut UVar2: u16;
  let mut UVar1: u16;
  astruct_33 *iVar3;
  astruct_33 *uVar3;
  let mut uVar1: u32;

  iVar3 = (astruct_33 *)param_1;
  uVar3 = (astruct_33 *)((u32)param_1 >> 0x10);
  if ((param_2 != 0x0) && (uVar1 = iVar3->field141_0x8e, ((int)uVar1 + 0x72) != 0x0)) {
    UVar2 = IsDlgButtonChecked(0xe1,iVar3->hwnd_0x6);
    if (UVar2 != 0x0) {
      pass1_1008_a930(iVar3->field142_0x92,0x1d5);
    }
    UVar1 = IsDlgButtonChecked(0xe2,iVar3->hwnd_0x6);
    if (UVar1 != 0x0) {
      pass1_1008_a930(iVar3->field142_0x92,0x1d6);
    }
    UVar1 = IsDlgButtonChecked(0xe3,iVar3->hwnd_0x6);
    if (UVar1 != 0x0) {
      pass1_1008_a930(iVar3->field142_0x92,0x1d7);
    }
    UVar1 = IsDlgButtonChecked(0xe5,iVar3->hwnd_0x6);
    if (UVar1 != 0x0) {
      pass1_1008_a930(iVar3->field142_0x92,0x1d8);
    }
    UVar1 = IsDlgButtonChecked(0xe6,iVar3->hwnd_0x6);
    if (UVar1 != 0x0) {
      pass1_1008_a930(iVar3->field142_0x92,0x1e2);
    }
    UVar1 = IsDlgButtonChecked(0xe7,iVar3->hwnd_0x6);
    if (UVar1 != 0x0) {
      pass1_1008_a930(iVar3->field142_0x92,0x1dc);
    }
    return;
  }
  DestroyWindow16(iVar3->hwnd_0x6);
  return;
}



pub fn pass1_1040_1e80(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32) -> u32

{
  let mut BStack6: bool;
  let mut uStack4: u16;

  BStack6 = 0x0;
  uStack4 = 0x0;
  if (param_5 == 0xe4) {
    pass1_1008_a9ec((u32)(param_2 + 0x92));
  }
  else {
    BStack6 = post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
    uStack4 = param_1;
  }
  return CONCAT22(uStack4,BStack6);
}
pub fn FUN_1040_1ec4(void)

{
  return;
}



StructD * pass1_1040_1ec8(StructD *param_1,param_2: u8)

{
  pass1_1040_1d24(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar6
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1040_1f5a(param_1: *mut astruct_57,mut param_2: u16 ,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  astruct_57 *paVar3;
  let mut unaff_DI: u16;
  let mut unaff_CS: u16;
  let mut uVar4: u32;
  u32 *puVar5;
  astruct_27 *paVar6;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa6: u16;
  u32 *puVar7;
  u32 *puVar8;
  let mut uVar9: u16;
  let mut local_16: u32;
  let mut uStack18: u32;
  astruct_57 *iVar6;
  let mut uVar6: u16;

  iVar6 = (astruct_57 *)param_1;
  uVar6 = ((u32)param_1 >> 0x10);
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfcf,param_2);
  (u32)(iVar6 + 0x1) = 0x0;
  iVar6[0x1].field10_0x14 = 0x0;
  iVar6[0x1].field11_0x18 = 0x0;
  param_1->field0_0x0 = 0x237e;
  iVar6->field1_0x2 = &PTR_LOOP_1050_1040;
  uVar2 = FUN_1010_830a(0x0,param_3,unaff_CS,_u16_1050_14cc,0x1cc);
  (iVar6 + 0x1)->field0_0x0 = uVar2;
  iVar6[0x1].field1_0x2 = param_3;
  uVar4 = pass1_1008_4772((astruct_76 *)CONCAT22(param_3,(iVar6 + 0x1)->field0_0x0));
  paVar3 = (astruct_57 *)(param_3 & 0xffff0000 | uVar4 >> 0x10);
  uVar9 = (uVar4 >> 0x10);
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_DI,0x48),in_stack_0000fe78,
                           in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
  local_16 = CONCAT22(((int)uVar4 + 0x8) + 0xa,0xa);
  uStack18 = CONCAT22(0x1d6,((int)uVar4 + 0x4) + -0xa);
  (u32)&iVar6[0x1].field2_0x4 = local_16;
  (u32)&iVar6[0x1].field4_0x8 = uStack18;
  (u32)&iVar6[0x1].field6_0xc = local_16;
  (u32)&iVar6[0x1].field8_0x10 = uStack18;
  puVar1 = &iVar6[0x1].field7_0xe;
  *puVar1 = *puVar1 + 0x14;
  puVar8 = &iVar6[0x1].field10_0x14;
  puVar7 = &iVar6[0x1].field11_0x18;
  uVar9 = uVar6;
  paVar6 = (astruct_27 *)
           mixed_1010_20ba((astruct_57 *)((u32)paVar3 & 0xffff0000 | (u32)puVar5 >> 0x10),_u16_1050_0ed0,
                           (u8 **)CONCAT22(puVar7,0x2b),in_stack_0000fe70,in_stack_0000ff94,in_stack_0000ff9a,
                           in_stack_0000ff9e);
  pass1_1010_0538(paVar6,(char **)CONCAT22(uVar6,puVar7),(char **)CONCAT22(uVar9,puVar8));
  return;
}

