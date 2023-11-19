use std::time::SystemTime;

use anyhow::Result;
use apriltag::{Detection, Detector, Family, Image};
use apriltag_image::prelude::*;
use image::{DynamicImage, Rgb};
use imageproc::drawing::draw_polygon_mut;
use nokhwa::utils::CameraIndex;
use show_image::{create_window, AsImageView, WindowProxy};

use crate::img_utils::{copy_image, thick_line_to_polygon, CapStyle};
use nokhwa::pixel_format::LumaFormat;
use nokhwa::Camera;
use nokhwa::utils::{RequestedFormat, RequestedFormatType};
use rusttype::{Scale, Font};

pub mod img_utils;

#[show_image::main]
fn main() -> Result<()> {
    let index = CameraIndex::Index(0); 
    let requested = RequestedFormat::new::<LumaFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    let mut camera = Camera::new(index, requested).unwrap();
    let window = create_window("AprilTag", Default::default()).unwrap();
    camera.open_stream()?;
    println!("{}", camera.is_stream_open());
    while camera.is_stream_open() {
        let frame = camera.frame().unwrap();
        let decoded = frame.decode_image::<LumaFormat>().unwrap();
        let mut detector = Detector::builder()
            .add_family_bits(Family::tag_16h5(), 1)
            .build()?;
        let image = Image::from_image_buffer(&decoded);
        // let tag_params = TagParams {
        //     cx: 1.0,
        //     cy: 1.0,
        //     fx: 1.0,
        //     fy: 1.0,
        //     tagsize: 0.04,
        // };
        let now = SystemTime::now();
        let tags = detector.detect(&image);
        // println!("{tags:#?}");
        // let tag_poses = tags
        //     .iter()
        //     .filter_map(|det| det.estimate_tag_pose(&tag_params));
        // for pose in tag_poses {
        //     println!("{pose:#?}");
        // }
        let elapsed = now.elapsed();
        println!("{:#?}", elapsed);
        display_april_tags(image::DynamicImage::ImageLuma8(decoded), tags, window.clone())?;
    }
    println!("{}", camera.is_stream_open());
    Ok(())
}

fn display_april_tags(background_image: DynamicImage, tags: Vec<Detection>, window: WindowProxy) -> anyhow::Result<()> {
    let mut image = background_image.to_rgb8();
    const RED: Rgb<u8> = Rgb([255, 0, 0]);
    for tag in tags {
        // filter (most) artifacts
        if tag.id() > 8 {
            continue;
        }

        let corners = tag.corners();
        // not sure if you can even have not 4 corners
        // shitty attempt to filter artifacts
        if corners.len() != 4 { 
            continue;
        }

        // calculate area
        let mut area = 0.0;
        for i in 0..corners.len() {
            let corner = corners[i].map(|x| x as i32);
            let next_corner = corners[(i + 1) % corners.len()].map(|x| x as i32);
            area += corner[0] as f64 * next_corner[1] as f64;
            area -= corner[1] as f64 * next_corner[0] as f64;
        }

        // hacky way to filter artifacts. will require changing.
        if (area.abs() / 2 as f64) < 1000.0 {
            continue; 
        }


        for i in 0..corners.len() {
            let corner = corners[i].map(|x| x as i32);
            let next_corner = corners[(i + 1) % corners.len()].map(|x| x as i32);
            let line =
                thick_line_to_polygon(corner.into(), next_corner.into(), 3, CapStyle::Square);
            draw_polygon_mut(&mut image, &line, RED);
        }

        // draw tag fidiucial id
        let center = tag.center().map(|x| x as i32);
        let text = format!("{}", tag.id());

        let font = Font::try_from_bytes(include_bytes!("../Roboto-Regular.ttf")).unwrap();

        imageproc::drawing::draw_text_mut(
            &mut image,
            RED,
            center[0],
            center[1],
            Scale::uniform(100.0),
            &font,
            &text,
        );
    }

    let image = DynamicImage::from(image);
    copy_image(&image);
    window.set_image("image-001", image.as_image_view()?)?; 
    Ok(())
}
