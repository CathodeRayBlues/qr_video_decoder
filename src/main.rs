use image;
use zbar_rust::ZBarImageScanner;
use std::fs::File;
use std::io::Write;
use opencv::videoio;
use opencv::videoio::prelude::*;
use opencv::core::*;
use opencv::imgcodecs;

fn main(){
    println!("let's make some noise...");
    //todo: CLI parameters for input and output files

    //initialize output file
    let mut base64out = File::create("/home/blues/test64out.txt").unwrap();
    //load the source video
    let mut video_file = videoio::VideoCapture::from_file("/home/blues/frames/test.mp4", videoio::CAP_FFMPEG).unwrap();
    
    //grab the first frame
    let mut frame = Mat::default();
    let mut frame_read_success = video_file.read(&mut frame).unwrap();
    
    while frame_read_success{
        //make the image buffer
        let mut imagebuffer : opencv::core::Vector<u8> = opencv::core::Vector::new();
        let mut image_params : opencv::core::Vector<i32> = opencv::core::Vector::new();
        let _image_encode_success = imgcodecs::imencode(".bmp", &frame, &mut imagebuffer, &mut image_params);
        //convert opencv buffer type to standard vec
        let buffer = imagebuffer.to_vec();
        // get the image library to load the buffer and convert it to the format needed for QR grid detection
        let img = image::load_from_memory_with_format(&buffer, image::ImageFormat::Bmp).unwrap().to_luma8();
        let (width, height) = img.dimensions();

        let luma_img = img;

        let luma_img_data: Vec<u8> = luma_img.to_vec();

        let mut scanner = ZBarImageScanner::new();

        let results = scanner.scan_y800(&luma_img_data, width, height).unwrap();

        for result in results {
            let content = String::from_utf8(result.data).unwrap();
            print!("{}", content);
            write!(&mut base64out, "{}", content);
        }
        //grab the next frame, or exit loop
        frame = Mat::default();
        frame_read_success = video_file.read(&mut frame).unwrap();
    }
    //finish the file
    base64out.flush().unwrap();
    
}