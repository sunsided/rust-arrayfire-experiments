use arrayfire::*;

fn main() {
    let filename = std::path::PathBuf::from(".images")
        .join("arrow-300x168.jpg")
        .to_str()
        .expect("unable to open the image file")
        .to_owned();

    let image = load_image::<f32>(filename, true) / 256f32;
    let image_size = image.dims();

    let hist = histogram(&image, 256, 0f64, 1f64);

    let hist_red = get_channel(&hist, 0);
    let hist_green = get_channel(&hist, 1);
    let hist_blue = get_channel(&hist, 2);

    let win = Window::new(
        image_size[1] as i32,
        image_size[0] as i32,
        "Image".to_string(),
    );
    let win_r = Window::new(512, 512, "Histogram (Red Channel)".to_string());
    let win_g = Window::new(512, 512, "Histogram (Green Channel)".to_string());
    let win_b = Window::new(512, 512, "Histogram (Blue Channel)".to_string());

    loop {
        win.draw_image(&image, None);
        win_r.draw_hist(&hist_red, 0f64, 1f64, None);
        win_g.draw_hist(&hist_green, 0f64, 1f64, None);
        win_b.draw_hist(&hist_blue, 0f64, 1f64, None);

        if win.is_closed() || win_r.is_closed() || win_g.is_closed() || win_b.is_closed() {
            break;
        }
    }
}

/// Gets a specific channel of an image, or histogram.
fn get_channel<T>(image: &Array<T>, channel: u64) -> Array<T>
where
    T: HasAfEnum,
{
    // The complicated path involves creating a sequence, then indexing into the array.
    // See: http://arrayfire.org/arrayfire-rust/arrayfire/book/indexing.html
    /*
    let seqs = &[
        Seq::default(),
        Seq::default(),
        Seq::new(channel as u32, channel as u32, 1),
        Seq::default(),
    ];

    index(&image, seqs)
    */

    // This one appears to do the same.
    slice(&image, channel)
}
