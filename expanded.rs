#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use clap::{Parser, Subcommand};
use wasmtime::{Config, Engine, Instance, Linker, Module, Store};
#[allow(clippy::all)]
pub mod host {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    pub trait Host: Sized {
        fn test(&mut self) -> anyhow::Result<()>;
    }
    pub fn add_to_linker<T, U>(
        linker: &mut wasmtime::component::Linker<T>,
        get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()>
    where
        U: Host,
    {
        let mut inst = linker.instance("host")?;
        inst.func_wrap(
            "test",
            move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                let host = get(caller.data_mut());
                let r = host.test();
                r
            },
        )?;
        Ok(())
    }
}
pub struct Task {
    dispose: wasmtime::component::Func,
    init: wasmtime::component::Func,
    run: wasmtime::component::Func,
}
const _: () = {
    use wasmtime::component::__internal::anyhow;
    impl Task {
        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<T>,
        ) -> anyhow::Result<(Self, wasmtime::component::Instance)> {
            let instance = linker.instantiate(&mut store, component)?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// defined within `store` and wrap them all up in the
        /// returned structure which can be used to interact with
        /// the wasm module.
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> anyhow::Result<Self> {
            let mut store = store.as_context_mut();
            let mut exports = instance.exports(&mut store);
            let mut __exports = exports.root();
            let dispose = *__exports.typed_func::<(), (i32,)>("dispose")?.func();
            let init = *__exports.typed_func::<(), (i32,)>("init")?.func();
            let run = *__exports.typed_func::<(), (i32,)>("run")?.func();
            Ok(Task { dispose, init, run })
        }
        pub fn init<S: wasmtime::AsContextMut>(
            &self,
            mut store: S,
        ) -> anyhow::Result<i32> {
            let callee = unsafe {
                wasmtime::component::TypedFunc::<(), (i32,)>::new_unchecked(self.init)
            };
            let (ret0,) = callee.call(store.as_context_mut(), ())?;
            callee.post_return(store.as_context_mut())?;
            Ok(ret0)
        }
        pub fn run<S: wasmtime::AsContextMut>(
            &self,
            mut store: S,
        ) -> anyhow::Result<i32> {
            let callee = unsafe {
                wasmtime::component::TypedFunc::<(), (i32,)>::new_unchecked(self.run)
            };
            let (ret0,) = callee.call(store.as_context_mut(), ())?;
            callee.post_return(store.as_context_mut())?;
            Ok(ret0)
        }
        pub fn dispose<S: wasmtime::AsContextMut>(
            &self,
            mut store: S,
        ) -> anyhow::Result<i32> {
            let callee = unsafe {
                wasmtime::component::TypedFunc::<(), (i32,)>::new_unchecked(self.dispose)
            };
            let (ret0,) = callee.call(store.as_context_mut(), ())?;
            callee.post_return(store.as_context_mut())?;
            Ok(ret0)
        }
    }
};
const _: &str = "interface host-funcs {\n  test: func() \n}\n\nworld task {\n  import host: host-funcs \n\n  default export interface {\n    init: func() -> s32\n    run: func() -> s32\n    dispose: func() -> s32\n  }\n}\n";
#[command(
    author = "Jesse Wellenberg",
    version = "0.0.1",
    about = "CLI app for Alice",
    long_about = None
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
impl clap::Parser for Cli {}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::CommandFactory for Cli {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("alice");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("alice");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::FromArgMatches for Cli {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = Cli {
            command: {
                <Commands as clap::FromArgMatches>::from_arg_matches_mut(
                    __clap_arg_matches,
                )?
            },
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        {
            #[allow(non_snake_case)]
            let command = &mut self.command;
            <Commands as clap::FromArgMatches>::update_from_arg_matches_mut(
                command,
                __clap_arg_matches,
            )?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::Args for Cli {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("Cli"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("Cli")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                );
            let __clap_app = <Commands as clap::Subcommand>::augment_subcommands(
                __clap_app,
            );
            let __clap_app = __clap_app
                .subcommand_required(true)
                .arg_required_else_help(true);
            __clap_app
                .author("Jesse Wellenberg")
                .version("0.0.1")
                .about("CLI app for Alice")
                .long_about(None)
                .propagate_version(true)
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("Cli")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                );
            let __clap_app = <Commands as clap::Subcommand>::augment_subcommands(
                __clap_app,
            );
            let __clap_app = __clap_app
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand_required(false)
                .arg_required_else_help(false);
            __clap_app
                .author("Jesse Wellenberg")
                .version("0.0.1")
                .about("CLI app for Alice")
                .long_about(None)
                .propagate_version(true)
        }
    }
}
struct HostState;
enum Commands {
    Add { name: Option<String> },
    Run { id: i32 },
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::FromArgMatches for Commands {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        if let Some((__clap_name, mut __clap_arg_sub_matches))
            = __clap_arg_matches.remove_subcommand()
        {
            let __clap_arg_matches = &mut __clap_arg_sub_matches;
            if __clap_name == "add" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::Add {
                    name: __clap_arg_matches.remove_one::<String>("name"),
                });
            }
            if __clap_name == "run" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::Run {
                    id: __clap_arg_matches
                        .remove_one::<i32>("id")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            {
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["The following required argument was not provided: "],
                                        &[::core::fmt::ArgumentV1::new_display(&"id")],
                                    ),
                                );
                                res
                            },
                        ))?,
                });
            }
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["The subcommand \'", "\' wasn\'t recognized"],
                                &[::core::fmt::ArgumentV1::new_display(&__clap_name)],
                            ),
                        );
                        res
                    },
                ),
            )
        } else {
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::MissingSubcommand,
                    "A subcommand is required but one was not provided.",
                ),
            )
        }
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut<'b>(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
            match self {
                Self::Add { name } if "add" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        if __clap_arg_matches.contains_id("name") {
                            *name = __clap_arg_matches.remove_one::<String>("name");
                        }
                    }
                }
                Self::Run { id } if "run" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        if __clap_arg_matches.contains_id("id") {
                            *id = __clap_arg_matches
                                .remove_one::<i32>("id")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    {
                                        let res = ::alloc::fmt::format(
                                            ::core::fmt::Arguments::new_v1(
                                                &["The following required argument was not provided: "],
                                                &[::core::fmt::ArgumentV1::new_display(&"id")],
                                            ),
                                        );
                                        res
                                    },
                                ))?;
                        }
                    }
                }
                s => {
                    *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                        __clap_arg_matches,
                    )?;
                }
            }
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::Subcommand for Commands {
    fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("add");
                {
                    let __clap_subcommand = __clap_subcommand
                        .group(
                            clap::ArgGroup::new("Add")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 1usize] = [clap::Id::from("name")];
                                    members
                                }),
                        );
                    let __clap_subcommand = __clap_subcommand
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("name")
                                .value_name("NAME")
                                .value_parser({
                                    use ::clap::builder::via_prelude::*;
                                    let auto = ::clap::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg;
                            let arg = arg;
                            arg
                        });
                    __clap_subcommand
                }
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("run");
                {
                    let __clap_subcommand = __clap_subcommand
                        .group(
                            clap::ArgGroup::new("Run")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 1usize] = [clap::Id::from("id")];
                                    members
                                }),
                        );
                    let __clap_subcommand = __clap_subcommand
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("id")
                                .value_name("ID")
                                .required(true && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap::builder::via_prelude::*;
                                    let auto = ::clap::builder::_AutoValueParser::<i32>::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg;
                            let arg = arg;
                            arg
                        });
                    __clap_subcommand
                }
            });
        __clap_app
    }
    fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("add");
                {
                    let __clap_subcommand = __clap_subcommand
                        .group(
                            clap::ArgGroup::new("Add")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 1usize] = [clap::Id::from("name")];
                                    members
                                }),
                        );
                    let __clap_subcommand = __clap_subcommand
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("name")
                                .value_name("NAME")
                                .value_parser({
                                    use ::clap::builder::via_prelude::*;
                                    let auto = ::clap::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg;
                            let arg = arg.required(false);
                            arg
                        });
                    __clap_subcommand
                }
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("run");
                {
                    let __clap_subcommand = __clap_subcommand
                        .group(
                            clap::ArgGroup::new("Run")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 1usize] = [clap::Id::from("id")];
                                    members
                                }),
                        );
                    let __clap_subcommand = __clap_subcommand
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("id")
                                .value_name("ID")
                                .required(true && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap::builder::via_prelude::*;
                                    let auto = ::clap::builder::_AutoValueParser::<i32>::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg;
                            let arg = arg.required(false);
                            arg
                        });
                    __clap_subcommand
                }
            });
        __clap_app
    }
    fn has_subcommand(__clap_name: &str) -> bool {
        if "add" == __clap_name {
            return true;
        }
        if "run" == __clap_name {
            return true;
        }
        false
    }
}
fn main() {
    let cli = Cli::parse();
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    let mut linker: Linker<HostState> = Linker::new(&engine);
    host::add_to_linker(
        &mut linker,
        |&mut ctx| {
            ::std::io::_print(
                ::core::fmt::Arguments::new_v1(&["Hello from host\n"], &[]),
            );
        },
    );
    match &cli.command {
        Commands::Add { name } => {
            ::std::io::_print(
                ::core::fmt::Arguments::new_v1(
                    &["Add called with \'", "\'\n"],
                    &[::core::fmt::ArgumentV1::new_debug(&name)],
                ),
            );
        }
        Commands::Run { id } => {
            ::std::io::_print(
                ::core::fmt::Arguments::new_v1(
                    &["Run called with id \'", "\'\n"],
                    &[::core::fmt::ArgumentV1::new_display(&id)],
                ),
            );
        }
    }
}
