use image;
use rqrr;
use std::fs::File;
use std::io::Write;
use opencv::videoio;
use opencv::videoio::prelude::*;
use opencv::core::*;
use opencv::imgcodecs;

fn main(){
    println!("let's make some noise...");
    //todo: automatically detect how many frames need decoding
    let frame_count = 1;
    
    let mut i = 1;
    //initialize file
    let mut base64out = File::create("/home/blues/test64out.txt").unwrap();

    let mut video_file = videoio::VideoCapture::from_file("/home/blues/frames/test.mp4", videoio::CAP_FFMPEG).unwrap();
    
    let mut frame = Mat::default();
    let _frame_read_success = video_file.read(&mut frame).unwrap();
    
    //let fakeImageBuffer : std::vec::Vec<u8> = Vec::new();
    let mut imagebuffer : opencv::core::Vector<u8> = opencv::core::Vector::new();
    let mut image_params : opencv::core::Vector<i32> = opencv::core::Vector::new();
    let _image_encode_success = imgcodecs::imencode("png", &frame, &mut imagebuffer, &mut image_params);

    while i <= frame_count{
        //let directory = "/home/blues/frames/";
        //let file_dir = format!("{}{}{}", directory, i, ".png");
        let buffer = imagebuffer.to_vec();
        let img = image::load_from_memory(&buffer).unwrap().to_luma8();
        // Prepare for detection
        let mut img = rqrr::PreparedImage::prepare(img);
        // Search for grids, without decoding
        let grids = img.detect_grids();

        println!("{}", grids.len());

        assert_eq!(grids.len(), 1);
        // Decode the grid
        let (_meta, content) = grids[0].decode().unwrap();
        //add decoded base64 line to buffer
        write!(&mut base64out, "{}", content);
        println!("decoded frame {} of {}", i, frame_count);
        i += 1;
    }
    //finish the file
    base64out.flush().unwrap();
    
}


