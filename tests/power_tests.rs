extern crate elikar;

#[test]
fn power_test(){
    #[allow(unused_variables)]
    let game = elikar::Elikar::new().unwrap();

    println!("Power state:{:?}",elikar::power::get_power_state());
    println!("Battery life time: {:?} second",elikar::power::get_battery_time());
    println!("Battery percentage:{:?} %",elikar::power::get_battery_percentage());

}