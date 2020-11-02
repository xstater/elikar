pub struct Data(i64);

pub trait Byte{
    fn byte(self) -> Data;
}
pub trait Kibibyte{
    fn kb(self) -> Data;
}
pub trait Mebibyte{
    fn mb(self) -> Data;
}
pub trait Gibibyte{
    fn gb(self) -> Data;
}
pub trait Tebibyte{
    fn tb(self) -> Data;
}

macro_rules! impl_for_all_int {
    ($trait_name : ident, $fn_name : ident,$factor : expr) => {
        impl $trait_name for i8 { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for u8 { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for i16 { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for u16 { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for i32 { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for u32 { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for i64 { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for u64 { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for i128 { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for u128 { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for isize { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for usize { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for f32 { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
        impl $trait_name for f64 { fn $fn_name(self) -> Data { Data(self as i64 * $factor) } }
    };
}

impl_for_all_int!(Byte,byte,1);
impl_for_all_int!(Kibibyte,kb,1024);
impl_for_all_int!(Mebibyte,mb,1_048_576);
impl_for_all_int!(Gibibyte,gb,1_073_741_824);
impl_for_all_int!(Tebibyte,tb,1_099_511_627_776);

impl Data{
    pub fn as_byte(&self) -> i64{
        self.0
    }
    pub fn as_kb(&self) -> i64{
        self.0 / 1024
    }
    pub fn as_mb(&self) -> i64{
        self.0 / 1_048_576
    }
    pub fn as_gb(&self) -> i64{
        self.0 / 1_073_741_824
    }
    pub fn as_tb(&self) -> i64{
        self.0 / 1_099_511_627_776
    }

    pub fn as_byte_f32(&self) -> f32{
        self.0 as f32
    }
    pub fn as_kb_f32(&self) -> f32{
        self.0 as f32 / 1024.0
    }
    pub fn as_mb_f32(&self) -> f32{
        self.0 as f32 / 1_048_576.0
    }
    pub fn as_gb_f32(&self) -> f32{
        self.0 as f32 / 1_073_741_824.0
    }
    pub fn as_tb_f32(&self) -> f32{
        self.0 as f32 / 1_099_511_627_776.0
    }

    pub fn as_byte_f64(&self) -> f64{
        self.0 as f64
    }
    pub fn as_kb_f64(&self) -> f64{
        self.0 as f64 / 1024.0
    }
    pub fn as_mb_f64(&self) -> f64{
        self.0 as f64 / 1_048_576.0
    }
    pub fn as_gb_f64(&self) -> f64{
        self.0 as f64 / 1_073_741_824.0
    }
    pub fn as_tb_f64(&self) -> f64{
        self.0 as f64 / 1_099_511_627_776.0
    }
}
