use anyhow::Result;
use apriltag::{Detection, Detector, Family, Image, TagParams};
use apriltag_image::prelude::*;
use image::{DynamicImage, Rgb};
use imageproc::drawing::draw_polygon_mut;
use show_image::{create_window, AsImageView};

use crate::img_utils::{copy_image, thick_line_to_polygon, CapStyle};

pub mod img_utils;

#[show_image::main]
fn main() -> Result<()> {
    let input_png = image::open("./apriltag.png")?;
    let mut detector = Detector::builder()
        .add_family_bits(Family::tag_36h11(), 1)
        .build()?;
    let image = Image::from_image_buffer(&input_png.to_luma8());
    let tag_params = TagParams {
        cx: 1.0,
        cy: 1.0,
        fx: 1.0,
        fy: 1.0,
        tagsize: 0.04,
    };
    let tags = detector.detect(&image);
    // println!("{tags:#?}");
    let tag_poses = tags
        .iter()
        .filter_map(|det| det.estimate_tag_pose(&tag_params));
    for pose in tag_poses {
        println!("{pose:#?}");
    }
    display_april_tags(input_png, tags)?;
    Ok(())
}

fn display_april_tags(background_image: DynamicImage, tags: Vec<Detection>) -> anyhow::Result<()> {
    let mut image = background_image.to_rgb8();
    const RED: Rgb<u8> = Rgb([255, 0, 0]);
    for tag in tags {
        let corners = tag.corners();
        for i in 0..corners.len() {
            let corner = corners[i].map(|x| x as i32);
            let next_corner = corners[(i + 1) % corners.len()].map(|x| x as i32);
            let line =
                thick_line_to_polygon(corner.into(), next_corner.into(), 3, CapStyle::Square);
            draw_polygon_mut(&mut image, &line, RED);
        }
    }

    let image = DynamicImage::from(image);
    copy_image(&image);
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image.as_image_view()?)?;
    window.wait_until_destroyed()?;
    Ok(())
}
