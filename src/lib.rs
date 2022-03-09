#![cfg_attr(feature = "no_std", no_std)]

pub mod processor;
pub use processor::Processor;

pub mod fnprocessors;
pub use fnprocessors::{FnMutProcessor, FnProcessor};

pub mod thenapi;
pub use thenapi::ProcessorThenAPI;

pub mod prelude {
    pub use crate::Processor;
    pub use crate::ProcessorThenAPI;
}

pub mod processors;
pub use processors::*;

pub mod cv;
pub use cv::CV;

#[cfg(test)]
mod tests {
    use crate as dps;
    use dps::prelude::*;

    // TODO: make it nicer, and also maybe this crate's feature or a separete crate
    pub fn assert_close(left: &[f32], right: &[f32], distance: f32) {
        assert_eq!(left.len(), right.len());

        fn close(l: f32, r: f32, distance: f32) -> bool {
            (l - r).abs() < distance
        }

        for (l, r) in left.iter().zip(right) {
            if !close(*l, *r, distance) {
                panic!("assertion failed:\n left: {:?}\n right: {:?}", left, right)
            }
        }
    }

    // TODO: Real tests
    //
    #[test]
    fn single_processor_example() {
        let mut buffer = [1.0; 512];
        dps::gain(0.5.into()).process(&mut [&mut buffer]);
    }

    #[test]
    fn inputs_example() {
        let input = [1.0; 512];
        let mut output = [0.0; 512];

        dps::copy([&input])
            .then(dps::gain(0.5.into()))
            .process(&mut [&mut output]);
    }

    #[test]
    fn gain_example() {
        let mut buffer = [1.0; 512];

        dps::hard_clip(0.8.into()) // apply hard clip
            .then(dps::gain(0.5.into())) // then apply gain
            .process(&mut [&mut buffer]); // finally let's apply the processing chain to the output buffer
    }

    #[test]
    fn feels_good() {
        let input: &[f32] = &[1.0; 4];
        let mut output: &mut [f32] = &mut [0.0; 4];

        let variable_gain: &[f32] = &[0.5, 1.0, 2.0, 3.0];

        dps::copy([&input])
            .then(dps::hard_clip(0.8.into()))
            .then(
                dps::gain(0.5.into())
                    .then(dps::gain(0.5.into()))
                    .then(dps::gain(2.0.into())),
            )
            .then(dps::gain(1.5.into()))
            .then(dps::apply([], |sample, []| sample / 2.0))
            .then(dps::gain(variable_gain.into()))
            .process(&mut [&mut output]);

        assert_close(&output, &[0.15, 0.3, 0.6, 0.9], 1e-5)
    }

    #[test]
    fn stereo_swap() {
        let mut output_l = [0.5; 4];
        let mut output_r = [1.0; 4];

        dps::stereo_swap().process(&mut [&mut output_l, &mut output_r]);

        assert_eq!(output_r, [0.5; 4]);
        assert_eq!(output_l, [1.0; 4]);
    }
}
