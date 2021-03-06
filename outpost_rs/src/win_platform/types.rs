

// type HANDLE16 = u16;
// type HCURSOR16 = HANDLE16;
// type HBRUSH16 = HANDLE16;
// type HMENU16 = HANDLE16;
// type WPARAM16 = u16;
// type HWND16 = HANDLE16;
// type HMODULE16 = HANDLE16;
// type HACCEL16 = HANDLE16;

// pub struct PALETTEENTRY
// {
//     pe_red: u8,
//     pe_green: u8,
//     pe_blue: u8,
//     pe_flags: u8,
// }



// pub struct RECT16
// {
//     x: i16,
//     y: i16,
// }

use sdl2::hint::Hint::Default;

struct POINT
{
    x: i16,
    y: i16,
}

// pub struct WINDOWPLACEMENT16 {
//     length: u16,
//     flags: u16,
//     show_cmd: u16,
//     pt_min_position: POINT,
//     pt_max_position: POINT,
//     rc_normal_position: POINT,
// }

// type HICON16 = HANDLE16;
// type SEGPTR = u32;

// pub struct WNDCLASS16
// {
//      let mut style: u16;
//     LPVOID    lpfn_wnd_proc;
//     i16       cb_cls_extra;
//     i16       cb_wnd_extra;
//     HANDLE16  h_instance;
//     HICON16   h_icon;
//     HCURSOR16 h_cursor;
//     HBRUSH16  hbr_background;
//     SEGPTR    lpsz_menu_name;
//     SEGPTR    lpsz_class_name;
// };

// struct TwoWords
// {
//      let mut a_0x0: u16;
//      let mut b_0x2: u16;
// };

// type COLORREF = u32;
// type HDC16 = HANDLE16;
// type HFILE16 = HANDLE16;
// type HGLOBAL16 = HANDLE16;
// type HPEN16 = HANDLE16;
// type HGDIOBJ16 = HANDLE16;

// pub struct LOGPALETTE
// {
//     pal_version: u16,
//     pal_num_entries: u16,
// }


// pub struct MSG16
// {
//     hwnd: HWND16,
//      message: u16,
//     wparam: WPARAM16,
//     lparam: LPARAM,
//     time: u32,
//     pt: POINT16,
// }

// pub struct BITMAPINFOHEADER
// {
//     bi_size: DWORD,
//     bi_width: LONG,
//     bi_height: LONG,
//     bi_planes: WORD,
//     bi_bit_count: WORD,
//     bi_compression: DWORD,
//     bi_size_image: DWORD,
//     bi_x_pels_per_meter: LONG,
//     bi_y_pels_per_meter: LONG,
//     bi_clr_used: DWORD,
//     bi_clr_important: DWORD,
// }

// pub struct PAINTSTRUCT16
// {
//     hdc: HDC16,
//     f_erase: BOOL16,
//     rc_pai16: RECT16,
//     f_restore: BOOL16,
//     f_inc_update: BOOL16,
//     rgb_reserved: [u8;16],
// };

// struct tagRGBQUAD
// {
//     rgbBlue: BYTE,
//     rgbGreen: BYTE,
//     rgbRed: BYTE,
//     rgbReserved: BYTE,
// };

// struct BITMAPINFO
// {
//     bim_header: BITMAPINFOHEADER,
//     bmi_colors: RGBQUAD,
// };


// typedef HANDLE16 HTASK16;
//
// typedef HANDLE16 HPALETTE16;
//
// typedef HANDLE16 HRSRC16;


// WARNING! conflicting data type names: /RGBQUAD - /wingdi.h/RGBQUAD



// struct tagPOINT
// {
//     x: LONG,
//     y: LONG,
// };

// struct tagMSG
// {
//     hwnd: HWND,
//      let mut message: 6: u1,
//     wParam: WPARAM,
//     lParam: LPARAM,
//     time: DWORD,
//     pt: POINT,
// };

// struct HWND__
// {
//     unused: i16,
// };

// typedef struct _devicemodeA _devicemodeA, *P_devicemodeA;
//
// typedef struct _devicemodeA DEVMODEA;


// struct _POINTL
// {
//     LONG x;
//     LONG y;
// };

// struct CONTEXT
// {
// };

// #define NULL 0;

