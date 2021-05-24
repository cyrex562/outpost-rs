use std::intrinsics::offset;

use crate::{string_ops, struct_ops, struct_ops2, sys2, sys_ops, ui_ops, winapi};
use crate::app_context::AppContext;
use crate::big_funcs::big_fn_1008_15d4;
use crate::cleanup::{destroy_win_1010_3202, window_msg_func_1010_7300};
use crate::draw::drawing_context::{get_dc_1020_921c, get_gui_dc_1018_4db0};
use crate::draw::misc::{call_draw_fn_1020_27b0, draw_1008_8288, draw_1020_9364, draw_1040_9948, draw_fn_1018_e4f2, draw_fn_1018_ec74, process_struct_1040_9252};
use crate::draw::misc::set_capture_1020_6184;
use crate::draw::palette::{realize_palette_1008_4e08, select_and_delete_palette_1020_92c4};
use crate::draw::rect::pt_in_rect_1010_40f8;
use crate::err_ops::{error_check_1000_17ce, set_error_mode_1010_8b14};
use crate::exit::fatal_app_exit_1000_3e9e;
use crate::file_ops::close::{close_file_1008_496c, close_file_1008_6dd0};
use crate::file_ops::read::{read_file_1008_7dee, read_from_file_1008_6e78};
use crate::file_ops::write::{write_to_file_1008_6e02, write_to_file_1008_7e1c};
use crate::func_ptr_funcs::{call_fn_ptr_1000_24cd, call_fn_ptr_1000_5586};
use crate::list_funcs::{clear_list_elements_1008_80d2, modify_list_1008_8168, modify_u16_list_1008_5bdc};
use crate::mem_funcs::alloc_mem::alloc_mem_1000_1708;
use crate::other_funcs::{mixed_fn_1010_830a, return3_1010_088c, return3_1010_0892, return3_1010_0898, return_10_1010_0886, set_timer_1008_91ba, switch_statement_1008_73ea, switch_stmt_1008_d818, zero_array_val_1008_5394, zero_list_1008_3e38};
use crate::pass::pass10_funcs::{pass1_1040_b040, pass1_1040_bfde, pass1_1040_ca16, pass1_1040_d0f8};
use crate::pass::pass11_funcs::{pass1_1008_c83a, pass1_1008_c85e};
use crate::pass::pass12_funcs::{pass1_1008_b38c, pass1_1008_b47a, pass1_1008_b4a0, pass1_1008_b61a, pass1_1008_b820};
use crate::pass::pass13_funcs::{pass1_1008_941a, pass1_1008_9628, pass1_1008_aa28, pass1_1008_ab54, pass1_1008_b200, pass1_1008_b366};
use crate::pass::pass14_funcs::{pass1_1008_3e0e, pass1_1008_3e76, pass1_1008_3e94, pass1_1008_3f92, pass1_1008_405c, pass1_1008_4d72, pass1_1008_4d84, pass1_1008_5784, pass1_1008_57c4, pass1_1008_5b12, pass1_1008_6562, pass1_1008_687a, pass1_1008_68ea, pass1_1008_6978, pass1_1008_6a04, pass1_1008_6d8a, pass1_fn_1008_60e8, pass1_fn_1008_612e};
use crate::pass::pass15_funcs::pass1_1020_bae6;
use crate::pass::pass17_funcs::{pass1_1030_5b00, pass1_1030_6c1a, pass1_1030_6ddc, pass1_1030_6e14, pass1_1030_73a8, pass1_1030_8326, pass1_1030_8344};
use crate::pass::pass19_funcs::{pass1_1018_e100, pass1_1018_e834, pass1_1020_022c, pass1_1040_23ea, pass1_1040_3966, pass1_1040_4068, pass1_1040_5cd6, pass1_1040_5d42, pass1_1040_5dc4, pass1_1040_5eaa, pass1_1040_6402, pass1_1040_a5d0};
use crate::pass::pass20_funcs::{pass1_1010_9044, pass1_1010_9130, pass1_1010_9172, pass1_1010_91cc, pass1_1010_9210, pass1_1010_a50c, pass1_1010_a568, pass1_1010_a58a, pass1_1010_a5ac, pass1_1010_a5ca, pass1_1010_a5ec, pass1_1010_ac62, pass1_1010_ad22, pass1_1010_ad64, pass1_1010_b028, pass1_1010_c320, pass1_1010_c3c2, pass1_1010_dd5e, pass1_1018_04b8, pass1_1018_0a50, pass1_1018_0a76, pass1_1018_0ac0, pass1_1018_0ae8, pass1_1018_0b08};
use crate::pass::pass2_funcs::pass1_1010_e964;
use crate::pass::pass4_funcs::{pass1_1028_dc52, pass1_1028_e4ec};
use crate::pass::pass6_funcs::{pass1_1038_387e, pass1_1038_801a, pass1_1038_993a, pass1_1038_a33c, pass1_1038_a494, pass1_1038_a69a, pass1_1038_a89e, pass1_1038_b6e0};
use crate::pass::pass7_funcs::{pass1_1018_179e, pass1_1018_181c, pass1_1018_1c9a, pass1_1018_1e78, pass1_1018_1f1a, pass1_1018_2678, pass1_1018_3424, pass1_1018_3710, pass1_1018_3a42, pass1_1018_3a7a, pass1_1018_3a94, pass1_1018_3cda, pass1_1018_3d44, pass1_1018_4cda, pass1_1018_4dce, pass1_1018_50ea, pass1_1018_5206, pass1_1018_526a, pass1_1018_57d2, pass1_1018_5e26, pass1_1018_6198, wsprintf_1018_34b6};
use crate::pass::pass8_funcs::{pass1_1008_e05e, pass1_1008_e10c, pass1_1008_e164, pass1_1010_01f8, pass1_1010_038e, pass1_1010_043a, pass1_1010_08e2, pass1_1010_091e, pass1_1010_0932, pass1_1010_0946, pass1_1010_1d80, pass1_1010_1dda, pass1_1010_1ea6, pass1_1010_1f62, pass1_1010_2024, pass1_1010_2ee2, pass1_1010_32c0, pass1_1010_375e, pass1_1010_3770, pass1_1010_3c52, pass1_1010_3c9e, pass1_1010_3cd0, pass1_1010_40cc, pass1_1010_41d6, pass1_1010_451a, pass1_1010_459e, pass1_1010_4f20, pass1_1010_5f1e, pass1_1010_5f4c, pass1_1010_5f7a, pass1_1010_5fd8, pass1_1010_6006, pass1_1010_60cc, pass1_1010_6566, pass1_1010_659a, pass1_1010_6604, pass1_1010_6ca2, pass1_1010_715c, pass1_1010_7818, pass1_1010_7d38, pass1_1010_7e40, pass1_1010_878c, process_struct_1010_20ba};
use crate::pass::pass9_funcs::{pass1_1008_e01c, pass1_1008_e038, pass1_1040_ac84};
use crate::pass::pass_funcs::{pass1_1000_4906, pass1_1008_392e, pass1_fn_1000_484c, pass1_fn_1000_5008};
use crate::sound_funcs::{mci_fn_1020_08b6, mci_send_cmd_1008_5c5c, mci_send_command_1008_5c7c, mci_send_command_1008_5c9e, mci_send_command_1008_5cb6, sound_fn_1008_53ae};
use crate::string_ops::{load, misc};
use crate::string_ops::load::{load_string_1008_b1f0, load_string_1010_84e0, load_str_1038_81be, load_str_1010_84ac};
use crate::string_ops::misc::{copy_string_1000_3d3e, fn_1008_6048, get_string_index_1000_3da4, load_string_1010_847e, load_string_1010_84e0, process_string_1000_3cea, process_string_1000_3dbe, process_string_1000_472c, process_string_1000_4d58, string_fn_1000_3f9c, string_fn_1008_5fd8, wsprintf_1018_35b0, string_fn_1018_3b9e};
use crate::struct_ops::struct_ops_2::{pass1_1038_7d10, pass1_1038_88f2, pass1_1038_8caa, pass1_1038_9f76, pass1_1038_ab82, pass1_1038_b772, pass1_1038_bddc, pass1_1038_c4a2, pass1_1038_c7b8, pass1_1038_cad8, pass1_1038_cd06, pass1_1038_d242, pass1_1038_d756, pass1_1038_e140, pass1_1038_e2d0, pass1_1038_e69a, pass1_1038_e99a, pass1_1038_eb9e, pass1_1038_eeda, pass1_1040_06e8, pass1_1040_0bfc, pass1_1040_0e1c, pass1_1040_123e, pass1_1040_181c, pass1_1040_1cb4, pass1_1040_1f5a, pass1_1040_2ea2, pass1_1040_34a2, process_struct_1000_179c, process_struct_1008_3ab8, process_struct_1008_4772, process_struct_1008_47cc, process_struct_1008_4834, process_struct_1008_48fe, process_struct_1008_4c58, process_struct_1008_574a, process_struct_1008_e3ec, process_struct_1008_e586, process_struct_1010_1d48, process_struct_1010_60fa, process_struct_1020_04f6, process_struct_1020_0baa, process_struct_1020_62e0, process_struct_1040_7728, process_struct_1040_9824, process_struct_1040_a598, process_struct_1040_bf3e, set_struct_1018_36e6, struct_fn_1000_160a};
use crate::structs::prog_structs_10::{Struct62, Struct73};
use crate::structs::prog_structs_12::{Struct431, Struct94};
use crate::structs::prog_structs_16::Struct588;
use crate::structs::prog_structs_17::Struct61;
use crate::structs::prog_structs_18::{Struct180, Struct566};
use crate::structs::prog_structs_1::Struct104;
use crate::structs::prog_structs_2::{Struct199, Struct318, Struct7};
use crate::structs::prog_structs_21::{Struct12, Struct25};
use crate::structs::prog_structs_23::{Struct356, Struct387, Struct43, WinStruct42};
use crate::structs::prog_structs_24::{Struct103, Struct2111, Struct6};
use crate::structs::prog_structs_25::{Struct219, Struct63, Struct65, Struct67, Struct71};
use crate::structs::prog_structs_26::{Struct22, Struct339, Struct340, Struct59};
use crate::structs::prog_structs_27::{pass1_struct_2, Struct298};
use crate::structs::prog_structs_28::{FileObject, Struct24, Struct300, Struct351, Struct40, Struct434, Struct447};
use crate::structs::prog_structs_29::{Struct123, Struct179, Struct23, Struct362, Struct41, Struct48};
use crate::structs::prog_structs_30::{Struct119, Struct124, Struct347};
use crate::structs::prog_structs_31::{Struct112, Struct16, Struct17, Struct20, Struct27, Struct32, Struct382, Struct4, Struct47};
use crate::structs::prog_structs_3::Struct661;
use crate::structs::prog_structs_4::Struct650;
use crate::structs::prog_structs_6::{Struct622, Struct675};
use crate::structs::prog_structs_7::{Struct376, Struct44, Struct628, Struct629};
use crate::structs::prog_structs_8::{Struct60, Struct68};
use crate::structs::prog_structs_9::{Struct594, Struct604, Struct636, Struct637};
use crate::sys_ops::{cleanup_fn_1020_28fc, get_sys_metrics_1020_7c1a, pass1_1038_8810, reg_class_1008_818c, reg_class_1008_96d2, reg_class_1040_98c0, win_msg};
use crate::sys_ops::dos_ops::{dos3_call_1000_4f2e, dos3_call_1000_51aa};
use crate::sys_ops::metrics::get_sys_metrics_1040_8c66;
use crate::sys_ops::proc::{make_proc_inst_1038_cf6c, make_proc_inst_1040_8fb8};
use crate::sys_ops::rsrc::load_rsrc_1010_4e9e;
use crate::sys_ops::win::{create_win_1008_9760, win_cleanup_func_1040_b0f8, win_fn_1038_c374, win_func_1018_6bb6};
use crate::sys_ops::win_msg::{post_win_msg_1040_7b3c, process_win_msg_1008_9510, send_msg_1010_7c42, send_msg_1040_1696, send_msg_1040_323c, send_win_msg_1008_84ba, send_win_msg_1020_097e, send_win_msg_1038_c228, win_fn_1040_3374};
use crate::sys_structs::{BITMAPINFO, PAINTSTRUCT16, RECT16};
use crate::typedefs::{ATOM, COLORREF, DLGPROC16, HANDLE16, HCURSOR16, HDC16, HFILE16, HGDIOBJ16, HINSTANCE16, HPALETTE16, HPEN16, HWND16, LPARAM, LRESULT, SEGPTR, WPARAM16};
use crate::ui_ops::{color, cursor, dialog, file_menu, msg_box, ui2, window};
use crate::ui_ops::cursor::{load_cursor_1020_7f7a, set_cursor_1038_bc30};
use crate::ui_ops::dialog::{send_dlg_item_msg_1038_8164, send_dlg_item_msg_1038_8618, send_dlg_item_msg_1038_87b2};
use crate::ui_ops::window::{gui_window_func_1020_536e, set_window_pos_1020_38aa};
use crate::util::{CARRY1, CONCAT11, CONCAT12, CONCAT13, CONCAT210, CONCAT212, CONCAT214, CONCAT22, CONCAT24, CONCAT26, CONCAT28, CONCAT31, CONCAT66, SBORROW1, SBORROW2, SCARRY1, SUB21, SUB41, SUB42, ZEXT24};
use crate::winapi::{BeginPaint16, BringWindowToTop16, ClientToScreen16, CreateDC16, CreatePen16, CreateSolidBrush16, CreateWindow16, DeleteDC16, DeleteObject16, DestroyIcon16, DestroyMenu16, DestroyWindow16, DrawIcon16, DrawText16, EnableMenuItem16, EnableWindow16, EnumChildWindows16, FillRect16, FreeProcInstance16, GetClassInfo16, GetClientRect16, GetDC16, GetDlgCtrlID16, GetDlgItem16, GetProp16, GetStockObject16, GetSubMenu16, GetSystemMetrics16, GetVersion16, GetWindowLong16, GetWindowRect16, GetWindowWord16, InvalidateRect16, IsIconic16, IsWindow16, LoadCursor16, LoadIcon16, LoadMenu16, LoadString16, MakeProcInstance16, MessageBeep16, MessageBox16, MoveWindow16, PostMessage16, PtInRect16, Rectangle16, RegisterClass16, ReleaseCapture16, ReleaseDC16, RemoveProp16, SelectObject16, SelectPalette16, SendDlgItemMessage16, SendMessage16, SetBkColor16, SetCapture16, SetCursor16, SetDlgItemText16, SetFocus16, SetTextColor16, SetWindowPos16, SetWindowText16, ShowWindow16, swi, TrackPopupMenu16, UpdateWindow16, WinHelp16, WritePrivateProfileString16, SetProp16};

// pub fn win_fn_1008_1414(param_1: u32,param_2: u32)

// {
//     let mut u_var1: u32;
//     let ppc_var2: fn();
//     let BVar3: bool;
//     let mut u_var4: u16;
//     let mut u_var5: u16;
//     let ppVar6: &mut  pass1_struct_2;
//     let mut i_var7: i32;
//
//
//     let mut u_var8: u16;
//
//     let mut unaff_DI: u16;
//     let mut unaff_ss: u16;
//     let pp_var9: &mut  Struct2551;
//     let mut u_var10: u16;
//     let u_var11: u8;
//     let u_var12: u8;
//     let mut local_50: u32;
//     let mut local_4c: u16;
//     let mut local_4a: u16;
//     let mut local_3c: u16;
//     let mut local_3a: u16;
//     let mut local_38: u16;
//     let mut local_36: u16;
//     let mut local_34: u16;
//     let mut local_32: u16;
//     let mut local_30: u16;
//     let mut local_2e: u16;
//     let mut local_2c: u16;
//     let mut local_2a: u32;
//     let mut uStack38: u16;
//     let mut local_24: u16;
//     let mut local_22: u16;
//     let mut local_20: u32;
//     let mut local_1c: u16;
//     let mut local_1a: u16;
//     let mut local_18: u32;
//     let mut local_14: u16;
//     let mut local_12: u16;
//     let mut local_10: u32;
//     let mut local_c: u16;
//     let mut local_a: u16;
//     let mut local_8: [u8;6];

//     pass1_1008_6d8a(CONCAT22(unaff_ss, local_8), param_2);
//     BVar3 = close_file_1008_6e78(CONCAT22(unaff_ss, local_8));
//     i_var7 = param_1;
//   // u_var10 = (param_1  >> 0x10);
//     if (BVar3 == 0)
//     {
//         if (ctx.g_u16_1050_0310 == 0)
//         {
//             ctx.g_u16_1050_0310 = 0x6d4;
//         }
//         u_var4 = ctx.g_u16_1050_0310;
//         load_string_1010_847e(ctx.g_struct_73_1050_14cc, (ctx.g_struct_73_1050_14cc >> 0x10),
//                               ctx.g_u16_1050_0310);
//         u_var8 = ctx.dx_reg;
//         pass1_fn_1008_60e8(u_var4, ctx.dx_reg);
//         u_var5 = u_var4;
//         load_string_1010_847e(ctx.g_struct_73_1050_14cc, (ctx.g_struct_73_1050_14cc >> 0x10), 0x57b);
//         MessageBeep16(0x10);
//         MessageBox16(0x10, CONCAT22(ctx.dx_reg, u_var5), CONCAT22(u_var8, u_var4),
//                      (i_var7 + 8));
//         error_check_1000_17ce(CONCAT22(u_var8, u_var4));
//         call_fn_ptr_1000_24cd(1);
//     }
//     set_cursor_1008_2dcc(i_var7, u_var10, 8);
//     _local_c = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_DI, 0x2f));
//   // u_var8 = (_local_c  >> 0x10);
//     local_10 = (_local_c + 0x20);
//   // ppVar6 = pass1_1030_8344(ctx._g_bool_1050_5748, (ctx._g_bool_1050_5748  >> 0x10),
//                              local_10);
//     _local_14 = CONCAT22(u_var8, ppVar6);
//     local_18 = &ppVar6.field_0x10;
//     u_var1 = (i_var7 + 0xe8);
//     ppc_var2 = ((i_var7 + 0xe8) + 4);
//     ppc_var2(0x1030, u_var1, (u_var1 >> 0x10), local_10, (local_10 >> 0x10),
//                 (local_18 + 2) + -1, 2);
//     local_22 = ctx.dx_reg;
//   // ppVar6 = pass1_1030_8344(ctx._g_bool_1050_5748, (ctx._g_bool_1050_5748  >> 0x10),
//                              0x4000001);
//     _local_1c = CONCAT22(local_22, ppVar6);
//     local_20 = &ppVar6.field_0x10;
//     local_24 = pass1_1030_8344(ctx._g_bool_1050_5748,
//                                     (ctx._g_bool_1050_5748 >> 0x10), local_20);
//     local_2a = (local_24 + 0xc);
//     uStack38 = (local_24 + 0x10);
//     i_var7 = pass1_1030_5b00(_local_14);
//     u_var11 = SUB21(&local_2a, 0);
//     u_var12 = (&local_2a >> 8);
//     u_var10 = unaff_ss;
//     pp_var9 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT13(u_var12, CONCAT12(u_var11, i_var7)));
//     pass1_1018_179e(pp_var9, CONCAT22(u_var10, CONCAT11(u_var12, u_var11)));
//     u_var11 = 0;
//     u_var12 = 4;
//     u_var5 = 0x1b;
//     u_var10 = 1;
//     pp_var9 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x1002b);
//     pass1_1010_043a(pp_var9, CONCAT13(u_var12, CONCAT12(u_var11, u_var10)), u_var5);
//     close_file_1008_6dd0(CONCAT22(unaff_ss, local_8));
//     return;
// }

// WARNING: Variable defined which should be unmapped: local_8

// pub fn set_focus_1038_c558(param_1: &mut  Struct20)

// {
//     let mut u_var1: u16;

//     win_gui_func_1040_78e2(param_1);
//     move_window_1040_826c(param_1, 0xffffffff);
//   // u_var1 = (param_1  >> 0x10);
//     ShowWindow16(5, (param_1 + 6));
//     SetFocus16((param_1 + 6));
//     return;
// }

pub unsafe fn gui_fn_1008_2c4e(param_1: &mut Struct16, param_2: u16, param_3: u16) {
    // let pi_var1: i32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    // let mut h_cursor_1: u16;
    // let h_cursor_2: HCURSOR16;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_6: u16;

    let h_cursor_1 = LoadCursor16(0, 0x7f02);
    let h_cursor_2 = SetCursor16(h_cursor_1);
    let mut pi_var1 = &mut param_1.field_0xf2;
    pi_var1 = pi_var1 + 1;
    if param_1.field_0xee != 0 {
        u_var5 = param_1.field_0xee;
        ppc_var3 = (param_1.field_0xee + 0x90);
        (**ppc_var3)(offset, u_var5, (u_var5 >> 0x10));
    }
    u_var5 = big_fn_1008_15d4(CONCAT22(param_2, param_1), param_3);
    //// _var4 = (u_var5  >> 0x10);
    param_1.field_0xee = u_var5;
    param_1.field_0xf0 = u_var4;
    ppc_var3 = (param_1.field_0xee + 8);
    (**ppc_var3)(offset, &param_1.field_0xee, u_var4);
    if (param_1.field_0xe8 != 0) {
        u_var2 = param_1.field_0xe8;
        ppc_var3 = (param_1.field_0xe8 + 0xc);
        (**ppc_var3)(offset, u_var2, (u_var2 >> 0x10), 0);
    }
    window::show_window_1038_b634(ctx.g_struct_112_001, (ctx.g_struct_112_001 >> 0x10));
    u_var2 = param_1.field_0xf4;
    window::show_window_1010_7a76(u_var2, (u_var2 >> 0x10));
    u_var5 = &param_1.field_0xee;
    ppc_var3 = (&param_1.field_0xee + 0xc);
    (**ppc_var3)(0x1010, u_var5, (u_var5 >> 0x10), 5);
    u_var5 = &param_1.field_0xee;
    BringWindowToTop16((u_var5 + 8));
    SetCursor16(h_cursor_2);
    return;
}

pub unsafe fn gui_get_info_func_1008_da12(param_1: &mut  Struct61, param_2: u32) {
    let pu_var1: &mut  u16;
    let mut b_var2: u8;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let hdc: HDC16;
    let i_var5: u16;
    let mut u_var6: i32;
    let local_AX_193: &mut  Struct62;
    let pa_var7: &mut  Struct94;
    let pa_var8: &mut  Struct94;
    let mut count: u16;
    //
    let pu_var9: &mut  u16;
    let mut i_var10: i32;
    let mut u_var11: u16;
    let mut local_20: u32;
    let mut local_18: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var11 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var11, param_1), (param_2 >> 0x10));
    param_1.field_0xa = 0;
    param_1.field_0xc = 0;
    zero_list_1008_3e38(CONCAT13(
        (param_2 >> 8),
        CONCAT12(param_2, &param_1.field_0xe),
    ));
    param_1.field_0x14 = 0;
    param_1.field_0x16 = 0;
    &param_1.field_0x18 = 0;
    CONCAT22(u_var11, param_1) = 0xdc80;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    hdc = GetDC16(0);
    i_var5 = GetDeviceCaps16(8, hdc);
    param_1.field_0xa = i_var5;
    i_var5 = GetDeviceCaps16(10, hdc);
    param_1.field_0xc = i_var5;
    pass1_1008_3e76(
        CONCAT22(u_var11, &param_1.field_0xe),
        0,
        (param_1.field_0xc + -0x1e0) / 2,
        (param_1.field_0xa + -0x280) / 2,
    );
    pu_var9 = ctx.dx_reg;
    u_var6 = GetDeviceCaps16(0x26, hdc);
    if (u_var6 & 0x100) != 0 {
        i_var5 = GetDeviceCaps16(0x68, hdc);
        param_1.field_0x14 = i_var5;
        local_AX_193 = GetDeviceCaps16(0x6a, hdc);
        param_1.field_0x16 = local_AX_193;
        if ctx.g_struct_ptr_1 == 0 {
            pa_var7 = ((local_AX_193 + 1) * 4);
            struct_fn_1000_160a();
        } else {
            pu_var9 = ctx.g_u16_ptr_1050_5f2e;
            pa_var7 = g_struct_ptr_1;
        }
        alloc_mem_1000_1708(((local_AX_193 + 1) * 4), 0, 1, pa_var7, pu_var9);
        _local_8 = CONCAT22(pu_var9, pa_var7);
        pa_var8 = ((param_1.field_0x16 + 1) * 4);
        if (ctx.g_struct_ptr_1 == 0) {
            ctx.g_u16_ptr_1050_5f2e = pu_var9;
            g_struct_ptr_1 = pa_var8;
            struct_fn_1000_160a();
        } else {
        }
        alloc_mem_1000_1708(pa_var8, 0, 1, g_struct_ptr_1, ctx.g_u16_ptr_1050_5f2e);
        param_1.field_0x18 = g_struct_ptr_1;
        param_1.field_0x1a = ctx.g_u16_ptr_1050_5f2e;
        if (_local_8 != 0x0) {
            if (&param_1.field_0x18 != 0) {
                count = param_1.field_0x16 / 2;
                GetSystemPalette16(CONCAT22(pu_var9, pa_var7), count, 0, hdc);
                GetSystemPalette16(
                    CONCAT22(pu_var9, &pa_var7.field_0x0 + count * 2),
                    count,
                    param_1.field_0x14 - count,
                    hdc,
                );
                local_20 = &param_1.field_0x18;
                local_10 = 0;
                while (
                    u_var4 = local_20,
                    pu_var1 = &param_1.field_0x16,
                    unsafe { *pu_var1 != local_10 } && unsafe { local_10 <= *pu_var1 },
                ) {
                    b_var2 = *(&pa_var7.field_0x0 + local_10 * 2);
                  // u_var3 = local_20  >> 0x10;
                    i_var10 = local_20;
                    local_20 = local_20 & 0xffff0000 | (i_var10 + 4);
                    u_var4 = CONCAT11(
                        *((&pa_var7.field_0x0 + local_10 * 2) + 1),
                        *(&pa_var7.field_0x0 + local_10 * 2 + 1),
                    );
                    (i_var10 + 2) = b_var2;
                    local_10 = local_10 + 1;
                }
            }
        }
        error_check_1000_17ce(_local_8);
    }
    ReleaseDC16(hdc, 0);
    return;
}

