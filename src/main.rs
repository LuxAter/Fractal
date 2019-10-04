extern crate clap;
extern crate image;
extern crate rayon;
use rayon::prelude::*;

fn main() {
    let fractal_types = ["mandelbrot", "julia"];
    let extension_types = ["png", "jpeg", "bmp", "tiff"];
    let matches = clap::App::new("fractal")
        .version("1.0")
        .author("Arden Rasmussen")
        .about("Renders fractals")
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .default_value("output")
                .help("Output files/directory for rendering"),
        )
        .arg(
            clap::Arg::with_name("extension")
                .short("e")
                .default_value("png")
                .possible_values(&extension_types)
                .help("Image file format to use"),
        )
        .arg(
            clap::Arg::with_name("xres")
                .short("x")
                .default_value("1000")
                .help("X resolution of output images"),
        )
        .arg(
            clap::Arg::with_name("yres")
                .short("y")
                .default_value("1000")
                .help("Y resolution of output images"),
        )
        .arg(
            clap::Arg::with_name("fractal")
                .required(true)
                .possible_values(&fractal_types)
                .help("Type of fractal to render"),
        )
        .arg(
            clap::Arg::with_name("bx")
                .long("bx")
                .default_value("-0.4")
                .help("X resolution of output images"),
        )
        .arg(
            clap::Arg::with_name("by")
                .long("by")
                .default_value("0.6")
                .help("Y resolution of output images"),
        )
        .get_matches();
    let resx = matches.value_of("xres").unwrap().parse::<u32>().unwrap();
    let resy = matches.value_of("yres").unwrap().parse::<u32>().unwrap();
    let bx = matches.value_of("bx").unwrap().parse::<f32>().unwrap();
    let by = matches.value_of("by").unwrap().parse::<f32>().unwrap();
    let scalex = 3.0 / resx as f32;
    let scaley = 3.0 / resy as f32;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    pool.install(move || {
        let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<_>> =
            image::ImageBuffer::new(resx, resy);
        let pixel = *imgbuf.get_pixel(1, 1);
        imgbuf.put_pixel(1, 1, pixel);
        let mut buffer = imgbuf.into_vec();
        println!("Generating Plot");
        buffer
            .par_chunks_mut(resx as usize * 3usize)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..resx {
                    let cx = y as f32 * scalex - 1.5;
                    let cy = x as f32 * scaley - 1.5;
                    let c = num::complex::Complex::new(bx, by);
                    let mut z = num::complex::Complex::new(cx, cy);

                    let mut i: u32 = 0;
                    while i < 100000 && z.norm() <= 2.0 {
                        z = z * z + c;
                        i += 1;
                    }

                    i = i % 255;
                    row[(x * 3) as usize] = i as u8;
                    row[(x * 3 + 1) as usize] = i as u8;
                    row[(x * 3 + 2) as usize] = i as u8;
                }
            });
        println!("Saving Image");
        let imgbuf: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<_>> =
            image::ImageBuffer::from_vec(resx, resy, buffer).unwrap();
        imgbuf
            .save(format!(
                "{}/{}.{}.{}",
                matches.value_of("output").unwrap(),
                bx,
                by,
                matches.value_of("extension").unwrap()
            ))
            .unwrap();
    });
}
