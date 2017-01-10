pub trait CastInt<T> {
    fn cast_int(self) -> T;
}

macro_rules! impl_using_as {
    ( $a:ty as $b:ty ) => {
        impl CastInt<$b> for $a {
            #[inline]
            fn cast_int(self) -> $b {
                self as $b
            }
        }
    };
    ( $a:ty as $( $b:ty )+ ) => {
        $(impl_using_as!($a as $b);)+
    };
}

impl_using_as!(u8 as u8 u16 u32 u64 i8 i16 i32 i64 char usize isize);
impl_using_as!(u16 as u8 u16 u32 u64 i8 i16 i32 i64 usize isize);
impl_using_as!(u32 as u8 u16 u32 u64 i8 i16 i32 i64 usize isize);
impl_using_as!(u64 as u8 u16 u32 u64 i8 i16 i32 i64 usize isize);
impl_using_as!(i8 as u8 u16 u32 u64 i8 i16 i32 i64 usize isize);
impl_using_as!(i16 as u8 u16 u32 u64 i8 i16 i32 i64 usize isize);
impl_using_as!(i32 as u8 u16 u32 u64 i8 i16 i32 i64 usize isize);
impl_using_as!(i64 as u8 u16 u32 u64 i8 i16 i32 i64 usize isize);
impl_using_as!(char as u8 u16 u32 u64 i8 i16 i32 i64 char usize isize);
impl_using_as!(bool as u8 u16 u32 u64 i8 i16 i32 i64 bool usize isize);
impl_using_as!(usize as u8 u16 u32 u64 i8 i16 i32 i64 usize isize);
impl_using_as!(isize as u8 u16 u32 u64 i8 i16 i32 i64 usize isize);