pub fn win_gui_fn_1040_ad24(param_1: &mut  Struct124, param_2: u16, param_3: u16, param_4: u32) {
    if param_3 == 0xeb {
        dialog::set_dialog_item_text_1040_ae04(CONCAT22(param_2, param_1));
    } else {
        if (param_3 != 0x1f0) {
            window::win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        msg_box::msg_box_1040_ad66(CONCAT22(param_2, param_1));
    }
    return;
}

pub unsafe fn win_gui_fn_1010_2a32(param_1: i32, uparam_2_00: i32, param_2: &HFILE16) {
    let pi_var1: &mut  i32;
    let pc_var2: String;
    let pbVar3: Vec<u8>;
    let left: u16;
    let top: u16;
    let right: u16;
    let bottom: u16;
    let in_struct_104_ptr: &mut  Struct104;
    let mut u_var4: u32;
    let mut entry: string;
    let mut string: string;
    let mut filename: string;
    let mut u_var5: u32;
    let mut bVar6: u8;
    let pu_var7: &mut  u32;
    let pu_var8: &mut  u32;
    let ppc_var9: fn();
    let pc_var10: &mut  code;
    let h_gdi_obj: HGDIOBJ16;
    let pu_var11: &mut  u16;
    let u_var12: u8;
    let u_var13: u8;
    let u_var14: u8;
    let u_var15: u8;
    let u_var16: u8;
    let u_var17: u8;
    let u_var18: u8;
    let u_var19: u8;
    let u_var20: u8;
    let u_var21: u8;
    let u_var22: u8;
    let u_var23: u8;
    let u_var24: u8;
    let u_var25: u8;
    let local_AX_123: &mut  Struct382;
    let mut u_var26: i32;
    let HVar27: HDC16;
    let pu_var28: Vec<u8>;
    let h_gdi_obj_00: HPALETTE16;
    let pu_var29: &mut  u32;
    let mut u_var30: u16;
    let BVar31: bool;
    let mut bVar32: u8;
    let mut in_dx: i32;


    let struct_a: &mut  Struct199;
    //
    let in_bx: &mut  u32;
    let mut i_var33: i32;
    let mut unaff_bp: u16;
    let mut unaff_si: i32;
    let mut i_var34: i32;
    let mut unaff_DI: u16;
    let mut unaff_es: u16;
    let mut unaff_cs: u16;
    let mut u_var35: u16;
    let mut unaff_ss: u16;
    let mut bVar36: bool;
    let mut bVar37: u8;
    let mut u_var38: u32;
    let mut in_stack_00000000: i32;
    let mut in_stack_00000002: u16;
    let mut local_36: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_26: u32;
    let mut local_22: u16;
    let mut iStack32: i32;
    let HStack30: HGDIOBJ16;
    let HStack28: HGDIOBJ16;
    let pcStack26: String;
    let pcStack24: String;
    let mut local_16: u32;
    let uStack18: u8;
    let uStack17: u8;
    let uStack16: u8;
    let uStack15: u8;
    let uStack14: u8;
    let uStack13: u8;
    let u_stack12: u8;
    let uStack11: u8;
    let uStack10: u8;
    let uStack9: u8;
    let mut bStack8: u8;
    let uStack7: u8;
    let uStack6: u8;
    let uStack5: u8;
    let mut local_4: u16;
    let uStack2: u8;
    let uStack1: u8;

    i_var33 = param_1;
    uStack9 = 0xfe;
    u_var13 = uStack9;
    uStack6 = 0xfe;
    u_var21 = uStack6;
    uStack2 = unaff_bp;
    uStack1 = (unaff_bp >> 8);
    local_4._0_1_ = 0;
    local_4 = 0;
    uStack10 = param_1;
    u_var12 = uStack10;
    uStack9 = (param_1 >> 8);
    u_var15 = uStack9;
    bStack8 = param_2_00;
    u_var18 = bStack8;
    uStack7 = (param_2_00 >> 8);
    u_var17 = uStack7;
    if ((param_2 + 0xec76 & 3) != 0) {}
    // goto LAB_1010_2ad8;
    local_AX_123 = (param_2 + 0xec76 >> 1);
    bVar36 = local_AX_123 < (s_version__d__d_1050_0012 + 10);
    if ((s_version__d__d_1050_0012 + 10) < local_AX_123) {}
    // goto LAB_1010_2ad8;
    unaff_cs = 0x1010;
    uStack6 = SUB21(in_bx, 0);
    u_var22 = uStack6;
    uStack5 = (in_bx >> 8);
    u_var24 = uStack5;
    uStack6 = unaff_ss;
    u_var23 = uStack6;
    uStack5 = (unaff_ss >> 8);
    u_var25 = uStack5;
    uStack10 = SUB41(param_2, 0);
    u_var14 = uStack10;
    uStack9 = (param_2 >> 8);
    u_var16 = uStack9;
  // bStack8 = (param_2  >> 0x10);
    u_var19 = bStack8;
    uStack7 = (param_2 >> 0x18);
    u_var20 = uStack7;
    uStack6 = u_var22;
    uStack5 = u_var24;
    match (local_AX_123) {
        // default:
        // goto switchD_1010_2ab5_caseD_0;
        0x1 | 0x3 | 0xb => {
            local_AX_123.field_0xa = in_bx;
        }
        0x9 | 0xf | 0x15 | 0x1b => {
            local_AX_123.field_0xa = in_bx;
            local_AX_123.field_0x10 = in_bx;
            local_AX_123.field_0xc = in_bx;
            return;
        }
        0x5 => {
            u_stack12 = 0x10;
            uStack11 = 0x10;
            uStack14 = 0x35;
            uStack13 = 0x40;
            BVar31 = write_to_file_1008_7e1c(
                param_2,
                ZEXT24(in_bx),
                CONCAT13(
                    (in_stack_00000000 >> 8),
                    CONCAT12(in_stack_00000000, unaff_bp),
                ),
            );
            if (BVar31 != 0) {
                return;
            }
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
        0x6 => {
            local_4._0_1_ = 0;
            // goto LAB_1010_2ad8;
        }
        0x7 => {
            uStack6 = 0x10;
            uStack5 = 0x10;
            bStack8 = 0x34;
            uStack7 = 0x48;
            unsafe {
                ppc_var9 = *in_bx;
            }
            (**ppc_var9)();
            i_var33 = param_2 + 0x105;
            uStack6 = i_var33;
            uStack5 = (i_var33 >> 8);
            uStack10 = ctx.g_struct_73_1050_14cc;
            uStack9 = (ctx.g_struct_73_1050_14cc >> 8);
          // bStack8 = (ctx.g_struct_73_1050_14cc  >> 0x10);
            uStack7 = (ctx.g_struct_73_1050_14cc >> 0x18);
            u_stack12 = 0x10;
            uStack11 = 0x10;
            uStack14 = 0x45;
            uStack13 = 0x48;
            win_gui_fn_1010_8170();
            i_var34 = param_2 * 4;
            (param_1 + i_var34 + 0x26) = i_var33;
            (param_1 + i_var34 + 0x28) = ctx.dx_reg;
            uStack6 = 0x50;
            uStack5 = 0x10;
            bStack8 = 0x80;
            uStack7 = 0x13;
            u_stack12 = 0;
            uStack11 = 0;
            uStack10 = 0;
            uStack9 = 0;
            uStack16 = 0;
            uStack15 = 0;
            uStack14 = 0;
            uStack13 = 0;
            in_struct_104_ptr = (param_1 + 0x26 + i_var34);
            local_16._2_1_ = SUB41(in_struct_104_ptr, 0);
            local_16._3_1_ = (in_struct_104_ptr >> 8);
          // uStack18 = (in_struct_104_ptr  >> 0x10);
            uStack17 = (in_struct_104_ptr >> 0x18);
            local_16._0_2_ = 0x1010;
            pcStack24 = &PTR_LOOP_1050_486c;
            u_var38 = process_struct_1008_4772(in_struct_104_ptr);
            uStack18 = u_var38;
            uStack17 = (u_var38 >> 8);
          // uStack16 = (u_var38  >> 0x10);
            uStack15 = (u_var38 >> 0x18);
            local_16._0_2_ = &ctx.PTR_LOOP_1050_1008;
            pcStack24 = 0x4879;
            local_16._2_1_ = uStack18;
            local_16._3_1_ = uStack17;
            uStack18 = uStack16;
            uStack17 = uStack15;
            HVar27 = CreateDC16(
                u_var38,
                (u_var38 & 0xff000000 | CONCAT12(uStack16, (u_var38 >> 0x10))),
                CONCAT13(uStack11, CONCAT12(u_stack12, CONCAT11(uStack13, uStack14))),
                CONCAT13(uStack7, CONCAT12(bStack8, CONCAT11(uStack9, uStack10))),
            );
            local_16._2_1_ = HVar27;
            local_16._3_1_ = (HVar27 >> 8);
            u_var5 = (_PTR_LOOP_1050_4230 + 0xe);
            pcStack24 = u_var5;
            local_16._0_2_ = (u_var5 >> 0x10);
            pcStack26 = (&local_16 + 2);
            bStack8 = pcStack26;
            uStack7 = (pcStack26 >> 8);
            u_stack12 = u_var5;
            uStack11 = (u_var5 >> 8);
          // uStack10 = (u_var5  >> 0x10);
            uStack9 = (u_var5 >> 0x18);
            uStack14 = 0x38;
            uStack13 = 0x15;
            uStack16 = 0x97;
            uStack15 = 0x48;
            uStack6 = u_var23;
            uStack5 = u_var25;
            realize_palette_1008_4e08();
            uStack6 = local_16._2_1_;
            uStack5 = local_16._3_1_;
            bStack8 = local_4;
            uStack7 = local_4;
            uStack10 = 8;
            uStack9 = 0x10;
            u_stack12 = 0xa5;
            uStack11 = 0x48;
            HStack28 = SelectObject16(
                CONCAT11(local_4, local_4),
                CONCAT11(local_16._3_1_, local_16._2_1_),
            );
            uStack6 = local_16._2_1_;
            uStack5 = local_16._3_1_;
            bStack8 = local_16._2_1_;
            uStack7 = local_16._3_1_;
            uStack10 = 0x38;
            uStack9 = 0x15;
            u_var35 = SUB42(offset, 0);
            u_stack12 = 0xb3;
            uStack11 = 0x48;
            HStack30 = SelectObject16(
                CONCAT11(local_16._3_1_, local_16._2_1_),
                CONCAT11(local_16._3_1_, local_16._2_1_),
            );
            iStack32 = 0;
            loop {
                u_var13 = uStack15;
                u_var12 = uStack16;
                pi_var1 = (param_1 + 0x74);
                uStack16 = u_var35;
                uStack15 = (u_var35 >> 8);
                if (unsafe { *pi_var1 == iStack32 } || unsafe { *pi_var1 < iStack32 }) {
                    break;
                }
                uStack6 = 8;
                uStack5 = 0;
                i_var33 = (iStack32 * 0x10 + param_2) * 8;
                i_var34 = i_var33 + (param_1 + 0x70);
                u_var35 = (param_1 + 0x72);
                bStack8 = u_var35;
                uStack7 = (u_var35 >> 8);
                uStack10 = i_var34;
                uStack9 = (i_var34 >> 8);
                pu_var28 = &uStack14;
                uStack14 = SUB21(pu_var28, 0);
                uStack13 = (pu_var28 >> 8);
                u_var35 = 0x1000;
                uStack18 = 0xe1;
                uStack17 = 0x48;
                u_stack12 = u_var23;
                uStack11 = u_var25;
                pass1_fn_1000_484c(
                    CONCAT13(u_var25, CONCAT12(u_var23, pu_var28)),
                    CONCAT13(uStack7, CONCAT12(bStack8, i_var34)),
                    8,
                );
                if (pu_var28 != 0x0) {
                    uStack6 = local_16._2_1_;
                    uStack5 = local_16._3_1_;
                    u_var5 = (param_1 + 0x70);
                    // u_var35 = (u_var5  >> 0x10);
                    i_var34 = u_var5;
                    left = (i_var34 + i_var33);
                    bStack8 = left;
                    uStack7 = (left >> 8);
                    i_var33 = i_var33 + i_var34;
                    top = (i_var33 + 2);
                    uStack10 = top;
                    uStack9 = (top >> 8);
                    right = (i_var33 + 4);
                    u_stack12 = right;
                    uStack11 = (right >> 8);
                    bottom = (i_var33 + 6);
                    uStack14 = bottom;
                    uStack13 = (bottom >> 8);
                    uStack16 = 0;
                    uStack15 = 0x10;
                    u_var35 = SUB42(offset, 0);
                    uStack18 = 8;
                    uStack17 = 0x49;
                    Rectangle16(
                        bottom,
                        right,
                        top,
                        left,
                        CONCAT11(local_16._3_1_, local_16._2_1_),
                    );
                }
                iStack32 = iStack32 + 1;
            }
            uStack6 = local_16._2_1_;
            uStack5 = local_16._3_1_;
            bStack8 = pcStack26;
            uStack7 = (pcStack26 >> 8);
            uStack10 = 0;
            uStack9 = 0;
            uStack14 = 0x24;
            uStack13 = 0x49;
            u_stack12 = uStack16;
            uStack16 = u_var12;
            uStack11 = uStack15;
            uStack15 = u_var13;
            h_gdi_obj_00 = SelectPalette16(0, pcStack26, CONCAT11(local_16._3_1_, local_16._2_1_));
            local_4._0_1_ = h_gdi_obj_00;
            local_4 = (h_gdi_obj_00 >> 8);
            uStack6 = 0x38;
            uStack5 = 0x15;
            bStack8 = 0x2a;
            uStack7 = 0x49;
            DeleteObject16(h_gdi_obj_00);
            uStack2 = local_16._2_1_;
            uStack1 = local_16._3_1_;
            local_4._0_1_ = HStack28;
            local_4 = (HStack28 >> 8);
            uStack6 = 0x38;
            uStack5 = 0x15;
            bStack8 = 0x35;
            uStack7 = 0x49;
            SelectObject16(HStack28, CONCAT11(local_16._3_1_, local_16._2_1_));
            uStack2 = local_16._2_1_;
            uStack1 = local_16._3_1_;
            local_4._0_1_ = HStack30;
            local_4 = (HStack30 >> 8);
            uStack6 = 0x38;
            uStack5 = 0x15;
            bStack8 = 0x40;
            uStack7 = 0x49;
            SelectObject16(HStack30, CONCAT11(local_16._3_1_, local_16._2_1_));
            uStack2 = local_16._2_1_;
            uStack1 = local_16._3_1_;
            local_4._0_1_ = 0x38;
            local_4 = 0x15;
            uStack6 = 0x48;
            uStack5 = 0x49;
            DeleteDC16(CONCAT11(local_16._3_1_, local_16._2_1_));
            h_gdi_obj = CONCAT11(local_4, local_4);
            uStack2 = 0x38;
            uStack1 = 0x15;
            local_4._0_1_ = 0x50;
            local_4 = 0x49;
            DeleteObject16(h_gdi_obj);
            return;
        }
        0x8 => {
            local_4._0_1_ = 3;
            // goto LAB_1010_2ad8;
        }
        0xd => {
            pbVar3 = (&local_AX_123.field_0x0 + unaff_si);
            unsafe {
                bVar37 = *pbVar3;
                bVar6 = *pbVar3 + in_dx;
                *pbVar3 = bVar6 + bVar36;
            }
            pu_var7 = (CARRY1(bVar37, in_dx) || CARRY1(bVar6, bVar36));
            pu_var8 = in_bx + -0x404;
            bVar37 = in_bx < 0x1010 || pu_var8 < pu_var7;
            pu_var29 = (pu_var8 - pu_var7);
            pc_var10 = swi(4);
            if (SBORROW2(in_bx, 0x1010) != SBORROW2(pu_var8, pu_var7)) {
                unsafe {
                    (*pc_var10)();
                }
                in_dx = ctx.dx_reg;
            }
            bVar36 = pu_var29 < 0x1010 || pu_var29 + -0x404 < bVar37;
            pbVar3 = (&local_AX_123.field_0x0 + unaff_si);
            unsafe {
                bVar37 = *pbVar3;
                bVar32 = in_dx;
                bVar6 = *pbVar3;
                *pbVar3 = bVar6 + bVar32 + bVar36;
                pc_var2 = (&local_AX_123.field_0x0 + unaff_si);
                *pc_var2 =
                    *pc_var2 + bVar32 + (CARRY1(bVar37, bVar32) || CARRY1(bVar6 + bVar32, bVar36));
            }
            bStack8 = (&uStack2 >> 8);
            u_stack12 = in_stack_00000000;
            uStack11 = (in_stack_00000000 >> 8);
            uStack10 = in_stack_00000002;
            uStack13 = uStack1;
            uStack15 = local_4;
            uStack14 = uStack2;
            uStack17 = 0x10;
            uStack16 = 0x10;
            local_16._3_1_ = 0x4d;
            uStack18 = 0x50;
            uStack9 = u_var13;
            pass1_1018_4cda(
                CONCAT11(uStack2, local_4),
                CONCAT13(uStack10, CONCAT12(uStack11, CONCAT11(u_stack12, uStack1))),
            );
            i_var33 = CONCAT11(uStack2, local_4);
            u_stack12 = in_stack_00000000;
            pu_var11 = CONCAT13(u_stack12, CONCAT12(uStack1, i_var33));
            unsafe {
                *pu_var11 = (s_SinternalPutBldg2_site_0x_08lx__1050_5099 + 1);
            }
            (i_var33 + 2) = 0x1010;
            uStack11 = 0xb3;
            uStack10 = 1;
            uStack13 = uStack1;
            uStack15 = local_4;
            uStack14 = uStack2;
            uStack17 = 0x18;
            uStack16 = 0x10;
            local_16._3_1_ = 0x65;
            uStack18 = 0x50;
            pass1_1018_4dce(CONCAT13(u_stack12, CONCAT12(uStack1, i_var33)), 0x1b3);
            _PTR_LOOP_1050_4230 = CONCAT13(
                u_stack12,
                CONCAT12(uStack1, CONCAT11(uStack2, local_4)),
            );
            return;
        }
        0xe => (param_1 + 0x20) = param_2,
        0x11 => {
            loop {
                // WARNING: Do nothing block with infinite loop
            }
        }
        0x12 => {
            PTR_LOOP_1050_10c6 = (0 < param_2);
            PTR_LOOP_1050_1142 = (2 < param_2)
        }
        0x13 => {
            uStack5 = (&uStack2 >> 8);
            i_var33 = param_1 * 8 + in_stack_00000000;
            if (((((i_var33 + 0x22) != 0) || ((i_var33 + 0x24) != 0)) || ((i_var33 + 0x26) != 0))
                || ((i_var33 + 0x28) != 0))
            {
                i_var33 = param_1 * 8 + in_stack_00000000;
                u_var5 = (i_var33 + 0x22);
                u_var4 = (i_var33 + 0x26);
                uStack14 = u_var4;
                uStack13 = (u_var4 >> 8);
              // u_stack12 = (u_var4  >> 0x10);
                uStack11 = (u_var4 >> 0x18);
                uStack18 = u_var5;
                uStack17 = (u_var5 >> 8);
              // uStack16 = (u_var5  >> 0x10);
                uStack15 = (u_var5 >> 0x18);
                local_16._2_1_ = 0x50;
                local_16._3_1_ = 0x10;
                local_16._0_2_ = s__d__d__d__d_1050_14ae;
                u_var4 = (in_stack_00000000 + 0xe);
                pcStack26 = u_var4;
              // pcStack24 = (u_var4  >> 0x10);
                HStack28 = 0x1010;
                HStack30 = 0x627c;
                uStack6 = u_var21;
                string_fn_1000_3f9c(
                    pcStack26,
                    pcStack24,
                    s__d__d__d__d_1050_14ae,
                    &ctx.g_alloc_addr_1050_1050,
                    u_var5,
                );
                u_stack12 = 0x50;
                uStack11 = 0x10;
                uStack14 = 0xb8;
                uStack13 = 0x13;
                entry = (param_1 * 4 + 0x1446);
                uStack18 = SUB41(entry, 0);
                uStack17 = (entry >> 8);
              // uStack16 = (entry  >> 0x10);
                uStack15 = (entry >> 0x18);
                string = (in_stack_00000000 + 0xe);
                local_16._0_2_ = string;
                local_16._2_1_ = (string >> 0x10);
                local_16._3_1_ = (string >> 0x18);
                filename = (in_stack_00000000 + 10);
                pcStack26 = filename;
              // pcStack24 = (filename  >> 0x10);
                HStack28 = 0x1000;
                HStack30 = 0x62a0;
                WritePrivateProfileString16(filename, string, entry, s_windows_1050_13b8);
            }
            return;
        }
        0x14 => (param_1 + 0x24) = param_2,
        0x17 => {
            struct_a = (in_dx + -1);
            uStack6 = unaff_DI;
            uStack5 = (unaff_DI >> 8);
            pbVar3 = (&local_AX_123.field_0x0 + unaff_si);
            unsafe {
                *pbVar3 = *pbVar3 | struct_a;
            }
            (param_1 + 0x12) = in_bx;
            (param_1 + 0x14) = struct_a;
            local_2a = 0;
            loop {
                uStack10 = unaff_cs;
                u_var12 = uStack10;
                uStack9 = (unaff_cs >> 8);
                u_var13 = uStack9;
                if (local_36 <= local_2a) {
                    uStack10 = 2;
                    uStack9 = 0;
                    bStack8 = 0;
                    uStack7 = 0;
                    u_var26 = param_1 + 0x1a;
                    uStack14 = u_var26;
                    uStack13 = (u_var26 >> 8);
                    uStack11 = ((param_2_00 << 0x10) >> 0x18);
                    local_16._0_2_ = 0x9f2f;
                    local_16._2_1_ = u_var12;
                    local_16._3_1_ = u_var13;
                    uStack18 = u_var14;
                    uStack17 = u_var16;
                    uStack16 = u_var19;
                    uStack15 = u_var20;
                    u_stack12 = u_var18;
                    BVar31 = read_file_1008_7dee(
                        param_2,
                        ((param_2_00 & 0xff00) << 0x10 | CONCAT12(u_var18, u_var26)),
                        2,
                    );
                    if (BVar31 != 0) {
                        uStack10 = 2;
                        uStack9 = 0;
                        bStack8 = 0;
                        uStack7 = 0;
                        u_var26 = param_1 + 0x1c;
                        uStack14 = u_var26;
                        uStack13 = (u_var26 >> 8);
                        uStack11 = ((param_2_00 << 0x10) >> 0x18);
                        local_16._2_1_ = 8;
                        local_16._3_1_ = 0x10;
                        local_16._0_2_ = 0x9f4a;
                        uStack18 = u_var14;
                        uStack17 = u_var16;
                        uStack16 = u_var19;
                        uStack15 = u_var20;
                        u_stack12 = u_var18;
                        BVar31 = read_file_1008_7dee(
                            param_2,
                            ((param_2_00 & 0xff00) << 0x10 | CONCAT12(u_var18, u_var26)),
                            2,
                        );
                        if (BVar31 != 0) {
                            uStack10 = 2;
                            uStack9 = 0;
                            bStack8 = 0;
                            uStack7 = 0;
                            u_var26 = param_1 + 0x1e;
                            uStack14 = u_var26;
                            uStack13 = (u_var26 >> 8);
                            uStack11 = ((param_2_00 << 0x10) >> 0x18);
                            local_16._2_1_ = 8;
                            local_16._3_1_ = 0x10;
                            local_16._0_2_ = 0x9f65;
                            uStack18 = u_var14;
                            uStack17 = u_var16;
                            uStack16 = u_var19;
                            uStack15 = u_var20;
                            u_stack12 = u_var18;
                            BVar31 = read_file_1008_7dee(
                                param_2,
                                ((param_2_00 & 0xff00) << 0x10 | CONCAT12(u_var18, u_var26)),
                                2,
                            );
                            if (BVar31 != 0) {
                                return;
                            }
                        }
                    }
                    ctx.g_u16_1050_0310 = 0x6d2;
                    return;
                }
                bStack8 = 8;
                uStack7 = 0;
                u_stack12 = 0xe4;
                uStack11 = 0x9e;
                u_var30 = local_36;
                process_struct_1000_179c(8, struct_a);
                local_2e = CONCAT22(struct_a, u_var30);
                if ((struct_a | u_var30) == 0) {
                    local_26 = 0;
                } else {
                    local_2e = ctx.s_1_1050_389a;
                    (u_var30 + 2) = &ctx.PTR_LOOP_1050_1008;
                    local_2e = 0xa1c4;
                    (u_var30 + 2) = 0x1010;
                    local_26 = local_2e;
                }
                uStack10 = 2;
                uStack9 = 0;
                bStack8 = 0;
                uStack7 = 0;
                uStack14 = SUB21(&local_22, 0);
                uStack13 = (&local_22 >> 8);
                local_16._2_1_ = 0;
                local_16._3_1_ = 0x10;
                local_16._0_2_ = 0x9e69;
                uStack18 = u_var14;
                uStack17 = u_var16;
                uStack16 = u_var19;
                uStack15 = u_var20;
                u_stack12 = u_var23;
                uStack11 = u_var25;
                BVar31 = read_file_1008_7dee(
                    param_2,
                    CONCAT13(u_var25, CONCAT12(u_var23, &local_22)),
                    2,
                );
                i_var33 = local_26;
              // uStack10 = (local_26  >> 0x10);
                u_var15 = uStack10;
                uStack9 = (local_26 >> 0x18);
                u_var17 = uStack9;
                u_stack12 = local_26;
                u_var12 = u_stack12;
                uStack11 = (local_26 >> 8);
                u_var13 = uStack11;
                if (BVar31 == 0) {
                    break;
                }
                uStack10 = 2;
                uStack9 = 0;
                bStack8 = 0;
                uStack7 = 0;
                u_var26 = i_var33 + 6;
                uStack14 = u_var26;
                uStack13 = (u_var26 >> 8);
              // u_stack12 = ((local_26 & 0xffff0000)  >> 0x10);
                uStack11 = ((local_26 & 0xffff0000) >> 0x18);
                local_16._2_1_ = 8;
                local_16._3_1_ = 0x10;
                local_16._0_2_ = 0x9e82;
                uStack18 = u_var14;
                uStack17 = u_var16;
                uStack16 = u_var19;
                uStack15 = u_var20;
                BVar31 = read_file_1008_7dee(
                    param_2,
                    (local_26 & 0xff000000 | CONCAT12(u_stack12, u_var26)),
                    2,
                );
                if (BVar31 == 0) {
                    break;
                }
                bStack8 = local_22;
                uStack7 = (local_22 >> 8);
                uStack14 = 8;
                uStack13 = 0x10;
                unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                uStack16 = 0xb6;
                uStack15 = 0x9e;
                u_stack12 = u_var14;
                uStack11 = u_var16;
                uStack10 = u_var19;
                uStack9 = u_var20;
                switch_statement_1008_73ea(param_2, param_2, local_22);
                (i_var33 + 4) = BVar31;
                u_var5 = (param_1 + 0x12);
                uStack14 = u_var5;
                uStack13 = (u_var5 >> 8);
              // u_stack12 = (u_var5  >> 0x10);
                uStack11 = (u_var5 >> 0x18);
                uStack16 = 8;
                uStack15 = 0x10;
                uStack18 = 0xd2;
                uStack17 = 0x9e;
                ppc_var9 = ((param_1 + 0x12) + 4);
                uStack10 = u_var12;
                uStack9 = u_var13;
                bStack8 = u_var15;
                uStack7 = u_var17;
                (**ppc_var9)();
                local_2a = local_2a + 1;
                struct_a = ctx.dx_reg;
            }
            if (local_26 == 0) {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            bStack8 = 1;
            uStack7 = 0;
            uStack14 = 8;
            uStack13 = 0x10;
            uStack16 = 0xa6;
            uStack15 = 0x9e;
            ppc_var9 = local_26;
            local_16._0_2_ = i_var33;
            local_16._2_1_ = u_var15;
            local_16._3_1_ = u_var17;
            u_stack12 = u_var12;
            uStack11 = u_var13;
            uStack10 = u_var15;
            uStack9 = u_var17;
            (**ppc_var9)();
            ctx.g_u16_1050_0310 = 0x6d2;
            return;
        }
        0x18 => {
            local_4._0_1_ = 2;
            // goto LAB_1010_2ad8;
        }
        0x19 => {
            uStack6 = 0x3c;
            uStack5 = 0;
            bStack8 = 0x2a;
            uStack7 = 0;
            uStack10 = 8;
            uStack9 = 0;
            uStack16 = 0x10;
            uStack15 = 0x10;
            uStack18 = 0x40;
            uStack17 = 0x6e;
            uStack14 = u_var12;
            uStack13 = u_var15;
            u_stack12 = u_var18;
            uStack11 = u_var17;
            u_var30 = pass1_1010_6ca2(CONCAT13(u_var17, CONCAT12(u_var18, param_1)), 8);
            if (u_var30 != 0) {
                param_1 = 0x1a;
                uStack2 = 0x52;
                uStack1 = 0x6e;
                pass1_1010_715c(CONCAT22(0x1a, i_var33), 0x1a);
            }
            if (param_2 == 0x2c) {
                uStack2 = 99;
                uStack1 = 0x6e;
                pass1_1010_715c(CONCAT22(0x1d, param_1), 0x1d);
            }
            uStack2 = 0x5a;
            uStack1 = 0;
            local_4._0_1_ = 0x10;
            local_4 = 0x10;
            uStack6 = 0x74;
            uStack5 = 0x6e;
            u_var30 = pass1_1010_6ca2(0x5a, 2);
            if (u_var30 != 0) {
                uStack2 = 0x27;
                uStack1 = 0x6d;
                pass1_1010_715c(0x1c005a, 0x1c);
            }
            return;
        }
        0x1a => {
            (param_1 + 0x26) = param_2;
        }
    }
    local_4._0_1_ = 1;
    // LAB_1010_2ad8:
    local_4 = 0;
    if ((local_4 == 1) || (local_4 == 2)) {
        if (local_4 == 1) {
            param_2 = ((param_1 + 0x20) + (param_1 + 0x22) + (param_1 + 0x24) + (param_1 + 0x26));
        }
        if (param_2 != 0) {
            u_var26 = param_2 / 2 + 1;
            if (5 < u_var26) {
                u_var26 = 5;
            }
            param_2 = u_var26;
            if ((local_4 == 1) && (PTR_LOOP_1050_10c6 != 0x0)) {
                if (4 < u_var26) {
                    u_var26 = 4;
                }
                param_2 = u_var26;
            }
        }
    }
    (local_4 * 0x7c + 0xed6) = param_2;
    uStack6 = 0xc;
    uStack5 = 0;
    u_stack12 = unaff_cs;
    uStack11 = (unaff_cs >> 8);
    uStack14 = 0x4b;
    uStack13 = 0x2b;
    uStack10 = u_var12;
    uStack9 = u_var15;
    bStack8 = u_var18;
    uStack7 = u_var17;
    pass1_1010_1f62(CONCAT13(u_var17, CONCAT12(u_var18, param_1)), 0xc);
    // switchD_1010_2ab5_caseD_0:
    return;
}

pub unsafe fn win_gui_fn_1010_32f4(param_1: &mut  Struct387, param_2: &mut  u32) {
    let pu_var1: &mut  u16;
    let pu_var2: &mut  u32;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let mut u_var5: u32;
    let mut u_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    //

    let local_bx_5: &mut  Struct387;
    let mut i_var9: i32;
    let mut i_var10: i32;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut unaff_cs: u16;
    let mut u_var13: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let temp_86224844142: &mut  u32;
    let mut temp_5f9d4a48c3: u32;
    let mut temp_5fd8766d76: u32;
    let fn_ptr_1: fn();

  // u_var11 = (param_1  >> 0x10);
    local_bx_5 = param_1;
    if (&local_bx_5.field_0x52 != 0) {
        unaff_cs = 0x1000;
        error_check_1000_17ce(&local_bx_5.field_0x52);
        &local_bx_5.field_0x52 = 0;
        local_bx_5.field_0x18 = 0;
    }
    u_var6 = param_2 | param_2;
    if ((param_2 != 0x0)
        && (
            fn_ptr_1 = (param_1 + 0x24),
            (**fn_ptr_1)(unaff_cs, param_1, param_2),
            u_var6 != 0,
        ))
    {
        unsafe {
            fn_ptr_1 = (*param_2 + 4);
        }
        (**fn_ptr_1)(unaff_cs, param_2);
        local_bx_5.field_0x18 = u_var6;
        if (u_var6 != 0) {
            local_bx_5.field_0x24 = 0;
            local_bx_5.field_0x26 = 0;
            g_struct_ptr_1 = local_bx_5.field_0x28;
            pu_var1 = &local_bx_5.field_0x18;
            unsafe {
                *pu_var1 = *pu_var1 - g_struct_ptr_1;
            }
            if (10 < local_bx_5.field_0x18) {
                local_bx_5.field_0x26 = 1;
                local_bx_5.field_0x18 = 10;
            }
            if (1 < local_bx_5.field_0x28) {
                local_bx_5.field_0x24 = 1;
            }
            if (ctx.g_struct_ptr_1 == 0) {
                ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
                struct_fn_1000_160a();
            } else {
            }
            u_var13 = 0x1000;
            alloc_mem_1000_1708(0x28, 0, 1, g_struct_ptr_1, ctx.g_u16_ptr_1050_5f2e);
            local_bx_5.field_0x52 = g_struct_ptr_1;
            &local_bx_5.field_0x54 = ctx.g_u16_ptr_1050_5f2e;
            if ((&local_bx_5.field_0x54 | local_bx_5.field_0x52) != 0) {
                u_var3 = (param_2 + 8);
                i_var7 = local_bx_5.field_0x10 + -10;
                local_c = 0;
                local_10 = 0;
                while (
                    pu_var1 = &local_bx_5.field_0x18,
                    unsafe { *pu_var1 != local_10 } && unsafe { local_10 <= *pu_var1 },
                ) {
                    u_var4 = &local_bx_5.field_0x52;
                    u_var6 = u_var4 + local_10 * 4;
                    u_var4 = u_var4 & 0xffff0000;
                    _local_30 = (u_var4 | u_var6);
                    temp_5fd8766d76 = ((local_bx_5.field_0x28 + local_10) * 4 + u_var3);
                    fn_ptr_1 = (param_1 + 0x1c);
                    u_var8 = local_10;
                    (**fn_ptr_1)(
                        u_var13,
                        param_1,
                        temp_5fd8766d76,
                        (temp_5fd8766d76 >> 0x10),
                        local_bx_5.field_0x22,
                    );
                    *_local_30 = u_var8;
                    *(u_var6 + 2) = ctx.dx_reg;
                    u_var5 = &local_bx_5.field_0x52;
                    u_var5 = (u_var5 + local_10 * 4);
                    local_c = local_c + (u_var5 + 0x24) + 8;
                    if (i_var7 < local_c) {
                        u_var13 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                        fn_1008_6048(
                            s_overflow_on_node__d_1050_11ca,
                            ctx.dx_reg,
                            SUB21(i_var7, 0),
                        );
                        local_bx_5.field_0x18 = local_10 - 1;
                        local_bx_5.field_0x26 = 1;
                        u_var5 = &local_bx_5.field_0x52;
                      // u_var12 = (u_var5  >> 0x10);
                        i_var10 = u_var5;
                        pu_var2 = (i_var10 + local_10 * 4);
                        u_var6 = (i_var10 + local_10 * 4 + 2);
                        if ((u_var6 | pu_var2) != 0) {
                            unsafe {
                                fn_ptr_1 = *pu_var2;
                            }
                            (**fn_ptr_1)(&ctx.PTR_LOOP_1050_1008, pu_var2, u_var6, 1);
                        }
                        u_var5 = &local_bx_5.field_0x52;
                        i_var10 = local_10 * 4;
                        (u_var5 + i_var10) = 0;
                        if (0 < local_10) {
                            u_var5 = &local_bx_5.field_0x52;
                          // u_var12 = (u_var5  >> 0x10);
                            i_var9 = u_var5;
                            pu_var2 = (i_var9 + i_var10 + -4);
                            u_var6 = (i_var9 + i_var10 + -2);
                            if ((u_var6 | pu_var2) != 0) {
                                unsafe {
                                    fn_ptr_1 = *pu_var2;
                                }
                                (**fn_ptr_1)(&ctx.PTR_LOOP_1050_1008, pu_var2, u_var6, 1);
                            }
                            u_var5 = &local_bx_5.field_0x52;
                            (local_10 * 4 + u_var5 + -4) = 0;
                        }
                    }
                    local_10 = local_10 + 1;
                }
                local_bx_5.field_0x20 = 10;
                u_var13 = local_bx_5.field_0x1e;
                u_var3 = &local_bx_5.field_0x52;
                window::update_window_1040_93aa(u_var3, (u_var3 >> 0x10), 10, u_var13);
                local_10 = 1;
                while (
                    pu_var1 = &local_bx_5.field_0x18,
                    unsafe { *pu_var1 != local_10 } && unsafe { local_10 <= *pu_var1 },
                ) {
                    u_var3 = &local_bx_5.field_0x52;
                    u_var3 = (local_10 * 4 + u_var3 + -4);
                    i_var7 = u_var3;
                  // u_var12 = (u_var3  >> 0x10);
                    u_var3 = &local_bx_5.field_0x52;
                    u_var3 = (u_var3 + local_10 * 4);
                    window::update_window_1040_93aa(
                        u_var3,
                        (u_var3 >> 0x10),
                        (i_var7 + 0x20) + (i_var7 + 0x24) + 0x8,
                        u_var13,
                    );
                    local_10 = local_10 + 1;
                }
            }
        }
    }
    return;
}

pub unsafe fn win_gui_fn_1010_79aa(param_1: &mut Struct2111, param_2: u16, param_3: u32) {
    let hwnd: HWND16;
    let mut u_var1: u32;
    let mut u_var2: u32;
    let local_AX_66: &mut  Struct17;

    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut local_1c: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

  // u_var3 = (param_1  >> 0x10);
    if ((param_1 + 0x1c) != 0) && (param_3 != 0 || (param_2 != 0)) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x1c));
        local_12 = 0;
        while {
            local_AX_66 = local_a;
            pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_66));
            local_e = CONCAT22(ctx.dx_reg, local_AX_66);
            if (ctx.dx_reg | local_AX_66) == 0 {}
            // goto LAB_1010_7a49;
            if ((param_2 == 0) && (local_AX_66.field_0x4 == param_3))
                || (param_3 == 0 && (u_var1 = local_AX_66.field_0x8, (u_var1 + 10) == param_2))
            {
                break;
            }
            (local_AX_66.field_0x4 != param_3)
                || (u_var1 = local_AX_66.field_0x8, (u_var1 + 10) != param_2)
        } {}
        local_12 = local_e;
        // LAB_1010_7a49:
        if (local_12 != 0) {
            u_var2 = (local_12 + 8);
            hwnd = (u_var2 + 6);
            SetFocus16(hwnd);
            BringWindowToTop16(hwnd);
            return;
        }
    }
    return;
}

pub unsafe fn win_gui_fn_1010_8096(param_1: &mut  u32, param_2: u16) {
    let in_struct_1: &mut Struct44;
    let u_var1: u8;
    let pc_var2: String;
    let extraout_var: u32;


    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: i32;
    let string_b: String;
    let mut local_312: u32;
    let mut local_30e: u32;
    let mut local_30a: u16;
    let mut local_308: u16;
    let mut string_1: [u8; 256];
    let mut string_2: [u8; 256];
    let mut string_3: [u8; 260];
    let mut u_var3: u16;

  // u_var5 = (param_1  >> 0x10);
    process_string_1000_4d58(((param_1 + 0xe82) * 4 + 0x2526), 0, 0);
    copy_string_1000_3d3e(CONCAT22(unaff_ss, string_3), CONCAT22(unaff_ss, string_2));
    if (param_2 == 2) {
        string_b = "b";
    } else {
        string_b = "a";
    }
    process_string_1000_3cea(CONCAT22(unaff_ss, string_3), string_b);
    process_string_1000_3cea(CONCAT22(unaff_ss, string_3), CONCAT22(unaff_ss, string_1));
    pc_var2 = string_3;
    set_error_mode_1010_8b14(param_1, pc_var2, unaff_ss);
    _local_30a = CONCAT22(ctx.dx_reg, pc_var2);
    i_var4 = ctx.dx_reg;
    if ((pc_var2 == string_3) && (ctx.dx_reg == unaff_ss)) {
        msg_box::msg_box_1010_8bb4(param_1, pc_var2, ctx.dx_reg);
        i_var4 = ctx.dx_reg;
    }
    unsafe {
        in_struct_1 = *param_1;
    }
    u_var1 = error_check_1000_17ce(in_struct_1);
    u_var3 = CONCAT31(extraout_var, u_var1);
    pass1_fn_1008_60e8(_local_30a, in_struct_1, in_struct_1);
    param_1 = u_var3;
    (param_1 + 2) = i_var4;
    return;
}

pub unsafe fn win_gui_fn_1010_8170(param_1: &mut  u32, param_2: i32) {
    let mut u_var1: i32;
    let in_dx: &mut  u16;
    let mut i_var2: i32;
    let local_bx_20: &mut Struct447;
    let mut u_var3: u16;
    let mut u_var4: u32;

  // u_var3 = (param_1  >> 0x10);
    i_var2 = param_1;
    u_var1 = (i_var2 + 0x680);
    local_bx_20 = (param_2 * 0x10);
    if (local_bx_20.field_0x16 != u_var1) {
        win_gui_fn_1010_8096(param_1, local_bx_20.field_0x16);
        u_var4 = pass1_1010_878c(i_var2, u_var3, local_bx_20.field_0x16);
      // in_dx = (u_var4  >> 0x10);
        u_var1 = u_var4;
        if ((i_var2 + 0x67c) == 0) {
            return;
        }
    }
    param_2 = param_2 * 0x10;
    pass1_1008_6562(
        (i_var2 + 0x67c),
        CONCAT22((param_2 + 0x1c), (param_2 + 0x1e)),
        (param_2 + 0x1a),
        u_var1,
        in_dx,
    );
    return;
}

pub unsafe fn win_gui_fn_1018_e8bc(param_1: &mut Struct44) {
    let local_bx_3: &mut  Struct376;
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    local_bx_3 = param_1;
    param_1.ptr_a_lo = 0xe912;
    local_bx_3.ptr_a_hi = 0x1018;
    if (&local_bx_3.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1dda(local_bx_3.u8_ptr_x14);
    }
    select_and_delete_palette_1020_92c4(param_1);
    return;
}

pub unsafe fn win_gui_fn_1018_e8ec(param_1: &mut  Struct376, param_2: u8) -> &mut  Struct376 {
    win_gui_fn_1018_e8bc(param_1);
    if (param_2 & 1) != 0 {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn win_gui_fn_1018_eada(param_1: &mut WinStruct42) {
    // let local_struct_1: &mut WinStruct42;
    // let local_struct_1_hi: &mut WinStruct42;
    let struct_var1: Struct199;
    // local_DXAX_28: i32;
    let mut local_DXAX_61: u32;
    let mut local_4: u16;

    struct_var1 = create_win_1008_9760(ctx, param_1);
    local_DXAX_28 = (struct_var1 >> 0x10);
    //// ocal_struct_1_hi = (param_1  >> 0x10);
    // local_struct_1 = param_1;
    local_DXAX_28._0_2_ =
        get_gui_dc_1018_4db0(*&local_struct_1.u32_xf2, local_struct_1.win_handle_0x8);
    process_struct_1000_179c(ctx, 0x28, local_DXAX_28);
    if ((local_DXAX_28 | local_DXAX_28) != 0) {
        local_DXAX_61 = draw_fn_1018_ec74(
            ctx,
            local_DXAX_28,
            local_DXAX_28,
            local_struct_1.win_handle_0x8,
        );
        local_struct_1.char_ptr_16_0xee = local_DXAX_61;
        local_struct_1.field_0xf0 = (local_DXAX_61 >> 0x10);
        return;
    }
    &local_struct_1.char_ptr_16_0xee = 0;
    return;
}

pub unsafe fn win_gui_fn_1018_eb3e(in_struct_1: &mut  Struct594) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    // ppu_var4: &mut  Vec<u8>;
    let pa_var5: &mut  Struct594;
    let local_struct_1: &mut  Struct594;
    let local_struct_1_hi: &mut  Struct594;
    let mut in_stack_0000fff2: u16;
    let temp_862c2f7bda0: Vec<u8>;

  // local_struct_1_hi = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    pu_var1 = local_struct_1.u8_ptr_16_xee;
    u_var2 = local_struct_1.field_0xf0;
    if (u_var2 | pu_var1) != 0 {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)();
    }
    if &local_struct_1.field_0xf6 != 0 {
        if (local_struct_1_hi | local_struct_1) == 0 {
            ppu_var4 = 0x0;
            pa_var5 = 0x0;
        } else {
            ppu_var4 = &local_struct_1.u32_xE2;
            pa_var5 = local_struct_1_hi;
        }
        pass1_1010_1ea6(*&local_struct_1.field_0xf6, CONCAT22(pa_var5, ppu_var4));
    }
    window::destroy_win_1008_628e(in_struct_1, in_stack_0000fff2);
    return;
}

pub unsafe fn win_gui_fn_1018_ed98(in_struct_1: &mut Struct44) {
    let local_bx_3: &mut  Struct376;
    let mut u_var1: i32;

  // u_var1 = (in_struct_1  >> 0x10);
    local_bx_3 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x1cc;
    local_bx_3.ptr_a_hi = 0x1020;
    if (&local_bx_3.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1ea6(
            local_bx_3.u8_ptr_x14,
            (in_struct_1 & 0xffff | u_var1 << 0x10),
        );
        // WARNING: Load size is inaccurate
        pass1_1010_1dda(local_bx_3.u8_ptr_x14);
    }
    select_and_delete_palette_1020_92c4(in_struct_1);
    return;
}

pub unsafe fn win_gui_fn_1020_01a6(in_struct_1: &mut  Struct376, in_byte_1: u8) -> &mut  Struct376 {
    win_gui_fn_1018_ed98(in_struct_1);
    if ((in_byte_1 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub unsafe fn win_gui_fn_1020_028c(param_1: &mut  Struct628, param_2: u16, param_3: u16) {
    pass1_1010_3c9e(param_1.field_0xe2);
    window::show_window_1008_68c6(param_1, param_2, param_3);
    return;
}

pub fn win_gui_fn_1020_0762(
    param_1: &mut  Struct65,
    param_2: u32,
    param_3: u32,
    param_4: u16,
    param_5: u32,
    param_6: u32,
) {
    let in_struct_1: &mut  Struct65;
    let in_struct_1_hi: &mut  Struct65;

    in_struct_1 = param_1;
  // in_struct_1_hi = (param_1  >> 0x10);
    cursor::load_cursor_fn_1020_01d8(
        in_struct_1,
        in_struct_1_hi,
        param_2,
        (param_2 >> 0x10),
        param_4,
        param_5,
        (param_5 >> 0x10),
        param_6,
    );
    in_struct_1.u16_xf0 = 0;
    &in_struct_1.u16_xf2 = param_3;
    param_1.ptr_a_lo = 0x81a;
    in_struct_1.ptr_a_hi = 0x1020;
    return;
}

pub unsafe fn win_fn_1020_1294(param_1: &mut  Struct637, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let ppVar4: &mut  pass1_struct_2;
    let pa_var5: &mut  Struct199;
    let h_cursor: &mut  u16;
    let h_cursor_00: HCURSOR16;
    let mut in_dx: i32;

    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar4 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x4000001,
    );
    if ((in_dx | ppVar4) == 0) {
        local_6 = param_3;
        local_4 = param_2;
      // u_var7 = (param_1  >> 0x10);
        i32_var6 = param_1;
        pass1_1010_40cc((i32_var6 + 0xf2));
        _local_a = CONCAT22(ctx.dx_reg, param_2);
        u_var1 = (i32_var6 + 0xf2);
        pa_var5 = (u_var1 + 0x76);
        local_e = u_var1 & 0xffff0000 | ZEXT24(pa_var5);
        pass1_1008_3e94(
            pa_var5,
            CONCAT22(unaff_ss, &local_12),
            CONCAT22(unaff_ss, &local_10),
        );
        local_6 = local_6 - local_10;
        local_4 = local_4 - local_12;
        h_cursor = &local_6;
        u_var2 = (i32_var6 + 0xf2);
        pt_in_rect_1010_40f8(u_var2, (u_var2 >> 0x10), h_cursor, unaff_ss);
        if (h_cursor != 0xffff) {
            h_cursor_00 = LoadCursor16(0x7f02, 0);
            SetCursor16(h_cursor_00);
            ppc_var3 = (*_local_a + 4);
            (**ppc_var3)();
            pass1_1008_3e0e(param_1);
            SetCursor16(h_cursor);
        }
    }
    return;
}

pub unsafe fn get_dc_1020_1418(param_1: &mut  u16, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let pu_var3: &mut  u16;
    let mut u_var4: u32;

    let local_bx_17: &mut  Struct63;
    let mut unaff_ss: u16;
    let ppVar5: &mut  Struct2551;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut in_stack_0000ffdc: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    i32_var6 = param_1;
  // u_var7 = (param_1  >> 0x10);
    get_sys_metrics_1020_7c1a(i32_var6, u_var7, param_2, (param_2 >> 0x10));
    (i32_var6 + 0x14) = 0;
    (i32_var6 + 0x18) = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | (i32_var6 + 0x1e)));
    unsafe {
        *param_1 = 0x1730;
    }
    (i32_var6 + 2) = 0x1020;
    ppVar5 = process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_0000ffdc, 0x2d),
    );
    (i32_var6 + 0x14) = ppVar5;
    (i32_var6 + 0x16) = (ppVar5 >> 0x10);
    _local_6 = process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_0000ffdc, 0x29),
    );
    u_var1 = (i32_var6 + 0x14);
    ppc_var2 = ((i32_var6 + 0x14) + 4);
    ppc_var2(0x10, u_var1, (u_var1 >> 0x10), 0, i32_var6, u_var7);
    local_8 = GetDC16((i32_var6 + 4));
    u_var1 = (i32_var6 + 0x14);
    (u_var1 + 0x7c) = local_8;
    u_var1 = (i32_var6 + 0x14);
    u_var4 = (u_var1 + 0x66);
    (i32_var6 + 0x18) = u_var4;
    ppc_var2 = ((i32_var6 + 0x18) + 0x14);
    ppc_var2();
    u_var1 = (_local_6 + 0xe);
    pass1_1008_4d84((u_var4 & 0xffff | ctx.dx_reg << 0x10), u_var1);
    pu_var3 = &local_8;
    realize_palette_1008_4e08(u_var1, pu_var3, unaff_ss);
    (i32_var6 + 0x1c) = pu_var3;
    return;
}

