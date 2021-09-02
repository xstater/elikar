extern crate elikar;

fn main(){
    println!("Power state:{:?}",elikar::power::power_state());
    println!("Battery life time: {:?} second",elikar::power::battery_time());
    println!("Battery percentage:{:?} %",elikar::power::battery_percentage());
}