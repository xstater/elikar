extern crate elikar;

use elikar::Elikar;

fn main(){
    let game = Elikar::new().unwrap();
    let system_info = game.system_info();
    let platform = system_info.platform();
    let cpu = system_info.cpu();
    let video = system_info.video();
    println!("Platform:{}",platform.name());
    println!("System Ram:{}",platform.system_ram());

    println!("CPU Cache Line Size:{}" ,cpu.cpu_cache_line_size());
    println!("CPU count:{}"           ,cpu.cpu_count());
    println!("Has 3DNow:{}"           ,cpu.has_3d_now());
    println!("Has AVX:{}"             ,cpu.has_avx());
    println!("Has AVX2:{}"            ,cpu.has_avx2());
    println!("Has AltiVec:{}"         ,cpu.has_alti_vec());
    println!("Has MMX:{}"             ,cpu.has_mmx());
    println!("Has RDTSC:{}"           ,cpu.has_rdtsc());
    println!("Has SSE:{}"             ,cpu.has_sse());
    println!("Has SSE2:{}"            ,cpu.has_sse2());
    println!("Has SSE3:{}"            ,cpu.has_sse3());
    println!("Has SSE41:{}"           ,cpu.has_sse41());
    println!("Has SSE42:{}"           ,cpu.has_sse42());

    println!("video drivers:{:?}"       ,video.all_drivers_name().unwrap());
    println!("current video drivers:{}" ,video.current_drivers_name());
    for screen in video.screens().unwrap(){
        println!("{}",screen);
    }
}