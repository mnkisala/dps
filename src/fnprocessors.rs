use crate::Processor;

pub struct FnMutProcessor<T: FnMut(&mut [&mut [f32]; CHANNELS]), const CHANNELS: usize> {
    processor: T,
}

impl<T: FnMut(&mut [&mut [f32]; CHANNELS]), const CHANNELS: usize> Processor<CHANNELS>
    for FnMutProcessor<T, CHANNELS>
{
    #[inline(always)]
    fn process(&mut self, buffers: &mut [&mut [f32]; CHANNELS]) {
        (self.processor)(buffers)
    }
}

impl<T: FnMut(&mut [&mut [f32]; CHANNELS]), const CHANNELS: usize> From<T>
    for FnMutProcessor<T, CHANNELS>
{
    #[inline(always)]
    fn from(f: T) -> Self {
        Self { processor: f }
    }
}

pub struct FnProcessor<T: Fn(&mut [&mut [f32]; CHANNELS]), const CHANNELS: usize> {
    processor: T,
}

impl<T: Fn(&mut [&mut [f32]; CHANNELS]), const CHANNELS: usize> Processor<CHANNELS>
    for FnProcessor<T, CHANNELS>
{
    #[inline(always)]
    fn process(&mut self, buffers: &mut [&mut [f32]; CHANNELS]) {
        (self.processor)(buffers)
    }
}

impl<T: Fn(&mut [&mut [f32]; CHANNELS]), const CHANNELS: usize> From<T>
    for FnProcessor<T, CHANNELS>
{
    #[inline(always)]
    fn from(f: T) -> Self {
        Self { processor: f }
    }
}
