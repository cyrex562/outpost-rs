// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn mixed_struct_op_1040_8fb8(
    param_1: U32Ptr,
    param_2: u16,
    param_3: &mut String,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: u16,
    param_9: u16,
    param_10: U32Ptr,
    param_11: u16,
) {
    let u_var1: u16;
    let u_var2: u16;
    let pvVar3: U32Ptr;
    let i_var4: i16;
    let u_var5: u16;
    let paVar6: &mut Struct43;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x389a;
    (i_var4 + 0x2) = 0x1008;
    (i_var4 + 0x4) = 0x0;
    (i_var4 + 0x8) = 0x0;
    (i_var4 + 0xc) = 0x0;
    (i_var4 + 0x10) = 0x0;
    (i_var4 + 0x14) = 0x0;
    (i_var4 + 0x18) = 0x0;
    (i_var4 + 0x1a) = param_8;
    (i_var4 + 0x1c) = param_7;
    (i_var4 + 0x36) = 0x5;
    (i_var4 + 0x38) = 0x0;
    (i_var4 + 0x3a) = 0x0;
    (i_var4 + 0x3c) = 0x2;
    (i_var4 + 0x3e) = 0x0;
    (i_var4 + 0x40) = param_2;
    *param_1 = 0x9800;
    (i_var4 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    u_var1 = (i_var4 + 0x36);
    (i_var4 + 0x28) = u_var1;
    (i_var4 + 0x26) = u_var1;
    (i_var4 + 0x2c) = 0x0;
    (i_var4 + 0x2a) = 0x0;
    if ((param_6 != 0x0) && (param_5 != 0x0)) {
        (i_var4 + 0x38) = 0x1;
        paVar6 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, param_6, param_11);
        (i_var4 + 0x8) = paVar6;
        (i_var4 + 0xa) = (paVar6 >> 0x10);
        param_10 = 0x1010;
        paVar6 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, param_5, param_11);
        // param_9 = (paVar6 >> 0x10);
        (i_var4 + 0xc) = paVar6;
        (i_var4 + 0xe) = param_9;
        if (param_4 == 0x0) {
            (i_var4 + 0x10) = 0x0;
        } else {
            param_10 = 0x1010;
            paVar6 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, param_4, param_11);
            // param_9 = (paVar6 >> 0x10);
            (i_var4 + 0x10) = paVar6;
            (i_var4 + 0x12) = param_9;
        }
    }
    u_var1 = (i_var4 + 0x36);
    (i_var4 + 0x30) = u_var1;
    (i_var4 + 0x2e) = u_var1;
    (i_var4 + 0x32) = 0x0;
    if (param_3 != 0x0) {
        param_10 = 0x1008;
        u_var2 = str_op_1008_60e8(param_3, param_9);
        (i_var4 + 0x4) = u_var2;
        (i_var4 + 0x6) = param_9;
    }
    (i_var4 + 0x22) = 0x0;
    (i_var4 + 0x1e) = 0x0;
    (i_var4 + 0x20) = 0x0;
    if (ctx.PTR_LOOP_1050_5e18 == 0x0) {
        pvVar3 = MakeProcInstance16(param_10, ctx.PTR_LOOP_1050_038c);
        ctx.PTR_LOOP_1050_5e18 = CONCAT22(param_9, pvVar3);
    }
    ctx.PTR_LOOP_1050_5e16 = ctx.PTR_LOOP_1050_5e16 + 0x1;
    return;
}

pub fn struct_1040_a598(param_1: U32Ptr) {
    let i_var1: &mut Struct259;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x0;
    i_var1.field_0x2 = 0x0;
    i_var1.field_0x6 = 0x0;
    i_var1.field_0xa = 0x0;
    i_var1.field_0xc = 0x0;
    i_var1.field_0x10 = 0x0;
    i_var1.field_0x12 = 0x0;
    i_var1.field_0x14 = 0x0;
    i_var1.field_0x16 = 0x0;
    return;
}

pub fn struct_1040_b082(param_1: &mut Struct57, param_2: u32) {
    let i_var1: &mut Struct437;
    let u_var1: u16;

    get_sys_metrics_1040_7728(param_1, 0x1, 0x0, param_2, (param_2 >> 0x10));
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x8e = 0x0;
    i_var1.field_0x90 = 0x0;
    param_1 = 0xb772;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    return;
}
