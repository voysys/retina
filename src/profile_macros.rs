#[cfg(feature = "tracy")]
#[macro_export]
macro_rules! profile_zone {
    ($name:expr) => {
        let _tracy_span = tracy_client::span!($name, 0);
    };
}

#[cfg(not(feature = "tracy"))]
#[macro_export]
macro_rules! profile_zone {
    ($name:expr) => {};
}

#[cfg(feature = "tracy")]
#[macro_export]
macro_rules! profile_plot {
    ($name:expr, $var_name:ident, $val:expr) => {{
        let $var_name: tracy_client::Plot = tracy_client::create_plot!($name);
        $var_name.point($val as f64);
    }};
}

#[cfg(not(feature = "tracy"))]
#[macro_export]
macro_rules! profile_plot {
    ($name:expr, $var_name:ident, $val:expr) => {};
}

#[cfg(feature = "tracy")]
#[macro_export]
macro_rules! set_thread_name {
    ($name:expr) => {
        tracy_client::set_thread_name($name);
    };
}

#[cfg(not(feature = "tracy"))]
#[macro_export]
macro_rules! set_thread_name {
    ($name:expr) => {};
}

#[cfg(feature = "tracy")]
#[macro_export]
macro_rules! mark_frame {
    () => {
        tracy_client::finish_continuous_frame!();
    };
}

#[cfg(not(feature = "tracy"))]
#[macro_export]
macro_rules! mark_frame {
    () => {};
}
