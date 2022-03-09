pub trait Processor<const CHANNELS: usize> {
    fn process(&mut self, buffers: &mut [&mut [f32]; CHANNELS]);
}
