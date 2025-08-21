use std::{f32::INFINITY, io::{self, Write}, rc::Rc};
use bevy::math::{vec3, Dir3, Ray3d, Vec3};

use crate::{color::*, hittable::Hittable, hittable_list::HittableList, sphere::sphere};
use crate::{interval::Interval};
pub type Point3 = Vec3;

mod color;
mod hittable;
mod sphere;
mod hittable_list;
mod util;
mod interval;

// aspect ratio is a real/ideal value
const ASPECT_RATIO: f32 = 16.0 / 9.0; 

// camera properties
const FOCAL_LENGTH: f32 = 1.0;
const CAMERA_CENTER: Point3 = Point3::ZERO;

// Image dimensions in INT
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32/ASPECT_RATIO) as u32;
// const image_height: u32 = if image_width < 1 {1} else {image_height};
// image width & height are rounded to integers for pixel values

// viewport properties
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);
// basically the image height and width step insure integer pixel values which are then coverted
// back into a REALER TRUER USABLE ratio, aka the viewport height and width

// viewport vectors and delta vectors, basically from (0,0) to whatever +u,+v
const VIEWPORT_U: Vec3 = vec3(VIEWPORT_WIDTH, 0.0, 0.0);
const VIEWPORT_V: Vec3 = vec3(0.0, -VIEWPORT_HEIGHT, 0.0);


fn main() {
    // these vectors are calculated at runtime even though they could be consts, todo!
    let pixel_delta_u: Vec3 = VIEWPORT_U / IMAGE_WIDTH as f32;
    let pixel_delta_v: Vec3 = VIEWPORT_V / IMAGE_HEIGHT as f32;
    
    // note: foward is in the -z direction, which is why the focal length vec is subbed from cam_center
    let viewport_upper_left = CAMERA_CENTER - vec3(0.0,0.0,FOCAL_LENGTH) - VIEWPORT_U/2f32 - VIEWPORT_V/2f32;
    // note: pixel00 is not where (0,0) on the u,v coords, pixel00 is the first middle of the first square pixel
    let pixel00_loc = viewport_upper_left + 0.5*(pixel_delta_u + pixel_delta_v);

    // logging and ppm writing handles
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut out_handle: io::StdoutLock<'static> = stdout.lock();
    let mut err_handle: io::StderrLock<'static> = stderr.lock();


    let _ = writeln!(out_handle, "P3\n{} {}\n255", IMAGE_WIDTH,IMAGE_HEIGHT); 


    let mut world: HittableList = HittableList::hittable_list(None);
    world.add(Rc::new(sphere(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(sphere(Point3::new(0.0, -50.0, -1.0), 45.0)));

    for h in 0..IMAGE_HEIGHT {
        for w in 0..IMAGE_WIDTH {
            let pixel_center: Point3 = pixel00_loc + (w as f32 * pixel_delta_u) + (h as f32 * pixel_delta_v);
            
            let r = Ray3d::new(CAMERA_CENTER, Dir3::new(pixel_center-CAMERA_CENTER).unwrap());
            let pixel_color = ray_color(&r, &world);
            write_color(&mut out_handle, &pixel_color);
        }

        let _ = writeln!(err_handle, "Scanlines remaining: {}", IMAGE_HEIGHT-h); 

    };
}


fn ray_color<T: Hittable>(r: &Ray3d, world: &T) -> Color {
    // by using Dir3d within Ray3d every Vec is normalized

    match world.hit(r, Interval::interval(0.0, INFINITY)) {
        Some(rec) => {
            return 0.5 * (rec.normal.as_vec3() + vec3(1.0, 1.0, 1.0));
        },
        None => ()
    }

    // remember -1 < y < 1, and not in the uv coordinates
    let t = (r.direction.y + 1.0) / 2.0;
    let color1 = Color::new(1.0,1.0,1.0);
    let color2 = Color::new(0.5,0.7,1.0);
    return color1.lerp(color2, t)
}
