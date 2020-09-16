extern crate elikar;

#[test]
fn sysinfo_test(){
    #[allow(unused_variables)]
    let game = elikar::Elikar::new().unwrap();

    println!("CPU Cache Line:{}",elikar::sysinfo::get_cpu_cache_line_size());
    println!("CPU count:{}"     ,elikar::sysinfo::get_cpu_count());
    println!("System Ram:{}"    ,elikar::sysinfo::get_system_ram());
    println!("Has 3DNow:{}"     ,elikar::sysinfo::has_3d_now());
    println!("Has AVX:{}"       ,elikar::sysinfo::has_avx());
    println!("Has AVX2:{}"      ,elikar::sysinfo::has_avx2());
    println!("Has AltiVec:{}"   ,elikar::sysinfo::has_alti_vec());
    println!("Has MMX:{}"       ,elikar::sysinfo::has_mmx());
    println!("Has RDTSC:{}"     ,elikar::sysinfo::has_rdtsc());
    println!("Has SSE:{}"       ,elikar::sysinfo::has_sse());
    println!("Has SSE2:{}"      ,elikar::sysinfo::has_sse2());
    println!("Has SSE3:{}"      ,elikar::sysinfo::has_sse3());
    println!("Has SSE41:{}"     ,elikar::sysinfo::has_sse41());
    println!("Has SSE42:{}"     ,elikar::sysinfo::has_sse42());
}