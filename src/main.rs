extern crate image;

use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::fs::File;
use std::path::Path;

use image::SubImage;
use image::GenericImageView;
use image::Pixel;

enum ImageMode {
    Full,
    Partial(usize, usize, usize, usize)
}

// TODO use std::path::Path
fn draw_image(img_path: String, mode: Option<ImageMode>) {
    println!("{:?}: started", thread::current().id());
    let mut stream = get_stream();
    println!("{:?}: connected", thread::current().id());

    let mut buf = Vec::new();
    let im = image::open(&Path::new(&img_path)).unwrap();
    for (x, y, pixel) in im.pixels() {
        write!(buf, "PX {} {} {:x?}{:x?}{:x?}\n", x, y, pixel.data[0], pixel.data[1], pixel.data[2]).unwrap();
    }
    println!("{:?}: painting", thread::current().id());
    loop {
        stream.write_all(&*buf).unwrap_or_else(|_| {
            println!("{:?}: broken, reconnecting", thread::current().id());
            stream = get_stream();
        });
    }
}

fn main() -> std::io::Result<()> {
/*     let width: u16 = 300;
    let height: u16 = 600;
    let num_threads: u16 = 20;
    (0..num_threads).for_each(|a| {
        thread::spawn(move || {
            draw_line(
                1300 + a * (width / num_threads),
                1300 + ((a + 1) * width / num_threads),
                600,
                600 + height,
            )
        });
    });
    std::thread::sleep(std::time::Duration::from_secs(10000000000));
    Ok(()) */
    draw_image("./gnuebermensch.png".to_string(), None);
    Ok(())
}

fn get_stream() -> TcpStream {
    loop {
        // 151.217.40.82:1234
        if let Ok(stream) = TcpStream::connect("151.217.177.136:1234") {
            break stream;
        }
        println!("{:?}: retrying", thread::current().id());
    }
}

fn draw_line(x0: u16, x1: u16, y0: u16, y1: u16) {
    println!("{:?}: started", thread::current().id());
    let mut stream = get_stream();
    println!("{:?}: connected", thread::current().id());

    let mut buf = Vec::new();
    for x in x0..x1 {
        for y in y0..y1 {
            write!(buf, "PX {} {} 000000\n", x, y).unwrap();
        }
    }

    println!("{:?}: painting", thread::current().id());
    loop {
        stream.write_all(&*buf).unwrap_or_else(|_| {
            println!("{:?}: broken, reconnecting", thread::current().id());
            stream = get_stream();
        });
    }
}