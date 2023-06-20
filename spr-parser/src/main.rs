use std::{env, io};
use std::io::{Read, Seek, Write, SeekFrom};
use std::fs::File;
use std::process;
use byteorder::{LittleEndian, ReadBytesExt};

struct SprHeader {
    version: u32,
    count: u16,
}

type SpriteGrid = Vec<RGB>;

#[derive(Debug, Clone, Copy)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
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
    let mut current_sprite: u16 =  0;
    for i in 0..sprite_count {
        current_sprite += 1;
        read_sprite(current_sprite, reader)?;
    }

    Ok(())
}
fn read_sprite<R: Read + Seek>(sprite_id : u16, reader : &mut R) -> io::Result<()> {
    let mut sprite: SpriteGrid = vec![RGB { r: 0, g: 0, b: 0 }; 32 * 32];
    let mut offset : u64 = 6 + ((sprite_id as u64 - 1)  * 4);
    reader.seek(SeekFrom::Start(offset))?;
    let sprite_offset = reader.read_u32::<LittleEndian>()?;
    reader.seek(SeekFrom::Start(sprite_offset as u64))?;
    let dR = reader.read_u8()?;
    let dG = reader.read_u8()?;
    let dB = reader.read_u8()?;
    let sprite_bytes = reader.read_u16::<LittleEndian>()?;

    let mut pixel_ctr = 0;
    let mut i: u32 = 0;
    while i < sprite_bytes as u32{
        let transparent_pixels = reader.read_u16::<LittleEndian>()?;
        let colored_pixels = reader.read_u16::<LittleEndian>()?;

        for _ in 0..transparent_pixels {
            sprite[pixel_ctr] = RGB { r: dR, g: dG, b: dB };
            pixel_ctr += 1;
        }

        for _ in 0..colored_pixels {
            let colorR = reader.read_u8()?;
            let colorG = reader.read_u8()?;
            let colorB = reader.read_u8()?;
            let _colorA = reader.read_u8()?;
            sprite[pixel_ctr] = RGB { r: colorR, g: colorG, b: colorB };
            pixel_ctr += 1;
        }

        i += 4 + (4 * colored_pixels as u32) ;
    }
    let mut sprite_file = File::create(format!("sprites/sprite_{}.bin", sprite_id))?;
    for i in 0..32*32  {
        sprite_file.write_all(&[sprite[i].r, sprite[i].g, sprite[i].b])?;
    }
    Ok(())
}