// typedef u32 (*swi_0x21_fn_ptr);
pub type swi_0x21_fn_ptr = fn;

pub struct RTL_CRITICAL_SECTION
{
    debug_info: *mut RTL_CRITICAL_SECTION_DEBUG,
    lock_count: i32,
    recursion_count: i32,
    owning_thread: HANDLE16,
    lock_semapthore: HANDLE16,
    spin_count: u32,
}

pub struct LIST_ENTRY
{
    flink: *mut LIST_ENTRY,
    blink: *mut LIST_ENTRY,
}


pub struct RTL_CRITICAL_SECTION_DEBUG
{
    some_type: u16,
    creator_back_trace_index: u16,
    critical_section: *mut RTL_CRITICAL_SECTION,
    process_locks_list: _LIST_ENTRY,
    entry_count: u32,
    contention_count: u32,
    flags: u32,
    creator_back_trace_index_high: u16,
    spare_word: u16,

}

pub enum EXCEPTION_DISPOSITION
{
    ExceptionContinueExecution = 0,
    ExceptionContinueSearch    = 1,
    ExceptionNestedException   = 2,
    ExceptionCollidedUnwind    = 3
}

pub struct EXCEPTION_RESGISTRATION_RECORD
{
    next: *mut EXCEPTION_REGISTRATION_RECORD,
    handler: *mut EXCEPTION_DISPOSITION,
}

// typedef void                 *PVOID;
// typedef PVOID                 HANDLE;
// typedef unsigned i16          DWORD;
// typedef unsigned short        WORD;
// typedef wchar_t               WCHAR;
// typedef char                  CHAR;
// typedef char                  TCHAR;
// typedef unsigned char         BYTE;
// typedef unsigned i16          u16;
// typedef unsigned long         u32;
// typedef i16                   i16;
// typedef long                  LONG;
// typedef long long             LONGLONG;
// typedef i16                   BOOL;
// typedef unsigned i16          SIZE_T;
// typedef char                 *LPSTR;
// typedef char                 *LPTSTR;
// typedef const char           *LPCTSTR;
// typedef const char           *LPCSTR;
// typedef unsigned short       *LPWSTR;
// typedef const unsigned short *LPCWSTR;
// typedef BYTE                 *LPBYTE;
// typedef LONG                 *LPLONG;
// typedef LONG                 *PLONG;
// typedef i16                  *LPi16;
// typedef DWORD                *LPDWORD;
// typedef DWORD                *PDWORD;
// typedef BOOL                 *LPBOOL;
// typedef void                 *LPVOID;
// typedef void                 *FARPROC;
// typedef i16                   LCID;
// typedef i16                   HKEY;
// typedef HKEY                 *PHKEY;
// typedef i16                   REGSAM;
// typedef HANDLE                HWND;
// typedef HANDLE                HINSTANCE;
// typedef HINSTANCE             HMODULE;
// typedef i16                   MMRESULT;
// typedef i16                   TOKEN_INFORMATION_CLASS;
//
// // typedef void *LPCRITICAL_SECTION;
// typedef void *LPSECURITY_ATTRIBUTES;
// typedef void *LPSTARTUPINFO;
// typedef void *LPPROCESS_INFORMATION;
// typedef void *LPTIME_ZONE_INFORMATION;
// typedef void *PINPUT_RECORD;
// typedef void *PHANDLER_ROUTINE;
// typedef void *LPWSAOVERLAPPED;
// typedef void *LPWSAOVERLAPPED_COMPLETION_ROUTINE;
// typedef void *PSID;
// typedef void *PSID_IDENTIFIER_AUTHORITY;
// typedef void *PACL;
// typedef void *PSECURITY_DESCRIPTOR;
// typedef void *LPSERVICE_TABLE_ENTRY;
// typedef void *LPSERVICE_STATUS;
// typedef void *HCRYPTPROV;
// typedef void *LPOVERLAPPED;
// #if defined(_WIN64)
// typedef unsigned __i1664 u32_PTR;
// #else
// typedef unsigned long u32_PTR;
// #endif
pub type HANDLE16 = u16;
// typedef let HBRUSH16: HANDLE16;
pub type HBRUSH16 = HANDLE16;
// typedef let HMENU16: HANDLE16;
pub type HMENU16 = HANDLE16;
// typedef uWPARAM16: i16;
pub type WPARAM16 = u16;
// typedef let HWND16: HANDLE16;
pub type HWND16 = HANDLE16;
// typedef let HMODULE16: HANDLE16;
pub type HMODULE16 = HANDLE16;
// typedef let HACCEL16: HANDLE16;
pub type HACCEL16 = HANDLE16;
// typedef let HTASK16: HANDLE16;
pub type HTASK16 = HANDLE16;
// typedef COLORREF: u32;
pub type COLORREF = u32;
// typedef let HDC16: HANDLE16;
pub type HDC16 = HANDLE16;
// typedef let HFILE16: HANDLE16;
pub type HFILE16 = HANDLE16;
// typedef let HGLOBAL16: HANDLE16;
pub type HGLOBAL16 = HANDLE16;
// typedef i8: u8;
// typedef let HPEN16: HANDLE16;
pub type HPEN16 = HANDLE16;
// typedef let HINSTANCE16: HANDLE16;
pub type HINSTANCE16 = HANDLE16;

