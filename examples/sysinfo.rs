extern crate elikar;

use elikar::Elikar;
use elikar::sysinfo;

fn main(){
    let _game = Elikar::new().unwrap();
    println!("Platform:{}"                  ,sysinfo::platform_info::name());
    println!("System Ram:{}"            ,sysinfo::platform_info::system_ram());

    println!("CPU Cache Line Size:{}" ,sysinfo::cpu_info::cpu_cache_line_size());
    println!("CPU count:{}"                 ,sysinfo::cpu_info::cpu_count());
    println!("Has 3DNow:{}"                 ,sysinfo::cpu_info::has_3d_now());
    println!("Has AVX:{}"                   ,sysinfo::cpu_info::has_avx());
    println!("Has AVX2:{}"                  ,sysinfo::cpu_info::has_avx2());
    println!("Has AltiVec:{}"               ,sysinfo::cpu_info::has_alti_vec());
    println!("Has MMX:{}"                   ,sysinfo::cpu_info::has_mmx());
    println!("Has RDTSC:{}"                 ,sysinfo::cpu_info::has_rdtsc());
    println!("Has SSE:{}"                   ,sysinfo::cpu_info::has_sse());
    println!("Has SSE2:{}"                  ,sysinfo::cpu_info::has_sse2());
    println!("Has SSE3:{}"                  ,sysinfo::cpu_info::has_sse3());
    println!("Has SSE41:{}"                 ,sysinfo::cpu_info::has_sse41());
    println!("Has SSE42:{}"                 ,sysinfo::cpu_info::has_sse42());

    println!("video drivers:{:?}"           ,sysinfo::video_info::all_drivers_name().unwrap());
    println!("current video drivers:{}"     ,sysinfo::video_info::current_drivers_name());
    for screen in sysinfo::video_info::screens().unwrap(){
        println!("{}",screen);
    }
}