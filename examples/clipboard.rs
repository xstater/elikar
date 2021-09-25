extern crate elikar;

use elikar::Elikar;
use elikar::clipboard::Clipboard;

fn main(){
    let game = Elikar::new().unwrap();
    let mut clipboard = game
        .current_stage_ref()
        .system_data_mut::<Clipboard>();

    println!("{}",clipboard.has());
    println!("{}",clipboard.get().unwrap());
    clipboard.set("s操你妈的s").unwrap();
    println!("{}",clipboard.get().unwrap());
    clipboard.set("sasdasdas").unwrap();
    println!("{}",clipboard.get().unwrap());
}