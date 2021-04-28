/// possible compiler hint that a branch is likely
#[cfg(feature = "hints")]
macro_rules! likely {
    ($e:expr) => {
        std::intrinsics::likely($e)
    };
}

/// possible compiler hint that a branch is likely
#[cfg(not(feature = "hints"))]
macro_rules! likely {
    ($e:expr) => {
        $e
    };
}

/// possible compiler hint that a branch is unlikely
#[cfg(feature = "hints")]
macro_rules! unlikely {
    ($e:expr) => {
        std::intrinsics::unlikely($e)
    };
}

/// possible compiler hint that a branch is unlikely
#[cfg(not(feature = "hints"))]
macro_rules! unlikely {
    ($e:expr) => {
        $e
    };
}

/// static cast to an i8
macro_rules! static_cast_i8 {
    ($v:expr) => {
        $v as i8
    };
}
