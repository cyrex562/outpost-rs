#![allow(non_snake_case)]

use crate::defines::U32Ptr;

pub struct AppContext {
    pub PTR_LOOP_1050_5f7e: U32Ptr,
    pub PTR_LOOP_1050_1000: U32Ptr,
    pub PTR_LOOP_1050_5f80: U32Ptr,
    pub PTR_LOOP_1050_63fe: U32Ptr,
    pub PTR_LOOP_1050_5f84: U32Ptr,
    pub PTR_LOOP_1050_5f4c: U32Ptr,
    pub PTR_LOOP_1050_5f48: U32Ptr,
    pub PTR_LOOP_1050_5f4a: U32Ptr,
    pub PTR_LOOP_1050_5f4e: U32Ptr,
    pub PTR_LOOP_1050_5f50: U32Ptr,
    pub PTR_LOOP_1050_5f52: U32Ptr,
    pub data_seg: u16,
    pub _DAT_1050_5f82: u16,
    pub DAT_1050_5f87: u16,
    pub s_tile2_bmp_1050_1538: U32Ptr,
    pub _PTR_LOOP_1050_65e2: U32Ptr,
    pub _PTR_LOOP_1050_0368: U32Ptr,
    pub PTR__LOOP_1050_0368: U32Ptr,
    pub PTR__LOOP_1050_0ed0: U32Ptr,
    pub PTR__LOOP_1050_14cc: U32Ptr,
    pub PTR_LOOP_1050_038c: U32Ptr,
    pub PTR__LOOP_1050_5748: U32Ptr,
    pub USHORT_1050_1028: u16,
    pub PTR_LOOP_1050_0310: U32Ptr,
    pub PTR__LOOP_1050_4230: U32Ptr,
    pub PTR__LOOP_1050_5b64: U32Ptr,
    pub _PTR_LOOP_1050_5b64: U32Ptr,
    pub PTR_LOOP_1050_5b68: U32Ptr,
    pub PTR_LOOP_1050_5b6a: U32Ptr,
    pub PTR_LOOP_1050_5b64: U32Ptr,
    pub PTR_LOOP_1050_000c: U32Ptr,
    pub PTR_LOOP_1050_000e: U32Ptr,
    pub PTR_LOOP_1050_000a: U32Ptr,
    pub DAT_1050_0004: i16,
    pub PTR_LOOP_1050_0010: U32Ptr,
    pub PTR_LOOP_1050_0002: U32Ptr,
    pub PTR_LOOP_1050_0000: U32Ptr,
    pub s_version__d__d_1050_0012: String,
    pub PTR_LOOP_1050_5f34: U32Ptr,
    pub PTR_LOOP_1050_5f36: U32Ptr,
    pub PTR_LOOP_1050_5f2e: U32Ptr,
    pub PTR_LOOP_1050_5f2c: U32Ptr,
    pub PTR_PTR_1050_5f1a: U32Ptr,
    pub PTR_LOOP_1050_5f1c: U32Ptr,
    pub PTR_PTR_1050_1f7e: U32Ptr,
    pub PTR_LOOP_1050_5f26: U32Ptr,
    pub PTR_LOOP_1050_5f22: U32Ptr,
    pub PTR_LOOP_1050_5f20: U32Ptr,
    pub PTR_LOOP_1050_5f1e: U32Ptr,
    pub PTR_LOOP_1050_6210: U32Ptr,
    pub PTR_LOOP_1050_5fc2: U32Ptr,
    pub PTR_LOOP_1050_5fc4: U32Ptr,
    pub PTR_LOOP_1050_5fb8: U32Ptr,
    pub s_New_failed_in_Op__Op__DialogHand_1050_0073: String,
    pub s_You_may_not_run_a_turn__The_game_1050_00df: String,
    pub PTR_LOOP_1050_5fd2: U32Ptr,
    pub PTR_LOOP_1050_5fd4: U32Ptr,
    pub PTR_LOOP_1050_5fba: U32Ptr,
    pub PTR_LOOP_1050_5fbc: U32Ptr,
    pub _PTR_LOOP_1050_5fc2: U32Ptr,
    pub s__C_FILE_INFO__1050_5f5c: String,
    pub PTR_LOOP_1050_5fbe: U32Ptr,
    pub PTR_LOOP_1050_5fc0: U32Ptr,
    pub PTR_LOOP_1050_61ec: U32Ptr,
    pub PTR_LOOP_1050_6066: U32Ptr,
    pub DAT_1050_61e8: u32,
    pub PTR_LOOP_1050_61ea: U32Ptr,
    pub s_TPPOPMENU_1050_43fa: String,
    pub s_SOLDefaultWindowClass_1050_01fe: String,
    pub PTR_LOOP_1050_48de: U32Ptr,
    pub _PTR_LOOP_1050_5f2c: U32Ptr,
    pub PTR_LOOP_1050_5f88: U32Ptr,
    pub PTR_LOOP_1050_5f78: U32Ptr,
    pub DAT_1050_5f8a: u32,
    pub PTR_LOOP_1050_5ff0: U32Ptr,
    pub DAT_1050_0009: i16,
    pub PTR_s_3_wav_1050_25cc_1050_6068: String,
    pub PTR_s_3_wav_1050_25cc_1050_607c: String,
    pub PTR_s_3_wav_1050_25cc_1050_6074: String,
    pub PTR_s_3_wav_1050_25cc_1050_6070: String,
    pub DAT_1050_605d: i16,
    pub PTR_LOOP_1050_6062: U32Ptr,
    pub s_MciSoundWindow_1050_02bd: String,
    pub s_SCInternalPutBldg2_site_0x_08lx__1050_5099: String,
}

