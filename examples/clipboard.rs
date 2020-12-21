extern crate elikar;

fn main(){
    use elikar::Elikar;
    use elikar::clipboard;

    let _game = Elikar::new().unwrap();

    println!("{}",clipboard::has());
    println!("{}",clipboard::get().unwrap());
    clipboard::set("s操你妈的s").unwrap();
    println!("{}",clipboard::get().unwrap());
    clipboard::set("sasdasdas").unwrap();
    println!("{}",clipboard::get().unwrap());
}