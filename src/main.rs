use ab_glyph_rasterizer::{Point, Rasterizer};

use std::{
    io,
    path::PathBuf,
    str::{self, FromStr},
};

fn main() {
    let mut rasterizer = Rasterizer::new(0, 0);

    for result in walkdir::WalkDir::new("fonts/") {
        let entry = result.unwrap();

        if entry.file_type().is_file() {
            let commands = read_commands(&entry);
            rasterize_glyph(&mut rasterizer, &commands);
            write_png(&entry, &rasterizer);
        }
    }
}

fn read_commands(entry: &walkdir::DirEntry) -> Vec<Command> {
    let bytes = std::fs::read(entry.path()).unwrap();
    let string = String::from_utf8(bytes).unwrap();
    let mut commands = vec![];
    for line in string.lines() {
        let cmd = Command::from_str(line).unwrap();
        commands.push(cmd);
    }

    commands
}

fn rasterize_glyph(rasterizer: &mut Rasterizer, commands: &[Command]) {
    for &cmd in commands {
        match cmd {
            Command::Reset(w, h) => {
                rasterizer.reset(w, h);
            }
            Command::Line(a, b) => {
                rasterizer.draw_line(a.into(), b.into());
            }
            Command::Quad(a, b, c) => {
                rasterizer.draw_quad(a.into(), b.into(), c.into());
            }
        }
    }
}

fn serialize_to_rgba(rasterizer: &Rasterizer) -> Vec<u8> {
    let (width, height) = rasterizer.dimensions();
    let mut rgba = Vec::<u8>::with_capacity(width * height * 4);

    rasterizer.for_each_pixel(|_, a| {
        let b = (a * 255.0).round().min(255.0) as u8;
        for _ in 0..4 {
            rgba.push(b);
        }
    });

    rgba
}

fn write_png(entry: &walkdir::DirEntry, rasterizer: &Rasterizer) {
    let path = PathBuf::from("glyphs")
        .join(entry.path().file_stem().unwrap())
        .with_extension("png");
    let file = std::fs::File::create(path).unwrap();
    let mut file = io::BufWriter::new(file);

    let rgba = serialize_to_rgba(rasterizer);

    let (width, height) = rasterizer.dimensions();
    let mut encoder = png::Encoder::new(&mut file, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::new(1.0));

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&rgba).unwrap(); // Save
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Reset(usize, usize),
    Quad(Point, Point, Point),
    Line(Point, Point),
}

fn parse_point(outer: &str) -> Point {
    let end = outer.len();
    let inner = &outer[1..end - 1];
    let mut iter = inner.split(",");
    let val = iter.next().unwrap();
    let x = f32::from_str(val).unwrap();
    let val = iter.next().unwrap();
    let y = f32::from_str(val).unwrap();
    Point { x, y }
}

impl str::FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(":");
        match iter.next().unwrap() {
            "reset" => {
                let mut iter = iter.next().unwrap().trim().split("x");
                let width = iter.next().unwrap().parse().unwrap();
                let height = iter.next().unwrap().parse().unwrap();

                Ok(Command::Reset(width, height))
            }
            "quad" => {
                let mut segments = iter.next().unwrap().trim().split(" ");

                let a = parse_point(segments.next().unwrap());
                let b = parse_point(segments.next().unwrap());
                let c = parse_point(segments.next().unwrap());

                Ok(Command::Quad(a, b, c))
            }
            "line" => {
                let mut segments = iter.next().unwrap().trim().split(" ");

                let a = parse_point(segments.next().unwrap());
                let b = parse_point(segments.next().unwrap());

                Ok(Command::Line(a, b))
            }
            cmd => panic!("unknown command: {}", cmd),
        }
    }
}
