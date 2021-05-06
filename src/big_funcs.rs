use crate::{
    defines::{
        pass1_struct_1, AppContext, Struct199, Struct306, Struct345, Struct65, Struct657, Struct675,
    },
    draw::{
        call_load_cursor_fn_1020_7554, load_cursor_1018_7248, load_cursor_1018_72b4,
        load_cursor_1018_75d8, load_cursor_1018_782a, load_cursor_fn, load_cursor_fn_1018_6d02,
        load_cursor_fn_1018_6d38, load_cursor_fn_1018_6d6e, load_cursor_fn_1018_6da4,
        load_cursor_fn_1018_6dda, load_cursor_fn_1018_6e10, load_cursor_fn_1018_6e46,
        load_cursor_fn_1018_6e7c, load_cursor_fn_1018_6eb2, load_cursor_fn_1018_6ee8,
        load_cursor_fn_1018_6f1e, load_cursor_fn_1018_6f54, load_cursor_fn_1018_6f8a,
        load_cursor_fn_1018_6fc0, load_cursor_fn_1018_6ff6, load_cursor_fn_1018_702c,
        load_cursor_fn_1018_7062, load_cursor_fn_1018_7098, load_cursor_fn_1018_70ce,
        load_cursor_fn_1018_7104, load_cursor_fn_1018_713a, load_cursor_fn_1018_7170,
        load_cursor_fn_1018_71a6, load_cursor_fn_1018_71dc, load_cursor_fn_1018_7212,
        load_cursor_fn_1018_727e, load_cursor_fn_1018_72ea, load_cursor_fn_1018_7320,
        load_cursor_fn_1018_7356, load_cursor_fn_1018_738c, load_cursor_fn_1018_73c2,
        load_cursor_fn_1018_73f8, load_cursor_fn_1018_745e, load_cursor_fn_1018_7494,
        load_cursor_fn_1018_74ca, load_cursor_fn_1018_7500, load_cursor_fn_1018_7536,
        load_cursor_fn_1018_756c, load_cursor_fn_1018_75a2, load_cursor_fn_1018_760e,
        load_cursor_fn_1018_7644, load_cursor_fn_1018_767a, load_cursor_fn_1018_76b0,
        load_cursor_fn_1018_76e6, load_cursor_fn_1018_771c, load_cursor_fn_1018_7752,
        load_cursor_fn_1018_7788, load_cursor_fn_1018_77be, load_cursor_fn_1018_77f4,
        load_cursor_fn_1018_7896, load_cursor_fn_1018_78cc, load_cursor_fn_1018_7902,
        load_cursor_fn_1018_7938, load_cursor_fn_1018_796e, load_cursor_fn_1018_79a4,
        load_cursor_fn_1018_79da, load_cursor_fn_1018_7a10, load_cursor_fn_1018_7a46,
        load_cursor_fn_1018_7a7c, load_cursor_fn_1018_7ab2, load_cursor_fn_1018_7ae8,
        load_cursor_fn_1018_7b1e, load_cursor_fn_1018_7b54, load_cursor_fn_1018_7b8a,
        load_cursor_fn_1018_7bc0, load_cursor_fn_1018_7bf6, load_cursor_fn_1018_7c2c,
        load_cursor_fn_1018_7c62, load_cursor_fn_1018_7c98, load_cursor_fn_1018_7cce,
        load_cursor_fn_1018_7d04, load_cursor_fn_1018_7d3a, load_cursor_fn_1018_7d70,
    },
    func_ptr_funcs::call_fn_ptr_1000_24cd,
    other_funcs::big_fn_1010_b038,
    pass4_funcs::pass1_1028_b58e,
    pass7_funcs::{
        pass1_1018_66cc, pass1_1018_c958, pass1_1018_c9a6, pass1_1018_c9f4, pass1_1018_ca48,
        pass1_1018_ca96, pass1_1018_caea, pass1_1018_cb38, pass1_1018_cb86, pass1_1018_cbda,
        pass1_1018_cc28, pass1_1018_cc76, pass1_1018_ccc4, pass1_1018_cd12, pass1_1018_cd60,
        pass1_1018_cf74, pass1_1018_e230, pass1_1040_a626,
    },
    pass_funcs::{pass1_1008_57a4, pass1_1008_5b12, pass1_1008_6978, pass1_fn_1008_60e8},
    sound_funcs::{mci_fn_1020_08b6, mci_send_cmd_1008_5c5c},
    string_funcs::fn_1008_6048,
    struct_funcs::{
        process_struct_1000_179c, process_struct_1010_20ba, process_struct_1018_e5dc,
        process_struct_1018_e91e,
    },
    ui_funcs::{call_load_cursor_1020_2524, load_cursor_1018_5840},
    util::{CONCAT22, SUB21, ZEXT24},
};

