extern crate elikar;

#[test]
fn power_test(){
    println!("Power state:{:?}",elikar::power::get_power_state());
    println!("Battery life time: {:?} second",elikar::power::get_battery_time());
    println!("Battery percentage:{:?} %",elikar::power::get_battery_percentage());

}