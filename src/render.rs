use std::{sync::atomic::{AtomicUsize, Ordering}};
use image::{ImageFormat, RgbImage};

static FRAME_NUMBER: AtomicUsize = AtomicUsize::new(0);

#[from_env]
const IMAGE_WIDTH: u32 = 1920;
#[from_env]
const IMAGE_HEIGHT: u32 = 1920;

pub fn render() {
    let frame_num = FRAME_NUMBER.fetch_add(1, Ordering::SeqCst);

    let frame = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let path = {
        let mut p = std::path::PathBuf::new();
        p.push(std::env::current_dir().expect("Cannot get current working directory"));
        p.push("frames");
        p.push(format!("{}.png", frame_num));
        p
    };
    
    frame.save_with_format(path, ImageFormat::Png).expect(&format!("Cannot save image frame at frames/{}.png", frame_num));
}