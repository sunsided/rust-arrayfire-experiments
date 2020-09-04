use arrayfire::*;

fn main() {
    set_device(0);
    info();

    let wnd = Window::new(1280, 720, String::from("White Noise"));

    // TODO: Need to clarify channel ordering. On my NVIDIA GPU, order is HWCN, but the original code had WHCN.
    let dims = Dim4::new(&[720, 1280, 1, 1]);

    loop {
        wnd.draw_image(&randu::<f32>(dims), None);
        if wnd.is_closed() {
            break;
        }
    }
}
