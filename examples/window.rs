extern crate elikar;

fn main() {
    use elikar::elikar::Elikar;
    use elikar::window;

    let _ek = Elikar::new().unwrap();

    let window = window::Builder::default()
        .title("测试窗口")
        .position_centred()
        .size(1280, 700)
        // .maximized()
        .opengl()
        .build()
        .unwrap();

    // window.set_size(1288,888);
    println!("size:{:?}", window.size());
    // window.set_position(200,200);
    println!("position:{:?}", window.position());
    // window.set_brightness(0.5).unwrap();
    println!("brightness:{}", window.brightness());
    // window.set_opacity(0.5).unwrap();
    println!("opacity:{:?}", window.opacity().unwrap());
    // window.set_title("shit").unwrap();
    println!("title:{}", window.title());

    let id = window.id().unwrap();
    println!("id:{}",id);
    // window.hide();
    // window.show();
    // window.maximize();
    // window.minimize();
    // window.raise();
    // window.restore();
    // window.set_fullscreen().unwrap();
    // window.set_fullscreen_desktop().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));
}

