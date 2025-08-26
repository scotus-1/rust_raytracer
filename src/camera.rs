use std::{f32::INFINITY, io, io::Write};

use bevy::math::{vec3, Dir3, Ray3d, Vec3};
use crate::{color::{write_color, Color}, hittable::Hittable, interval::Interval, util::{degs_to_rads, random_sample_sq}, Point3};


const ASPECT_RATIO_DEF: f32 = 16.0 / 9.0;

// camera properties
const FOCAL_LENGTH: f32 = 1.0;
const CAMERA_CENTER: Point3 = Point3::new(0.0, 0.0, 0.0);

// Image dimensions in INT
const IMAGE_WIDTH_DEF: u32 = 800;
const VIEWPORT_HEIGHT: f32 = 2.0;

const SAMPLES_PER_PIXEL_DEF: u32 = 10;
const MAX_CHILD_RAYS_DEF: u32 = 10;
const VFOV_DEF: u32 = 90;


pub struct Camera {
    // aspect ratio is a real/ideal value
    pub aspect_ratio: f32,
    pub image_width: u32,
    image_height: u32,
    pub samples_per_pixel: u32,
    pub pixel_sample_scale: f32,
    pub max_child_rays: u32,
    pub vfov: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3
}

impl Camera {
    pub fn render<T: Hittable>(&mut self, world: &T) {
        self.init();
        // logging and ppm writing handles
        let stdout = io::stdout();
        let stderr = io::stderr();
        let mut out_handle: io::StdoutLock<'static> = stdout.lock();
        let mut err_handle: io::StderrLock<'static> = stderr.lock();


        let _ = writeln!(out_handle, "P3\n{} {}\n255", self.image_width,self.image_height); 
        
            for h in 0..self.image_height {
                for w in 0..self.image_width {
                    let mut pixel_color = Color::ZERO;
                    for _sample in 0..self.samples_per_pixel {
                        let r = self.get_ray(h, w);
                        pixel_color += Self::ray_color(&r, self.max_child_rays, world);
                    }

                    write_color(&mut out_handle, &(self.pixel_sample_scale * pixel_color));
                }

                let _ = writeln!(err_handle, "Scanlines remaining: {}", self.image_height-h); 

            };
    }

    pub fn new() -> Self {
        Camera { aspect_ratio: ASPECT_RATIO_DEF, image_width: IMAGE_WIDTH_DEF, image_height: 0, samples_per_pixel: SAMPLES_PER_PIXEL_DEF, pixel_sample_scale: 1.0/SAMPLES_PER_PIXEL_DEF as f32,
                max_child_rays: MAX_CHILD_RAYS_DEF, vfov: VFOV_DEF, center: Point3::ZERO, pixel00_loc: Point3::ZERO, pixel_delta_u: Vec3::ZERO, pixel_delta_v: Vec3::ZERO }
    }

    // init recalculates all basis vectors important to rendering based off any changes made explicitly to cut time
    fn init(&mut self) {
        // const image_height: u32 = if image_width < 1 {1} else {image_height};
        // image width & height are rounded to integers for pixel values
        self.image_height = (self.image_width as f32/self.aspect_ratio) as u32;

        // viewport properties
        let theta = degs_to_rads(self.vfov as f32);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * FOCAL_LENGTH;
        let viewport_width: f32 = viewport_height * (self.image_width as f32 / self.image_height as f32);
        // basically the image height and width step insure integer pixel values which are then coverted
        // back into a REALER TRUER USABLE ratio, aka the viewport height and width

        // viewport vectors and delta vectors, basically from (0,0) to whatever +u,+v
        let viewport_u: Vec3 = vec3(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = vec3(0.0, -viewport_height, 0.0);
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;
        
        // note: foward is in the -z direction, which is why the focal length vec is subbed from cam_center
        let viewport_upper_left = CAMERA_CENTER - vec3(0.0,0.0,FOCAL_LENGTH) - viewport_u/2f32 - viewport_v/2f32;
        // note: pixel00 is not where (0,0) on the u,v coords, pixel00 is the first middle of the first square pixel
        self.pixel00_loc = viewport_upper_left + 0.5*(self.pixel_delta_u + self.pixel_delta_v);

    }
    
    fn ray_color<T: Hittable>(r: &Ray3d, child_rays: u32, world: &T) -> Color {
        // by using Dir3d within Ray3d every Vec is normalized
        if child_rays <= 0 {return Color::new(0.0, 0.0, 0.0);}
        match world.hit(r, Interval::interval(0.001, INFINITY)) {
            Some(rec) => {
                match rec.mat.scatter(&r.direction, &rec) {
                    Some((attenuation, scattered)) => {
                        return attenuation * Self::ray_color(&scattered, child_rays-1, world)
                    },
                    None => {return Color::new(0.0,0.0,0.0)}
                }
            },
            None => ()
        }

        // remember -1 < y < 1, and not in the uv coordinates
        let t = (r.direction.y + 1.0) / 2.0;
        let color1 = Color::new(1.0,1.0,1.0);
        let color2 = Color::new(0.5,0.7,1.0);
        return color1.lerp(color2, t)
    }

    fn get_ray(&self, h: u32, w: u32) -> Ray3d {
        // create a ray with origin at at pxl center and then direction within a certain square
        let offset = random_sample_sq();
        let pixel_sample_pos: Point3 = self.pixel00_loc
                                        + ((w as f32 + offset.x)  * self.pixel_delta_u) 
                                        + ((h as f32 + offset.y) * self.pixel_delta_v);
 
        let ray_direction: Vec3 = pixel_sample_pos - self.center;
        Ray3d { origin: self.center, direction: Dir3::new(ray_direction).unwrap() }
    }


}