pub type HICON16 = HANDLE16;

// typedef long LONGPTR;
pub type LONGPTR = libc::c_long;
// typedef LONGPTR LPARAM;
pub type LPARAM = LONGPTR;

pub type HGDIOBJ16 = HANDLE16;

pub type HPALETTE16 = HANDLE16;

// typedef LONGPTR LRESULT;
pub type LRESULT = LONGPTR;

pub type HCURSOR16 = HANDLE16;

// typedef SEGPTR: u32;
pub type SEGPTR = u32;

pub type HRSRC16 = HANDLE16;

// typedef WORD ATOM;
pub type ATOM = i16;

pub struct FLOATING_SAVE_AREA {
    // DWORD ControlWord;
    pub control_word: u32,
    // DWORD StatusWord;
    pub status_word: u32,
    // DWORD TagWord;
    pub tag_word: u32,
    // DWORD ErrorOffset;
    pub error_offset: u32,
    // DWORD ErrorSelector;
    pub error_selector: u32,
    // DWORD DataOffset;
    pub data_offset: u32,
    // DWORD DataSelector;
    pub data_selector: u32,
    // BYTE RegisterArea[80];
    pub register_area: [u8; 80],
    // DWORD Cr0NpxState;
    pub cr0_npx_state: u32,
}

// struct PALETTEENTRY {
//     BYTE pe_red;
//     BYTE pe_green;
//     BYTE pe_blue;
//     BYTE pe_flags;
// };
pub struct PALETTEENTRY {
    pe_red: u8,
    pe_green: u8,
    pe_blue: u8,
    pe_flags: u8,
}

impl FLOATING_SAVE_AREA {
    pub fn new() -> FLOATING_SAVE_AREA {
        FLOATING_SAVE_AREA {
            control_word: 0,
            status_word: 0,
            tag_word: 0,
            error_offset: 0,
            error_selector: 0,
            data_offset: 0,
            data_selector: 0,
            register_area: [0; 80],
            cr0_npx_state: 0,
        }
    }
}

pub struct CONTEXT {
    // DWORD ContextFlags;
    pub context_flags: u32,
    // DWORD Dr0;
    pub dr0: u32,
    // DWORD Dr1;
    pub dr1: u32,
    // DWORD Dr2;
    pub dr2: u32,
    // DWORD Dr3;
    pub dr3: u32,
    // DWORD Dr6;
    pub dr6: u32,
    // DWORD Dr7;
    pub dr7: u32,
    // FLOATING_SAVE_AREA FloatSave;
    pub float_save: FLOATING_SAVE_AREA::new(),
    // DWORD SegGs;
    pub seg_gs: u32,
    // DWORD SegFs;
    pub seg_fs: u32,
    // DWORD SegEs;
    pub seg_es: u32,
    // DWORD SegDs;
    pub seg_ds: u32,
    // DWORD Edi;,
    pub edi: u32,
    // DWORD Esi;
    pub esi: u32,
    // DWORD Ebx;
    pub ebx: u32,
    // DWORD Edx;
    pub edx: u32,
    // DWORD Ecx;
    pub ecx: u32,
    // DWORD Eax;
    pub eax: u32,
    // DWORD Ebp;
    pub ebp: u32,
    // DWORD Eip;
    pub eip: u32,
    // DWORD SegCs;
    pub seg_cs: u32,
    // DWORD EFlags;
    pub e_flags: u32,
    // DWORD Esp;
    pub esp: u32,
    // DWORD SegSs;
    pub seg_ss: u32,
    // BYTE ExtendedRegisters[512];
    pub extended_registers: [u8; 512],
}

