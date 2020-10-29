extern crate elikar;

#[test]
fn test(){
    use elikar::Elikar;

    #[allow(unused)]
    let mut game = Elikar::new().unwrap();
    #[allow(unused)]
    let clipboard = game.clipboard_mut();

    println!("{}",clipboard.has());
    println!("{}",clipboard.get().unwrap());
    clipboard.set("s操你妈的s").unwrap();
    println!("{}",clipboard.get().unwrap());
    clipboard.set("sasdasdas").unwrap();
    println!("{}",clipboard.get().unwrap());
}