use pelite::pe64::Pe;
use pelite::pe64::PeView;
use std::ptr;
use winapi::um::libloaderapi::GetModuleHandleW;

pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn format_option_key(key: &str) -> String {
    let mut result = String::new();
    let mut chars = key.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_uppercase() && !result.is_empty() {
            result.push(' ');
        }
        result.push(c);
    }
    capitalize_first(&result)
}

pub fn get_text_section() -> Option<(*const u8, usize)> {
    let h_module = unsafe { GetModuleHandleW(ptr::null()) };
    if h_module.is_null() {
        return None;
    }
    let image_base = h_module as usize;
    let mut buffer = vec![0u8; 0x1000];
    unsafe {
        ptr::copy_nonoverlapping(h_module as *const u8, buffer.as_mut_ptr(), buffer.len());
    }
    let pe = PeView::from_bytes(&buffer).ok()?;
    for section in pe.section_headers() {
        if let Ok(name) = std::str::from_utf8(&section.Name) {
            if name.trim_end_matches('\0') == ".text" {
                let rva = section.VirtualAddress as usize;
                let size = section.VirtualSize as usize;
                let va = image_base + rva;
                return Some((va as *const u8, size));
            }
        }
    }
    None
}
