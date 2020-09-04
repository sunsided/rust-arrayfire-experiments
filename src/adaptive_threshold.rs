use arrayfire::*;

enum LocalThresholdType {
    Mean,
    Median,
    MinMaxAverage,
}

fn main() {
    let filename = std::path::PathBuf::from(".images")
        .join("sudoku-300x225.jpg")
        .to_str()
        .expect("unable to open the image file")
        .to_owned();

    let image = load_image::<u8>(filename, true) / 255f32;
    let image_size = image.dims();

    // Magic numbers below brought to you by Fiddling Around Until It Works (tm).
    let t_mean = adaptive_threshold(&image, LocalThresholdType::Mean, 9, 0.03f32);
    let t_median = adaptive_threshold(&image, LocalThresholdType::Median, 13, 0.05f32);
    let t_minmax = adaptive_threshold(&image, LocalThresholdType::MinMaxAverage, 9, 0.01f32);

    let win = Window::new(
        2 * image_size[1] as i32,
        2 * image_size[0] as i32,
        "Adaptive Threshold".to_string(),
    );

    let quad = stack_images(image, t_mean, t_median, t_minmax);
    while !win.is_closed() {
        win.draw_image(&quad, None);
    }
}

/// Applies adaptive thresholding as described in [Image editing using ArrayFire: Part 3](https://arrayfire.com/image-editing-using-arrayfire-part-3-2/).
fn adaptive_threshold(
    in_: &Array<f32>,
    kind: LocalThresholdType,
    window_size: u64,
    threshold: f32,
) -> Array<f32> {
    let wr = window_size;
    let gray = rgb_to_gray(in_);

    let diff = match kind {
        LocalThresholdType::Mean => adaptive_threshold_mean(&gray, wr),
        LocalThresholdType::Median => adaptive_threshold_median(&gray, wr),
        LocalThresholdType::MinMaxAverage => adaptive_threshold_minmaxavg(&gray, wr),
    };

    1f32 - gt(&diff, &threshold, false)
}

/// Adaptive thresholding using the mean of a window region.
fn adaptive_threshold_mean(gray: &Array<f32>, wr: u64) -> Array<f32> {
    let box_kernel = constant::<f32>(1f32, Dim4::new(&[wr, wr, 1, 1])) / (wr * wr);
    let mean = convolve2(&gray, &box_kernel, ConvMode::DEFAULT, ConvDomain::SPATIAL);
    mean - gray
}

/// Adaptive thresholding using the median of a window region.
fn adaptive_threshold_median(gray: &Array<f32>, wr: u64) -> Array<f32> {
    let median = medfilt(&gray, wr, wr, BorderType::SYMMETRIC);
    median - gray
}

/// Adaptive thresholding using the average of the minimum and maximum value of a window region.
fn adaptive_threshold_minmaxavg(gray: &Array<f32>, wr: u64) -> Array<f32> {
    let min = minfilt(&gray, wr, wr, BorderType::ZERO);
    let max = maxfilt(&gray, wr, wr, BorderType::ZERO);
    let mean = (min + max) * 0.5f32;
    mean - gray
}

/// Converts RGB images to grayscale.
fn rgb_to_gray(image: &Array<f32>) -> Array<f32> {
    color_space(image, ColorSpace::GRAY, ColorSpace::RGB)
}

/// Converts grayscale images to RGB.
fn gray_to_rgb(image: &Array<f32>) -> Array<f32> {
    color_space(image, ColorSpace::RGB, ColorSpace::GRAY)
}

/// Stacks all four images into one.
fn stack_images(
    image: Array<f32>,
    t_mean: Array<f32>,
    t_median: Array<f32>,
    t_minmax: Array<f32>,
) -> Array<f32> {
    let row_top = join(1, &image, &gray_to_rgb(&t_mean));
    let row_bottom = join(1, &gray_to_rgb(&t_median), &gray_to_rgb(&t_minmax));
    join(0, &row_top, &row_bottom)
}
