#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use clap::{Parser, Subcommand};
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, define_wasi, WasiCtxBuilder};
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
#[allow(clippy::all)]
pub mod task {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    pub struct Task {
        dispose: wasmtime::component::Func,
        init: wasmtime::component::Func,
        run: wasmtime::component::Func,
    }
    impl Task {
        pub fn new(
            __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
        ) -> anyhow::Result<Task> {
            let dispose = *__exports.typed_func::<(), (i32,)>("dispose")?.func();
            let init = *__exports.typed_func::<(), (i32,)>("init")?.func();
            let run = *__exports.typed_func::<(), (i32,)>("run")?.func();
            Ok(Task { dispose, init, run })
        }
        pub fn call_dispose<S: wasmtime::AsContextMut>(
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
        pub fn call_init<S: wasmtime::AsContextMut>(
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
        pub fn call_run<S: wasmtime::AsContextMut>(
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
    }
}
pub struct TaskWorld {
    task: task::Task,
}
const _: () = {
    use wasmtime::component::__internal::anyhow;
    impl TaskWorld {
        pub fn add_to_linker<T, U>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> anyhow::Result<()>
        where
            U: host::Host,
        {
            host::add_to_linker(linker, get)?;
            Ok(())
        }
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
        /// Instantiates a pre-instantiated module using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate_pre<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance_pre: &wasmtime::component::InstancePre<T>,
        ) -> anyhow::Result<(Self, wasmtime::component::Instance)> {
            let instance = instance_pre.instantiate(&mut store)?;
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
            let task = task::Task::new(
                &mut __exports
                    .instance("task")
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            ::core::fmt::Arguments::new_v1(
                                &["exported instance `task` not present"],
                                &[],
                            ),
                        );
                        error
                    }))?,
            )?;
            Ok(TaskWorld { task })
        }
        pub fn task(&self) -> &task::Task {
            &self.task
        }
    }
};
const _: &str = "interface host-funcs {\r\n  test: func() \r\n}\r\n\r\ndefault world task-world {\r\n  export task: interface {\r\n    dispose: func() -> s32\r\n    init: func() -> s32\r\n    run: func() -> s32\r\n  }\r\n\r\n  import host: self.host-funcs \r\n}\r\n";
use host::*;
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
#[automatically_derived]
impl ::core::clone::Clone for HostState {
    #[inline]
    fn clone(&self) -> HostState {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for HostState {}
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
impl Host for WasiCtx {
    fn test(&mut self) -> anyhow::Result<()> {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["Howdyhay\n"], &[]));
        };
        Ok(())
    }
}
fn main() -> anyhow::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut component_linker: Linker<WasiCtx> = Linker::new(&engine);
    let mut linker: wasmtime::Linker<WasiCtx> = wasmtime::Linker::new(&engine);
    let ctx = WasiCtxBuilder::new()
        .inherit_env()?
        .inherit_stdout()
        .inherit_stderr()
        .inherit_stdio()
        .build();
    host::add_to_linker(&mut component_linker, |ctx| ctx)?;
    wasmtime_wasi::add_to_linker(&mut linker, |ctx| ctx);
    let component = Component::from_file(
        &engine,
        "./tasks/compiled/demotask/demotask.component.wasm",
    )?;
    let mut store = Store::new(&engine, ctx);
    let t = TaskWorld::instantiate(&mut store, &component, &linker)?.0;
    let init = t.task.call_init(&mut store)?;
    let run = t.task.call_init(&mut store)?;
    let dispose = t.task.call_init(&mut store)?;
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["Result of init was ", "\n"],
                &[::core::fmt::ArgumentV1::new_display(&init)],
            ),
        );
    };
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["Result of run was ", "\n"],
                &[::core::fmt::ArgumentV1::new_display(&run)],
            ),
        );
    };
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["Result of dispose was ", "\n"],
                &[::core::fmt::ArgumentV1::new_display(&dispose)],
            ),
        );
    };
    Ok(())
}
