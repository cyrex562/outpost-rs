
pub fn invalidate_rect_1020_3080(mut param_1: u32,mut param_2: i16)

{
  if (param_2 == 0x1) {
    (param_1 + 0x14) = 0x0;
    return;
  }
  if ((param_2 != 0x4) && (param_2 != 0x6)) {
    return;
  }
  InvalidateRect16(0x0,NULL,0x0);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
pub fn draw_op_1020_30be(astruct_762 *struct_param_1)

{
  let mut is_iconic: bool;
  astruct_762 *iVar5;
  astruct_762 *uVar5;
  HDC16 *pHVar1;
  HDC16 *pHVar2;
  RECT16 rect_30 [0x2];
  HGDIOBJ16 hbrush_40;
  HICON16 hicon_38;
  HDC16 local_24;
  u8 local_22 [0x20];
  let mut IVar4: i16;
  let mut uVar1: u32;
  let mut uVar2: u32;
  code **fn_ptr_1;

  uVar5 = (astruct_762 *)(struct_param_1 >> 0x10);
  iVar5 = (astruct_762 *)struct_param_1;
  local_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),iVar5.field4_0x4);
  is_iconic = IsIconic16(iVar5.field4_0x4);
  if (is_iconic == 0x0) {
    pass1_1018_0a50(iVar5.field19_0x14);
    pHVar2 = &local_24;
    IVar4 = (i16)&DAT_1050_1050;
    fn_ptr_1 = (code **)(CONCAT22(0x1050,pHVar2) + 0x8);
    pHVar1 = pHVar2;
    (**fn_ptr_1)(0x1018,pHVar2,&DAT_1050_1050);
    uVar2 = iVar5.field19_0x14;
    if ((uVar2 + 0x84) == 0x1) {
      unk_draw_op_1020_320e(struct_param_1,local_24);
    }
    fn_ptr_1 = (code **)(CONCAT22(IVar4,pHVar2) + 0x4);
    (**fn_ptr_1)(0x1018,pHVar2,IVar4,0x0,0x0,(char)&local_24,&DAT_1050_1050,pHVar1);
    uVar1 = iVar5.field19_0x14;
    if ((uVar1 + 0x84) != 0x1) {
      unk_draw_op_1020_320e(struct_param_1,local_24);
    }
    draw_op_1020_3488(struct_param_1);
    fn_ptr_1 = (code **)(CONCAT22(IVar4,pHVar2) + 0xc);
    (**fn_ptr_1)(0x1018,pHVar2,IVar4,&local_24,&DAT_1050_1050);
  }
  else if (PTR_LOOP_1050_0010 == NULL) {
    fn_ptr_1 = (code **)(iVar5.field19_0x14 + 0x2c);
    hicon_38 = (**fn_ptr_1)(s_tile2_bmp_1050_1538);
    if (hicon_38 != 0x0) {
      hbrush_40 = GetStockObject16(BLACK_BRUSH);
      GetClientRect16(rect_30,(HWND16)&DAT_1050_1050);
      FillRect16(hbrush_40,(RECT16 *)&stack0xffc4,(HDC16)&DAT_1050_1050);
      DrawIcon16(hicon_38,0x2,0x2,local_24);
    }
  }
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),iVar5.field4_0x4);
  return;
}



// WARNING: Unable to use type for symbol uVar4
pub fn unk_draw_op_1020_320e(astruct_762 *astruct762_param_1,HDC16 hdc_param_2)

{
  let mut puVar1: *mut u32;
  code **ppcVar2;
  let mut uVar3: u32;
  HPALETTE16 obj;
  let mut in_DX: u16;
  let mut extraout_DX: u16;
  astruct_762 *struct_a;
  let mut iVar7: i16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  DEVMODEA *uVar8;
  let mut device: *mut c_char;
  let mut driver: *mut c_char;
  let mut local_c: i16;
  let mut local_a: u32;
  HDC16 *pHStack6;
  HDC16 hdc_var4;
  let mut puVar2: *mut u32;
  let mut uVar4: u32;
  u8 uVar9;
  u8 uVar10;
  let mut uVar11: u16;

  hdc_var4 = hdc_param_2;
  uVar6 = (astruct762_param_1 >> 0x10);
  struct_a = (astruct_762 *)astruct762_param_1;
  uVar4 = struct_a.field19_0x14;
  if ((uVar4 + 0x84) == 0x1) {
    uVar3 = struct_a.field19_0x14;
    uVar7 = (uVar3 >> 0x10);
    iVar7 = (i16)uVar3;
    puVar1 = (iVar7 + 0x24);
    driver = s_dib_1050_42c6;
    device = NULL;
    uVar9 = '\0';
    uVar10 = '\0';
    uVar11 = 0x0;
    uVar8 = (DEVMODEA *)pass1_1008_4772((astruct_76 *)(puVar1 & 0xffff | (iVar7 + 0x26) << 0x10))
    ;
    hdc_var4 = CreateDC16(uVar8,CONCAT22(uVar11,CONCAT11(uVar10,uVar9)),device,driver);
    pHStack6 = &hdc_var4;
    ppcVar2 = (code **)(*puVar1 + 0x8);
    (**ppcVar2)(s_tile2_bmp_1050_1538);
    in_DX = extraout_DX;
  }
  pass1_1018_0d9a(struct_a.field19_0x14,CONCAT22(0x1050,&local_c),CONCAT22(0x1050,&local_a));
  uVar3 = struct_a.field19_0x14;
  draw_op_1020_33c0(in_DX,astruct762_param_1,*(COLORREF *)(uVar3 + 0x6c),local_c,local_a,0x1,hdc_var4);
  pass1_1018_1054(struct_a.field19_0x14,CONCAT22(0x1050,&local_c),CONCAT22(0x1050,&local_a));
  uVar3 = struct_a.field19_0x14;
  draw_op_1020_33c0(in_DX,astruct762_param_1,*(COLORREF *)(uVar3 + 0x74),local_c,local_a,0x2,hdc_var4);
  pass1_1018_1320(struct_a.field19_0x14,CONCAT22(0x1050,&local_c),CONCAT22(0x1050,&local_a));
  uVar3 = struct_a.field19_0x14;
  draw_op_1020_33c0(in_DX,astruct762_param_1,*(COLORREF *)(uVar3 + 0x68),local_c,local_a,0x1,hdc_var4);
  pass1_1018_15f6(struct_a.field19_0x14,CONCAT22(0x1050,&local_c),CONCAT22(0x1050,&local_a));
  if (local_c != 0x0) {
    uVar3 = struct_a.field19_0x14;
    draw_op_1020_33c0(in_DX,astruct762_param_1,*(COLORREF *)(uVar3 + 0x70),local_c,local_a,0x1,hdc_var4);
  }
  pass1_1018_108c(struct_a.field19_0x14,CONCAT22(0x1050,&local_c),CONCAT22(0x1050,&local_a));
  if (local_c != 0x0) {
    uVar3 = struct_a.field19_0x14;
    draw_op_1020_33c0(in_DX,astruct762_param_1,*(COLORREF *)(uVar3 + 0x78),local_c,local_a,0x0,hdc_var4);
  }
  uVar3 = struct_a.field19_0x14;
  if ((uVar3 + 0x84) == 0x1) {
    obj = SelectPalette16(0x0,(HPALETTE16)pHStack6,hdc_var4);
    DeleteObject16(obj);
    DeleteDC16(hdc_var4);
  }
  return;
}
pub fn draw_op_1020_33c0(mut param_1: u16 ,mut param_2: u32,COLORREF colorref_param_2,mut param_4: i16,mut param_5: u32,mut param_6: i16,
                      HDC16 hdc16_param_6)

