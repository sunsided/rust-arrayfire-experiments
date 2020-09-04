use arrayfire::*;

fn main() {
    set_backend(Backend::CPU);
    set_device(0);
    info();

    let wnd = Window::new(1280, 720, String::from("White Noise"));

    // Channel order appears to be HWCN regardless of the backend.
    let dims = Dim4::new(&[720, 1280, 1, 1]);

    loop {
        wnd.draw_image(&randu::<f32>(dims), None);
        if wnd.is_closed() {
            break;
        }
    }
}
