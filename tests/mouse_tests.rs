extern crate elikar;

use elikar::window::WindowsManager;

#[test]
fn mouse_test(){
    let game = elikar::elikar::Elikar::new().unwrap();
    let mut wm = WindowsManager::new(&game);
    let window = wm.window_builder()
        .title("mouse_test")
        .size(1280,900)
        .opengl()
        .build_mut()
        .unwrap();

}