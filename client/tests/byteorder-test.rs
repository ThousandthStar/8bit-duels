use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

fn main() {
    let mut wtr = vec![];
    wtr.write_u32::<BigEndian>(568).unwrap();
    println!("{:?}", wtr);
    let mut rdr = Cursor::new(wtr);
    println!("{}", rdr.read_u32::<BigEndian>().unwrap());
}