{
  HPEN16 pen_handle;
  HGDIOBJ16 object_handle;
  HBRUSH16 brush_handle;
  HGDIOBJ16 obj_handle_2;
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_DX: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut unaff_SS: u16;
  let mut uVar6: u16;
  HDC16 HVar7;
  let mut iStack20: i16;
  let mut puStack14: *mut u16;
  let mut uVar5: u16;

  if (param_4 != 0x0) {
    HVar7 = hdc16_param_6;
    pen_handle = CreatePen16(colorref_param_2,0x1,0x0);
    object_handle = SelectObject16(pen_handle,HVar7);
    HVar7 = hdc16_param_6;
    brush_handle = CreateSolidBrush16(colorref_param_2);
    obj_handle_2 = SelectObject16(brush_handle,HVar7);
    puStack14 = param_5;
    for (iStack20 = 0x0; iStack20 < param_4; iStack20 += 0x1) {
      uVar6 = (param_2 >> 0x10);
      uVar1 = param_4;
      pass1_1020_3540(param_1,param_2,uVar6,param_6,puStack14);
      if (param_6 < 0x1) {
        uVar2 = 0x3;
      }
      else {
        uVar2 = 0x4;
      }
      uVar3 = param_1;
      draw_polygon_1020_3602(param_2,uVar6,uVar2,uVar1,param_1);
      fn_ptr_1000_17ce(CONCAT22(param_1,uVar1));
      puStack14 = (puStack14 & 0xffff0000 | (puStack14 + 0x6));
      param_1 = uVar3;
    }
    obj_handle_2 = SelectObject16(obj_handle_2,hdc16_param_6);
    DeleteObject16(obj_handle_2);
    SelectObject16(object_handle,hdc16_param_6);
    DeleteObject16(pen_handle);
  }
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar3
pub fn draw_op_1020_3488(param_1: *mut astruct_762)

{
  let mut uVar6: u32;
  HPEN16 handle;
  HGDIOBJ16 handle_00;
  HGDIOBJ16 obj_handle_7;
  let mut uVar5: u16;
  let mut unaff_SS: u16;
  HDC16 in_stack_00000008;
  HDC16 hdc;
  let mut local_a: u32;
  let mut puStack6: *mut u16;
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut uVar4: u32;
  let mut left: i16;
  HDC16 hdc16_ffe2;

  uVar5 = (param_1 >> 0x10);
  uVar2 = (param_1 + 0x14);
  puStack6 = (uVar2 & 0xffff0000 | (uVar2 + 0x36));
  pass1_1008_3e94(puStack6,CONCAT22(0x1050,&local_a),CONCAT22(0x1050,&local_a + 0x2));
  uVar4 = (local_a - 0x3U) << 0x10;
  if ((local_a - 0x3U) < 0x0) {
    uVar4 = 0x0;
  }
  uVar1 = local_a - 0x3;
  uVar6 = uVar1;
  if (uVar1 < 0x0) {
    uVar6 = 0x0;
  }
  local_a = uVar4 | uVar6;
  uVar3 = (param_1 + 0x14);
  hdc = in_stack_00000008;
  handle = CreatePen16(*(COLORREF *)(uVar3 + 0x64),0x1,0x0);
  handle_00 = SelectObject16(handle,hdc);
  obj_handle_7 = GetStockObject16(HOLLOW_BRUSH);
  obj_handle_7 = SelectObject16(obj_handle_7,hdc16_ffe2);
  left = (i16)(local_a >> 0x10);
  Rectangle16(local_a + 0x6,left + 0x6,local_a,left,in_stack_00000008);
  SelectObject16(handle_00,in_stack_00000008);
  SelectObject16(obj_handle_7,in_stack_00000008);
  DeleteObject16(handle);
  return;
}
pub fn pass1_1020_3540(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,param_5: *mut u16)

{
  let mut iVar1: i16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  astruct_279 *iVar2;
  let mut iStack18: i16;
  let mut iStack12: i16;
  let mut iStack10: i16;
  let mut local_6: i16;
  let mut local_4: i16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pass1_1008_3e94(param_5,CONCAT22(0x1050,&local_6),CONCAT22(0x1050,&local_4));
  if (param_4 == 0x0) {
    iStack12 = 0x3;
    iStack10 = 0x42a6;
  }
  else if (param_4 == 0x1) {
    iStack12 = 0x4;
    iStack10 = s_SITEICON_1050_428d + 0x9;
  }
  else {
    if (param_4 != 0x2) {
      return;
    }
    iStack12 = 0x4;
    iStack10 = 0x42b2;
  }
  iVar1 = iStack12 << 0x2;
  mem_op_1000_179c(iVar1,paVar2);
  for (iStack18 = 0x0; iStack18 < iStack12; iStack18 += 0x1) {
    iVar2 = (astruct_279 *)(iStack18 * 0x4);
    (iVar2 + iVar1) = (iVar2 + iStack10) + local_4;
    (iVar2 + iVar1 + 0x2) = (iVar2 + iStack10 + 0x2) + local_6;
  }
  return;
}
pub fn draw_polygon_1020_3602(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  Polygon16(param_3,(POINT16 *)param_4,param_5);
  return;
}



StructD * pass1_1020_3616(StructD *param_1,param_2: u8)

{
  cleanup_win_ui_1020_2fea(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1020_3644(mut param_1: u32,param_2: *mut StructA,mut param_3: u16 ,mut param_4: u32,mut param_5: u16 ,
                     mut param_6: u16 )

{
  let iVar2: *mut StructA;
  let mut in_buf_len_5: u16;
  let mut in_stack_0000fe52: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7c: u16;
  let mut in_stack_0000ff80: u16;
  astruct_270 *iVar1;

  struct_1020_790e(&param_2.field0_0x0,NULL,param_3,param_4);
  in_buf_len_5 = (param_2 >> 0x10);
  iVar2 = (StructA *)param_2;
  iVar2[0x1].field20_0x26 = (astruct_243 *)0x389a;
  iVar2[0x1].field21_0x28 = 0x1008;
  iVar2[0x1].field20_0x26 = (astruct_243 *)0x3aa8;
  iVar2[0x1].field21_0x28 = 0x1008;
  iVar2[0x1].field29_0x34 = 0x0;
  iVar2[0x1].field37_0x3e = 0x0;
  iVar2[0x1].field38_0x42 = 0x0;
  param_2.field0_0x0 = 0x3d08;
  iVar2.field1_0x2 = 0x1020;
  iVar2[0x1].field20_0x26 = (astruct_243 *)0x3d9c;
  iVar2[0x1].field21_0x28 = 0x1020;
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,&iVar2.field5_0xa,
             in_buf_len_5);
  unk_str_op_1000_3d3e((param_2 & 0xffff0000 | ZEXT24(&iVar2.field60_0x5b)),s_VrMode_1050_42ca);
  iVar2.field140_0xac = 0x44c00000;
  window_op_1020_38aa(param_1,(StructA *)(param_2 & 0xffff | in_buf_len_5 << 0x10),param_6,param_5,
                      in_stack_0000ff7c,in_stack_0000ff80,in_stack_0000fe52,in_stack_0000ff76);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1020_36f6(mut param_1: u32,mut param_2: i16)

{
  let mut iVar1: i16;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut pcVar4: *mut c_char;
  let mut uVar5: u16;
  let mut HVar6: HWND16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut pcVar11: *mut c_char;
  u8 uVar12;
  u8 uVar13;
  let mut puStack1034: *mut u8,
  char local_406 [0x400];
  let mut uStack6: u32;

  iVar9 = param_1;
  uVar10 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    (iVar9 + 0x8) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  uStack6 = pass1_1018_1e78((iVar9 + 0x8),-0x1);
  uVar7 = (uStack6 >> 0x10);
  uVar3 = (iVar9 + 0x18);
  GetWindowText16(0x3ff,CONCAT22(0x1050,local_406),*(HWND16 *)(uVar3 + 0x6));
  pcVar4 = pass1_1000_472c(CONCAT22(0x1050,local_406),':');
  puStack1034 = CONCAT22(uVar7,pcVar4 + 0x2);
  *puStack1034 = 0x0;
  load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_406,(short)&DAT_1050_1050);
  uVar3 = (iVar9 + 0x18);
  ppcVar2 = (code **)((iVar9 + 0x18) + 0x18);
  (**ppcVar2)(0x1010,uVar3,(uVar3 >> 0x10),local_406,&DAT_1050_1050);
  uVar3 = (iVar9 + 0x8);
  iVar1 = (uVar3 + 0x4a);
  uVar3 = (iVar9 + 0x18);
  HVar6 = *(HWND16 *)(uVar3 + 0x6);
  SetDlgItemText16((uStack6 + 0x2),0x10f,HVar6);
  SetDlgItemText16((uStack6 + 0xa),0x110,HVar6);
  SetDlgItemText16((uStack6 + 0x12),0x112,HVar6);
  SetDlgItemText16((uStack6 + 0xe),0x113,HVar6);
  if (iVar1 != 0x0) {
    uVar5 = pass1_1018_1f1a((iVar9 + 0x8),(uStack6 + 0x1a));
    if (uVar5 != 0x0) {
      uVar12 = 0x11;
      uVar13 = 0x1;
      uVar3 = (uStack6 + 0x16);
      uVar7 = uVar3;
      uVar8 = (uVar3 >> 0x10);
      goto LAB_1020_3846;
    }
  }
  uVar12 = 0x11;
  uVar13 = 0x1;
  pcVar11 = load_string_1010_847e(_u16_1050_14cc,0x5d9);
  uVar8 = (pcVar11 >> 0x10);
  uVar7 = SUB42(pcVar11,0x0);//
LAB_1020_3846:
  SetDlgItemText16(CONCAT22(uVar8,uVar7),CONCAT11(uVar13,uVar12),HVar6);
  if (*(i32 *)(iVar9 + 0x1c) != 0x0) {
    uVar3 = (iVar9 + 0x1c);
    HVar6 = GetDlgItem16(*(INT16 *)(uStack6 + 0x1a),*(HWND16 *)(uVar3 + 0x6));
    SetFocus16(HVar6);
    return;
  }
  InvalidateRect16(0x0,NULL,0x0);
  return;
}
pub fn pass1_1020_3898(param_1: *mut astruct_30)

{
  destroy_window_1020_3b3e(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn window_op_1020_38aa(param_1: *mut astruct_57,param_2: *mut StructA,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,
                        mut param_6: u16 ,mut param_7: u16 ,mut param_8: u16 )

{
  let mut hwnd: HWND16;
  code **ppcVar1;
  let mut uVar2: u32;
  astruct_126 *paVar3;
  let mut paVar4: *mut Struct57;
  let mut uVar5: u32;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut paVar9: *mut Struct57;
  let mut uVar11: u32;
  let mut unaff_SI: u16;
  let mut uVar12: u16;
  let mut puVar13: *mut u32;
  let mut in_stack_0000fe08: u16;
  let mut in_stack_0000fe0c: u16;
  let mut in_stack_0000ff32: u16;
  let mut in_stack_0000ff36: u16;
  let mut in_stack_0000ff3a: u16;
  let mut uVar14: u16;
  let mut local_12: i16;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut iStack12: i16;
  i16 local_a [0x2];
  let mut iStack6: i16;
  let mut iStack4: i16;
  let pstructa_hi: *mut StructA;
  let pstructa_1: *mut StructA;
  let mut paVar10: *mut Struct57;

  pstructa_1 = (StructA *)param_2;
  uVar14 = (param_2 >> 0x10);
  create_window_ex_1008_9760(param_2);
  puVar13 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x6),param_7,param_8,param_5,param_6)
  ;
  paVar9 = (astruct_57 *)(param_1 & 0xffff0000);
  pstructa_1[0x1].field25_0x2e = (i16)puVar13;
  iVar6 = (i16)(puVar13 >> 0x10);
  pstructa_1[0x1].field26_0x30 = iVar6;
  pstructa_1[0x1].field10_0x14 = pstructa_1[0x1].field25_0x2e;
  pstructa_1[0x1].field11_0x16 = iVar6;
  if (param_2 == NULL) {
    paVar3 = NULL;
  }
  else {
    paVar9 = (astruct_57 *)(paVar9 | uVar14);
    paVar3 = (astruct_126 *)&pstructa_1[0x1].field20_0x26;
  }
  ppcVar1 = (code **)(*(u32*)&pstructa_1[0x1].field25_0x2e + 0x4);
  (**ppcVar1)(0x1010,&pstructa_1[0x1].field25_0x2e,0x0,paVar3,paVar9);
  pass1_1018_1a8e(paVar9,*(astruct_653 **)&pstructa_1[0x1].field25_0x2e);
  mem_op_1000_179c(0x20,paVar9);
  uVar7 = paVar9 | paVar3;
  paVar10 = (astruct_57 *)(paVar9 & 0xffff0000 | uVar7);
  if (uVar7 == 0x0) {
    &pstructa_1[0x1].field22_0x2a = 0x0;
  }
  else {
    unk_draw_op_1020_3da4(uVar7,param_3,(astruct_24 *)CONCAT22(paVar9,paVar3),param_2);
    pstructa_1[0x1].field22_0x2a = (astruct_160 *)paVar3;
    &pstructa_1[0x1].field_0x2c = paVar10;
  }
  uVar5 = &pstructa_1[0x1].field22_0x2a;
  &pstructa_1[0x1].field14_0x1c = uVar5;
  mem_op_1000_179c(0x42,paVar10);
  paVar4 = (astruct_57 *)uVar5;
  uVar7 = (astruct_57 *)paVar10 | paVar4;
  paVar9 = (astruct_57 *)(paVar10 & 0xffff0000 | uVar7);
  if (uVar7 == 0x0) {
    &pstructa_1[0x1].field_0x36 = 0x0;
  }
  else {
    pass1_1008_3bd6(paVar9,paVar4,(astruct_57 *)paVar10,0x0,0xf1,0x0,0x1ac01ad,
                    CONCAT22(pstructa_1.field4_0x8,0xf4),param_4,in_stack_0000fe08,in_stack_0000fe0c,in_stack_0000ff32,
                    in_stack_0000ff36,in_stack_0000ff3a);
    *(astruct_57 **)&pstructa_1[0x1].field_0x36 = paVar4;
    pstructa_1[0x1].field32_0x38 = paVar9;
  }
  uVar12 = 0x1000;
  mem_op_1000_179c(0x42,paVar9);
  uVar7 = (astruct_57 *)paVar9 | paVar4;
  uVar11 = paVar9 & 0xffff0000 | uVar7;
  if (uVar7 == 0x0) {
    &pstructa_1[0x1].field_0x3a = 0x0;
  }
  else {
    uVar12 = 0x1008;
    pass1_1008_3bd6(uVar11,paVar4,(astruct_57 *)paVar9,0x0,0xf500f1,0x0,0x1ae01af,CONCAT22(pstructa_1.field4_0x8,0xf5),
                    param_4,in_stack_0000fe08,in_stack_0000fe0c,in_stack_0000ff32,in_stack_0000ff36,in_stack_0000ff3a);
    *(astruct_57 **)&pstructa_1[0x1].field_0x3a = paVar4;
    &pstructa_1[0x1].field_0x3c = uVar11;
  }
  uVar5 = &pstructa_1[0x1].field25_0x2e;
  ppcVar1 = (code **)(*(u32*)&pstructa_1[0x1].field25_0x2e + 0x10);
  (**ppcVar1)(uVar12,(char)uVar5,(uVar5 >> 0x10));
  uVar12 = uVar11;
  uVar7 = paVar4.field1_0x2;
  paVar9 = (astruct_57 *)(uVar11 & 0xffff0000 | uVar7);
  uVar7 = MoveWindow16(0x1,paVar4.field3_0x6,paVar4.field2_0x4,uVar7,paVar4.field0_0x0,pstructa_1.field4_0x8);
  uVar12 = 0x1000;
  mem_op_1000_179c(0x8e,paVar9);
  uVar8 = paVar9 | uVar7;
  if (uVar8 == 0x0) {
    pstructa_1[0x1].field37_0x3e = 0x0;
  }
  else {
    uVar12 = SUB42(&PTR_LOOP_1050_1040,0x0);
    get_sys_metrics_1040_7728((astruct_57 *)CONCAT22(paVar9,uVar7),0x1,0x0,0xfc0,pstructa_1.field4_0x8);
    &pstructa_1[0x1].field37_0x3e = uVar7;
    (&pstructa_1[0x1].field37_0x3e + 0x2) = uVar8;
  }
  uVar2 = pstructa_1[0x1].field37_0x3e;
  (uVar2 + 0x74) = 0x1;
  uVar2 = pstructa_1[0x1].field37_0x3e;
  ppcVar1 = (code **)(pstructa_1[0x1].field37_0x3e + 0x8);
  (**ppcVar1)(uVar12,(char)uVar2,(uVar2 >> 0x10));
  if (((&pstructa_1[0x1].field37_0x3e + 0x2) | &pstructa_1[0x1].field37_0x3e) != 0x0) {
    uVar2 = pstructa_1[0x1].field37_0x3e;
    hwnd = *(HWND16 *)(uVar2 + 0x6);
    GetWindowRect16((RECT16 *)CONCAT13(0x10,CONCAT12(0x50,local_a)),pstructa_1.field4_0x8);
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_12),hwnd);
    iStack12 -= iStack16;
    iStack14 = iStack6 - local_a[0];
    local_12 = local_a[0];
    iStack16 = iStack4 + 0x3;
    SetWindowPos16(0x44,iStack12,iStack14,iStack16,local_a[0],0x0,hwnd);
  }
  return;
}
pub fn destroy_window_1020_3b3e(param_1: *mut astruct_30)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  i32 lVar4;
  let mut puVar5: *mut u8,
  astruct_30 *paVar6;
  astruct_30 *struct_5;
  astruct_30 *struct_6;
  let mut unaff_CS: u16;

  struct_6 = (astruct_30 *)(param_1 >> 0x10);
  struct_5 = (astruct_30 *)param_1;
  struct_5.field262_0x10e = 0x0;
  if (struct_5.field261_0x10a != 0x0) {
    lVar4 = struct_5.field261_0x10a;
    // 0x1538
    unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
    DestroyWindow16(*(HWND16 *)(lVar4 + 0x6));
  }
  puVar1 = struct_5.field246_0xf6;
  uVar2 = struct_5.field247_0xf8;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(unaff_CS,puVar1);
  }
  &struct_5.field246_0xf6 = 0x0;
  if (struct_5.field248_0xfa != 0x0) {
    puVar5 = (struct_6 | struct_5);
    if (param_1 == NULL) {
      paVar6 = NULL;
    }
    else {
      puVar5 = &struct_5.field_0xf2;
      paVar6 = struct_6;
    }
    pass1_1010_1ea6(struct_5.field248_0xfa,(StructD *)CONCAT22(paVar6,puVar5));
  }
  struct_5.field248_0xfa = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_3bd6(mut param_1: u16 ,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;

  uVar3 = (param_2 >> 0x10);
  uVar2 = param_2;
  mixed_draw_op_1020_3fa0((uVar2 + 0xf6));
  if ((uVar2 + 0x100) == 0x0) {
    (uVar2 + 0x100) = 0x1;
    uVar4 = (uVar2 + 0xfa);
    if ((uVar4 + 0x56) == 0x0) {
      iVar1 = 0x5;
    }
    else {
      iVar1 = 0x8;
    }
    uVar4 = pass1_1038_af40(uVar2,param_1,_PTR_LOOP_1050_5b7c,(uVar2 + 0x8),iVar1);
    (uVar2 + 0x10e) = uVar4;
    (uVar2 + 0x110) = (uVar4 >> 0x10);
  }
  return;
}
pub fn pass1_1020_3c32(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 )

