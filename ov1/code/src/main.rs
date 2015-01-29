use std::io::fs;
use std::io::{File, IoResult, USER_RWX};

const BLOCKSIZE : usize = 8_192;
static NBLOCKS_BIG : usize = 131_072;
static NBLOCKS : usize = 3;

fn write_to_file(f: &mut File,
                 buff: &[u8]) -> IoResult<()>{
    println!("Write to file");
    f.write(buff)
}

fn main() {
    println!("Welcome to the beautiful write speed test program");
    println!("This program will test the wiriting speeds of 2^n gb where n = [0..5]");

    let sizes = [1, 1];
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

    sizes.iter().map(|elem| {
        for i in (0..(*elem)*NBLOCKS) {
            write_to_file(&mut f, &buff).unwrap_or_else(|why| {
                panic!("Error! {}", why.desc)
            });
        }
    }).count(); // the iterator is so lazy that .count() is used to invoke it

    println!("Remove written file");
    fs::unlink(&Path::new("tmp/a.txt")).unwrap_or_else(|why| {
        panic!("Error! {}", why.desc)
    });

    println!("Remove temp directory");
    fs::rmdir(&Path::new("tmp")).unwrap_or_else(|why| {
        panic!("Error! {}", why.desc)
    });

    println!("Done");
}
