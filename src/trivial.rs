use arrayfire::*;

fn main() {
    const NUM_ROWS: u64 = 5;
    const NUM_COLS: u64 = 3;

    let dims = Dim4::new(&[NUM_ROWS, NUM_COLS, 1, 1]);
    let a = randu::<f32>(dims);

    af_print!("Create a 5-by-3 matrix of random floats on the GPU", a);

    info();
}
