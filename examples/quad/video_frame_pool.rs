use ringbuf::{
    RingBuffer,
    Consumer,
    Producer
};
use std::{
    thread,
    time::Duration,
    io::Result
};
///
/// VideoFramesBuffer
/// 
/// 
pub struct VideoFramesBuffer {
    cons: Consumer<u8>,
    prod: Producer<u8>
}

impl VideoFramesBuffer {
    pub fn new(pool_size: u32) -> VideoFramesBuffer {
        let buf = RingBuffer::<u8>::new(pool_size as usize);
        let (prod, cons) = buf.split();

        VideoFramesBuffer {
            prod,
            cons
        }
    }

    pub fn is_full(&mut self) -> bool {
        self.prod.is_full()
    }

    pub fn push(&mut self, frame: &mut VideoFrame) -> Result<usize> {
        let zero = [0xb9, 0xd9];
        let mut bytes = frame.to_bytes();
        bytes.append(&mut zero.to_vec());

        loop {
            if self.prod.is_full() {
                thread::sleep(Duration::from_millis(1));
            } else {
                let n = self.prod.read_from(&mut bytes.as_slice(), None).unwrap();
                if n == 0 {
                    break;
                }
            }
        }

        Result::Ok(0)
    }

    pub fn is_empty(&mut self) -> bool {
        self.cons.is_empty()
    }

    pub fn pop(&mut self, frame: &mut VideoFrame) -> Result<usize> {
        let zero = [0xb9, 0xd9];
        let mut bytes = Vec::<u8>::new();

        loop {
            if self.cons.is_empty() {
                if bytes.ends_with(&zero[..]) {
                    break;
                } else {
                    thread::sleep(Duration::from_millis(1));
                }
            } else {
                let n = self.cons.write_into(&mut bytes, None).unwrap();
            }
        }

        *frame = VideoFrame::from_bytes(bytes);

        Result::Ok(0)
    }
}
///
/// YUV420 frame buffer structure
/// 
pub struct VideoFrame {
    pub width: u16,
    pub height: u16,
    pub index: u32,

    buffer: Vec<u8>,
}

impl VideoFrame {
    pub fn new() -> VideoFrame {
        VideoFrame {
            width: 0,
            height: 0,
            index: 0,
            buffer: Vec::<u8>::new()
        }
    }
    pub fn from_data(width: u16, height: u16, index: u32, yuv: Vec<u8>) -> VideoFrame {
        VideoFrame {
            width: width,
            height: height,
            index: index,
            buffer: yuv
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.width.to_be_bytes().to_vec());
        bytes.append(&mut self.height.to_be_bytes().to_vec());
        bytes.append(&mut self.index.to_be_bytes().to_vec());
        bytes.append(&mut self.buffer.to_vec());
        bytes
    }

    pub fn from_bytes(bytes: Vec<u8>) -> VideoFrame {
        let width = u16::from_be_bytes([bytes[0], bytes[1]]);
        let height = u16::from_be_bytes([bytes[2], bytes[3]]);
        let index = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let buffer = bytes[8..].to_vec();

        VideoFrame {
            width: width,
            height: height,
            index: index,
            buffer: buffer
        }
    }
}

fn size_of_video_frame(width: u16, height: u16) -> usize {
    ((width * height * 3 / 2) + 8).into()
}

// fn run() {
//     let buf_size = 1024 * 768 * 24 * 60;
//     let buf = RingBuffer::<u8>::new(buf_size);
//     let (mut prod, mut cons) = buf.split();

//     let smsg = "The quick brown fox jumps over the lazy dog";

//     let pjh = thread::spawn(move || {
//         println!("-> sending message: '{}'", smsg);

//         let zero = [0xb9, 0xd9];
//         let mut bytes = smsg.as_bytes().chain(&zero[..]);
//         loop {
//             if prod.is_full() {
//                 println!("-> buffer is full, waiting");
//                 thread::sleep(Duration::from_millis(1));
//             } else {
//                 let n = prod.read_from(&mut bytes, None).unwrap();
//                 if n == 0 {
//                     break;
//                 }
//                 println!("-> {} bytes sent", n);
//             }
//         }

//         println!("-> message sent");
//     });

//     let cjh = thread::spawn(move || {
//         println!("<- receiving message");

//         let mut bytes = Vec::<u8>::new();
//         loop {
//             if cons.is_empty() {
//                 if bytes.ends_with(&[0]) {
//                     break;
//                 } else {
//                     println!("<- buffer is empty, waiting");
//                     thread::sleep(Duration::from_millis(1));
//                 }
//             } else {
//                 let n = cons.write_into(&mut bytes, None).unwrap();
//                 println!("<- {} bytes received", n);
//             }
//         }

//         assert_eq!(bytes.pop().unwrap(), 0);
//         let msg = String::from_utf8(bytes).unwrap();
//         println!("<- message received: '{}'", msg);

//         msg
//     });

//     pjh.join().unwrap();
//     let rmsg = cjh.join().unwrap();
// }
