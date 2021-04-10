/// possible compiler hint that a branch is likely
#[cfg(feature = "hints")]
#[macro_export]
macro_rules! likely {
    ($e:expr) => {
        std::intrinsics::likely($e)
    };
}

/// possible compiler hint that a branch is unlikely
#[cfg(feature = "hints")]
#[macro_export]
macro_rules! unlikely {
    ($e:expr) => {{
        std::intrinsics::unlikely($e)
    }};
}

/// possible compiler hint that a branch is likely
#[cfg(not(feature = "hints"))]
#[macro_export]
macro_rules! likely {
    ($e:expr) => {
        $e
    };
}

/// possible compiler hint that a branch is unlikely
#[cfg(not(feature = "hints"))]
#[macro_export]
macro_rules! unlikely {
    ($e:expr) => {
        $e
    };
}

/// static cast to an i8
#[macro_export]
macro_rules! static_cast_i8 {
    ($v:expr) => {
        mem::transmute::<_, i8>($v)
    };
}

/// static cast to an i32
#[macro_export]
macro_rules! static_cast_i32 {
    ($v:expr) => {
        mem::transmute::<_, i32>($v)
    };
}

/// static cast to an u32
#[macro_export]
macro_rules! static_cast_u32 {
    ($v:expr) => {
        mem::transmute::<_, u32>($v)
    };
}

/// static cast to an i64
#[macro_export]
macro_rules! static_cast_i64 {
    ($v:expr) => {
        mem::transmute::<_, i64>($v)
    };
}

/// static cast to an i64
#[macro_export]
macro_rules! static_cast_i128 {
    ($v:expr) => {
        mem::transmute::<_, i128>($v)
    };
}

/// static cast to an u64
#[macro_export]
macro_rules! static_cast_u64 {
    ($v:expr) => {
        mem::transmute::<_, u64>($v)
    };
}
