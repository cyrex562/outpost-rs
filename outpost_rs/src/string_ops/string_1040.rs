use crate::defines::Struct18;

pub fn string_1040_8520(
    param_1: &mut Struct18,
    param_2: u16,
    param_3: u16,
    param_4: i16,
    param_5: u16,
    param_6: u16,
    uparam_7: &mut String,
    param_8: u16,
) -> i16 {
    let UVar1: i32;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let iVar5: &mut Struct293;
    let u_var5: u16;
    let u_var6: u16;
    let mut pcVar7: String;
    let uStack32: u32;
    let uStack28: u32;
    let iStack22: i16;
    let uStack20: u16;
    let uStack18: u16;
    let iStack16: i16;
    let iStack14: i16;
    let local_a: u32;
    let uStack6: u32;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, 0xfc3, param_2);
    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    iVar5.field_0x8e = 0x0;
    iVar5.field_0x98 = param_3;
    iVar5.field_0x9a = 0x0;
    iVar5.field_0xb2 = 0x0;
    param_1 = 0x8ddc;
    iVar5.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    local_a = 0x0;
    uStack6 = 0x12c;
    iVar5.field_0x9e = 0x0;
    iVar5.field_0xa2 = 0x12c;
    _iStack14 = CONCAT22(param_8, &param_5);
    iStack16 = param_4;
    if (param_4 != 0x0) {
        _iStack14 = CONCAT22(param_8, &param_6);
        uStack18 = param_5;
        u_var4 = param_5;
        load_string_1010_84ac(
            ctx.PTR_LOOP_1050_14cc,
            (ctx.PTR_LOOP_1050_14cc >> 0x10),
            0x1010,
        );
        iVar5.field_0x94 = u_var4;
        iVar5.field_0x96 = param_7;
        iStack16 += -0x1;
    }
    iStack22 = 0x0;
    while (pu_var2 = _iStack14, iStack16 != 0x0) {
        _iStack14 = (_iStack14 & 0xffff0000 | (iStack14 + 0x2));
        uStack20 = *pu_var2;
        iStack16 = iStack16 + -0x1;
        pcVar7 = load_string_1010_847e(
            ctx.PTR_LOOP_1050_14cc,
            (ctx.PTR_LOOP_1050_14cc >> 0x10),
            0x1010,
        );
        // param_7 = (pcVar7 >> 0x10);
        uStack28 = pcVar7;
        u_var3 = str_op_1000_3da4(pcVar7);
        iStack22 += u_var3;
    }
    u_var4 = iStack22 + 0x1;
    u_var6 = 0x1000;
    iStack16 = iStack16 + -0x1;
    mem_op_1000_179c(u_var4, param_7, 0x1000);
    &iVar5.field_0x90 = u_var4;
    (&iVar5.field_0x90 + 0x2) = param_7;
    _iStack14 = CONCAT22(param_8, &param_6);
    iStack16 = param_4 + -0x1;
    if (iStack16 != 0x0) {
        _iStack14 = CONCAT22(param_8, &stack0x0012);
        uStack20 = param_6;
        UVar1 = iVar5.field_0x90;
        u_var6 = 0x1010;
        load_string_1010_84e0(
            0x1010,
            ctx.PTR_LOOP_1050_14cc,
            (ctx.PTR_LOOP_1050_14cc >> 0x10),
            0x3ff,
            UVar1,
            (UVar1 >> 0x10),
        );
        iStack16 += -0x1;
    }
    while (pu_var2 = _iStack14, iStack16 != 0x0) {
        _iStack14 = (_iStack14 & 0xffff0000 | (iStack14 + 0x2));
        uStack20 = *pu_var2;
        iStack16 = iStack16 + -0x1;
        pcVar7 = load_string_1010_847e(
            ctx.PTR_LOOP_1050_14cc,
            (ctx.PTR_LOOP_1050_14cc >> 0x10),
            0x1010,
        );
        u_var6 = 0x1000;
        uStack32 = pcVar7;
        pass1_1000_3cea(iVar5.field_0x90, pcVar7);
    }
    iStack16 = iStack16 + -0x1;
    load_icon_1040_8b92(param_1, u_var6);
    ctx.PTR_LOOP_1050_5df8 = 0x0;
    return iVar5;
}

pub fn string_1040_a626(param_1: U32Ptr, param_2: &mut String, param_3: u16) {
    let u_var1: u16;

    u_var1 = str_op_1008_60e8(param_2, param_3);
    *param_1 = u_var1;
    (param_1 + 0x2) = param_3;
    return;
}
