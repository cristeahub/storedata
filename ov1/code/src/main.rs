extern crate time;

use std::old_io::fs;
use std::old_io::{File, IoResult, USER_RWX};

const BLOCKSIZE : usize = 4_096;
static NBLOCKS : usize = 262_144;
//static NBLOCKS : usize = 2;
static NTESTS : usize = 6;

fn write_to_file(f: &mut File,
                 buff: &[u8]) -> IoResult<()>{
    f.write(buff)
}

fn reload_file() -> File {
    fs::unlink(&Path::new("tmp/a.txt")).unwrap_or_else(|why| {
        panic!("Error! {}", why.desc)
    });

    let mut f = match File::create(&Path::new("tmp/a.txt")) {
        Err(why) => panic!("Error! {}", why.desc),
        Ok(file) => file,
    };

    f
}

fn main() {
    println!("Welcome to the beautiful write speed test program");
    println!("This program will test the wiriting speeds of 2^n gb where n = [0..6]");

    let sizes = [1, 2, 4, 8, 16, 32];
    let mut times = [0, 0, 0, 0, 0, 0];
    let buff = [0u8; BLOCKSIZE];

    println!("Make temp dir");
    fs::mkdir(&Path::new("tmp"), USER_RWX).unwrap_or_else(|why| {
        println!("! {:?}", why.kind)
    });

    println!("Create file");
    let mut f = match File::create(&Path::new("tmp/a.txt")) {
        Err(why) => panic!("Error! {}", why.desc),
        Ok(file) => file,
    };

    let start_time = time::precise_time_ns();
    for i in range(0, NTESTS) {
        let elem = sizes[i];
        f = reload_file();
        let t1 = time::precise_time_ns();
        for i in (0..(elem*NBLOCKS)) {
            write_to_file(&mut f, &buff).unwrap_or_else(|why| {
                panic!("Error! {}", why.desc)
            });
        }
        let t2 = time::precise_time_ns();

        times[i] = t2-t1;
    }
    let end_time = time::precise_time_ns();

    println!("Remove written file");
    fs::unlink(&Path::new("tmp/a.txt")).unwrap_or_else(|why| {
        panic!("Error! {}", why.desc)
    });

    println!("Remove temp directory");
    fs::rmdir(&Path::new("tmp")).unwrap_or_else(|why| {
        panic!("Error! {}", why.desc)
    });

    println!("Total time: {}", end_time-start_time);
    for i in range(0, NTESTS) {
        println!("{} | \t time: {}", i, times[i]);
    }

    println!("Done");
}
