extern crate elikar;

#[test]
fn msgbox_test(){
    use elikar::msgbox::*;

    let id = elikar::msgbox::MsgboxBuilder::new()
        .information()
        .title("nmsl test")
        .message("nmsl ?")
        .add_button(ButtonDefaultKey::Nope,0,"Yes")
        .add_button(ButtonDefaultKey::Return,1,"No")
        .add_button(ButtonDefaultKey::Escape,2,"Cancel")
        .build()
        .unwrap();

    match id {
        0 => {
            println!("nmsl")
        },
        1 | 2 => {
            println!("nmms")
        },
        _ => {
            println!("cnm")
        }
    };
}