pub unsafe fn call_big_fn_1040_b17c(
    ctx: &mut AppContext,
    in_pustruct_a: &mut Struct345,
    in_u32_b: u32,
) {
    let mut u32_a: u32;
    let pu16_b: *mut u16;
    let mut u16_c: u16;
    let mut u16_d: u16;
    let pstruct_e: *mut Struct345;
    let mut u16_f: u16;
    let mut u16_g: u16;
    let pustruct_h: *mut pass1_struct_1;
    let mut u32_j: u32;
    let mut u16_k: u16;
    let mut u16_m: u16;
    let mut u16_n: u16;
    let pu_var2: u16;
    let u_var1: u16;
    let u_var3: u16;

    u16_n = 0;
    loop {
        // u_var4 = (param_1 >> 0x10);
        let pustruct_p = in_pustruct_a;
        pu_var2 = pustruct_p.field_0x90;
        // if *pu_var2 == u16_n || *pu_var2 < u16_n {
        //     break;
        // }
        // todo: get value at address
        if pu_var2 == u16_n || pu_var2 < u16_n {
            break;
        }
        u_var1 = pu_var2 + 2; // value at address + offset
                              //(u16_n * 10 + u_var1 + 4) = (u16_n * 2 + in_u32_b);
        todo!();
        u16_n = u16_n + 1;
    }
    pustruct_h = process_struct_1010_20ba(ctx._g_struct_372_1050_0ed0, CONCAT22(u16_f, 3));
    pu_var2 = (*pstruct_e).field_0x90;
    u32_j = (pu_var2 + 2) as u32;
    u16_n = 0;
    // TODO
    // while (*pu_var2 != u16_n && u16_n <= *pu_var2) {
    //     pu_var2 = (*pstruct_e).field_0x90;
    //     u_var3 = u16_n;
    //     big_fn_1010_b038(pustruct_h, (pu_var2 + 6));
    //     pass1_1040_a626(u32_j, CONCAT22(u16_d, u_var3));
    //     u32_j = u32_j & 0xffff0000 | (u32_j + 10);
    //     u16_n = u16_n + 1;
    // }
    return;
}

