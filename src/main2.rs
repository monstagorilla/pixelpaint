use std::{io::prelude::*, net::TcpStream};
use rayon::prelude::*;

struct Pixel{
    x: u8,
    y: u8,
    c: String,
}

fn main() -> std::io::Result<()> {
    let width: u8 = 100;
    let height: u8 = 100;
    let color: String = String::from("ff0000");
    let mut pixel_list: Vec<Pixel> = Vec::new();
    let mut stream = 
    for x in 0..width{
        for y in 0..height{

        }
    }
    //let mut pixel_matrix: Vec<Vec<String>> = vec![Vec::new(); width as usize];
    //for i in 0..height{
    //    pixel_matrix.push(vec![color.clone(); height as usize]);
    //}
    let addr: String = String::from("151.217.40.82:1234");
    //let mut stream = TcpStream::connect(addr).unwrap();
    loop{
        pixel_list.par_iter().for_each(|a| {let mut stream = TcpStream::connect(String::from("151.217.40.82:1234")).unwrap(); stream.write(format!("PX {} {} {}\n", a.x, a.y, a.c).as_bytes());println!("PX {} {} {}", a.x, a.y, a.c)});
    }
    Ok(())
}
