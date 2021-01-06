use image::{DynamicImage, GenericImageView};
use glium::texture::RawImage2d;
use glium::Display;

pub struct Texture {
    pub image: DynamicImage,
}

impl Texture {
    pub fn from_file(path: &str) -> Texture {
        Texture {
            image: image::open(path).unwrap(),
        }
    }

    pub fn as_raw_image_2d(&self) -> RawImage2d<u8> {
        return glium::texture::RawImage2d::from_raw_rgba_reversed(&self.image.to_rgba().into_raw(), self.image.dimensions());
    }
}

pub struct TextureBag {
    pub block: glium::Texture2d,
    pub lblock: glium::Texture2d,
    pub reversed_lblock: glium::Texture2d,
    pub line: glium::Texture2d,
    pub square: glium::Texture2d,
    pub squiggle: glium::Texture2d,
    pub reversed_squiggle: glium::Texture2d,
    pub tblock: glium::Texture2d,
    pub cup: glium::Texture2d,
}

impl TextureBag {
    pub fn init(display: &Display) -> TextureBag {
        TextureBag {
            block: glium::texture::Texture2d::new(display, Texture::from_file("block.png").as_raw_image_2d()).unwrap(),
            lblock: glium::texture::Texture2d::new(display, Texture::from_file("lblock.png").as_raw_image_2d()).unwrap(),
            reversed_lblock: glium::texture::Texture2d::new(display, Texture::from_file("reversed_lblock.png").as_raw_image_2d()).unwrap(),
            line: glium::texture::Texture2d::new(display, Texture::from_file("line.png").as_raw_image_2d()).unwrap(),
            square: glium::texture::Texture2d::new(display, Texture::from_file("square.png").as_raw_image_2d()).unwrap(),
            squiggle: glium::texture::Texture2d::new(display, Texture::from_file("squiggle.png").as_raw_image_2d()).unwrap(),
            reversed_squiggle: glium::texture::Texture2d::new(display, Texture::from_file("reversed_squiggle.png").as_raw_image_2d()).unwrap(),
            tblock: glium::texture::Texture2d::new(display, Texture::from_file("tblock.png").as_raw_image_2d()).unwrap(),
            cup: glium::texture::Texture2d::new(display, Texture::from_file("cup.png").as_raw_image_2d()).unwrap(),
        }
    }
}