impl AppContext {
    pub fn new() -> AppContext {
        AppContext{
            PTR_LOOP_1050_5f7e: 0,
            PTR_LOOP_1050_1000: 0,
            PTR_LOOP_1050_5f80: 0,
            PTR_LOOP_1050_63fe: 0,
            PTR_LOOP_1050_5f84: 0,
            PTR_LOOP_1050_5f4c: 0,
            data_seg: 0x1050,
            _DAT_1050_5f82: 0,
            DAT_1050_5f87: 0,
            PTR_LOOP_1050_5f48: 0,
            PTR_LOOP_1050_5f4a: 0,
            PTR_LOOP_1050_5f4e: 0,
            PTR_LOOP_1050_5f50: 0,
            PTR_LOOP_1050_5f52: 0,
            s_tile2_bmp_1050_1538: 0,
            _PTR_LOOP_1050_65e2: 0,
            _PTR_LOOP_1050_0368: 0,
            PTR__LOOP_1050_0368: 0,
            PTR__LOOP_1050_0ed0: 0,
            PTR__LOOP_1050_14cc: 0,
            PTR_LOOP_1050_038c: 0,
            PTR__LOOP_1050_5748: 0,
            USHORT_1050_1028: 0,
            PTR_LOOP_1050_0310: 0,
            PTR__LOOP_1050_4230: 0,
            PTR__LOOP_1050_5b64: 0,
            _PTR_LOOP_1050_5b64: 0,
            PTR_LOOP_1050_5b68: 0,
            PTR_LOOP_1050_5b6a: 0,
            PTR_LOOP_1050_5b64: 0,
            PTR_LOOP_1050_000c: 0,
            PTR_LOOP_1050_000e: 0,
            PTR_LOOP_1050_000a: 0,
            DAT_1050_0004: 0,
            PTR_LOOP_1050_0010: 0,
            PTR_LOOP_1050_0002: 0,
            PTR_LOOP_1050_0000: 0,
            s_version__d__d_1050_0012: "".to_string(),
            PTR_LOOP_1050_5f34: 0,
            PTR_LOOP_1050_5f36: 0,
            PTR_LOOP_1050_5f2e: 0,
            PTR_LOOP_1050_5f2c: 0,
            PTR_PTR_1050_5f1a: 0,
            PTR_LOOP_1050_5f1c: 0,
            PTR_PTR_1050_1f7e: 0,
            PTR_LOOP_1050_5f26: 0,
            PTR_LOOP_1050_5f22: 0,
            PTR_LOOP_1050_5f20: 0,
            PTR_LOOP_1050_5f1e: 0,
            PTR_LOOP_1050_6210: 0,
            PTR_LOOP_1050_5fc2: 0,
            PTR_LOOP_1050_5fc4: 0,
            PTR_LOOP_1050_5fb8: 0,
            s_New_failed_in_Op__Op__DialogHand_1050_0073: "".to_string(),
            s_You_may_not_run_a_turn__The_game_1050_00df: "".to_string(),
            PTR_LOOP_1050_5fd2: 0,
            PTR_LOOP_1050_5fd4: 0,
            PTR_LOOP_1050_5fba: 0,
            PTR_LOOP_1050_5fbc: 0,
            _PTR_LOOP_1050_5fc2: 0,
            s__C_FILE_INFO__1050_5f5c: "".to_string(),
            PTR_LOOP_1050_5fbe: 0,
            PTR_LOOP_1050_5fc0: 0,
            PTR_LOOP_1050_61ec: 0,
            PTR_LOOP_1050_6066: 0,
            DAT_1050_61e8: 0,
            PTR_LOOP_1050_61ea: 0,
            s_TPPOPMENU_1050_43fa: "".to_string(),
            s_SOLDefaultWindowClass_1050_01fe: "".to_string(),
            PTR_LOOP_1050_48de: 0,
            _PTR_LOOP_1050_5f2c: 0,
            PTR_LOOP_1050_5f88: 0,
            PTR_LOOP_1050_5f78: 0,
            DAT_1050_5f8a: 0,
            PTR_LOOP_1050_5ff0: 0,
            DAT_1050_0009: 0,
            PTR_s_3_wav_1050_25cc_1050_6068: "".to_string(),
            PTR_s_3_wav_1050_25cc_1050_607c: "".to_string(),
            PTR_s_3_wav_1050_25cc_1050_6074: "".to_string(),
            PTR_s_3_wav_1050_25cc_1050_6070: "".to_string(),
            DAT_1050_605d: 0,
            PTR_LOOP_1050_6062: 0,
            s_MciSoundWindow_1050_02bd: "".to_string(),
            s_SCInternalPutBldg2_site_0x_08lx__1050_5099: "".to_string()
        }
    }
}
