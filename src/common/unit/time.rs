pub struct Time(i64);

pub trait Nanosecond{
    fn ns(self) -> Time;
}
pub trait Microsecond{
    fn us(self) -> Time;
}
pub trait Millisecond{
    fn ms(self) -> Time;
}
pub trait Second{
    fn s(self) -> Time;
}
pub trait Minute{
    fn min(self) -> Time;
}
pub trait Hour{
    fn hr(self) -> Time;
}

macro_rules! impl_for_all_int {
    ($trait_name : ident, $fn_name : ident,$factor : expr) => {
        impl $trait_name for i8 { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for u8 { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for i16 { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for u16 { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for i32 { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for u32 { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for i64 { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for u64 { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for i128 { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for u128 { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for isize { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for usize { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for f32 { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
        impl $trait_name for f64 { fn $fn_name(self) -> Time { Time(self as i64 * $factor) } }
    };
}

impl_for_all_int!(Nanosecond,ns,1);
impl_for_all_int!(Microsecond,us,1000);
impl_for_all_int!(Millisecond,ms,1000000);
impl_for_all_int!(Second,s,1000000000);
impl_for_all_int!(Minute,min,60000000000);
impl_for_all_int!(Hour,hr,3600000000000);

impl Time{
    pub fn as_ns(&self) -> i64{
        self.0
    }
    pub fn as_us(&self) -> i64{
        self.0 / 1000
    }
    pub fn as_ms(&self) -> i64{
        self.0 / 1000000
    }
    pub fn as_s(&self) -> i64{
        self.0 / 1000000000
    }
    pub fn as_min(&self) -> i64{
        self.0 / 60000000000
    }
    pub fn as_hrs(&self) -> i64{
        self.0 / 3600000000000
    }


    pub fn as_ns_f32(&self) -> f32{
        self.0 as f32
    }
    pub fn as_us_f32(&self) -> f32{
        self.0 as f32 / 1000.0
    }
    pub fn as_ms_f32(&self) -> f32{
        self.0 as f32 / 1000000.0
    }
    pub fn as_s_f32(&self) -> f32{
        self.0 as f32 / 1000000000.0
    }
    pub fn as_min_f32(&self) -> f32{
        self.0 as f32 / 60000000000.0
    }
    pub fn as_hrs_f32(&self) -> f32{
        self.0 as f32 / 3600000000000.0
    }


    pub fn as_ns_f64(&self) -> f64{
        self.0 as f64
    }
    pub fn as_us_f64(&self) -> f64{
        self.0 as f64 / 1000.0
    }
    pub fn as_ms_f64(&self) -> f64{
        self.0 as f64 / 1000000.0
    }
    pub fn as_s_f64(&self) -> f64{
        self.0 as f64 / 1000000000.0
    }
    pub fn as_min_f64(&self) -> f64{
        self.0 as f64 / 60000000000.0
    }
    pub fn as_hrs_f64(&self) -> f64{
        self.0 as f64 / 3600000000000.0
    }
}
