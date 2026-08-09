// convert.rs
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::sync::mpsc;
use image::{ImageFormat, DynamicImage, imageops::FilterType};
use glob::glob;

fn convert_png_to_jpg(input_path: &Path, output_path: &Path, quality: u8, resize: Option<(u32, u32)>) -> Result<(), String> {
    let img = image::open(input_path).map_err(|e| e.to_string())?;
    let mut img = img.to_rgb8(); // JPG не поддерживает альфа-канал
    let dyn_img = DynamicImage::ImageRgb8(img);
    let mut img = if let Some((w, h)) = resize {
        dyn_img.resize(w, h, FilterType::Lanczos3)
    } else {
        dyn_img
    };
    // Сохранение
    let mut out = fs::File::create(output_path).map_err(|e| e.to_string())?;
    let mut encoder = image::jpeg::JpegEncoder::new_with_quality(&mut out, quality);
    encoder.encode_image(&img).map_err(|e| e.to_string())?;
    Ok(())
}

fn process_files(inputs: Vec<String>, quality: u8, resize: Option<(u32, u32)>, output_dir: &str, overwrite: bool, recursive: bool) {
    let mut files: Vec<PathBuf> = Vec::new();
    for item in inputs {
        let path = Path::new(&item);
        if path.is_file() && path.extension().map(|e| e == "png").unwrap_or(false) {
            files.push(path.to_path_buf());
        } else if path.is_dir() {
            if recursive {
                for entry in walkdir::WalkDir::new(path) {
                    let entry = entry.unwrap();
                    if entry.path().is_file() && entry.path().extension().map(|e| e == "png").unwrap_or(false) {
                        files.push(entry.path().to_path_buf());
                    }
                }
            } else {
                for entry in fs::read_dir(path).unwrap() {
                    let entry = entry.unwrap();
                    let p = entry.path();
                    if p.is_file() && p.extension().map(|e| e == "png").unwrap_or(false) {
                        files.push(p);
                    }
                }
            }
        } else if item.contains('*') {
            for entry in glob(&item).unwrap() {
                if let Ok(p) = entry {
                    if p.extension().map(|e| e == "png").unwrap_or(false) {
                        files.push(p);
                    }
                }
            }
        }
    }
    if files.is_empty() {
        println!("Не найдено PNG-файлов.");
        return;
    }
    fs::create_dir_all(output_dir).unwrap();
    let total = files.len();
    println!("Найдено {} PNG-файлов.", total);
    let (tx, rx) = mpsc::channel();
    let out_dir = output_dir.to_string();
    let threads = 4;
    let mut handles = vec![];
    for chunk in files.chunks((total + threads - 1) / threads) {
        let chunk = chunk.to_vec();
        let tx = tx.clone();
        let quality = quality;
        let resize = resize.clone();
        let out_dir = out_dir.clone();
        handles.push(thread::spawn(move || {
            for (i, input_path) in chunk.iter().enumerate() {
                let out_name = input_path.file_stem().unwrap().to_str().unwrap().to_string() + ".jpg";
                let out_path = Path::new(&out_dir).join(out_name);
                if out_path.exists() && !overwrite {
                    tx.send((i+1, format!("{} уже существует, пропуск.", out_path.display()))).unwrap();
                    continue;
                }
                tx.send((i+1, format!("Конвертация {} -> {}", input_path.display(), out_path.display()))).unwrap();
                if let Err(e) = convert_png_to_jpg(input_path, &out_path, quality, resize) {
                    tx.send((i+1, format!("Ошибка: {}", e))).unwrap();
                }
            }
        }));
    }
    drop(tx);
    let mut count = 0;
    for (idx, msg) in rx {
        count += 1;
        println!("[{}/{}] {}", count, total, msg);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Готово!");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Использование: {} <PNG-файлы/папки> [--quality N] [--resize ШxВ] [--output DIR] [--overwrite] [--recursive]", args[0]);
        std::process::exit(1);
    }
    let mut quality = 85;
    let mut resize = None;
    let mut output_dir = ".".to_string();
    let mut overwrite = false;
    let mut recursive = false;
    let mut inputs = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--quality" => {
                if i+1 < args.len() {
                    quality = args[i+1].parse().unwrap_or(85);
                    i += 2;
                } else { i += 1; }
            }
            "--resize" => {
                if i+1 < args.len() {
                    let s = &args[i+1];
                    let parts: Vec<&str> = s.split('x').collect();
                    if parts.len() == 2 {
                        let w = parts[0].parse().unwrap_or(0);
                        let h = parts[1].parse().unwrap_or(0);
                        if w > 0 && h > 0 {
                            resize = Some((w, h));
                        }
                    }
                    i += 2;
                } else { i += 1; }
            }
            "--output" => {
                if i+1 < args.len() {
                    output_dir = args[i+1].clone();
                    i += 2;
                } else { i += 1; }
            }
            "--overwrite" => {
                overwrite = true;
                i += 1;
            }
            "--recursive" => {
                recursive = true;
                i += 1;
            }
            _ => {
                inputs.push(args[i].clone());
                i += 1;
            }
        }
    }
    process_files(inputs, quality, resize, &output_dir, overwrite, recursive);
}
