use std::io::fs;
use std::io::{File, IoResult, USER_RWX};

fn write_to_file(f: &mut File,
                 buff: [u8; 64 * 1024],
                 size: isize) -> IoResult<()>{
    f.write(&buff)
}

fn main() {
    println!("Welcome to the beautiful write speed test program");
    println!("This program will test the wiriting speeds of 2^n gb where n = [0..5]");

    let sizes = [1, 2, 4, 8, 16, 32];
    let buff = [0u8; 64 * 1024];

    println!("Make temp dir");
    fs::mkdir(&Path::new("tmp"), USER_RWX).unwrap_or_else(|why| {
        println!("! {:?}", why.kind)
    });

    println!("Create file");
    let mut f = match File::create(&Path::new("tmp/a.txt")) {
        Err(why) => panic!("Error! {}", why.desc),
        Ok(file) => file,
    };

    println!("Write to file");
    write_to_file(&mut f, buff, sizes[0]).unwrap_or_else(|why| {
        panic!("Error! {}", why.desc)
    });

    println!("Done");
}
