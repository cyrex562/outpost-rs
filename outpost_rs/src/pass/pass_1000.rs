use crate::defines::{
    Struct171, Struct18, Struct197, Struct20, Struct79, Struct99, StructA, Struct_1000_09ca,
    Struct_1000_0c32, Struct_1000_2cb0, Struct_1000_30a4, Struct_1000_34cf, Struct_160, Struct_211,
    U32Ptr,
};
use crate::exit::exit_1000_25f2;
use crate::fn_ptr::fn_ptr_1000::{call_fn_ptr_1000_0dc6, fn_ptr_op_1000_2594};
use crate::global::Globals;
use crate::mem_1000::{
    free_mem_1000_407a, mem_1000_0670, mem_1000_167a, mem_1000_2bb6, mem_op_1000_0510,
    mem_op_1000_0838, mem_op_1000_0a48, mem_op_1000_1532, mem_op_1000_160a, mem_op_1000_1b9a,
    mem_op_1000_1dfa, mem_op_1000_21b6, mixed_mem_op_1000_3c51,
};
use crate::misc::ret_op_1000_55ac;
use crate::msg_box::{msg_box_op_1000_1f24, msg_box_op_1000_214c};
use crate::string::string_1000;
use crate::string::string_1000::{str_op_1000_3da4, str_op_1000_3dbe, string_1000_3d3e};
use crate::struct_ops::struct_1008;
use crate::sys_api::{
    dos3_call_1000_514e, dos3_call_1000_5174, dos3_call_op_1000_35fe, dos3_op_1000_256b,
    mixed_dos3_call_1000_370a, mixed_dos3_call_1000_39f2, sys_1000_30b4,
};
use crate::util::{
    address_of, make_u16_ptr, read_string_from_addr, read_string_from_rsrc, read_struct_from_addr,
    write_bool_to_addr, write_string_to_addr, CARRY2, CARRY4, CONCAT11, CONCAT12, CONCAT13,
    CONCAT22, SBORROW2, SEXT24, SUB42, ZEXT24,
};
use crate::win_struct::{CONTEXT, WNDCLASS16};
use crate::winapi::{
    swi, FatalAppExit16, FatalExit, GetModuleFileName16, GlobalDOSFree16, SegmentLimit,
};
use crate::{FatalAppExit16, FatalExit, string::string_1000::str_op_1000_28dc, win_struct::{HINSTANCE16, SEGPTR}, winapi::{DOS3Call, GetDOSEnvironment16}};

pub fn pass1_1000_010c() {
    unimplemented!()
}

pub fn pass1_1000_0368(param_1: U32Ptr, param_2: u16, param_3: U32Ptr) {
    let pu_var1: U32Ptr;

    if (param_1 + 0x4) == param_1 {
        (param_3 + param_2 * 0x2) = 0x0;
    } else {
        ((param_1 + 0x6) + 0x4) = (param_1 + 0x4);
        ((param_1 + 0x4) + 0x6) = (param_1 + 0x6);
        pu_var1 = (param_2 * 0x2 + param_3) as u32;
        if pu_var1 == param_1 {
            pu_var1 = (param_1 + 0x4);
        }
    }
    (param_1 + 0x4) = (param_3 + 0xa);
    (param_3 + 0xa) = param_1;
    return;
}

pub fn pass1_1000_05b4(param_1: u8, param_2: i16) {
    (param_2 + 0xa) = 0x1;
    (param_2 + 0x8) = 0x668;
    (param_2 + 0x13) = -((param_1 & 0x2) != 0x0) & 0x2;
    (param_2 + 0x10) = 0x0;
    (param_2 + 0xe) = 0x0;
    return;
}

pub fn pass1_1000_0782(
    param_1: &mut StructA,
    param_2: u16,
    param_3: i16,
    in_stack_00000004: u16,
) -> u16 {
    (param_3 + 0xe) = 0x0;
    (param_3 + 0x10) = param_3 + 0x14;
    (param_3 + 0x8) = 0x9a0;
    pass1_1000_07ac(((param_1 + 0x18) as u16), param_2 as i16, param_3);
    return 0x1;
}

pub fn pass1_1000_07ac(param_1: u16, param_2: i16, param_3: i16) {
    let pu_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;

    pu_var1 = (param_3 + 0x10) as u32;
    (param_3 + 0xe) = pu_var1 as i16;
    u_var3 = (param_2 + (param_3 - pu_var1)) as u16;
    i_var2 = (pu_var1 + (u_var3 - u_var3 % param_1)) as i16;
    (param_3 + 0x10) = i_var2;
    while pu_var1 < (i_var2 - param_1) as u32 {
        pu_var1 = (pu_var1 + param_1);
        pu_var1 = (pu_var1 + param_1);
    }
    pu_var1 = 0x0;
    return;
}

pub fn pass1_1000_07fc(
    ctx: &mut Globals,
    param_1: u16,
    param_2: u32,
    mut out: Option<Struct99>,
) {
    let pa_var1: Struct99;

    if (param_2 + 0x14) != -0x4153 {
        pass1_1000_1e61(ctx, param_1, 0xa, None, 0x0);
        out = None;
    }
    pa_var1 = mem_op_1000_0838(ctx, None, param_1);
    out = Some(pa_var1);
}

pub fn pass1_1000_093a(ctx: &mut Globals, param_1: &mut U32Ptr, param_2: u16, param_3: u16) -> u16 {
    let pi_var1: U32Ptr;
    if &ctx.PTR_LOOP_1050_000c != -0x352f {
        pass1_1000_1e61(ctx, param_3, 0xe, None, 0);
        return 0x0;
    }
    *param_1 = ctx.PTR_LOOP_1050_000e;
    if *param_1 == 0x0 {
        ctx.DAT_1050_0004 = 0x1;
    }
    ctx.PTR_LOOP_1050_000e = *param_1 as u32;
    pi_var1 = ctx.PTR_LOOP_1050_000a;
    pi_var1 = pi_var1 - 0x1;
    if pi_var1 == 0x0 {
        mem_op_1000_0510(ctx, 0x1, 0x0, param_3);
    }
    return 0x1;
}

pub unsafe fn pass1_1000_09a0(ctx: &mut Globals, param_1: &mut U32Ptr, param_2: u16) -> U32Ptr {
    let pu_var1: U32Ptr;
    let u_var2: u32;

    *param_1 = ctx.PTR_LOOP_1050_000e;
    if ctx.PTR_LOOP_1050_000e == 0x0 {
        ctx.DAT_1050_0004 = 0x1;
    }
    ctx.PTR_LOOP_1050_000a = ctx.PTR_LOOP_1050_000a - 0x1;
    pu_var1 = ctx.PTR_LOOP_1050_000e;
    ctx.PTR_LOOP_1050_000e = *param_1;
    if ctx.PTR_LOOP_1050_000a == 0x0 {
        u_var2 = mem_op_1000_0510(ctx, 0x1, 0x0, param_2);
        pu_var1 = u_var2;
    }
    return pu_var1;
}

pub fn pass1_1000_09ca(param_1: i16, opt_struct_1: Option<&mut Struct_1000_09ca>) -> u16 {
    let pu_var1: U32Ptr;
    let i_var2: i16;
    let struct_2: &mut Struct_211;
    let struct_3: &mut Struct_160;

    let struct_1 = opt_struct_1.unwrap();

    pu_var1 = struct_1.field_0xa as u32;
    struct_3 = read_struct_from_addr::<Struct_160>(
        (struct_1 + (param_1 - pu_var1) - 6 & 0xfffc) + pu_var1,
    );
    struct_3.field_0x0 = 0x1;
    struct_1.field_0x7 = pu_var1;
    struct_3.field_0x2 = struct_3;
    struct_3.field_0x1 = struct_3;
    struct_1.field_0x8 = struct_3.clone();
    if ((struct_1.field_0x0.field_0x6) & 0x7) == 0x2 {
        struct_1.field_0x9 = 0x8;
    } else {
        struct_2 = &mut struct_1.field_0x0;
        i_var2 = (struct_2 + 0x18);
        *struct_1[0x9] = (i_var2 - 0x5 & !-(i_var2 + 0x3 < 0x8)) + 0x8;
    }
    struct_3[-0x1] = (struct_3 - pu_var1);
    pu_var1 = (struct_3 - pu_var1) | 0x2;
    struct_1[0xc] = struct_3;
    struct_1[0xb] = struct_3[0x1];
    (struct_3[0x1] + 0x4) = pu_var1;
    struct_3[0x1] = pu_var1;
    struct_1[0x4] = 0xe08;
    return (pu_var1 & 0xfffc) as u16;
}

pub fn pass1_1000_0c32(param_1: u16, param_2: u16, param_3: &mut Struct_1000_0c32) -> u32 {
    let pu_var1: u16;
    let mut var2: U32Ptr;
    let pi_var3: i16;
    let u_var4: &mut Struct_1000_0c32;
    let u_var5: u16;
    let pu_var6: U32Ptr;
    let i_var7: &mut Struct_1000_0c32;
    let pu_var8: U32Ptr;
    let u_var9: u16;
    let u_stack14: u16;
    let pu_stack8: u16;
    let u_stack6: u16;

    pu_var8 = param_3.field_0xe;
    u_stack6 = 0x0;
    pu_var6 = pu_var8;
    loop {
        loop {
            u_var5 = *pu_var6;
            if param_1 <= u_var5 {
                u_var5 = (u_var5 & 0xfffc) - param_1;
                pu_var1 = (param_3 + 0x12);
                pu_stack8 = pu_var6;
                if *pu_var1 < u_var5 || *pu_var1 == u_var5 {
                    u_stack14 = param_1;
                    if (param_2 & 0x6) == 0x0 {
                        pu_stack8 = (u_var5 + pu_var6);
                        pu_stack8[-0x1] = u_var5;
                        *pu_var6 = u_var5 | 0x2;
                        pu_var8 = pu_var6[0x1];
                        var2 = (pu_stack8 + param_1) as u32;
                        *var2 = *var2 | 0x2;
                        *pu_stack8 = param_1 | 0x1;
                    } else {
                        *pu_var6 = param_1 & 0xff00 | pu_var6 & 0x2 | param_1 & 0xff | 0x1;
                        (pu_var6[0x2] + 0x2) = pu_var6[0x1];
                        (pu_var6[0x1] + 0x4) = pu_var6[0x2];
                        pu_var8 = (pu_var6 + param_1);
                        (pu_var8 + (u_var5 - 0x2)) = u_var5;
                        *pu_var8 = u_var5 | 0x2;
                        u_var5 = (param_3 + 0x10);
                        pu_var8[0x2] = u_var5;
                        pu_var8[0x1] = (u_var5 + 0x2);
                        ((u_var5 + 0x2) + 0x4) = pu_var8;
                        (u_var5 + 0x2) = pu_var8;
                    }
                } else {
                    pu_var8 = pu_var6[0x1];
                    (pu_var6[0x2] + 0x2) = pu_var8;
                    (pu_var6[0x1] + 0x4) = pu_var6[0x2];
                    pu_var1 = pu_var6;
                    pu_var1 = pu_var1 | 0x1;
                    u_stack14 = *pu_var6 & 0xfffc;
                    (pu_var6 + u_stack14) = (pu_var6 + u_stack14) | 0x2;
                }
                (param_3 + 0xe) = pu_var8;
                if (param_2 & 0x1) != 0x0 {
                    pu_var6 = pu_stack8;
                    // TODO
                    // for (u_var5 = u_stack14 - 0x2 >> 0x1; pu_var6 = pu_var6 + 0x1, u_var5 != 0x0;
                    //     u_var5 -= 0x1) {
                    //   *pu_var6 = 0x0;
                    // }
                    if (u_stack14 - 0x2 & 0x1) != 0x0 {
                        *pu_var6 = 0x0;
                    }
                }
                if ((param_2 & 0x2) != 0x0) && (pu_var8[0x1] == pu_var8[0x2]) {
                    *(param_3 + 0x4) = *((param_3 + 0x10) + 0x2) & 0xfffc;
                    u_var4 = (param_3 + 0x4);
                    var2 = (u_var4 + 0x3);
                    *var2 = *var2 | 0x80;
                }
                pi_var3 = (param_3 + 0xa);
                *pi_var3 = *pi_var3 + 0x1;
                return CONCAT22(0x1050, pu_stack8 + 0x1);
            }
            if u_stack6 < u_var5 {
                u_stack6 = u_var5;
            }
            pu_var6 = pu_var6[0x1];
            if pu_var6 == pu_var8 {
                break;
            }
        }
        if ((param_2 & 0x2) == 0x0) || ((param_2 & 0x40) != 0x0) {
            break;
        }
        u_var4 = param_3;
        // u_var9 = (u_var4 >> 0x10);
        i_var7 = u_var4;
        if (i_var7 + 0x34) == 0x0 {
            break;
        }
        u_stack6 = (i_var7 + 0x34)();
        pu_var6 = (param_3 + 0xe);
        if (u_stack6 < param_1) || (pu_var6 == 0x0) {
            break;
        }
    }
    (param_3 + 0x4)[0] = u_stack6 & 0xfffc;
    return 0x0;
}

pub unsafe fn pass1_1000_0e08(ctx: &mut Globals, param_1: i16, param_2: u16) -> u16 {
    let pu_var1: i16;
    let pb_var2: U32Ptr;
    let u_var3: u16;
    let pu_var4: i16;
    let pu_var5: i16;
    let b_var6: bool;
    let u_var7: u32;

    pu_var5 = (param_1 - 0x2);
    b_var6 = (pu_var5 & 0x2) != 0x0;
    if b_var6 {
        pu_var1 = pu_var5;
        pu_var1 = pu_var1 & 0xfe;
    } else {
        pu_var4 = (pu_var5 - (param_1 - 0x4));
        pu_var1 = pu_var4;
        *pu_var1 = *pu_var1 + (*pu_var5 & 0xfffc);
        pu_var5 = pu_var4;
    }
    pu_var4 = ((*pu_var5 & 0xfffc) + pu_var5);
    if (pu_var4 & 0x1) == 0x0 {
        pu_var1 = pu_var5;
        *pu_var1 = *pu_var1 + (*pu_var4 & 0xfffc);
        if pu_var4 == ctx.PTR_LOOP_1050_000e as i16 {
            ctx.PTR_LOOP_1050_000e = pu_var5 as u32;
        }
        (pu_var4[0x2] + 0x2) = pu_var4[0x1];
        (pu_var4[0x1] + 0x4) = pu_var4[0x2];
        pu_var4 = ((*pu_var5 & 0xfffc) + pu_var5);
    }
    pu_var4[-0x1] = *pu_var5 & 0xfffc;
    u_var3 = ctx.DAT_1050_0004 as u16;
    pu_var1 = pu_var4 + -0x1;
    if u_var3 <= *pu_var1 && *pu_var1 != u_var3 {
        u_var3 = *pu_var5 & 0xfffc;
        ctx.DAT_1050_0004 = u_var3 as i16;
    }
    pu_var1 = pu_var4;
    pu_var1 = pu_var1 & 0xfd;
    if b_var6 {
        if (ctx.PTR_LOOP_1050_0010 + 0x2) != ctx.PTR_LOOP_1050_0010 {
            pb_var2 = (ctx.DAT_1050_0004 + 0x3) as u32;
            *pb_var2 = *pb_var2 & 0x7f;
        }
        pu_var5[0x2] = ctx.PTR_LOOP_1050_0010;
        u_var3 = (ctx.PTR_LOOP_1050_0010 + 0x2) as u16;
        pu_var5[0x1] = u_var3;
        ((ctx.PTR_LOOP_1050_0010 + 0x2) + 0x4) = pu_var5 as u32;
        (ctx.PTR_LOOP_1050_0010 + 0x2) = pu_var5 as u32;
    }
    ctx.PTR_LOOP_1050_000a = ctx.PTR_LOOP_1050_000a + -0x1;
    if ctx.PTR_LOOP_1050_000a == 0x0 {
        u_var7 = mem_op_1000_0510(ctx, 0x1, 0x0, param_2);
        u_var3 = u_var7 as u16;
    }
    return u_var3;
}

