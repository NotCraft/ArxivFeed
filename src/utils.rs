#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

#[macro_export]
macro_rules! crate_homepage {
    () => {
        env!("CARGO_PKG_HOMEPAGE")
    };
}

#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}


