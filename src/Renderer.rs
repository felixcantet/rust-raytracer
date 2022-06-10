use crate::Scene::Scene;
use glam::Vec3;
use crate::VecExtension::VectorExtension;
use crate::Sphere::Sphere;
use crate::Hittable::Hittable;
use crate::HitInfoCollector::HitInfoCollector;
use crate::Camera::Camera;
use rayon::prelude::*;
use rayon::{ThreadPool, Scope};
use crate::Tracer::Tracer;
use rand::{thread_rng, Rng};

pub fn Render(scene: &mut Scene, width: u32, height: u32)
{
    let anti_aliasing = 4;
    let scale = 1.0 / ((anti_aliasing * anti_aliasing) as f32);
    let anti_aliasing_subdivide = 0.5 / (anti_aliasing as f32);
    let mut buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(width, height);
    let mut sphere = Sphere::new(Vec3::zero(), Vec3::one());

    let cpu_count = num_cpus::get();
    let chunk_buffer_size = (width * height * 3) / (cpu_count as u32);
    //let mut chunks: Vec<&mut [u8]> = buffer.chunks_mut(3 as usize).collect();
    // chunks.par_iter_mut().for_each(|p| {
    //     println!("{}", p.len());
    // });
    //
    //let mut boxedScene = Box::new(scene);

    buffer.par_chunks_mut(3)
        .enumerate()
        .for_each(move |(i, chunk)| {
            //boxedScene.camera.transform.translate(Vec3::one())


            //
            // let mut color : Vec3 = (0..6).into_par_iter().map(|_samples|{
            //     let mut rng = rand::thread_rng();
            //     let x = ((i as f32) % (width as f32) + rng.gen_range(-1.0..1.0)) / (width as f32);
            //     let y = ((i as f32) / (width as f32) + rng.gen_range(-1.0..1.0)) / (height as f32);
            //     let mut ray = &mut scene.camera.get_ray(x, y, (width as f32) / (height as f32));
            //     let intersect = scene.closest_intersected(ray);
            //     intersect.hitNormal.abs()
            // }).sum();
            // println!(color / 6);
            let x = (i as f32) % (width as f32);
            let y = (i as f32) / (width as f32);
            let mut color = Vec3::zero();
            for s in 0..anti_aliasing - 1 {
                for t in 0..anti_aliasing - 1 {
                    let i = ((x as f32) + (anti_aliasing_subdivide + 0.5 * ((t as f32) + 1.0))) / (width as f32);
                    let j = ((y as f32) + (anti_aliasing_subdivide + 0.5 * ((s as f32) + 1.0))) / (height as f32);
                    let mut ray = &mut scene.camera.get_ray(
                        i,
                        j,
                        (width as f32) / (height as f32),
                    );
                    let collector = scene.closest_intersected(&mut ray);
                    //let collector = scene.closest_intersected(&ray);
                    if collector.t > 0.0 {
                        color += collector.hitNormal.abs();
                    }
                }
            }
            // let mut ray = &mut scene.camera.get_ray(x, y, (width as f32) / (height as f32));
            // let intersect = scene.closest_intersected(ray);
            color *= scale;
            color = color.abs();

            // Gamma Correction :
            let r = color.x.powf(1.0 / 2.2);
            let g = color.y.powf(1.0 / 2.2);
            let b = color.z.powf(1.0 / 2.2);

            chunk[0] = (r * 255.99) as u8;
            chunk[1] = (g * 255.99) as u8;
            chunk[2] = (b * 255.99) as u8;
            // if intersect.t > 0.0 {
            //     println!("Hit : {:?}", intersect);
            // }
        });
    
    buffer.save("Output.png");
    //println!("Pos : {:?}", scene.camera.transform.get_position());

    //println!("Len : {}",chunks.len());


    // for x in 0..width - 1 {
    //     for y in 0..height - 1 {
    //         let mut color = Vec3::zero();
    //
    //         for s in 0..anti_aliasing - 1 {
    //             for t in 0..anti_aliasing - 1 {
    //                 let i = ((x as f32) + (anti_aliasing_subdivide + 0.5 * ((t as f32) + 1.0))) / (width as f32);
    //                 let j = ((y as f32) + (anti_aliasing_subdivide + 0.5 * ((s as f32) + 1.0))) / (height as f32);
    //                 let ray = scene.camera.get_ray(
    //                     i,
    //                     j,
    //                     (width as f32) / (height as f32)
    //                 );
    //                 let collector = scene.closest_intersected(&ray);
    //                 if collector.t > 0.0{
    //                    color += collector.hitNormal.abs();
    //
    //                 }
    //                 // let mut collector = HitInfoCollector::default();
    //                 // if sphere.intersect(&ray, &mut collector){
    //                 //     println!("Hit Pos : {:?}", collector.hitNormal);
    //                 //     color = collector.hitNormal.abs();// Vec3::one();
    //                 // }
    //
    //             }
    //         }
    //         color *= scale;
    //         *buffer.get_pixel_mut(x, y) = image::Rgb([(color.x * 255.0) as u8, (color.y * 255.0) as u8, (color.z * 255.0) as u8]);
    //
    //     }
    // }
    // buffer.save("Output.png");
}


pub fn render_pixel(x: u32, y: u32, camera: &mut Camera)
{}