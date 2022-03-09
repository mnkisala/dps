use crate::prelude::*;
use crate::CV;
use crate::{FnMutProcessor, FnProcessor};

#[inline(always)]
pub fn apply<
    'a,
    F: FnMut(f32, &[CV; N_CVS]) -> f32 + 'a,
    const N_CVS: usize,
    const CHANNELS: usize,
>(
    mut cvs: [CV<'a>; N_CVS],
    mut f: F,
) -> impl Processor<CHANNELS> + 'a {
    FnMutProcessor::from(
        #[inline(always)]
        move |buffers: &mut [&mut [f32]; CHANNELS]| {
            for buffer in buffers {
                for sample in buffer.iter_mut() {
                    *sample = f(*sample, &cvs);

                    for cv in cvs.iter_mut() {
                        cv.next_value()
                    }
                }
            }
        },
    )
}

#[inline(always)]
pub fn apply_channelwise<
    'a,
    F: FnMut([f32; CHANNELS], &[CV; N_CVS]) -> [f32; CHANNELS] + 'a,
    const N_CVS: usize,
    const CHANNELS: usize,
>(
    mut cvs: [CV<'a>; N_CVS],
    mut f: F,
) -> impl Processor<CHANNELS> + 'a {
    FnMutProcessor::from(
        #[inline(always)]
        move |buffers: &mut [&mut [f32]; CHANNELS]| {
            for i in 0..buffers[0].len() {
                let mut tmp = [0.0; CHANNELS];
                for channel in 0..CHANNELS {
                    tmp[channel] = buffers[channel][i];
                }

                let tmp = f(tmp, &cvs);

                for channel in 0..CHANNELS {
                    buffers[channel][i] = tmp[channel];
                }

                for cv in cvs.iter_mut() {
                    cv.next_value()
                }
            }
        },
    )
}

#[inline(always)]
pub fn stereo_swap() -> impl Processor<2> {
    apply_channelwise(
        [],
        #[inline(always)]
        |[a, b], []| [b, a],
    )
}

#[inline(always)]
pub fn gain<'a, const CHANNELS: usize>(gain: CV<'a>) -> impl Processor<CHANNELS> + 'a {
    apply(
        [gain],
        #[inline(always)]
        |sample, [gain]| sample * gain.value(),
    )
}

#[inline(always)]
pub fn hard_clip<'a, const CHANNELS: usize>(treshold: CV<'a>) -> impl Processor<CHANNELS> + 'a {
    apply(
        [treshold],
        #[inline(always)]
        |sample, [treshold]| sample.clamp(-treshold.value(), treshold.value()),
    )
}

#[inline(always)]
pub fn copy<'a, const CHANNELS: usize>(
    inputs: [&'a [f32]; CHANNELS],
) -> impl Processor<CHANNELS> + 'a {
    FnProcessor::from(
        #[inline(always)]
        move |outputs: &mut [&mut [f32]; CHANNELS]| {
            for (input, output) in inputs.iter().zip(outputs.iter_mut()) {
                output.copy_from_slice(input)
            }
        },
    )
}
