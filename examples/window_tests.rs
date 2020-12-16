extern crate elikar;

fn main(){
    use elikar::elikar::Elikar;
    use elikar::window;

    let _ek = Elikar::new().unwrap();

    let mut wm = window::Manager::new();
    let wid  = wm.add_windows(window::builder()
        .title("测试窗口")
        .position_centred()
        .size(1280, 700)
        .maximized()
        .opengl()
        .build()
        .unwrap());
    {
        let window = wm.window_mut(wid).unwrap();

        // window.set_size(1288,888);
        println!("size:{:?}", window.size());
        // window.set_position(200,200);
        println!("position:{:?}", window.position());
        // window.set_brightness(0.5).unwrap();
        println!("brightness:{}", window.brightness());
        // window.set_opacity(0.5).unwrap();
        println!("opacity:{:?}", window.opacity());
        // window.set_title("shit").unwrap();
        println!("title:{}", window.title());
    }
    println!("count:{}",wm.count());
    {
        println!("title:{}", wm.window(wid).unwrap().title());
    }
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

#[test]
fn builder_test(){
    use elikar::window;

    let _elikar = elikar::Elikar::new().unwrap();
    let mut wm = window::Manager::new();
    let mut window_cfg = elikar::window::Builder::new();
    window_cfg
        .title("asdsad")
        .position_default()
        .opengl();
    #[allow(unused)]
    let w1 = window_cfg.build().unwrap();
    window_cfg
        .position(10,10)
        .size(1024,768);
    let w2 = window_cfg.build().unwrap();
    #[allow(unused)]
    let wid1 = wm.add_windows(w1);
    let wid2 = wm.add_windows(w2);

    std::thread::sleep(std::time::Duration::from_secs(1));
}