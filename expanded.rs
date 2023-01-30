#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use clap::{Parser, Subcommand};
use wasmtime::{Engine, Instance, Linker, Module, Store};
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