pub unsafe fn win_fn_1040_a784(
    param_1: &mut  Struct124,
    param_2: &mut  Struct124,
    param_3: u16,
    param_4: u32,
) {
    let h_wnd: HWND16;
    let paVar1: &mut  Struct124;

    paVar1 = param_1;
    if (param_3 != 0xeb) {
        if (param_3 == 500) {
            msg_box::msg_box_1040_a85a(param_1, param_2);
            return;
        }
        if (param_3 == 0x1f7) {
            _PTR_LOOP_1050_5ef0 = (param_1 + 1);
            pass1_1038_af40(ctx.g_struct_112_001, *&param_1.field_0x8, 0x23);
            PostMessage16(0, 2, 0x111, &param_1.field_0x6);
            return;
        }
        if (param_3 != 0x17d8) {
            window::win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        SetWindowPos16(6, 0xed, 0x237, 0, 0, 0, &param_1.field_0x6);
        h_wnd = GetDlgItem16(0x17d8, &param_1.field_0x6);
        paVar1 = offset;
        EnableWindow16(0, h_wnd);
        &param_1[1].field_0x4 = 1;
        param_2 = param_1;
    }
    dialog::set_dialog_item_1040_a94a(CONCAT22(param_2, paVar1));
    return;
}

pub unsafe fn win_fn_1020_0dc4(in_struct_1: &mut WinStruct42, param_2: u16, param_3: u32) {
    let mut i_var1: i32;
    let local_struct_1_hi: &mut WinStruct42;

    cursor::call_load_cursor_1020_790e(in_struct_1, s_PCPOPMENU_1050_4256, param_2, param_3);
  // local_struct_1_hi = (in_struct_1  >> 0x10);
    i_var1 = in_struct_1;
    (i_var1 + 0xf2) = 0;
    (i_var1 + 0xf6) = 0;
    (i_var1 + 0xfa) = 0;
    in_struct_1.u16_x0 = 0x1384;
    (i_var1 + 2) = 0x1020;
    copy_string_1000_3d3e(
        (in_struct_1 & 0xffff0000 | (i_var1 + 0x5b)),
        s_VrMode_1050_4260,
    );
    (i_var1 + 0xac) = 0x44c00000;
    window::update_window_1020_10a0(in_struct_1);
    return;
}

pub unsafe fn win_fn_1020_09ba(struct_param_1: &mut WinStruct42) {
    // let local_struct_1: WinStruct42;
    // let local_struct_1_hi: WinStruct42;
    let in_struct_1_00: Struct636;
    let mut u_var1: u32;
    let mut local_4: u16;

    let struct_var_1 = create_win_1008_9760(ctx, struct_param_1);
    process_struct_1000_179c(0xe, &mut struct_var_1);
    // local_struct_1 = struct_param_1;
    //// ocal_struct_1_hi = (struct_param_1  >> 0x10);
    if struct_var_1 != 0x0 {
        u_var1 = process_struct_1020_0baa(struct_var_1, local_struct_1.win_handle_0x8);
        local_struct_1.u16_xe2 = u_var1;
        local_struct_1.u16_xe4 = (u_var1 >> 0x10);
        return;
    }
    struct_param_1.u16_xe2 = 0;
    return;
}

pub fn win_fn_1018_e6c6(ctx: &mut AppContext, in_struct_a: &mut WinStruct42) {
    let mut rc_1: u16;
    let mut string_base_a: String;
    let mut struct_a: Struct199;
    let mut local_dx_44: Vec<u8>;
    let mut struct_c_lo: WinStruct42;
    let mut struct_c_hi: WinStruct42;
    let mut struct_b: Struct199;
    let mut local_4: Vec<u8>;

    struct_b = create_win_1008_9760(ctx, in_struct_a);
  // struct_a = (struct_b  >> 0x10);
  // struct_c_hi = (in_struct_a  >> 0x10);
  //   struct_c_lo = in_struct_a;
    rc_1 = get_gui_dc_1018_4db0(&mut in_struct_a.u32_xf2,
                                in_struct_a.win_handle_0x8);
    process_struct_1000_179c(ctx, 0x18, &mut struct_b);
    local_dx_44 = (struct_a | rc_1);
    if local_dx_44 != 0x0 {
        string_base_a._0_1_ =
            pass1_1018_e834(rc_1, CONCAT22(in_struct_a.win_handle_0x8, struct_a));
        string_base_a._0_2_ = CONCAT11(string_base_a, string_base_a);
        struct_c_lo.char_ptr_16_0xee = string_base_a;
        struct_c_lo.field_0xf0 = local_dx_44;
        return;
    }
    &struct_c_lo.char_ptr_16_0xee = 0;
    return;
}

pub unsafe fn win_fn_1018_e384(ctx: &mut AppContext, struct_param_1: &mut WinStruct42) {
    let mut struct_var_1: Struct622 = Struct622::new();
    let mut local_DXAX_61: u32;
    let mut local_4: u16;

    let struct_var_2 = create_win_1008_9760(ctx, struct_param_1);
    //// truct_a = (paVar1  >> 0x10);
    //// ocal_WinStruct42_ptr_1_hi = (struct_param_1  >> 0x10);
    // local_WinStruct42_ptr_1 = struct_param_1;
    struct_var_1.u16_0x0 = get_gui_dc_1018_4db0(
        &struct_param_1.u32_xf2,
        local_WinStruct42_ptr_1.win_handle_0x8,
    );
    process_struct_1000_179c(0x18, struct_a);
    if (struct_a | struct_var_1) != 0 {
        let local_DXAX_61 = draw_fn_1018_e4f2(
            ctx,
            &mut struct_var_1,
            &mut local_WinStruct42_ptr_1.win_handle_0x8,
        );
        local_WinStruct42_ptr_1.char_ptr_16_0xee = local_DXAX_61;
        local_WinStruct42_ptr_1.field_0xf0 = (local_DXAX_61 >> 0x10);
        return;
    }
    local_WinStruct42_ptr_1.char_ptr_16_0xee = 0;
    return;
}

pub unsafe fn win_fn_1018_5e9a(param_1: &mut  Struct20) {
    let mut u_var1: u32;
    char * *ppc_var2;
    let mut u_var3: i32;
    let i_var4: u16;
    let pu_var5: &mut  u16;
    let struct_a: &mut  Struct199;


    let mut i32_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let ppVar10: &mut  Struct2551;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb6: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u32;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 6];
    let mut local_8: u16;
    let mut local_6: u32;

    win_gui_func_1040_78e2(param_1);
    ppVar10 = process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_0000ffb2, 0x39),
    );
  // struct_a = (ppVar10  >> 0x10);
    u_var3 = ppVar10;
  // u_var8 = (param_1  >> 0x10);
    i32_var6 = param_1;
    (i32_var6 + 0x8e) = u_var3;
    (i32_var6 + 0x90) = struct_a;
    process_struct_1000_179c(0x12, struct_a);
    if ((struct_a | u_var3) == 0) {
        (i32_var6 + 0x92) = 0;
    } else {
        pass1_1018_6198(CONCAT22(struct_a, u_var3), param_1, (i32_var6 + 6));
        (i32_var6 + 0x92) = u_var3;
        (i32_var6 + 0x94) = ctx.dx_reg;
    }
    u_var1 = (i32_var6 + 0x8e);
    local_6 = u_var1 & 0xffff0000 | (u_var1 + 10);
    GetClientRect16(CONCAT22(unaff_ss, local_e), (i32_var6 + 6));
    i_var4 = GetSystemMetrics16(4);
    (local_6 + 6) = i_var4 + local_8;
    ppVar10 = process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_0000ffb6, 0x48),
    );
  // local_14 = (ppVar10  >> 0x10);
    local_16 = ppVar10;
    local_10 = (local_16 + 10);
    local_12 = (local_16 + 0xc);
  // u_var9 = (local_6  >> 0x10);
    (local_6 + 2) = (local_12 - (local_6 + 6)) / 2;
    local_6 = local_10 / 2 + 3;
    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_28),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x100,
    );
    loop {
        pu_var5 = &local_28;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var5));
        if ((ctx.dx_reg | pu_var5) == 0) {
            break;
        }
        ppc_var2 = (pu_var5 + 8);
        if (ppc_var2 != 0x0) {
            process_string_1000_3cea((param_1 & 0xffff0000 | (i32_var6 + 0x10)), *ppc_var2);
        }
    }
  // u_var9 = (local_6  >> 0x10);
    i_var7 = local_6;
    SetWindowPos16(
        0x44,
        (i_var7 + 6),
        (i_var7 + 4),
        (i_var7 + 2),
        local_6,
        0,
        (i32_var6 + 6),
    );
    return;
}

pub unsafe fn win_cleanup_1018_4d22(param_1: &mut  Struct376) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let h_gdi_obj: HPALETTE16;
    let local_bx_4: &mut  Struct43;
    let mut u_var4: u16;
    let mut unaff_cs: u16;

  // u_var4 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    param_1.ptr_a_lo = (s_SinternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12);
    local_bx_4.field_0x2 = 0x1018;
    if (local_bx_4.h_dc != 0) {
        h_gdi_obj = SelectPalette16(0, local_bx_4.palette, local_bx_4.h_dc);
        DeleteObject16(h_gdi_obj);
        unaff_cs = SUB42(offset, 0);
        DeleteDC16(local_bx_4.h_dc);
    }
    pu_var1 = local_bx_4.field_0xa;
    u_var2 = local_bx_4.field_0xc;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)(unaff_cs, pu_var1, u_var2, 1);
    }
    pu_var1 = local_bx_4.field_0xe;
    u_var2 = local_bx_4.field_0x10;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)(unaff_cs, pu_var1, u_var2, 1);
    }
    _PTR_LOOP_1050_4230 = 0;
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn win_fn_1018_2978(param_1: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();

    let pu_var3: Vec<u8>;
    let pu_var4: Vec<u8>;
    let pu_var5: &mut  u16;
    let mut i32_var6: i32;

    let struct_a: &mut  Struct199;
    let pa_var7: &mut  Struct199;

    let mut u_var8: u16;
    let struct_a_00: &mut  Struct199;

    //

    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut unaff_ss: u16;
    let u_var12: u8;
    let mut local_42: u32;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: [u8; 36];
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_fn_1010_8096(ctx.g_struct_73_1050_14cc, 1);
    pu_var3 = local_2a;
    u_var12 = (unaff_ss >> 8);
    local_4 = ctx.dx_reg;
    process_struct_1008_48fe(
        CONCAT13(u_var12, CONCAT12(unaff_ss, pu_var3)),
        1,
        CONCAT22(ctx.dx_reg, in_ax),
    );
    u_var11 = 0x1000;
    pa_var7 = struct_a;
    process_struct_1000_179c(0x1e, struct_a);
    if ((pa_var7 | pu_var3) == 0) {
        pu_var4 = 0x0;
        u_var8 = 0;
    } else {
        pu_var4 = local_2a;
        u_var11 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pass1_1008_3f92(CONCAT22(pa_var7, pu_var3), CONCAT22(unaff_ss, pu_var4));
        u_var8 = ctx.dx_reg;
    }
    _local_2e = CONCAT22(u_var8, pu_var4);
    ppc_var2 = (*_local_2e + 0x14);
    ppc_var2(u_var11, pu_var4, u_var8);
    _local_32 = CONCAT22(struct_a_00, pu_var4);
    pa_var7 = struct_a_00;
    process_struct_1000_179c(0x14, struct_a_00);
    if ((pa_var7 | pu_var4) == 0) {
        pu_var4 = 0x0;
        u_var11 = 0;
    } else {
        process_struct_1008_4c58(CONCAT22(pa_var7, pu_var4));
        u_var11 = ctx.dx_reg;
    }
  // u_var10 = (param_1  >> 0x10);
    i_var9 = param_1;
    (i_var9 + 0xe) = pu_var4;
    (i_var9 + 0x10) = u_var11;
    pass1_1008_4d84((i_var9 + 0xe), _local_32);
    pu_var5 = &local_3a;
    pa_var7 = ctx.dx_reg;
    GetClientRect16(
        CONCAT13(u_var12, CONCAT12(unaff_ss, pu_var5)),
        ctx.g_h_window,
    );
    u_var11 = 0x1000;
    process_struct_1000_179c(0x1e, pa_var7);
    if ((pa_var7 | pu_var5) == 0) {
        (i_var9 + 10) = 0;
    } else {
        i32_var6 = (local_34 - local_38) + 1;
        u_var1 = (i_var9 + 0xe);
        u_var11 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pass1_1008_405c(
            pu_var5,
            pa_var7,
            u_var1,
            (u_var1 >> 0x10),
            i32_var6,
            (local_36 - local_3a) + 1,
        );
        (i_var9 + 10) = i32_var6;
        (i_var9 + 0xc) = ctx.dx_reg;
    }
    if (_local_2e != 0x0) {
        ppc_var2 = *_local_2e;
        ppc_var2(
            u_var11,
            _local_2e,
            (_local_2e >> 0x10),
            1,
            u_var8,
            _local_2e,
            _local_2e,
        );
    }
    close_file_1008_496c(local_2a);
    return;
}

pub unsafe fn win_fn_1010_71d6(param_1: u32, param_2: u16, param_3: &mut  u16) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut in_ax: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;

    let mut u_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var7 = (param_1  >> 0x10);
    pass1_1010_ad22((param_1 + 0x14));
    _local_6 = CONCAT22(ctx.dx_reg, in_ax);
    if ((ctx.dx_reg | in_ax) == 0) {
        return;
    }
    u_var8 = pass1_1030_73a8(CONCAT22(ctx.dx_reg, in_ax));
  // u_var6 = (u_var8  >> 0x10);
    u_var2 = u_var8;
    if (((u_var6 | u_var2) != 0) && ((u_var2 + 0x1c) == 0x8000002)) {
        return;
    }
    u_var1 = (in_ax + 0x2e);
    local_e._0_2_ = u_var1;
    if ((((in_ax + 0x30) | local_e) != 0) && ((local_e + 0x200) == 0x8000002)) {
        return;
    }
    u_var1 = (param_1 + 0x14);
  // u_var4 = pass1_1010_b028(u_var1, (u_var1  >> 0x10), u_var8);
    u_var5 = (u_var2 + 0x12);
    u_var3 = u_var5;
    if ((u_var5 != 4) && (u_var3 = param_2, u_var5 == 7)) {
        param_2 = 5;
        u_var3 = param_2;
    }
    param_2 = u_var3;
    u_var5 = param_2 - 2;
    if (u_var5 != 0) {
        if (param_2 == 3) {
            u_var5 = u_var4 - 0xb;
            if ((u_var5 == 0) || (u_var5 = u_var4 - 0x37, u_var5 == 0)) {
                local_14 = 0xb;
            } else {
                local_14 = 10;
            }
            // goto LAB_1010_72a7;
        }
        u_var5 = param_2 - 4;
        if (u_var5 == 0) {
            local_14 = 0x17;
            // goto LAB_1010_72a7;
        }
        u_var5 = param_2 - 5;
        if (u_var5 != 0) {
            pass1_1010_7818(param_1, u_var8);
            local_14 = u_var5;
            // goto LAB_1010_72a7;
        }
    }
    local_14 = 0xc;
    // LAB_1010_72a7:
    if (local_14 == 0) {
        return;
    }
    win_gui_fn_1010_79aa(param_1, 0, _local_6);
    if (u_var5 == 0) {
        window_msg_func_1010_7300(param_1, 0, 0, local_14, _local_6);
    }
    return;
}

pub unsafe fn win_fn_1010_7174(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    i_var2 = param_1;
  // u_var3 = (param_1  >> 0x10);
    if (param_2 == 0x13) {
        u_var1 = (i_var2 + 0x18);
        window::destroy_win_1010_7b26(param_1 & 0xffff0000 | (i_var2 - 10), (u_var1 + 0x28));
        return;
    }
    if (param_2 < 0x14) {
        if (param_2 == 0x1) {
            (i_var2 + 10) = 0;
            (i_var2 + 0x18) = 0;
            return;
        }
        if (param_2 == '\x05') {
            send_msg_1010_7c42(param_1 & 0xffff0000 | (i_var2 - 10));
            return;
        }
    }
    return;
}

pub fn win_fn_1040_c886(param_1: u32, param_2: u8, param_3: HDC16) {
    let pp_var1: fn();
    let p_uvar2: &mut  u16;
    let h_gdi_obj: HPALETTE16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;
    let mut local_8: u32;
    let mut local_4: u16;

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    if (((i_var3 + 10) | (i_var3 + 8)) != 0) {
        local_4 = 0;
        if ((i_var3 + 0x46) == 0) {
          // u_var5 = (_PTR_LOOP_1050_4230  >> 0x10);
            local_c = (_PTR_LOOP_1050_4230 + 0xe);
            pu_var2 = &param_3;
            unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            realize_palette_1008_4e08(local_c, (_PTR_LOOP_1050_4230 + 0x10), pu_var2, unaff_ss);
            local_4 = pu_var2;
        }
        local_8 = (i_var3 + 8);
        u_var5 = (i_var3 + 10);
        if ((((i_var3 + 0xe) | (i_var3 + 0xc)) != 0) && ((param_2 & 1) != 0)) {
            local_8 = (i_var3 + 0xc);
            u_var5 = (i_var3 + 0xe);
        }
        if (((i_var3 + 0x10) != 0) && ((param_2 & 4) != 0)) {
            local_8 = (i_var3 + 0x10);
            u_var5 = (i_var3 + 0x12);
        }
        pp_var1 = (local_8 + 4);
        (**pp_var1)(
            unaff_cs,
            local_8,
            u_var5,
            (i_var3 + 0x28),
            (i_var3 + 0x26),
            &param_3,
        );
        if ((i_var3 + 0x46) == 0) {
            h_gdi_obj = SelectPalette16(0, local_4, param_3);
            DeleteObject16(h_gdi_obj);
        }
    }
    return;
}

pub fn win_fn_1040_c028(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: i32;
    let mut i_var7: i32;
    let mut u_var9: u16;
    let unaff_cs: bool;
    let unaff_ss: &mut  u16;
    let rect: &mut  RECT16;
    let mut local_34: u32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let pu_var8: &mut  u16;

    i_var7 = param_1;
  // u_var9 = (param_1  >> 0x10);
    if (param_2 == 8) {
        GetClientRect16(CONCAT22(unaff_ss, &local_a), (i_var7 + 4));
        u_var1 = (i_var7 + 6);
        u_var3 = (i_var7 + 6);
        i_var5 = (u_var3 + 0x16);
        u_var3 = (i_var7 + 6);
        local_a = (u_var3 + 0x1a);
        u_var3 = (i_var7 + 6);
        local_8 = (u_var3 + 0x1c);
        if (i_var5 != 0) {
            if (i_var5 < 2) {
                i_var4 = 1;
            } else {
                i_var4 = 2;
            }
            u_var2 = ((i_var5 - i_var4) * 4 + u_var1 + 0x2a);
            i_var5 = u_var2;
            u_var2 = u_var2 & 0xffff0000;
            local_a = (i_var5 + 0x22) + (u_var2 | i_var5 + 0x1e);
        }
        u_var1 = (i_var7 + 6);
        local_6 = (u_var1 + 0x1e);
        local_4 = local_4 - 5;
    } else {
        if (param_2 != 9) {
            if (param_2 != 10) {
                return;
            }
            u_var1 = (i_var7 + 6);
            u_var6 = u_var1 + 0x2a;
            if (((i_var7 + 8) | u_var6) == 0) {
                return;
            }
            u_var3 = (i_var7 + 6);
            u_var2 = (((u_var3 + 0x16) + -1) * 4 + u_var6);
            i_var7 = u_var2;
            u_var2 = u_var2 & 0xffff0000;
            pu_var8 = (u_var2 | i_var7 + 0x1e);
          // u_var9 = (u_var2  >> 0x10);
            local_8 = (i_var7 + 0x20) - 8;
            unsafe {
                local_a = *pu_var8;
            }
            unsafe {
                local_6 = (i_var7 + 0x22) + *pu_var8;
            }
            local_4 = (i_var7 + 0x20);
            rect = &local_a;
            unaff_cs = 0;
            // goto LAB_1040_c19d;
        }
        local_a = 0;
        local_8 = 0;
        local_6 = 0;
        local_4 = 0;
        GetClientRect16(CONCAT22(unaff_ss, &local_a), (i_var7 + 4));
        u_var1 = (i_var7 + 6);
        local_a = (u_var1 + 0x1a);
        u_var1 = (i_var7 + 6);
        local_6 = (u_var1 + 0x1e);
        local_4 = local_4 - 5;
        u_var1 = (i_var7 + 6);
        u_var3 = (i_var7 + 6);
        i_var7 = (u_var3 + 0x16);
        if (0 < i_var7) {
            u_var1 = (u_var1 + i_var7 * 4 + 0x26);
            i_var7 = u_var1;
          // u_var9 = (u_var1  >> 0x10);
            local_8 = (i_var7 + 0x20) + (i_var7 + 0x24);
        }
    }
    unaff_ss = &local_a;
    rect = (&ctx.PTR_LOOP_1050_0000 + 1);
    // LAB_1040_c19d:
    InvalidateRect16(unaff_cs, rect, unaff_ss);
    return;
}

pub unsafe fn win_fn_1040_cace(param_1: u32) {
    let mut u_var1: u32;
    let mut b_var2: bool;
    let mut i_var3: i32;
    let i_var4: u16;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let unaff_ss: u16;
    let mut bVar7: bool;
    let mut b: u16;
    let mut u_var8: u16;
    let mut local_218: u16;
    let mut local_214: u16;
    let mut local_20c: u32;
    let mut local_208: [u8; 256];
    let mut local_108: [u8; 256];
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: bool;

  // u_var6 = (param_1  >> 0x10);
    i_var5 = param_1;
    local_6 = GetDlgItemInt16(0, &local_4, unaff_ss, 0x18e);
    local_8 = GetDlgItemInt16(0, &local_4, unaff_ss, 0x191);
    if (local_6 == 0) {
        return;
    }
    pass1_1018_50ea((i_var5 + 0x98), local_6, (i_var5 + 0x94));
    load_string_1010_84e0(
        ctx.g_struct_73_1050_14cc,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_208),
        0x57b,
    );
    u_var1 = (i_var5 + 0x94);
  // b = (ctx.g_struct_73_1050_14cc  >> 0x10);
    if (u_var1 + 0x36) == 0 {
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            b,
            0x3ff,
            CONCAT22(unaff_ss, local_108),
            0x7ed,
        );
        i_var3 = MessageBox16(
            0x34,
            CONCAT22(unaff_ss, local_208),
            CONCAT22(unaff_ss, local_108),
            (i_var5 + 8),
        );
    } else {
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            b,
            0x3ff,
            CONCAT22(unaff_ss, local_108),
            0x7ec,
        );
        i_var3 = MessageBox16(
            0x34,
            CONCAT22(unaff_ss, local_208),
            CONCAT22(unaff_ss, local_108),
            (i_var5 + 8),
        );
    }
    b_var2 = i_var3 == 6;
    bVar7 = false;
    if ((!b_var2) && (u_var1 = (i_var5 + 0x94), (u_var1 + 0x34) < 1)) {
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_ss, local_108),
            0x7ee,
        );
        i_var4 = MessageBox16(
            0x34,
            CONCAT22(unaff_ss, local_208),
            CONCAT22(unaff_ss, local_108),
            (i_var5 + 8),
        );
        bVar7 = i_var4 == 6;
        b_var2 = false;
    }
    if (b_var2) {
        _ctx.PTR_LOOP_1050_5f16 = (i_var5 + 0x94);
        u_var8 = 0x26;
    } else {
        if (!bVar7) {
            return;
        }
        _ctx.PTR_LOOP_1050_5a68 = (i_var5 + 0x94);
        u_var8 = 0x27;
    }
    pass1_1038_af40(ctx.g_struct_112_001, *(i_var5 + 8), u_var8);
    return;
}

pub unsafe fn win_fn_1008_84f4(param_1: u16, uparam_2_00: i32, param_2: WPARAM16, param_3: u32) {
    let pu8_var1: Vec<u8>;
    let mut u_var2: i32;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut cVar5: u8;
    let b_var6: bool;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let in_struct_1: &mut Struct44;
    let u_var8: u8;
    let HVar9: HWND16;
    let hwnd: HWND16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_4: u16;

    hwnd = &ctx.g_alloc_addr_1050_1050;
    HVar9 = param_3;
    in_struct_1 = GetWindowLong16(0, param_3);
  // u_var7 = (in_struct_1  >> 0x10);
    i_var4 = in_struct_1;
    if (param_3 == 0x1f) {
        (i_var4 + 4) = 0;
        KillTimer16(0xfa6, param_3);
        KillTimer16(0xfa7, param_3);
        ReleaseCapture16(hwnd);
    } else {
        cVar5 = param_3;
      // u_var8 = (param_3  >> 0x10);
        if (param_3 < 0x20) {
            if (param_3 != 0x14) {
                if (0x14 < param_3) {}
                // goto LAB_1008_8771;
                if (cVar5 == 0x1) {
                    // LAB_1008_8560:
                    win_fn_1008_8214(u_var8, cVar5, param_2, param_1, param_2_00);
                    return;
                }
                if (cVar5 == 0x2) {
                    error_check_1000_17ce(in_struct_1);
                } else {
                    if (cVar5 != 0xc) {
                        if (cVar5 != 0xf) {}
                        // goto LAB_1008_8771;
                        draw_1008_8288(param_3, i_var4, u_var7);
                    }
                }
            }
        } else {
            if (param_3 == 0x200) {
                if ((*(i_var4 + 4) & 1) != 0) {
                    GetClientRect16(CONCAT22(unaff_ss, &stack0xfff6), param_3);
                    i_var3 = (i_var4 + 4);
                    pu8_var1 = (i_var4 + 4);
                    unsafe {
                        *pu8_var1 = *pu8_var1 & 0xf3;
                    }
                    b_var6 = PtInRect16(CONCAT22(param_2_00, param_1), &stack0xfff6);
                    if (b_var6 == 0) {
                        pu8_var1 = (i_var4 + 4);
                        unsafe {
                            *pu8_var1 = *pu8_var1 | 2;
                        }
                    } else {
                        if (param_2_00 < local_4 >> 1) {
                            pu8_var1 = (i_var4 + 4);
                            unsafe {
                                *pu8_var1 = *pu8_var1 | 4;
                            }
                        } else {
                            pu8_var1 = (i_var4 + 4);
                            unsafe {
                                *pu8_var1 = *pu8_var1 | 8;
                            }
                        }
                        pu8_var1 = (i_var4 + 4);
                        unsafe {
                            *pu8_var1 = *pu8_var1 & 0xfd;
                        }
                    }
                    if ((i_var4 + 4) != i_var3) {
                        InvalidateRect16(1, 0x0, 0);
                        UpdateWindow16(param_3);
                    }
                }
            } else {
                if (param_3 < 0x201) {
                    if (param_3 == 0x81) {}
                    // goto LAB_1008_8560;
                    if (param_3 != 0x113) {
                        // LAB_1008_8771:
                        DefWindowProc16(
                            CONCAT13((param_2_00 >> 8), CONCAT12(param_2_00, param_1)),
                            param_2,
                            param_3,
                            param_3,
                        );
                        return;
                    }
                    if (param_2 == 0xfa6) {
                        KillTimer16(0xfa6, param_3);
                        SetTimer16(0, 0, 1, 0xfa7);
                        HVar9 = param_3;
                    }
                    if ((*(i_var4 + 4) & 2) == 0) {
                        send_win_msg_1008_84ba(u_var8, i_var4, u_var7, HVar9);
                    }
                } else {
                    if (param_3 != 0x201) {
                        if (param_3 == 0x202) {
                            KillTimer16(0xfa6, param_3);
                            KillTimer16(0xfa7, param_3);
                            ReleaseCapture16(hwnd);
                            u_var2 = (i_var4 + 4);
                            if (((u_var2 & 1) != 0) && ((u_var2 & 0xfffd) != 0)) {
                                pu8_var1 = (i_var4 + 4);
                                unsafe {
                                    *pu8_var1 = *pu8_var1 & 0xf2;
                                }
                                InvalidateRect16(1, 0x0, 0);
                                UpdateWindow16(param_3);
                            }
                            SendMessage16(*(i_var4 + 2), 0xf9, 0x111, in_struct_1.ptr_a_lo);
                            return;
                        }
                        if (param_3 != 0x203) {}
                        // goto LAB_1008_8771;
                    }
                    pu8_var1 = (i_var4 + 4);
                    unsafe {
                        *pu8_var1 = *pu8_var1 | 1;
                    }
                    GetClientRect16(CONCAT22(unaff_ss, &stack0xfff6), param_3);
                    if (param_2_00 < (local_4 >> 1)) {
                        pu8_var1 = (i_var4 + 4);
                        unsafe {
                            *pu8_var1 = *pu8_var1 | 4;
                        }
                    } else {
                        pu8_var1 = (i_var4 + 4);
                        unsafe {
                            *pu8_var1 = *pu8_var1 | 8;
                        }
                    }
                    send_win_msg_1008_84ba(param_3, i_var4, u_var7);
                    SetTimer16(0, 0, 300, 0xfa6);
                    InvalidateRect16(1, 0x0, 0);
                    UpdateWindow16(param_3);
                    SetCapture16(param_3);
                }
            }
        }
    }
    return;
}

pub fn win_fn_1008_8214(param_1: HWND16, param_2: i32, param_3: u16, param_4: u32) -> u16 {
    let mut in_ax: i32;
    let i_var1: u16;
    let in_dx: &mut  Struct199;
    let mut u_var2: u32;
    let pu_var3: &mut  u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0x81) {
        process_struct_1000_179c(6, in_dx);
        if ((in_dx | in_ax) == 0) {
            u_var2 = 0;
        } else {
            u_var2 = clear_list_elements_1008_80d2(CONCAT22(in_dx, in_ax));
        }
        SetWindowLong16(u_var2, (u_var2 >> 0x10));
    }
    if (param_2 == 1) {
        pu_var3 = GetWindowLong16(0, param_1);
        unsafe {
            *pu_var3 = (param_3 + 8);
        }
        i_var1 = GetDlgCtrlID16(param_1);
        (pu_var3 + 2) = i_var1;
    }
    return 1;
}

pub unsafe fn win_fn_1008_5f44(param_1: u16, param_2: u32, param_3: u32) -> LRESULT {
    let wVar1: u16;
    let LVar2: LRESULT;
    let paVar3: &mut  Struct219;
    let mut in_stack_0000fff8: u16;

    if (param_3 == 2) {
        wVar1 = GetWindowWord16(0, param_3);
        mci_send_command_1008_5cb6(
            ctx.g_struct_1050_02a0,
            (ctx.g_struct_1050_02a0 >> 0x10),
            wVar1,
        );
        paVar3 = process_struct_1010_20ba(
            ctx.g_struct_var_1050_0ed0,
            CONCAT22(in_stack_0000fff8, 0x37),
        );
        pass1_1008_aa28(paVar3, paVar3);
    } else {
        if (param_3 != 0x3b9) {
            LVar2 = DefWindowProc16(
                CONCAT22(param_2, param_1),
                (param_2 >> 0x10),
                param_3,
                param_3,
            );
            return LVar2;
        }
        DestroyWindow16(param_3);
    }
    return 0;
}

pub unsafe fn win_fn_1008_3bd6(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
    param_5: u32,
    param_6: u32,
    param_7: u32,
) {
    make_proc_inst_1040_8fb8(
        param_1,
        param_3,
        0,
        param_5,
        param_5,
        param_6,
        param_6,
        param_7,
    );
    CONCAT22(param_2, param_1) = 0x3cfc;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    (param_1 + 0x36) = 0;
    (param_1 + 0x26) = 0;
    process_struct_1040_9252(CONCAT22(param_2, param_1));
    window::create_win_1040_92dc(CONCAT22(param_2, param_1));
    window::update_window_1040_93aa(CONCAT22(param_2, param_1), param_3_00);
    return;
}

pub fn win_func_1008_3c34(param_1: u32, param_2: u8, param_3: HDC16) {
    let pp_var1: fn();
    let h_gdi_obj: HPALETTE16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u32;

  // u_var3 = (param_1  >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 10) | (i_var2 + 8)) != 0) {
        local_6 = (i_var2 + 8);
        if (((i_var2 + 0xc) != 0) && ((param_2 & 1) != 0)) {
            local_6 = (i_var2 + 0xc);
        }
        if (((i_var2 + 0x10) != 0) && ((param_2 & 4) != 0)) {
            local_6 = (i_var2 + 0x10);
        }
      // u_var3 = (_PTR_LOOP_1050_4230  >> 0x10);
        local_a = (_PTR_LOOP_1050_4230 + 0xe);
        local_c = &param_3;
        realize_palette_1008_4e08(local_a, (_PTR_LOOP_1050_4230 + 0x10), local_c, unaff_ss);
        pp_var1 = (local_6 + 4);
        (**pp_var1)();
        h_gdi_obj = SelectPalette16(0, local_c, param_3);
        DeleteObject16(h_gdi_obj);
    }
    return;
}

