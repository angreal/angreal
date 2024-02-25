//! The angreal app builder
//!

use crate::task::{AngrealArg, ANGREAL_ARGS, ANGREAL_GROUPS, ANGREAL_TASKS};
use clap::{App, AppSettings, Arg, ArgAction, Command};

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
            .arg(Arg::new("template").takes_value(true).required(true).help(
                "The template to use. Either a pre-downloaded template name, or url to a git repo.",
            )),
    )
}

fn add_project_subcommands(mut app: App<'static>) -> App<'static> {
    let mut top_level_groups = Vec::new();
    // first we generate our command groups
    let mut groups = Vec::new();
    for group in ANGREAL_GROUPS.lock().unwrap().clone().into_iter() {
        let mut cmd = Command::new(group.name.as_str());
        attr_copy!(str, cmd, about, group);
        let cmd = cmd.setting(AppSettings::SubcommandRequiredElseHelp);
        groups.push(cmd.clone());
    }

    // now we generate our commands
    for cmd in ANGREAL_TASKS.lock().unwrap().clone().into_iter() {
        let mut task = Command::new(cmd.name.as_str());
        attr_copy!(str, task, about, cmd);
        attr_copy!(str, task, long_about, cmd);

        // we generate each arg for the command and register it
        for arg in select_args(cmd.name.as_str()).into_iter() {
            let name_static: &'static str = Box::leak(Box::new(arg.name.clone()).into_boxed_str());
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
            task = task.arg(a);
        }

        if cmd.group.is_none() || cmd.group.clone().unwrap().is_empty() {
            // no group , register with `app`
            app = app.subcommand(task);
        } else {
            let c_groups = cmd.group.clone().unwrap();
            let task_parent_pos = groups
                .iter()
                .position(|x| x.get_name() == c_groups.last().unwrap().name.as_str())
                .unwrap();

            let mut task_parent = groups[task_parent_pos].clone();
            task_parent = task_parent.subcommand(task);
            groups.insert(task_parent_pos, task_parent);

            for (pos, e) in c_groups.iter().enumerate().rev() {
                if pos == 0 {
                    // register the top level task when we're done building the chain.
                    top_level_groups.push(e.name.clone());
                    break;
                }

                let this_subcommand_pos = groups
                    .iter()
                    .position(|x| x.get_name() == e.name.as_str())
                    .unwrap();
                let parent_subcommand_pos = groups
                    .iter()
                    .position(|x| x.get_name() == c_groups[pos - 1].name.as_str())
                    .unwrap();

                let this_subcommand = groups[this_subcommand_pos].clone();
                let mut parent_subcommand = groups[parent_subcommand_pos].clone();

                parent_subcommand = parent_subcommand.subcommand(this_subcommand);

                groups.insert(parent_subcommand_pos, parent_subcommand);
            }

            // loop groups backwards, register up the chain with a copy
        }
    }

    // sort, de duplicate, and register top level commands
    top_level_groups.sort();
    top_level_groups.dedup();
    for top in top_level_groups {
        let top_level = groups
            .iter()
            .find(|&x| x.get_name() == top.as_str())
            .unwrap();
        app = app.subcommand(top_level.clone());
    }

    app
}

/// Build the final CLI from the registered tasks
pub fn build_app(in_angreal_project: bool) -> App<'static> {
    // Build the initial App with angreal sub commands
    let mut app = base_app_setup();

    if !in_angreal_project {
        app = add_init_subcommand(app);
    } else {
        app = add_project_subcommands(app)
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
