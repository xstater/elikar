extern crate elikar;

#[test]
fn test(){
    use elikar::Elikar;

    #[allow(unused)]
    let game = Elikar::new().unwrap();
    #[allow(unused)]
    let clipboard = game.clipboard();

    println!("{}",clipboard.has());
    println!("{}",clipboard.get().unwrap());
    clipboard.set("s操你妈的s").unwrap();
    println!("{}",clipboard.get().unwrap());
    clipboard.set("sasdasdas").unwrap();
    println!("{}",clipboard.get().unwrap());
}