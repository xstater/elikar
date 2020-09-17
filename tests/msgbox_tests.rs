extern crate elikar;

#[test]
fn msgbox_test(){
    use elikar::msgbox::*;

    let id = elikar::msgbox::MsgboxBuilder::new()
        .information()
        .title("死妈测试")
        .message("你妈死了 ?")
        .add_button(ButtonDefaultKey::Return,0,"Yes")
        .add_button(ButtonDefaultKey::Nope,1,"No")
        .add_button(ButtonDefaultKey::Escape,2,"Cancel")
        .build()
        .unwrap();

    match id {
        0 => {
            alert("死妈测试","你妈死了")
        },
        1 | 2 => {
            alert("死妈测试","你妈没死")
        },
        _ => {
            println!("cnm")
        }
    };
}