// WARNING: Removing unreachable block (ram,0x10004090)
// WARNING: Removing unreachable block (ram,0x1000409a)
// WARNING: Removing unreachable block (ram,0x10004311)
// WARNING: Removing unreachable block (ram,0x1000438a)
// WARNING: Removing unreachable block (ram,0x10004372)
// WARNING: Removing unreachable block (ram,0x100043aa)
// WARNING: Removing unreachable block (ram,0x10004f47)
// WARNING: Removing unreachable block (ram,0x10004fa9)
// WARNING: Removing unreachable block (ram,0x10004fd7)
// WARNING: Removing unreachable block (ram,0x10004feb)
// WARNING: Removing unreachable block (ram,0x10005167)
// WARNING: Removing unreachable block (ram,0x1000518c)
// WARNING: Removing unreachable block (ram,0x100051f7)
// WARNING: Removing unreachable block (ram,0x100051c5)
// WARNING: Removing unreachable block (ram,0x100051d9)
// WARNING: Removing unreachable block (ram,0x10005214)
pub unsafe fn big_fn_1008_15d4(ctx: &mut AppContext, param_1: *mut Struct657, param_2: u16) {
    let pi32_a: *mut i32;
    let mut u32_b: u32;
    let mut i32_c: i32;
    let mut string_d: String;
    let mut i32_e: i32;
    let pustruct_f: *mut Struct199;
    let mut i32_g: i32;
    let mut extraout_dx_00: i32;
    let mut extraout_dx_01: i32;
    let mut extraout_dx_02: i32;
    let mut extraout_dx_03: i32;
    let mut extraout_dx_04: i32;
    let mut extraout_dx_05: i32;
    let mut extraout_dx_06: i32;
    let mut extraout_dx_07: i32;
    let pustruct_h: *mut Struct199;
    let mut extraout_dx_08: i32;
    let mut extraout_dx_09: i32;
    let mut extraout_dx_10: i32;
    let mut extraout_dx_11: i32;
    let mut extraout_dx_12: i32;
    let mut extraout_dx_13: i32;
    let mut extraout_dx_14: i32;
    let mut extraout_dx_15: i32;
    let mut extraout_dx_16: i32;
    let mut extraout_dx_17: i32;
    let mut extraout_dx_18: i32;
    let mut extraout_dx_19: i32;
    let mut extraout_dx_20: i32;
    let mut extraout_dx_21: i32;
    let mut extraout_dx_22: i32;
    let mut extraout_dx_23: i32;
    let mut extraout_dx_24: i32;
    let mut extraout_dx_25: i32;
    let mut extraout_dx_26: i32;
    let mut extraout_dx_27: i32;
    let mut extraout_dx_28: i32;
    let mut extraout_dx_29: i32;
    let mut extraout_dx_30: i32;
    let mut extraout_dx_31: i32;
    let mut extraout_dx_32: i32;
    let mut extraout_dx_33: i32;
    let mut extraout_dx_34: i32;
    let mut extraout_dx_35: i32;
    let mut extraout_dx_36: i32;
    let mut extraout_dx_37: i32;
    let mut extraout_dx_38: i32;
    let mut extraout_dx_39: i32;
    let mut extraout_dx_40: i32;
    let mut extraout_dx_41: i32;
    let mut extraout_dx_42: i32;
    let mut extraout_dx_43: i32;
    let mut extraout_dx_44: i32;
    let mut extraout_dx_45: i32;
    let mut extraout_dx_46: i32;
    let mut extraout_dx_47: i32;
    let mut extraout_dx_48: i32;
    let mut extraout_dx_49: i32;
    let mut extraout_dx_50: i32;
    let mut extraout_dx_51: i32;
    let mut extraout_dx_52: i32;
    let mut extraout_dx_53: i32;
    let mut extraout_dx_54: i32;
    let mut extraout_dx_55: i32;
    let mut extraout_dx_56: i32;
    let mut extraout_dx_57: i32;
    let mut extraout_dx_58: i32;
    let mut extraout_dx_59: i32;
    let mut extraout_dx_60: i32;
    let mut extraout_dx_61: i32;
    let mut extraout_dx_62: i32;
    let mut extraout_dx_63: i32;
    let mut extraout_dx_64: i32;
    let mut extraout_dx_65: i32;
    let mut extraout_dx_66: i32;
    let mut extraout_dx_67: i32;
    let mut extraout_dx_68: i32;
    let mut extraout_dx_69: i32;
    let mut extraout_dx_70: i32;
    let mut extraout_dx_71: i32;
    let mut extraout_dx_72: i32;
    let mut extraout_dx_73: i32;
    let mut extraout_dx_74: i32;
    let mut extraout_dx_75: i32;
    let mut extraout_dx_76: i32;
    let mut extraout_dx_77: i32;
    let mut extraout_dx_78: i32;
    let mut extraout_dx_79: i32;
    let mut extraout_dx_80: i32;
    let mut extraout_dx_81: i32;
    let mut extraout_dx_82: i32;
    let mut extraout_dx_83: i32;
    let mut extraout_dx_84: i32;
    let mut extraout_dx_85: i32;
    let mut extraout_dx_86: i32;
    let mut extraout_dx_87: i32;
    let mut extraout_dx_88: i32;
    let mut extraout_dx_89: i32;
    let mut extraout_dx_90: i32;
    let mut extraout_dx_91: i32;
    let mut extraout_dx_92: i32;
    let mut extraout_dx_93: i32;
    let mut i32_j: i32;
    let mut extraout_dx_94: i32;
    let mut extraout_dx_95: i32;
    let mut extraout_dx_96: i32;
    let mut extraout_dx_97: i32;
    let mut extraout_dx_98: i32;
    let mut extraout_dx_99: i32;
    let mut i32_k: i32;
    let mut i32_m: i32;
    let mut stack_seg_n: u16;
    let pustruct_p: *mut Struct65;
    let mut param_1_hi: u16;
    let mut u32_r: u32;
    let mut u32_s: u32;
    let mut u16_t: u16;
    let mut u16_v: u16;
    let mut string_w: String;
    let mut u32_x: u32;
    let mut in_1: u16;

    u32_x = 0;
    // TODO
    // i32_c = param_1;
    // TODO
    // pass1_1008_57a4(
    //     CONCAT22(stack_seg_n, string_w),
    //     param_1 & 0xffff0000 | (i32_c + 0xd2),
    // );
    let mut broke = false;
    while {
        string_d = string_w;
        // TODO
        //let in_1: *mut Struct306 = CONCAT22(stack_seg_n, string_d);
        pass1_1008_5b12(in_1);
        pustruct_f = i32_e | string_d;
        if pustruct_f == 0 {
            broke = true;
            break;
        }
        u32_b = string_d + 4;
        pustruct_f = string_d + 6;
        i32_k = u32_b;
        (i32_k + 0xde) != param_2
    } {}
    if !broke {
        u32_x = u32_b & 0xffff | ZEXT24(pustruct_f) << 0x10;
    }

    if (u32_x != 0) {
        return;
    }
    param_1_hi = (param_1 >> 0x10);
    match param_2 - 1 {
        0 => {
            process_struct_1000_179c(0xec, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                let mut input_a: *mut Struct675 = param_1;
                pass1_1008_6978(input_a, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe { *pi32_a = *pi32_a + 1 };
            mci_fn_1020_08b6(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_07;
        }
        2 => {
            process_struct_1000_179c(0xfa, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                let mut input_a: *mut Struct675 = param_1;
                pass1_1008_6978(input_a, 0, u32_x);
                return;
            }

            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            let input_a: *mut Struct65 = CONCAT22(pustruct_f, i32_k);
            process_struct_1018_e91e(input_a, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_05;
        }
        3 => {
            process_struct_1000_179c(0xf6, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;

                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_e230(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_03;
        }
        4 => {
            process_struct_1000_179c(0xf6, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            call_load_cursor_fn_1020_7554(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_04;
        }
        5 => {
            process_struct_1000_179c(0xf8, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_1018_5840(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_06;
        }
        6 => {
            process_struct_1000_179c(0xf6, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            call_load_cursor_1020_2524(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_02;
        }
        7 => {
            process_struct_1000_179c(0x118, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_accelerators_1020_41c8(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_01;
        }
        8 => {
            process_struct_1000_179c(0xf6, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            process_struct_1018_e5dc(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = i32_g;
        }
        9 => {
            process_struct_1000_179c(0xf6, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_66cc(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_00;
        }
        10 => {
            mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, 0x1d3);
            pustruct_f = pustruct_h;
            process_struct_1000_179c(0xf2, pustruct_h);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6d02(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_08;
        }
        0xb => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6d38(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_09;
        }
        0xc => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6d6e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_10;
        }
        0xd => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6da4(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_11;
        }
        0xe => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6dda(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_12;
        }
        0xf => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6e10(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_13;
        }
        0x10 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6e46(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_14;
        }
        0x11 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6e7c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_15;
        }
        0x12 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6eb2(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_16;
        }
        0x13 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6ee8(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_17;
        }
        0x14 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6f1e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_18;
        }
        0x15 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6f54(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_19;
        }
        0x16 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6f8a(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_20;
        }
        0x17 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6fc0(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_21;
        }
        0x18 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_6ff6(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_22;
        }
        0x19 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_702c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_23;
        }
        0x1a => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7062(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_24;
        }
        0x1b => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7098(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_25;
        }
        0x1c => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_70ce(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_26;
        }
        0x1d => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7104(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_27;
        }
        0x1e => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_713a(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_28;
        }
        0x20 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7170(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_29;
        }
        0x21 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_745e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_31;
        }
        0x22 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_71a6(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_32;
        }
        0x23 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_71dc(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_33;
        }
        0x24 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7212(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_34;
        }
        0x25 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }

            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_c958(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_86;
        }
        0x26 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_c9a6(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_87;
        }
        0x27 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_c9f4(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_88;
        }
        0x28 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_ca48(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_89;
        }
        _ => {
            fn_1008_6048(
                ctx.s_OpWnd__getKid__Unknown_target_mo_1050_01a3,
                pustruct_f,
                SUB21(i32_k, 0),
            );
            call_fn_ptr_1000_24cd(1);
            pass1_1008_6978(param_1, 0, u32_x);
            return;
        }

        0x29 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_ca96(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_90;
        }
        0x2a => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_caea(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_91;
        }
        0x2b => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cb38(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_92;
        }
        0x2c => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cb86(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_93;
        }
        0x2d => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pustruct_p = pass1_1018_cbda(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = (pustruct_p >> 0x10);
            i32_k = pustruct_p;
        }
        0x2e => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cc28(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_94;
        }
        0x2f => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cc76(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_95;
        }
        0x30 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_ccc4(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_96;
        }
        0x31 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cd12(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_97;
        }
        0x32 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cd60(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_98;
        }
        0x33 => {
            process_struct_1000_179c(0x114, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            pass1_1018_cf74(CONCAT22(pustruct_f, i32_k), (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_99;
        }
        0x34 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_73c2(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_30;
        }
        0x35 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7494(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_36;
        }
        0x36 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_74ca(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_37;
        }
        0x37 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7500(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_38;
        }
        0x38 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_73f8(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_39;
        }
        0x39 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7536(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_40;
        }
        0x3a => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_756c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_41;
        }
        0x3b => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_75a2(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_42;
        }
        0x3c => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_1018_75d8(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_43;
        }
        0x3d => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_760e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_44;
        }
        0x3e => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7644(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_45;
        }
        0x3f => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_767a(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_46;
        }
        0x40 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_76b0(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_47;
        }
        0x41 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_76e6(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_48;
        }
        0x42 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_771c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_49;
        }
        0x43 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7752(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_50;
        }
        0x44 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7788(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_51;
        }
        0x45 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_77be(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_52;
        }
        0x46 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_77f4(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_53;
        }
        0x47 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_1018_782a(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_54;
        }
        0x48 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_55;
        }
        0x49 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7896(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_56;
        }
        0x4a => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_78cc(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_57;
        }
        0x4b => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7902(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_58;
        }
        0x4c => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7938(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_59;
        }
        0x4d => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_796e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_60;
        }
        0x4e => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_79a4(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_61;
        }
        0x4f => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_79da(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_62;
        }
        0x50 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7a10(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_63;
        }
        0x51 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7a46(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_64;
        }
        0x52 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7a7c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_65;
        }
        0x54 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7ab2(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_66;
        }
        0x55 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7ae8(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_67;
        }
        0x56 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7b1e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_68;
        }
        0x57 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7b54(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_69;
        }
        0x58 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7b8a(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_70;
        }
        0x59 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7bc0(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_71;
        }
        0x5a => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7bf6(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_72;
        }
        0x5b => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7c2c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_73;
        }
        0x5c => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7c62(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_74;
        }
        0x5d => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7c98(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_35;
        }
        0x5e => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7cce(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_75;
        }
        0x5f => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7d04(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_76;
        }
        0x60 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7d3a(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_77;
        }
        0x61 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7d70(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_78;
        }
        0x62 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_1018_7248(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_79;
        }
        99 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_727e(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_80;
        }
        100 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_1018_72b4(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_81;
        }
        0x65 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_72ea(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_82;
        }
        0x66 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7320(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_83;
        }
        0x67 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_7356(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_84;
        }
        0x68 => {
            process_struct_1000_179c(0xf2, pustruct_f);
            if ((pustruct_f | i32_k) == 0) {
                u32_x = 0;
                pass1_1008_6978(param_1, 0, u32_x);
                return;
            }
            pi32_a = (i32_c + 0xcc);
            unsafe {
                *pi32_a = *pi32_a + 1;
            }
            load_cursor_fn_1018_738c(i32_k, pustruct_f, (i32_c + 0xcc), param_1);
            i32_j = extraout_dx_85;
        }
    };
    u32_x = CONCAT22(i32_j, i32_k);
    pass1_1008_6978(param_1, 0, u32_x);
    return;
}

fn load_accelerators_1020_41c8(
    i32_k: i32,
    pustruct_f: *mut Struct199,
    i32_c: i32,
    param_1: *mut Struct657,
) -> () {
    todo!()
}

pub unsafe fn call_big_fn_1010_1788(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
) {
    let u_var1: u8;
    let ppVar2: *mut pass1_struct_1;
    let mut in_stack_0000fff4: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar2 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000fff4 >> 0x10), 3),
    );
    u_var1 = pass1_1028_b58e(param_1_00);
    big_fn_1010_b038(ppVar2, u_var1);
    pass1_fn_1008_60e8();
    return;
}

pub unsafe fn call_big_fn_1010_1c16(param_1: u32, param_2: u32) {
    let u_var1: u8;

    u_var1 = pass1_1028_b58e(param_2);
    big_fn_1010_b038((param_1 + 0x6e), u_var1);
    pass1_fn_1008_60e8(0);
    return;
}

pub fn call_big_fn_1010_c7e2(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut extraout_dx: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    local_4 = 0;
    while (true) {
        u_var4 = (param_3 >> 0x10);
        i_var3 = param_3;
        if (param_3 == local_4 || param_3 < local_4) {
            break;
        }
        u_var1 = (i_var3 + 2);
        (local_4 * 10 + u_var1 + 4) = (local_4 * 2 + param_2);
        local_4 = local_4 + 1;
    }
    local_8 = (i_var3 + 2);
    local_4 = 0;
    while (param_3 != local_4 && local_4 <= param_3) {
        u_var2 = local_4;
        big_fn_1010_b038(param_1, (i_var3 + 6));
        pass1_1040_a626(local_8, CONCAT22(extraout_dx, u_var2));
        local_4 = local_4 + 1;
        local_8 = local_8 & 0xffff0000 | (local_8 + 10);
    }
    return;
}
