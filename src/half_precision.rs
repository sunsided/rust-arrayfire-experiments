use arrayfire::*;
use half::f16;
use std::panic;

fn main() {
    // NVIDIA GTX 980 Ti (Compute capability 5.2) on CUDA 10 doesn't do half-precision FP.
    set_backend(Backend::CPU);
    set_device(0);
    info();

    let result = panic::catch_unwind(|| {
        let values: Vec<_> = (1u8..101).map(std::convert::From::from).collect();
        let half_values = values.iter().map(|&x| f16::from_f32(x)).collect::<Vec<_>>();
        let hvals = Array::new(&half_values, Dim4::new(&[10, 10, 1, 1]));

        print(&hvals);
    });

    if result.is_err() {
        eprintln!("Backend doesn't support half::f16.");
    }
}