pub unsafe fn win_fn_1008_016e(ctx: &mut AppContext, param_1: u32) {
    let pp_var1: fn();
    let p_uvar2: &mut  u16;
    let mut cVar3: u8;
    let mut in_ax: i32;
    let pu_var4: Vec<u8>;
    let pu_var5: Vec<u8>;
    let mut u_var6: u32;
    let mut u_var7: u32;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let extraout_var_01: u32;
    let extraout_var_02: u32;
    let extraout_var_03: u32;
    let struct_a: &mut  Struct199;
    let mut u_var8: i32;


    let paVar9: &mut  Struct199;

    let pa_var10: &mut  Struct199;


    let extraout_dx_04: &mut  Struct199;
    let extraout_dx_05: &mut  Struct199;
    let mut extraout_dx_06: u16;
    let mut u_var11: u16;
    let mut i_var12: i32;
    let mut u_var13: u16;
    let mut unaff_ss: u16;
    let u_var14: u8;
    let u_var15: u8;
    let mut in_stack_0000fe46: u16;
    let mut local_13e: [u8; 172];
    let mut local_92: [u8; 128];
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    GetVersion16();
    local_8 = in_ax & 0xff;
    u_var6 = local_8;
    local_a = in_ax >> 8;
    local_4 = struct_a;
    if ((local_8 < 3) || ((paVar9 = struct_a, local_8 == 3 && (local_a < 10)))) {
        u_var13 = 0x1000;
        local_10 = struct_a;
        process_struct_1000_179c(0xb4, struct_a);
        local_12 = u_var6;
        u_var8 = local_10 | local_12;
        if (u_var8 == 0) {
            u_var6 = 0;
            u_var8 = 0;
        } else {
            u_var13 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            mixed_1040_8520(
                (u_var6 & 0xffff | local_10 << 0x10),
                0,
                0x10,
                2,
                0x5de,
                0x5dd,
            );
        }
        _local_e = (u_var6 & 0xffff | u_var8 << 0x10);
        pp_var1 = (*_local_e + 0x74);
        (**pp_var1)(u_var13, u_var6, u_var8);
        paVar9 = ctx.dx_reg;
        call_fn_ptr_1000_24cd(1);
    }
    fn_1008_6048(s_version__d__d_1050_0012, paVar9, SUB41(u_var6, 0));
    if ((local_8 == 3) && (0xb < local_a)) {
        ctx.PTR_LOOP_1050_0010 = (&ctx.PTR_LOOP_1050_0000 + 1);
    }
    LoadString16(
        0x80,
        CONCAT22(unaff_ss, local_92),
        0x578,
        ctx.g_h_instance_1050_038c,
    );
    pu_var4 = local_92;
    dos3_call_1000_51aa(pu_var4);
    if (pu_var4 != 0x0) {
        u_var14 = unaff_ss;
        u_var15 = (unaff_ss >> 8);
        LoadString16(
            0x80,
            CONCAT13(u_var15, CONCAT12(u_var14, local_13e)),
            0x57b,
            ctx.g_h_instance_1050_038c,
        );
        LoadString16(
            0x80,
            CONCAT13(u_var15, CONCAT12(u_var14, &stack0xfe42)),
            0x62e,
            ctx.g_h_instance_1050_038c,
        );
        pu_var4 = MessageBox16(
            0x10,
            CONCAT13(u_var15, CONCAT12(u_var14, local_13e)),
            CONCAT22(unaff_ss, &stack0xfe42),
            0,
        );
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(4, paVar9);
    if ((paVar9 | pu_var4) == 0) {
        pu_var5 = 0x0;
        pa_var10 = 0x0;
        local_12 = pu_var4;
        local_10 = paVar9;
    } else {
        pu_var5 = pu_var4;
        local_12 = pu_var4;
        local_10 = paVar9;
        zero_array_val_1008_5394(CONCAT22(paVar9, pu_var4));
        pa_var10 = ctx.dx_reg;
    }
  // u_var13 = (param_1  >> 0x10);
    i_var12 = param_1;
    (i_var12 + 8) = pu_var5;
    (i_var12 + 10) = pa_var10;
    u_var7 = (i_var12 + 8);
    pu_var2 = (i_var12 + 8);
    ctx._PTR_LOOP_1050_0298 = u_var7;
    *pu_var2 = 0x70;
    (pu_var2 + 2) = offset;
    process_struct_1000_179c(0x126, pa_var10);
    u_var8 = u_var7;
    paVar9 = (pa_var10 | u_var8);
    local_12 = u_var8;
    local_10 = pa_var10;
    if (paVar9 != 0x0) {
        pass1_1010_2024();
        paVar9 = ctx.dx_reg;
    }
    if (ctx.g_struct_var_1050_0ed0 == 0) {
        cVar3 = fn_1008_6048(s_New_failed_in_Op__Op_1050_0020, paVar9, SUB21(u_var8, 0));
        u_var8 = CONCAT31(extraout_var, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(0xe8c, paVar9);
    pa_var10 = (paVar9 | u_var8);
    local_12 = u_var8;
    local_10 = paVar9;
    if (pa_var10 != 0x0) {
        pass1_1010_7e40(CONCAT22(paVar9, u_var8));
        pa_var10 = ctx.dx_reg;
    }
    if (ctx.g_struct_73_1050_14cc == 0) {
        cVar3 = fn_1008_6048(
            s_New_failed_in_Op__Op__ResLibr_1050_0035,
            pa_var10,
            SUB21(u_var8, 0),
        );
        u_var8 = CONCAT31(extraout_var_00, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(0xb0, pa_var10);
    paVar9 = (pa_var10 | u_var8);
    local_12 = u_var8;
    local_10 = pa_var10;
    if (paVar9 != 0x0) {
        pass1_1038_aeca();
        paVar9 = ctx.dx_reg;
    }
    if (ctx.g_struct_112_001 == 0) {
        cVar3 = fn_1008_6048(
            s_New_failed_in_Op__Op__DialogCtr_1050_0053,
            paVar9,
            SUB21(u_var8, 0),
        );
        u_var8 = CONCAT31(extraout_var_01, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(10, paVar9);
    pa_var10 = (paVar9 | u_var8);
    local_12 = u_var8;
    local_10 = paVar9;
    if (pa_var10 != 0x0) {
        make_proc_inst_1038_cf6c();
        pa_var10 = extraout_dx_04;
    }
    if (ctx._PTR_LOOP_1050_1040 == 0) {
        cVar3 = fn_1008_6048(
            s_New_failed_in_Op__Op__DialogHand_1050_0073,
            pa_var10,
            SUB21(u_var8, 0),
        );
        u_var8 = CONCAT31(extraout_var_02, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(0x14, pa_var10);
    paVar9 = (pa_var10 | u_var8);
    local_12 = u_var8;
    local_10 = pa_var10;
    if (paVar9 != 0x0) {
        modify_u16_list_1008_5bdc(CONCAT22(pa_var10, u_var8));
        paVar9 = extraout_dx_05;
    }
    if ctx.g_struct_1050_02a0 == 0 {
        cVar3 = fn_1008_6048(
            s_New_failed_in_Op__Op__Simulator_1050_0097,
            paVar9,
            SUB21(u_var8, 0),
        );
        u_var8 = CONCAT31(extraout_var_03, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(0xfc, paVar9);
    local_10 = paVar9;
    local_12 = u_var8;
    if ((paVar9 | u_var8) == 0) {
        u_var8 = 0;
        u_var11 = 0;
    } else {
        win_fn_1008_0536();
        u_var11 = extraout_dx_06;
    }
    (i_var12 + 4) = u_var8;
    *(i_var12 + 6) = u_var11;
    if ((i_var12 + 4) == 0) {
        fn_1008_6048(s_New_failed_in_Op__Op_1050_00b7, u_var11, SUB21(u_var8, 0));
        call_fn_ptr_1000_24cd(1);
    }
    reg_class_1008_96d2((i_var12 + 4), in_stack_0000fe46);
    pp_var1 = ((i_var12 + 4) + 8);
    (**pp_var1)(0x1000);
    u_var7 = (i_var12 + 4);
    ctx.g_h_window = (u_var7 + 8);
    u_var7 = (i_var12 + 4);
    pp_var1 = ((i_var12 + 4) + 0xc);
    (**pp_var1)(0x1000, u_var7, (u_var7 >> 0x10), 3);
    u_var7 = (i_var12 + 4);
    UpdateWindow16((u_var7 + 8));
    return;
}

pub unsafe fn win_fn_1008_0536(param_1: &mut  Struct180) {
    let mut u_var1: u16;
    let HVar2: HCURSOR16;
    let HVar3: HGDIOBJ16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let ppVar6: &mut  Struct2551;
    let HVar7: HINSTANCE16;

    process_struct_1008_3ab8(param_1);
  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
    u_var1 = 0;
    (i_var4 + 0xe0) = 0;
    (i_var4 + 0xe4) = 0;
    (i_var4 + 0xe8) = 0;
    (i_var4 + 0xec) = 0;
    (i_var4 + 0xee) = 0;
    (i_var4 + 0xf2) = 0;
    (i_var4 + 0xf4) = 0;
    (i_var4 + 0xf8) = 0;
    param_1 = s_0_1050_389e;
    (i_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
    (i_var4 + 200) = (s_572_bmp_1050_2007 + 1);
    (i_var4 + 0xac) = 0;
    (i_var4 + 0xae) = 0x8700;
    HVar7 = ctx.g_h_instance_1050_038c;
    LoadIcon16();
    (i_var4 + 0xc2) = u_var1;
    HVar2 = LoadCursor16(0x7f00, 0);
    (i_var4 + 0xc4) = HVar2;
    HVar3 = GetStockObject16(4);
    (i_var4 + 0xc6) = HVar3;
    process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(HVar7, 0x48));
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (i_var4 + 10)), s_Outpost_1050_00d7);
    ppVar6 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(HVar7, 0x32));
    (i_var4 + 0xf4) = ppVar6;
    (i_var4 + 0xf6) = (ppVar6 >> 0x10);
    color::sys_color_func_1008_357e(i_var4, u_var5, 1);
    return;
}

pub unsafe fn win_fn_1008_12dc(param_1: u32, param_2: u32) {
    let pu_var1: Vec<u8>;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let h_var4: HCURSOR16;

    let mut hwnd: i32;



    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 6];
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = LoadCursor16(0x7f02, 0);
    local_6 = SetCursor16(local_4);
    pass1_1008_6d8a(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, local_c)),
        param_2,
    );
    pu_var1 = local_c;
    write_to_file_1008_6e02(pu_var1, unaff_ss);
  // u_var5 = (param_1  >> 0x10);
    if (pu_var1 == 0x0) {
        SetCursor16(local_6);
        local_28._0_2_ = ctx.g_struct_73_1050_14cc;
        local_28 = (ctx.g_struct_73_1050_14cc >> 0x10);
        u_var2 = ctx.g_u16_1050_0310;
        load_string_1010_847e(local_28, local_28, ctx.g_u16_1050_0310);
        hwnd = ctx.dx_reg;
        pass1_fn_1008_60e8(u_var2, ctx.dx_reg, 0);
        _local_10 = CONCAT22(hwnd, u_var2);
        local_28._0_2_ = ctx.g_struct_73_1050_14cc;
        local_28 = (ctx.g_struct_73_1050_14cc >> 0x10);
        u_var3 = u_var2;
        load_string_1010_847e(local_28, local_28, 0x57b);
        MessageBeep16(0x10);
        MessageBox16(
            0x10,
            CONCAT22(ctx.dx_reg, u_var3),
            CONCAT22(hwnd, u_var2),
            (param_1 + 8),
        );
    } else {
        (ctx._g_bool_1050_5748 + 8) = 0;
        h_var4 = SetCursor16(local_6);
        local_28._0_2_ = ctx.g_struct_73_1050_14cc;
        local_28 = (ctx.g_struct_73_1050_14cc >> 0x10);
        load_string_1010_847e(local_28, local_28, 0x6d3);
        pass1_fn_1008_60e8(h_var4, ctx.dx_reg, ctx.dx_reg);
        local_28._0_2_ = ctx.g_struct_73_1050_14cc;
        local_28 = (ctx.g_struct_73_1050_14cc >> 0x10);
        load_string_1010_847e(local_28, local_28, 0x57b);
        MessageBeep16(0);
        hwnd = (param_1 + 8);
        MessageBox16(0x40, CONCAT22(ctx.dx_reg, h_var4), (hwnd << 0x10), hwnd);
        _local_10 = CONCAT22(hwnd, hwnd);
    }
    error_check_1000_17ce((_local_10 & 0xffff | hwnd << 0x10));
    close_file_1008_6dd0(CONCAT22(unaff_ss, local_c));
    return;
}

pub unsafe fn win_fn_1008_1414(ctx: &mut AppContext, param_1: u32, param_2: &String) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let b_result: bool;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let ppVar6: &mut  pass1_struct_2;
    // let mut i_var7: i32;
    let mut u_var8: u16;
    let mut unaff_DI: u16;
    let mut unaff_ss: u16;
    let pp_var9: &mut  Struct2551;
    // let mut u_var10: u16;
    let u_var11: u8;
    let u_var12: u8;
    let mut local_50: u32;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut uStack38: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: FileObject = FileObject::new();

    pass1_1008_6d8a(ctx, &mut local_8, param_2);
    b_result = read_from_file_1008_6e78(ctx, &mut local_8);
    // i_var7 = param_1;
  // u_var10 = (param_1  >> 0x10);
    if b_result == 0 {
        if ctx.g_u16_1050_0310 == 0 {
            ctx.g_u16_1050_0310 = 0x6d4;
        }
        u_var4 = ctx.g_u16_1050_0310;
        load_string_1010_847e(
            ctx.g_struct_73_1050_14cc,
            ctx.g_u16_1050_0310,
        );
        u_var8 = ctx.dx_reg;
        pass1_fn_1008_60e8(u_var4, ctx.dx_reg);
        u_var5 = u_var4;
        load_string_1010_847e(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x57b,
        );
        MessageBeep16(0x10);
        MessageBox16(
            0x10,
            CONCAT22(ctx.dx_reg, u_var5),
            CONCAT22(u_var8, u_var4),
            (param_1 + 8),
        );
        error_check_1000_17ce(CONCAT22(u_var8, u_var4));
        call_fn_ptr_1000_24cd(1);
    }
    cursor::set_cursor_1008_2dcc(param_1, param_1, 8);
    _local_c = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_DI, 0x2f));
  // u_var8 = (_local_c  >> 0x10);
    local_10 = (_local_c + 0x20);
    ppVar6 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        param_1,
    );
    _local_14 = CONCAT22(u_var8, ppVar6);
    local_18 = &ppVar6.field_0x10;
    u_var1 = (param_1 + 0xe8);
    ppc_var2 = ((param_1 + 0xe8) + 4);
    ppc_var2(
        0x1030,
        u_var1,
        (u_var1 >> 0x10),
        local_10,
        (local_10 >> 0x10),
        (local_18 + 2) + -1,
        2,
    );
    local_22 = ctx.dx_reg;
    ppVar6 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x4000001,
    );
    _local_1c = CONCAT22(local_22, ppVar6);
    local_20 = &ppVar6.field_0x10;
    local_24 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        local_20,
    );
    local_2a = (local_24 + 0xc);
    uStack38 = (local_24 + 0x10);
    i_var7 = pass1_1030_5b00(_local_14);
    u_var11 = SUB21(&local_2a, 0);
    u_var12 = (&local_2a >> 8);
    u_var10 = unaff_ss;
    pp_var9 = process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT13(u_var12, CONCAT12(u_var11, param_1)),
    );
    pass1_1018_179e(pp_var9, CONCAT22(u_var10, CONCAT11(u_var12, u_var11)));
    u_var11 = 0;
    u_var12 = 4;
    u_var5 = 0x1b;
    param_1 = 1;
    pp_var9 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x1002b);
    pass1_1010_043a(
        pp_var9,
        CONCAT13(u_var12, CONCAT12(u_var11, u_var10)),
        u_var5,
    );
    close_file_1008_6dd0(CONCAT22(unaff_ss, local_8));
    return;
}

pub unsafe fn win_fn_1008_2b54(param_1: u32) -> u16 {
    let pp_var1: fn();
    let pc_var2: String;
    let mut in_ax: i32;
    let mut u_var3: u16;
    let in_dx: &mut  Struct199;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_aa: u16;
    let mut local_a8: u16;
    let mut local_a6: u16;
    let mut local_a4: u16;
    let mut local_56: [u8; 80];
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    if (_PTR_LOOP_1050_4230 == 0) {
        pc_var2 = load_string_1010_847e(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x5f2,
        );
        copy_string_1000_3d3e(CONCAT22(unaff_ss, local_56), pc_var2);
        pc_var2 = load_string_1010_847e(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x57b,
        );
        copy_string_1000_3d3e(CONCAT22(unaff_ss, &local_a6), pc_var2);
        local_4 = MessageBox16(
            0x21,
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_a6)),
            CONCAT22(unaff_ss, local_56),
            ctx.g_h_window,
        );
    } else {
        u_var5 = 0x1000;
        process_struct_1000_179c(0xb4, in_dx);
        u_var4 = in_dx | in_ax;
        if (u_var4 == 0) {
            u_var3 = 0;
            u_var4 = 0;
        } else {
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            u_var3 = mixed_1040_8520(
                CONCAT22(in_dx, in_ax),
                ctx.g_h_window,
                0x21,
                2,
                0x57b,
                0x5f2,
            );
        }
        _local_a6 = CONCAT22(u_var4, u_var3);
        pp_var1 = (*_local_a6 + 0x74);
        local_4 = (**pp_var1)(u_var5, u_var3, u_var4, in_ax);
    }
    local_6 = local_4;
    if (local_4 != 1) {
        local_6 = 0;
    }
    if (((local_6 != 0) && (ctx._g_bool_1050_5748 != 0))
        && (
            u_var4 = (ctx._g_bool_1050_5748 + 8),
            _local_a6 = (_local_a6 & 0xffff0000 | u_var4),
            u_var4 != 0,
        ))
    {
        PostMessage16(0, 0xb4, 0x111, (param_1 + 8));
        local_6 = 0;
    }
    return local_6;
}

pub fn win_fn_1008_2d22(param_1: u32) {
    let pi_var1: &mut  i32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let mut u_var7: u16;
    let mut u_var8: u32;

  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
    if (((i_var4 + 0xee) != 0)
        && (
            pi_var1 = (i_var4 + 0xf2),
            unsafe { *pi_var1 = *pi_var1 + -1 },
            (i_var4 + 0xf2) < 1,
        ))
    {
        u_var8 = (i_var4 + 0xee);
        ppc_var3 = ((i_var4 + 0xee) + 0x90);
        (**ppc_var3)();
        (i_var4 + 0xee) = 0;
        (i_var4 + 0xf2) = 0;
        if ((i_var4 + 0xe8) != 0) {
            u_var7 = 3;
            u_var6 = (i_var4 + 0xe8);
            ppc_var3 = ((i_var4 + 0xe8) + 0xc);
            (**ppc_var3)();
            window::show_win_1038_b68a(ctx.g_struct_112_001, (ctx.g_struct_112_001 >> 0x10));
            u_var2 = (i_var4 + 0xf4);
            window::show_window_1010_7ace(u_var2, (u_var2 >> 0x10));
            u_var2 = (i_var4 + 0xe8);
            ppc_var3 = ((i_var4 + 0xe8) + 0x98);
            (**ppc_var3)(0x1010, u_var2, (u_var2 >> 0x10), 1, u_var6, u_var7, u_var8);
            PostMessage16(0, 0xfc, 0x111, ctx.g_h_window);
        }
    }
    return;
}

pub unsafe fn win_fn_1008_3018(param_1: u32) {
    let string_b: String;
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut str_index: u16;
    let mut local_DX_17: i32;
    let mut unaff_si: u16;
    let mut local_114: u16;
    let mut local_112: u32;
    let mut local_10a: u32;
    let mut local_106: u32;
    let mut local_str: [u8; 256];

    local_str[0] = '\0';
    local_106 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 2));
  // u_var1 = (local_106  >> 0x10);
    i_var2 = local_106;
    string_b = *(i_var2 + 0x12);
    local_10a._0_2_ = string_b;
    if (((i_var2 + 0x14) | local_10a) == 0) {
        file_menu::open_save_1008_30cc(param_1);
    } else {
        copy_string_1000_3d3e(CONCAT22(ctx.stack_seg_reg, local_str), *(i_var2 + 0x1a));
        str_index = get_string_index_1000_3da4(CONCAT22(ctx.stack_seg_reg, local_str));
        if (local_str[str_index - 1] != '\\') {
            local_str[str_index] = '\\';
            local_str[str_index + 1] = '\0';
        }
        process_string_1000_3cea(CONCAT22(ctx.stack_seg_reg, local_str), string_b);
        if (local_str[0] != '\0') {
            win_fn_1008_12dc(param_1, local_str, ctx.stack_seg_reg);
            return;
        }
    }
    return;
}

pub fn win_gui_fn_1040_a2cc(param_1: &mut  Struct124, param_2: u32, param_3: u32) {
    let mut u_var1: i32;
    let mut u_var2: u32;

    if (param_3 == 0x1826) {
        if ((param_3 == 1) || (1 < param_3 - 1 && (param_3 - 3 < 2))) {
            u_var1 = 1;
        } else {
            u_var1 = 0;
        }
        return u_var1;
    }
  // u_var2 = window::win_gui_fn_1040_b54a(param_1, param_2, (param_2  >> 0x10), param_3);
    return u_var2;
}

pub unsafe fn win_fn_1040_9ce0(param_1: i32, param_2: u16, param_4: HWND16, param_3: u32) {
    let pu8_var1: Vec<u8>;
    let mut i_var2: i32;
    let mut id: u16;
    let mut i_var3: i32;
    let mut u8_var4: u8;

    let WVar5: WPARAM16;
    let b_var6: bool;
    let mut offset: i32;
    let struct_a: &mut  Struct199;

    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let in_struct_1: &mut Struct44;
    let pWVar8: &mut  WPARAM16;
    let LVar9: LRESULT;
    let mut u_var10: u32;
    let mut u_var11: u16;
    let h_var12: HWND16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let local_a: RECT16;

    h_var12 = &ctx.g_alloc_addr_1050_1050;
    in_struct_1 = GetWindowLong16(0, param_3);
  // struct_a = (in_struct_1  >> 0x10);
    i_var3 = in_struct_1;
  // u_var7 = ((in_struct_1 & 0xffff0000)  >> 0x10);
    if (param_3 == 0x30) {
        (i_var3 + 0x5a) = param_2;
    } else {
        if (param_3 < 0x31) {
            if (param_3 == 0x1f) {
                (i_var3 + 4) = 0;
                ReleaseCapture16(h_var12);
                return;
            }
            if (0x1f < param_3) {}
            // goto LAB_1040_a1ae;
            u8_var4 = param_3;
            if (u8_var4 == 8) {
                pu8_var1 = (i_var3 + 4);
                unsafe {
                    *pu8_var1 = *pu8_var1 & 0xf7;
                }
                local_1e._0_1_ = false;
                b_var6 = IsWindow16(param_2);
                if (b_var6 != 0) {
                    u_var10 = SendMessage16(0, 0, 0x87, param_2);
                    local_1e._0_1_ = (u_var10 & 0x20) == 0;
                }
                (i_var3 + 0x56) = 0;
                SendMessage16(0, (i_var3 + 0x5c), 0x401, (i_var3 + 2));
                if (((i_var3 + 0x5c) != 0) && ((i_var3 + 0x5c) != in_struct_1.ptr_a_lo)) {
                    SendDlgItemMessage16(local_1e, 1, 0x404, (i_var3 + 0x5c), (i_var3 + 2));
                }
                (i_var3 + 0x5c) = 0;
            } else {
                if (u8_var4 < 9) {
                    if (u8_var4 == 1) {
                        pWVar8 = GetWindowLong16(0, param_3);
                        i_var3 = pWVar8;
                      // u_var7 = ((pWVar8 & 0xffff0000)  >> 0x10);
                        (i_var3 + 2) = (param_1 + 8);
                        WVar5 = GetDlgCtrlID16(param_3);
                        unsafe {
                            *pWVar8 = WVar5;
                        }
                        (i_var3 + 0x56) = (param_1 + 0x12);
                        copy_string_1000_3d3e(
                            (pWVar8 & 0xffff0000 | (i_var3 + 6)),
                            *(param_1 + 0x16),
                        );
                        if ((*(param_1 + 0x12) & 1) != 0) {
                            SendMessage16(0, unsafe { *pWVar8 }, 0x401, (param_1 + 8));
                        }
                        if (((param_1 + 0x14) & 0x800) == 0) {
                            return;
                        }
                        pu8_var1 = (i_var3 + 4);
                        unsafe {
                            *pu8_var1 = *pu8_var1 | 4;
                        }
                        return;
                    }
                    if (u8_var4 == 2) {
                        error_check_1000_17ce(in_struct_1);
                        SetWindowLong16(0, 0);
                        return;
                    }
                    if (u8_var4 != 7) {}
                    // goto LAB_1040_a1ae;
                    pu8_var1 = (i_var3 + 4);
                    unsafe {
                        *pu8_var1 = *pu8_var1 | 8;
                    }
                    LVar9 = SendMessage16(0, 0, 0x400, (i_var3 + 2));
                    id = LVar9;
                    if (((LVar9 >> 0x10) == 0x534b)
                        && ((i_var3 + 0x5c) = id, id != in_struct_1.ptr_a_lo))
                    {
                        SendDlgItemMessage16(1, 0, 0x404, id, (i_var3 + 2));
                    }
                    SendMessage16(0, in_struct_1.ptr_a_lo, 0x401, (i_var3 + 2));
                    (i_var3 + 0x56) = 1;
                } else {
                    if (u8_var4 == 10) {
                        pu8_var1 = (i_var3 + 4);
                        unsafe {
                            *pu8_var1 = *pu8_var1 & 0xfb;
                        }
                        if (param_2 == 0) {
                            pu8_var1 = (i_var3 + 4);
                            unsafe {
                                *pu8_var1 = *pu8_var1 | 4;
                            }
                        }
                    } else {
                        if (u8_var4 != 0xc) {
                            if (u8_var4 == 0xf) {
                                draw_1040_9948(param_3, i_var3, u_var7);
                                return;
                            }
                            // goto LAB_1040_a1ae;
                        }
                        if (CONCAT22(param_2_00, param_1) != 0) {
                            copy_string_1000_3d3e(
                                (in_struct_1 & 0xffff0000 | (i_var3 + 6)),
                                CONCAT22(param_2_00, param_1),
                            );
                        }
                    }
                }
            }
            // goto LAB_1040_9e20;
        }
        if (param_3 == 0x200) {
            if ((*(i_var3 + 4) & 1) == 0) {
                return;
            }
            GetClientRect16(CONCAT22(unaff_ss, &local_a), param_3);
            i_var2 = (i_var3 + 4);
            b_var6 = PtInRect16(CONCAT22(param_2_00, param_1), &local_a);
            if (b_var6 == 0) {
                pu8_var1 = (i_var3 + 4);
                unsafe {
                    *pu8_var1 = *pu8_var1 & 0xfd;
                }
            } else {
                pu8_var1 = (i_var3 + 4);
                unsafe {
                    *pu8_var1 = *pu8_var1 | 2;
                }
            }
            param_1 = (i_var3 + 4) - i_var2;
        } else {
            if (param_3 < 0x201) {
                offset = param_3 - 0x81;
                if (offset == 0) {
                    process_struct_1000_179c(0x5e, struct_a);
                    if ((struct_a | offset) == 0) {
                        offset = 0;
                        h_var12 = 0;
                    } else {
                        process_struct_1040_9824(CONCAT22(struct_a, offset));
                        h_var12 = ctx.dx_reg;
                    }
                    SetWindowLong16(offset, h_var12);
                    return;
                }
                if (param_3 == 0x87) {
                    return;
                }
                h_var12 = param_3 - 0x100;
                if (h_var12 == 0) {
                    if ((param_2 == 0x26) || (param_2 == 0x25)) {
                        u_var7 = (i_var3 + 2);
                        u_var11 = 1;
                    } else {
                        if ((param_2 != 0x28) && (param_2 != 0x27)) {
                            if (((param_2 == 0x20) || (param_2 == 0xd))
                                && (&PTR_LOOP_1050_5ed8 == 0))
                            {
                                &PTR_LOOP_1050_5ed8 = 1;
                                pu8_var1 = (i_var3 + 4);
                                unsafe {
                                    *pu8_var1 = *pu8_var1 | 2;
                                }
                                // goto LAB_1040_9e20;
                            }
                            // goto LAB_1040_a1ae;
                        }
                        u_var7 = (i_var3 + 2);
                        u_var11 = 0;
                    }
                    GetNextDlgTabItem16(::offset, u_var11, param_3, u_var7);
                    SetFocus16(h_var12);
                    return;
                }
                if ((param_3 == 0x101) && (&PTR_LOOP_1050_5ed8 != 0)) {
                    &PTR_LOOP_1050_5ed8 = 0;
                    pu8_var1 = (i_var3 + 4);
                    unsafe {
                        *pu8_var1 = *pu8_var1 & 0xfd;
                    }
                    InvalidateRect16(1, 0x0, 0);
                    UpdateWindow16(param_3);
                    SendMessage16(0, in_struct_1.ptr_a_lo, 0x111, (i_var3 + 2));
                    return;
                }
                // LAB_1040_a1ae:
                DefWindowProc16(
                    CONCAT22(param_2_00, param_1),
                    param_2,
                    param_3,
                    param_3,
                );
                return;
            }
            if (param_3 == 0x201) {
                // LAB_1040_9e74:
                SetFocus16(param_3);
                pu8_var1 = (i_var3 + 4);
                unsafe {
                    *pu8_var1 = *pu8_var1 | 3;
                }
                InvalidateRect16(1, 0x0, 0);
                UpdateWindow16(param_3);
                SetCapture16(param_3);
                return;
            }
            if (param_3 == 0x202) {
                ReleaseCapture16(h_var12);
                GetClientRect16(CONCAT22(unaff_ss, &local_a), param_3);
                if ((*(i_var3 + 4) & 1) == 0) {
                    return;
                }
                pu8_var1 = (i_var3 + 4);
                unsafe {
                    *pu8_var1 = *pu8_var1 & 0xfc;
                }
                InvalidateRect16(1, 0x0, 0);
                UpdateWindow16(param_3);
                local_1e._0_1_ = param_2_00;
                local_1e = (param_2_00 >> 8);
                b_var6 = PtInRect16(
                    CONCAT13(local_1e, CONCAT12(local_1e, param_1)),
                    &local_a,
                );
                if (b_var6 == 0) {
                    return;
                }
                PostMessage16(0, in_struct_1.ptr_a_lo, 0x111, (i_var3 + 2));
                return;
            }
            if (param_3 == 0x203) {}
            // goto LAB_1040_9e74;
            if (param_3 != 0x404) {}
            // goto LAB_1040_a1ae;
            if (param_2 == 1) {
                (i_var3 + 0x56) = 1;
            } else {
                (i_var3 + 0x56) = 0;
            }
        }
    }
    if (param_1 == 0) {
        return;
    }
    // LAB_1040_9e20:
    InvalidateRect16(1, 0x0, 0);
    UpdateWindow16(param_3);
    return;
}

pub fn win_fn_1040_8b92(param_1: u32) {
    let mut bVar1: u8;
    let mut u_var2: u16;
    let mut u_var3: u16;

  // u_var3 = (param_1  >> 0x10);
    bVar1 = *(param_1 + 0x98) & 0xf0;
    if ((((bVar1 == 0x30) || (bVar1 == 0x10)) || (bVar1 == 0x10))
        || ((bVar1 == 0x40 || (bVar1 == 0x40)) || (bVar1 == 0x20)))
    {
        u_var2 = LoadIcon16();
        (param_1 + 0x8e) = u_var2;
    }
    return;
}

pub unsafe fn win_fn_1040_89a4(param_1: &mut  u32, param_2: &mut  u16) {
    let mut u_var1: u16;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut u_var4: u16;
    let ppVar5: &mut  Struct2551;
    let mut in_stack_0000fff0: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_win_msg_1008_9510(&PTR_LOOP_1050_5df4, &ctx.g_alloc_addr_1050_1050);
    u_var3 = (param_2 + 2);
    unsafe {
        u_var1 = *param_2;
    }
    u_var4 = 0x1010;
    ppVar5 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(in_stack_0000fff0, 2));
    if ((ppVar5 + 0x72) != 0) {
        u_var4 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        u_var3 = mci_send_command_1008_5c7c(ctx.g_struct_1050_02a0, CONCAT22(u_var3, u_var1));
        (param_1 + 0x8c) = u_var3;
    }
    unsafe {
        ppc_var2 = (*param_1 + 0x74);
    }
    ppc_var2(u_var4, param_1);
    return;
}

pub unsafe fn win_fn_1040_8718(param_1: &mut  Struct20) -> Vec<u8> {
    let pi_var1: &mut  i32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let ppVar5: &mut  Struct2551;
    let mut u_var6: u16;
    let mut menu: u16;
    let u_var7: u8;
    let u_var8: u8;
    let u_var9: u8;
    let u_var10: u8;
    let mut u_var11: i32;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut local_4: u16;

    process_win_msg_1008_9510(&PTR_LOOP_1050_5df4, &ctx.g_alloc_addr_1050_1050);
    i_var3 = param_1;
  // u_var11 = (param_1  >> 0x10);
    win_gui_func_1040_78e2(param_1);
    PTR_LOOP_1050_5df6 = (i_var3 + 6);
    if ((i_var3 + 0x94) != 0) {
        copy_string_1000_3d3e(
            (param_1 & 0xff000000 | CONCAT12((param_1 >> 0x10), i_var3 + 0x10)),
            *(i_var3 + 0x94),
        );
    }
    get_sys_metrics_1040_8c66(i_var3, u_var11);
    local_4 = *(i_var3 + 0x98) & 0xf;
    if (local_4 == 1) {
        (i_var3 + 0xae) = ((i_var3 + 0xaa) + -0xc4) / 2;
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0xff,
            CONCAT22(unaff_ss, &local_104),
            0x592,
        );
        u_var2 = (i_var3 + 0xae);
        window::create_win_1040_8bea(
            (param_1 & 0xffff | u_var11 << 0x10),
            1,
            1,
            u_var2,
            (u_var2 >> 0x10),
            CONCAT22(unaff_ss, &local_104),
        );
        pi_var1 = (i_var3 + 0xae);
        unsafe {
            *pi_var1 = *pi_var1 + 0x6c;
        }
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0xff,
            CONCAT22(unaff_ss, &local_104),
            0x7d8,
        );
        u_var2 = (i_var3 + 0xae);
        u_var7 = u_var2;
        u_var8 = (u_var2 >> 8);
      // u_var9 = (u_var2  >> 0x10);
        u_var10 = (u_var2 >> 0x18);
        menu = 2;
    } else {
        if (local_4 != 4) {
            (i_var3 + 0xae) = ((i_var3 + 0xaa) + -0x58) / 2;
            load_string_1010_84e0(
                ctx.g_struct_73_1050_14cc,
                (ctx.g_struct_73_1050_14cc >> 0x10),
                0xff,
                CONCAT22(unaff_ss, &local_104),
                0x592,
            );
            u_var2 = (i_var3 + 0xae);
            u_var7 = u_var2;
            u_var8 = (u_var2 >> 8);
          // u_var9 = (u_var2  >> 0x10);
            u_var10 = (u_var2 >> 0x18);
            u_var6 = 1;
            menu = 1;
            // goto LAB_1040_88a5;
        }
        (i_var3 + 0xae) = ((i_var3 + 0xaa) + -0xc4) / 2;
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0xff,
            CONCAT22(unaff_ss, &local_104),
            0x650,
        );
        u_var2 = (i_var3 + 0xae);
        window::create_win_1040_8bea(
            (param_1 & 0xffff | u_var11 << 0x10),
            1,
            6,
            u_var2,
            (u_var2 >> 0x10),
            CONCAT22(unaff_ss, &local_104),
        );
        pi_var1 = (i_var3 + 0xae);
        unsafe {
            *pi_var1 = *pi_var1 + 0x6c;
        }
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0xff,
            CONCAT22(unaff_ss, &local_104),
            0x651,
        );
        u_var2 = (i_var3 + 0xae);
        u_var7 = u_var2;
        u_var8 = (u_var2 >> 8);
      // u_var9 = (u_var2  >> 0x10);
        u_var10 = (u_var2 >> 0x18);
        menu = 7;
    }
    u_var6 = 0;
    // LAB_1040_88a5:
    window::create_win_1040_8bea(
        (param_1 & 0xffff | u_var11 << 0x10),
        u_var6,
        menu,
        CONCAT11(u_var8, u_var7),
        CONCAT11(u_var10, u_var9),
        CONCAT22(unaff_ss, &local_104),
    );
    ppVar5 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_120, 0x48));
  // u_var4 = (ppVar5  >> 0x10);
    local_104 = (ppVar5 + 10);
    local_4 = (ppVar5 + 0xc);
    SetWindowPos16(
        0x40,
        (i_var3 + 0xac),
        (i_var3 + 0xaa),
        (local_4 - (i_var3 + 0xac)) / 2,
        (local_104 - (i_var3 + 0xaa)) / 2,
        0,
        (i_var3 + 6),
    );
    PTR_LOOP_1050_5df4 = (&ctx.PTR_LOOP_1050_0000 + 1);
    process_win_msg_1008_9510(&PTR_LOOP_1050_5df4, &ctx.g_alloc_addr_1050_1050);
    window::destroy_window_1040_8b7e(i_var3, u_var11);
    PTR_LOOP_1050_5df6 = 0x0;
    if ((i_var3 + 0xb2) != 0) {
        ppVar5 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_11e, 0x37));
        i_var3 = pass1_1008_ab54(ppVar5);
        if (i_var3 != 0) {
            PostMessage16(0, 0xfc, 0x111, ctx.g_h_window);
        }
    }
    return PTR_LOOP_1050_5df8;
}