{
  char cVar1;
  let mut iVar2: i16;

  if (param_3 == 0xf5) {
    iVar2 = 0x1;//
LAB_1020_3c52:
    pass1_1018_1b02(*(astruct_95 **)(param_1 + 0xfa),iVar2);
    return;
  }
  if ((param_3 < 0xf6) && (cVar1 = (char)param_3, cVar1 != '\0')) {
    if (cVar1 == '\x01' || cVar1 == '\x02') {
      return;
    }
    if (cVar1 == -0xc) {
      iVar2 = 0x0;
      goto LAB_1020_3c52;
    }
  }
  pass1_1020_3c32(param_1,param_2,param_3);
  return;
}
pub fn pass1_1020_3c74(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 )

{
  pass1_1020_3c8c(CONCAT22(param_2,param_1),CONCAT22(param_3,(param_2 >> 0x10)));
  return;
}
pub fn pass1_1020_3c8c(mut param_1: u32,mut param_2: u32)

{
  pt_in_rect_1018_1bda((param_1 + 0xfa),param_2,(param_2 >> 0x10));
  return;
}



StructD * pass1_1020_3ca6(StructD *param_1,param_2: u8)

{
  let mut uVar1: u32;
  StructD *puStack10;

  uVar1 = param_1 & 0xffff0000;
  param_1 = (StructD *)(uVar1 | param_1 - 0xf2);
  param_1 = (uVar1 >> 0x10);
  if (param_1 == NULL) {
    param_1 = 0x0;
    param_1 = 0x0;
  }
  puStack10 = (StructD *)CONCAT22(param_1,param_1);
  puStack10.address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  cleanup_menu_ui_op_1020_795c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn FUN_1020_3cb4(void)

{
  return;
}



StructD * FUN_1020_3cb8(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  let mut iVar1: i16;
  let mut puStack10: *mut u16;

  if (param_2 == NULL) {
    iVar1 = 0x0;
    param_2 = 0x0;
  }
  else {
    iVar1 = param_2 + 0xf2;
  }
  puStack10 = CONCAT22(param_2,iVar1);
  *puStack10 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  cleanup_menu_ui_op_1020_795c(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



// WARNING: Instruction at (ram,0x10203dab) overlaps instruction at (ram,0x10203da8)
//
// WARNING: Variable defined which should be unmapped: param_16
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_3d08(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 ,
                    mut param_7: u16 ,mut param_8: u16 ,mut param_9: u16 ,uchar param_10,uchar param_11,uchar param_12,
                    uchar param_13,uchar param_14,mut param_15: u32,mut param_16: u16 ,mut param_17: u16 ,mut param_18: u16 ,
                    mut param_19: u16 )

{
  let mut pcVar1: *mut c_char;
  let mut pbVar2: *mut u8,
  let mut bVar3: bool;
  let mut bVar4: bool;
  code **ppcVar5;
  let mut puVar6: *mut u16;
  let mut uVar7: u32;
  let mut puVar8: *mut u32;
  HDC16 hdc;
  let pSVar9: *mut StructA;
  u8 bVar10;
  u8 bVar12;
  let mut iVar13: i16;
  u8 bVar17;
  char cVar18;
  HDC16 hdc_00;
  int16_t iVar14;
  HGDIOBJ16 HVar15;
  let mut puVar16: *mut u8,
  let mut uVar19: u16;
  u8 bVar20;
  u8 bVar21;
  let mut uVar22: u16;
  let mut in_register_0000000a: u16;
  u8 bVar24;
  let mut pcVar25: *mut c_char;
  let mut uVar26: u16;
  let mut uVar27: u16;
  let mut bVar28: bool;
  let mut bVar29: bool;
  let mut puVar30: *mut u32;
  let mut in_stack_0000fe8a: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffb8: u16;
  let mut uStack30: u16;
  code *pcStack4;
  u8 bVar11;
  let mut paVar23: *mut Struct57;

  pSVar9 = (StructA *)CONCAT22(param_19,param_18);
  bVar20 = (char)param_2 + (char)(param_1 >> 0x8) + param_10;
  *param_6 = 0x3c;
  paVar23 = (astruct_57 *)
            CONCAT22(in_register_0000000a,
                     CONCAT11((char)(param_2 >> 0x8) + '<' + (*(param_3 + param_5) < 0x20),bVar20));
  pcStack4 = caseD_a7;
  iVar13 = 0x203d;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar20;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 & bVar20;
  pcVar25 = CONCAT11(0x79,(char)param_5 + -0x37);
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar20;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar20;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar20;
  bVar10 = (u8)(param_6 + 0x2);
  bVar12 = 0x9 < (bVar10 & 0xf) | param_11;
  bVar11 = bVar10 + bVar12 * '\x06' & 0xf;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar20;
  bVar10 = 0x9 < bVar11 | bVar12;
  uVar19 = CONCAT11((char)(param_6 + 0x2 >> 0x8) + bVar12 + bVar10,bVar11 + bVar10 * '\x06') & 0xff0f;
  do {
    pbVar2 = (param_3 + iVar13);
    bVar21 = (u8)paVar23;
    *pbVar2 = *pbVar2 | bVar21;
    bVar12 = (u8)(uVar19 - 0x1);
    bVar3 = 0x9 < (bVar12 & 0xf);
    bVar20 = bVar3 | bVar10;
    pbVar2 = (param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar21;
    bVar4 = 0x9 < (bVar12 + bVar20 * '\x06' & 0xf);
    bVar17 = (char)(uVar19 - 0x1 >> 0x8) + bVar20 + (bVar4 | bVar20);
    pbVar2 = (param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar21;
    uVar19 = 0x0;
    bVar28 = &pcStack4 < (param_3 + iVar13);
    pbVar2 = (param_3 + iVar13 + 0x896);
    bVar24 = (u8)param_3;
    bVar29 = CARRY1(*pbVar2,bVar24) || CARRY1(*pbVar2 + bVar24,bVar28);
    *pbVar2 = *pbVar2 + bVar24 + bVar28;
    pbVar2 = (param_3 + iVar13 + 0x2038);
    bVar12 = *pbVar2;
    bVar11 = *pbVar2;
    *pbVar2 = bVar11 + bVar24 + bVar29;
    pcVar1 = (param_4 + iVar13);
    *pcVar1 = *pcVar1 + (char)(paVar23 >> 0x8) + (CARRY1(bVar12,bVar24) || CARRY1(bVar11 + bVar24,bVar29));
    pcVar1 = (param_3 + iVar13 + -0x64);
    *pcVar1 = *pcVar1 + bVar17 + '\x01';
    pbVar2 = (param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar21;
    pcVar1 = pcVar25;
    pcVar25 = pcVar25 + 0x1;
    bVar28 = *pcVar1 != '\0';
    if (-*pcVar1 < '\0') {
      pcVar1 = (param_4 + 0x37);
      *pcVar1 = *pcVar1 + bVar24 + bVar28;
      pbVar2 = (param_3 + iVar13);
      *pbVar2 = *pbVar2 | bVar21;
      iVar13 += -0x1;
      pbVar2 = (param_3 + iVar13);
      *pbVar2 = *pbVar2 | bVar21;
      pbVar2 = (param_3 + iVar13);
      *pbVar2 = *pbVar2 | bVar21;
      uVar22 = paVar23 - 0x1;
      paVar23 = (astruct_57 *)(paVar23 & 0xffff0000 | uVar22);
      pbVar2 = (param_3 + iVar13);
      bVar12 = (u8)uVar22;
      *pbVar2 = *pbVar2 | bVar12;
      if (*pbVar2 == 0x0) {
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 & bVar12;
code_r0x10203d96:
        pbVar2 = (param_3 + iVar13);
        bVar12 = (u8)paVar23;
        *pbVar2 = *pbVar2 | bVar12;
        pbVar2 = (param_3 + iVar13);
        *pbVar2 = *pbVar2 & bVar12;
        paVar23 = (astruct_57 *)
                  (paVar23 & 0xffff0000 |
                  CONCAT11((char)(paVar23 >> 0x8) * '\x02' + ((u8)uVar19 < 0x20),bVar12));
        pbVar2 = (param_3 + iVar13 + 0x1);
        *pbVar2 = *pbVar2 & bVar12;
        param_4 = &stack0xfff6;
        param_16 = pcStack4;
        param_17 = (pcStack4 >> 0x10);
        pSVar9 = (StructA *)param_15;
code_r0x10203db1:
        get_sys_metrics_1020_7c1a((astruct_40 *)CONCAT22(param_17,param_16),pSVar9);
        puVar6 = (param_4 + 0x6);
        uVar26 = (puVar6 >> 0x10);
        (puVar6 + 0x14) = 0x0;
        *puVar6 = 0x408a;
        (puVar6 + 0x2) = 0x1020;
        puVar30 = mixed_1010_20ba(paVar23,_u16_1050_0ed0,(u8 **)CONCAT22(uStack30,0x6),in_stack_0000fe8a,
                                  in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
        hdc = (HDC16)(puVar30 >> 0x10);
        uVar7 = (param_4 + 0x6);
        uVar26 = (uVar7 >> 0x10);
        iVar13 = uVar7;
        (iVar13 + 0x14) = puVar30;
        *(HDC16 *)(iVar13 + 0x16) = hdc;
        ppcVar5 = (code **)((iVar13 + 0x14) + 0x4);
        (**ppcVar5)(0x1010,(iVar13 + 0x14),hdc,0x0,iVar13,uVar26);
        uVar7 = (param_4 + 0x6);
        hdc_00 = GetDC16(*(HWND16 *)(uVar7 + 0x4));
        *(HDC16 *)(param_4 + -0x2) = hdc_00;
        iVar14 = SetMapMode16(0x1,hdc_00);
        uVar7 = (param_4 + 0x6);
        *(int16_t *)(uVar7 + 0x1e) = iVar14;
        HVar15 = GetStockObject16(0x0);
        HVar15 = SelectObject16(HVar15,hdc);
        uVar7 = (param_4 + 0x6);
        *(HGDIOBJ16 *)(uVar7 + 0x18) = HVar15;
        HVar15 = GetStockObject16(0x6);
        HVar15 = SelectObject16(HVar15,0x0);
        uVar7 = (param_4 + 0x6);
        *(HGDIOBJ16 *)(uVar7 + 0x1a) = HVar15;
        uVar7 = (param_4 + 0x6);
        uVar7 = (uVar7 + 0x14);
        (param_4 + -0x6) = (uVar7 + 0x24);
        puVar16 = (param_4 + -0x2);
        puVar8 = (param_4 + -0x6);
        ppcVar5 = (code **)(*puVar8 + 0x8);
        (**ppcVar5)(s_tile2_bmp_1050_1538,puVar8,(puVar8 >> 0x10),puVar16,&DAT_1050_1050);
        uVar7 = (param_4 + 0x6);
        uVar27 = (uVar7 >> 0x10);
        iVar13 = uVar7;
        (iVar13 + 0x1c) = puVar16;
        uVar26 = (param_4 + -0x2);
        (param_4 + -0x14) = uVar26;
        uVar7 = (iVar13 + 0x14);
        (uVar7 + 0x4c) = uVar26;
        return;
      }
      pbVar2 = (param_3 + iVar13);
      *pbVar2 = *pbVar2 & bVar12;
      pcVar1 = (param_4 + iVar13 + 0x20);
      bVar11 = (u8)param_1 & 0x1f;
      cVar18 = *pcVar1;
      *pcVar1 = *pcVar1 >> bVar11;
      pcVar1 = (param_4 + iVar13 + 0x6a);
      *pcVar1 = *pcVar1 + (u8)param_1 + ((param_1 & 0x1f) != 0x0 && (cVar18 >> bVar11 - 0x1 & 0x1U) != 0x0);
      pbVar2 = (param_3 + iVar13);
      *pbVar2 = *pbVar2 | bVar12;
      uVar19 = (param_3 + iVar13) * 0x10;
      pbVar2 = (pcVar25 + param_4 + 0x8);
      bVar12 = (u8)(uVar19 >> 0x8);
      uVar22 = uVar19 & 0xff | (u8)(bVar12 + *pbVar2) << 0x8;
      pcVar1 = (param_4 + iVar13 + 0x37);
      *pcVar1 = *pcVar1 + (char)(param_3 >> 0x8) + CARRY1(bVar12,*pbVar2);
    }
    else {
      pcVar1 = (param_4 + iVar13);
      *pcVar1 = *pcVar1 + bVar28;
      uVar22 = (param_3 + iVar13) * 0x10;
      if ((POPCOUNT(*pcVar1) & 0x1U) == 0x0) goto code_r0x10203db1;
    }
    pbVar2 = (param_3 + iVar13);
    bVar11 = (u8)paVar23;
    *pbVar2 = *pbVar2 | bVar11;
    pbVar2 = (param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar11;
    param_16 = (param_14 & 0x1) * 0x4000 | (param_13 & 0x1) * 0x200 | (param_12 & 0x1) * 0x100 |
               ((char)*pbVar2 < '\0') * 0x80 | (*pbVar2 == 0x0) * 0x40 |
               (u8)(bVar4 | bVar3 | bVar10 & 0x1) * 0x10 | ((POPCOUNT(*pbVar2) & 0x1U) == 0x0) * 0x4;
    pbVar2 = (param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar11;
    bVar12 = in(0x79);
    pbVar2 = (param_3 + iVar13);
    *pbVar2 = *pbVar2 & bVar11;
    if (-0x1 < (char)*pbVar2) {
      pSVar9 = (StructA *)CONCAT22(param_19,param_18);
      if ((bVar17 | *(param_4 - 0x1)) == 0x0) {
        param_16 = param_7;
        pSVar9 = (StructA *)CONCAT22(param_19,param_18);
      }
      goto code_r0x10203db1;
    }
    pbVar2 = (param_4 + 0x89c);
    bVar28 = CARRY1(*pbVar2,bVar12);
    *pbVar2 = *pbVar2 + bVar12;
    bVar21 = bVar17 + bVar12;
    cVar18 = bVar21 + bVar28;
    uVar19 = CONCAT11(cVar18,bVar12);
    pcStack4._0_3_ =
         CONCAT21((param_14 & 0x1) * 0x4000 | (SCARRY1(bVar17,bVar12) != SCARRY1(bVar21,bVar28)) * 0x800 |
                  (param_13 & 0x1) * 0x200 | (param_12 & 0x1) * 0x100 | (cVar18 < '\0') * 0x80 |
                  (cVar18 == '\0') * 0x40 | (u8)(bVar4 | bVar3 | bVar10 & 0x1) * 0x10 |
                  ((POPCOUNT(cVar18) & 0x1U) == 0x0) * 0x4 |
                  (CARRY1(bVar17,bVar12) || CARRY1(bVar21,bVar28)),pcStack4._0_1_);
    pcStack4 = (code *)(pcStack4 & 0xff000000 | (u16)pcStack4);
    pbVar2 = (param_3 + iVar13);
    *pbVar2 = *pbVar2 | bVar11;
    param_1 = uVar22 - 0x1;
    bVar10 = bVar4 | bVar20;
    if (param_1 == 0x0 || *pbVar2 == 0x0) goto code_r0x10203d96;
  } while( true );
}



// WARNING: Unable to use type for symbol uVar4
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_draw_op_1020_3da4(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_24,param_4: *mut StructA)

{
  let mut puVar2: *mut u32;
  let mut pUVar3: *mut u32;
  let mut i16: i16;
  HGDIOBJ16 white_pen_handle;
  HDC16 *pHVar4;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  astruct_24 *pstruct24_1;
  let mut pstruct_24_hi: u16;
  let mut puVar6: *mut u32;
  let mut in_stack_0000fe92: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc0: u16;
  HDC16 hdc;
  HDC16 hdc_00;
  HDC16 hdc_4;
  astruct_24 *iVar9;
  astruct_24 *uVar8;
  let mut puVar1: *mut u32;
  let mut uVar4: *mut u32;
  let mut in_stack_0000ffea: u16;
  code **fn_ptr_1;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1020_7c1a((astruct_40 *)param_3,param_4);
  pstruct_24_hi = (param_3 >> 0x10);
  pstruct24_1 = (astruct_24 *)param_3;
  pstruct24_1.field17_0x14 = NULL;
  param_3.offset_0x0 = 0x408a;
  pstruct24_1.base_0x2 = 0x1020;
  puVar6 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x6),in_stack_0000fe92,in_stack_0000ffb6
                           ,in_stack_0000ffbc,in_stack_0000ffc0);
  hdc = (HDC16)(puVar6 >> 0x10);
  &pstruct24_1.field17_0x14 = puVar6;
  *(HDC16 *)(&pstruct24_1.field17_0x14 + 0x2) = hdc;
  hdc_00 = 0x0;
  fn_ptr_1 = (code **)(*pstruct24_1.field17_0x14 + 0x4);
  (**fn_ptr_1)(0x1010,&pstruct24_1.field17_0x14,hdc,0x0,param_3);
  hdc_4 = GetDC16(pstruct24_1.field2_0x4);
  i16 = SetMapMode16(0x1,hdc_4);
  pstruct24_1.field21_0x1e = i16;
  white_pen_handle = GetStockObject16(WHITE_BRUSH);
  white_pen_handle = SelectObject16(white_pen_handle,hdc);
  pstruct24_1.field18_0x18 = white_pen_handle;
  white_pen_handle = GetStockObject16(WHITE_PEN);
  white_pen_handle = SelectObject16(white_pen_handle,hdc_00);
  pstruct24_1.obj_handle_0x1a = white_pen_handle;
  uVar4 = pstruct24_1.field17_0x14;
  puVar2 = (uVar4 + 0x24);
  pHVar4 = &hdc_4;
    // just 0x1538
  fn_ptr_1 = (code **)(*puVar2 + 0x8);
  (**fn_ptr_1)(s_tile2_bmp_1050_1538,puVar2,(puVar2 >> 0x10),pHVar4,&DAT_1050_1050);
  pstruct24_1.field20_0x1c = pHVar4;
  pUVar3 = pstruct24_1.field17_0x14;
  *(HDC16 *)(pUVar3 + 0x4c) = hdc_4;
  return;
}



// WARNING: Unable to use type for symbol uVar2
pub fn win_ui_palette_op_1020_3e84(StructD *param_1)

{
  HDC16 hdc;
  HPALETTE16 obj;
  StructD *struct_v1;
  let mut uVar1: u16;
  let mut unaff_SS: u16;
  i32 uVar2;

  uVar1 = (param_1 >> 0x10);
  struct_v1 = (StructD *)param_1;
  param_1.address_offset_field_0x0 = 0x408a;
  struct_v1.address_offset_field_0x2 = 0x1020;
  pass1_1010_1ea6(struct_v1->field12_0x14,(StructD *)(param_1 & 0xffff | uVar1 << 0x10));
  uVar2 = struct_v1->field12_0x14;
  hdc = *(HDC16 *)(uVar2 + 0x4c);
  SelectObject16(struct_v1->field13_0x18,hdc);
  SelectObject16(struct_v1->field14_0x1a,hdc);
  obj = SelectPalette16(0x0,*(HPALETTE16 *)&struct_v1->field_0x1c,hdc);
  DeleteObject16(obj);
  SetMapMode16(*(int16_t *)&struct_v1->field_0x1e,hdc);
  param_1->address_offset_field_0x0 = 0x3ab0;
  struct_v1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  struct_v1->address_offset_field_0x2 = 0x1008;
  return;
}
pub fn validate_rect_1020_3f12(mut param_1: u32,mut param_2: i16)

{
  let mut local_a: RECT16;
  let mut uStack6: u32;

  if (param_2 == 0x1) {
    (param_1 + 0x14) = 0x0;
    return;
  }
  if (param_2 != 0xd) {
    return;
  }
  local_a = (RECT16)0x8000e;
  uStack6 = 0x1100116;
  InvalidateRect16(0x0,&local_a,(HWND16)&DAT_1050_1050);
  local_a = (RECT16)0xf10000;
  uStack6 = 0x1220030;
  ValidateRect16(&local_a,(HWND16)&DAT_1050_1050);
  local_a = (RECT16)0xf100f5;
  uStack6 = 0x1220127;
  ValidateRect16(&local_a,(HWND16)&DAT_1050_1050);
  return;
}
pub fn mixed_draw_op_1020_3fa0(mut param_1: u32)

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut iStack56: i16;
  let mut uStack54: u32;
  let mut local_32: u32;
  let mut iStack46: i16;
  let mut uStack44: u32;
  let mut puStack40: *mut u32;
  HDC16 local_24;
  u8 local_22 [0x20];

  uVar6 = (param_1 >> 0x10);
  iVar4 = param_1;
  BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_22),*(HWND16 *)(iVar4 + 0x4));
  uVar3 = (iVar4 + 0x14);
  local_24 = *(HDC16 *)(uVar3 + 0x4c);
  uVar3 = (iVar4 + 0x14);
  puStack40 = (uVar3 + 0x24);
  uVar5 = puStack40;
  ppcVar2 = (code **)(*puStack40 + 0x4);
  (**ppcVar2)(s_tile2_bmp_1050_1538,uVar5,(puStack40 >> 0x10),0x0,&local_24,&DAT_1050_1050);
  uVar3 = (iVar4 + 0x14);
  iStack46 = (uVar3 + 0x44);
  uVar3 = (iVar4 + 0x14);
  uStack44 = (uVar3 + 0x40);
  uVar1 = (iVar4 + 0x14);
  pass1_1008_3e94((uVar1 & 0xffff0000 | (uVar1 + 0x3a)),CONCAT22(0x1050,&local_32),
                  CONCAT22(0x1050,&local_32 + 0x2));
  uStack54 = uStack44;
  for (iStack56 = 0x0; iStack56 < iStack46; iStack56 += 0x1) {
    draw_rect_1020_40ce(uStack54,(i16)local_32,(i16)(local_32 >> 0x10),local_24,uVar5);
    uStack54 = uStack54 & 0xffff0000 | (uStack54 + 0x18);
  }
  EndPaint16((PAINTSTRUCT16 *)CONCAT13(0x10,CONCAT12(0x50,local_22)),*(HWND16 *)(iVar4 + 0x4));
  return;
}
