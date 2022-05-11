extern crate alloc;
use alloc::{
    fmt::Debug,
    str::FromStr,
    string::{String, ToString},
    vec::Vec,
};

pub fn typecast_to_string<T>(list: Vec<T>) -> Vec<String>
where
    T: ToString,
{
    let mut ret: Vec<String> = Vec::new();
    for item in list.iter() {
        ret.push(item.to_string());
    }
    ret
}

pub fn typecast_from_string<T: FromStr + Debug>(list: Vec<String>) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut ret: Vec<T> = Vec::new();
    for item in list.iter() {
        ret.push(T::from_str(item).unwrap());
    }
    ret
}