pub fn mixed_1040_8520(
    param_1: &mut Struct103,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) -> &mut  Struct362 {
    let mut out_buffer: string;
    let paVar1: &mut  Struct362;
    let mut u_var2: u16;
    let pu_var3: &mut  u16;
    let mut u_var4: i32;
    let in_dx: &mut  Struct199;
    let mut unaff_cs: u16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u32;
    let in_string_1: String;
    let pcVar7: String;
    let mut uStack54: u16;
    let local_32: &mut  Struct362;
    let mut uStack48: u16;
    let uStack46: u8;
    let uStack45: u8;
    let uStack44: u8;
    let uStack43: u8;
    let paStack42: &mut  Struct73;
    let mut uStack40: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uStack40 = param_2;
    paStack42 = 0x0;
    uStack40._0_2_ = 0xfc3;
    uStack46 = 1;
    uStack45 = 0;
    uStack44 = 0;
    uStack43 = 0;
    local_32 = param_1;
    paVar1 = local_32;
  // uStack48 = (param_1  >> 0x10);
    u_var2 = uStack48;
    uStack54 = 0x853b;
    _local_32 = param_1;
    process_struct_1040_7728(param_1, (&ctx.PTR_LOOP_1050_0000 + 1), 0, 0xfc3, param_2);
    paVar1.field_0x8e = 0;
    paVar1.field_0x98 = param_3;
    paVar1.field_0x9a = 0;
    paVar1.field_0xb2 = 0;
    unsafe {
        *param_1 = 0x8ddc;
    }
    paVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    _local_a = 0;
    _local_6 = 300;
    paVar1.field_0x9e = 0;
    paVar1.field_0xa2 = 300;
    _local_e = CONCAT22(unaff_ss, &param_5);
    local_10 = param_4;
    if (param_4 != 0) {
        _local_e = CONCAT22(unaff_ss, &param_6);
        local_12 = param_5;
        uStack40 = param_5;
        paStack42 = ctx.g_struct_73_1050_14cc;
        uStack40._0_2_ = (ctx.g_struct_73_1050_14cc >> 0x10);
        uStack44 = unaff_cs;
        uStack43 = (unaff_cs >> 8);
        unaff_cs = 0x1010;
        uStack46 = 0xa7;
        uStack45 = 0x85;
        u_var6 = load_str_1010_84ac(paStack42, uStack40, param_5);
      // in_dx = (u_var6  >> 0x10);
        paVar1.field_0x94 = u_var6;
        paVar1.field_0x96 = in_dx;
        local_10 = local_10 - 1;
    }
    local_16 = 0;
    while (pu_var3 = _local_e, local_10 != 0) {
        _local_e = (_local_e & 0xffff0000 | (local_e + 2));
        unsafe {
            uStack40 = *pu_var3;
        }
        paStack42 = ctx.g_struct_73_1050_14cc;
        uStack40._0_2_ = (ctx.g_struct_73_1050_14cc >> 0x10);
        uStack44 = unaff_cs;
        uStack43 = (unaff_cs >> 8);
        uStack46 = 0xd6;
        uStack45 = 0x85;
        local_14 = uStack40;
        local_10 = local_10 - 1;
        in_string_1 = load_string_1010_847e(paStack42, uStack40, uStack40);
      // local_1a = (in_string_1  >> 0x10);
        paStack42 = 0x1010;
        unaff_cs = 0x1000;
        uStack44 = 0xe3;
        uStack43 = 0x85;
        pcVar7 = in_string_1;
        u_var4 = get_string_index_1000_3da4(in_string_1);
        uStack40 = (pcVar7 >> 0x10);
        local_1c = pcVar7;
      // in_dx = (in_string_1  >> 0x10);
        uStack40._0_2_ = in_string_1;
        local_16 = local_16 + u_var4;
    }
    uStack40 = (local_16 + 1);
    u_var5 = 0x1000;
    paStack42 = 0x85fd;
    uStack40._0_2_ = unaff_cs;
    local_10 = local_10 - 1;
    process_struct_1000_179c(uStack40, in_dx);
    paVar1.field_0x90 = uStack40;
    &paVar1.field_0x92 = in_dx;
    _local_e = CONCAT22(unaff_ss, &param_6);
    local_10 = param_4 - 1;
    if (local_10 != 0) {
        _local_e = CONCAT22(unaff_ss, &stack0x0012);
        local_14 = param_6;
        uStack40 = param_6;
        out_buffer = &paVar1.field_0x90;
        paStack42 = out_buffer;
        uStack40._0_2_ = (out_buffer >> 0x10);
        uStack44 = 0xff;
        uStack43 = 3;
        uStack48 = ctx.g_struct_73_1050_14cc;
      // uStack46 = (ctx.g_struct_73_1050_14cc  >> 0x10);
        uStack45 = (ctx.g_struct_73_1050_14cc >> 0x18);
        local_32 = &ctx.PTR_LOOP_1050_1000;
        u_var5 = 0x1010;
        load_string_1010_84e0(
            uStack48,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            out_buffer,
            param_6,
        );
        local_10 = local_10 - 1;
    }
    while (pu_var3 = _local_e, local_10 != 0) {
        _local_e = (_local_e & 0xffff0000 | (local_e + 2));
        unsafe {
            uStack40 = *pu_var3;
        }
        paStack42 = ctx.g_struct_73_1050_14cc;
        uStack40._0_2_ = (ctx.g_struct_73_1050_14cc >> 0x10);
        uStack44 = u_var5;
        uStack43 = (u_var5 >> 8);
        uStack46 = 0x5e;
        uStack45 = 0x86;
        local_14 = uStack40;
        local_10 = local_10 - 1;
        uStack40 = load_string_1010_847e(paStack42, uStack40, uStack40);
        pcVar7 = *&paVar1.field_0x90;
        uStack44 = SUB41(pcVar7, 0);
        uStack43 = (pcVar7 >> 8);
      // paStack42 = (pcVar7  >> 0x10);
        uStack46 = 0x10;
        uStack45 = 0x10;
        u_var5 = 0x1000;
        uStack48 = 0x8674;
        _local_20 = uStack40;
        process_string_1000_3cea(pcVar7, uStack40);
    }
    uStack44 = 0x8a;
    uStack43 = 0x86;
    paStack42 = u_var5;
    local_10 = local_10 - 1;
    uStack40 = param_1;
    win_fn_1040_8b92(param_1);
    PTR_LOOP_1050_5df8 = 0x0;
    return paVar1;
}

pub fn set_sys_modal_window_1040_81fe(param_1: u32) {
    SetSysModalWindow16((param_1 + 6));
    return;
}

pub unsafe fn win_fn_1040_800c(param_1: u32) {


    let local_bx_17: &mut  Struct6;
    let mut u_var1: u16;
    let mut iStack18: i32;
    let mut local_f: u8;
    let mut iStack16: i32;
    let mut w_command: u16;
    let lp_help_file: String;
    let mut h_wnd: u16;

    mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, 0x1f8);
  // u_var1 = (param_1  >> 0x10);
    local_bx_17 = param_1;
    if (local_bx_17.field_0x8a == 0) {
        h_wnd = local_bx_17.field_0x6;
        iStack16 = 0;
        w_command = 3;
        iStack18 = 0;
    } else {
        h_wnd = local_bx_17.field_0x6;
        w_command = 1;
        iStack18 = local_bx_17.field_0x8a;
        iStack16 = iStack18 >> 0xf;
    }
    _lp_help_file = CONCAT22(ctx.dx_reg, in_ax);
    WinHelp16(
        CONCAT22(iStack16, iStack18),
        w_command,
        _lp_help_file,
        h_wnd,
    );
    return;
}

pub fn win_gui_func_1040_78e2(param_1: &mut  Struct20) {
    let pp_var1: fn();
    let dlg_proc: DLGPROC16;
    let HVar2: HWND16;
    let local_bx_5: &mut  Struct32;
    let handle: HANDLE16;
    let mut u_var3: u16;
    let lVar4: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // handle = (param_1  >> 0x10);
    local_bx_5 = param_1;
    if (&local_bx_5.field_0xc == 0) {
      // u_var3 = (ctx._PTR_LOOP_1050_1040  >> 0x10);
        dlg_proc = (ctx._PTR_LOOP_1050_1040 + 4);
        HVar2 = (ctx._PTR_LOOP_1050_1040 + 6);
    } else {
        dlg_proc = local_bx_5.field_0xc;
        HVar2 = local_bx_5.field_0xe;
    }
    HVar2 = CreateDialog16(
        dlg_proc,
        HVar2,
        CONCAT22(local_bx_5.field_0xa, local_bx_5.h_instance),
        0,
    );
    local_bx_5.h_window = HVar2;
    GetWindowText16(0x50, param_1 & 0xffff0000 | ZEXT24(local_bx_5 + 1), HVar2);
    lVar4 = GetWindowLong16(-4, local_bx_5.h_window);
    SetWindowLong16(_PTR_LOOP_1050_5bcc, (_PTR_LOOP_1050_5bcc >> 0x10));
    SetProp16(local_bx_5, s_thisLo_1050_5dcd, local_bx_5.h_window);
    SetProp16(handle, s_thisHi_1050_5dd4, local_bx_5.h_window);
    local_a = lVar4;
    SetProp16(local_a, s_procLo_1050_5ddb, local_bx_5.h_window);
  // local_8 = (lVar4  >> 0x10);
    SetProp16(local_8, s_procHi_1050_5de2, local_bx_5.h_window);
    pp_var1 = (param_1 + 0x50);
    (**pp_var1)(offset, param_1);
    return;
}

pub fn win_fn_1040_748c(param_1: &mut  Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let unaff_ss: HWND16;
    let mut u_var3: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_2) {
        0xfa => {
            pp_var1 = ((param_1 + 1) + 0x18);
            (**pp_var1)()
        }
        _ => {
            window::win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
            return;
        }
        0xfd => {
            if (u16_1050_0ecc == 0) {
                return;
            }
            u16_1050_0ecc = 0;
            // goto LAB_1040_755d;
        }
        0xfe => {
            if (u16_1050_0ecc == 1) {
                return;
            }
            u16_1050_0ecc = 1;
            // goto LAB_1040_755d;
        }
        0xff => unsafe {
            if (u16_1050_0ecc == 2) {
                return;
            }
            u16_1050_0ecc = 2;
            // LAB_1040_755d:
            u_var2 = (param_1 + 1);
            pp_var1 = ((param_1 + 1) + 0x10);
            (**pp_var1)(0x40, u_var2, (u_var2 >> 0x10));
            u_var2 = (param_1 + 1);
            pass1_1010_2ee2(u_var2, (u_var2 >> 0x10));
            PostMessage16(0, 0x10a, 0x111, &param_1.field_0x6)
        }
        0x107 => {
            u_var3 = 0;
            // goto LAB_1040_75ba;
        }
        0x108 => unsafe {
            u_var3 = 1;
            // LAB_1040_75ba:
            u_var2 = (param_1 + 1);
            destroy_win_1010_3202(u_var2, (u_var2 >> 0x10), u_var3)
        }
        0x10a => unsafe {
            GetClientRect16(CONCAT22(unaff_ss, &local_a), &param_1.field_0x6);
            u_var2 = (param_1 + 1);
            local_8 = local_8 + 3;
            local_a = (u_var2 + 0x1a) - 9;
            local_6 = local_6 - 3;
            local_4 = local_4 - 3;
            InvalidateRect16(1, &local_a, unaff_ss);
            window::destroy_win_1010_2fa0((param_1 + 1));
            pass1_1010_32c0((param_1 + 1), 0);
            u_var2 = (param_1 + 1);
            pass1_1010_2ee2(u_var2, (u_var2 >> 0x10))
        }
        0x10c => {
            DestroyWindow16(&param_1.field_0x6);
        }
    }
    return;
}

pub unsafe fn win_fn_1040_65ba(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let pu_var3: &mut u16;
    let struct_a: &mut Struct199;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut unaff_si: u16;
    let mut u_var6: u16;
    let unaff_ss: HWND16;
    let mut u_var7: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    _local_6 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x2b));
    ctx.g_u16_ptr_1050_5f2e = (_local_6 >> 0x10);
    local_8 = return3_1010_0898();
    if (ctx.g_struct_ptr_1 == 0) {
        g_struct_ptr_1 = struct_fn_1000_160a();
    } else {
    }
    _local_1a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, g_struct_ptr_1);
    u_var2 = alloc_mem_1000_1708(
        (local_8 + 2) * 4,
        0,
        1,
        g_struct_ptr_1,
        ctx.g_u16_ptr_1050_5f2e,
    );
  // u_var6 = (param_1  >> 0x10);
    i_var4 = param_1;
    (i_var4 + 0x8e) = u_var2;
    (i_var4 + 0x90) = ctx.g_u16_ptr_1050_5f2e;
    local_a = 1;
    while (local_a <= local_8) {
      // _local_1a = pass1_1010_0946(_local_6, (_local_6  >> 0x10), local_a);
      // struct_a = (_local_1a  >> 0x10);
        local_22 = *_local_1a;
        local_20 = (_local_1a + 2);
        local_1e = 1;
        local_1c = 1;
        u_var2 = (i_var4 + 6);
        pu_var3 = &local_22;
        MapDialogRect16(CONCAT22(pu_var3, 0x1010), unaff_ss);
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | pu_var3) == 0) {
            u_var1 = (i_var4 + 0x8e);
            (u_var1 + local_a * 4) = 0;
        } else {
            u_var7 = win_fn_1008_3bd6(
                pu_var3,
                struct_a,
                0,
                CONCAT22(local_22, local_20),
                0x1000101,
                CONCAT22((_local_1a + 4), 0xff),
                CONCAT22(u_var2, (i_var4 + 6)),
            );
            u_var1 = (i_var4 + 0x8e);
          // u_var2 = (u_var1  >> 0x10);
            i_var5 = u_var1;
            (i_var5 + local_a * 4) = u_var7;
            (i_var5 + local_a * 4 + 2) = (u_var7 >> 0x10);
        }
        u_var1 = (i_var4 + 0x8e);
      // u_var2 = (u_var1  >> 0x10);
        i_var5 = u_var1;
        if ((i_var5 + local_a * 4) != 0) {
            u_var1 = (i_var5 + local_a * 4);
            window::enable_window_1040_9234(u_var1, (u_var1 >> 0x10), (_local_1a + 6));
        }
        local_a = local_a + 1;
    }
    window::move_window_1040_826c(param_1, u_var6, 0xff, 0xffff);
    ShowWindow16(5, (i_var4 + 6));
    return;
}

pub unsafe fn win_fn_1040_5800(param_1: &mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: i32;
    let pu_var4: Vec<u8>;
    let h_wnd: HWND16;
    let mut u_var5: u32;

    let paVar6: &mut Struct199;
    let mut i_var7: i32;
    let mut unaff_si: u16;
    let mut u_var8: u16;
    let unaff_ss: HWND16;
    let mut local_24: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: [u8; 8];
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0xeb) {
        _local_6 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 3));
      // paVar6 = (_local_6  >> 0x10);
        u_var5 = &param_1.field_0x90;
        if (u_var5 != 0) {
            local_a = u_var5;
            process_struct_1000_179c(0x18, paVar6);
            u_var3 = u_var5;
            if ((paVar6 | u_var3) == 0) {
                u_var3 = 0;
                paVar6 = 0x0;
            } else {
                process_struct_1040_a598((u_var5 & 0xffff | ZEXT24(paVar6) << 0x10));
                paVar6 = ctx.dx_reg;
            }
            param_1.field_0x90 = u_var3;
            &param_1.field_0x92 = paVar6;
            *&param_1.field_0x90 = 6;
            local_c = *&param_1.field_0x90;
            u_var3 = local_c * 10 + 2;
            process_struct_1000_179c(u_var3, paVar6);
            _local_18 = CONCAT22(paVar6, u_var3);
            if ((paVar6 | u_var3) == 0) {
                u_var2 = &param_1.field_0x90;
                (u_var2 + 2) = 0;
            } else {
                *_local_18 = local_c;
                call_fn_ptr_1000_5586(
                    0xa564,
                    &ctx.PTR_LOOP_1050_1040,
                    local_c,
                    10,
                    u_var3 + 2,
                    paVar6,
                );
                u_var2 = &param_1.field_0x90;
              // u_var8 = (u_var2  >> 0x10);
                i_var7 = u_var2;
                (i_var7 + 2) = u_var3 + 2;
                (i_var7 + 4) = paVar6;
            }
            u_var2 = &param_1.field_0x90;
            (u_var2 + 6) = (local_a + 6);
            u_var2 = &param_1.field_0x90;
            (u_var2 + 10) = 4;
            u_var2 = &param_1.field_0x90;
            (u_var2 + 0x12) = &param_1.field_0xa;
            u_var8 = 0x1010;
            pass1_1010_a50c(_local_6, 0x10505d78, &param_1.field_0x90);
            if (local_a != 0) {
                pass1_1040_a5d0(local_a);
                u_var8 = 0x1000;
                error_check_1000_17ce(local_a);
            }
            pp_var1 = (CONCAT22(param_2_00, param_1) + 0x70);
            (**pp_var1)(u_var8, param_1, param_2_00);
            pu_var4 = pass1_1040_5cd6(CONCAT22(param_2_00, param_1));
            if (pu_var4 != 0x0) {
                pass1_1040_5eaa(CONCAT22(param_2_00, param_1));
                (param_1 + 1) = 0;
            }
            pass1_1040_5dc4(CONCAT22(param_2_00, param_1));
            GetWindowRect16(CONCAT22(local_14, u_var8), unaff_ss);
            InvalidateRect16(&param_1[1].field_0x8, 0x0, 0);
            if (&param_1[1].field_0x8 != 0) {
                &param_1[1].field_0x8 = 0;
            }
        }
    } else {
        if (param_2 != 0x13b) {
            window::win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
            return;
        }
        h_wnd = GetDlgItem16(0x1790, &param_1.field_0x6);
        EnableWindow16(1, h_wnd);
    }
    return;
}

pub fn pass1_1040_57d4(param_1: Vec<u8>) {
    pass1_1040_5d42(param_1);
    pass1_1040_5eaa(param_1);
    pass1_1040_5dc4(param_1);
    window::set_window_pos_1040_b230(param_1);
    return;
}

pub fn pass1_1040_5238(param_1: Vec<u8>) -> Vec<u8> {
    let pp_var1: fn();

    pp_var1 = ((param_1 + 0x94) + 8);
    (**pp_var1)();
    return 0x0;
}

pub unsafe fn win_fn_1040_477e(param_1: u32) {
    let u_var1: u8;
    let mut u_var2: u16;
    let extraout_var: u32;
    let mut u_var4: i32;
    let mut u_var5: i32;

    let ppVar6: &mut Struct2551;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut in_stack_0000ffee: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut u_var3: u32;

    window::set_window_pos_1040_b230(param_1, (param_1 >> 0x10));
    ppVar6 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(in_stack_0000ffee, 3));
  // u_var4 = (ppVar6  >> 0x10);
    u_var8 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var7 = 0x5d68;
    u_var5 = u_var4;
    u_var1 = string_fn_1008_5fd8(0x7d3, -0x15);
    u_var3 = CONCAT31(extraout_var, u_var1);
    u_var2 = u_var3;
    process_string_1000_3cea((u_var3 & 0xffff | u_var5 << 0x10), CONCAT22(u_var8, u_var7));
    pass1_1010_e964(ppVar6, u_var4);
    process_string_1000_3cea(
        (u_var3 & 0xffff | u_var5 << 0x10),
        CONCAT22(ctx.dx_reg, u_var2),
    );
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x10)),
        (u_var3 & 0xffff | u_var5 << 0x10),
    );
    error_check_1000_17ce((u_var3 & 0xffff | u_var5 << 0x10));
    return;
}

pub unsafe fn win_gui_fn_1040_45e8(param_1: &mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let in_struct_1: &mut Struct44;
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: i32;
    let paVar4: &mut Struct44;

    let pa_var5: &mut Struct199;
    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut u_var7: u16;
    let pp_var8: &mut Struct2551;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 != 0xeb) {
        window::win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
        return;
    }
    pp_var8 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 3));
  // pa_var5 = (pp_var8  >> 0x10);
    in_struct_1 = &param_1.field_0x90;
    if (in_struct_1 != 0x0) {
        paVar4 = in_struct_1;
        process_struct_1000_179c(0x18, pa_var5);
        u_var3 = paVar4;
        if ((pa_var5 | u_var3) == 0) {
            u_var3 = 0;
            pa_var5 = 0x0;
        } else {
            process_struct_1040_a598((paVar4 & 0xffff | ZEXT24(pa_var5) << 0x10));
            pa_var5 = ctx.dx_reg;
        }
        param_1.field_0x90 = u_var3;
        &param_1.field_0x92 = pa_var5;
        *&param_1.field_0x90 = 0x14;
        i32_var6 = &param_1.field_0x90;
        u_var3 = i32_var6 * 10 + 2;
        process_struct_1000_179c(u_var3, pa_var5);
        _local_10 = CONCAT22(pa_var5, u_var3);
        if ((pa_var5 | u_var3) == 0) {
            u_var2 = &param_1.field_0x90;
            (u_var2 + 2) = 0;
        } else {
            *_local_10 = i32_var6;
            call_fn_ptr_1000_5586(
                0xa564,
                &ctx.PTR_LOOP_1050_1040,
                i32_var6,
                10,
                u_var3 + 2,
                pa_var5,
            );
            u_var2 = &param_1.field_0x90;
          // u_var7 = (u_var2  >> 0x10);
            i32_var6 = u_var2;
            (i32_var6 + 2) = u_var3 + 2;
            (i32_var6 + 4) = pa_var5;
        }
        u_var2 = &param_1.field_0x90;
        (u_var2 + 6) = (in_struct_1 + 6);
        u_var2 = &param_1.field_0x90;
        (u_var2 + 10) = 1;
        u_var2 = &param_1.field_0x90;
        (u_var2 + 0x12) = &param_1.field_0xa;
        u_var7 = 0x1010;
        pass1_1010_a50c(pp_var8, 0x10505d40, &param_1.field_0x90);
        if (in_struct_1 != 0x0) {
            pass1_1040_a5d0(in_struct_1);
            u_var7 = 0x1000;
            error_check_1000_17ce(in_struct_1);
        }
        pp_var1 = (CONCAT22(param_2_00, param_1) + 0x70);
        (**pp_var1)(u_var7, param_1, param_2_00);
    }
    return;
}

pub unsafe fn win_fn_1040_410e(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let unaff_ss: String;
    let ppVar4: &mut Struct2551;
    let pu_var5: &mut u16;
    let pu_var6: &mut u16;
    let pcVar7: String;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: [u8; 6];
    let mut local_2a: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 10];

    win_gui_func_1040_78e2(param_1);
    pass1_1000_4906(CONCAT22(unaff_ss, local_c), 0, 10);
  // u_var3 = (param_1  >> 0x10);
    i_var2 = param_1;
    u_var1 = (i_var2 + 0x8e);
    string_fn_1000_3f9c(
        local_c,
        unaff_ss,
        0x5d38,
        &ctx.g_alloc_addr_1050_1050,
        (u_var1 + 0x76),
    );
    local_e = GetDlgItem16(0xfb5, (i_var2 + 6));
    SendMessage16(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, local_c)),
        0,
        0xc,
        local_e,
    );
    SetFocus16(local_e);
    SendMessage16(-0x10000, 0, 0x401, local_e);
    GetWindowRect16(CONCAT22(&local_16, 0x1538), unaff_ss);

    pass1_1000_4906(CONCAT22(unaff_ss, &local_1e), 0, 8);
    u_var1 = (i_var2 + 0x8e);
  // _local_22 = pass1_1010_5f7a(u_var1, (u_var1  >> 0x10), 0, 7);
    if (_local_22 != 0x0) {
        _local_1e = *_local_22;
        _local_1a = (_local_22 + 4);
    }
    if ((local_1c == 0) && (local_1e == 0)) {
        zero_list_1008_3e38(CONCAT22(unaff_ss, local_30));
        u_var1 = (i_var2 + 0x96);
        pass1_1018_2678(u_var1, (u_var1 >> 0x10), CONCAT22(unaff_ss, local_30));
        pass1_1008_3e94(
            local_30,
            CONCAT22(unaff_ss, &local_32),
            CONCAT22(unaff_ss, &local_2a),
        );
        pu_var6 = &local_34;
        pu_var5 = &local_36;
        pcVar7 = unaff_ss;
        ppVar4 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(pu_var5, 0x48));
        pass1_1008_3e94(
            (ppVar4 + 0xe),
            CONCAT22(unaff_ss, pu_var5),
            CONCAT22(pcVar7, pu_var6),
        );
        _local_1a = CONCAT22(local_10 - local_14, local_12 - local_16);
        _local_1e = CONCAT22(
            (((ppVar4 + 0xc) * -0x14) / 600 - (local_10 - local_14)) + local_36 + local_32,
            local_34 + local_2a,
        );
    }
    window::move_window_1040_826c(i_var2, u_var3, local_1c, local_1e);
    ShowWindow16(5, (i_var2 + 6));
    return;
}

pub fn invalidate_rect_1040_3ddc() {
    let unaff_ss: HWND16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_a = 0x780005;
    local_6 = 0xdc0069;
    InvalidateRect16(0, &local_a, unaff_ss);
    return;
}

pub unsafe fn release_dc_1040_3d5e(param_1: u32) -> u16 {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let pu_var5: &mut u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var4 = (param_1  >> 0x10);
    i_var2 = param_1;
    local_4 = GetDC16((i_var2 + 6));
    pu_var5 = mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, (i_var2 + 0xa4));
    unsafe {
        i_var3 = *pu_var5;
    }
    pp_var1 = (i_var3 + 8);
    unsafe {
        (**pp_var1)(0x1010, pu_var5, &local_4);
    }
    pp_var1 = (i_var3 + 4);
    unsafe {
        (**pp_var1)(0x1010, pu_var5, 0x50078, &local_4);
    }
    pp_var1 = (i_var3 + 0xc);
    (**pp_var1)(0x1010, pu_var5, &local_4);
    ReleaseDC16(local_4, (i_var2 + 6));
    return 0;
}

pub unsafe fn win_func_1040_3c64(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let LVar3: LRESULT;
    let paVar4: &mut Struct318;
    let b_var5: bool;
    let mut local_8: u16;

    if (param_3 == (s_You_may_not_run_a_turn__The_game_1050_0172 + 0x14)) {
        LVar3 = SendDlgItemMessage16(0, 0, 0x409, 400, (param_1 + 6));
        u_var1 = GetDlgItemInt16(0, 0x0, 0, 0x18e);
        set_struct_1018_36e6((param_1 + 0x8e), u_var1, LVar3, (param_1 + 0xa0));
        pass1_1038_af40(ctx.g_struct_112_001, *(param_1 + 8), 0x22);
        SendMessage16(0, 2, 0x111, (param_1 + 6));
        b_var5 = 1;
        paVar4 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x1002b);
        pass1_1010_038e(paVar4, b_var5);
    } else {
        if (param_3 + -0xc3 < &dos_alloc_addr_1050_0002) {
            // LAB_1040_3c7f:
            post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3);
            return;
        }
        if (param_3 + -0xc4 < (&PTR_DAT_0005_0000_1050_0004 + 1)
            || param_3 == (s_You_may_not_run_a_turn__The_game_1050_0172 + 0x1b))
        {
            (param_1 + 0xa0) = param_3;
            u_var2 = string_fn_1018_3b9e((param_1 + 0x8e), param_3);
            dialog::send_dialog_item_msg_1040_3f12(param_1, param_2, u_var2);
        } else {
            if (param_3 + -0xc4 != &BYTE_1050_0008) {}
            // goto LAB_1040_3c7f;
            if (param_3 != 1) {
                return;
            }
        }
        dialog::win_gui_dialog_func_1040_3e08(CONCAT22(param_2, param_1));
    }
    return;
}

pub fn gui_win_fn_1040_3b1e(param_1: &mut Struct25) {
    let in_struct_a: &mut Struct298;
    let b_var1: bool;
    let HVar2: HWND16;

    let mut i_var3: i32;
    let mut unaff_si: u16;
    let mut u_var4: i32;
    let mut unaff_ss: u16;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_10e: u16;
    let puStack268: Vec<u8>;
    let BStack264: bool;
    let paStack262: &mut Struct566;
    let mut local_8c: [u8; 130];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 2));
    local_a = (_local_6 + 0x68);
  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_ss, local_8c), (i_var3 + 6));
    wsprintf16(
        &local_10e,
        CONCAT22(local_8c, unaff_ss),
        CONCAT22(local_a, unaff_ss),
    );
    SetWindowText16(CONCAT22(unaff_ss, &local_10e), (i_var3 + 6));
    pass1_1018_3d44(
        ((i_var3 + 0x8e) & 0xffff | (i_var3 + 0x96) << 0x10),
        param_1 & 0xffff0000 | u_var4,
        param_1 & 0xffff0000 | (i_var3 + 0x96),
    );
  // local_11a = (ctx.g_struct_73_1050_14cc  >> 0x10);
    load_string_1010_84e0(
        ctx.g_struct_73_1050_14cc,
        local_11a,
        0x80,
        CONCAT22(unaff_ss, &local_10e),
        0x7d7,
    );
    local_118 = *CONCAT22(0x80, local_11a);
    wsprintf16(
        local_8c,
        CONCAT22(&local_10e, unaff_ss),
        CONCAT22(local_118, unaff_ss),
    );
    paStack262 = (i_var3 + 6);
    BStack264 = 0x187;
    puStack268 = local_8c;
    local_10e = SUB42(offset, 0);
    SetDlgItemText16(CONCAT22(unaff_ss, puStack268), 0x187, paStack262);
    paStack262 = (i_var3 + 6);
    BStack264 = 0x188;
    puStack268 = 0x188;
    local_10e = SUB42(offset, 0);
    b_var1 = CheckRadioButton16(0x188, 0x18d, 0x188, paStack262);
    (i_var3 + 0xa0) = 0x188;
    paStack262 = (i_var3 + 0xa0);
    in_struct_a = (i_var3 + 0x8e);
  // BStack264 = (in_struct_a  >> 0x10);
    puStack268 = offset;
    local_10e = 0x3c2b;
    string_fn_1018_3b9e(in_struct_a, paStack262);
    local_10e = 0x1018;
    puStack268 = i_var3;
    BStack264 = b_var1;
    paStack262 = ctx.dx_reg;
    dialog::send_dialog_item_msg_1040_3f12();
    puStack268 = 0x3c47;
    BStack264 = i_var3;
    paStack262 = u_var4;
    dialog::win_gui_dialog_func_1040_3e08(param_1);
    paStack262 = (i_var3 + 6);
    BStack264 = 0x186;
    puStack268 = 0x3c56;
    HVar2 = GetDlgItem16(0x186, paStack262);
    (i_var3 + 0x9a) = HVar2;
    return;
}

pub fn win_fn_1040_3ae8(param_1: &mut Struct20) {
    let mut u_var1: u16;

    win_gui_func_1040_78e2(param_1);
    window::move_window_1040_826c(param_1, 0xffffffff);
  // u_var1 = (param_1  >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub unsafe fn win_fn_1040_37f0(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut unaff_ss: u16;
    let mut in_stack_0000fbec: u16;
    let b_var1: bool;
    let mut local_40c: [u8; 1026];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3 == 0x193) {
        _local_6 =
            struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(in_stack_0000fbec, 2));
        local_a = (_local_6 + 0x68);
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_ss, local_40c),
            0x44f,
        );
        MessageBox16(0x30, local_a, CONCAT22(unaff_ss, local_40c), (param_1 + 6));
        pass1_1018_3710((param_1 + 0x8e));
        PostMessage16(0, 2, 0x111, (param_1 + 6));
    } else {
        if (param_3 != 0x194) {
            post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3);
            return;
        }
        pass1_1038_af40(ctx.g_struct_112_001, *(param_1 + 8), 0x21);
        SendMessage16(0, 2, 0x111, (param_1 + 6));
        b_var1 = 1;
        _local_6 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x1002b);
        pass1_1010_038e(_local_6, b_var1);
    }
    return;
}

pub fn set_focus_1040_355a(param_1: &mut Struct20) {
    let mut u_var1: u16;

    win_gui_func_1040_78e2(param_1);
    window::move_window_1040_826c(param_1, 0xffffffff);
  // u_var1 = (param_1  >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub fn win_fn_1040_3590(param_1: u32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let hwnd: HWND16;


    let mut extraout_d_x_01: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_5b0: u16;
    let mut uStack1454: u16;
    let uStack1450: u8;
    let uStack1449: u8;
    let mut local_5a8: u16;
    let mut local_5a6: u16;
    let mut local_5a4: u16;
    let mut local_5a2: u16;
    let mut local_5a0: u16;
    let mut local_59e: u16;
    let mut local_59c: u16;
    let mut local_59a: u32;
    let mut local_596: u32;
    let mut local_592: u16;
    let mut local_590: u16;
    let mut local_58e: u16;
    let mut uStack1420: u16;
    let uStack1418: u8;
    let uStack1417: u8;
    let HStack1410: HWND16;
    let HStack1408: HWND16;
    let mut local_50c: [u8; 256];
    let mut local_40c: u32;
    let mut local_408: u16;
    let mut local_406: u16;
    let mut local_404: [u8; 1026];

    _local_408 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_5b0, 2));
    local_40c = (_local_408 + 0x68);
  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_ss, local_50c), (i_var3 + 6));
    wsprintf16(
        &local_58e,
        CONCAT22(local_50c, unaff_ss),
        CONCAT22(local_40c, unaff_ss),
    );
    local_592 = SetWindowText16(CONCAT22(unaff_ss, &local_58e), (i_var3 + 6));
    u_var1 = (i_var3 + 0x8e);
    local_5a6 = u_var1;
  // local_5a4 = (u_var1  >> 0x10);
    wsprintf_1018_34b6(local_5a6, local_5a4);
    local_590 = ctx.dx_reg;
    pass1_1018_3d44(
        *(i_var3 + 0x8e),
        CONCAT22(unaff_ss, &local_59a),
        CONCAT22(unaff_ss, &local_596),
    );
    HVar2 = GetDlgItem16(0x193, (i_var3 + 6));
    (i_var3 + 0x98) = HVar2;
    EnableWindow16(1, HVar2);
    uStack1454 = ctx.g_struct_73_1050_14cc;
    load_string_1010_84e0(
        uStack1454,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_404),
        1099,
    );
    uStack1450 = SUB21(local_404, 0);
    uStack1449 = (local_404 >> 8);
    wsprintf16(
        local_50c,
        CONCAT13(uStack1449, CONCAT12(uStack1450, unaff_ss)),
        CONCAT22(local_596, unaff_ss),
    );
    local_59a = (i_var3 + 6);
    local_59a._0_2_ = 0x195;
    local_59a = GetDlgItem16(0x195, local_59a);
    SetWindowText16(CONCAT22(unaff_ss, local_50c), local_59a);
    local_596._0_2_ = (i_var3 + 6);
    local_59a = 0x196;
    local_59a._0_2_ = offset;
    HVar2 = GetDlgItem16(0x196, local_596);
    u_var1 = (i_var3 + 0x8e);
    local_59a._0_2_ = u_var1;
    local_59a = (u_var1 >> 0x10);
    local_596._0_2_ = HVar2;
    wsprintf_1018_34b6();
    local_59a._0_2_ = HVar2;
    local_59a = ctx.dx_reg;
    SetWindowText16(CONCAT22(ctx.dx_reg, HVar2), local_596);
    local_596 = (i_var3 + 6);
    local_596._0_2_ = 0x197;
    local_59a = offset;
    local_59a._0_2_ = 0x36cb;
    GetDlgItem16(0x197, local_596);
    local_596 = 0x443;
    local_59a = local_404;
    local_59a._0_2_ = 0x3ff;
  // local_59c = (ctx.g_struct_73_1050_14cc  >> 0x10);
    load_string_1010_84e0(
        ctx.g_struct_73_1050_14cc,
        local_59c,
        0x3ff,
        CONCAT22(unaff_ss, local_59a),
        0x443,
    );
    local_596 = offset;
    local_59a = local_404;
    local_59a._0_2_ = 0x1010;
    SetWindowText16(CONCAT22(unaff_ss, local_59a), offset);
    local_592 = 0x44c;
    local_596._0_2_ = local_404;
    local_59a = 0x3ff;
    local_59c = ctx.g_struct_73_1050_14cc;
    local_59a._0_2_ = (ctx.g_struct_73_1050_14cc >> 0x10);
    load_string_1010_84e0(
        local_59c,
        local_59a,
        0x3ff,
        CONCAT22(unaff_ss, local_596),
        0x44c,
    );
    local_596 = local_59a;
    local_592 = local_59a;
    local_59a = local_404;
    wsprintf16(
        local_50c,
        CONCAT22(local_59a, unaff_ss),
        CONCAT22(local_59a, unaff_ss),
    );
    uStack1418 = 0x38;
    uStack1417 = 0x15;
    uStack1420 = 0x3732;
    HVar2 = GetDlgItem16(0x198, (i_var3 + 6));
    uStack1418 = SUB21(local_50c, 0);
    uStack1417 = (local_50c >> 8);
    uStack1420 = offset;
    local_58e = 0x3742;
    SetWindowText16(CONCAT22(unaff_ss, local_50c), HVar2);
    uStack1418 = 0x51;
    uStack1417 = 0x37;
    hwnd = GetDlgItem16(0x199, (i_var3 + 6));
    uStack1418 = 99;
    uStack1417 = 0x37;
    HVar2 = hwnd;
    wsprintf_1018_35b0();
    if ((extraout_d_x_01 | HVar2) == 0) {
        uStack1418 = 0xff;
        uStack1417 = 3;
        local_58e = ctx.g_struct_73_1050_14cc;
      // uStack1420 = (ctx.g_struct_73_1050_14cc  >> 0x10);
        local_590 = 0x1018;
        local_592 = 0x3785;
        load_string_1010_84e0(
            local_58e,
            uStack1420,
            0x3ff,
            CONCAT22(unaff_ss, local_404),
            0x44d,
        );
        uStack1418 = 0x10;
        uStack1417 = 0x10;
        uStack1420 = 0x3794;
        SetWindowText16(CONCAT22(unaff_ss, local_404), hwnd);
        HStack1410 = (i_var3 + 6);
        HVar2 = GetDlgItem16(0x19a, HStack1410);
        HStack1410 = 0x44e;
        uStack1420 = ctx.g_struct_73_1050_14cc;
      // uStack1418 = (ctx.g_struct_73_1050_14cc  >> 0x10);
        uStack1417 = (ctx.g_struct_73_1050_14cc >> 0x18);
        local_58e = offset;
        local_590 = 0x37bd;
        load_string_1010_84e0(
            uStack1420,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_ss, local_404),
            0x44e,
        );
        uStack1418 = 0xcc;
        uStack1417 = 0x37;
        HStack1410 = HVar2;
        SetWindowText16(CONCAT22(unaff_ss, local_404), HVar2);
        HStack1408 = (i_var3 + 0x98);
        HStack1410 = 0;
        EnableWindow16(0, HStack1408);
        return;
    }
    uStack1418 = 0x18;
    uStack1417 = 0x10;
    uStack1420 = 0x37eb;
    SetWindowText16(CONCAT22(extraout_d_x_01, HVar2), hwnd);
    return;
}

