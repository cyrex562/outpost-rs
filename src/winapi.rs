#![allow(non_snake_case)]

use crate::win_struct::{ATOM, BITMAPINFO, COLORREF, CONTEXT, HANDLE16, HBRUSH16, HCURSOR16, HDC16, HFILE16, HGDIOBJ16, HGLOBAL16, HICON16, HINSTANCE16, HMENU16, HMODULE16, HPALETTE16, HPEN16, HRSRC16, HTASK16, HWND16, LOGPALETTE, LPARAM, LRESULT, MSG16, POINT16, RECT16, SEGPTR, WNDCLASS16, WPARAM16};
use std::hint::unreachable_unchecked;

// void FatalExit(void)
pub fn FatalExit() {
    unimplemented!()
}
// DWORD GetVersion16(void)
pub fn GetVersion16() -> u32 {
    unimplemented!()
}
// HGLOBAL16 GLobalAlloc16(UINT16 flags, DWORD size)
pub fn GLobalAlloc16(flags: u16, size: u32) -> HGLOBAL16 {
    unimplemented!()
}
// HGLOBAL16 GlobalReAlloc16(HGLOBAL16 handle, DWORD size, UINT16 flags)
pub fn GlobalReAlloc16(handle: HGLOBAL16, size: u32, flags: u16) -> HGLOBAL16 {
    unimplemented!()
}
// HGLOBAL16 GlobalFree16(HGLOBAL16 handle)
pub fn GlobalFree16(handle: HGLOBAL16) -> HGLOBAL16 {
    unimplemented!()
}
// SEGPTR WIN16_GlobalLock16(HGLOBAL16 handle)
pub fn WIN16_GlobalLock16(handle: HGLOBAL16) -> SEGPTR {
    unimplemented!()
}
// BOOL16 GlobalUnlock16(HGLOBAL16 handle)
pub fn GlobalUnlock16(handle: HGLOBAL16) -> bool {
    unimplemented!()
}
// DWORD GlobalSize16(HGLOBAL16 handle)
pub fn GlobalSize16(handle: HGLOBAL16) -> u32 {
    unimplemented!()
}
// DWORD GlobalHandle16(WORD sel)
pub fn GlobalHandle16(sel: u16) -> u32 {
    unimplemented!()
}
// HGLOBAL16 LockSegment16(HGLOBAL16 handle)
pub fn LockSegment16(handle: HGLOBAL16) -> HGLOBAL16 {
    unimplemented!()
}
// BOOL16 WaitEvent16(HTASK16 h_task)
pub fn WaitEvent16(task: HTASK16) -> bool {
    unimplemented!()
}
// INT16 GetModuleFileName16(HINSTANCE16 h_module, LPSTR lp_file_name, INT16 n_size)
pub fn GetModuleFileName16(h_module: HINSTANCE16, lp_file_name: &String, n_size: i16) -> i16 {
    unimplemented!()
}
// LPVOID MakeProcInstance16(LPVOID func, HANDLE16 h_instance)
pub fn MakeProcInstance16(func: fn(), h_instance: HANDLE16) -> *mut u8 {
    unimplemented!()
}
// void FreeProcInstance16(LPVOID func)
pub fn FreeProcInstance16(func: fn()) {
    unimplemented!()
}
// HRSRC16 FindResource16(HMODULE16 h_module, LPCSTR name, LPCSTR type)
pub fn FindResource16(h_module: HMODULE16, name: &String, rsrc_type: &String) -> HRSRC16 {
    unimplemented!()
}
// HGLOBAL16 LoadResource16(HMODULE16 h_module, HRSRC16 h_rsrc)
pub fn LoadResource16(h_module: HMODULE16, h_rsrc: HRSRC16) -> HGLOBAL16 {
    unimplemented!()
}
// SEGPTR WIN16_LockResource16(HGLOBAL16 handle)
pub fn WIN16_LockResource16(handle: HGLOBAL16) -> SEGPTR {
    unimplemented!()
}
// BOOL16 FreeResource16(HGLOBAL16 handle)
pub fn FreeResource16(handle: HGLOBAL16) -> bool {
    unimplemented!()
}
// HFILE16 _lclose16(HFILE16 h_file)
pub fn _lclose16(h_file: HFILE16) -> HFILE16 {
    unimplemented!()
}
// HFILE16 _lcreat16(LPCSTR path, INT16 attr)
pub fn _lcreat16(path: &String, attr: i16) -> HFILE16 {
    unimplemented!()
}
// long _llseek16(HFILE16 h_file, long l_offset, INT16 n_origin)
pub fn _llseek16(h_file: HFILE16, l_offset: libc::c_long, n_origin: i16) -> libc::c_long {
    unimplemented!()
}
// HFILE16 _lopen16(LPCSTR path, INT16 mode)
pub fn _lopen16(path: &String, mode: i16) -> HFILE16 {
    unimplemented!()
}
// INT16 lstrlen16(LPCSTR str)
pub fn lstrlen16(a: &String) -> i16 {
    unimplemented!()
}
// void InitTask16(CONTEXT * context)
pub fn InitTask16(context: &mut CONTEXT) {
    unimplemented!()
}
// void DOS3Call(CONTEXT * context)
pub fn DOS3Call(context: &mut CONTEXT) {
    unimplemented!()
}
// UINT16 SetErrorMode16(UINT16 mode)
pub fn SetErrorMode16(mode: u16) -> u16 {
    unimplemented!()
}
// void __AHSHIFT(void)
pub fn __AHSHIFT() {
    unimplemented!()
}
// void __AHINCR(void)
pub fn __AHINCR() {
    unimplemented!()
}
// void OutputDebugString16(LPCSTR str)
pub fn OutputDebugString16(str: &mut String) {
    unimplemented!()
}
// INT16 GetPrivateProfileString16(LPCSTR section, LPCSTR entry, LPCSTR def_val, LPSTR buffer, UINT16 len, LPCSTR filename)
pub fn GetPrivateProfileString16(section: &String, entry: &String, def_val &String, buffer: &mut String, len: usize, filename: &String) -> i16 {
    unimplemented!()
}
// BOOL16 WritePrivateProfileString16(LPCSTR section, LPCSTR entry, LPCSTR string, LPCSTR filename)
pub fn WritePrivateProfileString16(section: &String, entry: &String, str_to_write: &String, filename: &String) -> bool {
    unimplemented!()
}
// SEGPTR GetDOSEnvironment16(void)
pub fn GetDOSEnvironment16() -> SEGPTR {
    unimplemented!()
}
// void FatalAppExit16(UINT16 action, LPCSTR str)
pub fn FatalAppExit16(action: u16, reason: &str) {
    unimplemented!()
}
// HINSTANCE16 WinExec16(LPCSTR lp_cmd_line, UINT16 n_cmd_show)
pub fn WinExec16(lp_cmd_line: &String, n_cmd_show: u16) -> HINSTANCE16 {
    unimplemented!()
}
// void __WINFLAGS(void)
pub fn __WINFLAGS() {
    unimplemented!()
}
// DWORD GlobalDOSAlloc16(DWORD size)
pub fn GlobalDOSAlloc16(size: usize) -> usize {
    unimplemented!()
}
// WORD GlobalDOSFree16(WORD sel)
pub fn GlobalDOSFree16(sel: u16) -> u16 {
    unimplemented!()
}
// WORD GlobalPageLock16(HGLOBAL16 handle)
pub fn GlobalPageLock16(handle: HGLOBAL16) -> u16 {
    unimplemented!()
}
// WORD GlobalPageUnlock16(HGLOBAL16 handle)
pub fn GlobalPageUnlock16(handle: HGLOBAL16) -> u16 {
    unimplemented!()
}
// void hmemcpy16(LPVOID dst, LPCVOID src, long count)
pub fn hmemcpy16(dst: *mut u8, src: *mut u8, count: libc::c_long) {
    unimplemented!()
}
// long WIN16_hread(HFILE16 h_file, SEGPTR buffer, long count)
pub fn WIN16_hread(h_file: HFILE16, buffer: SEGPTR, count: usize) -> usize {
    unimplemented!()
}
// long _hwrite16(HFILE16 h_file, LPCSTR buffer, long count)
pub fn _hwrite16(h_file: HFILE16, buffer: &mut String, count: usize) -> usize {
    unimplemented!()
}
// COLORREF SetBkColor16(HDC16 hdc, COLORREF color)
pub fn SetBkColor16(hdc: HDC16, color: COLORREF) -> COLORREF {
    unimplemented!()
}
// int16_t SetMapMode16(HDC16 hdc, int16_t mode)
pub fn SetMapMode16(hdc: HDC16, mode: i16) -> i16 {
    unimplemented!()
}
// COLORREF SetTextColor16(HDC16 hdc, COLORREF color)
pub fn SetTextColor16(hdc: HDC16, color: COLORREF) -> COLORREF {
    unimplemented!()
}
// BOOL16 LineTo16(HDC16 hdc, INT16 x, INT16 y)
pub fn LineTo16(hdc: HDC16, x: i16, y: i16) -> bool {
    unimplemented!()
}
// DWORD MoveTo16(HDC16 hdc, INT16 x, INT16 y)
pub fn MoveTo16(hdc: HDC16, x: i16, y: i16) -> u32 {
    unimplemented!()
}
// BOOL16 Ellipse16(HDC16 hdc, INT16 left, INT16 top, INT16 right, INT16 bottom)
pub fn Ellipse16(hdc: HDC16, left: i16, top: i16, right: i16, bottom: i16) -> bool {
    unimplemented!()
}
// BOOL16 Rectangle16(HDC16 hdc, INT16 left, INT16 top, INT16 right, INT16 bottom)
pub fn Rectangle16(hdc: HDC16, left: i16, top: i16, right: i16, bottom: i16) -> bool {
    unimplemented!()
}
// BOOL16 TextOut16(HDC16 hdc, INT16 x, INT16 y, char * str, INT16 count)
pub fn TextOut16(hdc: HDC16, x: i16, y: i16, out_text: &String, count: usize) -> bool {
    unimplemented!()
}
// BOOL16 Polygon16(HDC16 hdc, POINT16 * pt, INT16 count)
pub fn Polygon16(hdc: HDC16, pt: &[POINT16], count: i16) -> bool {
    unimplemented!()
}
// HGDIOBJ16 SelectObject16(HDC16 hdc, HGDIOBJ16 handle)
pub fn SelectObject16(hdc: HDC16, handle: HGDIOBJ16) -> HGDIOBJ16 {
    unimplemented!()
}
// HDC16 CreateDC16(LPCSTR driver, LPCSTR device, LPCSTR output, DEVMODEA * init_data)
pub fn CreateDC16(driver: &String, device: &String, output: &String, init_data: &_devicemodeA) -> HDC16 {
    unimplemented!()
}
// HPEN16 CreatePen16(INT16 style, INT16 width, COLORREF color)
pub fn CreatePen16(style: i16, width: i16, color: COLORREF) -> HPEN16 {
    unimplemented!()
}
// HBRUSH16 CreateSolidBrush16(COLORREF color)
pub fn CreateSolidBrush16(color: COLORREF) -> HBRUSH16 {
    unimplemented!()
}
// BOOL16 DeleteDC16(HDC16 hdc)
pub fn DeleteDC16(hdc: HDC16) -> bool {
    unimplemented!()
}
// BOOL16 DeleteObject16(HGDIOBJ16 obj)
pub fn DeleteObject16(obj: HGDIOBJ16) -> bool {
    unimplemented!()
}
// DWORD GetCurrentPosition16(HDC16 hdc)
pub fn GetCurrentPosition16(hdc: HDC16) -> u32 {
    unimplemented!()
}
// INT16 GetDeviceCaps16(HDC16 hdc, INT16 cap)
pub fn GetDeviceCaps16(hdc: HDC16, cap: i16) -> i16 {
    unimplemented!()
}
// HGDIOBJ16 GetStockObject16(INT16 obj)
pub fn GetStockObject16(obj: i16) -> HGDIOBJ16 {
    unimplemented!()
}
// DWORD GetTextExtent16(HDC16 hdc, LPCSTR str, INT16 count)
pub fn GetTextExtent16(hdc: HDC16, text: &String, count: usize) -> u32 {
    unimplemented!()
}
// BOOL16 UnrealizeObject16(HGDIOBJ16 obj)
pub fn UnrealizeObject16(obj: HGDIOBJ16) -> bool {
    unimplemented!()
}
// HPALETTE16 CreatePalette16(LOGPALETTE * palette)
pub fn CreatePalette16(palette: &mut LOGPALETTE) -> HPALETTE16 {
    unimplemented!()
}
// UINT16 GetSystemPaletteEntries(HDC16 hdc, UINT16 start, UINT16 count, PALETTEENTRY * entries)
pub fn GetSystemPaletteEntries(hdc: HDC16, start: u16, count: u16, entries: &[PALETTEENTRY]) -> u16 {

}
// INT16 StretchDIBits16(HDC16 hdc, INT16 x_dst, INT16 y_dst, INT16 width_dst, INT16 height_dst, INT16 x_src, INT16 y_src, INT16 width_src, INT16 height_src, PVOID bits, BITMAPINFO * info, UINT16 w_usage, DWORD dw_rop)
pub fn StretchDIBits16(hdc: HDC16, x_dst: i16, y_dst: i16, width_dst: i16, height_dst: i16, x_src: i16, width_src: i16, height_src: i16, bits: *mut u8, info: &BITMAPINFO, w_usage: u16, dw_rop: u32) -> i16 {
    unimplemented!()
}
// INT16 SetDIBitsToDevice(HDC16 hdc, INT16 x_dest, INT16 y_dest, INT16 cx, INT16 cy, INT16 x_src, INT16 y_src, UINT16 startscan, UINT16 lines, LPCVOID bits, BITMAPINFO * info, UINT16 coloruse)
pub fn SetDIBitsToDevice(hdc: HDC16, x_dest: i16, y_dest: i16, cx: i16, cy: i16, x_src: i16, y_src: i16, startscan: u16, lines: u16, bits: *mut u8, info: &BITMAPINFO, coloruse: u16) -> i16 {
    unimplemented!()
}
// BOOL16 MoveToEx16(HDC16 hdc, INT16 x, INT16 y, POINT16 * pt)
pub fn MoveToEx16(hdc: HDC16, x: i16, y: i16, pt: &POINT16) -> bool {
    unimplemented!()
}
// INT16 MessageBox16(HWND16 hwnd, LPCSTR text, LPCSTR title, UINT16 type)
pub fn MessageBox16(hwnd: HWND16, text: &String, title: &String, box_type: u16) -> i16 {
    unimplemented!()
}
// INT16 InitApp16(HINSTANCE16 h_instance)
pub fn InitApp16(h_inst: HINSTANCE16) -> i16 {
    unimplemented!()
}
// void PostQuitMessage16(INT16 exit_code)
pub fn PostQuitMessage16(exit_code: i16) {
    unimplemented!()
}
// UINT16 SetTimer16(HWND16 hwnd, UINT16 id, UINT16 timeout, LPVOID proc)
pub fn SetTimer16(hwnd: HWND16, id: u16, timeout: u16, proc: u32) -> u16 {
    unimplemented!()
}
// BOOL16 KillTimer16(HWND16 hwnd, UINT16 id)
pub fn KillTimer16(hwnd: HWND16, id: u16) -> bool {
    unimplemented!()
}
// BOOL16 GetCursorPos16(POINT16 * pt)
pub fn GetCursorPos16(pt: &POINT16) -> bool {
    unimplemented!()
}
// HWND16 SetCapture16(HWND16 hwnd)
pub fn SetCapture16(hwnd: HWND16) -> HWND16 {
    unimplemented!()
}
// BOOL16 ReleaseCapture16(void)
pub fn ReleaseCapture16() -> bool {
    unimplemented!()
}
// HWND16 SetFocus16(HWND16 hwnd)
pub fn SetFocus16(hwnd: HWND16) -> HWND16 {
    unimplemented!()
}
// HANDLE16 RemoveProp16(HWND16 hwnd, LPCSTR str)
pub fn RemoveProp16(hwnd: HWND16, text: &String) -> HANDLE16 {
    unimplemented!()
}
// HANDLE16 GetProp16(HWND16 hwnd, LPCSTR str)
pub fn GetProp16(hwnd: HWND16, text: &String) -> HANDLE16 {
    unimplemented!()
}
// BOOL16 SetProp16(HWND16 hwnd, LPCSTR str, HANDLE16 handle)
pub fn SetProp16(hwnd: HWND16, text: &String, handle:HANDLE16) -> bool {
    unimplemented!()
}
// void ClientToScreen16(HWND16 hwnd, POINT16 * lppnt)
pub fn ClientToScreen16(hwnd: HWND16, lppnt: &POINT16) {
    unimplemented!()
}
// void ScreenToClient16(HWND16 hwnd, POINT16 * lppnt)
pub fn ScreenToClient16(hwnd: HWND16, lppnt: &POINT16) {
    unimplemented!()
}
// BOOL16 IsIconic16(HWND16 hwnd)
pub fn IsIconic16(hwnd: HWND16) -> bool {
    unimplemented!()
}
// void GetWindowRect16(HWND16 hwnd, RECT16 * rect)
pub fn GetWindowRect16(hwnd: HWND16, rect: &mut RECT16) {
    unimplemented!()
}
// void GetClientRect16(HWND16 hwnd, RECT16 * rect)
pub fn GetClientRect16(hwnd: HWND16, rect: &mut RECT16) {
    unimplemented!()
}
// BOOL16 EnableWindow16(HWND16 hwnd, BOOL16 enable)
pub fn EnableWindow16(hwnd: HWND16, enable: bool) -> bool {
    unimplemented!()
}
// BOOL16 IsWindowEnabled16(HWND16 hwnd)
pub fn IsWindowEnabled16(hwnd: HWND16) -> bool {
    unimplemented!()
}
// INT16 GetWindowText16(HWND16 hwnd, SEGPTR lp_string, INT16 n_max_count)
pub fn GetWindowText16(hwnd: HWND16, lp_string: &SEGPTR, n_max_count: i16) -> i16 {
    unimplemented!()
}
// BOOL16 SetWindowText16(HWND16 hwnd, SEGPTR lp_string)
pub fn SetWindowText16(hwnd: HWND16, lp_string: &SEGPTR) -> bool {
    unimplemented!()
}
// HDC16 BeginPaint16(HWND16 hwnd, PAINTSTRUCT16 * lps)
pub fn BeginPaint16(hwnd: HWND16, lps: &mut PAINSTRUCT16) -> HDC16 {
    unimplemented!()
}
// BOOL16 EndPaint16(HWND16 hwnd, PAINTSTRUCT16 * lps)
pub fn EndPaint16(hwnd: HWND16, lps: &mut PAINSTRUCT16) -> bool {
    unimplemented!()
}
// HWND16 CreateWindow16(LPCSTR class_name, LPCSTR window_name, DWORD style, INT16 x, INT16 y, INT16 width, INT16 height, HWND16 parent, HMENU16 hmenu, HINSTANCE16 instance, LPVOID data)
pub fn CreateWindow16(class_name: &String, window_name: &String, style: u32, x: i16, y: i16, width: i16, height: i16, parent: HWND16, hmenu: HMENU16, instance: HINSTANCE16, data: *mut u8) -> HWND16 {
    unimplemented!()
}
// BOOL16 ShowWindow16(HWND16 hwnd, INT16 cmd)
pub fn ShowWindow16(hwnd: HWND16, cmd: i16) -> bool {
    unimplemented!()
}
// BOOL16 BringWindowToTop16(HWND16 hwnd)
pub fn BringWindowToTop16(hwnd: HWND16) -> bool {
    unimplemented!()
}
// BOOL16 IsWindow16(HWND16 hwnd)
pub fn IsWindow16(hwnd: HWND16) -> bool {
    unimplemented!()
}
// BOOL16 DestroyWindow16(HWND16 hwnd)
pub fn DestroyWindow16(hwnd: HWND16) -> bool {}
// BOOL16 EnumChildWindows1(HWND16 parent, LPVOID func, LPARAM lparam)
pub fn EnumChildWindows1(parent: HWND16, func: fn(), lparam: LPARAM) -> bool {
    unimplemented!()
}
// BOOL16 MoveWindow16(HWND16 hwnd, INT16 x, INT16 y, INT16 cx, INT16 cy, BOOL16 repaint)
pub fn MoveWindow16(hwnd: HWND16, x: i16, y: i16, cx: i16, cy: i16, repaint: bool) -> bool {
    unimplemented!()
}
// ATOM RegisterClass16(WNDCLASS16 * wc)
pub fn RegisterClass16(wc: &WNDCLASS16) -> ATOM {
    unimplemented!()
}
// HDC16 GetDC16(HWND16 hwnd)
pub fn GetDC16(hwnd: HWND16) -> HDC16 {
    unimplemented!()
}
// HDC16 GetWindowDC16(HWND16 hwnd)
pub fn GetWindowDC16(hwnd: HWND16) -> HDC16 {
    unimplemented!()
}
// INT16 ReleaseDC16(HWND16 hwnd, HDC16 hdc)
pub fn ReleaseDC16(hwnd: HWND16, hdc: HDC16) -> i16 {
    unimplemented!()
}
// HCURSOR16 SetCursor16(HCURSOR16 hcursor)
pub fn SetCursor16(hcursor: HCURSOR16) -> HCURSOR16 {
    unimplemented!()
}
// INT16 ShowCursor16(BOOL16 b_show)
pub fn ShowCursor16(b_show: bool) -> i16 {
    unimplemented!()
}
// BOOL16 PtInRect16(RECT16 * rect, POINT16 pt)
pub fn PtInRect16(rect: &mut RECT16, pt: POINT16) -> bool {
    unimplemented!()
}
// INT16 FillRect16(HDC16 hdc, RECT16 * rect, HBRUSH16 hbrush)
pub fn FillRect16(hdc: HDC16, rect: &RECT16, hbrush: HBRUSH16) -> i16 {
    unimplemented!()
}
// INT16 FrameRect16(HDC16 hdc, RECT16 * rect, HBRUSH16 hbrush)
pub fn FrameRect16(hdc: HDC16, rect: &RECT16, hbrush: HBRUSH16) -> i16 {
    unimplemented!()
}
// BOOL16 DrawIcon16(HDC16 hdc, INT16 x, INT16 y, HICON16 h_icon)
pub fn DrawIcon16(hdc: HDC16, x: i16, y: i16, h_icon: HICON16) -> bool {
    unimplemented!()
}
// INT16 DrawText16(HDC16 hdc, LPCSTR str, INT16 count, RECT16 * rect, UINT16 flags)
pub fn DrawText16(hdc: HDC16, text: &String, count: i16, rect: &RECT16, flags: u16) -> i16 {
    unimplemented!()
}
// HWND16 CreateDialog16(HINSTANCE16 hinst, LPCSTR dlg_template, HWND16 owner, LPVOID dlg_proc)
pub fn CreateDialog16(h_inst: HINSTANCE16, dlg_template: &String, owner: HWND16, dlg_proc: fn()) -> HWND16 {
    unimplemented!()
}
// BOOL16 IsDialogMessage16(HWND16 hwnd_dlg, MSG16 * msg16)
pub fn IsDialogMessage16(hwnd_dlg: HWND16, msg: &mut MSG16) -> bool {
    unimplemented!()
}
// HWND16 GetDlgItem16(HWND16 hwnd_dlg, INT16 id)
pub fn GetDlgItem16(hwnd_dlg: HWND16, id: i16) -> HWND16 {
    unimplemented!()
}
// void SetDlgItemText16(HWND16 hwnd, INT16 id, SEGPTR lp_string)
pub fn SetDlgItemText16(hwnd: HWND16, id: i16, lp_string: &SEGPTR) {
    unimplemented!()
}
// void SetDlgItemInt16(HWND16 hwnd, INT16 id, UINT16 value, BOOL16 f_signed)
// UINT16 GetDlgItemInt16(HWND16 hwnd, INT16 id, BOOL16 * translated, BOOL16 f_signed)
// BOOL16 CheckRadioButton16(HWND16 hwnd_dlg, UINT16 first_id, UINT16 last_id, UINT16 check_id)
// BOOL16 CheckDlgButton16(HWND16 hwnd, INT16 id, UINT16 check)
// UINT16 IsDlgButtonChecked(HWND16 hwnd, UINT16 id)
pub fn IsDlgButtonChecked(hwnd: HWND16, id: u16) -> u16 {
    unimplemented!()
}
// LRESULT SendDlgItemMessage16(HWND16 hwnd, INT16 id, UINT16 msg, WPARAM16 w_param, LPARAM l_param)
// void MapDialogRect16(HWND16 hwnd, RECT16 * rect)
// void MessageBeep16(UINT16 i)
// LRESULT DefWindowProc16(HWND16 hwnd, UINT16 msg, WPARAM16 wparam, LPARAM lparam)
// BOOL16 GetMessage16(MSG16 * msg, HWND16 hwnd, UINT16 first, UINT16 last)
pub fn GetMessage16(msg: &mut MSG16, hwnd: HWND16, first: u16, last: u16) -> bool {
    unimplemented!()
}
// BOOL16 PostMessage16(HWND16 hwnd, UINT16 msg, WPARAM16 wparam, LPARAM lparam)
pub fn PostMessage16(hwnd: HWND16, msg: u16, wparam: WPARAM16, lparam: LPARAM) -> bool {
    unimplemented!()
}
// LRESULT SendMessage16(HWND16 hwnd, UINT16 msg, WPARAM16 wparam, LPARAM lparam)
pub fn SendMessage16(hwnd: HWND16, msg: u16, wparam: WPARAM16, lparam: LPARAM) -> LRESULT {
    unimplemented!()
}
// BOOL16 TranslateMessage16(MSG16 * msg)
// long DispatchMessage16(MSG16 * msg)
// LRESULT CallWindowProc16(LPVOID func, HWND16 hwnd, UINT16 msg, WPARAM16 wparam, LPARAM lparam)
// void UpdateWindow16(HWND16 hwnd)
// void InvalidateRect16(HWND16 hwnd, RECT16 * rect, BOOL16 erase)
// void ValidateRect16(HWND16 hwnd, RECT16 * rect)
// WORD GetWindowWord16(HWND16 hwnd, INT16 offset)
pub fn GetWindowWord16(hwnd: HWND16, offset: i16) -> i16 {
    unimplemented!()
}
// WORD SetWindowWord16(HWND16 hwnd, INT16 offset, WORD newval)
// long GetWindowLong16(HWND16 hwnd, INT16 offset)
pub fn GetWindowLong16(hwnd: HWND16, offset: i16) -> libc::c_long {
    unimplemented!()
}
// long SetWindowLong16(HWND16 hwnd, INT16 offset, long newval)
// HMENU16 LoadMenu16(HINSTANCE16 instance, LPCSTR name)
// BOOL16 DestroyMenu16(HMENU16 menu)
pub fn DestroyMenu16(menu: HMENU16) -> bool {
    unimplemented!()
}
// BOOL16 CheckMenuItem16(HMENU16 hmenu, UINT16 w_item_id, UINT16 w_flags)
// BOOL16 EnableMenuItem16(HMENU16 hmenu, UINT16 w_item_id, UINT16 w_flags)
// HMENU16 GetSubMenu16(HMENU16 h_menu, INT16 n_pos)
// BOOL16 WinHelp16(HWND16 hwnd, LPCSTR lp_help_file, UINT16 w_command, DWORD dw_data)
// HCURSOR16 LoadCursor16(HINSTANCE16 h_instance, LPCSTR name)
// HICON16 LoadIcon16(HINSTANCE16 h_instance, LPCSTR name)
// INT16 LoadString16(HINSTANCE16 instance, UINT16 resource_id, LPSTR buffer, INT16 buf_len)
pub fn LoadString16(
    instance: HINSTANCE16,
    resource_id: u16,
    buffer: &mut String,
    buf_len: u16,
) -> i16 {
    unimplemented!()
}
// HACCEL16 LoadAccelerators16(HINSTANCE16 instance, LPCSTR lp_table_name)
// INT16 TranslateAccelerator16(HWND16 hwnd, HACCEL16 haccel, MSG16 * msg)
// INT16 GetSystemMetrics16(INT16 index)
// COLORREF GetSysColor16(INT16 index)
// void SetSysColors16(INT16 count, INT16 * list, COLORREF * values)
// BOOL16 GrayString16(HDC16 hdc, HBRUSH16 param_2, LPVOID gsprc, LPARAM lparam, INT16 cch, INT16 x, INT16 y, INT16 cx, INT16 cy)
// HWND16 SetSysModalWindow(HWND16 hwnd)
// HWND16 GetNextDlgTabItem16(HWND16 hwnd_dlg, HWND16 hwnd_ctrl, BOOL16 f_previous)
// BOOL16 SetWindowPos16(HWND16 hwnd, HWND16 hwnd_insert_after, INT16 x, INT16 y, INT16 cx, INT16 cy, WORD flags)
// UINT16 GetMenuState16(HMENU16 hmenu, UINT16 w_item_id, UINT16 w_flags)
// INT16 GetDlgCtrlID16(HWND16 hwnd)
// HPALETTE16 SelectPalette16(HDC16 hdc, HPALETTE16 hpal, BOOL16 b_force_background)
pub fn SelectPalette16(hdc: HDC16, hpal: HPALETTE16, b_force_background: bool) -> HPALETTE16 {
    unimplemented!()
}
// UINT16 RealizePalette16(HDC16 hdc)
// BOOL16 GetWindowPlacement16(HWND16 hwnd, WINDOWPLACEMENT16 * wp16)
// BOOL16 SetWindowPlacement16(HWND16 hwnd, WINDOWPLACEMENT16 * wp16)
// BOOL16 GetClassInfo16(HINSTANCE16 h_inst16, SEGPTR name, WNDCLASS16 * wc)
// BOOL16 InsertMenu16(HMENU16 hmenu, UINT16 pos, UINT16 flags, UINT16 id, SEGPTR data)
// BOOL16 DeleteMenu16(HMENU16 hmenu, UINT16 npos, UINT16 wflags)
// BOOL16 ModifyMenu16(HMENU16 hmenu, UINT16 pos, UINT16 flags, UINT16 id, SEGPTR data)
// BOOL16 TrackPopupMenu16(HMENU16 hmenu, UINT16 wflags, INT16 x, INT16 y, INT16 n_reserved, HWND16 hwnd, RECT16 * lp_rect)
// INT16 wsprintf16(LPSTR buffer, LPCSTR spec, WORD * valist)
// INT16 wvsprintf16(LPSTR buffer, LPCSTR spec, WORD * args)
pub fn wvsprintf16(buffer: &mut String, spec: &mut LPCSTR, args: *mut ushort) -> i16 {
    unimplemented!()
}
// HWND16 CreateWIndowEx16(DWORD ex_style, LPCSTR class_name, LPCSTR window_name, DWORD style, INT16 x, INT16 y, INT16 width, INT16 height, HWND16 parent, HMENU16 hmenu, HINSTANCE16 instance, LPVOID data)
// BOOL16 DestroyIcon16(HICON16 h_icon)
pub fn DestroyIcon16(h_icon: HICON16) -> bool {
    unimplemented!()
}
// BOOL16 DestroyCursor16(HCURSOR16 h_cursor)
// DWORD mciSendCommand16(UINT16 w_dev_id, UINT16 w_msg, DWORD dw_param1, DWORD p2)
pub fn mciSendCommand16(w_dev_id: u16, w_msg: u16, dw_parm1: u32, p2: u32) -> u32 {
    unimplemented!()
}
// BOOL16 mciGetErrorString16(DWORD w_error, LPSTR lp_str_buffer, UINT16 u_length)
pub fn mciGetErrorString16(w_error: u32, lp_str_buffer: &mut String, u_length: u16) -> bool {
    unimplemented!()
}
// BOOL16 GetOpenFileName16(SEGPTR ofn)
// BOOL16 GetSaveFileName16(SEGPTR ofn)

pub fn swi(a: u16) -> u32 {
    unimplemented!()
}

pub fn SegmentLimit(param_1: u16) ->u16 {
    unimplemented!()
}