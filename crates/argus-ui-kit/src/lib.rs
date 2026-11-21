//! Windows-first visual component library for GPUI Kit.
//! All domain data, application routes and user actions belong to the host app.
//! Native DWM materials depend on the Windows GPUI backend.
#[cfg(not(target_os = "windows"))]
compile_error!("argus-ui-kit currently requires Windows; argus-ui-core is platform-independent.");

pub mod backdrop;
pub mod components;
pub mod design;
pub mod typography;
