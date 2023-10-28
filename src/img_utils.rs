use std::{borrow::Cow, f64::consts::FRAC_PI_2};

use arboard::{Clipboard, ImageData};
use image::DynamicImage;
use imageproc::point::Point;

pub enum CapStyle {
    Butt,
    Square,
}

pub fn thick_line_to_polygon(
    line_start: (i32, i32),
    line_end: (i32, i32),
    thickness: i32,
    cap_style: CapStyle,
) -> [Point<i32>; 4] {
    let dx = f64::from(line_end.0 - line_start.0);
    let dy = f64::from(line_end.1 - line_start.1);
    let line_angle_rad = dy.atan2(dx);
    let travel_angle = line_angle_rad
        + match cap_style {
            CapStyle::Square => FRAC_PI_2 / 2.0,
            CapStyle::Butt => FRAC_PI_2,
        };
    let distance_from_center = f64::from(thickness) / 2.0;
    let offset = (
        (travel_angle.cos() * distance_from_center).round() as i32,
        (travel_angle.sin() * distance_from_center).round() as i32,
    );
    [
        Point::new(line_start.0 + offset.0, line_start.1 + offset.1),
        Point::new(line_end.0 + offset.0, line_end.1 + offset.1),
        Point::new(line_end.0 - offset.0, line_end.1 - offset.1),
        Point::new(line_start.0 - offset.0, line_start.1 - offset.1),
    ]
}

pub fn copy_image(image: &DynamicImage) {
    let rgb = image.to_rgba8();
    let data = ImageData {
        height: rgb.height() as usize,
        width: rgb.width() as usize,
        bytes: Cow::Borrowed(rgb.as_raw()),
    };
    // println!("{:?}", data);
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_image(data).unwrap();
}
