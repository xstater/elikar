extern crate elikar;

#[test]
fn window_test(){
    use elikar::elikar::Elikar;

    #[allow(unused_variables)]
    let ek = Elikar::new()
        .unwrap();

    #[allow(unused_variables)]
    let mut window = elikar::window::Window::new("Fuck Rust",100,100,1280,800)
        .unwrap();

    // window.set_size(1288,888);
    println!("size:{:?}",window.size());
    window.set_position(200,200);
    println!("position:{:?}",window.position());
    // window.set_brightness(0.5).unwrap();
    println!("brightness:{}",window.brightness());
    // window.set_opacity(0.5).unwrap();
    println!("opacity:{:?}",window.opacity());
    // window.set_title("shit").unwrap();
    println!("title:{}",window.title());

    // window.hide();
    // window.maximize();
    // window.minimize();
    // window.raise();
    // window.restore();
    // window.set_fullscreen().unwrap();
    // window.set_fullscreen_desktop().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

}