pub unsafe fn win_fn_1040_311a(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut i_var1: i32;
    let mut i_var2: i32;
    let ppVar3: &mut pass1_struct_2;

    let pu_var4: Vec<u8>;
    let pa_var5: &mut Struct318;
    let mut u_var6: u16;
    let BVar7: bool;
    let mut local_12: u16;
    let mut local_10: u16;

    send_msg_1040_323c(param_1, param_2_00);
    load_string_1010_847e(
        ctx.g_struct_73_1050_14cc,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    if (param_2 == 0x181) {
        i_var1 = param_1 + 0x9a;
        i_var2 = i_var1;
        pass1_1018_3cda(
            (param_1 + 0x96),
            CONCAT22(param_2_00, param_1 + 0x19a),
            CONCAT22(param_2_00, i_var1),
        );
        pass1_1018_3424((param_1 + 0x96));
        if (i_var2 == 0) {
            u_var6 = 0x21;
        } else {
            pass1_1018_3a42((param_1 + 0x96), CONCAT22(param_2_00, i_var1));
            pu_var4 = ctx.dx_reg;
            ppVar3 = pass1_1030_8344(
                ctx._g_bool_1050_5748,
                (ctx._g_bool_1050_5748 >> 0x10),
                CONCAT22(ctx.dx_reg, i_var2),
            );
            ctx.PTR_LOOP_1050_5f0c = pass1_1030_8344(
                ctx._g_bool_1050_5748,
                (ctx._g_bool_1050_5748 >> 0x10),
                &ppVar3.field_0x10,
            );
            PTR_LOOP_1050_5f10 = (&ctx.PTR_LOOP_1050_0000 + 1);
            u_var6 = 0x25;
            ctx.PTR_LOOP_1050_5f0e = pu_var4;
        }
        pass1_1038_af40(ctx.g_struct_112_001, *(param_1 + 8), u_var6);
        SendMessage16(0, 2, 0x111, (param_1 + 6));
        BVar7 = 1;
        pa_var5 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x1002b);
        pass1_1010_038e(pa_var5, BVar7);
    } else {
        if ((param_2 == 0x181) || (1 < param_2 - 0x182)) {
            post_win_msg_1040_7b3c(param_1, param_2_00, param_3, param_2, param_2);
            return;
        }
        window::set_window_pos_1040_331a(param_1, param_2_00, param_3, param_2, param_2);
    }
    return;
}

pub fn set_focus_1040_2f5a(param_1: &mut Struct20) {
    let mut u_var1: u16;

    win_gui_func_1040_78e2(param_1);
    window::move_window_1040_826c(param_1, 0xffffffff);
  // u_var1 = (param_1  >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub unsafe fn win_fn_1040_2f90(param_1: u32) {
    let HVar1: HWND16;
    let mut u_var2: u32;


    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let ppVar5: &mut Struct2551;
    let in_stack_0000fed2: u8;
    let in_stack_0000fed3: u8;
    let mut local_126: u16;
    let mut local_124: u16;
    let mut local_122: u16;
    let mut local_120: u16;
    let mut local_11e: u32;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_116: u32;
    let mut local_112: u32;
    let mut local_10e: [u8; 130];
    let mut local_8c: [u8; 130];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = struct_ops::process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT13(in_stack_0000fed3, CONCAT12(in_stack_0000fed2, 2)),
    );
    local_a = (_local_6 + 0x68);
  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_ss, local_8c), (i_var3 + 6));
    wsprintf16(
        local_10e,
        CONCAT22(local_8c, unaff_ss),
        CONCAT22(local_a, unaff_ss),
    );
    SetWindowText16(CONCAT22(unaff_ss, local_10e), (i_var3 + 6));
    HVar1 = GetDlgItem16(0x182, (i_var3 + 6));
    (i_var3 + 0x92) = HVar1;
    pass1_1018_3a94(
        *(i_var3 + 0x96),
        CONCAT22(unaff_ss, &local_116),
        CONCAT22(unaff_ss, &local_112),
    );
    local_126 = local_112;
  // local_124 = (local_112  >> 0x10);
    win_fn_1040_3374(
        i_var3,
        (param_1 >> 0x10),
        local_126,
        local_124,
        (i_var3 + 0x92),
    );
    ppVar5 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_120, 0x2f));
    u_var2 = (ppVar5 + 0x24);
    pass1_1018_3a7a(*(i_var3 + 0x96), u_var2);
    SendMessage16(
        CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, u_var2)),
        0xffff,
        0x40d,
        (i_var3 + 0x92),
    );
    HVar1 = GetDlgItem16(0x183, (i_var3 + 6));
    (i_var3 + 0x94) = HVar1;
    local_124 = local_116;
  // local_122 = (local_116  >> 0x10);
    win_fn_1040_3374(param_1, u_var4, local_124, local_122, HVar1);
    local_124 = ctx.g_struct_73_1050_14cc;
  // local_122 = (ctx.g_struct_73_1050_14cc  >> 0x10);
    load_string_1010_847e(local_124, local_122, 0x531);
    SendDlgItemMessage16(
        CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, HVar1)),
        0,
        0x403,
        0x183,
        (i_var3 + 6),
    );
    SendDlgItemMessage16(0x40dffff, 0xffff, 0x40d, 0x183, (i_var3 + 6));
    HVar1 = GetDlgItem16(0x181, (i_var3 + 6));
    (i_var3 + 0x8e) = HVar1;
    HVar1 = GetDlgItem16(0x184, (i_var3 + 6));
    (i_var3 + 0x90) = HVar1;
    return;
}

pub fn win_fn_1040_2d48(param_1: u32) {
    let mut u_var1: u16;
    let mut value: u16;
    let unaff_ss: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: bool;

    dialog::set_dlg_item_txt_1040_b45e(param_1);
    u_var1 = GetDlgItemInt16(1, &local_4, unaff_ss, 0x163);
    value = GetDlgItemInt16(1, &local_4, unaff_ss, 0x165);
    if (u_var1 != 0) {
        value = value / u_var1;
    }
    SetDlgItemInt16(1, value, 0x165, (param_1 + 6));
    return;
}

pub unsafe fn win_fn_1040_2512(param_1: u32, param_2: u32, param_3: u16) {
    let pi_var1: &mut i32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: i32;
    let HVar5: HWND16;
    let pu_var6: Vec<u8>;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let in_dx: &mut Struct199;

    let mut u_var9: i32;
    let mut i_var10: i32;
    let mut i_var11: i32;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let unaff_ss: String;
    let u_var14: u8;
    let in_stack_0000ffdc: String;
    let mut local_20: u16;
    let mut local_1e: [u8; 4];
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    if (param_3 != 2) {
        i_var10 = param_1;
        if (0x19d < param_3 - 2) {
          // u_var12 = (param_1  >> 0x10);
            u_var14 = (unaff_ss >> 8);
            if (param_3 - 0x1a0 < 0x14 || param_3 == 0x1b4) {
                u_var7 = IsDlgButtonChecked16(param_3, (i_var10 + 6));
                if (u_var7 == 0) {
                    pi_var1 = (i_var10 + 0x92);
                    unsafe {
                        *pi_var1 = *pi_var1 + 1;
                    }
                    if (0 < (i_var10 + 0x92)) {
                        (i_var10 + 0x94) = 0;
                    }
                    u_var3 = (i_var10 + 0x8e);
                    if ((u_var3 + 0x28) == (i_var10 + 0x92)) {
                        HVar5 = GetDlgItem16(0xfb1, (i_var10 + 6));
                        EnableWindow16(0, HVar5);
                    }
                } else {
                    pi_var1 = (i_var10 + 0x92);
                    unsafe {
                        *pi_var1 = *pi_var1 + -1;
                    }
                    HVar5 = GetDlgItem16(0xfb1, (i_var10 + 6));
                    IsWindowEnabled16(offset, HVar5);
                    u_var4 = ctx.dx_reg;
                    if (HVar5 == 0) {
                        HVar5 = GetDlgItem16(0xfb1, (i_var10 + 6));
                        EnableWindow16(1, HVar5);
                    }
                    if ((i_var10 + 0x92) < 1) {
                        (i_var10 + 0x94) = 1;
                    }
                    pass1_1018_1c9a(*(i_var10 + 0x8e), param_3);
                    u_var8 = pass1_1018_1e78((i_var10 + 0x8e), 0xffff);
                    _local_a = (u_var8 & 0xffff | u_var4 << 0x10);
                    if ((u_var4 | u_var8) == 0) {
                        local_c = 0;
                    } else {
                        local_c = (u_var8 + 0x1c);
                    }
                    mci_send_command_1008_5c7c(ctx.g_struct_1050_02a0, CONCAT22(local_c, 1));
                }
                if ((i_var10 + 0x92) < 0) {
                    return;
                }
                u_var3 = (i_var10 + 0x8e);
                if ((u_var3 + 0x28) < (i_var10 + 0x92)) {
                    return;
                }
                string_fn_1000_3f9c(
                    &local_16,
                    unaff_ss,
                    s__d_1050_5cf4,
                    &ctx.g_alloc_addr_1050_1050,
                    *(i_var10 + 0x92),
                );
                SetDlgItemText16(
                    CONCAT13(u_var14, CONCAT12(unaff_ss, &local_16)),
                    0xfb2,
                    (i_var10 + 6),
                );
                return;
            }
            u_var4 = param_3 - 0xfb1;
            if (u_var4 == 0) {
                if ((i_var10 + 0x92) < 0) {
                    process_struct_1000_179c(0xb4, in_dx);
                    u_var9 = in_dx | u_var4;
                    local_1a = u_var4;
                    if (u_var9 == 0) {
                        u_var4 = 0;
                        u_var9 = 0;
                    } else {
                        mixed_1040_8520(
                            CONCAT13((in_dx >> 8), CONCAT12(in_dx, u_var4)),
                            ctx.g_h_window,
                            0x30,
                            2,
                            0x57b,
                            0x57c,
                        );
                    }
                    _local_a = CONCAT22(u_var9, u_var4);
                    ppc_var2 = (*_local_a + 0x74);
                    ppc_var2(0, u_var4, u_var9);
                    return;
                }
                if (0 < (i_var10 + 0x92)) {
                    process_struct_1000_179c(0xb4, in_dx);
                    u_var9 = in_dx | u_var4;
                    local_1a = u_var4;
                    if (u_var9 == 0) {
                        u_var4 = 0;
                        u_var9 = 0;
                    } else {
                        mixed_1040_8520(
                            CONCAT13((in_dx >> 8), CONCAT12(in_dx, u_var4)),
                            ctx.g_h_window,
                            0x21,
                            2,
                            0x57b,
                            0x57d,
                        );
                    }
                    _local_a = CONCAT22(u_var9, u_var4);
                    pass1_1008_941a(CONCAT13(u_var14, CONCAT12(unaff_ss, local_1e)), 1, 0xc2);
                    pu_var6 = local_1e;
                    ppc_var2 = (*_local_a + 0x6c);
                    ppc_var2(
                        &ctx.PTR_LOOP_1050_1008,
                        _local_a,
                        (_local_a >> 0x10),
                        pu_var6,
                    );
                    in_stack_0000ffdc = unaff_ss;
                    if (pu_var6 == &dos_alloc_addr_1050_0002) {
                        return;
                    }
                }
                _local_16 = struct_ops::process_struct_1010_20ba(
                    ctx.g_struct_var_1050_0ed0,
                    CONCAT22(in_stack_0000ffdc, 6),
                );
                local_c = 0x1a0;
                while {
                    u_var7 = IsDlgButtonChecked16(local_c, (i_var10 + 6));
                    if (u_var7 == 1) {
                      // u_var13 = (_local_16  >> 0x10);
                        i_var11 = _local_16;
                        (i_var11 + (i_var11 + 0x56) * 2 + 0x4e) = local_c;
                        pi_var1 = (i_var11 + 0x56);
                        unsafe {
                            *pi_var1 = *pi_var1 + 1;
                        }
                    }
                    local_c = local_c + 1;
                    local_c < 0x1b5
                } {}
                u_var4 = (i_var10 + 0x92);
                _local_a = (_local_a & 0xffff0000 | u_var4);
                u_var3 = (i_var10 + 0x8e);
                (u_var3 + 0x28) = u_var4;
                PostMessage16(0, 200, 0x111, ctx.g_h_window);
                param_3 = 1;
            }
        }
        post_win_msg_1040_7b3c(
            i_var10,
            (param_1 >> 0x10),
            param_2,
            (param_2 >> 0x10),
            param_3,
        );
    }
    return;
}

pub fn pass1_1040_1ab0(param_1: i32, param_2: Vec<u8>, param_3: Vec<u8>, param_2_00: Vec<u8>) {
    let mut local_6: u32;

    local_6 = 0;
    if (param_2_00 == 0x1831) {
        (param_1 + 0x92) = 1;
        (param_1 + 0x94) = 1;
        dialog::check_dialog_func_1040_1b8a(param_1, param_2);
    } else {
        local_6 = post_win_msg_1040_7b3c(param_1, CONCAT22(param_3, param_2), param_2_00);
    }
    return local_6;
}

pub unsafe fn win_fn_1040_12bc(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let unaff_ss: String;
    let l_param: LPARAM;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];

    win_gui_func_1040_78e2(param_1);
  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    u_var1 = (i_var3 + 0x8e);
    string_fn_1000_3f9c(
        local_54,
        unaff_ss,
        s__u_1050_5cd4,
        &ctx.g_alloc_addr_1050_1050,
        *(u_var1 + 10),
    );
    HVar2 = GetDlgItem16(0xfd2, (i_var3 + 6));
    SendMessage16(CONCAT22(unaff_ss, local_54), 0, 0xc, HVar2);
    SetFocus16(HVar2);
    SendMessage16(-0x10000, 0, 0x401, HVar2);
    window::move_window_1040_826c(param_1, u_var4, 0xffff, 0xffff);
    l_param = load_string_1010_847e(
        ctx.g_struct_73_1050_14cc,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    HVar2 = GetDlgItem16((s_vrpal_bmp_1050_183a + 5), (i_var3 + 6));
    send_msg_1040_1696(i_var3, u_var4, HVar2);
    SendMessage16(l_param, 0xffff, 0x40d, HVar2);
    HVar2 = GetDlgItem16((s_vrpal_bmp_1050_183a + 4), (i_var3 + 6));
    send_msg_1040_1696(i_var3, u_var4, HVar2);
    SendMessage16(l_param, 0xffff, 0x40d, HVar2);
    ShowWindow16(5, (i_var3 + 6));
    return;
}

pub unsafe fn pass1_1040_109c(param_1: i32, param_2: Vec<u8>, param_3: Vec<u8>, param_3_00: Vec<u8>) {
    let mut u_var1: u32;
    let mut b_var2: bool;
    let mut i_var3: i32;
    let ppVar4: &mut Struct2551;
    let mut in_stack_0000fff2: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    b_var2 = false;
    if (param_3_00 == 0x1c1) {
        (param_1 + 0x96) = 2;
        b_var2 = true;
    } else {
        if (param_3_00 == 0x1c2) {
            (param_1 + 0x96) = 1;
            b_var2 = true;
        } else {
            if (param_3_00 != 0x1830) {
                post_win_msg_1040_7b3c(param_1, param_2, param_3, param_3_00);
                return;
            }
            ppVar4 = struct_ops::process_struct_1010_20ba(
                ctx.g_struct_var_1050_0ed0,
                CONCAT22(in_stack_0000fff2, 0x32),
            );
            u_var1 = (param_1 + 0x92);
            i_var3 = win_gui_fn_1010_79aa(ppVar4, 0xfb6, (u_var1 + 6));
            if (i_var3 == 0) {
                u_var1 = (param_1 + 0x92);
                u_var1 = (u_var1 + 6);
                window_msg_func_1010_7300(ppVar4, 0, 0, 0xc, u_var1, (u_var1 >> 0x10));
            }
        }
    }
    if (b_var2) {
        u_var1 = (param_1 + 0x8e);
        (u_var1 + 10) = (param_1 + 0x96);
    }
    return;
}

pub unsafe fn win_fn_1040_07dc(param_1: i32, param_2: u16, param_3: u16, param_4: u16, param_5: i32) {
    let pp_var1: fn();
    let u_var2: u8;
    let i_var3: u16;
    let b_var4: bool;
    let extraout_var: u32;


    let mut u_var6: i32;

    let mut unaff_ss: u16;
    let pa_var7: &mut Struct434;
    let pu_var8: &mut u32;
    let u_var9: u8;
    let u_var10: u8;
    let u_var12: u8;
    let mut local_810: u32;
    let mut local_80c: u16;
    let mut local_80a: u16;
    let mut local_808: u16;
    let mut local_806: [u8; 1024];
    let mut local_406: u32;
    let mut local_6: u32;
    let mut u_var5: u32;
    let u_var11: u8;

    local_6 = 0;
    u_var9 = param_1;
    u_var10 = param_2_00;
    u_var11 = unaff_ss;
    u_var12 = (unaff_ss >> 8);
    if (param_3 == 0x73) {
        window::enable_win_1040_0acc(param_1, u_var10, 0);
        u_var6 = ctx.dx_reg;
        u_var2 = string_fn_1008_5fd8(0x7d0, -0x2f);
        u_var5 = CONCAT31(extraout_var, u_var2);
        local_406 = u_var5 & 0xffff | u_var6 << 0x10;
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT13(u_var12, CONCAT12(u_var11, local_806)),
            1999,
        );
        i_var3 = MessageBox16(
            0x34,
            CONCAT13(u_var12, CONCAT12(u_var11, local_806)),
            CONCAT13((u_var6 >> 8), CONCAT12(u_var6, u_var5)),
            ctx.g_h_window,
        );
        error_check_1000_17ce((u_var5 & 0xffff | u_var6 << 0x10));
        if (i_var3 == 6) {
            b_var4 = PostMessage16(0, 0xcb, 0x111, ctx.g_h_window);
            post_win_msg_1040_7b3c(u_var9, param_2_00, param_2, param_4, 1);
            local_6 = CONCAT22(ctx.dx_reg, b_var4);
        }
    } else {
        if (param_3 < 0x74) {
            if (param_3 == 0x6e) {
                (ctx.g_struct_112_001 + 0xae) = 0x99;
                pu_var8 = pass1_1038_af40(ctx.g_struct_112_001, *(param_1 + 6), 2);
                unsafe {
                    pp_var1 = (*pu_var8 + 0x3c);
                }
                (**pp_var1)(&PTR_LOOP_1050_1038, pu_var8, (pu_var8 >> 0x10));
                SetFocus16((param_1 + 6));
                return;
            }
            if (0x6e < param_3) {
                // LAB_1040_09f9:
                post_win_msg_1040_7b3c(u_var9, u_var10, param_2, param_4, param_3);
                return;
            }
            if (param_3 == 0x2) {
                // LAB_1040_09b4:
                post_win_msg_1040_7b3c(u_var9, u_var10, 0, 0, 2);
                PostMessage16(0, 0xee, 0x111, ctx.g_h_window);
                return;
            }
            if (param_3 != 'd') {}
            // goto LAB_1040_09f9;
            PostMessage16(0, 100, 0x111, ctx.g_h_window);
            local_810._0_2_ = 0;
            // goto LAB_1040_0821;
        }
        if (param_3 != 0x74) {
            if (param_3 == 0xee) {}
            // goto LAB_1040_09b4;
            if (param_3 == 0x13d) {
                window::enable_win_1040_0acc(param_1, u_var10, 1);
                return;
            }
            // goto LAB_1040_09f9;
        }
        window::enable_win_1040_0acc(param_1, u_var10, 0);
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT13(u_var12, CONCAT12(u_var11, &local_406)),
            0x756,
        );
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT13(u_var12, CONCAT12(u_var11, local_806)),
            0x757,
        );
        i_var3 = MessageBox16(
            0x34,
            CONCAT13(u_var12, CONCAT12(u_var11, &local_406)),
            CONCAT13(u_var12, CONCAT12(u_var11, local_806)),
            ctx.g_h_window,
        );
        if (i_var3 == 6) {
            b_var4 = PostMessage16(0, 0x7a, 0x111, ctx.g_h_window);
            post_win_msg_1040_7b3c(u_var9, param_2_00, param_2, param_4, 1);
            local_6 = CONCAT22(ctx.dx_reg, b_var4);
            pa_var7 =
                struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_810, 2));
            process_struct_1010_60fa(pa_var7);
        }
    }
    local_810._0_2_ = 1;
    // LAB_1040_0821:
    window::enable_win_1040_0acc(u_var9, param_2_00, local_810);
    return;
}

pub unsafe fn win_fn_1040_0000(param_1: &mut Struct20) {
    let mut i_var1: i32;
    let p_uvar2: &mut u16;
    let mut u_var3: u16;


    let struct_a: &mut Struct199;
    let mut unaff_si: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut u_var6: u32;
    let pu_var7: Vec<u8>;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    // Segment:    9
    // Offset:     0006f820
    // Length:     d974
    // Min Alloc:  d974
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    win_gui_func_1040_78e2(param_1);
    local_4 = 8;
    local_a = 0;
    struct_a = ctx.dx_reg;
    while (
        i_var4 = param_1,
      // u_var5 = (param_1  >> 0x10),
        local_a < local_4,
    ) {
        i_var1 = local_a * 0xe;
        local_24 = (i_var1 + 0x5c60);
        local_22 = (i_var1 + 0x5c62);
        local_20 = 1;
        local_1e = 1;
        u_var3 = (i_var4 + 6);
        pu_var2 = &local_24;
        MapDialogRect16(CONCAT22(pu_var2, unaff_cs), unaff_ss);
        unaff_cs = 0x1000;
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | pu_var2) == 0) {
            u_var6 = 0;
        } else {
            unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            u_var6 = win_fn_1008_3bd6(
                pu_var2,
                struct_a,
                1,
                CONCAT22(local_24, local_22),
                0x1030104,
                CONCAT22((i_var1 + 0x5c64), 0x102),
                CONCAT22(u_var3, (i_var4 + 6)),
            );
        }
        _local_8 = u_var6;
        window::enable_window_1040_0558(i_var4, u_var5, local_a);
        local_a = local_a + 1;
        struct_a = ctx.dx_reg;
    }
    window::move_window_1040_826c(i_var4, u_var5, 0xffff, 0xffff);
    _local_12 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x48));
  // u_var3 = (_local_12  >> 0x10);
    local_c = (_local_12 + 10);
    local_e = (_local_12 + 0xc);
    GetWindowRect16(CONCAT22(&local_1a, 0x1010), unaff_ss);
    local_1c = local_16 - local_1a;
    local_1a = (local_c / 2 - local_1c) - 3;
    if (local_1a < 0) {
        local_1a = 0;
    }
    SetWindowPos16(0x41, 0, 0, local_18, local_1a, 0, (i_var4 + 6));
    pu_var7 = pass1_1038_af40(ctx.g_struct_112_001, *(i_var4 + 6), 0x17);
    (i_var4 + 0x96) = pu_var7;
    (i_var4 + 0x98) = (pu_var7 >> 0x10);
    u_var3 = mci_send_command_1008_5c7c(ctx.g_struct_1050_02a0, 0x9e0001);
    (i_var4 + 0x8c) = u_var3;
    return;
}

pub unsafe fn win_fn_1038_ec16(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let pu_var3: &mut u16;
    let struct_a: &mut Struct199;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut unaff_si: u16;
    let mut u_var6: u16;
    let unaff_ss: HWND16;
    let mut u_var7: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    _local_6 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x2b));
    ctx.g_u16_ptr_1050_5f2e = (_local_6 >> 0x10);
    local_8 = return3_1010_0892();
    if (ctx.g_struct_ptr_1 == 0) {
        g_struct_ptr_1 = struct_fn_1000_160a();
    } else {
    }
    _local_1a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, g_struct_ptr_1);
    u_var2 = alloc_mem_1000_1708(
        (local_8 + 2) * 4,
        0,
        1,
        g_struct_ptr_1,
        ctx.g_u16_ptr_1050_5f2e,
    );
  // u_var6 = (param_1  >> 0x10);
    i_var4 = param_1;
    (i_var4 + 0x8e) = u_var2;
    (i_var4 + 0x90) = ctx.g_u16_ptr_1050_5f2e;
    local_a = 1;
    while (local_a <= local_8) {
      // _local_1a = pass1_1010_0932(_local_6, (_local_6  >> 0x10), local_a);
      // struct_a = (_local_1a  >> 0x10);
        local_22 = *_local_1a;
        local_20 = (_local_1a + 2);
        local_1e = 1;
        local_1c = 1;
        u_var2 = (i_var4 + 6);
        pu_var3 = &local_22;
        MapDialogRect16(CONCAT22(pu_var3, 0x1010), unaff_ss);
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | pu_var3) == 0) {
            u_var1 = (i_var4 + 0x8e);
            (u_var1 + local_a * 4) = 0;
        } else {
            u_var7 = win_fn_1008_3bd6(
                pu_var3,
                struct_a,
                0,
                CONCAT22(local_22, local_20),
                0x1000101,
                CONCAT22((_local_1a + 4), 0xff),
                CONCAT22(u_var2, (i_var4 + 6)),
            );
            u_var1 = (i_var4 + 0x8e);
          // u_var2 = (u_var1  >> 0x10);
            i_var5 = u_var1;
            (i_var5 + local_a * 4) = u_var7;
            (i_var5 + local_a * 4 + 2) = (u_var7 >> 0x10);
        }
        u_var1 = (i_var4 + 0x8e);
      // u_var2 = (u_var1  >> 0x10);
        i_var5 = u_var1;
        if ((i_var5 + local_a * 4) != 0) {
            u_var1 = (i_var5 + local_a * 4);
            window::enable_window_1040_9234(u_var1, (u_var1 >> 0x10), (_local_1a + 6));
        }
        local_a = local_a + 1;
    }
    window::move_window_1040_826c(param_1, u_var6, 0xff, 0xffff);
    ShowWindow16(5, (i_var4 + 6));
    return;
}

pub unsafe fn win_fn_1038_e348(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let pu_var3: &mut u16;
    let struct_a: &mut Struct199;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut unaff_si: u16;
    let mut u_var6: u16;
    let unaff_ss: HWND16;
    let mut u_var7: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    _local_6 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x2b));
    ctx.g_u16_ptr_1050_5f2e = (_local_6 >> 0x10);
    local_8 = return3_1010_088c();
    if (ctx.g_struct_ptr_1 == 0) {
        g_struct_ptr_1 = struct_fn_1000_160a();
    } else {
    }
    _local_1a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, g_struct_ptr_1);
    u_var2 = alloc_mem_1000_1708(
        (local_8 + 2) * 4,
        0,
        1,
        g_struct_ptr_1,
        ctx.g_u16_ptr_1050_5f2e,
    );
  // u_var6 = (param_1  >> 0x10);
    i_var4 = param_1;
    (i_var4 + 0x8e) = u_var2;
    (i_var4 + 0x90) = ctx.g_u16_ptr_1050_5f2e;
    local_a = 1;
    while (local_a <= local_8) {
      // _local_1a = pass1_1010_091e(_local_6, (_local_6  >> 0x10), local_a);
      // struct_a = (_local_1a  >> 0x10);
        local_22 = *_local_1a;
        local_20 = (_local_1a + 2);
        local_1e = 1;
        local_1c = 1;
        u_var2 = (i_var4 + 6);
        pu_var3 = &local_22;
        MapDialogRect16(CONCAT22(pu_var3, 0x1010), unaff_ss);
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | pu_var3) == 0) {
            u_var1 = (i_var4 + 0x8e);
            (u_var1 + local_a * 4) = 0;
        } else {
            u_var7 = win_fn_1008_3bd6(
                pu_var3,
                struct_a,
                0,
                CONCAT22(local_22, local_20),
                0x1000101,
                CONCAT22((_local_1a + 4), 0xff),
                CONCAT22(u_var2, (i_var4 + 6)),
            );
            u_var1 = (i_var4 + 0x8e);
          // u_var2 = (u_var1  >> 0x10);
            i_var5 = u_var1;
            (i_var5 + local_a * 4) = u_var7;
            (i_var5 + local_a * 4 + 2) = (u_var7 >> 0x10);
        }
        u_var1 = (i_var4 + 0x8e);
      // u_var2 = (u_var1  >> 0x10);
        i_var5 = u_var1;
        if ((i_var5 + local_a * 4) != 0) {
            u_var1 = (i_var5 + local_a * 4);
            window::enable_window_1040_9234(u_var1, (u_var1 >> 0x10), (_local_1a + 6));
        }
        local_a = local_a + 1;
    }
    window::move_window_1040_826c(param_1, u_var6, 0xff, 0xffff);
    ShowWindow16(5, (i_var4 + 6));
    return;
}

pub fn gui_window_func_1038_e19a(param_1: &mut Struct20) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let local_6: &mut Struct24;

    win_gui_func_1040_78e2(param_1);
  // u_var2 = (param_1  >> 0x10);
    i_var1 = param_1;
    CheckRadioButton16(0x1807, 0x1807, 0x1807, (i_var1 + 6));
    window::move_window_1040_826c(i_var1, u_var2, 200, 200);
    ShowWindow16(5, (i_var1 + 6));
    return;
}

pub fn pass1_1038_e03e(param_1: Vec<u8>) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var4 = (param_1  >> 0x10);
    u_var2 = return_10_1010_0886();
    local_6 = 1;
    while (local_6 <= u_var2) {
        u_var1 = (param_1 + 0x92);
      // u_var6 = pass1_1010_08e2(u_var1, (u_var1  >> 0x10), local_6);
        u_var1 = (param_1 + 0x96);
      // u_var5 = (u_var1  >> 0x10);
        i_var3 = u_var1;
        if ((i_var3 + local_6 * 4) != 0) {
            u_var1 = (i_var3 + local_6 * 4);
            window::enable_window_1040_9234(u_var1, (u_var1 >> 0x10), (u_var6 + 6));
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub unsafe fn win_fn_1038_d8ae(param_1: &mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let pu_var3: &mut u16;

    let struct_a: &mut Struct199;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let unaff_ss: HWND16;
    let mut u_var7: u32;
    let mut in_stack_0000ffd0: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = LoadCursor16(0x7f02, 0);
    local_6 = SetCursor16(local_4);
    win_gui_func_1040_78e2(param_1);
  // u_var6 = (param_1  >> 0x10);
    i_var4 = param_1;
    ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
    local_8 = return_10_1010_0886();
    if (ctx.g_struct_ptr_1 == 0) {
        g_struct_ptr_1 = struct_fn_1000_160a();
    } else {
    }
    _local_1e = CONCAT22(ctx.g_u16_ptr_1050_5f2e, g_struct_ptr_1);
    u_var2 = alloc_mem_1000_1708(
        (local_8 + 2) * 4,
        0,
        1,
        g_struct_ptr_1,
        ctx.g_u16_ptr_1050_5f2e,
    );
    (i_var4 + 0x96) = u_var2;
    (i_var4 + 0x98) = ctx.g_u16_ptr_1050_5f2e;
    local_a = 1;
    while (local_a <= local_8) {
        u_var1 = (i_var4 + 0x92);
      // _local_1e = pass1_1010_08e2(u_var1, (u_var1  >> 0x10), local_a);
      // struct_a = (_local_1e  >> 0x10);
        local_26 = *_local_1e;
        local_24 = (_local_1e + 2);
        local_22 = 1;
        local_20 = 1;
        u_var2 = (i_var4 + 6);
        pu_var3 = &local_26;
        MapDialogRect16(CONCAT22(pu_var3, 0x1010), unaff_ss);
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | pu_var3) == 0) {
            u_var1 = (i_var4 + 0x96);
            (u_var1 + local_a * 4) = 0;
        } else {
            u_var7 = win_fn_1008_3bd6(
                pu_var3,
                struct_a,
                0,
                CONCAT22(local_26, local_24),
                0x1000101,
                CONCAT22((_local_1e + 4), 0xff),
                CONCAT22(u_var2, (i_var4 + 6)),
            );
            u_var1 = (i_var4 + 0x96);
          // u_var2 = (u_var1  >> 0x10);
            i_var5 = u_var1;
            (i_var5 + local_a * 4) = u_var7;
            (i_var5 + local_a * 4 + 2) = (u_var7 >> 0x10);
        }
        u_var1 = (i_var4 + 0x96);
      // u_var2 = (u_var1  >> 0x10);
        i_var5 = u_var1;
        if ((i_var5 + local_a * 4) != 0) {
            u_var1 = (i_var5 + local_a * 4);
            (u_var1 + 0x3e) = 1;
            u_var1 = (i_var4 + 0x96);
            u_var1 = (u_var1 + local_a * 4);
            window::enable_window_1040_9234(u_var1, (u_var1 >> 0x10), (_local_1e + 6));
        }
        local_a = local_a + 1;
    }
    _local_e = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(in_stack_0000ffd0, 2));
    SetWindowText16((_local_e + 0x68), (i_var4 + 6));
    ShowWindow16(5, (i_var4 + 6));
    SetCursor16(local_6);
    return;
}

pub fn win_fn_1038_d400(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let HVar1: HWND16;
    let BVar2: bool;




    let mut unaff_ss: u16;
    let ppVar3: &mut Struct2551;
    let WVar4: WPARAM16;
    let mut u_var5: u16;
    let i32_var6: u16;
    let mut u_var7: u16;
    let mut in_stack_0000ffe6: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 4];
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    local_8 = 0;
    match (param_2) {
        0x145 => {
            HVar1 = GetDlgItem16(0x146, (param_1 + 6));
            EnableWindow16(1, HVar1);
            local_6 = 0x13f0647;
            u_var7 = 0x1f1;
            // goto LAB_1038_d490;
        }
        0x146 => unsafe {
            local_6 = 0x1400648;
            pass1_1008_941a(CONCAT22(unaff_ss, local_c), 1, 0xc4);
            mci_send_command_1008_5c9e(ctx.g_struct_1050_02a0, CONCAT22(unaff_ss, local_c));
            u_var7 = 0x86;
            ppVar3 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x860009);
            pass1_1010_6604(ppVar3, u_var7);
            HVar1 = GetDlgItem16(0x145, (param_1 + 6));
            BVar2 = EnableWindow16(0, HVar1);
            HVar1 = (param_1 + 6);
            u_var5 = 0xc;
            i32_var6 = 0x13f;
            WVar4 = 0;
            load_string_1010_847e(
                ctx.g_struct_73_1050_14cc,
                (ctx.g_struct_73_1050_14cc >> 0x10),
                0x649,
            );
            SendDlgItemMessage16(
                CONCAT22(ctx.dx_reg, BVar2),
                WVar4,
                u_var5,
                i32_var6,
                HVar1,
            );
            HVar1 = GetDlgItem16(0x146, (param_1 + 6));
            BVar2 = EnableWindow16(0, HVar1);
            pass1_1010_659a(ppVar3, 0x86);
            if (BVar2 == 0) {
                HVar1 = GetDlgItem16(0x14a, (param_1 + 6));
                BVar2 = EnableWindow16(0, HVar1);
                HVar1 = (param_1 + 6);
                u_var5 = 0xc;
                i32_var6 = 0x144;
                WVar4 = 0;
                load_string_1010_847e(
                    ctx.g_struct_73_1050_14cc,
                    (ctx.g_struct_73_1050_14cc >> 0x10),
                    0x531,
                );
                SendDlgItemMessage16(
                    CONCAT22(ctx.dx_reg, BVar2),
                    WVar4,
                    u_var5,
                    i32_var6,
                    HVar1,
                );
            }
            ppVar3 = struct_ops::process_struct_1010_20ba(
                ctx.g_struct_var_1050_0ed0,
                CONCAT22(in_stack_0000ffe6, 2),
            );
            if ((ppVar3 + 0x20) != 0) {
                PostMessage16(0, 0xaf, 0x111, ctx.g_h_window);
            }
        }
        0x147 => {
            HVar1 = GetDlgItem16(0x148, (param_1 + 6));
            EnableWindow16(1, HVar1);
            local_6 = 0x1410647;
            u_var7 = 0x1f5;
            // goto LAB_1038_d490;
        }
        0x148 => unsafe {
            HVar1 = GetDlgItem16(0x149, (param_1 + 6));
            EnableWindow16(1, HVar1);
            local_6 = 0x1420647;
            u_var7 = 0x1f2;
            // LAB_1038_d490:
            mci_send_cmd_1008_5c5c(ctx.g_struct_1050_02a0, u_var7)
        }
        0x149 => {
            local_6 = 0x1430648;
            PostMessage16(0, 0xb8, 0x111, ctx.g_h_window);
            DestroyWindow16((param_1 + 6))
        }
        0x14a => {
            HVar1 = GetDlgItem16(0x145, (param_1 + 6));
            BVar2 = EnableWindow16(1, HVar1);
            HVar1 = (param_1 + 6);
            u_var5 = 0xc;
            i32_var6 = 0x140;
            WVar4 = 0;
            load_string_1010_847e(
                ctx.g_struct_73_1050_14cc,
                (ctx.g_struct_73_1050_14cc >> 0x10),
                0x649,
            );
            SendDlgItemMessage16(CONCAT22(ctx.dx_reg, BVar2), WVar4, u_var5, i32_var6, HVar1)
        }
        0x14b => {
            HVar1 = GetDlgItem16(0x147, (param_1 + 6));
            EnableWindow16(1, HVar1)
        }
        _ => {
            post_win_msg_1040_7b3c(param_1, param_2_00, param_3, param_2, param_2);
            return;
        }
    }
    if (((local_6 != 0) && (local_6 != 0)) && (BVar2 = IsWindow16((param_1 + 6)), BVar2 != 0))
    {
        HVar1 = (param_1 + 6);
        WVar4 = 0;
        u_var5 = 0xc;
        load_string_1010_847e(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            local_6,
        );
        SendDlgItemMessage16(
            CONCAT22(ctx.dx_reg, BVar2),
            WVar4,
            u_var5,
            local_6,
            HVar1,
        );
    }
    if (local_8 != 0) {
        PostMessage16(0, local_8, 0x111, ctx.g_h_window);
    }
    return;
}

