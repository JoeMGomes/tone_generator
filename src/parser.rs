use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use std::str;

pub fn parse_file() -> io::Result<()> {
    let f = File::open("input.tone")?;
    let mut f = BufReader::new(f);

    let mut split_iter = f.split(b']').map(|l| l.unwrap());
    
    let x = &split_iter.next().unwrap();

    let s = match str::from_utf8(x) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("READ {:?}", s);
    println!("READ {:?}", split_iter.next().unwrap());

    // let mut buf = vec![];

    // //clean garbage
    // match f.read_until(b'[', &mut buf){
    //     Ok(u8 )=> (),
    //     Err(e)=> ()
    // }
    // buf.drain(..buf.len()-1);

    // parse_next(&mut f);
    // parse_next(&mut f);

    Ok(())
}

pub fn parse_next(cursor: &mut std::io::BufReader<std::fs::File>) -> Vec<u8> {
    let mut buf = vec![];
    match cursor.read_until(b']', &mut buf) {
        Ok(u8) => (),
        Err(e) => (),
    }
    let s = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("READ: {:?}", s);
    return buf;
}
