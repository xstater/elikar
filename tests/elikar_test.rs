extern crate elikar;

#[test]
fn elikar_test(){
    use elikar::elikar::Elikar;

    #[allow(unused_variables)]
    let ek = Elikar::new("elikar_test",(0,1,0))
        .unwrap();

    println!("{}",ek.name());
    println!("{:?}",ek.version());
}