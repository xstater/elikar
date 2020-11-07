extern crate elikar;

#[test]
fn elikar_test(){
    let mut game = elikar::Elikar::new().unwrap();
    let wm = game.windows_manager_mut();
    let _window = wm.window_builder()
        .title("elikar test")
        .opengl()
        .build_mut()
        .unwrap();

    game.run();
}