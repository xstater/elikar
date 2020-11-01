extern crate elikar;

use elikar::Elikar;

#[test]
fn sysinfo_test(){
    let game = Elikar::new().unwrap();
    let sysinfo = game.system_info();
    let videoinfo = sysinfo.get_video_info();

    println!("Platform:{}"                  ,sysinfo.get_platform());
    println!("CPU Cache Line Size:{} bytes" ,sysinfo.get_cpu_cache_line_size());
    println!("CPU count:{}"                 ,sysinfo.get_cpu_count());
    println!("System Ram:{} MiB"            ,sysinfo.get_system_ram());
    println!("Has 3DNow:{}"                 ,sysinfo.has_3d_now());
    println!("Has AVX:{}"                   ,sysinfo.has_avx());
    println!("Has AVX2:{}"                  ,sysinfo.has_avx2());
    println!("Has AltiVec:{}"               ,sysinfo.has_alti_vec());
    println!("Has MMX:{}"                   ,sysinfo.has_mmx());
    println!("Has RDTSC:{}"                 ,sysinfo.has_rdtsc());
    println!("Has SSE:{}"                   ,sysinfo.has_sse());
    println!("Has SSE2:{}"                  ,sysinfo.has_sse2());
    println!("Has SSE3:{}"                  ,sysinfo.has_sse3());
    println!("Has SSE41:{}"                 ,sysinfo.has_sse41());
    println!("Has SSE42:{}"                 ,sysinfo.has_sse42());

    println!("video drivers:{:?}",videoinfo.all_drivers_name().unwrap());
    println!("current video drivers:{}",videoinfo.current_drivers_name());
    for screen in videoinfo.screens().unwrap(){
        println!("{}",screen);
    }
}