impl CONTEXT {
    pub fn new() -> CONTEXT {
        CONTEXT {
            context_flags: 0,
            dr0: 0,
            dr1: 0,
            dr2: 0,
            dr3: 0,
            dr6: 0,
            dr7: 0,
            float_save: FLOATING_SAVE_AREA::new(),
            seg_gs: 0,
            seg_fs: 0,
            seg_es: 0,
            seg_ds: 0,
            edi: 0,
            esi: 0,
            ebx: 0,
            edx: 0,
            ecx: 0,
            eax: 0,
            ebp: 0,
            eip: 0,
            seg_cs: 0,
            e_flags: 0,
            esp: 0,
            seg_ss: 0,
            extended_registers: [0;512],
        }
    }
}

#[derive(Clone,Debug,PartialEq,PartialOrd,Default)]
pub struct RECT16 {
    // x: i16;
    pub x: i16,
    // y: i16;
    pub y: i16,
}

impl RECT16 {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}


pub struct POINT16 {
    // x: i16;
    pub x: i16,
    // y: i16;
    pub y: i16,
}

pub struct WINDOWPLACEMENT16 {
    // Ulength: i16;
    pub length: u16,
    // Uflags: i16;
    pub flags: u16,
    // Ushow_cmd: i16;
    pub show_cmd: u16,
    // struct POpt_min_position: i16;
    pub pt_min_position: POINT16,
    // struct POpt_max_position: i16;
    pub pt_max_position: POINT16,
    // struct let rc_normal_position: RECT16;
    pub rc_normal_position: RECT16,
}

pub struct WNDCLASS16 {
    // Ustyle: i16;
    pub style: u16,
    // LPVOID lpfn_wnd_proc;
    pub lpfn_wnd_proc: u32,
    // cb_cls_extra: i16;
    pub cb_cls_extra: i16,
    // cb_wnd_extra: i16;
    pub cb_wnd_extra: i16,
    // let h_instance: HANDLE16;
    pub h_instance: HANDLE16,
    // let h_icon: HICON16;
    pub h_icon: HICON16,
    // let h_cursor: HCURSOR16;
    pub h_cursor: HCURSOR16,
    // let hbr_background: HBRUSH16;
    pub hbr_background: HBRUSH16,
    // lpsz_menu_name: SEGPTR;
    pub lpsz_menu_name: SEGPTR,
    // lpsz_class_name: SEGPTR;
    pub lpsz_class_name: SEGPTR,
}

// typedef long i32;

// typedef struct TwoWords TwoWords, *PTwoWords;

struct TwoWords {
    // Ua_0x0: i16;
    a_0x0: u16,
    // Ub_0x2: i16;
    b_0x2: u16,
}

// typedef uBOOL16: i16;

// typedef let HGDIOBJ16: HANDLE16;

// typedef struct LOGPALETTE LOGPALETTE, *PLOGPALETTE;

// typedef uWORD: i16;

pub struct LOGPALETTE {
    // WORD pal_version;
    pub pal_version: u16,
    // WORD pal_num_entries;
    pub pal_num_entries: u16,
}

// typedef struct MSG16 MSG16, *PMSG16;

// typedef ulong DWORD;

pub struct MSG16 {
    // HWND16 hwnd;
    pub hwnd: HWND16,
    // Umessage: i16;
    pub message: u16,
    // wparam: WPARAM16;
    pub wparam: WPARAM16,
    // lparam: LPARAM;
    pub lparam: LPARAM,
    // DWORD time;
    pub time: u32,
    // struct POpt: i16;
    pub pt: POINT16,
}

