use std::{env, io};
use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use std::process;
use byteorder::{LittleEndian, ReadBytesExt};

struct SprHeader {
    version: u32,
    count: u16,
}

fn main() -> io::Result<()> {

    let filename: String = match parse_command_line() {
        Ok(filename) => filename,
        Err(e) => {
            println!("Error parsing command line: {}", e);
            process::exit(1);
        },
    };

    let mut file: File = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            process::exit(1);
        },
    };
    println!("File opened successfully");

    let header: SprHeader = match read_header(&mut file) {
        Ok(header) => header,
        Err(e) => {
            println!("Error reading header: {}", e);
            process::exit(1);
        },
    };

    println!("Version: {}, count {}", header.version, header.count);

    read_all_sprites(header.count, &mut file)?;

    Ok(())
}


fn parse_command_line() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 { 
        Err("Please provide spr file path via  /path/to/file.spr")
    } else
    {
        Ok(args[1].clone())
    }
}

fn read_header<R: Read + Seek>(reader: &mut R) -> io::Result<SprHeader> {
    let mut buffer = [0; 6];
    reader.seek(SeekFrom::Start(0))?;
    reader.read_exact(&mut buffer)?;

    let version = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
    let count = u16::from_le_bytes([buffer[4], buffer[5]]);
    Ok (SprHeader { version, count })
}

fn read_all_sprites<R: Read + Seek>(sprite_count : u16, reader: &mut R) -> io::Result<()> {
    let mut current_sprite: u16 =  1;
    for i in 0..sprite_count {
        current_sprite += 1;
        read_sprite(current_sprite, reader)?;
        if i == 5 {
            break;
        }
    }

    Ok(())
}
fn read_sprite<R: Read + Seek>(sprite_id : u16, reader : &mut R) -> io::Result<()> {
    let mut offset : u64 = 6 + ((sprite_id as u64 - 1)  * 4);
    reader.seek(SeekFrom::Start(offset))?;
    let sprite_offset = reader.read_u32::<LittleEndian>()?;
    reader.seek(SeekFrom::Start(sprite_offset as u64))?;
    let sprite_size = reader.read_u16::<LittleEndian>()?;
    let transparent_pixels = reader.read_u16::<LittleEndian>()?;
    let colored_pixels = reader.read_u16::<LittleEndian>()?;
    
    println!("Reading sprite {}, sprite size {}, offset {}, trans pix: {}, color pix {}", sprite_id, sprite_size, offset, transparent_pixels, colored_pixels);
    Ok(())
}