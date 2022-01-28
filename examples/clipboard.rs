use elikar::clipboard::Clipboard;
use elikar::Elikar;

fn main() {
    let game = Elikar::new().unwrap();
    let world = game.world();
    let world = world.read().unwrap();
    let mut clipboard = world
        .resource_mut::<Clipboard>().unwrap();

    println!("{}", clipboard.has());
    println!("{}", clipboard.get().unwrap());
    clipboard.set("s操你妈的s").unwrap();
    println!("{}", clipboard.get().unwrap());
    clipboard.set("sasdasdas").unwrap();
    println!("{}", clipboard.get().unwrap());
}
