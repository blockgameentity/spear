use pelite::pe64::{Pe, PeFile};
use std::fs;

#[allow(dead_code)]
pub fn rva_to_offset_manual(
    sections: &pelite::pe64::headers::SectionHeaders,
    rva: usize,
) -> Option<usize> {
    for section in sections.iter() {
        if rva >= section.VirtualAddress as usize
            && rva < (section.VirtualAddress + section.VirtualSize) as usize
        {
            return Some(rva - section.VirtualAddress as usize + section.PointerToRawData as usize);
        }
    }
    None
}

#[allow(dead_code)]
pub fn walk_dir_simple(
    sections: &pelite::pe64::headers::SectionHeaders,
    buffer: &[u8],
    dir_rva: u32,
    level: usize,
    base_rva: u32,
) {
    if let Some(dir_offset) = rva_to_offset_manual(sections, dir_rva as usize) {
        if dir_offset + 16 > buffer.len() {
            return;
        }
        let num_named =
            u16::from_le_bytes(buffer[dir_offset + 12..dir_offset + 14].try_into().unwrap())
                as usize;
        let num_id =
            u16::from_le_bytes(buffer[dir_offset + 14..dir_offset + 16].try_into().unwrap())
                as usize;
        let num_entries = num_named + num_id;
        let entries_start = dir_offset + 16;
        for i in 0..num_entries {
            let entry_offset = entries_start + i * 8;
            if entry_offset + 8 > buffer.len() {
                continue;
            }
            let offset_to = u32::from_le_bytes(
                buffer[entry_offset + 4..entry_offset + 8]
                    .try_into()
                    .unwrap(),
            );
            if offset_to & 0x80000000 != 0 {
                let sub_dir_rva = base_rva + (offset_to & !0x80000000);
                walk_dir_simple(sections, buffer, sub_dir_rva, level + 1, base_rva);
            } else {
                let data_entry_rva = base_rva + offset_to;
                if let Some(data_entry_offset) =
                    rva_to_offset_manual(sections, data_entry_rva as usize)
                {
                    if data_entry_offset + 16 <= buffer.len() {
                        let res_rva_bytes = &buffer[data_entry_offset..data_entry_offset + 4];
                        let size_bytes = &buffer[data_entry_offset + 4..data_entry_offset + 8];
                        let res_rva = u32::from_le_bytes(res_rva_bytes.try_into().unwrap());
                        let size = u32::from_le_bytes(size_bytes.try_into().unwrap()) as usize;
                        if let Some(res_offset) = rva_to_offset_manual(sections, res_rva as usize) {
                            if res_offset + size <= buffer.len() {
                                let res_data = &buffer[res_offset..res_offset + size];
                                crate::core::resources::analyze_data_cached(res_data);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn parse_pe_resources(exe_path: &std::path::Path) {
    log::info!("[+] Parsing PE for resources...");
    log::info!("[+] Executable path: {:?}", exe_path);
    let buffer = fs::read(&exe_path).unwrap();
    log::info!("[+] Read {} bytes from executable", buffer.len());
    if let Ok(pe) = PeFile::from_bytes(&buffer) {
        log::info!("[+] PE parsed successfully");
        let mut base_rva = 0;
        for section in pe.section_headers().iter() {
            let name = std::str::from_utf8(&section.Name)
                .unwrap_or("")
                .trim_end_matches('\0');
            if name == ".rsrc" {
                base_rva = section.VirtualAddress;
                log::info!("[+] Found .rsrc section at RVA 0x{:x}", base_rva);
                break;
            }
        }
        log::info!("[+] Walking resource directory...");
        walk_dir_simple(&pe.section_headers(), &buffer, base_rva, 0, base_rva);
        log::info!("[+] Resource parsing complete");
    } else {
        log::info!("[!] Failed to parse PE");
    }
}
