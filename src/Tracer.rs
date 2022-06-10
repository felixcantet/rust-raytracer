use crate::Scene::Scene;
use glam::Vec3;
use image::{Rgb, ImageBuffer};
use crate::Ray::Ray;
use rayon::prelude::*;
use crate::VecExtension::VectorExtension;

#[derive(Debug, Copy, Clone)]
pub struct RenderSettings {
    pub samples: u16,
    pub width: u32,
    pub height: u32,
    //pub output_path: String,
    anti_aliasing_subdivide: f32,
    depth: usize
}

impl RenderSettings {
    pub fn new(samples: u16, width: u32, height: u32, depth: usize) -> Self {
        RenderSettings {
            samples,
            width,
            height,
            //output_path,
            anti_aliasing_subdivide: 0.5 / samples as f32,
            depth
        }
    }

    pub fn get_aspect_ratio(&self) -> f32
    {
        self.width as f32 / self.height as f32
    }

    pub fn get_antialiasing_subdivide(&self) -> f32 {
        0.5 / self.samples as f32
    }
}

pub struct Tracer {
    scene: Scene,
    settings: RenderSettings,
    buffer: image::ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Tracer {
    pub fn new(scene: Scene, settings: RenderSettings) -> Self {
        Tracer {
            scene,
            settings,
            buffer: ImageBuffer::new(settings.width, settings.height),
        }
    }

    pub fn to_2d_index(&self, index_1d: usize) -> (usize, usize) {
        (
            index_1d % self.settings.width as usize,
            index_1d / self.settings.width as usize
        )
    }

    pub fn get_uv(&self, x: usize, y: usize, s: usize, t: usize) -> (f32, f32)
    {
        let subdivided = self.settings.anti_aliasing_subdivide;
        let i = (x as f32) + (subdivided + 0.5 * (t as f32)) + 1.0;
        let j = (y as f32) + (subdivided + 0.5 * (s as f32)) + 1.;

        (i / (self.settings.width as f32), j / (self.settings.height as f32))
    }

    pub fn get_ray(&self, index: usize, s: usize, t: usize) -> Ray {
        let (x, y) = self.to_2d_index(index);
        let (x, y) = self.get_uv(x, y, s, t);
        self.scene.camera.get_ray(x, y, self.settings.get_aspect_ratio())
    }

    pub fn trace_pixel(&self, pixel_index: usize) -> Vec3 {
        let mut final_color = Vec3::zero();

        for s in 0..self.settings.samples as usize{
            for t in 0..self.settings.samples as usize{
                let mut color = Vec3::zero();
                let mut ray = self.get_ray(pixel_index, s, t);
                let mut intersect = self.scene.closest_intersected(&mut ray);
                if intersect.t < 0.0{

                    continue;
                }
                color += intersect.hitNormal.abs();
                let mut depth = 1;
                while intersect.t > 0.0 && depth > self.settings.depth{
                    let normal = intersect.hitNormal;
                    let origin = intersect.hitPosition + (normal * f32::EPSILON);
                    ray = Ray::new(origin, normal.normalize());
                    intersect = self.scene.closest_intersected(&mut ray);
                    //color = Vec3::new(color.x * intersect.hitNormal.x, color.y * intersect.hitNormal.y, color.z * intersect.hitNormal.z);
                    color *= intersect.hitNormal.abs();
                }
                final_color += color;

            }
        }
        final_color * (1.0 / (self.settings.samples as f32 * self.settings.samples as f32))
    }

    pub fn render(&mut self) {

        let mut buffer: image::ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(self.settings.width, self.settings.height);
        buffer.par_chunks_mut(3).enumerate().for_each(move |(i, chunk)|{
            let color = self.trace_pixel(i);
            chunk[0] = (color.x * 255.99) as u8;
            chunk[1] = (color.y * 255.99) as u8;
            chunk[2] = (color.z * 255.99) as u8;
        });

        buffer.save("Output.png");

    }


    // pub fn Trace(&self, scene: &Scene, ){
    //
    // }
}