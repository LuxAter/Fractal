extern crate clap;
extern crate image;
extern crate rayon;
use rayon::prelude::*;

mod cmap;

fn mandelbrot(
    res: std::vec::Vec<u32>,
    center: num::complex::Complex64,
    domain: f64,
    cmap: std::vec::Vec<u32>,
    file: std::string::String,
) {
    let imgbuf: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<_>> =
        image::ImageBuffer::new(res[0], res[1]);
    let mut buffer = imgbuf.into_vec();
    let scale = domain / res[0].min(res[1]) as f64;
    let local_center = if res[0] > res[1] {
        num::complex::Complex64::new(
            center.re - ((res[0] as f64) * domain / ((res[1] as f64) * 2.0)),
            -center.im - domain / 2.0,
        )
    } else if res[0] < res[1] {
        num::complex::Complex64::new(
            center.re - domain / 2.0,
            -center.im - ((res[1] as f64) * domain / ((res[0] as f64) * 2.0)),
        )
    } else {
        num::complex::Complex64::new(center.re - domain / 2.0, -center.im - domain / 2.0)
    };
    let max_iter = ((res[0].max(res[1]) as f64 * 1.0) / domain).max(1000.0) as u64;
    println!("Max Iter: {}", max_iter);
    println!("LOCAL CENTER: {}", local_center);
    buffer
        .par_chunks_mut(res[0] as usize * 3usize)
        .enumerate()
        .for_each(|(y, row)| {
            for x in 0..res[0] {
                let mut z = num::complex::Complex64::new(0.0, 0.0);
                let c = (scale * num::complex::Complex64::new(x as f64, y as f64)) + local_center;
                let mut i: u64 = 0;
                while i < max_iter && z.norm() <= 2.0 {
                    z = z * z + c;
                    i += 1;
                }
                if z.norm() >= 2.0 {
                    i = i % 255;
                    let color = cmap[i as usize];
                    row[(x * 3) as usize] = ((color >> 16) & 0xff) as u8;
                    row[(x * 3 + 1) as usize] = ((color >> 8) & 0xff) as u8;
                    row[(x * 3 + 2) as usize] = (color & 0xff) as u8;
                }
            }
        });
    let imgbuf: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<_>> =
        image::ImageBuffer::from_vec(res[0], res[1], buffer).unwrap();
    println!("Saving image to {}", file);
    imgbuf.save(file).unwrap();
}

fn julia() {
    println!("Generating Mandelbrot");
}

