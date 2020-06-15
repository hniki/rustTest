use std::io::{Cursor, Seek, SeekFrom};
use byteorder::{NariveEndian, BigEndian, LittleEndian, ReadByTesExt, ByteOrder};
use bitreader::{BitReader, ReadInto};
use std::io;
use std::error::Error;


pub type ByteCursor = Cursor<Vec<u8>>;

pub fn read_u8(cursor: &mut ByteCursor) -> io::Result<u8>{
    cursor.read_u8()
}

pub fn read_u16(cursor: &mut ByteCursor) -> io::Result<u16>{
    cursor.read_u16::<BigEndian>()
}