use image::{ImageBuffer, Rgb};
use std::slice::ChunksMut;

pub struct ImageWriter{
    pub buffer: ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl ImageWriter{

    pub fn new(width: u32, height: u32) -> ImageWriter{
        ImageWriter{
            buffer: image::ImageBuffer::new(width, height)
        }
    }

    pub fn get_chunks(&mut self, chunk_size: usize) -> ChunksMut<u8>
    {
        self.buffer.chunks_mut(chunk_size)
    }

}