pub fn pass1_1000_0ed4(
    ctx: &mut Globals,
    param_1: u16,
    mut param_2: u16,
    param_3: u8,
    param_4: u16,
    param_5: u16,
    param_6: &mut Struct18,
    param_7: u16,
) -> i32 {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let mut struct_1: Option<&mut Struct18>;
    let u_var5: u16;
    let pu_var6: &mut Struct18;
    let pu_var7: U32Ptr;
    let uvar8: u16;
    let uvar9: Option<&mut StructA>;
    let uvar10: u16;
    let l_stack12: i32;
    let u_stack8: u16;
    let ustack: u16;
    let ustack4: U32Ptr;

    if (ctx.PTR_LOOP_1050_000c & 0xfff8) == 0xcad0 {
        let u_stack6 = 0x0;
        ustack4 = address_of(ctx.PTR_LOOP_1050_0002);
        if (param_3 & 0x8) == 0x0 {
            struct_1 = Some(param_6);
        } else {
            struct_1 = None;
            param_2 = 0x0;
        }
        l_stack12 = CONCAT22(param_2, struct_1.field_0x0) as i32;
        u_stack8 = pass1_1000_0fb8(
            ctx,
            param_1,
            param_4,
            param_6,
            param_5,
            param_3 as u16,
            struct_1,
            param_2,
        );
        if u_stack8 == 0x0 {
            return CONCAT22(param_7, param_6.field_0x0) as i32;
        }
        if (param_3 & 0x8) == 0x0 {
            l_stack12 = mem_op_1000_0a48(
                ctx,
                param_3 as u8,
                param_4 as u32,
                CONCAT22(ustack4 as u16, u_stack6),
                param_1 as u8,
            );
            // u_var3 = (l_stack12 >> 0x10);
            pu_var7 = l_stack12 as u32;
            if l_stack12 != 0x0 {
                pu_var6 = param_6;
                // TODO
                // for (u_var5 = u_stack8 >> 0x1; u_var5 != 0x0; u_var5 -= 0x1) {
                //   pu_var2 = pu_var7;
                //   pu_var7 = pu_var7 + 0x1;
                //   pu_var1 = pu_var6;
                //   pu_var6 = pu_var6 + 0x1;
                //   *pu_var2 = *pu_var1;
                // }

                // TODO
                // for (u_var5 = ((u_stack8 & 0x1) != 0x0); u_var5 != 0x0; u_var5 -= 0x1) {
                //   pu_var2 = pu_var7;
                //   pu_var7 = (pu_var7 + 0x1);
                //   pu_var1 = pu_var6;
                //   pu_var6 = (pu_var6 + 0x1);
                //   *pu_var2 = *pu_var1;
                // }
                call_fn_ptr_1000_0dc6(ctx, param_6, param_1);
            }
            return l_stack12;
        }
        if (param_5 | param_4) == 0x0 {
            return 0x0;
        }
        uvar8 = 0x5;
        uvar9 = u_stack6;
        uvar10 = ustack4 as u16;
    } else {
        uvar8 = 0xe;
        uvar9 = None;
        uvar10 = 0x0;
    }
    pass1_1000_1e61(ctx, param_1, uvar8, uvar9, uvar10);
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1000_0fb8(
    ctx: &mut Globals,
    param_1: u16,
    param_2: u16,
    param_3: &mut Struct18,
    param_4: u16,
    param_5: u16,
    param_6: Option<&mut Struct18>,
    param_7: u16,
) -> u16 {
    let pu_var1: U32Ptr;
    let b_var2: u8;
    let u_var3: u16;
    let bvar4: bool;
    let i_var5: i16;
    let u_var6: u16;
    let pu_var7: U32Ptr;
    let pu_var8: U32Ptr;
    let u_var9: u32;
    let u_stack4: u16;

    if (param_4 | param_2) == 0x0 {
        pass1_1000_1e61(
            ctx,
            param_1,
            0x4,
            Some(read_struct_from_addr(ctx.PTR_LOOP_1050_0000)),
            ctx.PTR_LOOP_1050_0002 as u16,
        );
        if (param_7 | param_6) != 0x0 {
            param_6.field_0x1 = 0x0;
            param_6.field_0x0 = 0x0;
            return 0x0;
        }
        return 0x1;
    }
    b_var2 = (ctx.PTR_LOOP_1050_000c & 0x7) as u8;
    if (ctx.PTR_LOOP_1050_000c & 0x7) != 0x0 {
        if b_var2 == 0x1 {
            u_var3 = (ctx.PTR_LOOP_1050_0000 + 0x18) as u16;
            if false {
                return 0x0;
            }
            if param_4 == 0x0 {
                if param_2 <= u_var3 {
                    return 0x0;
                }
                return u_var3;
            }
            return u_var3;
        }
        if b_var2 != 0x2 {
            if b_var2 != 0x3 {
                if (param_7 | param_6) != 0x0 {
                    param_6.field_0x1 = 0x0;
                    param_6.field_0x0 = 0x0;
                    return 0x0;
                }
                return 0x1;
            }
            if (((param_7 | param_6) != 0x0) && (param_4 == 0x0))
                && (false || (param_2 <= (ctx.PTR_LOOP_1050_0000 + 0x1c) as u16))
            {
                u_var9 = pass1_1000_1284(ctx, CONCAT22(0x1050, param_3 as u16), param_1);
                u_var3 = u_var9 as u16;
                if u_var9 <= CONCAT22(param_4, param_2) {
                    return u_var3;
                }
                if (false) && (u_var3 <= param_2) {
                    return u_var3;
                }
                return param_2;
            }
            i_var5 = mem_1000_0670(
                param_5,
                CONCAT22(param_7, Some(&param_6).unwrap().field_0x0),
                param_2,
                0x0,
                param_4 as i16,
                param_1,
            ) as i16;
            if i_var5 != 0x0 {
                return 0x0;
            }
            if (param_7 | &param_6) != 0x0 {
                return 0x0;
            }
            return 0x1;
        }
    }
    pu_var8 = (param_3 + -0x2);
    u_var3 = *pu_var8 & 0x7ffc;
    u_stack4 = u_var3 - 0x2;
    if ((param_3 + -0x1) & 0x80) != 0x0 {
        u_stack4 = u_var3 - 0x6;
    }
    if (true) && (param_4 != 0x0 || (u_stack4 < param_2)) {
        if (true) {
            if param_4 != 0x0 {
                return u_stack4;
            }
            if (ctx.PTR_LOOP_1050_0000 + 0x1c) < param_2 as u32 {
                return u_stack4;
            }
        }
    }
    let BVar4 = pass1_1000_115c(ctx, param_2 as i16, pu_var8);
    if BVar4 == 0x0 {
        return u_stack4;
    }
    if (param_5 & 0x1) != 0x0 {
        u_var3 = (*pu_var8 & 0x7ffc) - 0x2;
        if u_stack4 < param_2 {
            pu_var7 = (u_stack4 + param_3);
            i_var5 = -u_stack4 as i16;
        } else {
            if u_var3 <= param_2 {
                return 0x0;
            }
            pu_var7 = (param_2 + param_3);
            i_var5 = -param_2 as i16;
        }
        u_var3 += i_var5;
        // TODO
        // for (u_var6 = u_var3 >> 0x1; u_var6 != 0x0; u_var6 -= 0x1) {
        //   pu_var1 = pu_var7;
        //   pu_var7 = pu_var7 + 0x1;
        //   *pu_var1 = 0x0;
        // }
        if (u_var3 & 0x1) != 0x0 {
            *pu_var7 = 0x0;
        }
    }
    return 0x0;
}

pub fn pass1_1000_115c(ctx: &mut Globals, param_1: i16, param_2: U32Ptr) -> bool {
    let ptr_1: U32Ptr;
    let ptr_2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let ptr_3: U32Ptr;
    let i_var6: i16;
    let u_stack4: u16;

    u_var3 = *param_2 & 0x7ffc;
    u_var4 = (param_1 + 0x5 & 0xfffc) as u16;
    u_var4 = (u_var4 - 0x8 & !-(u_var4 < 0x8)) + 0x8;
    if u_var3 < u_var4 {
        ptr_3 = (u_var3 + param_2) as u32;
        if ((ptr_3 & 0x1) != 0x0) || ((*ptr_3 & 0xfffc) + u_var3 < u_var4) {
            return false;
        }
        if ptr_3 == ctx.PTR_LOOP_1050_000e {
            ctx.PTR_LOOP_1050_000e = ptr_3[0x1];
        }
        (ptr_3[0x2] + 0x2) = ptr_3[0x1];
        (ptr_3[0x1] + 0x4) = ptr_3[0x2];
        u_stack4 = ((*ptr_3 & 0xfffc) + u_var3) - u_var4;
        if u_stack4 < ctx.s_version__d__d_1050_0012._0_2_ {
            ptr_2 = param_2;
            *ptr_2 = *ptr_2 + (*ptr_3 & 0xfffc);
            ptr_1 = (ptr_3 + (*ptr_3 & 0xfffc));
            *ptr_1 = *ptr_1 | 0x2;
            return true;
        }
    } else {
        u_stack4 = u_var3 - u_var4;
        if u_stack4 < ctx.s_version__d__d_1050_0012._0_2_ {
            return true;
        }
        ptr_3 = (u_var3 + param_2) as u32;
        if (ptr_3 & 0x1) == 0x0 {
            u_stack4 += *ptr_3 & 0xfffc;
            if ptr_3 == ctx.PTR_LOOP_1050_000e {
                ctx.PTR_LOOP_1050_000e = ptr_3[0x1];
            }
            (ptr_3[0x2] + 0x2) = ptr_3[0x1];
            (ptr_3[0x1] + 0x4) = ptr_3[0x2];
        }
        if *ctx.DAT_1050_0004 < u_stack4 {
            *ctx.DAT_1050_0004 = u_stack4;
        }
    }
    *param_2 = *param_2 & 0x8003 | u_var4;
    (u_var4 + param_2) = u_stack4 | 0x2;
    i_var6 = (u_var4 + param_2) as i16;
    (i_var6 + 0x4) = ctx.PTR_LOOP_1050_0010 as i16;
    (i_var6 + 0x2) = (ctx.PTR_LOOP_1050_0010 + 0x2) as i16;
    ((ctx.PTR_LOOP_1050_0010 + 0x2) + 0x4) = i_var6 as u32;
    (ctx.PTR_LOOP_1050_0010 + 0x2) = i_var6 as u32;
    ((i_var6 + u_stack4) + -0x2) = u_stack4 as i16;
    ptr_1 = (i_var6 + u_stack4) as u32;
    *ptr_1 = *ptr_1 & 0xfd;
    return true;
}

pub fn pass1_1000_1284(ctx: &mut Globals, param_1: u32, param_2: u16) -> u32 {
    let b_var1: u8;
    let u_var2: u16;
    let u_var3: u32;
    let b_var4: u8;
    let u_var5: u16;
    let b_var6: bool;
    let dvar7: u32;
    let u_stack6: u16;
    let i_stack4: i16;

    if (&ctx.PTR_LOOP_1050_000c & 0xfff8) != 0xcad0 {
        pass1_1000_1e61(ctx, param_2, 0xe, None, 0);
        return 0xffffffff;
    }
    b_var1 = ctx.PTR_LOOP_1050_000c as u8;
    b_var4 = b_var1 & 0x7;
    if (b_var1 & 0x7) != 0x0 {
        if b_var4 == 0x1 {
            u_var3 = 0x0;
            return u_var3 + 0x18;
        }
        if b_var4 != 0x2 {
            if b_var4 != 0x3 {
                return 0xffffffff;
            }
            dvar7 = mem_op_1000_1532(ctx, param_2);
            return CONCAT22((dvar7 >> 0x10) - (dvar7 < 0x14), (dvar7 - 0x14) as u16);
        }
    }
    u_var2 = (param_1 + -0x2) as u16;
    u_var5 = u_var2 & 0x7ffc;
    u_stack6 = u_var5 - 0x2;
    i_stack4 = 0x0;
    if (u_var2 & 0x8000) != 0x0 {
        b_var6 = u_stack6 < 0x4;
        u_stack6 = u_var5 - 0x6;
        i_stack4 = i16::from(!b_var6);
    }
    return CONCAT22(i_stack4 as u16, u_stack6);
}

pub fn pass1_1000_15ce(
    ctx: &mut Globals,
    param_1: &mut u16,
    param_2: &mut u16,
    param_3: &mut u16,
) {
    let pu_var1: U32Ptr;
    let u_var2: u16;

    u_var2 = param_2 | param_1;
    while u_var2 != 0x0 {
        pu_var1 = param_1[0];
        *param_2 = param_1[0x1];
        GlobalDOSFree16(*param_3);
        // param_1 = pu_var1;
        *param_3 = ctx.s_tile2_bmp_1050_1538 as u16;
        u_var2 = param_2 | pu_var1;
    }
    return;
}

pub fn pass1_1000_16aa(
    ctx: &mut Globals,
    param_1: &mut Struct18,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) -> u16 {
    let u_var1: u16;
    let l_var1: i32;

    if (param_2 | param_1) == 0x0 {
        u_var1 = mem_1000_167a(ctx, param_3, param_5, param_4);
        return u_var1;
    }
    if param_3 == 0x0 {
        pass1_1000_16ee(ctx, param_1, param_2, param_5);
        return 0x0;
    }
    l_var1 = pass1_1000_0ed4(ctx, param_5, param_6, 0x0, param_3, 0x0, param_1, param_2);
    return l_var1 as u16;
}

pub fn pass1_1000_16ee(ctx: &mut Globals, param_1: &mut Struct18, param_2: u16, param_3: u16) {
    if (param_2 | param_1) != 0x0 {
        call_fn_ptr_1000_0dc6(ctx, param_1, param_3);
    }
    return;
}

pub fn pass1_1000_17e8(ctx: &mut Globals, param_1: U32Ptr, param_2: U32Ptr) -> U32Ptr {
    let pu_var1: U32Ptr;
    pu_var1 = ctx.PTR_LOOP_1050_5f34;
    ctx.PTR_LOOP_1050_5f34 = param_1;
    ctx.PTR_LOOP_1050_5f36 = param_2;
    return pu_var1;
}

pub unsafe fn pass1_1000_180c(
    ctx: &mut Globals,
    param_1: u16,
    param_2: &mut Struct18,
    param_3: u16,
) -> u16 {
    let ptr_1: U32Ptr;
    let var_2: i32;

    if (ctx.PTR_LOOP_1050_5f2e | ctx.PTR_LOOP_1050_5f2c) == 0x0 {
        ptr_1 = mem_op_1000_160a(ctx, Some(param_2), param_3) as u32;
        if (param_2 | ptr_1) == 0x0 {
            return 0x0;
        }
    }
    var_2 = mem_op_1000_0a48(
        ctx,
        0x0,
        param_1 as u32,
        CONCAT22(ctx.PTR_LOOP_1050_5f2e as u16, ctx.PTR_LOOP_1050_5f2c as u16),
        param_3 as u8,
    );
    return var_2 as u16;
}

pub unsafe fn pass1_1000_183c(
    ctx: &mut Globals,
    param_1: u16,
    param_2: u16,
    param_3: u16,
) -> u16 {
    let pu_var1: U32Ptr;
    let l_var2: i32;

    pu_var1 = 0x0;
    if (param_2 * param_1 >> 0x10) != 0x0 {
        return 0x0;
    }
    ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx, None, param_3) as u32;
    ctx.PTR_LOOP_1050_5f2e = pu_var1;
    if ((ctx.PTR_LOOP_1050_5f2e | ctx.PTR_LOOP_1050_5f2c) == 0x0)
        && ((pu_var1 | ctx.PTR_LOOP_1050_5f2c) == 0x0)
    {
        return 0x0;
    }
    l_var2 = mem_op_1000_0a48(
        ctx,
        0x1,
        ((param_2 * param_1) as u32),
        CONCAT22(ctx.PTR_LOOP_1050_5f2e as u16, ctx.PTR_LOOP_1050_5f2c as u16),
        param_3 as u8,
    );
    return l_var2 as u16;
}

pub unsafe fn pass1_1000_188e(
    ctx: &mut Globals,
    param_1: &mut Struct18,
    param_2: u16,
    param_3: u16,
    param_4: &mut Struct18,
    param_5: u16,
    param_6: u16,
) -> u16 {
    let u_var1: u16;
    let l_var2: i32;

    if (param_2 | param_1) == 0x0 {
        u_var1 = pass1_1000_180c(ctx, param_3, param_4, param_5);
        return u_var1;
    }
    if param_3 == 0x0 {
        pass1_1000_18d2(ctx, param_1, param_2, param_5);
        return 0x0;
    }
    l_var2 = pass1_1000_0ed4(ctx, param_5, param_6, 0x0, param_3, 0x0, param_1, param_2);
    return l_var2 as u16;
}

pub fn pass1_1000_18d2(ctx: &mut Globals, param_1: &mut Struct18, param_2: u16, param_3: u16) {
    if (param_2 | param_1) != 0x0 {
        call_fn_ptr_1000_0dc6(ctx, param_1, param_3);
    }
    return;
}

pub fn pass1_1000_1a54(
    ctx: &mut Globals,
    param_1: U32Ptr,
    param_3: u16,
    param_4: u16,
    param_2: u16,
) -> u16 {
    let u_var1: u16;
    let u_var2: u16;

    if (param_2 + 0x14) != -0x4153 {
        pass1_1000_1e61(ctx, param_4, 0xa, None, 0x0);
        return 0x0;
    }
    u_var1 = pass1_1000_1ab0(param_1 as u16);
    if u_var1 < ((param_2 + 0x18) + 0x14) as u16 {
        u_var2 = 0x0;
    } else {
        u_var2 = (param_2 + 0x1a) as u16;
        (param_2 + 0x1a) = u_var1 as i16;
        (param_2 + 0x1c) = (u_var1 >> 0x2) as i16;
    }
    return u_var2;
}

pub fn pass1_1000_1ab0(param_1: u16) -> u16 {
    let u_var1: u16;
    let u_var2: u16;

    if param_1 == 0x2000 {
        return 0x2000;
    }
    if param_1 < 0xfff0 {
        if param_1 < 0x1001 {
            return 0x1000;
        }
        u_var1 = 0x2000;
        if param_1 < 0x2001 {
            loop {
                u_var2 = u_var1;
                u_var1 = u_var2 >> 0x1;
                if param_1 > u_var1 {
                    break;
                }
            }
            return u_var2 & 0xfffe;
        }
        while (u_var1 *= 0x2, u_var1 != 0x0) {
            if param_1 <= u_var1 {
                return (u_var1 + 0x10 & -(u_var1 < 0xfff0)) - 0x10;
            }
        }
    }
    return 0xfff0;
}

