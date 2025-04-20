use dollib::DolHeader;

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

    let header: Vec<u32> = buffer[0..=0xFF]
        .chunks(4)
        .map(|v| u32::from_be_bytes(v[0..4].try_into().unwrap()))
        .collect::<Vec<u32>>();
    /*
    All values in the header are unsigned big-endian 32-bit
    start   end     len
    0x0 	0x3 	4 	File offset to start of Text0
    0x04 	0x1b 	24 	File offsets for Text1..6
    skip 0, take 7

    0x1c 	0x47 	44 	File offsets for Data0..10
    skip 7 take 11

    0x48 	0x4B 	4 	Loading address for Text0
    0x4C 	0x8F 	68 	Loading addresses for Text1..6, Data0..10
    skip 18 take 18

    0x90 	0xD7 	72 	Section sizes for Text0..6, Data0..10
    skip 36 take 18

    0xD8 	0xDB 	4 	BSS address
    skip 54 take 1

    0xDC 	0xDF 	4 	BSS size
    skip 55 take 1

    0xE0 	0xE3 	4 	Entry point
    skip 56 take 1

    0xE4 	0xFF 		padding
    */
    let textoffs: [u32; 7] = header.iter().take(7).map(|e| *e).collect::<Vec<u32>>()[0..7]
        .try_into()
        .unwrap();
    let dataoffs: [u32; 11] = header
        .iter()
        .skip(7)
        .take(11)
        .map(|e| *e)
        .collect::<Vec<u32>>()[0..11]
        .try_into()
        .unwrap();
    let load_addrs: [u32; 18] = header
        .iter()
        .skip(18)
        .take(18)
        .map(|e| *e)
        .collect::<Vec<u32>>()[0..18]
        .try_into()
        .unwrap();

    let sect_sizes: [u32; 18] = header
        .iter()
        .skip(36)
        .take(28)
        .map(|e| *e)
        .collect::<Vec<u32>>()[0..18]
        .try_into()
        .unwrap();

    let bss_addr: u32 = header
        .iter()
        .skip(54)
        .take(1)
        .map(|e| *e)
        .collect::<Vec<u32>>()[0];

    let bss_size: u32 = header
        .iter()
        .skip(55)
        .take(1)
        .map(|e| *e)
        .collect::<Vec<u32>>()[0];

    let entry_point: u32 = header
        .iter()
        .skip(56)
        .take(1)
        .map(|e| *e)
        .collect::<Vec<u32>>()[0];

    let file = DolHeader {
        textoffs,
        dataoffs,
        load_addrs,
        sect_sizes,
        bss_addr,
        bss_size,
        entry_point,
    };

    println!("{file}")
}