pub unsafe fn win_fn_1038_d2a2(param_1: &mut Struct20) {
    let l_param: LPARAM;
    let pu_var1: &mut u16;
    let HVar2: HWND16;

    let struct_a: &mut Struct199;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let unaff_ss: HWND16;
    let w_param: WPARAM16;
    let mut msg: u16;
    let id: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut local_22: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
    win_gui_func_1040_78e2(param_1);
    local_4 = 7;
    local_a = 0;
    struct_a = ctx.dx_reg;
    while (
        i_var4 = param_1,
      // u_var5 = (param_1  >> 0x10),
        local_a < local_4,
    ) {
        i_var3 = local_a * 0xc;
        local_16 = (i_var3 + 0x5c0c);
        local_14 = (i_var3 + 0x5c0e);
        local_12 = 1;
        local_10 = 1;
        u_var7 = (i_var4 + 6);
        pu_var1 = &local_16;
        MapDialogRect16(CONCAT22(pu_var1, u_var6), unaff_ss);
        u_var6 = 0x1000;
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | pu_var1) == 0) {
            _local_8 = 0;
        } else {
            u_var6 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            _local_8 = win_fn_1008_3bd6(
                pu_var1,
                struct_a,
                1,
                CONCAT22(local_16, local_14),
                0x1030104,
                CONCAT22((i_var3 + 0x5c10), 0x102),
                CONCAT22(u_var7, (i_var4 + 6)),
            );
        }
      // struct_a = (_local_8  >> 0x10);
        if ((local_a * 0xc + 0x5c12) == 0) {
            u_var6 = SUB42(offset, 0);
            EnableWindow16(0, (_local_8 + 0x18));
        }
        local_a = local_a + 1;
    }
    u_var8 = 0x86;
    _local_e = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x860009);
    i_var3 = pass1_1010_659a(_local_e, u_var8);
    if (i_var3 == 0) {
        HVar2 = GetDlgItem16(0x14a, (i_var4 + 6));
        EnableWindow16(0, HVar2);
        HVar2 = (i_var4 + 6);
        msg = 0xc;
        id = 0x144;
        w_param = 0;
        l_param = load_string_1010_847e(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x531,
        );
        SendDlgItemMessage16(l_param, w_param, msg, id, HVar2);
    }
    window::move_window_1040_826c(i_var4, u_var5, 0xffff, 0xffff);
    ShowWindow16(5, (i_var4 + 6));
    u_var6 = mci_send_command_1008_5c7c(ctx.g_struct_1050_02a0, 0x9a0001);
    (i_var4 + 0x8c) = u_var6;
    return;
}

pub fn win_func_1038_cd88(param_1: &mut Struct20) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    win_gui_func_1040_78e2(param_1);
    window::move_window_1040_826c(param_1, 0xffffffff);
  // u_var2 = (param_1  >> 0x10);
    i_var1 = param_1;
    ShowWindow16(5, (i_var1 + 6));
    (i_var1 + 0x92) = 1;
    process_win_msg_1008_9510();
    DestroyWindow16((i_var1 + 6));
    return;
}

pub fn gui_window_func_1038_c89c(param_1: &mut Struct22) {
    let mut u_var1: u32;
    let h_wnd: HWND16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let local_10: &mut Struct22;
    let mut h_wnd_2: u16;
    let temp_5f4a343e5a: &mut Struct23;

    win_gui_func_1040_78e2(param_1);
  // u_var3 = (param_1  >> 0x10);
    i_var2 = param_1;
    CheckRadioButton16(0xfac, 0xfad, 0xfac, (i_var2 + 6));
    u_var1 = (i_var2 + 0x8e);
    (u_var1 + 10) = 1;
    u_var1 = (i_var2 + 0x8e);
    temp_5f4a343e5a = (u_var1 + 0x12);
    if (temp_5f4a343e5a == &PTR_DAT_0005_0000_1050_0004) {
        // LAB_1038_c8da:
        h_wnd = GetDlgItem16(0xfce, (i_var2 + 6));
        if (h_wnd != 0) {
            EnableWindow16(1, h_wnd);
        }
        h_wnd_2 = GetDlgItem16(1, (i_var2 + 6));
        if (h_wnd_2 == 0) {}
        // goto LAB_1038_c93c;
        local_10 = 0x0;
    } else {
        if (((temp_5f4a343e5a + -5) < 1) || (SBORROW2((temp_5f4a343e5a + -5), 1))) {}
        // goto LAB_1038_c93c;
        if (temp_5f4a343e5a != &BYTE_1050_0008 && 0 < (temp_5f4a343e5a + -7)) {
            if (temp_5f4a343e5a != &BYTE_1050_0009) {}
            // goto LAB_1038_c93c;
            // goto LAB_1038_c8da;
        }
        h_wnd_2 = GetDlgItem16(0xfce, (i_var2 + 6));
        if (h_wnd_2 == 0) {}
        // goto LAB_1038_c93c;
        local_10 = (&ctx.PTR_LOOP_1050_0000 + 1);
    }
    EnableWindow16(local_10, h_wnd_2);
    // LAB_1038_c93c:
    window::move_window_1040_826c(param_1, 200, 0);
    ShowWindow16(5, (i_var2 + 6));
    return;
}

pub unsafe fn win_fn_1038_c672(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut unaff_ss: u16;
    let mut b: u16;
    let mut local_404: [u8; 1026];

  // b = (ctx.g_struct_73_1050_14cc  >> 0x10);
    if (param_3 == 0x17d) {
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            b,
            0x3ff,
            CONCAT22(unaff_ss, local_404),
            0x453,
        );
        MessageBox16(
            0x30,
            (param_1 + 0x92),
            CONCAT22(unaff_ss, local_404),
            (param_1 + 6),
        );
    } else {
        if (param_3 != 0x17e) {
            post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3);
            return;
        }
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            b,
            0x3ff,
            CONCAT22(unaff_ss, local_404),
            0x454,
        );
        MessageBox16(
            0x30,
            (param_1 + 0x92),
            CONCAT22(unaff_ss, local_404),
            (param_1 + 6),
        );
        pass1_1008_e164((param_1 + 0x8e));
    }
    PostMessage16(0, 2, 0x111, (param_1 + 6));
    return;
}

pub unsafe fn win_fn_1038_c58e(param_1: u32) {
    let mut i_var1: i32;
    let mut unaff_si: u16;
    let mut u_var2: i32;
    let mut unaff_ss: u16;
    let mut local_816: u16;
    let mut local_814: u16;
    let mut local_80e: i32;
    let mut uStack2060: u16;
    let mut uStack2058: u16;
    let mut uStack2052: u16;
    let HStack2050: HWND16;
    let mut local_40c: [u8; 1026];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 2));
    local_a = (_local_6 + 0x68);
  // u_var2 = (param_1  >> 0x10);
    i_var1 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_ss, local_40c), (i_var1 + 6));
    wsprintf16(
        &local_80e,
        CONCAT22(local_40c, unaff_ss),
        CONCAT22(local_a, unaff_ss),
    );
    SetWindowText16(CONCAT22(unaff_ss, &local_80e), (i_var1 + 6));
    local_80e = u_var2;
    pass1_1008_e038(
        (i_var1 + 0x8e) & 0xffff | (i_var1 + 0x96) << 0x10,
        param_1 & 0xffff0000 | u_var2,
        param_1 & 0xffff0000 | (i_var1 + 0x96),
    );
    local_80e = 0x7d6;
  // local_816 = (ctx.g_struct_73_1050_14cc  >> 0x10);
    load_string_1010_84e0(
        ctx.g_struct_73_1050_14cc,
        local_816,
        0x400,
        CONCAT22(unaff_ss, &local_80e),
        0x7d6,
    );
  // local_80e = (*(i_var1 + 0x92)  >> 0x10);
    local_814 = *CONCAT22(0x400, local_816);
    wsprintf16(
        local_40c,
        CONCAT22(&local_80e, unaff_ss),
        CONCAT22(local_814, unaff_ss),
    );
    HStack2050 = (i_var1 + 6);
    uStack2052 = 0x17f;
    uStack2058 = SUB42(offset, 0);
    uStack2060 = 0xc66c;
    SetDlgItemText16(CONCAT22(unaff_ss, local_40c), 0x17f, HStack2050);
    return;
}

pub fn set_focus_1038_c558(param_1: &mut Struct20) {
    let mut u_var1: u16;

    win_gui_func_1040_78e2(param_1);
    window::move_window_1040_826c(param_1, 0xffffffff);
  // u_var1 = (param_1  >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub fn set_focus_1038_c044(param_1: &mut Struct20) {
    let mut u_var1: u16;

    win_gui_func_1040_78e2(param_1);
    window::move_window_1040_826c(param_1, 0xffffffff);
  // u_var1 = (param_1  >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub unsafe fn win_fn_1038_c07a(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut i_var1: i32;
    let unaff_ss: String;
    let u_var2: u8;
    let mut local_70c: [u8; 512];
    let mut local_50c: [u8; 256];
    let mut local_40c: [u8; 1026];
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    send_win_msg_1038_c228(param_1, param_2_00);
    _local_6 = load_string_1010_847e(
        ctx.g_struct_73_1050_14cc,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    u_var2 = (unaff_ss >> 8);
    if param_2 == 0x177 {
        pass1_1008_e05e(
            (param_1 + 0x8e),
            2,
            CONCAT22(param_2_00, (param_1 + 0x19e)),
            CONCAT22(param_2_00, param_1 + 0x9e),
        );
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x200,
            CONCAT22(unaff_ss, local_40c),
            0x451,
        );
        string_fn_1000_3f9c(local_70c, unaff_ss, local_40c, unaff_ss, (param_1 + 0x19e));
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT22(unaff_ss, local_50c),
            0x57b,
        );
        MessageBox16(
            0x30,
            CONCAT13(u_var2, CONCAT12(unaff_ss, local_50c)),
            CONCAT22(unaff_ss, local_70c),
            (param_1 + 6),
        );
    } else {
        if (param_2 != 0x178) {
            if ((param_2 != 0x178) && (param_2 - 0x179 < 2)) {
                window::set_window_pos_1038_c31a(param_1, param_2_00, param_3, param_2, param_2);
                return;
            }
            post_win_msg_1040_7b3c(param_1, param_2_00, param_3, param_2, param_2);
            return;
        }
        _local_a = CONCAT22(param_2_00, param_1 + 0x9e);
        i_var1 = pass1_1008_e10c(
            (param_1 + 0x8e),
            CONCAT22(param_2_00, param_1 + 0x19e),
            CONCAT22(param_2_00, param_1 + 0x9e),
        );
        if (i_var1 == 0) {
            load_string_1010_84e0(
                ctx.g_struct_73_1050_14cc,
                (ctx.g_struct_73_1050_14cc >> 0x10),
                0x3ff,
                CONCAT22(unaff_ss, local_40c),
                0x450,
            );
            load_string_1010_84e0(
                ctx.g_struct_73_1050_14cc,
                (ctx.g_struct_73_1050_14cc >> 0x10),
                0x3ff,
                CONCAT22(unaff_ss, local_50c),
                0x57b,
            );
            MessageBox16(
                0x30,
                CONCAT13(u_var2, CONCAT12(unaff_ss, local_50c)),
                CONCAT22(unaff_ss, local_40c),
                (param_1 + 6),
            );
            return;
        }
        pass1_1008_e01c(
            (param_1 + 0x8e),
            CONCAT22(param_2_00, param_1 + 0x19e),
            _local_a,
        );
        pass1_1038_af40(ctx.g_struct_112_001, *(param_1 + 8), 0x1f);
    }
    PostMessage16(0, 2, 0x111, (param_1 + 6));
    return;
}

pub unsafe fn win_fn_1038_bea4(param_1: u32) {
    let mut u_var1: u32;
    let HVar2: HWND16;

    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let ppVar5: &mut Struct2551;
    let mut l_param: u32;
    let LVar6: LRESULT;
    let in_stack_0000fed2: u8;
    let in_stack_0000fed3: u8;
    let mut local_126: u16;
    let mut local_124: u16;
    let mut local_122: u16;
    let mut local_120: u16;
    let mut local_11e: u32;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_116: u32;
    let mut local_112: u32;
    let mut local_10e: [u8; 130];
    let mut local_8c: [u8; 130];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = struct_ops::process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT13(in_stack_0000fed3, CONCAT12(in_stack_0000fed2, 2)),
    );
    local_a = (_local_6 + 0x68);
  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_ss, local_8c), (i_var3 + 6));
    wsprintf16(
        local_10e,
        CONCAT22(local_8c, unaff_ss),
        CONCAT22(local_a, unaff_ss),
    );
    SetWindowText16(CONCAT22(unaff_ss, local_10e), (i_var3 + 6));
    HVar2 = GetDlgItem16(0x179, (i_var3 + 6));
    (i_var3 + 0x92) = HVar2;
    process_struct_1008_e3ec(
        (i_var3 + 0x8e),
        CONCAT22(unaff_ss, &local_116),
        CONCAT22(unaff_ss, &local_112),
    );
    local_126 = local_112;
  // local_124 = (local_112  >> 0x10);
    win_fn_1038_c374(
        i_var3,
        (param_1 >> 0x10),
        local_126,
        local_124,
        (i_var3 + 0x92),
    );
    ppVar5 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_120, 0x2f));
    u_var1 = (i_var3 + 0x8e);
  // local_126 = (u_var1  >> 0x10);
    l_param = process_struct_1008_e586(u_var1, local_126, (ppVar5 + 0x24));
    SendMessage16(l_param, 0xffff, 0x40d, (i_var3 + 0x92));
    HVar2 = GetDlgItem16(0x17a, (i_var3 + 6));
    (i_var3 + 0x94) = HVar2;
    local_124 = local_116;
  // local_122 = (local_116  >> 0x10);
    win_fn_1038_c374(param_1, u_var4, local_124, local_122, HVar2);
    local_124 = ctx.g_struct_73_1050_14cc;
  // local_122 = (ctx.g_struct_73_1050_14cc  >> 0x10);
    load_string_1010_847e(local_124, local_122, 0x531);
    LVar6 = SendMessage16(CONCAT22(ctx.dx_reg, HVar2), 0, 0x403, (i_var3 + 0x94));
    (i_var3 + 0x9c) = LVar6;
    SendMessage16(ctx.dx_reg, 0xffff, 0x40d, (i_var3 + 0x94));
    HVar2 = GetDlgItem16(0x178, (i_var3 + 6));
    (i_var3 + 0x96) = HVar2;
    HVar2 = GetDlgItem16(0x177, (i_var3 + 6));
    (i_var3 + 0x98) = HVar2;
    HVar2 = GetDlgItem16(0x184, (i_var3 + 6));
    (i_var3 + 0x9a) = HVar2;
    return;
}

pub fn pass1_1038_af34() {
    ctx.g_struct_112_001 = 0;
    return;
}