pub struct BITMAPINFOHEADER {
    // DWORD bi_size;
    pub bi_size: u32,
    // LONG bi_width;
    pub bi_width: libc::c_long,
    // LONG bi_height;
    pub bi_height: libc::c_long,
    // WORD bi_planes;
    pub bi_planes: u16,
    // WORD bi_bit_count;
    pub bi_bit_count: u16,
    // DWORD bi_compression;
    pub bi_compression: u32,
    // DWORD bi_size_image;
    pub bi_size_image: u32,
    // LONG bi_x_pels_per_meter;
    pub bi_x_pels_per_meter: libc::c_long,
    // LONG bi_y_pels_per_meter;
    pub bi_y_pels_per_meter: libc::c_long,
    // DWORD bi_clr_used;
    pub bi_clr_used: u32,
    // DWORD bi_clr_important;
    pub bi_clr_important: u32,
}

pub struct PAINTSTRUCT16 {
    // let hdc: HDC16;
    pub hdc: HDC16,
    // BOOL16 f_erase;
    pub f_erase: u16,
    // struct let rc_paint: RECT16;
    pub rc_paint: RECT16,
    // BOOL16 f_restore;
    pub f_restore: u16,
    // BOOL16 f_inc_update;
    pub f_inc_updated: u16,
    // BYTE rgb_reserved[16];
    pub rgb_reserved: [u8; 16],
}

pub struct tagRGBQUAD {
    // BYTE rgbBlue;
    pub rgbBlue: u8,
    // BYTE rgbGreen;
    pub rgbGreen: u8,
    // BYTE rgbRed;
    pub rgbRed: u8,
    // BYTE rgbReserved;
    pub rgbReserved: u8,
}

pub struct BITMAPINFO {
    // BITMAPINFOHEADER bim_header;
    pub bim_header: BITMAPINFOHEADER,
    // RGBQUAD bmi_colors;
    pub bmi_colors: tagRGBQUAD,
}

// typedef uu16: i16;

// typedef let HPALETTE16: HANDLE16;

// typedef let HRSRC16: HANDLE16;

// typedef ulong u32;

// WARNING! conflicting data type names: /RGBQUAD - /wingdi.h/RGBQUAD

// typedef struct tagMSG tagMSG, *PtagMSG;

// typedef struct tagMSG MSG;

// typedef struct HWND__ HWND__, *PHWND__;

// typedef struct HWND__ * HWND;

// typedef uint UINT;

// typedef uint UINTPTR;

// typedef UINTPTR WPARAM;

// typedef struct tagPOINT tagPOINT, *PtagPOINT;

// typedef struct tagPOINT POINT;

pub struct tagPOINT {
    // LONG x;
    pub x: libc::c_long,
    // LONG y;
    pub y: libc::c_long,
}

pub struct tagMSG {
    // HWND hwnd;
    pub hwnd: HWND16,
    // UINT message;
    pub message: libc::c_uint,
    // WPARAM wParam;
    pub wParam: WPARAM16,
    // lParam: LPARAM;
    pub lParam: LPARAM,
    // DWORD time;
    pub time: u32,
    // POINT pt;
    pub pt: POINT16,
}

// struct HWND__ {
//     int unused;
// };

// typedef struct _devicemodeA _devicemodeA, *P_devicemodeA;

// typedef struct _devicemodeA DEVMODEA;

// typedef union _union_655 _union_655, *P_union_655;

// typedef union _union_658 _union_658, *P_union_658;

// typedef struct _struct_656 _struct_656, *P_struct_656;

// typedef struct _struct_657 _struct_657, *P_struct_657;

// typedef struct _POINTL _POINTL, *P_POINTL;

// typedef struct _POINTL POINTL;

pub struct _POINTL {
    // LONG x;
    pub x: libc::c_long,
    // LONG y;
    pub y: libc::c_long,
}

pub struct _struct_657 {
    // POINTL dmPosition;
    pub dmPosition: _POINTL,
    // DWORD dmDisplayOrientation;
    pub dmDisplayOrientation: u32,
    // DWORD dmDisplayFixedOutput;
    pub dmDisplayFixedOutput: u32,
}

pub struct _struct_656 {
    // dmOrientation: i16;
    pub dmOrientation: i16,
    // dmPaperSize: i16;
    pub dmPaperSize: i16,
    // dmPaperLength: i16;
    pub dmPaperLength: i16,
    // dmPaperWidth: i16;
    pub dmPaperWidth: i16,
    // dmScale: i16;
    pub dmScale: i16,
    // dmCopies: i16;
    pub dmCopies: i16,
    // dmDefaultSource: i16;
    pub dmDefaultSource: i16,
    // dmPrintQuality: i16;
    pub dmPrintQuality: i16,
}

