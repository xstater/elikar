extern crate elikar;

use elikar::window::{Manager,Builder};

#[test]
fn window_test(){
    use elikar::elikar::Elikar;

    let ek = Elikar::new().unwrap();

    let mut wm = Manager::new(&ek);
    let wid  = wm.add_windows(Builder::new()
        .title("测试窗口")
        .position_centred()
        .size(1280, 700)
        .maximized()
        .opengl()
        .build(&ek)
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
    let elikar = elikar::Elikar::new().unwrap();
    let mut wm = elikar::window::Manager::new(&elikar);
    let mut window_cfg = elikar::window::Builder::new();
    window_cfg
        .title("asdsad")
        .position_default()
        .opengl();
    let w1 = window_cfg.build(&elikar).unwrap();
    window_cfg
        .position_centred()
        .size(1024,768);
    let w2 = window_cfg.build(&elikar).unwrap();
    let wid1 = wm.add_windows(w1);
    let wid2 = wm.add_windows(w2);
    wm.window_mut(wid2).unwrap()
        .hide();

    std::thread::sleep(std::time::Duration::from_secs(1));
}