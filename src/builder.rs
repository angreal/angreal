//! The angreal app builder
//!

use crate::task::{AngrealArg, ANGREAL_ARGS, ANGREAL_GROUPS, ANGREAL_TASKS};
use clap::{App, AppSettings, Arg, ArgAction, Command};

/// Get the args for a given command.
pub fn select_args(name: String) -> Vec<AngrealArg> {
    let this = ANGREAL_ARGS.lock().unwrap().clone();

    this.iter()
        .cloned()
        .filter(|a| a.command_name == name.clone())
        .collect()
}

/// Build the final CLI from the registered tasks
pub fn build_app(in_angreal_project: bool) -> App<'static> {
    // Build the initial App with angreal sub commands
    let mut app = Command::new("angreal")
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
        .version(version!());
    if !in_angreal_project {
        app = app.subcommand(Command::new("init")
                        .about("Initialize an Angreal template from source.")
                        .arg(
                            Arg::new("force")
                            .short('f')
                            .long("--force")
                            .takes_value(false)
                            .help("Force the rendering of a template, even if paths/files already exist.")
                        )
                        .arg(
                            Arg::new("defaults")
                            .short('d')
                            .long("--defaults")
                            .takes_value(false)
                            .help("Use default values provided in the angreal.toml.")
                        )
                        .arg(
                            Arg::new("template")
                            .takes_value(true)
                            .required(true)
                            .help("The template to use. Either a pre-downloaded template name, or url to a git repo.")
                        )
                    )
    } else {
        // Generate groups into commands
        let mut top_level_groups = Vec::new();
        let mut groups = Vec::new();
        let registered_groups = ANGREAL_GROUPS.lock().unwrap().clone();
        for group in registered_groups.into_iter() {
            let mut cmd = Command::new(group.name.as_str());
            attr_copy_str!(cmd, about, group);
            let cmd = cmd.setting(AppSettings::SubcommandRequiredElseHelp);
            groups.push(cmd.clone());
        }
        // Generate all tasks into commands
        let registered_tasks = ANGREAL_TASKS.lock().unwrap().clone();
        for cmd in registered_tasks.into_iter() {
            let mut task = Command::new(cmd.name.as_str());
            attr_copy_str!(task, about, cmd);
            attr_copy_str!(task, long_about, cmd);

            let args = select_args(cmd.name.clone());
            for arg in args.into_iter() {
                let name = &*Box::leak(Box::new(arg.name));

                let mut a = Arg::new(name.as_str());
                attr_copy_bool!(a, takes_value, arg);
                attr_copy_str!(a, default_value, arg);
                attr_copy_bool!(a, require_equals, arg);
                attr_copy_bool!(a, multiple_values, arg);
                attr_copy_u64!(a, number_of_values, arg);
                attr_copy_u64!(a, max_values, arg);
                attr_copy_u64!(a, min_values, arg);
                attr_copy_char!(a, short, arg);
                attr_copy_str!(a, long, arg);
                attr_copy_str!(a, long_help, arg);
                attr_copy_str!(a, help, arg);
                attr_copy_bool!(a, required, arg);
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

        // Register all "top level commands"

        for top in top_level_groups {
            let top_level = groups
                .iter()
                .find(|&x| x.get_name() == top.as_str())
                .unwrap();
            app = app.subcommand(top_level.clone());
        }
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
