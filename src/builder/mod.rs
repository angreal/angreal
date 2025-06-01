//! The angreal app builder
//!
pub mod command_tree;

use crate::task::{AngrealArg, ANGREAL_ARGS, ANGREAL_TASKS};
use clap::{App, AppSettings, Arg, ArgAction, Command};

use command_tree::CommandNode;

/// Get the args for a given command.
pub fn select_args(name: &str) -> Vec<AngrealArg> {
    ANGREAL_ARGS
        .lock()
        .unwrap()
        .iter()
        .filter(|a| a.command_name == name)
        .cloned()
        .collect()
}

fn base_app_setup() -> App<'static> {
    Command::new("angreal")
        .setting(AppSettings::NoBinaryName)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count)
                .global(true)
                .help("verbose level, (may be used multiple times for more verbosity)"),
        )
        .version(version!())
}

fn add_init_subcommand(app: App<'static>) -> App<'static> {
    app.subcommand(
        Command::new("init")
            .about("Initialize an Angreal template from source.")
            .arg(
                Arg::new("force")
                    .short('f')
                    .long("--force")
                    .takes_value(false)
                    .help("Force the rendering of a template, even if paths/files already exist."),
            )
            .arg(
                Arg::new("defaults")
                    .short('d')
                    .long("--defaults")
                    .takes_value(false)
                    .help("Use default values provided in the angreal.toml."),
            )
            .arg(
                Arg::new("values_file")
                    .long("--values")
                    .takes_value(true)
                    .help("Provide Values to template, bypassing template toml."),
            )
            .arg(Arg::new("template").takes_value(true).required(true).help(
                "The template to use. Either a pre-downloaded template name, or url to a git repo.",
            )),
    )
}

fn add_completion_subcommands(app: App<'static>) -> App<'static> {
    app.subcommand(
        Command::new("_complete")
            .hide(true) // Hidden from help
            .about("Generate shell completions (internal use)")
            .arg(
                Arg::new("args")
                    .multiple_values(true)
                    .help("Arguments to complete"),
            ),
    )
    .subcommand(
        Command::new("_completion")
            .hide(true) // Hidden from help
            .about("Generate completion script (internal use)")
            .arg(
                Arg::new("shell")
                    .takes_value(true)
                    .help("Shell to generate completion for"),
            ),
    )
}

fn add_project_subcommands(mut app: App<'static>) -> App<'static> {
    // Build the command tree
    let mut root = CommandNode::new_group("angreal".to_string(), None);

    // Add all commands to the tree
    for cmd in ANGREAL_TASKS.lock().unwrap().clone() {
        root.add_command(cmd);
    }

    // Convert command tree to clap App structure
    fn build_clap_command(node: &CommandNode) -> Command<'static> {
        let mut cmd = Command::new(Box::leak(node.name.clone().into_boxed_str()));

        if let Some(about) = &node.about {
            let about_static: &'static str = Box::leak(about.clone().into_boxed_str());
            cmd = cmd.about(Some(about_static));
        }

        // If this is a command node (has command data), add its arguments
        if let Some(command) = &node.command {
            let args = select_args(&command.name);
            for arg in args {
                let name_static: &'static str =
                    Box::leak(Box::new(arg.name.clone()).into_boxed_str());
                let mut a = Arg::new(name_static);
                attr_copy!(bool, a, takes_value, arg);
                attr_copy!(str, a, default_value, arg);
                attr_copy!(bool, a, require_equals, arg);
                attr_copy!(bool, a, multiple_values, arg);
                attr_copy!(u64, a, number_of_values, arg);
                attr_copy!(u64, a, max_values, arg);
                attr_copy!(u64, a, min_values, arg);
                attr_copy!(char, a, short, arg);
                attr_copy!(str, a, long, arg);
                attr_copy!(str, a, long_help, arg);
                attr_copy!(str, a, help, arg);
                attr_copy!(bool, a, required, arg);

                if arg.is_flag.unwrap() {
                    a = a.action(ArgAction::SetTrue);
                }
                cmd = cmd.arg(a);
            }
        }

        // If this is a group node, add SubcommandRequiredElseHelp setting
        if !node.children.is_empty() {
            cmd = cmd.setting(AppSettings::SubcommandRequiredElseHelp);

            // Add all child commands
            for child in node.children.values() {
                cmd = cmd.subcommand(build_clap_command(child));
            }
        }

        cmd
    }

    // Add all top-level commands and groups to the app
    for child in root.children.values() {
        app = app.subcommand(build_clap_command(child));
    }

    app
}

/// Build the final CLI from the registered tasks
pub fn build_app(in_angreal_project: bool) -> App<'static> {
    // Build the initial App with angreal sub commands
    let mut app = base_app_setup();

    // Always add completion subcommands (hidden)
    app = add_completion_subcommands(app);

    if in_angreal_project {
        app = add_project_subcommands(app);
    } else {
        app = add_init_subcommand(app);
    }
    app
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_app_in_project() {
        let app = build_app(true);
        assert_eq!(None, app.find_subcommand("init"));
    }

    #[test]
    fn test_generate_app_out_project() {
        let app = build_app(false);
        assert_ne!(None, app.find_subcommand("init"));
    }
}