pub fn pass1_1000_1afe(
    ctx: &mut Globals,
    param_1: u16,
    param_2: &mut StructA,
    param_3: u16,
    unaff_cs: u16,
) -> bool {
    let u_var1: u16;

    if param_1 == 0x0 {
        u_var1 = 0x0;
    } else {
        u_var1 = param_1 + 0x1 & 0xfffe;
    }
    if (param_2 + 0x14) == -0x4153 {
        if (u_var1 < param_1) || ((param_2 + 0x1a) - 0x14 < u_var1) {
            pass1_1000_1e61(ctx, unaff_cs, 0x3, Some(param_2), param_3);
        } else {
            if (param_2 + 0x2) == 0x0 {
                (param_2 + 0x18) = u_var1;
                return true;
            }
        }
        return false;
    }
    pass1_1000_1e61(ctx, unaff_cs, 0xa, None, 0x0);
    return false;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1000_1e61(
    ctx: &mut Globals,
    param_1: u16,
    param_2: u16,
    param_3: Option<&mut StructA>,
    param_4: u16,
) -> u16 {
    let b_var1: bool;
    let b_var2: bool;
    let u_var3: u16;
    let u_stack64: u16;
    let u_stack62: Option<&mut StructA>;
    let u_stack60: u16;
    let pc_stack6: u32;
    let pu_stack4: U32Ptr;
    let u_var3: u16;

    u_var3 = ctx.data_seg;
    u_stack62 = param_3;
    u_stack60 = param_4;
    u_stack64 = param_2;
    pu_stack4 = ctx.data_seg as u32;
    if true {
        pc_stack6 = ctx.PTR_1050_5f1a;
        if (ctx.PTR_LOOP_1050_5f1c | ctx.PTR_1050_5f1a) == 0x0 {
            pc_stack6 = 0x0;
            pu_stack4 = 0x0;
        } else {
            b_var1 = mem_op_1000_21b6(ctx.PTR_1050_5f1a as u16, ctx.PTR_LOOP_1050_5f1c as u16);
            pc_stack6 = ctx.PTR_1050_5f1a;
            pu_stack4 = ctx.PTR_LOOP_1050_5f1c;
            if b_var1 == false {
                ctx.PTR_1050_5f1a = ctx.PTR_1050_1f7e;
                ctx.PTR_LOOP_1050_5f1c = ctx.PTR_LOOP_1050_1000;
                pc_stack6 = ctx.PTR_1050_1f7e;
                pu_stack4 = ctx.PTR_LOOP_1050_1000;
            }
        }
        if (pu_stack4 | pc_stack6) != 0x0 {
            b_var2 = msg_box_op_1000_1f24(ctx.PTR_1050_5f1a as i16, ctx.data_seg, 0x0, 0x1000);
            if b_var2 == false {
                u_var3 = (*pc_stack6)(0x1000, &u_stack64);
            } else {
                pu_stack4 = 0x0;
                pc_stack6 = 0x0;
                u_var3 = 0x0;
            }
            if (pu_stack4 | pc_stack6) != 0x0 {
                pass1_1000_1f68(ctx, u_var3);
            }
            return u_var3;
        }
    }
    return 0x0;
}

pub fn pass1_1000_1f68(ctx: &mut Globals, param_1: u16) {
    ctx.PTR_LOOP_1050_5f26 = ctx.PTR_LOOP_1050_5f26 + -0x1;
    if true && (ctx.PTR_LOOP_1050_5f26 < 0x0) {
        ctx.PTR_LOOP_1050_5f26 = 0x0;
    }
    return;
}

pub unsafe fn pass1_1000_1f7e(param_1: U32Ptr, param_2: u16) -> bool {
    let c_var1: u8;
    let b_var2: bool;
    let u_var3: u16;
    let i_var4: i16;
    // let mut pc_var5: String;
    let mut pc_var5 = "".to_string();

    u_var3 = *param_1;
    if false {
        return false;
    }
    if u_var3 == 0xf {
        //LAB_1000_1fb6:
        i_var4 = 0x1;
    } else {
        if u_var3 < 0x10 {
            c_var1 = u_var3 as u8;
            if c_var1 == '\x02' as u8 {
                // TODO
                // goto LAB_1000_1fb6;
            }
            if ('\0' as u8) < (c_var1 - 0x2) && ((c_var1 + -0x3) < 0xf) {
                i_var4 = 0x0;
                // TODO
                // goto LAB_1000_1fbe;
            }
        }
        i_var4 = 0x0;
        u_var3 = 0x1;
    }
    //LAB_1000_1fbe:
    pc_var5 = string_1000::string_1000_1fd2(u_var3 as i16);
    b_var2 = msg_box_op_1000_214c(0x0, i_var4, &pc_var5, param_2);
    return b_var2;
}

pub fn pass1_1000_1fea(ctx: &mut Globals) -> bool {
    let pu_var1: U32Ptr;
    let b_var2: bool;
    pu_var1 = ctx.PTR_LOOP_1050_5f22 + 0x1;
    ctx.PTR_LOOP_1050_5f22 = pu_var1;
    b_var2 = ctx.PTR_LOOP_1050_5f22 == 0x0;
    if (b_var2) && ((ctx.PTR_LOOP_1050_5f20 | ctx.PTR_LOOP_1050_5f1e) != 0x0) {
        ctx.PTR_LOOP_1050_5f22 = ctx.PTR_LOOP_1050_0002;
    }
    if true {
        return true;
    }
    return false;
}

pub unsafe fn pass1_1000_201c(param_1: i16, param_2: i16, param_3: u16) {
    let u_var1: u16;
    let u_var2: u32;
    let u_var3: u16;
    let bvar4: bool;
    let i_var5: i16;
    let u_var6: u16;

    if param_1 == 0x0 {
        (param_2 + 0x6) = 0x0;
        (param_2 + 0x4) = 0x0;
    }
    u_var3 = ((param_2 + 0x6) | (param_2 + 0x4)) as u16;
    while u_var3 != 0x0 {
        let mut a: u16 = (param_2 + 0x4) as u16;
        let mut b: i16 = (param_2 + 0x6) as i16;
        let BVar4 = pass1_1000_206c(&mut a, &mut b);
        if BVar4 == 0x0 {
            u_var2 = (param_2 + 0x4) as u32;
            // u_var6 = (u_var2 >> 0x10);
            i_var5 = u_var2 as i16;
            u_var1 = (i_var5 + 0x2c) as u16;
            (param_2 + 0x4) = (i_var5 + 0x2a);
            (param_2 + 0x6) = u_var1 as i16;
        } else {
            mem_op_1000_1b9a(
                0x1,
                ((param_2 + 0x4) as u32),
                ((param_2 + 0x6) as u16),
                param_3,
            );
        }
        u_var3 = ((param_2 + 0x6) | (param_2 + 0x4)) as u16;
    }
    return;
}

pub unsafe fn pass1_1000_206c(param_1: &mut u16, param_2: &mut i16) -> bool {
    let u_var1: bool;
    let mut b: u16 = 0x42;
    let mut c: u16 = 0x2;
    u_var1 = pass1_1000_21d2(&mut c, &mut b, param_1, param_2, 0x1);
    if (u_var1 != false) && ((param_1 + 0x14) == -0x4153) {
        return true;
    }
    return false;
}

pub fn pass1_1000_20a2(param_1: u16, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let u_stack8: u16;
    let u_stack4: u16;

    i_var1 = (param_1 + 0x2e) as i16;
    u_var2 = (param_1 + 0x30);
    u_stack8 = 0x0;
    u_var3 = (i_var1 + 0x4) as u16;
    u_stack4 = (i_var1 + 0x6) as u16;
    u_var7 = 0x0;
    if (u_stack4 | u_var3) != 0x0 {
        while (
            u_var6 = u_var3,
            u_var4 = u_stack4,
            u_var6 != param_1 || (u_stack4 != param_2),
        ) {
            u_var3 = (u_var6 + 0x2a);
            u_stack4 = (u_var6 + 0x2c);
            u_var7 = u_var6;
            u_stack8 = u_var4;
            if (u_stack4 | u_var3) == 0x0 {
                return;
            }
        }
        if (u_stack8 | u_var7) != 0x0 {
            u_var2 = (u_var6 + 0x2c);
            (u_var7 + 0x2a) = (u_var6 + 0x2a);
            (u_var7 + 0x2c) = u_var2;
            return;
        }
        u_var5 = (u_var6 + 0x2c);
        (i_var1 + 0x4) = (u_var6 + 0x2a) as i16;
        (i_var1 + 0x6) = u_var5 as i16;
    }
    return;
}

pub unsafe fn pass1_1000_21d2(
    param_1: &mut u16,
    param_2: &mut u16,
    param_3: &mut u16,
    param_4: &mut i16,
    param_5: u8,
) -> bool {
    let mut u_var1 = 0u32;
    let b_var2: bool;

    if true {
        b_var2 = mem_op_1000_1dfa(0x0, *param_1 as u8, *param_3, *param_4 as u16);
        if b_var2 != false {
            return false;
        }
        if (param_1 & 0x4) == 0x0 {
            u_var1 = SegmentLimit(*param_4 as u16) as u32;
            if !((u_var1 >> 0x10) & 0x1) {
                return false;
            }
            if param_2 == 0x0 {
                return true;
            }
            if CARRY4(param_3 as u32, (param_2 - 0x1) as u32) {
                return false;
            }
            if param_3 + (param_2 - 0x1) <= u_var1 as u16 {
                return true;
            }
            return false;
        }
    }
    b_var2 = pass1_1000_22c0(param_3, param_4, param_2, param_2, param_1);
    if b_var2 == false {
        return false;
    }
    return true;
}

pub unsafe fn pass1_1000_2242(
    param_1: &mut u16,
    param_2: &mut u16,
    param_3: &mut u16,
    param_4: &mut i16,
    param_5: u16,
    param_6: U32Ptr,
) -> u32 {
    let u_var1: u16;
    let u_var2: u16;
    let b_var3: bool;

    u_var1 = param_2 | param_1;
    loop {
        if u_var1 == 0x0 {
            return 0x0;
        }
        u_var1 = *param_1;
        if param_2 != 0x0 {
            u_var1 = 0xffff;
        }
        if CARRY2(*param_3, u_var1) != 0 {
            u_var1 = -*param_3;
        }
        b_var3 = *param_1 < u_var1;
        *param_1 -= u_var1;
        *param_2 -= b_var3;
        u_var2 = (*param_6)(u_var1, param_5, param_3, param_4);
        if (u_var2 != 0x0) {
            break;
        }
        // b_var3 = CARRY2(*param_3, u_var1);
        *param_3 += u_var1;
        *param_4 += b_var3 * 0x100;
        u_var1 = param_2 | param_1;
    }
    return CONCAT22(param_2 + CARRY2(u_var2, *param_1), u_var2 + param_1);
}

pub unsafe fn pass1_1000_22c0(
    param_1: &mut u16,
    param_2: &mut i16,
    param_3: &mut u16,
    param_4: &mut u16,
    param_5: &mut u16,
) -> bool {
    pass1_1000_2242(param_3, param_4, param_1, param_2, *param_5, 0x1dfa) == 0
}

pub unsafe fn pass1_1000_24db(ctx: &mut Globals, param_1: i16, param_2: u16) {
    let pc_var1: u32;
    let i_var2: i16;
    let u_var3: u16;
    let c_var4: u8;
    let u_var5: u16;

    i_var2 = (param_2 + 0x1) as i16;
    u_var5 = SUB42(ctx.data_seg, 0x0) as u16;
    ctx.PTR_LOOP_1050_5fc9._0_1_ = 0x0;
    u_var3 = 0x1;
    if (false) {
        fn_ptr_op_1000_2594(0x68b6, 0x68b6);
        fn_ptr_op_1000_2594(ctx.PTR_LOOP_1050_6210, 0x620c);
        ret_op_1000_55ac(param_1 as u32, u_var3, u_var5, i_var2);
    }
    c_var4 = (u_var3 >> 0x8) as u8;
    fn_ptr_op_1000_2594(ctx.PTR_LOOP_1050_6210, ctx.PTR_LOOP_1050_6210);
    fn_ptr_op_1000_2594(ctx.PTR_LOOP_1050_6210, ctx.PTR_LOOP_1050_6210);
    dos3_op_1000_256b();
    if (c_var4 == '\0' as u8) {
        if true {
            pc_var1 = swi(0x21);
            (*pc_var1)();
        } else {
            DOS3Call(read_struct_from_addr::<CONTEXT>(ctx.PTR_LOOP_1050_1000));
        }
    }
    return;
}

pub fn pass1_1000_25a8(ctx: &mut Globals, param_1: u16, param_2: u16) {
    pass1_1000_2913(ctx, 0xfc, param_1, param_2);
    pass1_1000_2913(ctx, 0xff, param_1, param_2);
    return;
}

pub unsafe fn pass1_1000_25d2(
    ctx: &mut Globals,
    param_1: i16,
    param_2: i16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: U32Ptr,
) -> U32Ptr {
    let pu_var1: U32Ptr;
    let pi_var2: U32Ptr;
    let mut pc_var3: &String;
    let pu_var4: U32Ptr;
    let mut u_var5: u16 = 0;
    let pi_var6: U32Ptr;
    // let mut str: String;
    let pc_var10 = "".String();
    let pi_var7: U32Ptr;
    let mut pc_var8: String;
    let i_var9: i16;

    pu_var4 = (param_2 + 0x1 & 0xfffe) as u32;
    if (pu_var4 < param_1 as u32)
        && (
            u_var5 = -(pu_var4 + -param_1) as u16,
            pu_var1 = ctx.PTR_LOOP_1050_000a,
            *pu_var1 < u_var5 || *pu_var1 == u_var5,
        )
    {
        pu_var1 = ctx.PTR_LOOP_1050_000c;
        if u_var5 <= *pu_var1 && *pu_var1 != u_var5 {
            ctx.PTR_LOOP_1050_000c = u_var5 as u32;
        }
        // WARNING: Could not recover jumptable at 0x100025f0. Too many
        // branches
        // WARNING: Treating indirect jump as call
        pi_var6 = (*param_6)();
        return pi_var6;
    }
    i_var9 = 0x0;
    pass1_1000_25a8(ctx, param_3, param_4);
    pass1_1000_2913(ctx, i_var9, param_3, param_4);
    pc_var10 = str_op_1000_28dc(ctx, 0x0);
    if pc_var10 != 0x0 {
        i_var9 = 0x9;
        if *pc_var10 == 'M' {
            i_var9 = 0xf;
        }
        pc_var10 = pc_var10 + i_var9;
        i_var9 = 0x22;
        pc_var8 = pc_var10;
        loop {
            if i_var9 == 0x0 {
                break;
            }
            i_var9 += -0x1;
            pc_var3 = &pc_var8;
            pc_var8 = pc_var8[1..].to_string();
            if *pc_var3 == '\r' {
                break;
            }
        }
        pc_var8[-0x1] = '\0';
    }
    FatalAppExit16(param_4, pc_var10);
    FatalExit();
    pi_var6 = ctx.PTR_LOOP_1050_63fe;
    loop {
        pi_var2 = pi_var6;
        pi_var6 = pi_var6 + 0x1;
        i_var9 = *pi_var2;
        pi_var7 = pi_var6;
        pi_var7 = (i_var9 + 0x1) as u32;
        if (i_var9 == param_1) || (pi_var7 == 0x0) {
            return pi_var7;
        }
        i_var9 = -0x1;
        loop {
            if i_var9 == 0x0 {
                break;
            }
            i_var9 += -0x1;
            pi_var2 = pi_var6;
            pi_var6 = (pi_var6 + 0x1);
            if *pi_var2 == '\0' {
                break;
            }
        }
    }
}

// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING (jumptable): Heritage AFTER dead removal. Example location: r0x10505fc2 :
// 0x1000270c
// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: ram

pub unsafe fn pass1_1000_262c(
    ctx: &mut Globals,
    param_1: u16,
    param_2: u16,
    param_3: &String,
    param_4: HINSTANCE16,
    in_dx: u16,
    unaff_es: u16,
    stack0x0004: u16,
    stack0xfffa: u16,
) {
    let mut pc_var1: &String;
    let c_var2: u8;
    let u_var3: u16;
    let pu_var4: U32Ptr;
    let ivar5: i16;
    let mut u_var6: String;
    let u_var7: u16;
    let u_var8: u16;
    // let in_dx: U32Ptr;
    let i_var9: i16;
    // char * *ppc_var10;
    let mut ppc_var10: Vec<String>;
    let mut pc_var11: String;
    let mut pc_var12: String;
    let mut pc_var13: String;
    // let unaff_es: u16;
    let u_var14: u16;
    let pu_stack4: U32Ptr;
    let mut p_cstack2: &mut String;

    ctx.PTR_LOOP_1050_5fd2 = *param_1 as u32;
    ctx.PTR_LOOP_1050_5fd4 = *param_2 as u32;
    *param_2 = 0x263d;
    *param_1 = pass1_1000_2950(ctx, 0x8, *in_dx, unaff_es, param_4) as u16;
    p_cstack2 = read_string_from_addr(ctx.PTR_LOOP_1050_5f4c);
    pu_stack4 = *in_dx as u32;
    ctx.PTR_LOOP_1050_5fc2 = *param_1 as u32;
    ctx.PTR_LOOP_1050_5fc4 = *in_dx as u32;
    ivar5 = GetModuleFileName16(
        param_4,
        (&ctx.s_You_may_not_run_a_turn__The_game_1050_00df[25..].to_string()),
        *param_1 as i16,
    );
    pu_stack4[ivar5] = 0x0;
    i_var9 = 0x1;
    ctx.PTR_LOOP_1050_5fb8 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
    pc_var11 = (ctx.s_New_failed_in_Op__Op__DialogHand_1050_0073[0xe..].to_string());
    //LAB_1000_266c:
    loop {
        loop {
            pc_var1 = &pc_var11;
            pc_var11 = pc_var11[1..].to_string();
            c_var2 = pc_var1[0] as u8;
            if c_var2 != ' ' as u8 {
                break;
            }
        }
        if c_var2 != '\t' as u8 {
            break;
        }
    }
    if (c_var2 != '\r' as u8) && (c_var2 != '\0' as u8) {
        ctx.PTR_LOOP_1050_5fb8 = ctx.PTR_LOOP_1050_5fb8 + 0x1;
        loop {
            // pc_var11 = pc_var11 + -0x1;
            //LAB_1000_267f:
            pc_var1 = &pc_var11;
            pc_var11 = pc_var11[1..].to_string();
            c_var2 = pc_var1[0];
            if (c_var2 == ' ' as u8) || (c_var2 == '\t' as u8) {
                // goto LAB_1000_266c;
            }
            if (c_var2 == '\r' as u8) || (c_var2 == '\0' as u8) {
                break;
            }
            if c_var2 == '\"' as u8 {
                //LAB_1000_26b8:
                loop {
                    loop {
                        loop {
                            pc_var1 = &pc_var11;
                            pc_var11 = pc_var11[1..].to_string();
                            c_var2 = pc_var1[0];
                            if (c_var2 == '\r' as u8) || (c_var2 == '\0' as u8) {
                                // goto LAB_1000_26e8;
                            }
                            if c_var2 == '\"' as u8 {
                                // goto LAB_1000_267f;
                            }
                            if c_var2 == '\\' as u8 {
                                break;
                            }
                            i_var9 += 0x1;
                        }
                        u_var7 = 0x0;
                        loop {
                            pc_var13 = pc_var11;
                            u_var7 += 0x1;
                            pc_var11 = pc_var13[1..].to_string();
                            c_var2 = pc_var13[0];
                            if c_var2 == '\\' as u8 {
                                break;
                            }
                        }
                        if c_var2 == '\"' as u8 {
                            break;
                        }
                        i_var9 += u_var7;
                        pc_var11 = pc_var13;
                    }
                    i_var9 = i_var9 + (u_var7 >> 0x1) + ((u_var7 & 0x1) != 0x0);
                    if (u_var7 & 0x1) == 0 {
                        break;
                    }
                }
                // TODO: goto LAB_1000_267f;
            }
            if c_var2 != '\\' as u8 {
                i_var9 += 0x1;
                // TODO: goto LAB_1000_267f;
            }
            u_var7 = 0x0;
            loop {
                u_var7 += 0x1;
                pc_var1 = &pc_var11;
                pc_var11 = pc_var11[1..].to_string();
                c_var2 = pc_var1[0];
                if c_var2 == '\\' as u8 {
                    break;
                }
            }
            if c_var2 == '\"' as u8 {
                i_var9 = i_var9 + (u_var7 >> 0x1) + ((u_var7 & 0x1) != 0x0);
                if (u_var7 & 0x1) == 0x0 {
                    // TODO: goto LAB_1000_26b8;
                }
                // TODO: goto LAB_1000_267f;
            }
            i_var9 += u_var7;
        }
    }
    //LAB_1000_26e8:
    *p_cstack2 = read_string_from_rsrc(ctx.data_seg);
    i_var9 = -((ctx.PTR_LOOP_1050_5fb8 + (ctx.PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + i_var9 + 0x1)
        & 0xfffe) as i16;
    ctx.PTR_LOOP_1050_5fba = (&param_1 + i_var9);
    pc_var13 = (&param_1 + (ctx.PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + i_var9);
    ctx.PTR_LOOP_1050_5fbc = param_3.clone();
    (&p_cstack2 + i_var9) = param_3;
    pu_var4 = ctx.PTR_LOOP_1050_5fc4;
    u_var14 = (&p_cstack2 + i_var9);
    (&param_1 + i_var9) = ctx.PTR_LOOP_1050_5fc2;
    (&param_2 + i_var9) = pu_var4;
    ppc_var10 = (&stack0x0004 + i_var9);
    (&p_cstack2 + i_var9) = (&param_1 + i_var9);
    (&pu_stack4 + i_var9) = ctx.s_tile2_bmp_1050_1538;
    (&stack0xfffa + i_var9) = 0x271f;
    u_var6 = pass1_1000_29dc(param_3);
    u_var3 = ctx.PTR_LOOP_1050_5f7e as u16;
    pc_var11 = (ctx.s_New_failed_in_Op__Op__DialogHand_1050_0073[0xe..].to_string());
    //LAB_1000_272e:
    loop {
        loop {
            pc_var1 = &pc_var11;
            pc_var11 = pc_var11[1..].to_string();
            c_var2 = pc_var1[0];
            if c_var2 != ' ' as u8 {
                break;
            }
        }
        if c_var2 == '\t' as u8 {
            break;
        }
    }
    if (c_var2 == '\r' as u8) || (c_var2 == '\0' as u8) {
        //LAB_1000_27c1:
        (&p_cstack2 + i_var9) = ctx.s_tile2_bmp_1050_1538;
        (&pu_stack4 + i_var9) = 0x27c5;
        u_var6 = pass1_1000_29dc(param_3);
        // *ppc_var10 = 0x0;
        // ppc_var10[0x1] = 0x0;
        // WARNING: Could not recover jumptable at 0x100027d2. Too many
        // branches
        // WARNING: Treating indirect jump as call
        (*&ctx.PTR_LOOP_1050_5fd2)();
        ctx.PTR_LOOP_1050_5fc2 =
            CONCAT22(ctx.PTR_LOOP_1050_5fc4 as u16, ctx.PTR_LOOP_1050_5fc2 as u16);
        return;
    }
    ppc_var10[0] = pc_var13.clone();
    ppc_var10[0x1] = param_3.clone();
    // ppc_var10 = ppc_var10 + 0x2;
    loop {
        // pc_var11 = pc_var11 + -0x1;
        //LAB_1000_274f:
        pc_var1 = &pc_var11;
        pc_var11 = pc_var11[1..].to_string();
        c_var2 = pc_var1[0];
        if (c_var2 == ' ' as u8) || (c_var2 == '\t' as u8) {
            pc_var1 = &pc_var13;
            pc_var13 = pc_var13[1..].to_string();
            pc_var1[0] = '\0';
            //       TODO: goto LAB_1000_272e;
        }
        if (c_var2 == '\r' as u8) || (c_var2 == '\0' as u8) {
            //LAB_1000_27be:
            pc_var13[0] = '\0';
            //       TODO: goto LAB_1000_27c1;
        }
        pc_var12 = pc_var11;
        if c_var2 == '\"' as u8 {
            //LAB_1000_278b:
            loop {
                pc_var11 = pc_var12[1..].to_string();
                c_var2 = pc_var12[0];
                if ((c_var2 == '\r' as u8) || (c_var2 == '\0' as u8)) {
                    // goto LAB_1000_27be;
                }
                if (c_var2 == '\"' as u8) {
                    break;
                }
                if (c_var2 == '\\' as u8) {
                    u_var7 = 0x0;
                    loop {
                        pc_var12 = pc_var11;
                        u_var7 += 0x1;
                        pc_var11 = pc_var12[1..].to_string();
                        c_var2 = pc_var12[0];
                        if c_var2 != '\\' as u8 {
                            break;
                        }
                    }
                    if (c_var2 == '\"' as u8) {
                        // TODO: refactor for loop
                        // for (u_var8 = u_var7 >> 0x1; u_var8 != 0x0; u_var8 -= 0x1) {
                        //   pc_var1 = pc_var13;
                        //   pc_var13 = pc_var13 + 0x1;
                        //   *pc_var1 = '\\';
                        // }
                        if (u_var7 & 0x1) == 0x0 {
                            break;
                        }
                        pc_var1 = &pc_var13;
                        pc_var13 = pc_var13[1..].to_string();
                        pc_var1[0] = '\"' as u8;
                        pc_var12 = pc_var11;
                    } else {
                        // TODO: refactor for loop
                        // for (; u_var7 != 0x0; u_var7 -= 0x1) {
                        //   pc_var1 = pc_var13;
                        //   pc_var13 = pc_var13 + 0x1;
                        //   *pc_var1 = '\\';
                        // }
                    }
                } else {
                    pc_var1 = &pc_var13;
                    pc_var13 = pc_var13[1..].to_string();
                    pc_var1[0] = c_var2;
                    pc_var12 = pc_var11;
                }
            }
            //       TODO: goto LAB_1000_274f;
        }
        if c_var2 != '\\' as u8 {
            pc_var1 = &pc_var13;
            pc_var13 = pc_var13[1..].to_string();
            pc_var1[0] = c_var2;
            //       TODO: goto LAB_1000_274f;
        }
        u_var7 = 0x0;
        loop {
            u_var7 += 0x1;
            pc_var1 = &pc_var11;
            pc_var11 = pc_var11[1..].to_string();
            c_var2 = pc_var1[0];
            if c_var2 != '\\' as u8 {
                break;
            }
        }
        if (c_var2 == '\"' as u8) {
            // TODO: refactor for loop
            // for (u_var8 = u_var7 >> 0x1; u_var8 != 0x0; u_var8 -= 0x1) {
            //   pc_var1 = pc_var13;
            //   pc_var13 = pc_var13 + 0x1;
            //   *pc_var1 = '\\';
            // }
            pc_var12 = pc_var11;
            if ((u_var7 & 0x1) == 0x0) {
                // goto LAB_1000_278b;
            }
            pc_var1 = &pc_var13;
            pc_var13 = pc_var13[1..].to_string();
            pc_var1[0] = '\"' as u8;
            //       TODO: goto LAB_1000_274f;
        }
        // TODO: refactor for loop
        // for (; u_var7 != 0x0; u_var7 -= 0x1) {
        //   pc_var1 = pc_var13;
        //   pc_var13 = pc_var13 + 0x1;
        //   *pc_var1 = '\\';
        // }
    }
}

pub unsafe fn pass1_1000_27d6(ctx: &mut Globals, param_1: u32) {
    let pi_var1: U32Ptr;
    let mut pc_var2: String;
    let pu_var3: U32Ptr;
    let mut pi_var4: &mut String;
    let c_var5: u8;
    let svar6: SEGPTR;
    let pu_var7: U32Ptr;
    let ppu_var8: U32Ptr;
    let i_var9: i16;
    let u_var10: u16;
    let pu_var11: U32Ptr;
    let i_var12: i16;
    let pi_var13: U32Ptr;
    let pi_var14: U32Ptr;
    let mut pc_var15: String = String::new();
    let mut pi_var16: &mut String;
    let b_var17: bool;
    let pu_var18: U32Ptr;

    svar6 = GetDOSEnvironment16();
    if svar6 != 0x0 {
        *param_1 = 0x0;
    }
    i_var12 = 0x0;
    // pc_var15 = 0x0;
    i_var9 = -0x1;
    if param_1 != 0x0 {
        c_var5 = *0x0;
        while c_var5 != '\0' as u8 {
            loop {
                if i_var9 == 0x0 {
                    break;
                }
                i_var9 += -0x1;
                pc_var2 = pc_var15;
                pc_var15 = pc_var15[1..].to_string();
                if *pc_var2 == '\0' {
                    break;
                }
            }
            i_var12 += 0x1;
            pc_var2 = pc_var15;
            pc_var15 = pc_var15[1..].to_string();
            c_var5 = pc_var2[0];
        }
    }
    u_var10 = 0x9;
    pu_var11 = *param_1 as u32;
    pu_var7 = pass1_1000_2950(
        ctx,
        0x9,
        *param_1,
        *param_1,
        ctx.s_tile2_bmp_1050_1538 as u16,
    );
    pu_var18 = pu_var11;
    ppu_var8 = pass1_1000_2950(
        ctx,
        u_var10 as i16,
        pu_var11 as u16,
        *param_1,
        ctx.s_tile2_bmp_1050_1538 as u16,
    );
    pi_var13 = 0x0;
    ctx.PTR_LOOP_1050_5fbe = ppu_var8;
    ctx.PTR_LOOP_1050_5fc0 = pu_var11;
    loop {
        if i_var12 == 0x0 {
            *ppu_var8 = 0x0;
            ppu_var8[0x1] = 0x0;
            return;
        }
        b_var17 = *pi_var13 == ctx.s__C_FILE_INFO__1050_5f5c._0_2_;
        if b_var17 {
            pi_var16 = &mut ctx.s__C_FILE_INFO__1050_5f5c;
            i_var9 = 0x6;
            pi_var14 = pi_var13;
            loop {
                if i_var9 == 0x0 {
                    break;
                }
                i_var9 += -0x1;
                pi_var4 = pi_var16;
                pi_var16 = &mut pi_var16[1..].to_string();
                pi_var1 = pi_var14;
                pi_var14 = pi_var14 + 0x1;
                b_var17 = *pi_var1 == *pi_var4;
                if b_var17 == false {
                    break;
                }
            }
            if !b_var17 {
                // goto LAB_1000_2867;
            }
        } else {
            //LAB_1000_2867:
            *ppu_var8 = pu_var7;
            ppu_var8[0x1] = pu_var18;
            ppu_var8 = ppu_var8 + 0x2;
        }
        loop {
            pi_var1 = pi_var13;
            pi_var13 = (pi_var13 + 0x1);
            c_var5 = *pi_var1 as u8;
            pu_var3 = pu_var7;
            pu_var7 = (pu_var7 + 0x1);
            *pu_var3 = c_var5 as u16;
            if c_var5 == '\0' as u8 {
                break;
            }
        }
        i_var12 += -0x1;
    }
}

pub fn pass1_1000_2913(ctx: &mut Globals, param_1: i16, param_2: u16, param_3: u16) {
    let mut string_1: String;
    let mut string_2: String;
    let i_var3: i16;

    if ctx.PTR_LOOP_1050_61ec != 0x0 {
        string_2 = str_op_1000_28dc(ctx, param_1);
        if string_2 != 0x0 {
            i_var3 = -0x1;
            loop {
                if i_var3 == 0x0 {
                    break;
                }
                i_var3 += -0x1;
                string_1 = string_2;
                string_2 = string_2[1..].to_string();
                if *string_1 == '\0' {
                    break;
                }
            }
            pass1_1000_55b1(ctx, 0x2944, param_2, param_3);
        }
    }
    return;
}

pub unsafe fn pass1_1000_2950(
    ctx: &mut Globals,
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
) -> U32Ptr {
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let mut pc_var3: String;
    let pu_var4: U32Ptr;
    let mut string_a: String;
    let i_var5: i16;
    let pu_var6: U32Ptr;
    let in_ax: u16;
    let mut pu_var7: U32Ptr = 0;
    let mut pc_var8: String;
    let u_var9: u16;
    let pcVar: String;

    pu_var4 = ctx.PTR_LOOP_1050_6066;
    ctx.PTR_LOOP_1050_6066 = ctx.PTR_LOOP_1050_1000;
    u_var9 = param_3;
    // pu_var7 = (fn mem_1000_167a(in_AX, param_4, param_2) -> u16;
    ctx.PTR_LOOP_1050_6066 = pu_var4;
    if (param_2 | pu_var7) != 0x0 {
        return pu_var7;
    }
    i_var5 = param_1;
    pass1_1000_25a8(ctx, param_3, param_4);
    pass1_1000_2913(ctx, param_1, param_3, param_4);
    string_a = str_op_1000_28dc(ctx, i_var5);
    if string_a != 0x0 {
        i_var5 = 0x9;
        if *string_a == 'M' {
            i_var5 = 0xf;
        }
        string_a = string_a[i_var5..];
        i_var5 = 0x22;
        pc_var8 = string_a;
        loop {
            if i_var5 == 0x0 {
                break;
            }
            i_var5 += -0x1;
            pc_var3 = pc_var8;
            pc_var8 = pc_var8[1..].to_string();
            if *pcVar == '\r' {
                break;
            }
        }
        pc_var8[-0x1] = '\0';
    }
    FatalAppExit16(param_4, &string_a);
    FatalExit();
    pu_var7 = ctx.PTR_LOOP_1050_63fe;
    loop {
        pu_var1 = pu_var7;
        pu_var7 = pu_var7 + 0x1;
        u_var2 = *pu_var1;
        pu_var6 = pu_var7;
        pu_var6 = (u_var2 + 0x1) as u32;
        if (u_var2 == u_var9) || (pu_var6 == 0x0) {
            return pu_var6;
        }
        i_var5 = -0x1;
        loop {
            if i_var5 == 0x0 {
                break;
            }
            i_var5 += -0x1;
            pu_var1 = pu_var7;
            pu_var7 = (pu_var7 + 0x1);
            if *pu_var1 == '\0' {
                break;
            }
        }
    }
}

pub fn pass1_1000_29af(ctx: &mut Globals, param_1: u16) {
    pass1_1000_29b5(ctx, &mut (param_1 & 0xff));
    return;
}

pub fn pass1_1000_29b5(ctx: &mut Globals, param_1: &mut u16) {
    let c_var1: char;

    ctx.PTR_LOOP_1050_5f88 = *param_1 as u32;
    c_var1 = (param_1 >> 0x8);
    if c_var1 != '\0' {
        // goto
        // LAB_1000_29d2;
    }
    if ctx.PTR_LOOP_1050_5f88 < 0x22 {
        if ctx.PTR_LOOP_1050_5f88 < 0x20 {
            if 0x13 < ctx.PTR_LOOP_1050_5f88 {
                // goto
                // LAB_1000_29cc;
            }
        } else {
            *param_1 = 0x5;
        }
    } else {
        //LAB_1000_29cc:
        *param_1 = 0x13;
    }
    c_var1 = *((param_1 & 0xff) + 0x5fd6);
    //LAB_1000_29d2:
    ctx.PTR_LOOP_1050_5f78 = u32::from(c_var1);
    return;
}

pub fn pass1_1000_29dc(ctx: &mut Globals, param_1: &String) -> String {
    if ctx.___EXPORTEDSTUB != (0xb8) {
        return param_1;
    }
    return ctx.uRam100029ed;
}

pub unsafe fn pass1_1000_2a00(
    ctx: &mut Globals,
    param_1: &mut Struct_1000_2cb0,
    param_2: i16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u8,
) -> u16 {
    let b_var1: bool;
    let pi_var2: String;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let pu_stack20: U32Ptr;
    let local_10: u8 = 0;
    let u_stack15: u8 = 0;
    // let local_e: [u8; 8];
    let mut local_e: U32Ptr = 0;
    let mut u_stack6: u16;
    let local_4: u16;
    let i_stack2: i16;

    i_stack2 = param_2 + 0x1;
    local_4 = SUB42(ctx.data_seg, 0x0) as u16;
    u_var5 = 0xffff;
    if ((param_1.field_0x5) & 0x40) != 0x0 {
        *(param_1.field_0x5) = 0x0;
        return 0xffff;
    }
    if ((param_1.field_0x5) & 0x83) == 0x0 {
        // goto
        // LAB_1000_2af2;
    }
    u_var5 = pass1_1000_2fa4(param_1, param_3, param_4, param_5, param_6);
    u_stack6 = param_1[0x7a];
    struct_1008::struct_1000_2cb0(param_1, param_4);
    if ctx.DAT_1050_5f8a < (param_1.field_0xb) as u32 {
        pi_var2 = pass1_1000_55b1(ctx, 0x2a63, param_3, param_4);
        // if pi_var2 < 0x0 {
        //     // goto
        //     // LAB_1000_2a6a;
        // }
        //LAB_1000_2a82:
        b_var1 = false;
    } else {
        i_var3 = dos3_call_op_1000_35fe((param_1.field_0xb), i_stack2) as i16;
        if -0x1 < i_var3 {
            // goto
            // LAB_1000_2a82;
        }
        //LAB_1000_2a6a:
        b_var1 = true;
    }
    if !b_var1 {
        if u_stack6 == 0x0 {
            // goto
            // LAB_1000_2af2;
        }
        string_1000_3d3e(
            read_string_from_addr(CONCAT22(param_5, local_10 as u16)),
            read_string_from_addr(0x10505fea),
        );
        pu_stack20 = local_e;
        if local_10 == '\\' as u8 {
            pu_stack20 = u_stack15 as u32;
        } else {
            string_1000::string_1000_3cea(
                read_string_from_addr(CONCAT22(param_5, local_10 as u16)),
                read_string_from_addr(0x10505fec),
            );
        }
        pass1_1000_3e82(&mut u_stack6, CONCAT22(param_5, pu_stack20 as u16), 0xa);
        u_var4 = dos3_call_1000_514e(i_stack2);
        if u_var4 == 0x0 {
            // goto
            // LAB_1000_2af2;
        }
    }
    u_var5 = 0xffff;
    //LAB_1000_2af2:
    *(param_1 + 0x5) = 0x0;
    return u_var5;
}

pub unsafe fn pass1_1000_2b02(
    ctx: &mut Globals,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u8,
    param_6: u16,
    param_7: i16,
) -> U32Ptr {
    let mut pu_var1: &mut u16 = &mut 0;
    let i_stack2: i16;

    i_stack2 = param_7 + 0x1;
    *pu_var1 = pass1_1000_35aa(ctx);
    if (param_6 | pu_var1) == 0x0 {
        *pu_var1 = 0x0;
    } else {
        pu_var1 = &mut pass1_1000_2d34(
            ctx,
            param_1,
            param_2,
            CONCAT22(param_4, param_3),
            param_5,
            pu_var1,
            i_stack2,
        );
    }
    return *pu_var1 as u32;
}

pub unsafe fn pass1_1000_2b3c(
    ctx: &mut Globals,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: i16,
) {
    let i_stack2: i16;

    i_stack2 = param_6 + 0x1;
    pass1_1000_2b02(
        ctx, param_1, param_2, param_3, param_4, 0x0, param_5, i_stack2,
    );
    return;
}

pub fn pass1_1000_2b5c(
    ctx: &mut Globals,
    param_1: &mut Struct_1000_2cb0,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: i16,
    param_7: u16,
    param_8: u16,
    in_af: u8,
) -> u16 {
    let u_var1: u16;
    let u_var2: u16;
    let i_stack2: i16;

    i_stack2 = param_6 + 0x1;
    u_var1 = pass1_1000_2e74(ctx, param_1, param_7);
    u_var2 = sys_1000_30b4(
        ctx,
        param_1,
        ctx.data_seg,
        CONCAT22(param_4, param_3),
        i_stack2,
        param_1,
        param_5,
        param_7,
        param_8,
    );
    pass1_1000_2f00(u_var1 as i16, param_1, param_5, param_7, param_8, in_af);
    return u_var2;
}

pub fn pass1_1000_2ba0(
    ctx: &mut Globals,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u8,
    stack0xfffe: u16,
) {
    pass1_1000_3024(param_1, param_2, param_3, param_4);
    if ctx.PTR_LOOP_1050_5fc9 != '\0' {
        pass1_1000_3f5c(ctx, stack0xfffe as i16, param_1, param_2, param_3, param_4);
    }
    return;
}

pub fn pass1_1000_2d34(
    ctx: &mut Globals,
    param_1: u16,
    param_2: u16,
    param_3: U32Ptr,
    param_4: u8,
    param_5: &mut u16,
    param_6: i16,
) -> u16 {
    let b_var1: u8;
    let b_var2: bool;
    let b_var3: bool;
    let u_var4: u16;
    let u_stack14: u8;
    let b_stack8: u8;
    let u_stack6: u8;
    let i_stack2: i16;

    i_stack2 = param_6 + 0x1;
    b_stack8 = ctx.PTR_LOOP_1050_6062 as u8;
    b_var3 = false;
    b_var1 = param_3 as u8;
    if b_var1 == 0x77 {
        u_var4 = 0x301;
    } else {
        if 0x77 < b_var1 {
            return 0x0;
        }
        if b_var1 != 0x61 {
            if b_var1 != 0x72 {
                return 0x0;
            }
            u_var4 = 0x0;
            u_stack6 = 0x1;
            //       TODO: goto LAB_1000_2d6c;
        }
        u_var4 = 0x109;
    }
    u_stack6 = 0x2;
    //LAB_1000_2d6c:
    b_var2 = true;
    //LAB_1000_2d71:
    *param_3 = (*param_3 & 0xffff0000 | (*param_3 + 0x1));
    if (*param_3 == 0x0) || (!b_var2) {
        u_var4 = mixed_dos3_call_1000_370a(ctx, param_1, param_2, u_var4, param_4, 0x1a4, i_stack2);
        if u_var4 < 0x0 {
            return 0x0;
        }
        ctx.PTR_LOOP_1050_5fee = ctx.PTR_LOOP_1050_5fee + 0x1;
        *(param_5 + 0x5) = u_stack6;
        param_5[0x1] = 0x0;
        *param_5 = 0x0;
        param_5[0x4] = 0x0;
        param_5[0x3] = 0x0;
        u_stack14 = u_var4 as u8;
        *(param_5 + 0xb) = u_stack14;
        (param_5 + 0x78) = b_stack8;
        param_5[0x2] = 0x0;
        param_5[0x7a] = 0x0;
        return *param_5;
    }
    b_var1 = *param_3;
    if b_var1 == 0x74 {
        if (u_var4 & 0xc000) == 0x0 {
            u_var4 |= 0x4000;
            //       TODO: goto LAB_1000_2d71;
        }
    } else {
        if 0x74 < b_var1 {
            // goto
            // LAB_1000_2da4;
        }
        if b_var1 == 0x2b {
            if (u_var4 & 0x2) != 0x0 {
                // goto
                // LAB_1000_2da4;
            }
            u_var4 = u_var4 & 0xfffe | 0x2;
            u_stack6 = 0x80;
            //       TODO: goto LAB_1000_2d71;
        }
        if b_var1 == 0x62 {
            if (u_var4 & 0xc000) == 0x0 {
                u_var4 |= 0x8000;
                //         TODO: goto LAB_1000_2d71;
            }
        } else {
            if b_var1 != 0x63 {
                if (b_var1 != 0x6e) || (b_var3) {
                    // goto
                    // LAB_1000_2da4;
                }
                b_var3 = true;
                b_stack8 &= 0xbf;
                //         TODO: goto LAB_1000_2d71;
            }
            if !b_var3 {
                b_var3 = true;
                b_stack8 |= 0x40;
                //         TODO: goto LAB_1000_2d71;
            }
        }
    }
    //LAB_1000_2da4:
    b_var2 = false;
    //   TODO: goto LAB_1000_2d71;
    return 0;
}

pub fn pass1_1000_2e74(ctx: &mut Globals, param_1: &mut Struct_1000_2cb0, param_2: u16) -> u16 {
    let pu_var1: U32Ptr;
    let u_var2: U32Ptr;
    let u_var3: u16;
    let pu_var4: U32Ptr;
    let pu_var5: U32Ptr;

    if ctx.PTR_LOOP_1050_61ec != 0x0 {
        pu_var5 = param_1 + 0x78;
        pu_var4 = 0x5ff2;
        pu_var4 = ctx.PTR_LOOP_1050_5ff6;
        if (param_1 == 0x621c) || (param_1 == 0x6228) {
            if (((param_1 + 0x5) & 0xc) == 0x0) && ((pu_var5 & 0x1) == 0x0) {
                u_var2 = *pu_var4;
                u_var3 = pu_var4[0x1];
                if (u_var2 | u_var3) == 0x0 {
                    u_var2 = mem_1000_167a(ctx, 0x200, param_2, u_var3) as u32;
                    if u_var3 == 0x0 {
                        return 0x0;
                    }
                    *pu_var4 = u_var2;
                    pu_var4[0x1] = u_var3;
                }
                param_1.field_0x3 = read_struct_from_addr::<Struct18>(u_var2).clone();
                param_1.field_0x4 = u_var3;
                param_1.field_0x0 = u_var2 as u16;
                param_1.field_0x0 = u_var3;
                param_1.field_0x2 = 0x200;
                param_1.field_0x79 = 0x200;
                pu_var1 = param_1.field_0x5 as u32;
                pu_var1 = pu_var1 | 0x2;
                pu_var5 = 0x11;
                return 0x1;
            }
        } else {
            if ctx.DAT_1050_5f8a <= (param_1.field_0xb) as u32 {
                pu_var1 = pu_var5;
                pu_var1 = pu_var1 | 0x10;
            }
        }
    }
    return 0x0;
}

pub fn pass1_1000_2f00(
    param_1: i16,
    param_2: &mut Struct_1000_2cb0,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u8,
) {
    if (((param_2 + 0x78) & 0x10) != 0x0) && ((((param_2 + 0xb) + 0x5f90) & 0x40) != 0x0) {
        pass1_1000_2fa4(param_2, param_3, param_4, param_5, param_6);
        if param_1 != 0x0 {
            (param_2.field_0x78) = 0x0;
            param_2.field_0x79 = 0x0;
            param_2.field_0x0 = 0x0;
            param_2.field_0x1 = 0x0;
            // param_2.field_0x3 = 0x0;
            param_2.field_0x4 = 0x0;
        }
    }
    return;
}

pub fn pass1_1000_2f48(
    ctx: &mut Globals,
    param_1: &mut Struct_1000_2cb0,
    param_2: i16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u8,
) -> u16 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let i_stack2: i16;

    i_stack2 = param_2 + 0x1;
    if param_1 == 0x0 {
        u_var1 = pass1_1000_3038(0x0, param_3, param_4, param_5, param_6) as u16;
    } else {
        u_var1 = pass1_1000_2fa4(param_1, param_3, param_4, param_5, param_6);
        if u_var1 == 0x0 {
            if ((param_1.field_0x78) & 0x40) != 0x0 {
                pu_var2 = pass1_1000_400a(ctx, (param_1.field_0xb as i16), i_stack2 as u16);
                // u_var1 = !(pu_var2 != 0);
            }
        } else {
            u_var1 = 0xffff;
        }
    }
    return u_var1;
}

pub fn pass1_1000_2fa4(
    ctx: &mut Globals,
    param_1: &mut Struct_1000_2cb0,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u8,
) -> u16 {
    let pi_var1: U32Ptr;
    let b_var2: u8;
    let i_var3: i16;
    let pu_var4: U32Ptr;
    let pu_var5: U32Ptr;
    let u_var6: u16;

    u_var6 = 0x0;
    b_var2 = (param_1 + 0x5);
    if ((b_var2 & 0x3) == 0x2) && ((b_var2 & 0x8) != 0x0 || (((param_1 + 0x78) & 0x1) != 0x0)) {
        pu_var4 = (param_1.field_0x0 - param_1.field_0x3.field_0x0) as u32;
        if 0x0 < pu_var4 {
            pu_var5 = mixed_dos3_call_1000_39f2(
                ctx,
                param_1.field_0xb as u32,
                read_string_from_addr(CONCAT22(param_1.field_0x4, param_1.field_0x3.field_0x0)),
                pu_var4,
                param_2,
                param_3,
                param_4,
                param_5,
            );
            if pu_var5 == pu_var4 {
                if ((param_1 + 0x5) & 0x80) != 0x0 {
                    pi_var1 = param_1.field_0x5 as u32;
                    pi_var1 = pi_var1 & 0xfd;
                }
            } else {
                pi_var1 = param_1.field_0x5 as u32;
                pi_var1 = pi_var1 | 0x20;
                u_var6 = 0xffff;
            }
        }
    }
    i_var3 = param_1.field_0x4 as i16;
    param_1.field_0x0 = param_1.field_0x3.field_0x0;
    param_1.field_0x1 = i_var3 as u16;
    param_1.field_0x2 = 0x0;
    return u_var6;
}

pub fn pass1_1000_3024(param_1: u16, param_2: u16, param_3: u16, param_4: u8) {
    pass1_1000_3038(0x1, param_1, param_2, param_3, param_4);
    return;
}

pub fn pass1_1000_3038(param_1: i16, param_2: u16, param_3: u16, param_4: u16, param_5: u8) -> i16 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let i_var3: i16;
    let i_stack4: i16;

    i_var3 = 0x0;
    i_stack4 = 0x0;
    // TODO: refactor for loop
    //   for (pu_var2 = &ctx.PTR_LOOP_1050_6210; pu_var2 <= ctx.PTR_LOOP_1050_5ff0;
    //       pu_var2 = pu_var2 + 0xc) {
    //     if ((param_1 == 0x1) && ((pu_var2[0xa] & 0x83) != 0x0)) {
    //       u_var1 = pass1_1000_2f48(CONCAT22(0x1050,pu_var2),&stack0xfffe,param_2,param_3,
    //                               param_4,param_5);
    //       if (u_var1 != 0xffff) {
    //         i_var3 += 0x1;
    //       }
    //     }
    //     else {
    //       if ((param_1 == 0x0) &&
    //          (((pu_var2[0xa] & 0x2) != 0x0 &&
    //           (u_var1 = pass1_1000_2f48(CONCAT22(0x1050,pu_var2),&stack0xfffe,param_2,
    //                                    param_3,param_4,param_5), u_var1 == 0xffff)))) {
    //         i_stack4 = -0x1;
    //       }
    //     }
    //   }
    if param_1 == 0x1 {
        i_stack4 = i_var3;
    }
    return i_stack4;
}

// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING: Unable to track spacebase fully for stack

pub unsafe fn pass1_1000_30a4(
    ctx: &mut Globals,
    param_1: U32Ptr,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
    param_6: u16,
    param_7: u16,
    param_8: u16,
    param_9: &String,
    param_10: u8,
) -> u16 {
    let pu_var1: U32Ptr;
    let c_var2: u8;
    let mut pc_var3: String;
    let b_var4: u8;
    let u_var5: u16;
    let pu_var6: U32Ptr;

    pu_var6 = (param_5 + (param_3 + param_6) + param_10) as u32;
    pu_var1 = pu_var6;
    *pu_var1 = *pu_var1 ^ pu_var6;
    pu_var1 = (pu_var6 + param_3 + 0x31);
    *pu_var1 = *pu_var1 ^ param_4;
    pu_var1 = (pu_var6 + -0x3acf);
    *pu_var1 = *pu_var1 ^ param_3;
    pu_var1 = pu_var6 + -0x3794;
    *pu_var1 = *pu_var1 ^ param_2;
    (param_1 + -0x2) = (param_4 + 0x1) as i16 as u32;
    (param_1 + -0x4) = ctx.data_seg;
    (param_1 + -0x6) = param_8 as u32;
    (param_1 + -0x8) = 0x30c5;
    exit_1000_25f2(
        ctx,
        ((param_1 + -0x8) as u16),
        ((param_1 + -0x6) as u16),
        ((param_1 + -0x4) as i16),
        0x214,
        param_7,
        param_8,
        param_9,
    );
    (param_1 + -0x6) = pu_var6 as u32;
    (param_1 + -0x8) = (param_6 ^ pu_var6) as u32;
    (param_1 + -0xc) = 0x0;
    *(param_1 + -0x9) = 0x0;
    pc_var3 = read_string_from_addr((param_1 + 0x8) as u32).clone();
    c_var2 = pc_var3[0];
    write_string_to_addr(param_1 + 0x8, &pc_var3[1..].to_string());
    *(param_1 + -0x6) = c_var2;
    if (c_var2 != '\0' as u8) & &(-0x1 < (param_1 + -0xc)) {
        if (c_var2 - 0x20) < 0x59 {
            b_var4 = ((c_var2 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var4 = 0x0;
        }
        b_var4 = ((b_var4 * 0x8 as u8 + *(param_1 + -0x9)) + 0x5ffe) >> 0x4;
        (param_1 + -0x9) = b_var4 as i16 as u32;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var5 = (b_var4 * 0x2 + 0x30a4)();
        return u_var5;
    }
    return (param_1 + -0xc) as u16;
}

// WARNING (jumptable): Unable to track spacebase fully for stack

pub unsafe fn pass1_1000_3113(param_1: u16, param_2: u16) -> u16 {
    let c_var1: u8;
    let mut pc_var2: String;
    let b_var3: u8;
    let u_var4: u16;

    let mut a: i16 = 0x1i16;

    pass1_1000_3552(&mut a, param_1 as i16, param_2, 0, 0, 0);
    pc_var2 = read_string_from_addr((param_1 + 0xa) as u32).clone();
    c_var1 = pc_var2[0];
    write_string_to_addr(((param_1 + 0xa) as u32), &pc_var2[0x1..].to_string());
    write_string_to_addr(((param_1 - 0x4) as u32), &c_var1.to_string());
    if (c_var1 != '\0' as u8) && (-0x1 < (param_1 - 0xa)) {
        if (c_var1 - 0x20) < 0x59 {
            b_var3 = ((c_var1 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var3 = 0x0;
        }
        b_var3 = ((b_var3 * 0x8 + *(param_1 - 0x7)) + 0x5ffe) >> 0x4;
        (param_1 - 0x7) = b_var3 as u16;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var4 = (b_var3 * 0x2 + 0x30a4)();
        return u_var4;
    }
    return param_1 - 0xa;
}

// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

pub fn pass1_1000_311e(param_1: i16, param_2: u16) -> u16 {
    let c_var1: u8;
    let mut pc_var2: String;
    let b_var3: u8;
    let u_var4: u16;

    (param_1 + -0x12) = 0x0;
    (param_1 + -0xc) = 0x0;
    (param_1 + -0x14) = 0x0;
    (param_1 + -0x6) = 0x20;
    (param_1 + -0xe) = 0xffff;
    pc_var2 = read_string_from_addr(param_1 + 0xa);
    c_var1 = pc_var2[0];
    write_string_to_addr(((param_1 + 0xa) as u32), &pc_var2[1..].to_string());
    *(param_1 + -0x4) = c_var1;
    if (c_var1 != '\0' as u8) && (-0x1 < (param_1 + -0xa)) {
        if (c_var1 - 0x20) < 0x59 {
            b_var3 = ((c_var1 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var3 = 0x0;
        }
        b_var3 = ((b_var3 * 0x8 + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        (param_1 + -0x7) = b_var3 as i16;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var4 = (b_var3 * 0x2 + 0x30a4)();
        return u_var4;
    }
    return (param_1 + -0xa) as u16;
}

// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

pub unsafe fn pass1_1000_3134(param_1: i16, param_2: u16) -> u16 {
    let pb_var1: U32Ptr;
    let c_var2: u8;
    let mut pc_var3: String;
    let b_var4: u8;
    let u_var5: u16;

    c_var2 = *(param_1 + -0x4);
    if (c_var2 == '-' as u8) {
        pb_var1 = (param_1 + -0x6) as u32;
        *pb_var1 = *pb_var1 | 0x4;
    } else {
        if (c_var2 == '+' as u8) {
            pb_var1 = (param_1 + -0x6) as u32;
            *pb_var1 = *pb_var1 | 0x1;
        } else {
            if (c_var2 == ' ' as u8) {
                pb_var1 = (param_1 + -0x6) as u32;
                *pb_var1 = *pb_var1 | 0x2;
            } else {
                if (c_var2 == '#' as u8) {
                    pb_var1 = (param_1 + -0x6) as u32;
                    *pb_var1 = *pb_var1 | 0x80;
                } else {
                    pb_var1 = (param_1 + -0x6) as u32;
                    *pb_var1 = *pb_var1 | 0x8;
                }
            }
        }
    }
    pc_var3 = read_string_from_addr((param_1 + 0xa) as u32).clone();
    c_var2 = pc_var3[0];
    write_string_to_addr(((param_1 + 0xa) as u32), &pc_var3[0x1..].to_string());
    *(param_1 + -0x4) = c_var2;
    if (c_var2 != '\0' as u8) && (-0x1 < (param_1 + -0xa)) {
        if (c_var2 - 0x20) < 0x59 {
            b_var4 = ((c_var2 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var4 = 0x0;
        }
        b_var4 = ((b_var4 * 0x8 + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        (param_1 + -0x7) = b_var4 as i16;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var5 = (b_var4 * 0x2 + 0x30a4)();
        return u_var5;
    }
    return (param_1 + -0xa) as u16;
}

// WARNING (jumptable): Unable to track spacebase fully for stack

pub unsafe fn pass1_1000_3168(param_1: U32Ptr, param_2: u16) -> u16 {
    let pb_var1: U32Ptr;
    let c_var2: u8;
    let mut pc_var3: String;
    let b_var4: bool;
    let u_var5: u16;

    c_var2 = *(param_1 + -0x4);
    if c_var2 == '*' as u8 {
        u_var5 = pass1_1000_34cf(param_1, param_2);
        if u_var5 < 0x0 {
            u_var5 = -u_var5;
            pb_var1 = (param_1 + -0x6) as u32;
            *pb_var1 = *pb_var1 | 0x4;
        }
    } else {
        u_var5 = ((param_1 + -0xc) * 0xa + (c_var2 - 0x30)) as u16;
    }
    (param_1 + -0xc) = u_var5 as u32;
    pc_var3 = read_string_from_addr((param_1 + 0xa) as u32).clone();
    c_var2 = pc_var3[0];
    // (param_1 + 0xa) = pc_var3[1..].to_string();
    write_string_to_addr((param_1 + 0xa), &pc_var3[1..].to_string())(param_1 - 0x4) = c_var2;
    if (c_var2 != '\0' as u8) && (-0x1 < (param_1 + -0xa)) {
        if (c_var2 - 0x20) < 0x59 {
            b_var4 = ((c_var2 - 0x20) + 0x5ffe) & 0xf > 0;
        } else {
            b_var4 = false;
        }
        b_var4 = ((b_var4 * 0x8 + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        // (param_1 - 0x7) = b_var4;
        write_bool_to_addr((param_1 - 0x7), b_var4);
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var5 = (b_var4 * 0x2 + 0x30a4)();
        return u_var5;
    }
    return (param_1 + -0xa) as u16;
}

// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

pub fn pass1_1000_3194(param_1: i16, param_2: u16) -> u16 {
    let c_var1: u8;
    let mut pc_var2: &mut String;
    let b_var3: u8;
    let u_var4: u16;

    (param_1 + -0xe) = 0x0;
    pc_var2 = read_string_from_addr((param_1 + 0xa) as u32);
    c_var1 = pc_var2[0];
    // (param_1 + 0xa) = pc_var2[1..].to_string();
    write_string_to_addr(((param_1 + 0xa) as u32), &pc_var2[1..].to_string()) * (param_1 + -0x4) =
        c_var1;
    if (c_var1 != '\0' as u8) && (-0x1 < (param_1 + -0xa)) {
        if (c_var1 - 0x20) < 0x59 {
            b_var3 = ((c_var1 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var3 = 0x0;
        }
        b_var3 = ((b_var3 * 0x8 + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        (param_1 + -0x7) = b_var3 as i16;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var4 = (b_var3 * 0x2 + 0x30a4)();
        return u_var4;
    }
    return (param_1 + -0xa) as u16;
}

// WARNING (jumptable): Unable to track spacebase fully for stack

pub unsafe fn pass1_1000_319c(param_1: U32Ptr, param_2: u16) -> u16 {
    let c_var1: u8;
    let mut pc_var2: &mut String;
    let b_var3: bool;
    let u_var4: u16;

    c_var1 = *(param_1 + -0x4);
    if c_var1 == '*' as u8 {
        u_var4 = pass1_1000_34cf(param_1, param_2);
        if u_var4 < 0x0 {
            u_var4 = 0xffff;
        }
    } else {
        u_var4 = ((param_1 + -0xe) * 0xa + (c_var1 - 0x30)) as u16;
    }
    (param_1 - 0xe) = u_var4 as u32;
    pc_var2 = read_string_from_addr(param_1 + 0xa);
    c_var1 = pc_var2[0];
    // (param_1 + 0xa) = pc_var2[1..].to_string();
    write_string_to_addr((param_1 + 0xa), &pc_var2[1..].to_string()) * (param_1 - 0x4) = c_var1;

    if (c_var1 != '\0' as u8) && (-0x1 < (param_1 - 0xa)) {
        if (c_var1 - 0x20) < 0x59 {
            b_var3 = ((c_var1 - 0x20) + 0x5ffe) & 0xf > 0;
        } else {
            b_var3 = false;
        }
        b_var3 = ((b_var3 * 0x8 + *(param_1 - 0x7)) + 0x5ffe) >> 0x4;
        // (param_1 - 0x7) = b_var3;
        write_bool_to_addr(param_1 - 0x7, b_var3);
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var4 = (b_var3 * 0x2 + 0x30a4)();
        return u_var4;
    }
    return (param_1 + -0xa) as u16;
}

// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

pub unsafe fn pass1_1000_31c5(param_1: i16, param_2: u16) -> u16 {
    let pb_var1: U32Ptr;
    let c_var2: u8;
    let mut pc_var3: &mut String;
    let b_var4: u8;
    let u_var5: u16;

    c_var2 = *(param_1 + -0x4);
    if c_var2 == 'l' as u8 {
        pb_var1 = (param_1 - 0x6) as u32;
        *pb_var1 = *pb_var1 | 0x10;
    } else {
        if c_var2 == 'F' as u8 {
            pb_var1 = (param_1 - 0x6) as u32;
            *pb_var1 = *pb_var1 | 0x20;
        } else {
            if c_var2 == 'N' as u8 {
                pb_var1 = (param_1 - 0x5) as u32;
                *pb_var1 = *pb_var1 | 0x10;
            } else {
                if c_var2 == 'L' as u8 {
                    pb_var1 = (param_1 - 0x5) as u32;
                    *pb_var1 = *pb_var1 | 0x4;
                } else {
                    pb_var1 = (param_1 - 0x5) as u32;
                    *pb_var1 = *pb_var1 | 0x8;
                }
            }
        }
    }
    pc_var3 = read_string_from_addr((param_1 + 0xa) as u32);
    c_var2 = pc_var3[0];
    // (param_1 + 0xa) = pc_var3 + 0x1;
    write_string_to_addr(((param_1 + 0xa) as u32), &pc_var3[1..].to_string());
    // (param_1 - 0x4) = c_var2;
    if (c_var2 != '\0' as u8) && (-0x1 < (param_1 - 0xa)) {
        if (c_var2 - 0x20) < 0x59 {
            b_var4 = ((c_var2 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var4 = 0x0;
        }
        b_var4 = ((b_var4 * 0x8 + *(param_1 + -0x7)) + 0x5ffe) >> 0x4;
        (param_1 + -0x7) = b_var4 as i16;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var5 = (b_var4 * 0x2 + 0x30a4)();
        return u_var5;
    }
    return (param_1 - 0xa) as u16;
}

// WARNING (jumptable): Unable to track spacebase fully for stack

pub unsafe fn pass1_1000_31f7(
    ctx: &mut Globals,
    param_1: u16,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: i16,
    param_5: u16,
) -> u16 {
    let pi_var1: U32Ptr;
    let pb_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let c_var4: u8;
    let mut string_5: &mut String;
    let b_var6: u8;
    let u_var7: u16;
    let i_var8: i16;
    let mut i_var9: i16;
    let i_var10: i16;
    let mut pu_var11: U32Ptr = 0;
    let mut u_var12: u16 = 0;
    let mut pc_var13: &mut String;
    let mut pc_var14: &mut String;
    let b_var15: bool;
    let mut u_var16: u16;

    c_var4 = *(param_2 + -0x4);
    if (c_var4 == 'd' as u8) || (c_var4 == 'i' as u8) {
        pb_var2 = (param_2 + -0x6);
        *pb_var2 = *pb_var2 | 0x40;
        //LAB_1000_3399:
        *(param_2 + -0x8) = 0xa;
        //LAB_1000_33d4:
        if ((param_2 + -0x6) & 0x10) == 0x0 {
            u_var7 = pass1_1000_34cf(param_2, param_5);
            if ((param_2 + -0x6) & 0x40) == 0x0 {
                u_var16 = u_var7;
            } else {
                u_var16 = SEXT24(u_var7) as u16;
            }
        } else {
            u_var16 = pass1_1000_34d8(param_2, param_5) as u16;
        }
        if (((param_2 + -0x6) & 0x40) != 0x0) && (u_var16 < 0x0) {
            pb_var2 = (param_2 + -0x5);
            *pb_var2 = *pb_var2 | 0x1;
            u_var16 = CONCAT22(-((u_var16 >> 0x10) + (u_var16 != 0x0)), -u_var16) as u16;
        }
        if (param_2 + -0xe) < 0x0 {
            (param_2 + -0xe) = 0x1;
        } else {
            pb_var2 = (param_2 + -0x6);
            *pb_var2 = *pb_var2 & 0xf7;
        }
        if u_var16 == 0x0 {
            (param_2 + -0x12) = 0x0;
        }
        // pu_var11 = (param_2 + -0x8);
        pu_var11 = param_2 - 0x8;
        pass1_1000_356e(
            &mut u_var16,
            pu_var11,
            param_2,
            (param_2 - 0xe),
            (param_2 - 0x17),
            param_5,
            param_5,
        );
        if (((param_2 + -0x5) & 0x2) != 0x0) && (pu_var11 == 0x0 || ((param_2 + -0x17) != 0x30)) {
            *(param_2 + -0x18) = 0x30;
            pu_var11 = (pu_var11 + 0x1);
        }
    } else {
        if c_var4 == 'u' as u8 {
            // goto
            // LAB_1000_3399;
        }
        if c_var4 == 'X' as u8 {
            *(param_2 + -0x3) = 0x7;
            //LAB_1000_33a9:
            if ((param_2 + -0x6) & 0x80) != 0x0 {
                (param_2 + -0x12) = 0x2;
                *(param_2 + -0x10) = 0x30;
                *(param_2 + -0xf) = *(param_2 + -0x3) + 'Q';
            }
            *(param_2 + -0x8) = 0x10;
            //       TODO: goto LAB_1000_33d4;
        }
        if c_var4 == 'x' as u8 {
            *(param_2 + -0x3) = 0x27;
            //       TODO: goto LAB_1000_33a9;
        }
        if c_var4 == 'o' as u8 {
            if ((param_2 + -0x6) & 0x80) != 0x0 {
                pb_var2 = (param_2 + -0x5);
                *pb_var2 = *pb_var2 | 0x2;
            }
            *(param_2 + -0x8) = 0x8;
            //       TODO: goto LAB_1000_33d4;
        }
        if c_var4 == 'c' as u8 {
            u_var7 = pass1_1000_34cf(param_2, param_5);
            *(param_2 + -0x216) = u_var7;
            *pu_var11 = (&ctx.PTR_LOOP_1050_0000 + 0x1) as i16;
        } else {
            if c_var4 == 's' as u8 {
                pass1_1000_34e6(param_1, param_2, param_5);
                *pu_var11 = ctx.DAT_1050_605d;
                if (param_3 != 0x0) || (param_4 != 0x0) {
                    i_var10 = (param_2 + -0xe) as i16;
                    pu_var11 = param_3;
                    if i_var10 != 0x0 {
                        b_var15 = true;
                        loop {
                            if i_var10 == 0x0 {
                                break;
                            }
                            i_var10 = i_var10 - 0x1;
                            pu_var3 = (*pu_var11) as u32;
                            pu_var11 = (pu_var11 + 0x1);
                            b_var15 = *pu_var3 == '\0' as u16;
                            if b_var15 {
                                break;
                            }
                        }
                        if b_var15 {
                            pu_var11 = (pu_var11 + -0x1);
                        }
                    }
                    pu_var11 = (pu_var11 - param_3);
                }
            } else {
                if c_var4 == 'n' as u8 {
                    pass1_1000_34e6(param_1, param_2, param_5);
                    *param_3 = (param_2 + -0xa);
                    if ((param_2 + -0x6) & 0x10) != 0x0 {
                        param_3[0x1] = 0x0;
                    }
                    //           TODO: goto LAB_1000_30cf;
                }
                if c_var4 == 'p' as u8 {
                    if ((param_2 + -0x6) & 0x30) == 0x0 {
                        u_var7 = pass1_1000_34cf(param_2, param_5);
                        u_var16 = u_var7;
                    } else {
                        u_var16 = pass1_1000_34d8(param_2, param_5) as u16;
                        // u_var12 = (u_var16 >> 0x10);
                        if ((param_2 + -0x5) & 0x18) == 0x0 {
                            *(param_2 + -0x3) = 0x7;
                            let mut a = 0x4;
                            pass1_1000_356e(
                                &mut u_var16,
                                0x10,
                                param_2,
                                a,
                                (param_2 + -0x20e),
                                param_5,
                                param_5,
                            );
                            pass1_1000_356e(
                                &mut u_var12,
                                0x10,
                                param_2,
                                a,
                                (param_2 + -0x213),
                                param_5,
                                param_5,
                            );
                            *(param_2 + -0x212) = 0x3a;
                            pu_var11 = ctx.DAT_1050_0009 as u32;
                            //               TODO: goto LAB_1000_3444;
                        }
                    }
                    *(param_2 + -0x3) = 0x7;
                    let mut a = 0x4i16;
                    pass1_1000_356e(
                        &mut u_var16,
                        0x10,
                        param_2,
                        a as u32,
                        (param_2 - 0x213),
                        param_5,
                        param_5,
                    );
                    pu_var11 = ctx.DAT_1050_0004 as u32;
                } else {
                    if (c_var4 == 'E' as u8) || (c_var4 == 'G' as u8) {
                        pi_var1 = (param_2 - 0x14);
                        *pi_var1 = *pi_var1 + 0x1;
                    }
                    pb_var2 = (param_2 - 0x6);
                    *pb_var2 = *pb_var2 | 0x40;
                    b_var6 = ((param_2 - 0x4) | 0x20) as u8;
                    i_var10 = (param_2 - 0xe) as i16;
                    if i_var10 < 0x1 {
                        if i_var10 == 0x0 {
                            if b_var6 == 0x67 {
                                (param_2 - 0xe) = 0x1;
                            }
                        } else {
                            (param_2 - 0xe) = 0x6;
                        }
                    }
                    pc_var13 = read_string_from_addr(param_2 - 0x216);
                    if ((param_2 - 0x5) & 0x4) == 0x0 {
                        (*ctx.PTR_s_3_wav_1050_25cc_1050_6068)();
                        pi_var1 = (param_2 + 0xe);
                        *pi_var1 = *pi_var1 + 0x8;
                    } else {
                        (*ctx.PTR_s_3_wav_1050_25cc_1050_607c)();
                        pi_var1 = (param_2 + 0xe);
                        *pi_var1 = *pi_var1 + 0xa;
                    }
                    if (((param_2 - 0x6) & 0x80) != 0x0) && ((param_2 + -0xe) == 0x0) {
                        (*ctx.PTR_s_3_wav_1050_25cc_1050_6074)();
                    }
                    if (b_var6 == 0x67) && (((param_2 - 0x6) & 0x80) == 0x0) {
                        (*ctx.PTR_s_3_wav_1050_25cc_1050_6070)();
                    }
                    if *pc_var13 == '-' {
                        pc_var13 = read_string_from_addr(param_2 - 0x215);
                        pb_var2 = (param_2 - 0x5);
                        *pb_var2 = *pb_var2 | 0x1;
                    }
                    i_var10 = -0x1;
                    pc_var14 = pc_var13;
                    loop {
                        if i_var10 == 0x0 {
                            break;
                        }
                        i_var10 += -0x1;
                        string_5 = pc_var14;
                        pc_var14 = &mut pc_var14[1..].to_string();
                        if *string_5 == '\0' {
                            break;
                        }
                    }
                    // pu_var11 = (pc_var14 + (-0x1 - pc_var13));
                }
            }
        }
    }
    //LAB_1000_3444:
    if ((param_2 + -0x6) & 0x40) != 0x0 {
        if ((param_2 + -0x5) & 0x1) == 0x0 {
            if ((param_2 + -0x6) & 0x1) == 0x0 {
                if ((param_2 + -0x6) & 0x2) != 0x0 {
                    *(param_2 + -0x10) = 0x20;
                    (param_2 + -0x12) = 0x1;
                }
            } else {
                *(param_2 + -0x10) = 0x2b;
                (param_2 + -0x12) = 0x1;
            }
        } else {
            *(param_2 + -0x10) = 0x2d;
            (param_2 + -0x12) = 0x1;
        }
    }
    i_var8 = ((param_2 + -0xc) - pu_var11) as i16;
    i_var10 = (param_2 + -0x12) as i16;
    i_var9 = i_var8 - i_var10;
    if i_var8 < i_var10 {
        i_var9 = 0x0;
    }
    if ((param_2 + -0x6) & 0xc) == 0x0 {
        pass1_1000_3552(&mut i_var9, param_2 as i16, param_5, 0, 0, 0);
    }
    pass1_1000_3534((param_2 + -0x12), param_2 as i16, param_5, 0, 0, 0, 0, 0);
    if (((param_2 + -0x6) & 0x8) != 0x0) && (((param_2 + -0x6) & 0x4) == 0x0) {
        pass1_1000_3552(&mut i_var9, param_2 as i16, param_5, 0, 0, 0);
    }
    pass1_1000_3534(pu_var11, param_2 as i16, param_5, 0, 0, 0, 0, 0);
    if ((param_2 + -0x6) & 0x4) != 0x0 {
        pass1_1000_3552(&mut i_var9, param_2 as i16, param_5, 0, 0, 0);
    }
    //LAB_1000_30cf:
    string_5 = &mut read_string_from_addr((param_2 + 0xa) as u32).clone();
    c_var4 = string_5[0];
    write_string_to_addr((param_2 + 0xa), &string_5[1..].to_string());
    *(param_2 + -0x4) = c_var4;
    if (c_var4 != '\0' as u8) && (-0x1 < (param_2 + -0xa)) {
        if (c_var4 - 0x20) < 0x59 {
            b_var6 = ((c_var4 - 0x20) + 0x5ffe) & 0xf;
        } else {
            b_var6 = 0x0;
        }
        b_var6 = ((b_var6 * 0x8 as u8 + *(param_2 + -0x7)) + 0x5ffe) >> 0x4;
        (param_2 + -0x7) = b_var6 as i16 as u32;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many
        // branches
        // WARNING: Treating indirect jump as call
        u_var7 = (b_var6 * 0x2 + 0x30a4)();
        return u_var7;
    }
    return (param_2 - 0xa) as u16;
}

pub unsafe fn pass1_1000_34cf(param_1: U32Ptr, param_2: u16) -> u16 {
    let u_var1: u16;
    let pu_var2: &mut Struct197;

    pu_var2 = &mut param_1.field_0xe;
    u_var1 = pu_var2.field_0x0;
    (param_1.field_0xe) = pu_var2 + 0x2;
    return u_var1;
}

pub unsafe fn pass1_1000_34d8(param_1: U32Ptr, param_2: u16) -> u32 {
    let u_var1: u16;
    let u_var2: u16;
    let pu_var3: U32Ptr;

    pu_var3 = (param_1 + 0xe);
    u_var1 = *pu_var3;
    u_var2 = (pu_var3 + 0x2) as u16;
    (param_1 + 0xe) = pu_var3 + 0x4;
    return CONCAT22(u_var2, u_var1);
}

pub unsafe fn pass1_1000_34e6(param_1: u16, param_2: U32Ptr, param_3: u16) -> u32 {
    let u_var1: u16;
    let u_var2: u32;

    if (((param_2 + -0x6) & 0x20) != 0x0) {
        u_var2 = pass1_1000_34d8(param_2, param_3);
        return u_var2;
    }
    u_var1 = pass1_1000_34cf(param_2, param_3);
    if (u_var1 == 0x0) {
        return (param_1 << 0x10) as u32;
    }
    return CONCAT22(param_1, u_var1);
}

pub unsafe fn pass1_1000_3503(
    param_1: u8,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
    param_6: u8,
) -> i16 {
    let mut string_4: String;
    let mut string_1: String;
    let mut string_2: &mut String;
    let mut u_var4: u16 = 0u16;
    let mut string_3: &mut String;
    let mut u_var6: u16 = 0u16;

    string_2 = read_string_from_addr((param_3 + 0x6) as u32);

    string_3 = string_2;
    string_4 = string_3[2..].to_string();
    string_4[0] = string_4[0] - 1;
    if string_4[0] < 0x0 {
        u_var4 = mem_1000_2bb6(
            param_1 as u16,
            &string_3,
            param_3,
            u_var6,
            param_4,
            param_5,
            param_6,
            param_2,
        );
        if u_var4 == 0xffff {
            return -0x1;
        }
    } else {
        string_1 = string_2[0];
        string_2[0] = string_2[0] + 0x1;
        string_1[0] = param_1;
    }
    return 0x0;
}

pub unsafe fn pass1_1000_3534(
    param_1: U32Ptr,
    param_2: i16,
    param_3: u16,
    in_dx: u16,
    unaff_di: u16,
    unaff_es: u16,
    unaff_cs: u16,
    unaff_af: u8,
    in_af: u8,
) {
    let pi_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    // let in_dx: u16;
    // let unaff_di: U32Ptr;
    let u_var4: u16;
    // let unaff_es: u16;
    // let unaff_cs: u16;
    // let in_af: u8;

    if param_1 != 0x0 {
        pi_var1 = (param_2 + -0xa) as u32;
        *pi_var1 = *pi_var1 + param_1;
        u_var4 = 0x0;
        loop {
            pu_var2 = *unaff_di as u32;
            *unaff_di = *unaff_di + 0x1;
            u_var3 = pass1_1000_3503(*pu_var2, in_dx, param_2, unaff_cs, param_3, in_af);
            u_var4 |= u_var3;
            *param_1 += -0x1;
            if param_1 == 0 {
                break;
            }
        }
        if u_var4 != 0x0 {
            (param_2 + -0xa) = 0xffff;
        }
    }
    return;
}

pub unsafe fn pass1_1000_3552(
    param_1: &mut i16,
    param_2: i16,
    param_3: u16,
    in_dx: u16,
    unaff_cs: u16,
    in_af: u8,
) {
    let pi_var1: U32Ptr;
    let u_var2: u16;
    // let in_dx: u16;
    let u_var3: u16;
    // let unaff_cs: u16;
    // let in_af: u8;

    if (param_1 != 0x0) {
        pi_var1 = (param_2 + -0xa) as u32;
        *pi_var1 = *pi_var1 + param_1;
        u_var3 = 0x0;
        loop {
            u_var2 = pass1_1000_3503(in_dx as u8, in_dx, param_2, unaff_cs, param_3, in_af) as u16;
            u_var3 |= u_var2;
            *param_1 += -0x1;
            if param_1 == 0 {
                break;
            }
        }
        if (u_var3 != 0x0) {
            (param_2 + -0xa) = 0xffff;
        }
    }
    return;
}

pub fn pass1_1000_356e(
    param_1: &mut u16,
    param_2: U32Ptr,
    param_3: U32Ptr,
    param_4: U32Ptr,
    param_5: U32Ptr,
    param_6: U32Ptr,
    param_7: u16,
    param_8: u16,
) {
    let mut pb_var1: U32Ptr;
    let u_var2: u32;
    let b_var3: u8;

    while ((0x0 < param_5 || (param_1 != 0x0)) || (param_3 != 0x0)) {
        u_var2 = *param_3;
        *param_3 /= param_2;
        u_var2 = u_var2 % param_2 << 0x10 | param_1;
        *param_1 = (u_var2 / param_2) as u16;
        b_var3 = ((u_var2 % param_2) + 0x30) as u8;
        if (0x39 < b_var3) {
            b_var3 += *(param_4 + -0x3);
        }
        pb_var1 = *param_6;
        *param_6 = *param_6 + -0x1;
        *pb_var1 = b_var3;
        *param_5 += -0x1;
    }
    return;
}

pub unsafe fn pass1_1000_35aa(ctx: &mut Globals) -> u16 {
    let pu_var1: U32Ptr;

    pu_var1 = ctx.PTR_LOOP_1050_6210;
    loop {
        if ctx.PTR_LOOP_1050_5ff0 < pu_var1 {
            return 0x0;
        }
        if ((pu_var1 + 0x5) & 0x83) == 0x0 {
            break;
        }
        pu_var1 = pu_var1 + 0x6;
    }
    *(pu_var1 + 0x5) = 0x0;
    pu_var1[0x2] = 0x0;
    pu_var1[0x4] = 0x0;
    pu_var1[0x3] = 0x0;
    pu_var1[0x1] = 0x0;
    *pu_var1 = 0x0;
    *(pu_var1 + 0xb) = 0xff;
    return pu_var1 as u16;
}

pub fn pass1_1000_39e1() {
    return;
}

pub fn pass1_1000_3bac(ctx: &mut Globals, stack0x0004: u16) -> i16 {
    let i_var1: i16;

    if (ctx.PTR_LOOP_1050_5f48 < &stack0x0004) {
        i_var1 = -(ctx.PTR_LOOP_1050_5f48 + -&stack0x0004);
    } else {
        i_var1 = 0x0;
    }
    return i_var1;
}

pub unsafe fn pass1_1000_3bc0(
    ctx: &mut Globals,
    param_1: &mut i16,
    param_2: &mut i16,
    param_3: U32Ptr,
    param_4: &mut String,
    mut param_5: u16,
    param_6: &String,
) {
    let pi_var1: U32Ptr;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let u_var4: u16;
    let i_var5: i16;
    let pu_var6: U32Ptr;
    let b_var7: bool;
    let u_var8: u32;

    if ((param_2 + 0x2) & 0x1) != 0x0 {
        pass1_1000_3cb7(*param_2);
        u_var4 = *param_3;
        if (u_var4 & 0x1) != 0x0 {
            *param_1 = (*param_1 - u_var4) + -0x1;
        }
        u_var4 = (param_2 + 0x4);
        if u_var4 != 0x0 {
            u_var3 = param_1 + 0x2 + u_var4;
            if !CARRY2(param_1 + 0x2, u_var4) {
                *param_4 = pass1_1000_29dc(param_6).clone();
                u_var4 = ctx.PTR_LOOP_1050_6066 as u16;
                if u_var4 == 0x1000 {
                    // goto
                    // LAB_1000_3c12;
                }
                u_var2 = 0x8000;
                while u_var4 <= u_var2 {
                    u_var2 >>= 0x1;
                    if u_var2 == 0x0 {
                        // goto
                        // LAB_1000_3c2b;
                    }
                }
                if u_var2 < 0x8 {
                    // goto
                    // LAB_1000_3c2b;
                }
                u_var4 = u_var2 << 0x1;
                //         TODO: goto LAB_1000_3c12;
            }
            u_var2 = 0x0;
            u_var4 = 0xfff0;
            if (u_var3 == 0x0) {
                loop {
                    b_var7 = false;
                    u_var8 = mixed_mem_op_1000_3c51(
                        &mut u_var2,
                        &mut u_var3,
                        *param_2,
                        param_4,
                        &mut param_5,
                        0x3c23,
                    );
                    if !b_var7 {
                        break;
                    }
                    if u_var4 == 0xfff0 {
                        return;
                    }
                    //LAB_1000_3c2b:
                    u_var4 = 0x10;
                    //LAB_1000_3c12:
                    u_var4 -= 0x1;
                    u_var2 = u_var4 + u_var3;
                    if (CARRY2(u_var4, u_var3)) {
                        u_var2 = 0x0;
                    }
                    u_var4 = !u_var4;
                    u_var2 &= u_var4;
                }
                i_var5 = u_var8 - (param_2 + 0x4);
                (param_2 + 0x4) = u_var8;
                (param_2 + 0xa) = param_3;
                pi_var1 = (param_2 + 0xc);
                *pi_var1 = i_var5 + -0x1;
                pu_var6 = (pi_var1 + i_var5);
                *pu_var6 = 0xfffe;
                (param_2 + 0xc) = pu_var6;
            }
        }
    }
    return;
}

pub unsafe fn pass1_1000_3cb7(param_1: i16) {
    let u_var1: u16;
    let pu_var2: U32Ptr;

    pu_var2 = (param_1 + 0xa) as u32;
    if (pu_var2 == (param_1 + 0xc) as u32) {
        pu_var2 = (param_1 + 0x8) as u32;
    }
    loop {
        u_var1 = *pu_var2;
        if (u_var1 == 0xfffe) {
            break;
        }
        pu_var2 = (pu_var2 + (u_var1 & 0xfffe) + 0x2);
    }
    return;
}

pub fn pass1_1000_3cd8(param_1: u16, param_2: u16, stack0xfffe: u16) {
    free_mem_1000_407a(param_1, param_2, stack0xfffe);
    return;
}

pub fn pass1_1000_3d7a(param_1: u32, param_2: &String) -> i16 {
    let pb_var1: U32Ptr;
    let mut pc_var2: &String;
    let pb_var3: U32Ptr;
    let i_var4: i16;
    let u_var5: u16;
    let mut pc_var6: String;
    let pb_var7: U32Ptr;
    let mut pc_var8: &String;
    let pb_var9: U32Ptr;
    let u_var10: u16;
    let mut b_var11: bool = false;
    let b_var12: bool;

    pb_var7 = param_1;
    // u_var10 = (param_2 >> 0x10);
    pc_var8 = param_2;
    i_var4 = 0x0;
    u_var5 = 0xffff;
    loop {
        if u_var5 == 0x0 {
            break;
        }
        u_var5 -= 0x1;
        pc_var2 = pc_var8;
        pc_var8 = pc_var8 + 0x1;
        if *pc_var2 == '\0' {
            break;
        }
    }
    pc_var6 = String::new();
    // pc_var6 = !u_var5;
    // b_var11 = pc_var8 < pc_var6;
    // pb_var9 = (pc_var8 + -pc_var6);
    b_var12 = pb_var9 == 0x0;
    loop {
        if pc_var6 == 0x0 {
            break;
        }
        // pc_var6 = pc_var6 - 0x1;
        // pb_var3 = pb_var9;
        // pb_var9 = pb_var9 + 0x1;
        pb_var1 = pb_var7;
        pb_var7 = pb_var7 + 0x1;
        b_var11 = *pb_var1 < *pb_var3;
        b_var12 = *pb_var1 == *pb_var3;
        if !b_var12 {
            break;
        }
    }
    if !b_var12 {
        i_var4 = (0x1 - b_var11) - (b_var11 != false);
    }
    return i_var4;
}

pub fn pass1_1000_3de8(
    param_1: &mut String,
    param_2: &mut String,
    param_3: &mut u16,
    param_4: u16,
    param_5: u16,
) -> u16 {
    let pb_var1: U32Ptr;
    let mut pc_var2: &mut String;
    let mut pc_var3: &mut String;
    let b_var4: u8;
    let u_var5: u16;
    let i_var6: i16;
    let mut pc_var7: &mut String;
    let mut pc_var8: &mut String;
    let u_var9: u16;
    let u_var10: u16;
    let b_var11: bool;

    if param_3 != 0x0 {
        // u_var9 = (param_1 >> 0x10);
        pc_var8 = param_1;
        u_var5 = *param_3;
        pc_var7 = pc_var8;
        loop {
            if u_var5 == 0x0 {
                break;
            }
            u_var5 -= 0x1;
            pc_var2 = pc_var7;
            pc_var7 = pc_var7 + 0x1;
            if *pc_var2 == '\0' {
                break;
            }
        }
        i_var6 = param_3 - u_var5;
        // u_var10 = (param_2 >> 0x10);
        pc_var7 = param_2;
        loop {
            if i_var6 == 0x0 {
                break;
            }
            i_var6 += -0x1;
            pc_var3 = pc_var8;
            pc_var8 = pc_var8 + 0x1;
            pc_var2 = pc_var7;
            pc_var7 = pc_var7 + 0x1;
            if *pc_var2 != *pc_var3 {
                break;
            }
        }
        b_var4 = pc_var7[-0x1];
        u_var5 = 0x0;
        pb_var1 = (pc_var8 + -0x1);
        b_var11 = b_var4 == *pb_var1;
        if b_var4 < *pb_var1 || b_var11 {
            if b_var11 {
                return 0x0;
            }
            u_var5 = 0xfffe;
        }
        *param_3 = !u_var5;
    }
    return *param_3;
}

pub fn pass1_1000_3e2c(string_1: &String) -> i16 {
    let string_2: &String;
    let b_var2: u8;
    let b_var3: u8;
    let i_var4: i16;
    let string_3: &String;
    let u_var6: u16;

    // u_var6 = (param_1 >> 0x10);
    string_3 = string_1;
    i_var4 = 0x0;
    loop {
        loop {
            string_2 = string_3;
            string_3 = string_3 + 0x1;
            b_var2 = *string_2;
            if b_var2 != 0x20 {
                break;
            }
        }
        if b_var2 != 0x9 {
            break;
        }
    }
    b_var3 = b_var2;
    if (b_var2 != 0x2d) && (b_var2 != 0x2b) {
        // goto LAB_1000_3e4d;
    }
    loop {
        string_2 = string_3;
        string_3 = string_3 + 0x1;
        b_var3 = *string_2;
        //LAB_1000_3e4d:
        if (0x39 < b_var3) || (b_var3 < 0x30) {
            break;
        }
        i_var4 = i_var4 * 0xa + (b_var3 - 0x30);
    }
    if b_var2 == 0x2d {
        i_var4 = -i_var4;
    }
    return i_var4;
}

// pub fn pass1_1000_3e2c(param_1: u32) -> i16
//
// {
//     let pb_var1: U32Ptr;
//     let b_var2: u8;
//     let b_var3: u8;
//     let i_var4: i16;
//     let pb_var5: U32Ptr;
//     let u_var6: u16;
//
//     // u_var6 = (param_1 >> 0x10);
//     pb_var5 = param_1;
//     i_var4 = 0x0;
//     loop {
//         loop {
//             pb_var1 = pb_var5;
//             pb_var5 = pb_var5 + 0x1;
//             b_var2 = *pb_var1;
//             if b_var2 != 20 {
//                 break;
//             }
//         }
//         if b_var2 != 0x9 {
//             break;
//         }
//     }
//     if ((b_var2 != 0x2d) && (b_var3 = b_var2, b_var2 != 0x2b)) {
//         // goto LAB_1000_3e4d;
//     }
//     loop {
//         pb_var1 = pb_var5;
//         pb_var5 = pb_var5 + 0x1;
//         b_var3 = *pb_var1;
// //LAB_1000_3e4d:
//         if ((0x39 < b_var3) || (b_var3 < 0x30)) { break; }
//         i_var4 = i_var4 * 0xa + (b_var3 - 0x30);
//     }
//     if (b_var2 == 0x2d) {
//         i_var4 = -i_var4;
//     }
//     return i_var4;
// }

pub fn pass1_1000_3e82(param_1: &mut u16, param_2: U32Ptr, param_3: u16) -> U32Ptr {
    let pb_var1: U32Ptr;
    let u_var2: u32;
    let b_var3: u8;
    let u_var5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let pb_var8: U32Ptr;
    let pb_var9: U32Ptr;
    let pb_var10: U32Ptr;
    let pb_var11: U32Ptr;
    let u_var12: u16;
    let b_var13: bool;
    let c_var4: u8;

    u_var6 = 0x0;
    if param_3 == 0xa {
        u_var6 = param_1 >> 0xf;
    }
    // u_var12 = (param_2 >> 0x10);
    pb_var9 = param_2;
    pb_var10 = pb_var9;
    pb_var8 = pb_var9;
    if (true && (param_3 == 0xa)) & &(u_var6 < 0x0) {
        pb_var10 = pb_var9 + 0x1;
        *param_2 = '-';
        b_var13 = *param_1 != 0x0;
        *param_1 = -*param_1;
        u_var6 = -(u_var6 + b_var13);
        pb_var8 = pb_var10;
    }
    loop {
        u_var7 = 0x0;
        u_var5 = u_var6;
        if u_var6 != 0x0 {
            u_var5 = u_var6 / param_3;
            u_var7 = u_var6 % param_3;
        }
        u_var2 = CONCAT22(u_var7, *param_1);
        *param_1 = (u_var2 / param_3) as u16;
        c_var4 = (u_var2 % param_3) as u8;
        b_var3 = c_var4 + 0x30;
        if 0x39 < b_var3 {
            b_var3 = c_var4 + 0x57;
        }
        pb_var11 = pb_var10 + 0x1;
        *pb_var10 = b_var3;
        u_var6 = u_var5;
        pb_var10 = pb_var11;
        if (u_var5 | param_1) == 0 {
            break;
        }
    }
    *pb_var11 = 0x0;
    loop {
        pb_var11 = pb_var11 + -0x1;
        pb_var1 = pb_var11;
        b_var3 = *pb_var1;
        *pb_var1 = *pb_var8;
        *pb_var8 = b_var3;
        pb_var10 = pb_var8 + 0x2;
        pb_var8 = pb_var8 + 0x1;
        if pb_var10 >= pb_var11 {
            break;
        }
    }
    return pb_var9;
}

pub fn pass1_1000_3ec0(ctx: &mut Globals, param_1: u16, param_2: u16) -> i16 {
    let mut u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let unaff_si: u16 = 0;
    let u_var4: u16;
    let pu_var4: u32;

    pu_var4 = CONCAT22(ctx.PTR_LOOP_1050_5fc0 as u16, ctx.PTR_LOOP_1050_5fbe as u16);
    if ((ctx.PTR_LOOP_1050_5fc0 | ctx.PTR_LOOP_1050_5fbe) != 0x0) && ((param_2 | param_1) != 0x0) {
        u_var1 = str_op_1000_3da4(read_string_from_addr(CONCAT22(param_2, param_1)));
        loop {
            // u_var4 = (pu_var4 >> 0x10);
            u_var3 = pu_var4 as u16;
            if ((u_var3 + 0x2) | pu_var4) == 0x0 {
                break;
            }
            u_var2 = str_op_1000_3da4(read_string_from_addr(CONCAT22(
                (u_var3 + 0x2),
                pu_var4 as u16,
            )));
            u_var2 = pass1_1000_3de8(
                read_string_from_addr(CONCAT22((u_var3 + 0x2), pu_var4 as u16)),
                read_string_from_addr(CONCAT22(param_2, param_1)),
                &mut u_var1,
                unaff_si,
                u_var3,
            );
            if ((u_var1 < u_var2) && (*(*pu_var4 + u_var1) == '=')) && (u_var2 == 0x0) {
                return (pu_var4 + u_var1 + 0x1) as i16;
            }
            pu_var4 = (pu_var4 & 0xffff0000 | (u_var3 + 0x4));
        }
    }
    return 0x0;
}

pub fn pass1_1000_3f5c(
    ctx: &mut Globals,
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u8,
) -> i16 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let i_var3: i16;
    let i_stack2: i16;

    i_stack2 = param_1 + 0x1;
    i_var3 = 0x0;
    if ctx.PTR_LOOP_1050_61ec == 0x0 {
        pu_var2 = ctx.PTR_LOOP_1050_6210;
    } else {
        pu_var2 = 0x6234;
    }
    // TODO: refactor for loop
    // for (; pu_var2 <= ctx.PTR_LOOP_1050_5ff0; pu_var2 = pu_var2 + 0x6) {
    //   u_var1 = pass1_1000_2a00(pu_var2,&i_stack2,param_2,param_3,param_4,param_5);
    //   if (u_var1 != 0xffff) {
    //     i_var3 += 0x1;
    //   }
    // }
    return i_var3;
}

pub fn pass1_1000_400a(ctx: &mut Globals, param_1: i16, param_2: u16) -> U32Ptr {
    let pu_var1: U32Ptr;
    let i_stack2: i16;

    i_stack2 = (param_2 + 0x1) as i16;
    if (param_1 < 0x0) || (ctx.PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1) {
        ctx.PTR_LOOP_1050_5f78 = ctx.DAT_1050_0009 as u32;
        pu_var1 = 0xffff;
    } else {
        if ((ctx.PTR_LOOP_1050_61ec == 0x0)
            || (param_1 < ctx.DAT_1050_5f8a as i16 && (0x2 < param_1)))
            && (0x31d < CONCAT11(ctx.DAT_1050_5f83, ctx.DAT_1050_5f82))
        {
            pu_var1 = ctx.PTR_LOOP_1050_5f88;
            pu_var1 = dos3_call_1000_5174(i_stack2 as u16) as u32;
            if (((param_1 + 0x5f90) & 0x1) == 0x0) || (pu_var1 != 0x0) {
                ctx.PTR_LOOP_1050_5f88 = pu_var1;
                ctx.PTR_LOOP_1050_5f78 = ctx.DAT_1050_0009 as u32;
                pu_var1 = 0xffff;
            }
        } else {
            pu_var1 = 0x0;
        }
    }
    return pu_var1;
}

pub fn pass1_1000_41e0(ctx: &mut Globals, param_1: i16) -> u16 {
    let pi_stack6: U32Ptr;

    pi_stack6 = CONCAT22(ctx.PTR_LOOP_1050_6192, ctx.PTR_LOOP_1050_6190);
    loop {
        if (ctx.PTR_LOOP_1050_6190 + (ctx.PTR_LOOP_1050_6194 & 0xfffc) <= pi_stack6) {
            return 0x0;
        }
        if (*pi_stack6 == param_1) {
            break;
        }
        pi_stack6 = (pi_stack6 & 0xffff0000 | ZEXT24((pi_stack6 + 0x4) as u16));
    }
    *pi_stack6 = 0x0;
    return (pi_stack6 + 0x2) as u16;
}

// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1000_422a(
    ctx: &mut Globals,
    param_1: &mut i16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
) -> i16 {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let pu_var3: U32Ptr;
    let pu_var4: U32Ptr;
    let pi_stack6: U32Ptr;

    pi_stack6 = CONCAT22(ctx.PTR_LOOP_1050_6192, ctx.PTR_LOOP_1050_6190);
    loop {
        if ctx.PTR_LOOP_1050_6190 + (ctx.PTR_LOOP_1050_6194 & 0xfffc) <= pi_stack6 {
            pu_var2 = ctx.PTR_LOOP_1050_6194 + 0x28;
            pu_var4 = ctx.PTR_LOOP_1050_6192;
            pu_var3 = pass1_1000_16aa(
                ctx,
                ctx.PTR_LOOP_1050_6190,
                ctx.PTR_LOOP_1050_6192,
                pu_var2 as u16,
                ctx.PTR_LOOP_1050_6192,
                param_3,
                param_4,
            ) as u32;
            if (pu_var4 | pu_var3) == 0x0 {
                *param_1 = 0x0;
            } else {
                pu_var1 = pu_var3 + (ctx.PTR_LOOP_1050_6194 & 0xfffc);
                pi_stack6 = CONCAT22(pu_var4 as u16, pu_var1 as u16);
                ctx.PTR_LOOP_1050_6190 = pu_var3;
                ctx.PTR_LOOP_1050_6192 = pu_var4;
                *pi_stack6 = param_1;
                (pu_var1 + 0x2) = param_2 as u32;
                ctx.PTR_LOOP_1050_6194 = pu_var2;
                pass1_1000_4906(
                    read_struct_from_addr::<Struct20>(CONCAT22(
                        pu_var4 as u16,
                        (pu_var1 + 0x4) as u16,
                    )),
                    None,
                    0x24,
                );
            }
            return *param_1;
        }
        if *pi_stack6 == 0x0 {
            break;
        }
        pi_stack6 = (pi_stack6 & 0xffff0000 | ZEXT24((pi_stack6 + 0x4) as u16));
    }
    (pi_stack6 + 0x2) = param_2 as u32;
    *pi_stack6 = param_1;
    return *param_1;
}

pub fn pass1_1000_43f0(ctx: &mut Globals, param_1: u16, param_2: u16) {
    if ctx.PTR_LOOP_1050_68b4 == 0x0 {
        pass1_1000_440c(ctx, param_2);
        ctx.PTR_LOOP_1050_68b4 = ctx.PTR_LOOP_1050_68b4 + 0x1;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1000_440c(ctx: &mut Globals, param_1: u16) {
    let c_var1: u8;
    let mut string_1 = String::new();
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let l_var6: i32;
    let u_var7: u16;
    let u_var8: u16;
    let mut string_2 = String::new();

    u_var3 = pass1_1000_3ec0(ctx, 0x61ca, ctx.data_seg) as u16;
    string_2 = read_string_from_addr(CONCAT22(param_1, u_var3)).clone();
    ctx._DAT_1050_61ce = ctx.DAT_1050_61ce;
    if ((param_1 | u_var3) != 0x0) && (*string_2 != '\0') {
        let mut a = 0x3u16;
        str_op_1000_3dbe(
            read_string_from_addr(ctx.PTR_USHORT_1050_1050_1050_61de),
            read_string_from_addr(CONCAT22(param_1, u_var3)),
            &mut a,
        );
        string_2 = read_string_from_addr(CONCAT22(param_1, u_var3 + 0x3)).clone();
        c_var1 = string_2[0];
        if c_var1 == '-' as u8 {
            string_2 = read_string_from_addr(CONCAT22(param_1, u_var3 + 0x4)).clone();
        }
        u_var5 = 0x0;
        u_var8 = 0x0;
        u_var7 = 0xe10;
        i_var4 = pass1_1000_3e2c(string_2 & 0xffff | param_1 << 0x10);
        ctx._DAT_1050_61ce = pass1_1000_52be(i_var4 as u16, u_var5, u_var7, u_var8);
        // for (; (pc_var2 = pc_stack8, *pc_stack8 == '+' ||
        //        (('/' < *pc_stack8 && (*pc_stack8 < ':'))));
        //     pc_stack8 = (pc_stack8 & 0xffff0000 | (pc_stack8 + 0x1)))
        // {
    }
    if string_2[0] == ':' {
        u_var5 = 0x0;
        u_var8 = 0x0;
        u_var7 = 0x3c;
        string_2 = string_2[1..].to_string();
        i_var4 = pass1_1000_3e2c(&string_2[1..].to_string());
        l_var6 = pass1_1000_52be(i_var4 as u16, u_var5, u_var7, u_var8) as i32;
        // u_var5 = (l_var6 >> 0x10);
        ctx._DAT_1050_61ce += l_var6;
        // TODO: refactor for loop
        // for (; (pc_var2 = pc_stack8, '/' < *pc_stack8 && (*pc_stack8 < ':'));
        //     pc_stack8 = (pc_stack8 & 0xffff0000 | (pc_stack8 + 0x1))
        //     ) {
        // }
        if string_2[0] == ':' {
            string_2 = string_2[1..].to_string();
            let mut string_3 = i_var4 = pass1_1000_3e2c(&string_2[1..].to_string());
            ctx._DAT_1050_61ce += CONCAT22(u_var5, i_var4 as u16);
            // TODO: refactor for loop
            // for (; ('/' < *pc_stack8 && (*pc_stack8 < ':'));
            //     pc_stack8 = (pc_stack8 & 0xffff0000 |
            //                        (pc_stack8 + 0x1))) {
            // }
        }
    }
    ctx.PTR_LOOP_1050_61d0 = (ctx._DAT_1050_61ce >> 0x10);
    if c_var1 == '-' as u8 {
        ctx.DAT_1050_61ce = CONCAT22(
            -(ctx.PTR_LOOP_1050_61d0 + (ctx.DAT_1050_61ce != 0x0)),
            -ctx.DAT_1050_61ce,
        );
    }
    // ctx.DAT_1050_61d2 = string_2[0..0];
    write_string_to_addr(ctx.DAT_1050_61d2, &string_2[0]);
    if ctx.DAT_1050_61d2 == 0x0 {
        ctx.PTR_1050_61e0[0] = '\0';
    } else {
        let mut a = 0x3u16;
        str_op_1000_3dbe(
            read_string_from_addr(ctx.PTR_1050_61e0),
            &mut string_2,
            &mut a,
        );
    }

    ctx.PTR_LOOP_1050_61d0 = (ctx._DAT_1050_61ce >> 0x10);
    return;
}

pub fn pass1_1000_455a(param_1: u32, param_2: &mut String) -> u16 {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;
    let i_var4: i16;
    let uvar5: u16;
    let u_var6: u32;
    let i_stack6: i16;

    if (((param_1 + 0xa) < 0x43) || ((param_1 + 0x8) < 0x3)) || (0x9 < (param_1 + 0x8)) {
        // goto
        // LAB_1000_4623;
    }
    if ((param_1 + 0x8) < 0x4) || (0x8 < (param_1 + 0x8)) {
        u_var3 = (param_1 + 0xa) as u16;
        if (u_var3 < 0x57) || ((param_1 + 0x8) != 0x3) {
            i_stack6 = ((param_1 + 0x8) * 0x2 + 0x61b2) as i16;
        } else {
            i_stack6 = (((param_1 + 0x8) * 0x2 + 0x61b0) + 0x7) as i16;
        }
        if ((u_var3 & 0x3) == 0x0) {
            i_stack6 += 0x1;
        }
        u_var3 = (u_var3 - 0x46) * 0x16d + ((u_var3 - 0x1) >> 0x2) + i_stack6;
        let mut a = u_var3 - 0xd;
        let mut b = 0x7u16;
        u_var6 = pass1_1000_52f0(&mut a, (u_var3 >> 0xf) - (u_var3 < 0xd), &mut b, 0x0);
        i_stack6 = (u_var6 - i_stack6) as i16;
        i_var4 = -i_stack6;
        if (param_1 + 0x8) == 0x3 {
            i_var2 = (param_1 + 0xe) as i16;
            if (i_var4 < i_var2) || (-i_var2 == i_stack6 && (0x1 < (param_1 + 0x4))) {}
        //       TODO: goto LAB_1000_460e;
        } else {
            pi_var1 = (param_1 + 0xe);
            i_var2 = *pi_var1;
            if ((SBORROW2(*pi_var1 as u32, i_var4 as u32) != i_var2 as u32) + (i_stack6 < 0x0))
                || ((i_var2 == i_var4) && ((param_1 + 0x4) < 0x1))
            {
                // goto
                // LAB_1000_460e;
            }
        }
        //LAB_1000_4623:
        uvar5 = 0x0;
    } else {
        //LAB_1000_460e:
        uvar5 = 0x1;
    }
    return uvar5;
}

pub fn pass1_1000_462e(
    ctx: &mut Globals,
    param_1: u16,
    param_2: i16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: i16,
    param_7: i16,
    param_8: &mut String,
    param_9: u16,
) -> i16 {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let i_var4: i16;
    let uvar5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let u_var8: u16;
    let u_var9: u32;
    let i_stack26: i16;
    // let local_16: [u8; 4] = [0;4];
    let mut local_16: U32Ptr = 0;
    let u_stack18: u16;
    let i_stack14: i16;
    let i_stack12: i16;
    let i_stack8: i16;
    let local_4: u16;
    let i_stack2: i16;
    let u_var10: u16;
    let u_var11: u16;
    let u_var12: u16;
    let u_var13: u16;

    i_stack2 = param_7 + 0x1;
    local_4 = ctx.data_seg;
    u_var8 = (param_2 * 0x2 + 0x61ae) as u16;
    if ((param_1 & 0x3) == 0x0) && (0x2 < param_2) {
        u_var8 += 0x1;
    }
    pass1_1000_43f0(ctx, i_stack2 as u16, param_9);
    u_var13 = 0x0;
    u_var12 = 0x3c;
    u_var11 = 0x0;
    u_var10 = 0x3c;
    u_var1 = (param_1 * 0x16d);
    u_var2 = (param_1 + 0x3) >> 0x2;
    u_var3 = u_var2 + param_3;
    u_var6 = u_var1 + u_var3;
    u_var7 = u_var6 + u_var8;
    u_var9 = pass1_1000_52be(
        u_var7 + 0xe44,
        ((param_1 * 0x16d) >> 0x10)
            + ((param_1 + 0x3) >> 0xf)
            + (param_3 >> 0xf)
            + CARRY2(u_var2, param_3)
            + CARRY2(u_var1, u_var3)
            + (u_var8 >> 0xf)
            + CARRY2(u_var6, u_var8)
            + (0xf1bb < u_var7),
        0x18,
        0x0,
    );
    u_var9 = pass1_1000_52be(
        (u_var9 + param_4) as u16,
        ((u_var9 >> 0x10) + (param_4 >> 0xf) + CARRY2(u_var9 as u16, param_4)) as u16,
        u_var10,
        u_var11,
    );
    i_var4 = pass1_1000_52be(
        (u_var9 + param_5) as u16,
        ((u_var9 >> 0x10) + (param_5 >> 0xf) + CARRY2(u_var9 as u16, param_5)) as u16,
        u_var12,
        u_var13,
    ) as i16;
    i_stack26 = i_var4 + param_6 + ctx.DAT_1050_61ce;
    i_stack8 = (param_3 + u_var8) as i16;
    i_stack12 = (param_1 + 0x50) as i16;
    i_stack14 = param_2 + -0x1;
    u_stack18 = param_4;
    if ctx.DAT_1050_61d2 != 0x0 {
        uvar5 = pass1_1000_455a(local_16, param_8);
        if uvar5 != 0x0 {
            i_stack26 += -0xe10;
        }
    }
    return i_stack26;
}

pub fn pass1_1000_472c(param_1: &mut String, param_2: u8) -> U32Ptr {
    let mut string_1: &mut String;
    let u_var2: u16;
    let mut string_2: &mut String;
    let mut string_3: &mut String;
    let b_var6: bool;

    string_2 = param_1;
    b_var6 = true;
    u_var2 = 0xffff;
    string_3 = string_2;
    loop {
        if u_var2 == 0x0 {
            break;
        }
        u_var2 -= 0x1;
        string_1 = string_3;
        string_3 = &mut string_3[1..].to_string();
        b_var6 = *string_1 == '\0';
        if b_var6 {
            break;
        }
    }
    u_var2 = !u_var2;
    loop {
        if u_var2 == 0x0 {
            break;
        }
        u_var2 -= 0x1;
        string_1 = string_2;
        string_2 = string_2 + 0x1;
        b_var6 = param_2 == string_1[0];
        if b_var6 {
            break;
        }
    }
    if !b_var6 {
        if param_2 != '\0' as u8 {
            return 0x0;
        }
        string_2 = string_2 + 0x1;
    }
    return string_2 + -0x1;
}

pub fn pass1_1000_47a4(ctx: &mut Globals, param_1: u32, param_2: u32, param_3: u16) -> u16 {
    let pb_var1: U32Ptr;
    let b_var2: u8;
    let pu_var3: U32Ptr;
    let pb_var4: U32Ptr;
    let i_var5: i16;
    let pb_var6: U32Ptr;
    let pu_var7: U32Ptr;
    let u_var8: u16;
    // let local_22: [u16; 0x10];
    let mut local_22: U32Ptr = 0;

    pu_var7 = local_22;
    // TODO: refactor for loop
    // for (i_var5 = 0x10; i_var5 != 0x0; i_var5 += -0x1) {
    //   pu_var3 = pu_var7;
    //   pu_var7 = pu_var7 + 0x1;
    //   *pu_var3 = 0x0;
    // }
    pb_var6 = param_2;
    loop {
        pb_var1 = pb_var6;
        pb_var6 = pb_var6 + 0x1;
        b_var2 = *pb_var1;
        if b_var2 == 0x0 {
            break;
        }
        pb_var1 = (local_22 + (b_var2 >> 0x3));
        *pb_var1 = *pb_var1 | '\x01' << (b_var2 & 0x7);
    }
    pb_var1 = param_1;
    if param_1 == 0x0 {
        pb_var1 = ctx.pbRam105061e4;
    }
    loop {
        ctx.pbRam105061e4 = pb_var1;
        // u_var8 = (pbRam105061e4 >> 0x10);
        pb_var6 = (ctx.pbRam105061e4 + 0x1);
        b_var2 = *ctx.pbRam105061e4;
        if b_var2 == 0x0 {
            return 0x0;
        }
        pb_var1 = (ctx.pbRam105061e4 & 0xffff0000 | ZEXT24(pb_var6 as u16));
        if (('\x01' << (b_var2 & 0x7) & (local_22 + (b_var2 >> 0x3))) != 0x0) == false {
            break;
        }
    }
    loop {
        pb_var4 = pb_var6;
        b_var2 = *pb_var4;
        if b_var2 == 0x0 {
            // goto
            // LAB_1000_483c;
        }
        pb_var6 = pb_var4 + 0x1;
        if (('\x01' << (b_var2 & 0x7) & (local_22 + (b_var2 >> 0x3))) == 0x0) == false {
            break;
        }
    }
    *pb_var4 = 0x0;
    pb_var4 = pb_var4 + 0x1;
    //LAB_1000_483c:
    ctx.pbRam105061e4 = (ctx.pbRam105061e4 & 0xffff0000 | ZEXT24(pb_var4 as u16));
    return ctx.pbRam105061e4;
}

pub unsafe fn pass1_1000_484c(param_1: &mut u32, param_2: &mut u32, param_3: &mut u16) -> u16 {
    let pb_var1: U32Ptr;
    let pb_var2: U32Ptr;
    let mut i_var3: i16 = 0;
    let u_var4: u16;
    let u_var5: u16;
    let pb_var6: U32Ptr;
    let pb_var7: U32Ptr;
    let mut i_var8 = 0i16;
    let b_var9: bool;
    let b_var10: bool;

    if param_3 == 0x0 {
        return 0x0;
    }
    loop {
        // i_var8 = (param_2 >> 0x10);
        pb_var7 = *param_2;
        // i_var3 = (param_1 >> 0x10);
        pb_var6 = *param_1;
        u_var4 = !pb_var7 as u16;
        u_var4 = ((param_3 - 0x1) - u_var4 & -(param_3 - 0x1 < u_var4)) + u_var4;
        u_var5 = !pb_var6 as u16;
        u_var4 = (u_var4 - u_var5 & -(u_var4 < u_var5)) + u_var5 + 0x1;
        b_var9 = *param_3 < u_var4;
        *param_3 -= u_var4;
        b_var10 = param_3 == 0x0;
        loop {
            if u_var4 == 0x0 {
                break;
            }
            u_var4 -= 0x1;
            pb_var2 = pb_var7;
            pb_var7 = pb_var7 + 0x1;
            pb_var1 = pb_var6;
            pb_var6 = pb_var6 + 0x1;
            b_var9 = *pb_var1 < *pb_var2;
            b_var10 = *pb_var1 == *pb_var2;
            if b_var10 == false {
                break;
            }
        }
        *param_2 = *param_2 & 0xffff0000 | ZEXT24(pb_var7 as u16);
        // if !b_var10 {
        //     return (0x1 - b_var9) - (b_var9 != 0x0);
        // }
        if param_3 == 0x0 {
            return u_var4;
        }
        if pb_var6 == 0x0 {
            i_var3 += 0x6c;
        }
        *param_1 = CONCAT22(i_var3 as u16, pb_var6 as u16);
        if pb_var7 == 0x0 {
            *param_2 = ((i_var8 + 0x6c) << 0x10) as u32;
            *param_1 = CONCAT22(i_var3 as u16, pb_var6 as u16);
        }
    }
}

pub fn pass1_1000_48a8(param_1: &mut u32, param_2: &mut u32, param_3: &mut i16) -> u16 {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let mut i_var3 = 0i16;
    let u_var4: u16;
    let u_var5: u16;
    let pu_var6: U32Ptr;
    let pu_var7: U32Ptr;
    let i_var8: i16 = 0;

    if param_3 != 0x0 {
        loop {
            // i_var3 = (param_2 >> 0x10);
            pu_var6 = *param_2;
            // i_var8 = (param_1 >> 0x10);
            pu_var7 = *param_1;
            u_var4 = !pu_var7 as u16;
            u_var4 = ((param_3 - 0x1) - u_var4 & -(param_3 - 0x1 < u_var4)) + u_var4;
            u_var5 = !pu_var6 as u16;
            u_var4 = (u_var4 - u_var5 & -(u_var4 < u_var5)) + u_var5 + 0x1;
            *param_3 -= u_var4;
            // TODO: refactor for loop
            // for (u_var5 = u_var4 >> 0x1; u_var5 != 0x0; u_var5 -= 0x1) {
            //   pu_var2 = pu_var7;
            //   pu_var7 = pu_var7 + 0x1;
            //   pu_var1 = pu_var6;
            //   pu_var6 = pu_var6 + 0x1;
            //   *pu_var2 = *pu_var1;
            // }
            // TODO: refactor for loop
            // for (u_var4 = ((u_var4 & 0x1) != 0x0); u_var4 != 0x0; u_var4 -= 0x1) {
            //   pu_var2 = pu_var7;
            //   pu_var7 = (pu_var7 + 0x1);
            //   pu_var1 = pu_var6;
            //   pu_var6 = (pu_var6 + 0x1);
            //   *pu_var2 = *pu_var1;
            // }
            if param_3 == 0x0 {
                break;
            }
            if pu_var6 == 0x0 {
                i_var3 += 0x6c;
            }
            *param_1 = *param_1 & 0xffff0000 | ZEXT24(pu_var7 as u16);
            *param_2 = CONCAT22(i_var3 as u16, pu_var6 as u16);
            if pu_var7 == 0x0 {
                *param_1 = ((i_var8 + 0x6c) << 0x10) as u32;
                *param_2 = CONCAT22(i_var3 as u16, pu_var6 as u16);
            }
        }
    }
    return *param_1 as u16;
}

pub fn pass1_1000_4906(
    struct_1: &mut Struct20,
    in_wnd_class: Option<&WNDCLASS16>,
    param_3: u16,
) -> Struct20 {
    let pu_var1: U32Ptr;
    let u_var2: u8;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let pu_var7: &mut Struct20;
    let i_var8: i16;

    if param_3 != 0x0 {
        // i_var8 = (param_1 >> 0x10);
        u_var5 = !struct_1.field_0x0;
        u_var6 = param_3;
        if u_var5 != 0x0 {
            u_var6 = (u_var5 - param_3 & -(u_var5 < param_3)) + param_3;
            u_var5 = param_3 - u_var6;
        }
        u_var3 = in_wnd_class & 0xff | in_wnd_class << 0x8;
        pu_var7 = struct_1;
        // TODO: refactor for loop
        // for (u_var4 = u_var6 >> 0x1; u_var4 != 0x0; u_var4 -= 0x1) {
        //   pu_var1 = pu_var7;
        //   pu_var7 = pu_var7 + 0x1;
        //   *pu_var1 = u_var3;
        // }
        // for (u_var6 = ((u_var6 & 0x1) != 0x0);
        //     u_var2 = (in_wnd_class & 0xff), u_var6 != 0x0; u_var6 -= 0x1) {
        //   pu_var1 = pu_var7;
        //   pu_var7 = (pu_var7 + 0x1);
        //   *pu_var1 = u_var2;
        // }
        if u_var5 != 0x0 {
            // TODO: refactor for loop
            // for (u_var6 = u_var5 >> 0x1; u_var6 != 0x0; u_var6 -= 0x1) {
            //   pu_var1 = pu_var7;
            //   pu_var7 = pu_var7 + 0x1;
            //   *pu_var1 = u_var3;
            // }
            // for (u_var6 = ((u_var5 & 0x1) != 0x0); u_var6 != 0x0; u_var6 -= 0x1) {
            //   pu_var1 = pu_var7;
            //   pu_var7 = (pu_var7 + 0x1);
            //   *pu_var1 = u_var2;
            // }
        }
    }
    return struct_1;
}

pub fn pass1_1000_49b2(param_1: u16) -> i16 {
    return ((param_1 ^ param_1 >> 0xf) - (param_1 >> 0xf)) as i16;
}

pub fn pass1_1000_49c6(
    ctx: &mut Globals,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: &mut u16,
    param_6: u16,
    param_7: U32Ptr,
    param_8: i16,
) -> u16 {
    let u_var1: u16;
    let u_var2: u16;
    let u_var3: u16;
    let i_var4: i16;
    let l_var5: u32;
    let u_stack20: u16;
    let u_stack18: u16;
    let u_stack14: u16;
    let u_stack12: u16;
    let u_stack10: i16;
    let u_stack8: u16;
    let u_stack6: u16;
    let local_4: u16;
    let i_stack2: i16;

    i_stack2 = param_8 + 0x1;
    local_4 = SUB42(ctx.data_seg, 0x0) as u16;
    u_stack20 = param_3;
    u_stack18 = param_4;
    l_var5 = pass1_1000_52be(param_5 - 0x1, u16::from(-(param_5 == 0x0)), param_6, 0x0);
    u_stack8 = (l_var5 + 0x8) as u16;
    u_stack6 = (((l_var5 + 0x8) >> 0x10) * 0x100 + param_4) as u16;
    loop {
        if u_stack6 < u_stack18 {
            return 0x0;
        }
        if (u_stack6 <= u_stack18) && (u_stack8 < u_stack20) {
            return 0x0;
        }
        u_stack14 = param_5 >> 0x1;
        if u_stack14 == 0x0 {
            if (param_5 != 0x0) && (i_var4 = (*param_7)(), i_var4 == 0x0) {
                return u_stack20;
            }
            return 0x0;
        }
        u_var1 = u_stack14;
        if (param_5 & 0x1) == 0x0 {
            u_var1 = u_stack14 - 0x1;
        }
        u_var2 = (u_var1 * param_6);
        u_var3 = u_var2 + u_stack20;
        let iStack10 = ((u_var1 * param_6 >> 0x10) + CARRY2(u_var2, u_stack20)) * 0x100 + u_stack18;
        u_stack12 = u_var3;
        i_var4 = (*param_7)();
        if (i_var4 == 0x0) {
            break;
        }
        if (i_var4 < 0x0) {
            u_stack8 = -param_6 + u_stack12;
            u_stack6 = (CARRY2(-param_6, u_stack12) - (param_6 != 0x0)) * 0x100 + iStack10;
            u_var1 = param_5 & 0x1;
            *param_5 = u_stack14;
            if (u_var1 == 0x0) {
                *param_5 = u_stack14 - 0x1;
            }
        } else {
            u_stack20 = param_6 + u_stack12;
            u_stack18 = CARRY2(param_6, u_stack12) * 0x100 + iStack10;
            *param_5 = u_stack14;
        }
    }
    return u_var3;
}

pub fn pass1_1000_4aea(
    ctx: &mut Globals,
    param_1: &mut u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: U32Ptr,
    param_6: i16,
    param_7: i16,
    param_8: u16,
    param_9: u16,
    param_10: u16,
) {
    let pu_var1: U32Ptr;
    let ppc_var2: u32;
    let l_var3: i32;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let mut u_var7: i16;
    let i_var8: i16;
    let u_var9: u16;
    let u_var10: u16;
    let pu_var11: &mut Struct171;
    let u_var11: i16;
    let u_var12: u16;
    let b_var13: bool;
    let u_var14: u16;
    let u_var15: u16;
    let u_var16: u16;
    let u_var17: u16;
    let u_var18: u16;
    let u_var19: u16;
    let u_stack4: u16;
    let i_stack2: i16;

    i_stack2 = param_6 + 0x1;
    u_stack4 = SUB42(ctx.data_seg, 0x0) as u16;
    u_var12 = SUB42(ctx.data_seg, 0x0) as u16;
    if ((param_4 != 0x0) && (param_3 != 0x0)) {
        // TODO: refactor for loop
        //       for (i_var8 = param_3 + -0x1; i_var8 != 0x0; i_var8 += -0x1) {
        //       i_var4 = (*param_5)(param_9);
        //       if (i_var4 < 0x0) {
        //         u_var5 = param_3 - 0x1;
        //         i_var8 = 0x0;
        //         loop {
        //           u_var5 >>= 0x1;
        //           i_var8 += -0x1;
        //       if (i_var8 != 0x0 && u_var5 != 0x0) == false { break;}
        //         }
        //         if (((-i_var8 * 0x8 >> 0x10) != 0x0) ||
        //            (u_var5 = pass1_1000_3bac(), u_var5 < (-i_var8 * 0x8))) {
        //           exit_1000_25f2(0x4b7b,param_9,param_7,-0x4,param_8,param_9,param_10);
        //           return;
        //         }
        //         pu_var11 = &stack0xfff6;
        //         l_var3 = (param_3 - 0x1) * param_4;
        //         u_var6 = l_var3;
        //         u_var5 = u_var6 + param_1;
        //         u_var6 = ((l_var3 >> 0x10) + CARRY2(u_var6,param_1)) * 0x100 +
        //                 param_2;
        // //LAB_1000_4b7d:
        //         if (pu_var11 <= &stack0xffee) {
        //           return;
        //         }
        // //LAB_1000_4b81:
        //         if ((param_2 < u_var6) || ((param_2 <= u_var6 && (param_1 < u_var5)))) {
        //           pu_var1 = &pu_var11.field_0x14;
        //           u_var10 = u_var5 + *pu_var1;
        //           u_var9 = u_var6 + (-CARRY2(u_var5,*pu_var1) & 0x6c);
        //           u_var14 = param_1;
        //           u_var15 = param_2;
        //           u_var18 = u_var5;
        //           u_var19 = u_var6;
        //           u_var7 = param_1;
        //           u_var11 = param_2;
        // //LAB_1000_4bbc:
        //           loop {
        //             pu_var1 = &pu_var11.field_0x14;
        //             b_var13 = CARRY2(param_1,*pu_var1);
        //             param_1 += *pu_var1;
        //             param_2 += -b_var13 & 0x6c;
        //             if ((param_1 != u_var18) || (param_2 != u_var19)) {
        //               ppc_var2 = &pu_var11.field_0x16;
        //               i_var8 = (**ppc_var2)(param_9,param_1,param_2,u_var7,u_var11);
        //               if (i_var8 < 0x1) {
        //                 if (i_var8 != 0x0) {
        //                   u_var14 = param_1;
        //                   u_var15 = param_2;
        //                 }
        // //                 TODO: goto LAB_1000_4bbc;
        //               }
        //             }
        //             loop {
        //               u_var17 = u_var6;
        //               u_var16 = u_var5;
        //               pu_var1 = &pu_var11.field_0x14;
        //               b_var13 = u_var10 < *pu_var1;
        //               u_var10 -= *pu_var1;
        //               u_var9 -= -b_var13 & 0x6c;
        //               ppc_var2 = &pu_var11.field_0x16;
        //               i_var8 = (**ppc_var2)(param_9,u_var7,u_var11,u_var10,u_var9);
        //               if (0x0 < i_var8) break;
        //               u_var5 = u_var10;
        //               u_var6 = u_var9;
        //             } while (((i_var8 != 0x0) || (u_var5 = u_var16, u_var6 = u_var17, u_var10 != u_var7))
        //                     || (u_var9 != u_var11));
        //             if ((u_var9 < param_2) || ((u_var9 <= param_2 && (u_var10 <= param_1))))
        // //             TODO: goto LAB_1000_4c58;
        //             pass1_1000_4ceb(pu_var11.field_0x14,param_1,u_var10,u_var9);
        //             u_var14 = param_1;
        //             u_var15 = param_2;
        //             u_var5 = u_var10;
        //             u_var6 = u_var9;
        //           } while( true );
        //         }
        // //         TODO: goto LAB_1000_4b7d;
        //       }
        //     }
    }
    return;
    //LAB_1000_4c58:
    *param_1 = u_var7 as u16;
    *param_2 = u_var11;
    pass1_1000_4ceb(pu_var11.field_0x14, &mut u_var7, u_var10 as i16, u_var9);
    u_var11 = ((u_var19 - (-(u_var18 < u_var16) & 0x6c)) - u_var17)
        + (-CARRY2(u_var18 - u_var16, *param_1) & 0x6c)
        + param_2;
    u_var7 = -((u_var18 - u_var16) + param_1 < u_var14) & 0x6c;
    u_var5 = u_var14;
    u_var6 = u_var15;
    if (u_var7 <= u_var11) && (u_var15 <= (u_var11 - u_var7) as u16) {
        u_var5 = u_var18;
        u_var6 = u_var19;
        *param_1 = u_var16;
        *param_2 = u_var17;
    }
    //   TODO: goto LAB_1000_4b81;
}

pub fn pass1_1000_4ceb(param_1: &mut u16, param_2: &mut i16, param_3: i16, param_4: u16) {
    let pu_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let u_var3: u8;
    let u_var4: u16;

    if (*param_1 & 0x1) != 0x0 {
        *param_1 -= 0x1;
        pu_var1 = (*param_1 + param_3) as u32;
        u_var3 = *pu_var1;
        *pu_var1 = *(param_1 + param_2);
        *(param_1 + param_2) = u_var3;
        if param_1 == 0x0 {
            return;
        }
    }
    loop {
        *param_1 -= 0x2;
        pu_var2 = (*param_1 + param_3) as u32;
        u_var4 = *pu_var2;
        *pu_var2 = (param_1 + param_2);
        (param_1 + param_2) = u_var4;
        if param_1 == 0 {
            break;
        }
    }
    return;
}

pub fn pass1_1000_4d0c(ctx: &mut Globals, param_1: u16) {
    ctx.DAT_1050_61e8 = param_1 as u32;
    ctx.PTR_LOOP_1050_61ea = 0x0;
    return;
}

pub fn pass1_1000_4d24(ctx: &mut Globals) -> u16 {
    let l_var1: i32;

    l_var1 = pass1_1000_52be(
        ctx.DAT_1050_61e8 as u16,
        ctx.PTR_LOOP_1050_61ea as u16,
        ctx.s_TPPnpmOPMENU_1050_43fa[0x3..].to_string(),
        0x3,
    ) as i32;
    ctx.PTR_LOOP_1050_61ea = ((l_var1 + 0x269ec3) >> 0x10) as u32;
    ctx.DAT_1050_61e8 = (l_var1 + 0x269ec3) as u32;
    return (ctx.PTR_LOOP_1050_61ea & 0x7fff) as u16;
}

pub unsafe fn pass1_1000_4f1a(
    ctx: &mut Globals,
    param_1: i16,
    param_2: u16,
    param_3: u16,
) -> U32Ptr {
    let pi_var1: U32Ptr;
    let mut pc_var2: String;
    let mut string_1: String;
    let pi_var3: U32Ptr;
    let pi_var4: U32Ptr;
    let mut pc_var5: String;
    let i_var6: i16;
    let i_var7: i16;

    i_var7 = 0x19;
    i_var6 = 0x19;
    pass1_1000_25a8(ctx, param_2, param_3);
    pass1_1000_2913(ctx, i_var6, param_2, param_3);
    string_1 = str_op_1000_28dc(ctx, i_var7);
    if string_1 != 0x0 {
        i_var6 = 0x9;
        if string_1[0] == 'M' {
            i_var6 = 0xf;
        }
        string_1 = string_1[i_var6..].to_string();
        i_var6 = 0x22;
        pc_var5 = string_1;
        loop {
            if i_var6 == 0x0 {
                break;
            }
            i_var6 += -0x1;
            pc_var2 = pc_var5;
            pc_var5 = pc_var5[1..].to_string();
            if pc_var2 == '\r' {
                break;
            }
        }
        pc_var5[-0x1] = '\0';
    }
    FatalAppExit16(param_3, &string_1);
    FatalExit();
    pi_var4 = ctx.PTR_LOOP_1050_63fe;
    loop {
        pi_var1 = pi_var4;
        pi_var4 = pi_var4 + 0x1;
        i_var6 = *pi_var1;
        pi_var3 = pi_var4;
        pi_var3 = (i_var6 + 0x1) as u32;
        if (i_var6 == param_1) || (pi_var3 == 0x0) {
            return pi_var3;
        }
        i_var6 = -0x1;
        loop {
            if i_var6 == 0x0 {
                break;
            }
            i_var6 += -0x1;
            pi_var1 = pi_var4;
            pi_var4 = (pi_var4 + 0x1);
            if *pi_var1 == '\0' {
                break;
            }
        }
    }
}

pub fn pass1_1000_4f2e(ctx: &mut Globals, param_1: u16) -> u16 {
    let func_ptr_1: u32;
    let mut u_var2: u16;
    let u_var3: u8;

    let mut u_var2 = 0x3b50;
    let mut u_var3 = 0x0;
    if true {
        let func_ptr_1 = swi(0x21);
        u_var2 = (func_ptr_1)(ctx.data_seg, param_1 + 0x1);
    } else {
        DOS3Call(&mut ctx.PTR_LOOP_1050_1000);
    }
    if !u_var3 {
        return 0x0;
    }
    pass1_1000_29b5(ctx, &mut u_var2);
    return 0xffff;
}

pub fn pass1_1000_5008(param_1: u16, param_2: u16, param_3: u16, param_4: i16) {
    let unaff_cs: u16 = 0;
    let unaff_ss: u16 = 0;
    let i_stack2: i16;

    i_stack2 = param_4 + 0x1;
    pass1_1000_5026(
        0x0, param_1, param_2, param_3, &i_stack2, unaff_cs, unaff_ss,
    );
    return;
}

pub fn pass1_1000_5026(a: u16, b: u16, c: u16, d: u16, e: &u16, f: u16, g: u16) {
    unimplemented!()
}

pub fn pass1_1000_5224(param_1: u16, param_2: u16, param_3: &mut u16, param_4: u16) -> u32 {
    let u_var1: u32;
    let l_var2: i32;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let u_var8: u16;
    let b_var10: bool;
    let c_var11: u8;
    let u_var9: u16;

    c_var11 = u8::from(param_2 < 0x0);
    if c_var11 {
        b_var10 = *param_1 != 0x0;
        *param_1 = -*param_1;
        *param_2 = -b_var10 - *param_2;
    }
    if param_4 < 0x0 {
        c_var11 += '\x01';
        b_var10 = param_3 != 0x0;
        *param_3 = -*param_3;
        *param_4 = -b_var10 - param_4;
    }
    u_var3 = param_1;
    u_var5 = *param_3;
    u_var6 = param_2;
    u_var9 = param_4;
    if param_4 == 0x0 {
        u_var3 = param_2 / param_3;
        i_var4 = ((param_2 % param_3 << 0x10 | param_1) / param_3);
    } else {
        loop {
            u_var8 = u_var9 >> 0x1;
            u_var5 = u_var5 >> 0x1 | ((u_var9 & 0x1) != 0x0) << 0xf;
            u_var7 = u_var6 >> 0x1;
            u_var3 = u_var3 >> 0x1 | ((u_var6 & 0x1) != 0x0) << 0xf;
            u_var6 = u_var7;
            u_var9 = u_var8;
            if u_var8 == 0x0 {
                break;
            }
        }
        u_var1 = CONCAT22(u_var7, u_var3) / u_var5;
        i_var4 = u_var1 as i16;
        l_var2 = param_3 * (u_var1 & 0xffff);
        // u_var3 = (l_var2 >> 0x10);
        u_var5 = u_var3 + i_var4 * param_4;
        if ((CARRY2(u_var3, (i_var4 * param_4) as u16)) || u32::from((param_2 < u_var5)))
            || (param_2 <= u_var5 && (param_1 < l_var2 as u16))
        {
            i_var4 += -0x1;
        }
        u_var3 = 0x0;
    }
    if c_var11 == '\x01' as u8 {
        b_var10 = i_var4 != 0x0;
        i_var4 = -i_var4;
        u_var3 = -b_var10 - u_var3;
    }
    return CONCAT22(u_var3, i_var4 as u16);
}

pub fn pass1_1000_52be(param_1: u16, param_2: u16, param_3: u16, param_4: u16) -> u32 {
    if (param_4 | param_2) == 0x0 {
        return (param_1 * param_3) as u32;
    }
    return (param_1 * param_3 & 0xffff
        | ((param_1 * param_3 >> 0x10) + param_2 * param_3 + param_1 * param_4) << 0x10)
        as u32;
}

pub fn pass1_1000_52f0(
    param_1: &mut u16,
    param_2: &mut u16,
    param_3: &mut u16,
    param_4: u16,
) -> u32 {
    let u_var1: u32;
    let l_var2: i32;
    let u_var3: u16;
    let u_var4: u16;
    let i_var5: i16;
    let i_var6: i16;
    let u_var7: u16;
    let u_var8: u16;
    let u_var9: u16;
    let u_var10: u16;
    let u_var11: u16;
    let b_var12: bool;
    let b_var13: bool;

    b_var13 = *param_2 < 0x0;
    if b_var13 {
        b_var12 = *param_1 != 0x0;
        *param_1 = -*param_1;
        *param_2 = -b_var12 - *param_2;
    }
    u_var11 = u16::from(b_var13);
    if param_4 < 0x0 {
        b_var13 = *param_3 != 0x0;
        *param_3 = -*param_3;
        *param_4 = -b_var13 - *param_4;
    }
    u_var3 = *param_1;
    u_var4 = *param_3;
    u_var8 = *param_2;
    u_var9 = *param_4;
    if (param_4 == 0x0) {
        i_var5 = ((*param_2 % *param_3 << 0x10 | *param_1) % *param_3) as i16;
        i_var6 = 0x0;
        if ((u_var11 - 0x1) < 0x0) {
            // goto
            // LAB_1000_538a;
        }
    } else {
        loop {
            u_var10 = u_var9 >> 0x1;
            u_var4 = u_var4 >> 0x1 | ((u_var9 & 0x1) != 0x0) << 0xf;
            u_var7 = u_var8 >> 0x1;
            u_var3 = u_var3 >> 0x1 | ((u_var8 & 0x1) != 0x0) << 0xf;
            u_var8 = u_var7;
            u_var9 = u_var10;
            if u_var10 == 0 {
                break;
            }
        }
        u_var1 = CONCAT22(u_var7, u_var3) / u_var4;
        u_var3 = (u_var1 * param_4) as u16;
        l_var2 = (u_var1 & 0xffff) * param_3;
        // u_var8 = (l_var2 >> 0x10);
        u_var4 = l_var2 as u16;
        u_var9 = u_var8 + u_var3;
        if ((CARRY2(u_var8, u_var3)) > 0 || (*param_2 < u_var9))
            || (*param_2 <= u_var9 && (*param_1 < u_var4))
        {
            b_var13 = u_var4 < *param_3;
            u_var4 -= param_3;
            u_var9 = (u_var9 - param_4) - b_var13;
        }
        i_var5 = u_var4 - param_1;
        i_var6 = (u_var9 - param_2) - (u_var4 < *param_1);
        if -0x1 < (u_var11 - 0x1) {
            // goto
            // LAB_1000_538a;
        }
    }
    b_var13 = i_var5 != 0x0;
    i_var5 = -i_var5;
    i_var6 = -b_var13 - i_var6;
    //LAB_1000_538a:
    return CONCAT22(i_var6 as u16, i_var5 as u16);
}

pub fn pass1_1000_5390(param_1: u16, param_2: u16, param_3: u16, param_4: u16) -> u32 {
    let u_var1: u32;
    let l_var2: i32;
    let u_var3: u16;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let u_var8: u16;
    let u_var9: u16;

    u_var3 = param_1;
    u_var8 = param_4;
    u_var6 = param_2;
    u_var9 = param_3;
    if param_4 == 0x0 {
        u_var3 = param_2 / param_3;
        i_var4 = ((param_2 % param_3 << 0x10 | param_1) / param_3) as i16;
    } else {
        loop {
            u_var5 = u_var8 >> 0x1;
            u_var9 = u_var9 >> 0x1 | ((u_var8 & 0x1) != 0x0) << 0xf;
            u_var7 = u_var6 >> 0x1;
            u_var3 = u_var3 >> 0x1 | ((u_var6 & 0x1) != 0x0) << 0xf;
            u_var8 = u_var5;
            u_var6 = u_var7;
            if u_var5 == 0 {
                break;
            }
        }
        u_var1 = CONCAT22(u_var7, u_var3) / u_var9;
        i_var4 = u_var1 as i16;
        l_var2 = (param_3 * (u_var1 & 0xffff)) as i32;
        // u_var3 = (l_var2 >> 0x10);
        u_var8 = u_var3 + i_var4 * param_4;
        if ((CARRY2(u_var3, (i_var4 * param_4) as u16)) || u32::from((param_2 < u_var8)))
            || (param_2 <= u_var8 && (param_1 < l_var2 as u16))
        {
            i_var4 += -0x1;
        }
        u_var3 = 0x0;
    }
    return CONCAT22(u_var3, i_var4 as u16);
}

pub fn pass1_1000_53f0(param_1: u16, param_2: u16, param_3: u16, param_4: u16) -> u32 {
    let u_var1: u32;
    let l_var2: i32;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let i_var6: i16;
    let i_var7: i16;
    let u_var8: u16;
    let u_var9: u16;
    let u_var10: u16;
    let b_var11: bool;

    u_var3 = param_1;
    u_var4 = param_4;
    u_var9 = param_2;
    u_var10 = param_3;
    if (param_4 == 0x0) {
        i_var6 = ((param_2 % param_3 << 0x10 | param_1) % param_3) as i16;
        i_var7 = 0x0;
    } else {
        loop {
            u_var5 = u_var4 >> 0x1;
            u_var10 = u_var10 >> 0x1 | ((u_var4 & 0x1) != 0x0) << 0xf;
            u_var8 = u_var9 >> 0x1;
            u_var3 = u_var3 >> 0x1 | ((u_var9 & 0x1) != 0x0) << 0xf;
            u_var4 = u_var5;
            u_var9 = u_var8;
            if u_var5 == 0 {
                break;
            }
        }
        u_var1 = CONCAT22(u_var8, u_var3) / u_var10;
        u_var3 = (u_var1 * param_4) as u16;
        l_var2 = ((u_var1 & 0xffff) * param_3) as i32;
        // u_var9 = (l_var2 >> 0x10);
        u_var4 = l_var2 as u16;
        u_var10 = u_var9 + u_var3;
        if (((CARRY2(u_var9, u_var3)) || u32::from((param_2 < u_var10)))
            || (param_2 <= u_var10 && (param_1 < u_var4)))
        {
            b_var11 = u_var4 < param_3;
            u_var4 -= param_3;
            u_var10 = (u_var10 - param_4) - b_var11;
        }
        i_var6 = -(u_var4 - param_1) as i16;
        i_var7 = -(u_var4 - param_1 != 0x0) - ((u_var10 - param_2) - (u_var4 < param_1));
    }
    return CONCAT22(i_var7 as u16, i_var6 as u16);
}

pub fn pass1_1000_545a(param_1: u32, param_2: u32) -> i16 {
    let pb_var1: U32Ptr;
    let b_var2: u8;
    let b_var3: u8;
    let b_var4: u8;
    let pb_var5: U32Ptr;
    let pb_var6: U32Ptr;

    pb_var6 = param_2;
    pb_var5 = param_1;
    b_var4 = 0xff;
    loop {
        loop {
            if b_var4 == 0x0 {
                // goto
                // LAB_1000_5499;
            }
            pb_var1 = pb_var6;
            pb_var6 = pb_var6 + 0x1;
            b_var4 = *pb_var1;
            b_var3 = *pb_var5;
            pb_var5 = pb_var5 + 0x1;
            if b_var3 != b_var4 {
                break;
            }
        }
        b_var2 = b_var4 + 0xbf + (-((b_var4 + 0xbf) < 0x1a) & 0x20) + 0x41;
        b_var3 += 0xbf;
        b_var4 = b_var3 + (-(b_var3 < 0x1a) & 0x20) + 0x41;
        if b_var4 != b_var2 {
            break;
        }
    }
    b_var4 = (b_var4 < b_var2) * -0x2 + 0x1;
    //LAB_1000_5499:
    return b_var4 as i16;
}

pub fn pass1_1000_54a0(param_1: u32, param_2: u16, param_3: u16) -> u16 {
    let pu_var1: U32Ptr;
    let u_var2: u8;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u16;
    let pu_var7: U32Ptr;
    let i_var8: i16;

    if (param_3 != 0x0) {
        // i_var8 = (param_1 >> 0x10);
        u_var5 = -param_1 as u16;
        u_var6 = param_3;
        if (u_var5 != 0x0) {
            u_var6 = (u_var5 - param_3 & -(u_var5 < param_3)) + param_3;
            u_var5 = param_3 - u_var6;
        }
        u_var3 = param_2 & 0xff | param_2 << 0x8;
        pu_var7 = param_1;
        // TODO: refactor for loop
        // for (u_var4 = u_var6 >> 0x1; u_var4 != 0x0; u_var4 -= 0x1) {
        //   pu_var1 = pu_var7;
        //   pu_var7 = pu_var7 + 0x1;
        //   *pu_var1 = u_var3;
        // }
        // for (u_var6 = ((u_var6 & 0x1) != 0x0); u_var2 = (param_2 & 0xff),
        //     u_var6 != 0x0; u_var6 -= 0x1) {
        //   pu_var1 = pu_var7;
        //   pu_var7 = (pu_var7 + 0x1);
        //   *pu_var1 = u_var2;
        // }
        if u_var5 != 0x0 {
            // TODO: refactor for loop
            // for (u_var6 = u_var5 >> 0x1; u_var6 != 0x0; u_var6 -= 0x1) {
            //   pu_var1 = pu_var7;
            //   pu_var7 = pu_var7 + 0x1;
            //   *pu_var1 = u_var3;
            // }
            // for (u_var6 = ((u_var5 & 0x1) != 0x0); u_var6 != 0x0; u_var6 -= 0x1) {
            //   pu_var1 = pu_var7;
            //   pu_var7 = (pu_var7 + 0x1);
            //   *pu_var1 = u_var2;
            // }
        }
    }
    return param_1 as u16;
}

pub fn pass1_1000_54e8(
    param_1: U32Ptr,
    param_2: u16,
    param_3: i16,
    param_4: i16,
    param_5: i16,
    param_6: u16,
) {
    let i_var1: i16;
    let i_var2: i16;
    let u_stack14: u16;
    let u_stack10: i16;
    let u_stack8: u16;

    i_var2 = param_5 + param_4 * param_3;
    i_var1 = param_3;
    while (i_var1 += -0x1, -0x1 < i_var1) {
        i_var2 -= param_4;
        u_stack8 = param_6;
        u_stack14 = 0x5506;
        let iStack10 = i_var2;
        (*param_1)();
    }
    return;
}

pub fn pass1_1000_5512(
    param_1: &mut U32Ptr,
    param_2: u16,
    param_3: i16,
    param_4: &mut u16,
    param_5: u16,
) {
    let b_var1: bool;
    let u_stack4: u16;

    pass1_1000_52be(*param_3, *param_4, param_5, 0x0);
    loop {
        b_var1 = param_3 == 0x0;
        *param_3 += -0x1;
        *param_4 -= b_var1;
        if *param_4 < 0x0 {
            break;
        }
        u_stack4 = param_2;
        (*param_1)();
    }
    return;
}

pub fn pass1_1000_5586(
    param_1: U32Ptr,
    param_2: u16,
    param_3: i16,
    param_4: i16,
    param_5: i16,
    param_6: u16,
) {
    let i_var1: i16;
    let i_var2: i16;
    let u_stack14: u16;
    let u_stack10: i16;
    let u_stack8: u16;

    i_var1 = param_5;
    i_var2 = param_3;
    while (i_var2 += -0x1, -0x1 < i_var2) {
        u_stack8 = param_6;
        u_stack14 = 0x559b;
        let iStack10 = i_var1;
        (*param_1)();
        i_var1 += param_4;
    }
    return;
}

pub fn pass1_1000_55b1(ctx: &mut Globals, param_1: i16, param_2: u16, param_3: u16) -> String {
    let string_6: &mut String;
    let mut string_2: String;
    let mut exit_reason: String;
    let string_5: &mut String;
    let string_4: &mut String;
    let mut string_3: String;
    let i_var7: i16;
    let i_var8: i16;
    let mut string_4: &mut String;

    i_var8 = 0x14;
    i_var7 = 0x14;
    pass1_1000_25a8(ctx, param_2, param_3);
    pass1_1000_2913(ctx, i_var7, param_2, param_3);
    exit_reason = str_op_1000_28dc(ctx, i_var8);
    if exit_reason != 0x0 {
        i_var7 = 0x9;
        if *exit_reason == 'M' {
            i_var7 = 0xf;
        }
        exit_reason = exit_reason[i_var7..];
        i_var7 = 0x22;
        string_3 = exit_reason;
        loop {
            if i_var7 == 0x0 {
                break;
            }
            i_var7 += -0x1;
            string_2 = string_3;
            string_3 = string_3[1..].to_string();
            if *string_2 == '\r' {
                break;
            }
        }
        string_3[-0x1] = '\0';
    }
    FatalAppExit16(param_3, &exit_reason);
    FatalExit();
    string_4 = read_string_from_addr(ctx.PTR_LOOP_1050_63fe);
    loop {
        string_6 = string_4;
        string_4 = string_4 + 0x1;
        i_var7 = string_6[0];
        string_5 = string_4;
        // string_5 = (i_var7 + 0x1);
        if (i_var7 == param_1) || (string_5 == 0x0) {
            return string_5;
        }
        i_var7 = -0x1;
        loop {
            if i_var7 == 0x0 {
                break;
            }
            i_var7 += -0x1;
            string_4 = string_4;
            string_4 = (string_4 + 0x1);
            if *string_4 == '\0' {
                break;
            }
        }
    }
}