fn main() {
    let matches = clap::App::new("fractal")
        .version("1.0")
        .author("Arden Rasmussen")
        .about("Renders fractals")
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .long("out")
                .default_value("output")
                .help("Output files/directory for rendering"),
        )
        .arg(
            clap::Arg::with_name("extension")
                .short("e")
                .long("ext")
                .default_value("png")
                .possible_values(&["png", "jpeg", "bmp", "tiff"])
                .help("Image file format to use"),
        )
        .arg(
            clap::Arg::with_name("xres")
                .short("x")
                .long("xres")
                .default_value("1920")
                .help("X resolution of output images"),
        )
        .arg(
            clap::Arg::with_name("yres")
                .short("y")
                .long("yres")
                .default_value("1080")
                .help("Y resolution of output images"),
        )
        .arg(
            clap::Arg::with_name("cmap")
                .long("cmap")
                .default_value("parula")
                .possible_values(&[
                    "yellow_blue",
                    "parula",
                    "parula_r",
                    "Blues",
                    "Blues_r",
                    "BrBG",
                    "BrBG_r",
                    "BuGn",
                    "BuGn_r",
                    "BuPu",
                    "BuPu_r",
                    "CMRmap",
                    "CMRmap_r",
                    "Dark2",
                    "Dark2_r",
                    "GnBu",
                    "GnBu_r",
                    "Greens",
                    "Greens_r",
                    "Greys",
                    "Greys_r",
                    "OrRd",
                    "OrRd_r",
                    "Oranges",
                    "Oranges_r",
                    "PRGn",
                    "PRGn_r",
                    "Paired",
                    "Paired_r",
                    "Pastel1",
                    "Pastel1_r",
                    "Pastel2",
                    "Pastel2_r",
                    "PiYG",
                    "PiYG_r",
                    "PuBu",
                    "PuBuGn",
                    "PuBuGn_r",
                    "PuBu_r",
                    "PuOr",
                    "PuOr_r",
                    "PuRd",
                    "PuRd_r",
                    "Purples",
                    "Purples_r",
                    "RdBu",
                    "RdBu_r",
                    "RdGy",
                    "RdGy_r",
                    "RdPu",
                    "RdPu_r",
                    "RdYlBu",
                    "RdYlBu_r",
                    "RdYlGn",
                    "RdYlGn_r",
                    "Reds",
                    "Reds_r",
                    "Set1",
                    "Set1_r",
                    "Set2",
                    "Set2_r",
                    "Set3",
                    "Set3_r",
                    "Spectral",
                    "Spectral_r",
                    "Wistia",
                    "Wistia_r",
                    "YlGn",
                    "YlGnBu",
                    "YlGnBu_r",
                    "YlGn_r",
                    "YlOrBr",
                    "YlOrBr_r",
                    "YlOrRd",
                    "YlOrRd_r",
                    "afmhot",
                    "afmhot_r",
                    "autumn",
                    "autumn_r",
                    "binary",
                    "binary_r",
                    "bone",
                    "bone_r",
                    "brg",
                    "brg_r",
                    "bwr",
                    "bwr_r",
                    "cividis",
                    "cividis_r",
                    "cool",
                    "cool_r",
                    "coolwarm",
                    "coolwarm_r",
                    "copper",
                    "copper_r",
                    "cubehelix",
                    "cubehelix_r",
                    "flag",
                    "flag_r",
                    "gist_earth",
                    "gist_earth_r",
                    "gist_gray",
                    "gist_gray_r",
                    "gist_heat",
                    "gist_heat_r",
                    "gist_ncar",
                    "gist_ncar_r",
                    "gist_rainbow",
                    "gist_rainbow_r",
                    "gist_stern",
                    "gist_stern_r",
                    "gist_yarg",
                    "gist_yarg_r",
                    "gnuplot",
                    "gnuplot2",
                    "gnuplot2_r",
                    "gnuplot_r",
                    "gray",
                    "gray_r",
                    "hot",
                    "hot_r",
                    "hsv",
                    "hsv_r",
                    "inferno",
                    "inferno_r",
                    "jet",
                    "jet_r",
                    "magma",
                    "magma_r",
                    "nipy_spectral",
                    "nipy_spectral_r",
                    "ocean",
                    "ocean_r",
                    "pink",
                    "pink_r",
                    "plasma",
                    "plasma_r",
                    "prism",
                    "prism_r",
                    "rainbow",
                    "rainbow_r",
                    "seismic",
                    "seismic_r",
                    "spring",
                    "spring_r",
                    "summer",
                    "summer_r",
                    "tab10",
                    "tab10_r",
                    "tab20",
                    "tab20_r",
                    "tab20b",
                    "tab20b_r",
                    "tab20c",
                    "tab20c_r",
                    "terrain",
                    "terrain_r",
                    "twilight",
                    "twilight_r",
                    "twilight_shifted",
                    "twilight_shifted_r",
                    "viridis",
                    "viridis_r",
                    "winter",
                    "winter_r",
                ])
                .help("Color map to use for rendering"),
        )
        .arg(
            clap::Arg::with_name("fractal")
                .required(true)
                .possible_values(&["mandelbrot", "julia"])
                .help("Type of fractal to render"),
        )
        .arg(
            clap::Arg::with_name("domain")
                .short("d")
                .long("domain")
                .default_value("3")
                .allow_hyphen_values(true)
                .help("Domain/domain range for the render or zoom"),
        )
        .arg(
            clap::Arg::with_name("frames")
                .short("f")
                .long("frames")
                .default_value("100")
                .help("Number of frames for render for any animation"),
        )
        .arg(
            clap::Arg::with_name("center")
                .short("c")
                .long("center")
                .default_value("0,0")
                .allow_hyphen_values(true)
                .help("Center of image, or range for linear animation"),
        )
        .get_matches();
    let resx = matches.value_of("xres").unwrap().parse::<u32>().unwrap();
    let resy = matches.value_of("yres").unwrap().parse::<u32>().unwrap();
    let mut animation = false;
    let mut domain: std::vec::Vec<f64> = vec![];
    let mut center: std::vec::Vec<num::complex::Complex64> = vec![];
    let domain_str = matches.value_of("domain").unwrap();
    if domain_str.contains(":") {
        let domain_range = domain_str.split(":").collect::<std::vec::Vec<&str>>();
        domain.push(domain_range[0].parse::<f64>().unwrap());
        domain.push(domain_range[1].parse::<f64>().unwrap());
        animation = true;
    } else {
        domain.push(domain_str.parse::<f64>().unwrap());
    }
    let center_str = matches.value_of("center").unwrap();
    if center_str.contains(":") {
        let center_range = center_str.split(":").collect::<std::vec::Vec<&str>>();
        let center_start_str = center_range[0].split(",").collect::<std::vec::Vec<&str>>();
        let center_stop_str = center_range[1].split(",").collect::<std::vec::Vec<&str>>();
        center.push(num::complex::Complex64::new(
            center_start_str[0].parse::<f64>().unwrap(),
            center_start_str[1].parse::<f64>().unwrap(),
        ));
        center.push(num::complex::Complex64::new(
            center_stop_str[0].parse::<f64>().unwrap(),
            center_stop_str[1].parse::<f64>().unwrap(),
        ));
        animation = true;
    } else {
        let center_str = center_str.split(",").collect::<std::vec::Vec<&str>>();
        center.push(num::complex::Complex64::new(
            center_str[0].parse::<f64>().unwrap(),
            center_str[1].parse::<f64>().unwrap(),
        ));
    }
    let frames = matches.value_of("frames").unwrap().parse::<u32>().unwrap();
    let cmap = cmap::construct_cmaps(matches.value_of("cmap").unwrap());
    if animation {
        println!("Rendering Animation [{}]", frames);
    } else {
        println!("Rendering Image");
        println!("Centered at: {}, with diamiter of {}", center[0], domain[0]);
        let file = format!(
            "{}.{}",
            matches.value_of("output").unwrap(),
            matches.value_of("extension").unwrap()
        );
        match matches.value_of("fractal").unwrap() {
            "mandelbrot" => mandelbrot(vec![resx, resy], center[0], domain[0], cmap, file),
            "julia" => julia(),
            _ => (),
        };
    }
    // julia(
    //     num::complex::Complex64::new(-0.4, 0.6),
    //     resx,
    //     resy,
    //     format!(
    //         "{}.{}",
    //         matches.value_of("output").unwrap(),
    //         matches.value_of("extension").unwrap()
    //     ),
    // );
}
