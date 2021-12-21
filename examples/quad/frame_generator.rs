/// generate a frame with YUV planes(YUV420 format)
/// Y size is width x height
/// U and V sizes are width x height / 2
/// the output buffer will contains samples in this order
/// YYYYUUVV
/// 
/// The index'th sample will be set to 235 (a white point)
pub fn generate_frame(width: u32, height: u32, index: usize) -> Vec<u8> {
    let frame_size = (width as u32 * height as u32) as usize;
    let mut yuv = vec![127 as u8; (width * height * 3 / 2) as usize];

    // fill y plane as black
    for y_index in 0..frame_size {
        // YUV(16,127,127) is black
        yuv[y_index as usize] = 16
    }
    // set white color
    // YUV(235,127,127) is white
    for i in 0..10 {
        for j in 0..10 {
            let mut r = (i * width as usize) + (index * 10 + j);
            r = r % (width * height) as usize;
            yuv[r] = 235;
        }
    }
    // return
    yuv
}
