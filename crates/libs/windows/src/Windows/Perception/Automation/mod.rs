#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Perception_Automation_Core")]
pub mod Core;
#[cfg(feature = "implement")]
::core::include!("impl.rs");