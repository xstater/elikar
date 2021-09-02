extern crate elikar;

use elikar::Elikar;

fn main(){
    let game = Elikar::new().unwrap();
    let mut clipboard = game.clipboard();

    println!("{}",clipboard.has());
    println!("{}",clipboard.get().unwrap());
    clipboard.set("s操你妈的s").unwrap();
    println!("{}",clipboard.get().unwrap());
    clipboard.set("sasdasdas").unwrap();
    println!("{}",clipboard.get().unwrap());
}