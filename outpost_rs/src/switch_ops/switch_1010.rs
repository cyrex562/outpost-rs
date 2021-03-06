// pub fn  switchD_1010:2ab5::caseD_13(param_1: u32,param_2: i16)

// {
//   let u_var1: u32;
//   let i_var2: i16;
//   let unaff_SS: u16;
//   let in_AF: u8;

//   i_var2 = param_2 * 0x8 + param_1;
//   if (((((i_var2 + 0x22) != 0x0) || ((i_var2 + 0x24) != 0x0)) ||
//       ((i_var2 + 0x26) != 0x0)) || ((i_var2 + 0x28) != 0x0)) {
//     u_var1 = (param_1 + 0xe);
//     sys_1000_3f9c(u_var1,(u_var1 >> 0x10),
//                   ctx.s__d__d__d__d_1050_14ae,ctx.data_seg,
//                   (param_2 * 0x8 + param_1 + 0x22),
//                   &stack0xfffe,param_1._2_2_,0x1000,unaff_SS,in_AF);
//     u_var1 = (param_1 + 0xa);
//     WritePrivateProfileString16
//               (&ctx.PTR_LOOP_1050_1000,u_var1,(u_var1 >> 0x10),
//                (param_1 + 0xe));
//   }
//   return;
// }

pub fn switch_1010_6646(param_1: u16, param_2: u16, param_3: U32Ptr, param_4: u16) {
    match (param_4) {
        0x83 => {
            *param_3 = 0xa;
        }

        0x84 => {
            *param_3 = 0xe;
        }

        0x85 => {
            *param_3 = 0x12;
        }

        0x86 => {
            *param_3 = 0x16;
        }

        0x87 => {
            *param_3 = 0x1a;
        }

        0x88 => {
            *param_3 = 0x1e;
        }

        0x89 => {
            *param_3 = 0x22;
        }

        _ => {
            *param_3 = 0x0;
        }
    }
    return;
}
