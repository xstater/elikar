extern crate elikar;

#[test]
fn mouse_test(){
    let mut game = elikar::elikar::Elikar::new().unwrap();
    let wm = game.windows_manager_mut();
    let window = wm.window_builder()
        .title("mouse_test")
        .size(1280,900)
        .opengl()
        .build_mut()
        .unwrap();
    let mouse = game.mouse();

    let mut count : i32 = 0;
    loop {
        let btn = mouse.button();
        println!("left:{} right:{} middle:{} x1:{} x2:{}",
            btn.is_left(),
            btn.is_right(),
            btn.is_middle(),
            btn.is_x1(),
            btn.is_x2());
        count += 1;
        if count > 1000000 { break; }
    }
}