use extendr_api::prelude::*;
use extendr_api::Integers;

/// Return string `"Hello from Rust!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello from Rust!"
}


#[extendr]
fn nchar2(x: Strings) -> Integers {
    x
        .into_iter()
        .map(|xi| {
            let nchar: usize = xi.len();
            Rint::from(nchar as i32)
        })
        .collect::<Integers>()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustInRExample;
    fn hello_world;
    fn nchar2;
}