pub unsafe fn pass1_1038_af40(param_1: &mut Struct112, param_2: Vec<u8>, param_3: u16) -> &mut u16 {
    let pp_var1: fn();
    let mut u_var2: u32;
    let u_var3: u8;
    let extraout_ah: u8;
    let struct_a: &mut Struct199;
    let mut u_var4: i32;
    let mut in_bx: i32;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let pa_var7: &mut Struct68;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut local_8: u16;

    // i_var8 = param_1;
  // u_var9 = (param_1  >> 0x10);
    u_var6 = window::gui_window_func_1038_b72e(param_1, param_3);
  // struct_a = (u_var6  >> 0x10);
    if (u_var6 != 0x0) {}
    // goto LAB_1038_b61f;
    u_var5 = SUB42(&PTR_LOOP_1050_1038, 0);
    PTR_LOOP_1050_5b82 = u_var6;
    match (param_3) {
        1 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x8e, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {
                // LAB_1038_afa0:
                u_var5 = 0x1000;
                pa_var7 = 0x0;
            } else {
                pa_var7 = pass1_1038_9f76((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            }
        }
        2 => {
            process_struct_1000_179c(0x96, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_181c((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        3 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_e99a((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2)
        }
        4 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_c7b8((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2)
        }
        5 => {
            process_struct_1000_179c(0x96, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_23ea((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        6 => {
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_06e8((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2)
        }
        7 => {
            process_struct_1000_179c(0x9c, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_4068((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        8 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x9a, struct_a);
          // u_var4 = (u_var6  >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_b772(in_bx, u_var4, 0, 0, 0, 0, param_2)
        }
        9 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x8e, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_e140((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2)
        }
        10 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x90, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_a33c((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0xb => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x90, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_a494((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0xc => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x90, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_a69a((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0xd => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x90, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_a89e((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0xe => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x94, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_e69a((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0xf => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x94, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_cd06((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x10 => {
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_0bfc((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2)
        }
        0x11 => {
            process_struct_1000_179c(0x9a, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_0e1c(
                (u_var6 & 0xff000000 | CONCAT12((u_var6 >> 0x10), in_bx)),
                0x0,
                0x0,
                param_2,
            );
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x12 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x9a, struct_a);
          // u_var4 = (u_var6  >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_d756(in_bx, u_var4, param_2)
        }
        0x13 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var3 = pass1_1038_cad8(u_var6 & 0xffff0000 | in_bx, param_2);
            pa_var7 = CONCAT22(u_var4, CONCAT11(extraout_ah, u_var3))
        }
        0x14 => {
            process_struct_1000_179c(0xaa, struct_a);
          // u_var4 = (u_var6  >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_1f5a(in_bx, u_var4, param_2)
        }
        0x15 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x8e, struct_a);
          // u_var4 = (u_var6  >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_d242(in_bx, u_var4, param_2)
        }
        0x16 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x9a, struct_a);
          // u_var4 = (u_var6  >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_eeda(in_bx, u_var4, param_2)
        }
        0x17 => {
            process_struct_1000_179c(0x96, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = 0x1018;
            pa_var7 = pass1_1018_5e26((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        // default:
        // goto switchD_1038_b581_caseD_18;
        0x19 => {
            process_struct_1000_179c(0x96, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_1cb4((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x1a => {
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_123e((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2)
        }
        0x1b => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x8e, struct_a);
          // u_var4 = (u_var6  >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_ab82(in_bx, u_var4, param_2)
        }
        0x1c => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
          // u_var4 = (u_var6  >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_e2d0(in_bx, u_var4, param_2)
        }
        0x1d => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
          // u_var4 = (u_var6  >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_eb9e(in_bx, u_var4, param_2)
        }
        0x1e => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x29e, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_bddc((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x1f => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x9a, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_c4a2((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x20 => {
            process_struct_1000_179c(0x29a, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_2ea2((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x21 => {
            process_struct_1000_179c(0xa6, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_3966((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x22 => {
            process_struct_1000_179c(0x9a, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_34a2((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x23 => {
            process_struct_1000_179c(0x9c, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_ac84((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0x25 => {
            process_struct_1000_179c(0xa0, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_ca16((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0x26 => {
            process_struct_1000_179c(0xa2, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_d0f8((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0x27 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0xa0, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
          // u_var4 = (u_var6  >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_88f2((u_var6 & 0xffff0000 | in_bx), param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x28 => {
            process_struct_1000_179c(0x96, struct_a);
          // u_var4 = (u_var6  >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_6402(in_bx, u_var4, param_2)
        }
        0x29 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x98, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_7d10(u_var6 & 0xffff0000 | in_bx, param_2)
        }
        0x2a => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x98, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_8caa((u_var6 & 0xffff0000 | in_bx), param_2);
        }
    }
    (param_3 * 4 + param_1) = pa_var7;
    (param_3 * 4 + param_1 + 2) = (pa_var7 >> 0x10);
    // switchD_1038_b581_caseD_18:
    if ((param_3 * 4 + param_1) != 0) {
        if ((param_1 + 0xae) != 0) {
            u_var2 = (param_3 * 4 + param_1);
            (u_var2 + 0x6e) = (param_1 + 0xae);
        }
        (param_1 + 0xae) = 0;
        u_var2 = (param_3 * 4 + param_1);
        pp_var1 = ((param_3 * 4 + param_1) + 8);
        (**pp_var1)(u_var5, u_var2, (u_var2 >> 0x10));
    }
    // LAB_1038_b61f:
    return CONCAT22((param_3 * 4 + param_1 + 2), (param_3 * 4 + param_1));
}

pub unsafe fn pass1_1038_aeca(param_1: &mut Struct65) -> &mut Struct65 {
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut local_b6: u16;
    let mut local_b4: u16;
    let mut local_5c: [u8; 90];

  // u_var1 = (param_1  >> 0x10);
    (param_1 + 0xac) = 0;
    (param_1 + 0xae) = 0;
    if (ctx.g_struct_112_001 == 0x0) {
        ctx.g_struct_112_001 = param_1;
    }
    pass1_1000_4906(param_1, 0, 0xac);
    cursor::load_cursor_1008_80ee(local_5c, unaff_ss);
    cursor::load_cursor_1040_9854(&local_b6, unaff_ss);
    local_b6 = ctx.s_1_1050_389a;
    local_b4 = &ctx.PTR_LOOP_1050_1008;
    modify_list_1008_8168(CONCAT22(unaff_ss, local_5c));
    return param_1;
}

pub fn pass1_1038_aeaa(param_1: &mut Struct44) -> &mut Struct44 {
    let pu8_var1: Vec<u8>;
    let mut b_var2: u8;
    let mut cVar3: u8;
    let mut u8_var4: u8;
    let mut in_CL: u8;
    let mut in_dx: i32;
    let mut b_var5: u8;
    let mut in_bx: i32;
    let mut bVar6: u8;
    let pu_var7: &mut u16;
    let unaff_bp: &mut u16;
    let unaff_si: Vec<u8>;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut bVar8: bool;
    let mut bVar9: bool;
    let mut uStack00aa: u16;
    let mut uStack00ac: u16;
    let pa_var10: &mut Struct65;
    let mut bStack78: u8;
    let puStack34: Vec<u8>;

    puStack34 = &stack0xfffe;
    pu_var7 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var7 = pu_var7 + -1;
        unsafe {
            *pu_var7 = *unaff_bp;
        }
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    bVar6 = (in_bx >> 8);
    unaff_si[in_bx] = bVar6;
    b_var5 = ((in_dx + 1) >> 8);
    bVar8 = CARRY1(b_var5, bVar6) || CARRY1(b_var5 + bVar6, in_CF);
    u8_var4 = (in_dx + 1);
    pa_var10 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pu8_var1 = unaff_si + in_bx;
    unsafe {
        b_var5 = *pu8_var1;
        b_var2 = *pu8_var1 - u8_var4;
        bVar9 = *pu8_var1 < u8_var4 || b_var2 < bVar8;
        *pu8_var1 = b_var2 - bVar8;
        if (*pu8_var1 != 0
            && (SBORROW1(b_var5, u8_var4) != SBORROW1(b_var2, bVar8)) == (*pu8_var1 < '\0'))
        {
            pu8_var1 = unaff_si;
            bVar8 = CARRY1(*pu8_var1, bVar6) || CARRY1(*pu8_var1 + bVar6, bVar9);
            *pu8_var1 = *pu8_var1 + bVar6 + bVar9;
            b_var5 = bStack78 + in_bx;
            bVar9 = CARRY1(bStack78, in_bx) || CARRY1(b_var5, bVar8);
            bStack78 = b_var5 + bVar8;
            pu8_var1 = unaff_si + -0x4f;
            b_var5 = *pu8_var1;
            b_var2 = *pu8_var1;
            *pu8_var1 = b_var2 + bVar6 + bVar9;
            pu8_var1 = unaff_si + -0x78;
            *pu8_var1 =
                *pu8_var1 + in_CL + (CARRY1(b_var5, bVar6) || CARRY1(b_var2 + bVar6, bVar9));
            uStack00aa = 0;
            uStack00ac = 0;
            if (ctx.g_struct_112_001 == 0) {
                ctx.g_struct_112_001 = CONCAT22(&stack0xbf2a, &stack0xfffe);
            }
            puStack34 = &stack0xfffe;
            pass1_1000_4906(pa_var10, 0, 0xac);
            cursor::load_cursor_1008_80ee();
            cursor::load_cursor_1040_9854();
            modify_list_1008_8168(CONCAT22(unaff_ss, &stack0xbebe));
            return pa_var10;
        }
        if (*pu8_var1 != 0) {
            error_check_1000_17ce(param_1);
        }
    }
    return param_1;
}

pub unsafe fn pass1_1038_ae56(param_1: &mut Struct44) -> &mut Struct44 {
    let pu8_var1: Vec<u8>;
    let pu_var2: &mut u32;
    let pcVar3: String;
    let mut u8_var4: u8;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut cVar7: u8;
    let mut cVar8: u8;
    let mut in_cx: u16;
    let mut bVar9: u8;
    let mut bVar10: u8;
    let mut in_dx: i32;
    let mut bVar12: u8;
    let mut i_var11: i32;
    let mut bVar13: u8;
    let mut in_bx: u16;
    let mut i_var14: i32;
    let pu_var15: Vec<u8>;
    let pu_var16: &mut u16;
    let unaff_bp: &mut u16;
    let pu_var17: Vec<u8>;
    let pu_var18: Vec<u8>;
    let unaff_si: Vec<u8>;
    let mut unaff_DI: i32;
    let unaff_es: Vec<u8>;
    let pu_var19: Vec<u8>;
    let mut unaff_ss: u16;
    let mut b_var20: bool;
    let mut b_var21: bool;
    let mut in_stack_0000007c: u16;
    let mut bStack007d: u8;
    let mut in_stack_0000c741: u32;
    let local_4e: u8;
    let puStack34: Vec<u8>;

    puStack34 = &stack0xfffe;
    pu_var16 = &stack0xfffe;
    pu_var15 = &stack0xfffe;
    pu_var17 = &stack0xfffe;
    cVar8 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var16 = pu_var16 + -1;
        unsafe {
            *pu_var16 = *unaff_bp;
        }
        cVar8 = cVar8 + -1;
        '\0' < cVar8
    } {}
    bVar9 = (in_cx >> 8);
    pu_var19 = &stack0xc73f;
    bVar10 = in_dx;
    b_var20 = CARRY1(bStack007d, bVar10)
        || CARRY1(
            bStack007d + bVar10,
            unaff_si[CONCAT11((in_bx >> 8), 0x40)] < bVar10,
        );
    pu8_var1 = &stack0x4079 + unaff_si;
    bVar12 = (in_dx >> 8);
    unsafe {
        b_var21 = CARRY1(*pu8_var1, bVar12) || CARRY1(*pu8_var1 + bVar12, b_var20);
        *pu8_var1 = *pu8_var1 + bVar12 + b_var20;
        pu8_var1 = unaff_si;
        u8_var4 = *pu8_var1;
        bVar13 = *pu8_var1 + 0x40;
        b_var20 = 0xbf < *pu8_var1 || CARRY1(bVar13, b_var21);
        *pu8_var1 = bVar13 + b_var21;
        cVar8 = in_cx;
        if ((*pu8_var1 == 0)
            || (SCARRY1(u8_var4, '@') != SCARRY1(bVar13, b_var21)) != (*pu8_var1 < '\0'))
        {
            pu8_var1 = unaff_si + 0x3fc8;
            *pu8_var1 = *pu8_var1 + cVar8 + b_var20;
            cVar7 = PTR_LOOP_1050_4080;
            PTR_LOOP_1050_4080._0_1_ = '@';
            i_var14 = CONCAT11(cVar7, 0x40);
            pu8_var1 = unaff_si;
            *pu8_var1 = *pu8_var1 + cVar8 + (unaff_si[0x4040] < bVar10);
            pu8_var1 = unaff_si + i_var14 + 0x10;
            *pu8_var1 = *pu8_var1 + 0x54;
            pu8_var1 = unaff_si + i_var14 + 0x10;
            *pu8_var1 = *pu8_var1 - 0x34;
            pu_var2 = (unaff_si + i_var14 + 0x10);
            u_var5 = *pu_var2;
            *pu_var2 = *pu_var2 + 0x81b6;
            pu_var2 = (unaff_si + i_var14 + 0x10);
            u_var6 = *pu_var2;
            *pu_var2 = *pu_var2 + 0x60ea;
            pu8_var1 = unaff_si + i_var14;
            *pu8_var1 = (*pu8_var1 - (in_dx & 0xff)) - (0x9f15 < u_var6);
            i_var11 = (in_dx & 0xff | (bVar12 + cVar7 + (0x7e49 < u_var5)) << 8) - 1;
            pu8_var1 = unaff_si + i_var14 + 0x10;
            *pu8_var1 = *pu8_var1 + 0x66;
            pu8_var1 = unaff_si + i_var14 + 0x10;
            u8_var4 = *pu8_var1;
            *pu8_var1 = *pu8_var1 - 0x22;
            bVar13 = (i_var11 >> 8);
            if (-1 < *pu8_var1) {
                pu8_var1 = unaff_si + i_var14;
                *pu8_var1 = (*pu8_var1 - i_var11)
                    - (CARRY1(bVar13, bVar9) || CARRY1(bVar13 + bVar9, 0x21 < u8_var4));
                // do
                // {
                //     // WARNING: Do nothing block with infinite loop
                // } while (true);
            }
            pcVar3 = (unaff_DI + 8);
            *pcVar3 = *pcVar3 + bVar13;
          // pu_var15 = (in_stack_0000c741  >> 0x10);
            pu_var19 = unaff_es;
        } else {
            b_var20 = 0xbf < bVar12 || CARRY1(bVar12 + 0x40, b_var20);
            pu8_var1 = unaff_si + 0x4040;
            u8_var4 = *pu8_var1;
            bVar13 = *pu8_var1 - bVar10;
            b_var21 = *pu8_var1 < bVar10 || bVar13 < b_var20;
            *pu8_var1 = bVar13 - b_var20;
            if ((*pu8_var1 == 0)
                || (SBORROW1(u8_var4, bVar10) != SBORROW1(bVar13, b_var20)) != (*pu8_var1 < '\0'))
            {
                if (*pu8_var1 != 0) {
                    error_check_1000_17ce(param_1);
                }
                return param_1;
            }
            pu8_var1 = unaff_si;
            b_var20 = 0xbf < *pu8_var1 || CARRY1(*pu8_var1 + 0x40, b_var21);
            *pu8_var1 = *pu8_var1 + 0x40 + b_var21;
            b_var21 = 0xbf < local_4e || CARRY1(local_4e + 0x40, b_var20);
            local_4e = local_4e + 0x40 + b_var20;
            pu8_var1 = unaff_si + -0x4f;
            u8_var4 = *pu8_var1;
            bVar13 = *pu8_var1;
            *pu8_var1 = bVar13 + 0x40 + b_var21;
            pu8_var1 = unaff_si + -0x78;
            *pu8_var1 = *pu8_var1 + cVar8 + (0xbf < u8_var4 || CARRY1(bVar13 + 0x40, b_var21));
            pu_var18 = &stack0xc72d;
            pu_var17 = &stack0xc72d;
            if (ctx.g_struct_112_001 != 0) {}
            // goto LAB_1038_aeed;
        }
    }
    ctx.g_struct_112_001 = CONCAT22(pu_var19, pu_var15);
    pu_var18 = pu_var17;
    // LAB_1038_aeed:
    puStack34 = &stack0xfffe;
    pass1_1000_4906((pu_var18 + 6), 0, 0xac);
    cursor::load_cursor_1008_80ee();
    cursor::load_cursor_1040_9854();
    (pu_var18 + -0xb4) = ctx.s_1_1050_389a;
    (pu_var18 + -0xb2) = &ctx.PTR_LOOP_1050_1008;
    modify_list_1008_8168(CONCAT22(unaff_ss, pu_var18 + -0x5a));
    return CONCAT22((pu_var18 + 8), (pu_var18 + 6));
}

pub unsafe fn win_fn_1038_a788(param_1: u32, param_2: i32) {
    let hwnd: HWND16;
    let mut u_var1: i32;
    let h_wnd: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let ppVar4: &mut Struct2551;
    let pu_var5: Vec<u8>;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: [u8; 80];

    if (param_2 != 0) {
      // u_var3 = (param_1  >> 0x10);
        i_var2 = param_1;
        hwnd = GetDlgItem16(0x115, (i_var2 + 6));
        GetWindowText16(0x50, CONCAT22(unaff_ss, local_52), hwnd);
        u_var1 = get_string_index_1000_3da4(CONCAT22(unaff_ss, local_52));
        if (u_var1 != 0) {
            pu_var5 = local_52;
            ppVar4 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(pu_var5, 2));
            pass1_1010_5fd8(ppVar4, CONCAT22(unaff_ss, pu_var5));
            h_wnd = GetWindowWord16(-8, (i_var2 + 6));
            PostMessage16(0, 0x105, 0x111, h_wnd);
            window::destroy_win_1040_7b98(i_var2, u_var3, 1);
        }
    }
    return;
}

pub unsafe fn win_fn_1038_a584(param_1: u32, param_2: i32) {
    let hwnd: HWND16;
    let mut u_var1: i32;
    let h_wnd: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let paVar4: &mut Struct431;
    let pu_var5: Vec<u8>;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: [u8; 80];

    if (param_2 != 0) {
      // u_var3 = (param_1  >> 0x10);
        i_var2 = param_1;
        hwnd = GetDlgItem16(0x114, (i_var2 + 6));
        GetWindowText16(0x50, CONCAT22(unaff_ss, local_52), hwnd);
        u_var1 = get_string_index_1000_3da4(CONCAT22(unaff_ss, local_52));
        if (u_var1 != 0) {
            pu_var5 = local_52;
            paVar4 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(pu_var5, 2));
            pass1_1010_6006(paVar4, CONCAT22(unaff_ss, pu_var5));
            h_wnd = GetWindowWord16(-8, (i_var2 + 6));
            PostMessage16(0, 0x105, 0x111, h_wnd);
            window::destroy_win_1040_7b98(i_var2, u_var3, 1);
        }
    }
    return;
}

pub unsafe fn dc_func_1038_9ffa(param_1: u32) -> u16 {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut u_var3: u16;
    let pu_var4: &mut u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var3 = (param_1  >> 0x10);
    local_4 = GetDC16((param_1 + 6));
    pu_var4 = mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, 3);
    unsafe { i_var2 = *pu_var4 };
    pp_var1 = (i_var2 + 8);
    (**pp_var1)(0x1010, pu_var4, &local_4);
    pp_var1 = (i_var2 + 4);
    (**pp_var1)(0x1010, pu_var4, 0x50005, &local_4);
    pp_var1 = (i_var2 + 0xc);
    (**pp_var1)(0x1010, pu_var4, &local_4);
    ReleaseDC16(local_4, (param_1 + 6));
    return 0;
}

pub unsafe fn win_fn_1038_9bc8(param_1: &mut Struct20) {
    let pi_var1: &mut i32;
    let ppc_var2: fn();
    let mut i_var3: i32;
    let i_var4: u16;
    let mut i_var5: i32;
    let hdc: HDC16;
    let mut i32_var6: i32;
    let HVar7: HWND16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let unaff_ss: HWND16;
    let ppVar10: &mut Struct2551;
    let pu_var11: &mut u16;
    let u_var12: u8;
    let u_var13: u8;
    let HVar14: HWND16;
    let mut in_stack_0000ffc2: u32;
    let mut u_var15: u16;
    let in_string_1: String;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: [u8; 2];
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var15 = (in_stack_0000ffc2  >> 0x10);
    win_gui_func_1040_78e2(param_1);
    if (PTR_LOOP_1050_5ef8 == (&PTR_DAT_0005_0000_1050_0004 + 1)) {
        PTR_LOOP_1050_5ef8 = 0x0;
    }
    u_var12 = SUB21(&local_4, 0);
    u_var13 = (&local_4 >> 8);
    pu_var11 = &local_6;
    HVar7 = unaff_ss;
    HVar14 = unaff_ss;
    _local_a = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(pu_var11, 0x48));
    pass1_1008_3e94(
        (_local_a + 0xe),
        CONCAT22(HVar7, pu_var11),
        CONCAT22(HVar14, CONCAT11(u_var13, u_var12)),
    );
    i_var4 = GetSystemMetrics16(4);
    i_var5 = i_var4 * PTR_LOOP_1050_5ef8 + 10;
    PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 1;
    _local_e = CONCAT22(i_var5 + local_4, i_var5 + local_6);
  // u_var8 = (param_1  >> 0x10);
    i_var5 = param_1;
    GetWindowRect16(
        CONCAT13((local_16 >> 8), CONCAT12(local_16, 0x1538)),
        unaff_ss,
    );
    hdc = GetDC16(0);
    i_var4 = GetDeviceCaps16(10, hdc);
    ReleaseDC16(hdc, 0);
    if (i_var4 < local_10) {
        _local_e = _local_e & 0xffff0000 | ((local_14 - (local_10 - i_var4)) + 1);
    }
    SetWindowPos16(1, 0, 0, _local_e, (_local_e >> 0x10), 0, (i_var5 + 6));
    in_string_1 = CONCAT22(u_var15, 3);
    u_var9 = 0x1010;
    ppVar10 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, in_string_1);
    i32_var6 = ppVar10 + 0xa4;
  // u_var15 = (ppVar10  >> 0x10);
    local_24 = 0;
    while (i_var3 = local_24 * 2, (i_var3 + i32_var6) != 0) {
        in_string_1 = (in_string_1 & 0xffff0000);
        u_var9 = SUB42(offset, 0);
        HVar7 = GetDlgItem16((i_var3 + i32_var6), (i_var5 + 6));
        (i_var5 + i_var3 + 0x94) = HVar7;
        local_24 = local_24 + 1;
        pi_var1 = (i_var5 + 0x128);
        unsafe {
            *pi_var1 = *pi_var1 + 1;
        }
    }
    ppc_var2 = (param_1 + 0x6c);
    ppc_var2(u_var9, i_var5, u_var8, (in_string_1 >> 0x10));
    return;
}

pub unsafe fn win_fn_1038_977a(param_1: i32, param_2: u16) {
    let pp_var1: fn();
    let mut u_var2: i32;
    let mut u_var3: u16;
    let in_dx: &mut Struct199;
    let mut u_var4: i32;
    let unaff_ss: u16;
    let mut u_var5: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: [u8; 4];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: bool;

    local_8 = 0;
    u_var5 = (param_1 + 6);
    u_var2 = GetDlgItemInt16(1, &local_4, unaff_ss, 0xfa8);
    local_6 = u_var2;
    if (u_var2 != 0) {
        process_struct_1000_179c(0xb4, in_dx);
        u_var4 = in_dx | u_var2;
        if (u_var4 == 0) {
            u_var3 = 0;
            u_var4 = 0;
        } else {
            u_var3 = mixed_1040_8520(
                CONCAT22(in_dx, u_var2),
                (param_1 + 6),
                0x41,
                2,
                0x5db,
                0x5da,
            );
        }
        _local_c = CONCAT22(u_var4, u_var3);
        pass1_1008_941a(CONCAT22(unaff_ss, local_10), 1, 0xc3);
        pp_var1 = (*_local_c + 0x6c);
        local_8 = (**pp_var1)(
            &ctx.PTR_LOOP_1050_1008,
            _local_c,
            (_local_c >> 0x10),
            local_10,
            unaff_ss,
            u_var5,
            u_var2,
        );
    }
    if ((local_8 == 1) || (local_6 == 0)) {
        window::destroy_window_1040_b726();
    }
    return;
}

pub unsafe fn win_fn_1038_95fc(param_1: u32) {
    let pp_var1: fn();
    let mut u_var2: i32;
    let paVar3: &mut Struct199;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let struct_a: &mut Struct199;


    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut u_var7: u16;
    let unaff_ss: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let local_10: bool;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 8));
    _local_a = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 9));
  // paVar3 = (_local_a  >> 0x10);
    u_var2 = _local_a;
    process_struct_1000_179c(0xc, paVar3);
    struct_a = (paVar3 | u_var2);
    if (struct_a == 0x0) {
        paVar3 = 0x0;
        struct_a = 0x0;
    } else {
        paVar3 = process_struct_1008_574a(CONCAT22(paVar3, u_var2));
    }
    _local_e = CONCAT22(struct_a, paVar3);
    local_14 = 0;
    while (local_14 < 0xf) {
        u_var10 = (param_1 + 6);
        u_var4 = GetDlgItemInt16(1, &local_10, unaff_ss, (local_14 * 0xe + 0x5a72));
        if (u_var4 != 0) {
            if ((local_14 * 0xe + 0x5a7c) < 0x83) {
                u_var5 = u_var4;
                process_struct_1000_179c(8, struct_a);
                _local_18 = CONCAT22(struct_a, u_var5);
                if ((struct_a | u_var5) == 0) {
                    local_1e = 0;
                } else {
                    *_local_18 = ctx.s_1_1050_389a;
                    (u_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
                    *_local_18 = 0xa1c4;
                    (u_var5 + 2) = 0x1010;
                    local_1e = _local_18;
                }
              // u_var7 = (local_1e  >> 0x10);
                i32_var6 = local_1e;
                *(i32_var6 + 6) = u_var4;
                (i32_var6 + 4) = (local_14 * 0xe + 0x5a7c);
                pp_var1 = (*_local_e + 4);
                (**pp_var1)(
                    0x1000,
                    _local_e,
                    (_local_e >> 0x10),
                    i32_var6,
                    u_var7,
                    u_var10,
                );
                struct_a = ctx.dx_reg;
            } else {
                if ((local_14 * 0xe + 0x5a7c) == 0x89) {
                    u_var9 = (local_14 * 0xe + 0x5a7c);
                    u_var8 = u_var4;
                } else {
                    u_var9 = (local_14 * 0xe + 0x5a7c);
                    u_var8 = 0;
                }
                pass1_1010_6566(_local_a, u_var8, u_var4, u_var9);
                struct_a = ctx.dx_reg;
            }
        }
        local_14 = local_14 + 1;
    }
    (_local_6 + 10) = _local_e;
    PostMessage16(0, 0xed, 0x111, ctx.g_h_window);
    return;
}

pub fn win_fn_1038_9294(param_1: u32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let unaff_ss: u16;
    let local_6: bool;
    let local_4: bool;

    window::set_window_pos_1040_b230(param_1);
  // u_var3 = (param_1  >> 0x10);
    i_var2 = param_1;
    u_var1 = GetDlgItemInt16(1, &local_4, unaff_ss, 0xfa9);
    *(i_var2 + 0x94) = u_var1;
    u_var1 = GetDlgItemInt16(1, &local_6, unaff_ss, 0xfa8);
    *(i_var2 + 0x96) = u_var1;
    dialog::dialog_item_func_1038_98b4(i_var2, u_var3);
    mci_send_command_1008_5c7c(ctx.g_struct_1050_02a0, 0x950001);
    return;
}

pub unsafe fn win_gui_fn_1038_92f6(
    param_1: &mut Struct124,
    param_2_00: Vec<u8>,
    param_3: u16,
    param_2: u32,
) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: i32;
    let i_var4: i16;
    let mut u_var5: u32;

    let paVar6: &mut Struct199;
    let mut i_var7: i32;
    let mut unaff_si: u16;
    let mut u_var8: u16;
    let unaff_ss: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0xeb) {
        _local_6 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 3));
      // paVar6 = (_local_6  >> 0x10);
        u_var5 = &param_1.field_0x90;
        if (u_var5 != 0) {
            local_a = u_var5;
            process_struct_1000_179c(0x18, paVar6);
            u_var3 = u_var5;
            _local_10 = (u_var5 & 0xffff | ZEXT24(paVar6) << 0x10);
            if ((paVar6 | u_var3) == 0) {
                u_var3 = 0;
                paVar6 = 0x0;
            } else {
                process_struct_1040_a598((u_var5 & 0xffff | ZEXT24(paVar6) << 0x10));
                paVar6 = ctx.dx_reg;
            }
            param_1.field_0x90 = u_var3;
            &param_1.field_0x92 = paVar6;
            *&param_1.field_0x90 = 0x11;
            local_c = *&param_1.field_0x90;
            u_var3 = local_c * 10 + 2;
            process_struct_1000_179c(u_var3, paVar6);
            _local_10 = CONCAT22(paVar6, u_var3);
            if ((paVar6 | u_var3) == 0) {
                u_var1 = &param_1.field_0x90;
                (u_var1 + 2) = 0;
            } else {
                *_local_10 = local_c;
                call_fn_ptr_1000_5586(
                    0xa564,
                    &ctx.PTR_LOOP_1050_1040,
                    local_c,
                    10,
                    u_var3 + 2,
                    paVar6,
                );
                u_var1 = &param_1.field_0x90;
              // u_var8 = (u_var1  >> 0x10);
                i_var7 = u_var1;
                (i_var7 + 2) = u_var3 + 2;
                (i_var7 + 4) = paVar6;
            }
          // u_var8 = (local_a  >> 0x10);
            u_var1 = &param_1.field_0x90;
            (u_var1 + 6) = (local_a + 6);
            u_var1 = &param_1.field_0x90;
            (u_var1 + 10) = (local_a + 10);
            u_var1 = &param_1.field_0x90;
            (u_var1 + 0x12) = &param_1.field_0xa;
            u_var8 = 0x1010;
            pass1_1010_a50c(_local_6, 0x10505b42, &param_1.field_0x90);
            local_14 = local_a;
            _local_10 = local_a;
            if (local_a != 0) {
                pass1_1040_a5d0(local_a);
                u_var8 = 0x1000;
                error_check_1000_17ce(local_a);
            }
            ppc_var2 = (CONCAT22(param_2_00, param_1) + 0x70);
            ppc_var2(u_var8, param_1, param_2_00);
        }
    } else {
        if (param_2 != 0xf9) {
            window::win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
            return;
        }
        i_var4 = pass1_1038_993a(param_1, param_2_00, param_3);
        if (-1 < i_var4) {
            local_16 = GetDlgItemInt16(1, &local_1a, unaff_ss, (i_var4 * 0xe + 0x5a72));
            if (local_1a != 0) {
                u_var1 = &param_1[1].field_0x4;
                win_gui_fn_1010_2a32(u_var1, (u_var1 >> 0x10), local_16, (i_var4 * 0xe + 0x5a72));
            }
        }
    }
    return;
}

pub unsafe fn win_gui_fn_1038_8d98(param_1: &mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    if param_3 == 0xeb {
        win_fn_1038_8f74(param_1);
    } else {
        if param_3 != (ctx.s_vrpal_bmp_1050_183a + 7) {
            window::win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        load::load_string_1038_8dda(CONCAT22(param_2, param_1));
    }
    return;
}

pub unsafe fn win_fn_1038_8f74(param_1: &mut Struct124) -> LRESULT {
    let mut i_var1: i32;
    let h_wnd: HWND16;
    // let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let u_var4: u32;
    let l_var5: LRESULT;
    let enable: bool;
    let mut local_510: u16;
    let mut local_50e: u16;
    let mut local_50c: u16;
    let mut u_stack1290: u32;
    let hwnd_1502: HWND16;
    let mut local_40c: [u8; 8];
    let mut local_404: u16;
    let mut local_402: u16;

  // u_var3 = (param_1  >> 0x10);
  //   i_var2 = param_1;
  //   SendDlgItemMessage16(0, 0, 0xb, (ctx.s_650_bmp_1050_1859 + 2), (param_1 + 6));
    SendDlgItemMessage16(param_1 + 6, ctx.s_650_bmp_1050_1859 + 2, 0xb, 0, 0);
    // SendDlgItemMessage16(0, 0, 0x405, (ctx.s_650_bmp_1050_1859 + 2), (param_1 + 6));
    SendDlgItemMessage16(param_1 + 6, ctx.s_650_bmp_1050_1859 + 2, 0x405, 0, 0);
    i_var1 = pass1_1008_c83a((param_1 + 0x94));
    if i_var1 == 0 {
        _local_404 = pass1_1008_c85e((param_1 + 0x94));
        pass1_1008_5784(local_40c, _local_404);
        loop {
            u_var4 = pass1_1008_5b12(local_40c);
            if u_var4 == 0 {
                break;
            }
            wsprintf16(
                &local_50c,
                CONCAT22(0x5a6c, unaff_ss),
                CONCAT22((u_var4 + 4), 0x1050),
            );
            hwnd_1502 = (param_1 + 6);
            u_stack1290 = 0x185b0401;
            local_50c = 0;
            _local_510 = CONCAT22(unaff_ss, &local_50c);
            // SendDlgItemMessage16(_local_510, 0, 0x401, (s_650_bmp_1050_1859 + 2), hwnd_1502);
            SendDlgItemMessage16(hwnd_1502, ctx.s_650_bmp_1050_1859 + 2, 0x401, 0, _local_510);
        }
        // h_wnd = GetDlgItem16(1, (param_1 + 6));
        h_wnd = GetDlgItem16(param_1 + 6, 1);
        enable = true;
    } else {
        load_string_1010_84e0(
            ctx.g_struct_73_1050_14cc,
            (ctx.g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_ss, &local_404),
            0x452,
        );
        SendDlgItemMessage16(
            CONCAT22(unaff_ss, &local_404),
            0,
            0x401,
            (s_650_bmp_1050_1859 + 2),
            (param_1 + 6),
        );
        // h_wnd = GetDlgItem16(1, (param_1 + 6));
        h_wnd = GetDlgItem16(param_1 + 6, 1);
        enable = 0;
    }
    // EnableWindow16(enable, h_wnd);
    EnableWindow16(h_wnd, enable);
    // l_var5 = SendDlgItemMessage16(0, 1, 0xb, (s_650_bmp_1050_1859 + 2), (param_1 + 6));
    l_var5 = SendDlgItemMessage16(param_1 + 6, ctx.s_650_bmp_1050_1859 + 2, 0xb, 1, 0);
    return l_var5;
}

pub fn pass1_1038_89e8(param_1: Vec<u8>) {
    dialog::send_dlg_item_msg_1038_8b58(param_1);
    return;
}

pub fn win_gui_fn_1038_89f8(param_1: &mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    if (param_3 == 0xeb) {
        dialog::send_dlg_item_msg_1038_8b58(CONCAT22(param_2, param_1));
    } else {
        if (param_3 != (s_vrpal_bmp_1050_183a + 7)) {
            window::win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        msg_box::msg_box_1038_8a3a(CONCAT22(param_2, param_1));
    }
    return;
}

pub unsafe fn win_gui_fn_1038_7dc6(param_1: &mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let mut bVar1: bool;
    let mut local_4: u16;

    bVar1 = false;
    if (param_2 == (s_logo_bmp_1050_1850 + 4)) {
        if (param_2 != 1) {}
        // goto LAB_1038_7e8c;
        send_dlg_item_msg_1038_8618(CONCAT22(param_2_00, param_1));
    } else {
        if (param_2 < (s_logo_bmp_1050_1850 + 5)) {
            if (param_2 == 0xeb) {
                dialog::send_dialog_item_msg_1038_844a(CONCAT22(param_2_00, param_1));
            } else {
                if (param_2 == 0xfb) {
                    dialog::send_dlg_item_msg_1038_7eac(CONCAT22(param_2_00, param_1));
                } else {
                    if (param_2 != (s_vrpal_bmp_1050_183a + 7)) {
                        // LAB_1038_7e77:
                        window::win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
                        return;
                    }
                    load_str_1038_81be(CONCAT22(param_2_00, param_1));
                }
            }
            // goto LAB_1038_7e8c;
        }
        if (param_2 == (s_logo_bmp_1050_1850 + 5)) {
            if (param_2 != 1) {}
            // goto LAB_1038_7e8c;
            send_dlg_item_msg_1038_87b2(CONCAT22(param_2_00, param_1));
        } else {
            if (param_2 == (s_logo_bmp_1050_1850 + 6)) {
                if (param_2 != 1) {}
                // goto LAB_1038_7e8c;
                pass1_1038_8810(CONCAT22(param_2_00, param_1));
            } else {
                if (param_2 == (s_logo_bmp_1050_1850 + 8)) {
                    dialog::send_dlg_item_msg_1038_7fae(CONCAT22(param_2_00, param_1));
                } else {
                    if (param_2 != s_650_bmp_1050_1859) {}
                    // goto LAB_1038_7e77;
                    pass1_1038_801a(CONCAT22(param_2_00, param_1));
                }
            }
        }
    }
    bVar1 = true;
    // LAB_1038_7e8c:
    if (bVar1) {
        window::set_window_text_1038_8358(CONCAT22(param_2_00, param_1));
        window::enable_window_1038_806a(CONCAT22(param_2_00, param_1));
    }
    return;
}

pub unsafe fn win_gui_fn_1020_7824(param_1: &mut Struct622, param_2: u16) {

    let mut i_var1: i32;

    let ppVar2: &mut Struct2551;
    let mut local_e: u16;
    let mut local_6: u32;
    let mut temp_5fa3752684: u32;
    let fn_ptr_1: fn();

    get_dc_1020_921c(param_1, param_2);
    (param_1 + 0x14) = 0;
    param_1.u16_0x0 = 0x7902;
    (param_1 + 2) = 0x1020;
    ppVar2 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_e, 0x25));
    ctx.dx_reg = (ppVar2 >> 0x10);
    (param_1 + 0x14) = ppVar2;
    (param_1 + 0x16) = ctx.dx_reg;
    (param_1 + 6) = (param_1 + 0x14);
    (param_1 + 8) = ctx.dx_reg;
    temp_5fa3752684 = (param_1 + 0x14);
    i_var1 = param_1 + 10;
    fn_ptr_1 = ((temp_5fa3752684 + 10) + 8);
    (**fn_ptr_1)();
    (param_1 + 0x12) = i_var1;
    draw_1020_9364((param_1 & 0xffff | param_1 << 0x10));
    return;
}

pub unsafe fn win_gui_fn_1020_76aa(in_struct_1: &mut WinStruct42) {
    let paVar1: &mut Struct199;
    let struct_a: &mut Struct199;
    let mut u_var2: i32;
    let local_struct_1: &mut WinStruct42;
    let local_struct_1_hi: &mut WinStruct42;
    let paVar3: &mut Struct199;
    let mut local_4: u16;

    let mut paVar3 = create_win_1008_9760(ctx, in_struct_1);
  // struct_a = (paVar3  >> 0x10);
  // local_struct_1_hi = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    paVar1 = get_gui_dc_1018_4db0(*&local_struct_1.u32_xf2, local_struct_1.win_handle_0x8);
    process_struct_1000_179c(0x18, struct_a);
    u_var2 = struct_a | paVar1;
    if (u_var2 != 0) {
        win_gui_fn_1020_7824(CONCAT22(struct_a, paVar1), local_struct_1.win_handle_0x8);
        local_struct_1.char_ptr_16_0xee = paVar1;
        local_struct_1.field_0xf0 = u_var2;
        return;
    }
    &local_struct_1.char_ptr_16_0xee = 0;
    return;
}

pub unsafe fn win_gui_fn_1020_75f0(param_1: &mut Struct675) {
    let pi_var1: &mut i32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let struct_a: &mut Struct199;
    let paVar4: &mut Struct199;

    let local_bx_4: &mut Struct675;
    let local_es_4: &mut Struct675;
    let mut unaff_ss: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: [u8; 4];
    let fn_ptr_1: fn();

  // local_es_4 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0xee != 0) {
        fn_ptr_1 = (local_bx_4.field_0xee + 8);
        (**fn_ptr_1)();
    }
    if (local_bx_4.field_0xea == 0) {
        local_bx_4.field_0xea = 1;
        pass1_1008_941a(CONCAT22(unaff_ss, local_6), 1, 0x91);
        u_var3 = ZEXT24(local_6);
        mci_send_command_1008_5c9e(ctx.g_struct_1050_02a0, CONCAT22(unaff_ss, local_6));
        local_bx_4.field_0xec = u_var3;
        paVar4 = struct_a;
        process_struct_1000_179c(0x112, struct_a);
        if ((paVar4 | u_var3) == 0) {
            u_var2 = 0;
            _local_a = 0x0;
        } else {
            pi_var1 = &local_bx_4.field_0xcc;
            unsafe {
                *pi_var1 = *pi_var1 + 1;
            }
            win_gui_fn_1020_3644(u_var3, paVar4, local_bx_4.field_0xcc, param_1);
            u_var2 = u_var3;
            _local_a = (u_var3 & 0xffff | ctx.dx_reg << 0x10);
        }
        pass1_1008_6978(param_1, 0, _local_a & 0xffff0000 | u_var2);
        fn_ptr_1 = (*_local_a + 0xc);
        (**fn_ptr_1)(8, _local_a, local_8, 5);
    }
    return;
}

pub unsafe fn win_gui_fn_1020_43f6(in_struct_1: &mut WinStruct42) {
    let struct_var1: Struct2551;
    let var2: u32;
    let mut u_var3: u32;
    // let local_struct_1: &mut WinStruct42;
    // let local_struct_1_hi: &mut WinStruct42;
    let mut var_4: u16;
    let mut local_8: u16;
    let fn_ptr_1: fn();

    // local_struct_1 = in_struct_1;
    //// ocal_struct_1_hi = (in_struct_1  >> 0x10);
    create_win_1008_9760(ctx, in_struct_1);
    get_gui_dc_1018_4db0(local_struct_1.u32_xfa, local_struct_1.win_handle_0x8);
    struct_var1 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(var_4, 0x32));
    &local_struct_1.field_0x10e = struct_var1;
    (&local_struct_1.field_0x10e + 2) = (struct_var1 >> 0x10);
    fn_ptr_1 = (local_struct_1.field_0x10e + 4);
    var2 = (**fn_ptr_1)();
    process_struct_1000_179c(0x30, (var2 >> 0x10));
    if (var2 == 0) {
        local_struct_1.u32_xf6 = 0;
    } else {
        u_var3 = process_struct_1020_62e0(
            var2,
            CONCAT22(local_struct_1.win_handle_0x8, (var2 >> 0x10)),
        );
        &local_struct_1.u32_xf6 = u_var3;
        (&local_struct_1.u32_xf6 + 2) = (u_var3 >> 0x10);
    }
    gui_window_func_1020_536e(in_struct_1, 0, 0x3ffff);
    return;
}

pub unsafe fn win_gui_fn_1020_434c(
    in_struct_1_1: &mut Struct661,
    in_struct_1_2: &mut Struct661,
    param_3: &mut u32,
    param_4: u16,
    param_5: u16,
    param_6: i32,
) {
    if (param_6 == 1) {
        set_capture_1020_6184(CONCAT22(in_struct_1_2, in_struct_1_1), param_5);
        return;
    }
    if (param_6 == 2) {
        gui_window_func_1020_536e(
            in_struct_1_1,
            in_struct_1_2,
            param_3,
            param_3_00,
            param_5,
            2,
        );
        return;
    }
    pass1_1008_68ea(
        in_struct_1_1,
        in_struct_1_2,
        param_3,
        param_3_00,
        param_5,
        param_6,
    );
    return;
}

pub unsafe fn win_fun_1020_2ae4(param_1: &mut u32, param_2: i32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut cVar3: u8;
    let mut u_var4: u16;
    let mut u_var5: i32;



    let local_bx_14: &mut Struct4;
    let mut u_var6: u16;
    let pp_var7: &mut Struct2551;
    let w_param: WPARAM16;
    let mut i_var8: i32;
    let mut h_wnd: u16;
    let mut local_c: u16;
    let mut local_a: u16;

    if (param_2 != 0x129) {
        local_bx_14 = param_1;
      // u_var6 = (param_1  >> 0x10);
        if (0x129 < param_2) {
            if (param_2 == 0x12a) {
                h_wnd = local_bx_14.field_0x8;
                w_param = 0xf012;
            } else {
                if (param_2 == 299) {
                    return;
                }
                if (param_2 == 300) {
                    h_wnd = local_bx_14.field_0x8;
                    w_param = 0xf020;
                } else {
                    if (param_2 == 0x12d) {
                        return;
                    }
                    if (param_2 != 0x12e) {
                        return;
                    }
                    h_wnd = local_bx_14.field_0x8;
                    w_param = 0xf060;
                }
            }
            PostMessage16(0, w_param, 0x112, h_wnd);
            return;
        }
        if (param_2 == 0xfb) {
            pp_var7 = struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_c, 0x30));
            u_var4 = SUB42(pp_var7, 0);
            pass1_1010_375e(pp_var7);
            unsafe {
                ppc_var2 = (*param_1 + 0x14);
            }
            ppc_var2(
                0x1010,
                local_bx_14,
                (param_1 >> 0x10),
                1,
                u_var4,
                ctx.dx_reg,
            );
            pass1_1010_375e(pp_var7);
            pass1_1018_181c(local_bx_14.field_0xf2, CONCAT22(ctx.dx_reg, u_var4));
            return;
        }
        if (param_2 < 0xfc) {
            cVar3 = param_2;
            u_var5 = param_2 & 0xff00 | (cVar3 + 0x91);
            if ((cVar3 + 0x91) == 0) {
                mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, 0x1f8);
                WinHelp16(
                    0x2a,
                    1,
                    CONCAT22(ctx.dx_reg, u_var5),
                    local_bx_14.field_0x8,
                );
                return;
            }
            if (cVar3 == 'r') {
                i_var8 = &local_bx_14.field_0xa;
                u_var4 = u_var6;
                pp_var7 =
                    struct_ops::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(i_var8, 0x30));
                pass1_1010_3770(pp_var7, CONCAT22(u_var4, i_var8));
                pass1_1038_af40(ctx.g_struct_112_001, local_bx_14.field_0x8, 3);
                return;
            }
            if (cVar3 == 'u') {
                u_var1 = local_bx_14.field_0xf2;
                pass1_1018_0a76(u_var1, (u_var1 >> 0x10));
                InvalidateRect16(0, 0x0, 0);
                return;
            }
        }
    }
    return;
}

pub unsafe fn win_fn_1020_2cf0(ctx: &mut AppContext, param_1: &mut WinStruct42) {
    let pp_var1: fn();
    let mut u_var2: i32;
    let pi_var3: &mut u16;
    let mut u_var4: u16;
    let struct_a: &mut Struct199;
    let pa_var5: &mut Struct199;
    // let mut i32_var6: i32;
    let mut unaff_si: u16;
    // let mut u_var7: i32;
    let pp_var8: &mut Struct2551;
    let mut u_var9: u32;
    let mut local_4: u16;

    create_win_1008_9760(ctx, param_1);
    //// _var7 = (param_1  >> 0x10);
    // i32_var6 = param_1;
    pp_var8 = struct_ops::process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(unaff_si, (i32_var6 + 0xfc)),
    );
  // u_var4 = (pp_var8  >> 0x10);
    (i32_var6 + 0xf2) = pp_var8;
    (i32_var6 + 0xf4) = u_var4;
    u_var2 = (i32_var6 + 0xf2);
    (i32_var6 + 0xe0) = u_var2;
    (i32_var6 + 0xe2) = u_var4;
    LoadIcon16(
        0x1010,
        s_SITEICON_1050_428d,
        &ctx.g_alloc_addr_1050_1050,
        ctx.g_h_instance_1050_038c,
    );
    (i32_var6 + 0xc2) = u_var2;
    pp_var1 = ((i32_var6 + 0xf2) + 0x30);
    (**pp_var1)();
    pa_var5 = struct_a;
    process_struct_1000_179c(0x22, struct_a);
    if ((pa_var5 | u_var2) == 0) {
        (i32_var6 + 0xf6) = 0;
    } else {
        gui_win_fn_1020_2ede(u_var2, pa_var5, param_1);
        (i32_var6 + 0xf6) = u_var2;
        (i32_var6 + 0xf8) = ctx.dx_reg;
    }
    (i32_var6 + 0xe8) = (i32_var6 + 0xf6);
    pass1_1018_0ac0((i32_var6 + 0xf2), param_1 & 0xffff | u_var7 << 0x10);
    u_var9 = pass1_1018_0b08((i32_var6 + 0xf2));
    pi_var3 = u_var9;
    pp_var1 = (param_1 + 0x14);
    (**pp_var1)();
    pp_var1 = ((i32_var6 + 0xf2) + 0x10);
    (**pp_var1)();
    MoveWindow16(
        1,
        pi_var3[3],
        pi_var3[2],
        pi_var3[1],
        unsafe { *pi_var3 },
        (i32_var6 + 8),
    );
    pass1_1008_3e0e(param_1);
    UpdateWindow16((i32_var6 + 8));
    return;
}

pub unsafe fn call_cleanup_fn_1020_2e24(in_struct_1: &mut Struct376, param_2: u8) -> &mut Struct376 {
    cleanup_fn_1020_28fc(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub unsafe fn gui_win_fn_1020_2ede(param_1: &mut u16, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let HVar3: HDC16;
    let mut i_var4: i32;
    let obj_handle: HPEN16;
    let HVar5: HGDIOBJ16;
    let ppVar6: &mut Struct2551;
    let in_struct_104_ptr: &mut Struct104;
    let mut u_var7: u32;
    let u_var8: u8;
    let u_var9: u8;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let u_var13: u8;
    let u_var14: u8;
    let mut i_var15: i32;
    let mut u_var16: u16;
    let mut in_stack_0000ffea: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var10 = (param_2  >> 0x10);
    i_var15 = param_1;
  // u_var16 = (param_1  >> 0x10);
    get_sys_metrics_1020_7c1a(i_var15, u_var16, param_2, u_var10);
    (i_var15 + 0x14) = 0;
    (i_var15 + 0x18) = 0;
    (i_var15 + 0x1a) = 0;
    (i_var15 + 0x1c) = 0;
    (i_var15 + 0x1e) = 0;
    (i_var15 + 0x20) = 0;
    unsafe {
        *param_1 = 0x363c;
    }
    (i_var15 + 2) = 0x1020;
    ppVar6 = struct_ops::process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_0000ffea, (param_2 + 0xfc)),
    );
    (i_var15 + 0x14) = ppVar6;
    (i_var15 + 0x16) = (ppVar6 >> 0x10);
    u_var1 = (i_var15 + 0x14);
    ppc_var2 = ((i_var15 + 0x14) + 4);
    ppc_var2(0x1010, u_var1, (u_var1 >> 0x10), 0, i_var15, u_var16);
    u_var13 = 0xc2;
    u_var14 = 0x42;
    u_var11 = 0;
    u_var12 = 0;
    u_var8 = 0;
    u_var9 = 0;
    u_var10 = 0;
    in_struct_104_ptr = pass1_1018_0a50((i_var15 + 0x14));
    u_var7 = process_struct_1008_4772(in_struct_104_ptr);
    HVar3 = CreateDC16(
        u_var7,
        CONCAT13(u_var9, CONCAT12(u_var8, (u_var7 >> 0x10))),
        CONCAT22(u_var11, u_var10),
        CONCAT13(u_var14, CONCAT12(u_var13, u_var12)),
    );
    (i_var15 + 0x18) = HVar3;
    i_var4 = i_var15 + 0x18;
    ppc_var2 = (in_struct_104_ptr + 8);
    ppc_var2(
        offset,
        in_struct_104_ptr,
        (in_struct_104_ptr >> 0x10),
        i_var4,
        u_var16,
    );
    (i_var15 + 0x20) = i_var4;
    u_var1 = (i_var15 + 0x14);
    obj_handle = CreatePen16((u_var1 + 100), 1, 0);
    (i_var15 + 0x1a) = obj_handle;
    HVar5 = SelectObject16(obj_handle, (i_var15 + 0x18));
    (i_var15 + 0x1c) = HVar5;
    HVar5 = GetStockObject16(5);
    HVar5 = SelectObject16(HVar5, (i_var15 + 0x18));
    (i_var15 + 0x1e) = HVar5;
    return;
}

pub fn invalidate_rect_1020_3080(param_1: u32, param_2: i32) {
    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if ((param_2 != 4) && (param_2 != 6)) {
        return;
    }
    InvalidateRect16(0, 0x0, 0);
    return;
}

pub unsafe fn win_gui_fn_1020_2a46(in_struct_1: &mut Struct628, param_2: u16, param_3: u16) {
    pass1_1018_0ae8(in_struct_1.field_0xf2, 1);
    window::show_window_1008_68c6(in_struct_1, param_2, param_3);
    return;
}

pub unsafe fn pass1_1020_289a(param_1: &mut WinStruct42, param_2: u16, param_3: u32) {
    let mut i_var1: i32;
    let local_struct_1_hi: &mut WinStruct42;

    cursor::call_load_cursor_1020_790e(param_1, s_SCPOPMENU_1050_427c, param_2, param_3);
  // local_struct_1_hi = (param_1  >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0xf2) = 0;
    (i_var1 + 0xf6) = 0;
    (i_var1 + 0xfa) = 0;
    (i_var1 + 0xfc) = 0;
    param_1.u16_x0 = (s__P__P__P__P__P__P__P__P__P__P__P_1050_2e35 + 0x15);
    (i_var1 + 2) = 0x1020;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (i_var1 + 0x5b)), s_VrMode_1050_4286);
    (i_var1 + 0xac) = 0x44c00000;
    return;
}

pub unsafe fn win_gui_fn_1020_2642(in_struct_1: &mut WinStruct42) {
    // let paVar1: &mut Struct650;
    // let struct_a: &mut Struct199;
    // let local_struct_1: &mut WinStruct42;
    // let local_struct_1_hi: &mut WinStruct42;
    // let struct_var2: Struct199;
    let mut u_var3: u32;
    let mut local_4: u16;

    let mut struct_var2 = create_win_1008_9760(ctx, in_struct_1);
    //// truct_a = (struct_var2  >> 0x10);
    //// ocal_struct_1_hi = (in_struct_1  >> 0x10);
    // local_struct_1 = in_struct_1;
    let mut var2 = get_gui_dc_1018_4db0(&in_struct_1.u32_xf2,
                                        local_struct_1.win_handle_0x8);
    process_struct_1000_179c(0x18, struct_a);
    if (struct_a | var2) != 0 {
        u_var3 = call_draw_fn_1020_27b0(ctx, var2, CONCAT22(local_struct_1.win_handle_0x8, struct_a));
        local_struct_1.char_ptr_16_0xee = u_var3;
        local_struct_1.field_0xf0 = (u_var3 >> 0x10);
        return;
    }
    local_struct_1.char_ptr_16_0xee = 0;
    return;
}

pub unsafe fn win_gui_fn_1020_3644(in_struct_1: &mut WinStruct42, param_2: u16, param_3: u32) {
    let local_struct_1: &mut WinStruct42;
    let local_struct_1_hi: &mut WinStruct42;

    cursor::call_load_cursor_1020_790e(in_struct_1, 0x0, param_2, param_3);
  // local_struct_1_hi = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.u32_xf2 = ctx.s_1_1050_389a;
    local_struct_1.field_0xf4 = &ctx.PTR_LOOP_1050_1008;
    local_struct_1.u32_xf2 = (ctx.s_18_2_1050_3aa5 + 3);
    local_struct_1.field_0xf4 = &ctx.PTR_LOOP_1050_1008;
    local_struct_1.field_0x100 = 0;
    local_struct_1.field_0x10a = 0;
    local_struct_1.field_0x10e = 0;
    in_struct_1.u16_x0 = 0x3d08;
    local_struct_1.u16_x2 = 0x1020;
    local_struct_1.u32_xf2 = 0x3d9c;
    local_struct_1.field_0xf4 = 0x1020;
    load_string_1010_84e0(
        ctx.g_struct_73_1050_14cc,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.u32_0xa)),
        0x5e9,
    );
    copy_string_1000_3d3e(
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.class_name_0x5b)),
        s_VrMode_1050_42ca,
    );
    local_struct_1.style_0xac = 0x44c00000;
    set_window_pos_1020_38aa(local_struct_1, local_struct_1_hi);
    return;
}

pub fn win_fn_1020_36f6(ctx: &mut AppContext, param_1: u32, param_2: i32) {
    let mut i_var1: i32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let pu_var4: Vec<u8>;
    let mut HVar5: HWND16;
    let mut h_var6: HWND16;
    let mut u_var7: u32;
    let mut in_dx: i32;

    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let mut u_var10: u16;
    let u_var11: u8;
    let u_var12: u8;
    let mut local_40e: u16;
    let mut local_40c: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    let mut local_406: [u8; 1024];
    let mut local_6: u16;
    let mut local_4: u16;

    i_var8 = param_1;
  // u_var9 = (param_1  >> 0x10);
    if (param_2 == 1) {
        (i_var8 + 8) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    u_var7 = pass1_1018_1e78((i_var8 + 8), 0xffff);
    _local_6 = (u_var7 & 0xffff | in_dx << 0x10);
    u_var3 = (i_var8 + 0x18);
    GetWindowText16(0x3ff, CONCAT22(unaff_ss, local_406), (u_var3 + 6));
    pu_var4 = local_406;
    process_string_1000_472c(CONCAT22(unaff_ss, pu_var4), ':');
    _local_40a = CONCAT22(in_dx, pu_var4 + 2);
    *_local_40a = 0;
    load_string_1010_84e0(
        ctx.g_struct_73_1050_14cc,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_406),
        *_local_6,
    );
    u_var3 = (i_var8 + 0x18);
    ppc_var2 = ((i_var8 + 0x18) + 0x18);
    ppc_var2(0x1010, u_var3, (u_var3 >> 0x10), local_406);
    u_var3 = (i_var8 + 8);
    i_var1 = (u_var3 + 0x4a);
    u_var3 = (i_var8 + 0x18);
    h_var6 = (u_var3 + 6);
    HVar5 = h_var6;
    SetDlgItemText16((_local_6 + 2), 0x10f, h_var6);
    SetDlgItemText16((_local_6 + 10), 0x110, h_var6);
    SetDlgItemText16((_local_6 + 0x12), 0x112, h_var6);
    SetDlgItemText16((_local_6 + 0xe), 0x113, h_var6);
    if (i_var1 != 0) {
        HVar5 = pass1_1018_1f1a((i_var8 + 8), (_local_6 + 0x1a));
        if (HVar5 != 0) {
            u_var11 = 0x11;
            u_var12 = 1;
            u_var3 = (_local_6 + 0x16);
            HVar5 = u_var3;
          // u_var10 = (u_var3  >> 0x10);
            // goto LAB_1020_3846;
        }
    }
    u_var11 = 0x11;
    u_var12 = 1;
    load_string_1010_847e(
        ctx.g_struct_73_1050_14cc,
        (ctx.g_struct_73_1050_14cc >> 0x10),
        0x5d9,
    );
    u_var10 = ctx.dx_reg;
    // LAB_1020_3846:
    SetDlgItemText16(CONCAT22(u_var10, HVar5), CONCAT11(u_var12, u_var11), h_var6);
    if ((i_var8 + 0x1c) != 0) {
        u_var3 = (i_var8 + 0x1c);
        h_var6 = GetDlgItem16((_local_6 + 0x1a), (u_var3 + 6));
        SetFocus16(h_var6);
        return;
    }
    InvalidateRect16(0, 0x0, 0);
    return;
}
