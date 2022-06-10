use crate::Scene::Scene;
use glam::Vec3;
use crate::VecExtension::VectorExtension;
use crate::Sphere::Sphere;
use crate::Hittable::Hittable;
use crate::HitInfoCollector::HitInfoCollector;
use crate::Camera::Camera;

pub fn Render(scene: &mut Scene, width: u32, height: u32)
{
    let anti_aliasing = 2;
    let scale = 1.0 / ((anti_aliasing * anti_aliasing) as f32);
    let anti_aliasing_subdivide = 0.5 / (anti_aliasing as f32);
    let mut buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(width, height);
    let mut sphere = Sphere::new(Vec3::zero(), Vec3::one());

    for x in 0..width - 1 {
        for y in 0..height - 1 {
            let mut color = Vec3::zero();

            for s in 0..anti_aliasing - 1 {
                for t in 0..anti_aliasing - 1 {
                    let i = ((x as f32) + (anti_aliasing_subdivide + 0.5 * ((t as f32) + 1.0))) / (width as f32);
                    let j = ((y as f32) + (anti_aliasing_subdivide + 0.5 * ((s as f32) + 1.0))) / (height as f32);
                    let ray = scene.camera.get_ray(
                        i,
                        j,
                        (width as f32) / (height as f32)
                    );

                    //if scene.closest_intersected(&ray).t > 0.0{
//                        color = Vec3::one();
                    //
                    // }
                    let mut collector = HitInfoCollector::default();
                    if sphere.intersect(&ray, &mut collector){
                        println!("Hit Pos : {:?}", collector.hitNormal);
                        color = collector.hitNormal.abs();// Vec3::one();
                    }

                }
            }
            //color *= scale;
            *buffer.get_pixel_mut(x, y) = image::Rgb([(color.x * 255.0) as u8, (color.y * 255.0) as u8, (color.z * 255.0) as u8]);

        }
    }
    buffer.save("Output.png");
}


pub fn render_pixel(x: u32, y: u32, camera: &mut Camera)
{

}