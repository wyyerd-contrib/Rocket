#![feature(proc_macro)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[cfg(feature = "databases")]
mod databases_tests {
    #[database("foo")]
    struct MyStruct(usize);
}
