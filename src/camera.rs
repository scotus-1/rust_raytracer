use std::{f32::INFINITY, io, io::Write};

use bevy::math::{vec3, Dir3, Ray3d, Vec3};
use rand::Rng;
use crate::{color::{write_color, Color}, hittable::Hittable, interval::Interval, Point3};


const ASPECT_RATIO_DEF: f32 = 16.0 / 9.0;

// camera properties
const FOCAL_LENGTH: f32 = 1.0;
const CAMERA_CENTER: Point3 = Point3::ZERO;

// Image dimensions in INT
const IMAGE_WIDTH_DEF: u32 = 800;
const VIEWPORT_HEIGHT: f32 = 2.0;

const SAMPLES_PER_PIXEL_DEF: u32 = 10;


pub struct Camera {
    // aspect ratio is a real/ideal value
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    pixel_sample_scale: f32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3
}

impl Camera {
    pub fn render<T: Hittable>(&self, world: &T) {
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
                        pixel_color += Self::ray_color(&r, world);
                    }

                    write_color(&mut out_handle, &(self.pixel_sample_scale * pixel_color));
                }

                let _ = writeln!(err_handle, "Scanlines remaining: {}", self.image_height-h); 

            };
    }

    pub fn new() -> Self {
        let mut cam = Camera { aspect_ratio: ASPECT_RATIO_DEF, image_width: IMAGE_WIDTH_DEF, image_height: 0, samples_per_pixel: SAMPLES_PER_PIXEL_DEF, pixel_sample_scale: 1.0/SAMPLES_PER_PIXEL_DEF as f32,
                                center: Point3::ZERO, pixel00_loc: Point3::ZERO, pixel_delta_u: Vec3::ZERO, pixel_delta_v: Vec3::ZERO };
        cam.init(ASPECT_RATIO_DEF, IMAGE_WIDTH_DEF);
        cam
    }

    // init recalculates all basis vectors important to rendering based off any changes made explicitly to cut time
    fn init(&mut self, aspect_ratio: f32, image_width: u32) {
        // const image_height: u32 = if image_width < 1 {1} else {image_height};
        // image width & height are rounded to integers for pixel values
        self.image_height = (image_width as f32/aspect_ratio) as u32;

        // viewport properties
        let viewport_width: f32 = VIEWPORT_HEIGHT * (image_width as f32 / self.image_height as f32);
        // basically the image height and width step insure integer pixel values which are then coverted
        // back into a REALER TRUER USABLE ratio, aka the viewport height and width

        // viewport vectors and delta vectors, basically from (0,0) to whatever +u,+v
        let viewport_u: Vec3 = vec3(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = vec3(0.0, -VIEWPORT_HEIGHT, 0.0);
        self.pixel_delta_u = viewport_u / image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;
        
        // note: foward is in the -z direction, which is why the focal length vec is subbed from cam_center
        let viewport_upper_left = CAMERA_CENTER - vec3(0.0,0.0,FOCAL_LENGTH) - viewport_u/2f32 - viewport_v/2f32;
        // note: pixel00 is not where (0,0) on the u,v coords, pixel00 is the first middle of the first square pixel
        self.pixel00_loc = viewport_upper_left + 0.5*(self.pixel_delta_u + self.pixel_delta_v);

    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.init(aspect_ratio, self.image_width);
    }

    pub fn set_image_width(&mut self, image_width: u32) {
        self.image_width = image_width;
        self.init(self.aspect_ratio, image_width);
    }

    pub fn set_samples_per_pixel(&mut self, spp: u32) {
        self.samples_per_pixel = spp;
        self.pixel_sample_scale = 1.0/spp as f32;
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

    fn get_ray(&self, h: u32, w: u32) -> Ray3d {
        // create a ray with origin at at pxl center and then direction within a certain square
        let offset = Self::random_sample_sq();
        let pixel_sample_pos: Point3 = self.pixel00_loc
                                        + ((w as f32 + offset.x)  * self.pixel_delta_u) 
                                        + ((h as f32 + offset.y) * self.pixel_delta_v);
 
        let ray_direction: Vec3 = pixel_sample_pos - self.center;
        Ray3d { origin: self.center, direction: Dir3::new(ray_direction).unwrap() }
    }

    fn random_sample_sq() -> Vec3 {
        let mut rand = rand::thread_rng();
        Vec3 { x: rand.gen::<f32>() - 0.5 , y: rand.gen::<f32>() - 0.5, z: 0.0 }
    }
}