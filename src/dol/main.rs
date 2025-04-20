use std::{
    collections::VecDeque,
    fs::{self, File},
    io::Read,
};

fn main() {
    let filename = "./tests/zayd/broadway/dols/addx.dol";
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    let header: Vec<u32> = buffer[0..0xFF]
        .chunks(4)
        .map(|e| u32::from_be_bytes(e[0..4].try_into().unwrap()))
        .collect();
    /*
    All values in the header are unsigned big-endian 32-bit
    start   end     len
    0x0 	0x3 	4 	File offset to start of Text0
    0x04 	0x1b 	24 	File offsets for Text1..6
    0x1c 	0x47 	44 	File offsets for Data0..10
    0x48 	0x4B 	4 	Loading address for Text0
    0x4C 	0x8F 	68 	Loading addresses for Text1..6, Data0..10
    0x90 	0xD7 	72 	Section sizes for Text0..6, Data0..10
    0xD8 	0xDB 	4 	BSS address
    0xDC 	0xDF 	4 	BSS size
    0xE0 	0xE3 	4 	Entry point
    0xE4 	0xFF 		padding
    */
    let mut textoffs: [&u32; 7] = header.iter().take(7).collect::<Vec<&u32>>()[0..7]
        .try_into()
        .unwrap();
    let mut dataoffs: [&u32; 11] = header.iter().skip(7).take(11).collect::<Vec<&u32>>()[0..11]
        .try_into()
        .unwrap();
}
