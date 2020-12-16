extern crate elikar;

fn main(){
    use elikar::msgbox::*;

    elikar::msgbox::MsgboxBuilder::information()
        .title("死妈测试")
        .message("你妈死了 ?")
        .add_button(
            ButtonDefaultKey::Return,
            "Yes",
            ||alert("死妈测试","你妈死了"))
        .add_button(
            ButtonDefaultKey::Nope,
            "No",
            ||alert("死妈测试","你妈没死"))
        .add_button(
            ButtonDefaultKey::Escape,
            "Cancel",
            ||alert("死妈测试","你妈没死"))
        .build()
        .unwrap();
}