#![warn(
    clippy::cognitive_complexity,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::float_equality_without_abs,
    keyword_idents,
    clippy::missing_const_for_fn,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::mod_module_files,
    non_ascii_idents,
    noop_method_call,
    clippy::option_if_let_else,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::semicolon_if_nothing_returned,
    clippy::unseparated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::similar_names,
    clippy::suspicious_operation_groupings,
    //unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    clippy::unused_self,
    clippy::use_debug,
    clippy::used_underscore_binding,
    clippy::useless_let_if_seq,
    clippy::wildcard_dependencies,
    clippy::wildcard_imports
)]

pub mod prom_core;
pub mod prom_plugins;
pub mod prom_utilities;

pub mod prelude {
    pub use super::{
        prom_core::{
            scheduler::{
                Scheduler,
                injection_types::{
                    event::{
                        Event, queue::EventQueue, reader::EventReader, writer::EventWriter
                    }, 
                    resource::{
                        mut_referenced::ResMut, referenced::Res
                    }, 
                    world::{
                        command_queue::{WriteWorld, Command}, world::ReadWorld
                    }, 
                }
            },
            unity::plugin::Plugin,
            app::{
                App, 
                app_builder::AppBuilder
            },
        },
        prom_plugins::prom_plugins,
        prom_utilities::resources,
    };
    
    pub use event_driver::EventDriver as Event;
    //pub use hecs::Entity;
    //pub use cgmath::Matrix4;
    pub fn concat_title(lhs: &'static str) -> impl Fn(&str) -> String {
        move |rhs: &str| format!("{}: {}", lhs, rhs)
    }

    pub type SupportedFloat = f32;
    pub type Padding = u32;
}
