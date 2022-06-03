use zbar_rust::ZBarImageScanner;
use std::fs::File;
use std::io::Write;
use std::env;
use opencv::videoio;
use opencv::videoio::prelude::*;
use opencv::core::*;
use opencv::imgcodecs;

fn main(){
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let output = &args[2];
    println!("let's make some noise...");
    //todo: CLI parameters for input and output files

    //initialize output file
    let mut base64out = File::create(output).unwrap();
    //load the source video
    let mut video_file = videoio::VideoCapture::from_file(input, videoio::CAP_FFMPEG).unwrap();
    
    //grab the first frame
    let mut frame = Mat::default();
    let mut frame_read_success = video_file.read(&mut frame).unwrap();
    let frame_count = video_file.get(videoio::CAP_PROP_FRAME_COUNT).unwrap();
    let mut current_frame = 1;

    while frame_read_success{
        //define width and height for this frame. Some videos change their resolution in the middle which is why I check this every frame instead of just once.
        let width = video_file.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap() as u32;
        let height = video_file.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap() as u32;
        
        //convert frame data to greyscale
        let mut gray = Mat::default();
			opencv::imgproc::cvt_color(
				&frame,
				&mut gray,
				opencv::imgproc::COLOR_BGR2GRAY,
				0,
		).unwrap();

        //make the image buffer
        let mut imagebuffer : opencv::core::Vector<u8> = opencv::core::Vector::new();
        let mut image_params : opencv::core::Vector<i32> = opencv::core::Vector::new();
        let _image_encode_success = imgcodecs::imencode(".pnm", &gray, &mut imagebuffer, &mut image_params);
        //convert opencv buffer type to standard vec
        let buffer = imagebuffer.to_vec();
        //start scanning
        let mut scanner = ZBarImageScanner::new();
        let results = scanner.scan_y800(&buffer, width, height).unwrap();
        //write results
        for result in results {
            let content = String::from_utf8(result.data).unwrap();
            println!("decoded frame {} of {}", current_frame,frame_count);
            write!(&mut base64out, "{}", content);
        }
        current_frame += 1;
        //grab the next frame, or exit loop
        frame = Mat::default();
        frame_read_success = video_file.read(&mut frame).unwrap();
    }
    //finish the file
    base64out.flush().unwrap();
}