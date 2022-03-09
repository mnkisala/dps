use crate::Processor;

pub trait ConsOrNil<const CHANNELS: usize>: Processor<CHANNELS> {}

pub struct Nil;

impl<const CHANNELS: usize> Processor<CHANNELS> for Nil {
    #[inline(always)]
    fn process(&mut self, _buffers: &mut [&mut [f32]; CHANNELS]) {}
}

impl<const CHANNELS: usize> ConsOrNil<CHANNELS> for Nil {}

pub struct Cons<T: Processor<CHANNELS>, Next: ConsOrNil<CHANNELS>, const CHANNELS: usize> {
    processor: T,
    next: Next,
}
impl<T: Processor<CHANNELS>, Next: ConsOrNil<CHANNELS>, const CHANNELS: usize> ConsOrNil<CHANNELS>
    for Cons<T, Next, CHANNELS>
{
}

impl<T, Next, const CHANNELS: usize> Processor<CHANNELS> for Cons<T, Next, CHANNELS>
where
    T: Processor<CHANNELS>,
    Next: ConsOrNil<CHANNELS>,
{
    #[inline(always)]
    fn process(&mut self, buffers: &mut [&mut [f32]; CHANNELS]) {
        self.processor.process(buffers);
        self.next.process(buffers);
    }
}

pub trait ProcessorThenAPI<const CHANNELS: usize>: Processor<CHANNELS> {
    fn then<Next: Processor<CHANNELS>>(
        self,
        next: Next,
    ) -> Cons<Self, Cons<Next, Nil, CHANNELS>, CHANNELS>
    where
        Self: Sized;
}

impl<T: Processor<CHANNELS>, const CHANNELS: usize> ProcessorThenAPI<CHANNELS> for T {
    #[inline(always)]
    fn then<Next: Processor<CHANNELS>>(
        self,
        next: Next,
    ) -> Cons<Self, Cons<Next, Nil, CHANNELS>, CHANNELS>
    where
        Self: Sized,
    {
        Cons {
            processor: self,
            next: Cons {
                processor: next,
                next: Nil,
            },
        }
    }
}
