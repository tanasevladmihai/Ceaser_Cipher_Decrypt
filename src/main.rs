use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

struct Signature {
    ext: &'static str,
    magic: &'static [u8],
    offset: usize,
    note: &'static str,
}

const SIGNATURES: &[Signature] = &[
    Signature { ext: "png",  magic: &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], offset: 0, note: "PNG Image" },
    Signature { ext: "jpg",  magic: &[0xFF, 0xD8, 0xFF], offset: 0, note: "JPEG Image" },
    Signature { ext: "pdf",  magic: b"%PDF", offset: 0, note: "PDF Document" },
    Signature { ext: "bmp",  magic: b"BM", offset: 0, note: "BMP Image" },
    Signature { ext: "exe",  magic: b"MZ", offset: 0, note: "Windows Executable" },
    Signature { ext: "zip",  magic: b"PK\x03\x04", offset: 0, note: "ZIP or Office Open XML (DOCX/XLSX/PPTX)" },
    Signature { ext: "ole",  magic: &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1], offset: 0, note: "Legacy Office (DOC/XLS/PPT)" },
    Signature { ext: "gz",   magic: &[0x1F, 0x8B], offset: 0, note: "GZIP/TAR.GZ Archive" },
    Signature { ext: "mp3",  magic: b"ID3", offset: 0, note: "MP3 Audio (ID3)" },
    Signature { ext: "avi",  magic: b"RIFF", offset: 0, note: "AVI Video (RIFF)" },
    Signature { ext: "mp4",  magic: b"ftyp", offset: 4, note: "MP4 Video" },
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "tanase_vlad";
    if !Path::new(path).exists() {
        eprintln!("Error: File '{}' not found.", path);
        return Ok(());
    }

    let encrypted = fs::read(path)?;
    println!("Analyzing {} ({} bytes)...", path, encrypted.len());

    let mut found_count = 0;

    for shift in 0..=255u8 {
        for sig in SIGNATURES {
            if is_match(&encrypted, shift, sig) {
                found_count += 1;
                let decrypted: Vec<u8> = encrypted.iter().map(|b| b.wrapping_sub(shift)).collect();
                let out_name = format!("output_shift_{:03}.{}", shift, sig.ext);
                
                save_file(&out_name, &decrypted)?;
                println!("MATCH: Shift {:3} | Type: {:5} | File: {} ({})", shift, sig.ext, out_name, sig.note);
            }
        }
    }

    if found_count == 0 {
        println!("No signatures found. Must try manual inspection or checking different offsets.");
    } else {
        println!("\nSuccess: Found {} candidate(s).", found_count);
    }

    Ok(())
}
fn is_match(data: &[u8], shift: u8, sig: &Signature) -> bool {
    let end = sig.offset + sig.magic.len();
    if data.len() < end {
        return false;
    }

    let slice = &data[sig.offset..end];
    for (i, &encrypted_byte) in slice.iter().enumerate() {
        if encrypted_byte.wrapping_sub(shift) != sig.magic[i] {
            return false;
        }
    }
    true
}

fn save_file(path: &str, data: &[u8]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(data)?;
    Ok(())
}
