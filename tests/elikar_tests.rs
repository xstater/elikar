extern crate elikar;

#[test]
fn elikar_test(){
    use elikar::elikar::Elikar;

    #[allow(unused_variables)]
    let ek = Elikar::new()
        .unwrap();

    #[allow(unused_variables)]
    let window = elikar::window::Window::new("Fuck Rust",100,100,1280,800)
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

}