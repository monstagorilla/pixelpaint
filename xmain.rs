use std::io::prelude::*;
use std::net::TcpStream;
use rayon::prelude::*;

fn main() -> std::io::Result<()> {
    let mut vec: Vec<u16> = Vec::new();
    let width: u16 = 100;
    let height: u16 = 100;
    let num_threads: u16 = 4;
    rayon::ThreadPoolBuilder::new().num_threads(num_threads as usize).build_global().unwrap();
    (0..num_threads).into_par_iter().for_each(|a| draw_line(800+a * (width / num_threads), 800 + ((a+1) * width / num_threads), 800, 800+height)); 
    //let mut stream = TcpStream::connect("151.217.40.82:1234").unwrap();
    //loop{
    //    for x in 800..900{
    //        for y in 800..900{
    //            stream.write(format!("PX {} {} 00ff00\n", x, y).as_bytes());
    //        }
    //    }
    //}
    Ok(())
}

fn draw_line(x0: u16, x1: u16, y0: u16, y1: u16){
    let mut stream = TcpStream::connect("151.217.40.82:1234").unwrap();
    println!("started draw line: x: {}, y0:{}, y1:{}", x0, y0, y1);
    loop{
        for x in x0..x1{
            for y in y0..y1{
                //println!("PX {} {} 00ff00\n", x, y);
                stream.write(format!("PX {} {} 00ff00\n", x, y).as_bytes());
            }
        }
    }
}
