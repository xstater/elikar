use elikar::Elikar;

fn main() {
    let game = Elikar::new().unwrap();
    let world = game.elikar_world();
    let mut clipboard = world.clipboard_mut();

    println!("{}", clipboard.has());
    println!("{}", clipboard.get().unwrap());
    clipboard.set("s操你妈的s").unwrap();
    println!("{}", clipboard.get().unwrap());
    clipboard.set("sasdasdas").unwrap();
    println!("{}", clipboard.get().unwrap());
}
