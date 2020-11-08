extern crate elikar;

use elikar::Elikar;
use elikar::sysinfo::SystemInfo;

#[test]
fn sysinfo_test(){
    let game = Elikar::new().unwrap();
    let sysinfo = SystemInfo::new(&game);
    let platinfo = sysinfo.platform_info();
    let cpuinfo = sysinfo.cpu_info();
    let videoinfo = sysinfo.video_info();

    println!("Platform:{}"                  ,platinfo.name());
    println!("System Ram:{} MiB"            ,platinfo.system_ram().as_mb());

    println!("CPU Cache Line Size:{} bytes" ,cpuinfo.cpu_cache_line_size().as_byte());
    println!("CPU count:{}"                 ,cpuinfo.cpu_count());
    println!("Has 3DNow:{}"                 ,cpuinfo.has_3d_now());
    println!("Has AVX:{}"                   ,cpuinfo.has_avx());
    println!("Has AVX2:{}"                  ,cpuinfo.has_avx2());
    println!("Has AltiVec:{}"               ,cpuinfo.has_alti_vec());
    println!("Has MMX:{}"                   ,cpuinfo.has_mmx());
    println!("Has RDTSC:{}"                 ,cpuinfo.has_rdtsc());
    println!("Has SSE:{}"                   ,cpuinfo.has_sse());
    println!("Has SSE2:{}"                  ,cpuinfo.has_sse2());
    println!("Has SSE3:{}"                  ,cpuinfo.has_sse3());
    println!("Has SSE41:{}"                 ,cpuinfo.has_sse41());
    println!("Has SSE42:{}"                 ,cpuinfo.has_sse42());

    println!("video drivers:{:?}"           ,videoinfo.all_drivers_name().unwrap());
    println!("current video drivers:{}"     ,videoinfo.current_drivers_name());
    for screen in videoinfo.screens().unwrap(){
        println!("{}",screen);
    }
}