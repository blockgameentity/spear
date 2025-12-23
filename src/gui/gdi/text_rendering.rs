use crate::core::resources::*;
use winapi::shared::windef::SIZE;
use winapi::um::wingdi::{
    AddFontMemResourceEx, BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleDC,
    CreateDIBSection, CreateFontW, DIB_RGB_COLORS, DeleteDC, DeleteObject, GetTextExtentPoint32W,
    RGB, SelectObject, SetBkMode, SetTextColor, TRANSPARENT, TextOutW,
};
use winapi::um::winuser::{GetDC, ReleaseDC};

#[allow(dead_code)]
pub fn render_text_texture(
    ctx: &egui::Context,
    text: &str,
    font_height: i32,
    font_weight: i32,
    text_width: f32,
    text_height: f32,
) -> (Option<egui::TextureHandle>, egui::Vec2) {
    log::info!("[+] Rendering text texture for: {}", text);
    let text_utf16: Vec<u16> = text.encode_utf16().collect();
    let text_len = text_utf16.len() as i32;
    if let Some(font_data) = &*FONT_DATA.lock().unwrap() {
        log::info!("[+] Adding font memory resource...");
        unsafe {
            AddFontMemResourceEx(
                font_data.as_ptr() as *mut winapi::ctypes::c_void,
                font_data.len() as u32,
                std::ptr::null_mut(),
                &mut 0,
            );
        }
        log::info!("[+] Font loaded");
    } else {
        log::info!("[!] Font data not available");
    }
    let font_name = "NeueHaasGroteskText Pro Md".to_string();
    log::info!("Parsed font name: {}", font_name);
    let font_name_wide: Vec<u16> = font_name.encode_utf16().chain(std::iter::once(0)).collect();

    let screen_dc = unsafe { GetDC(std::ptr::null_mut()) };
    log::info!("[+] Got screen DC");

    let hdc = unsafe { CreateCompatibleDC(screen_dc) };
    log::info!("[+] Compatible DC created");

    let hfont = unsafe {
        CreateFontW(
            font_height,
            0,
            0,
            0,
            font_weight,
            0,
            0,
            0,
            1,
            4,
            0,
            6, // CLEARTYPE_NATURAL_QUALITY
            2,
            font_name_wide.as_ptr(),
        )
    };
    if hfont.is_null() {
        log::info!("[!] Font creation failed");
        unsafe { DeleteDC(hdc) };
        unsafe { ReleaseDC(std::ptr::null_mut(), screen_dc) };
        return (None, egui::Vec2::ZERO);
    }
    log::info!("[+] Font created");
    unsafe { SelectObject(hdc, hfont as *mut _) };

    let mut size: SIZE = unsafe { std::mem::zeroed() };
    unsafe { GetTextExtentPoint32W(hdc, text_utf16.as_ptr(), text_len, &mut size) };
    log::info!("[+] Text size measured: {}x{}", size.cx, size.cy);

    let mut bmi: BITMAPINFO = unsafe { std::mem::zeroed() };
    bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = size.cx;
    bmi.bmiHeader.biHeight = -size.cy; // top-down
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB;

    let mut pixels_ptr: *mut winapi::ctypes::c_void = std::ptr::null_mut();
    let bitmap = unsafe {
        CreateDIBSection(
            screen_dc,
            &bmi,
            DIB_RGB_COLORS,
            &mut pixels_ptr as *mut *mut winapi::ctypes::c_void,
            std::ptr::null_mut(),
            0,
        )
    };
    unsafe { SelectObject(hdc, bitmap as *mut _) };
    unsafe { SetBkMode(hdc, TRANSPARENT as i32) };
    unsafe { SetTextColor(hdc, RGB(255, 255, 255)) };
    unsafe { TextOutW(hdc, 0, 0, text_utf16.as_ptr(), text_len) };
    log::info!("[+] Text rendered to bitmap");

    // Convert pixels to RGBA with alpha = brightness
    // TODO: implement better AA
    let pixels = unsafe {
        std::slice::from_raw_parts(pixels_ptr as *const u8, (size.cx * size.cy * 4) as usize)
    };
    let mut rgba_pixels = pixels.to_vec();
    for chunk in rgba_pixels.chunks_exact_mut(4) {
        let b = chunk[0];
        let g = chunk[1];
        let r = chunk[2];
        if r == 0 && g == 0 && b == 0 {
            chunk[3] = 0; // transparent background
        } else {
            chunk[3] = 255; // opaque text with GDI antialiasing
        }
    }

    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        [size.cx as usize, size.cy as usize],
        &rgba_pixels,
    );
    let texture = Some(ctx.load_texture("text", color_image, egui::TextureOptions::default()));
    let text_size = egui::vec2(text_width, text_height);
    log::info!("[+] Text texture created");

    unsafe { DeleteObject(bitmap as *mut _) };
    unsafe { DeleteObject(hfont as *mut _) };
    unsafe { DeleteDC(hdc) };
    unsafe { ReleaseDC(std::ptr::null_mut(), screen_dc) };
    log::info!("[+] GDI cleanup done");

    (texture, text_size)
}
