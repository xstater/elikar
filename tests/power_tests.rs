extern crate elikar;

#[test]
fn power_test(){
    println!("Power state:{:?}",elikar::power::power_state());
    println!("Battery life time: {:?} second",elikar::power::battery_time());
    println!("Battery percentage:{:?} %",elikar::power::battery_percentage());
}