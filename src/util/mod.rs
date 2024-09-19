pub mod delay;
pub mod animation;

macro_rules! iterable_enum {
    ($visibility:vis, $name:ident, $($member:tt),*) => {
        $visibility enum $name {$($member),*}
        impl $name {
            fn iterate() -> Vec<$name> {
                vec![$($name::$member,)*]
            }
        }
    };
    ($name:ident, $($member:tt),*) => {
        iterable_enum!(, $name, $($member),*)
    };
}

pub fn overflow_add(a: f32, b: f32, max: f32, min: f32) -> f32 {
    let mut result = a + b;
    if result > max {
        result = min + (result - max);
    }
    if result < min {
        result = max - (min - result);
    }
    return result;
}