// union _union_655 {
//     struct _struct_656 field0;
//     struct _struct_657 field1;
// };

// union _union_658 {
//     DWORD dmDisplayFlags;
//     DWORD dmNup;
// };

pub struct DEVMODEA {
    // BYTE dmDeviceName[32];
    pub dmDeviceName: [u8; 32],
    // WORD dmSpecVersion;
    pub dmSpecVersion: u16,
    // WORD dmDriverVersion;
    pub dmDriverVersion: u16,
    // WORD dmSize;
    pub dmSize: u16,
    // WORD dmDriverExtra;
    pub dmDriverExtra: u16,
    // DWORD dmFields;
    pub dmFields: u32,
    // union _union_655 field_0x2c;
    pub field_0x2c: _struct_656,
    // dmColor: i16;
    pub dmColor: i16,
    // dmDuplex: i16;
    pub dmDuplex: i16,
    // dmYResolution: i16;
    pub dmYResolution: i16,
    // dmTTOption: i16;
    pub dmTTOption: i16,
    // dmCollate: i16;
    pub dmCollate: i16,
    // BYTE dmFormName[32];
    pub dmFormName: [u8; 32],
    // WORD dmLogPixels;
    pub dmLogPixels: i16,
    // DWORD dmBitsPerPel;
    pub dmBitsPerPel: u32,
    // DWORD dmPelsWidth;
    pub dmPelsWidth: u32,
    // DWORD dmPelsHeight;
    pub dmPelsHeight: u32,
    // union _union_658 field_0x74;
    pub field_0x74: _struct_656,
    // DWORD dmDisplayFrequency;
    pub dmDisplayFrequency: u32,
    // DWORD dmICMMethod;
    pub dmICMMethod: u32,
    // DWORD dmICMIntent;
    pub dmICMIntent: u32,
    // DWORD dmMediaType;
    pub dmMediaType: u32,
    // DWORD dmDitherType;
    pub dmDitherType: u32,
    // DWORD dmReserved1;
    pub dmReserved1: u32,
    // DWORD dmReserved2;
    pub dmReserved2: u32,
    // DWORD dmPanningWidth;
    pub dmPanningWidth: u32,
    // DWORD dmPanningHeight;
    pub dmPanningHeight: u32,
}

//typedef struct tagBITMAPINFOHEADER tagBITMAPINFOHEADER, *PtagBITMAPINFOHEADER;

pub struct tagBITMAPINFOHEADER {
    // DWORD biSize;
    pub biSize: u32,
    // LONG biWidth;
    pub biWidth: libc::c_long,
    // LONG biHeight;
    pub biHeight: libc::c_long,
    // WORD biPlanes;
    pub biPlanes: u16,
    // WORD biBitCount;
    pub biBitCount: u16,
    // DWORD biCompression;
    pub biCompression: u32,
    // DWORD biSizeImage;
    pub biSizeImage: u32,
    // LONG biXPelsPerMeter;
    pub biXPelsPerMeter: libc::c_long,
    // LONG biYPelsPerMeter;
    pub biYPelsPerMeter: libc::c_long,
    // DWORD biClrUsed;
    pub biClrUsed: u32,
    // DWORD biClrImportant;
    pub biClrImportant: u32,
}

// typedef struct _CONTEXT _CONTEXT, *P_CONTEXT;
//
// typedef struct _CONTEXT CONTEXT;
//
// typedef struct _FLOATING_SAVE_AREA _FLOATING_SAVE_AREA, *P_FLOATING_SAVE_AREA;
//
// typedef struct _FLOATING_SAVE_AREA FLOATING_SAVE_AREA;

// typedef CHAR: u8;

// typedef CHAR * LPCSTR;

// typedef CHAR * LPSTR;

// typedef CHAR * PCHAR;

// typedef void * PVOID;

// typedef uint UINT32;

// typedef uUINT8: u8;

// typedef ulong ULONGPTR;

// typedef int16_t: i16;

// typedef ulong ULONG;

// typedef uUCHAR: u8;

// typedef void * LPCVOID;
