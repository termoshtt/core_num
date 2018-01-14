#![no_std]
#![feature(core_intrinsics)]

macro_rules! intrinsics {
    ($([$method:ident; $intr_f32:ident, $intr_f64:ident]),*) => {

    pub trait FloatIntrinsics {$(
        fn $method(self) -> Self;
    )*}

    impl FloatIntrinsics for f32 {$(
        fn $method(self) -> Self {
            unsafe { core::intrinsics::$intr_f32(self) }
        }
    )*}

    impl FloatIntrinsics for f64 {$(
        fn $method(self) -> Self {
            unsafe { core::intrinsics::$intr_f64(self) }
        }
    )*}
}}

intrinsics!(
    [cos; cosf32, cosf64],
    [sin; sinf32, sinf64],
    [abs; fabsf32, fabsf64],
    [exp; expf32, expf64],
    [exp2; exp2f32, exp2f64],
    [log; logf32, logf64],
    [log2; log2f32, log2f64],
    [log10; log10f32, log10f64]
);
