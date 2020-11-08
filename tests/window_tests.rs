extern crate elikar;

use elikar::window::{WindowsManager, WindowBuilder};

#[test]
fn window_test(){
    use elikar::elikar::Elikar;

    let ek = Elikar::new().unwrap();

    let mut wm = WindowsManager::new(&ek);
    let wid  = wm.add_windows(WindowBuilder::new()
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
