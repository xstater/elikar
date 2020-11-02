extern crate elikar;

use elikar::common::unit::Second;

#[test]
fn power_test(){
    println!("Power state:{:?}",elikar::power::power_state());
    println!("Battery life time: {:?} second",elikar::power::battery_time().unwrap_or(0_i32.s()).as_s());
    println!("Battery percentage:{:?} %",elikar::power::battery_percentage());
}