//! Core structures for describing tasks and arguments
//!

use log::debug;
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;

/// Registers the Command and Arg structs to the python api in the `angreal` module
pub fn register(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    debug!("Registering Angreal types to Python module");
    m.add_class::<AngrealCommand>()?;
    m.add_class::<AngrealArg>()?;
    m.add_class::<AngrealGroup>()?;
    m.add_class::<ToolDescription>()?;
    debug!("Successfully registered all Angreal types");
    Ok(())
}

/// A long lived structure that stores AngrealCommands upon registration, keyed by full path
pub static ANGREAL_TASKS: Lazy<Mutex<HashMap<String, AngrealCommand>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// A long lived structure that stores AngrealArgs for commands upon registration, keyed by command path
pub static ANGREAL_ARGS: Lazy<Mutex<HashMap<String, Vec<AngrealArg>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// A long lived structure that stores Angreal Groups for commands upon registration
pub static ANGREAL_GROUPS: Lazy<Mutex<Vec<AngrealGroup>>> = Lazy::new(|| Mutex::new(vec![]));

// Thread-local storage for tracking the last registered command path for argument linking
thread_local! {
    static LAST_COMMAND_PATH: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Set the current command path for argument registration
pub fn set_current_command_path(path: String) {
    LAST_COMMAND_PATH.with(|p| *p.borrow_mut() = Some(path));
}

/// Get the current command path for argument registration
pub fn get_current_command_path() -> Option<String> {
    LAST_COMMAND_PATH.with(|p| p.borrow().clone())
}

/// Generate a full path key for a command based on its group hierarchy
pub fn generate_command_path_key(command: &AngrealCommand) -> String {
    let path = match &command.group {
        None => command.name.clone(),
        Some(groups) => {
            let group_path = groups
                .iter()
                .map(|g| g.name.clone())
                .collect::<Vec<_>>()
                .join(".");
            format!("{}.{}", group_path, command.name)
        }
    };
    // Strip any leading dots that might have been introduced
    path.strip_prefix('.').unwrap_or(&path).to_string()
}

/// Generate a full path key for a command based on group names and command name
pub fn generate_path_key_from_parts(groups: &[String], command_name: &str) -> String {
    let path = if groups.is_empty() {
        command_name.to_string()
    } else {
        format!("{}.{}", groups.join("."), command_name)
    };
    // Strip any leading dots that might have been introduced
    path.strip_prefix('.').unwrap_or(&path).to_string()
}

/// A group is just a special type of sub-command
#[derive(Clone, Debug)]
#[pyclass(name = "Group")]
pub struct AngrealGroup {
    /// The name of the command group
    #[pyo3(get)]
    pub name: String,
    /// The about of the command group
    #[pyo3(get)]
    pub about: Option<String>,
}

/// Rich description for exposing a command as an MCP tool
///
/// This class allows task authors to provide detailed, prose-based descriptions
/// that help AI agents understand when and how to use a command effectively.
/// The description is essentially a mini-prompt that teaches the agent about the tool.
///
/// # Example
/// ```python
/// import angreal
///
/// @angreal.command(
///     name="build",
///     about="Build the project",
///     tool=angreal.ToolDescription("""
/// Compiles all Rust crates and creates the Python wheel for distribution.
///
/// ## When to use
/// - Before releasing a new version
/// - Testing production builds locally
///
/// ## When NOT to use
/// - During iterative development (use `cargo build` directly)
///
/// ## Examples
/// ```
/// angreal build
/// angreal build --release
/// ```
///
/// ## Preconditions
/// - Rust toolchain installed
/// - Run `angreal dev check-deps` first if unsure
/// """)
/// )
/// def build():
///     pass
/// ```
#[derive(Clone, Debug)]
#[pyclass(name = "ToolDescription")]
pub struct ToolDescription {
    /// The full prose description of the tool (markdown supported)
    #[pyo3(get)]
    pub description: String,
    /// Risk level: "safe", "read_only", or "destructive"
    #[pyo3(get)]
    pub risk_level: String,
}

#[pymethods]
impl ToolDescription {
    #[new]
    #[pyo3(signature = (description, *, risk_level = None))]
    fn __new__(description: &str, risk_level: Option<&str>) -> Self {
        let risk = risk_level.unwrap_or("safe");
        // Validate risk_level
        let validated_risk = match risk {
            "safe" | "read_only" | "destructive" => risk.to_string(),
            _ => {
                log::warn!(
                    "Invalid risk_level '{}', defaulting to 'safe'. Valid values: safe, read_only, destructive",
                    risk
                );
                "safe".to_string()
            }
        };

        ToolDescription {
            description: description.to_string(),
            risk_level: validated_risk,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "ToolDescription(description=<{} chars>, risk_level='{}')",
            self.description.len(),
            self.risk_level
        )
    }
}
/// Methods exposed in the python API
#[pymethods]
impl AngrealGroup {
    #[new]
    #[pyo3(signature = (name, about=None))]
    fn __new__(name: &str, about: Option<&str>) -> Self {
        let group = AngrealGroup {
            name: name.to_string(),
            about: about.map(|i| i.to_string()),
        };

        let mut groups = ANGREAL_GROUPS.lock().unwrap();
        if !groups.iter().any(|g| g.name == group.name) {
            debug!("Adding new group: {}", group.name);
            groups.push(group.clone());
        } else {
            debug!("Group {} already exists, skipping add", group.name);
        }
        drop(groups);
        debug!(
            "Current ANGREAL_GROUPS state: {:#?}",
            ANGREAL_GROUPS.lock().unwrap()
        );
        group
    }
}

/// A command describes a subcommand to be registered with the CLI
#[derive(Debug)]
#[pyclass(name = "Command")]
pub struct AngrealCommand {
    /// The name of the sub command
    #[pyo3(get)]
    pub name: String,
    /// A short description of what the command does
    #[pyo3(get)]
    pub about: Option<String>,
    /// A longer description of what the command does
    #[pyo3(get)]
    pub long_about: Option<String>,
    /// The actual function that is executed when the command is run
    #[pyo3(get)]
    pub func: Py<PyAny>,
    /// The group this command belongs to
    #[pyo3(get)]
    pub group: Option<Vec<AngrealGroup>>,
    /// Rich tool description for MCP integration
    #[pyo3(get)]
    pub tool: Option<ToolDescription>,
}

impl Clone for AngrealCommand {
    fn clone(&self) -> Self {
        Python::attach(|py| Self {
            name: self.name.clone(),
            about: self.about.clone(),
            long_about: self.long_about.clone(),
            func: self.func.clone_ref(py),
            group: self.group.clone(),
            tool: self.tool.clone(),
        })
    }
}

/// Methods exposed to the python API
#[pymethods]
impl AngrealCommand {
    /// Initialization method for the object. The command is registered to `ANGREAL_TASKS` upon instantiation from the python api
    ///
    ///
    /// The decorated version is the most ergonmoic way to use this object.
    /// # Example
    /// ```python
    /// import angreal
    ///
    /// @angreal.command(name='test',about='a short message',
    /// long_about='a much longer message`)
    /// def test-message():
    ///     pass
    /// ```
    ///
    /// ```python
    /// import angreal
    ///
    /// def test-message():
    ///     pass
    ///
    /// angreal.Command(name='test',about='a short message',
    /// long_about='a much longer message`, func=test-message)
    /// ```
    #[new]
    #[pyo3(signature = (name, func, about=None, long_about=None, group=None, tool=None))]
    fn __new__(
        name: &str,
        func: Py<PyAny>,
        about: Option<&str>,
        long_about: Option<&str>,
        group: Option<Vec<AngrealGroup>>,
        tool: Option<ToolDescription>,
    ) -> Self {
        debug!("Creating new AngrealCommand with name: {}", name);
        let cmd = AngrealCommand {
            name: name.to_string(),
            about: about.map(|i| i.to_string()),
            long_about: long_about.map(|i| i.to_string()),
            group,
            func,
            tool,
        };

        let path_key = generate_command_path_key(&cmd);
        ANGREAL_TASKS
            .lock()
            .unwrap()
            .insert(path_key.clone(), cmd.clone());

        // Set current command path for argument registration
        set_current_command_path(path_key.clone());

        debug!(
            "Registered new command '{}' with path key: {}",
            name, path_key
        );
        debug!(
            "Updated ANGREAL_TASKS registry size: {}",
            ANGREAL_TASKS.lock().unwrap().len()
        );
        cmd
    }
    /// Add a (task::AngrealGroup) to the task::AngrealCommand called on
    pub fn add_group(&mut self, group: AngrealGroup) -> PyResult<()> {
        debug!("Adding group '{}' to command '{}'", group.name, self.name);

        // Get the current path key for this command
        let old_path_key = generate_command_path_key(self);

        if self.group.is_none() {
            debug!(
                "Initializing empty group vector for command '{}'",
                self.name
            );
            self.group = Some(Vec::new());
        }

        let mut g = self.group.as_mut().unwrap().clone();

        debug!("Adding group '{}' to command '{}'", group.name, self.name);
        g.insert(0, group);
        self.group = Some(g.clone());

        // Generate new path key and update registry
        let new_path_key = generate_command_path_key(self);
        let mut tasks = ANGREAL_TASKS.lock().unwrap();

        // Remove old entry and insert with new key
        if let Some(_cmd) = tasks.remove(&old_path_key) {
            tasks.insert(new_path_key.clone(), self.clone());
            debug!(
                "Updated command path from '{}' to '{}'",
                old_path_key, new_path_key
            );
        } else {
            // Fallback: just insert with new key
            tasks.insert(new_path_key.clone(), self.clone());
            debug!("Inserted command with new path: '{}'", new_path_key);
        }

        debug!("Current ANGREAL_TASKS registry size: {}", tasks.len());
        drop(tasks);

        // Also update arguments registry with new path key
        let mut args_registry = ANGREAL_ARGS.lock().unwrap();
        if let Some(args) = args_registry.remove(&old_path_key) {
            args_registry.insert(new_path_key.clone(), args);
            debug!(
                "Moved arguments from '{}' to '{}'",
                old_path_key, new_path_key
            );
        }

        Ok(())
    }
}

/// An argument to augment the behavior of an angreal command
#[derive(Clone, Debug)]
#[pyclass(name = "Arg")]
pub struct AngrealArg {
    /// The name of the argument, required to match the name in the function being executed by the command
    #[pyo3(get)]
    pub name: String,
    /// The command name associated with this argument (for backward compatibility)
    #[pyo3(get)]
    pub command_name: String,
    /// The full command path (internal use for collision resolution)
    pub command_path: String,
    /// Whether or not the argument consumes a value from the command line
    #[pyo3(get)]
    pub takes_value: Option<bool>,
    /// The default value to be applied to the arg.
    #[pyo3(get)]
    pub default_value: Option<String>,
    /// whether or not the argument is a flag (bool)
    #[pyo3(get)]
    pub is_flag: Option<bool>,
    /// Whether or not the argument requires an `=` behind it to set a value
    #[pyo3(get)]
    pub require_equals: Option<bool>,
    /// Whether or not the argument takes multiple values
    #[pyo3(get)]
    pub multiple_values: Option<bool>,
    /// The number of values the argument takes
    #[pyo3(get)]
    pub number_of_values: Option<u32>,
    /// The maximum number of values the argument takes
    #[pyo3(get)]
    pub max_values: Option<u32>,
    /// The minimum number of values the argument takes
    #[pyo3(get)]
    pub min_values: Option<u32>,
    /// The python type to apply the the consumed value (int, string, float)
    #[pyo3(get)]
    pub python_type: Option<String>,
    /// the short flag to be used on the command line (i.e. `-s`)
    #[pyo3(get)]
    pub short: Option<char>,
    /// the long flag to be used on the command line (i.e. `--long`)
    #[pyo3(get)]
    pub long: Option<String>,
    /// A verbose help message to be displayed
    #[pyo3(get)]
    pub long_help: Option<String>,
    /// a shorter help message to be displayed
    #[pyo3(get)]
    pub help: Option<String>,
    /// whether or not the argument is required
    #[pyo3(get)]
    pub required: Option<bool>,
}

#[pymethods]
impl AngrealArg {
    /// Adds an argument to an angreal command.
    ///
    /// The decorated version is the most ergonmoic way to use this object.
    /// # Example
    /// ```python
    /// import angreal
    ///
    /// @angreal.command(name='echo',about='a needless echo replacement',
    /// @angreal.argument(name="phrase", help="the phrase to echo", required=True)
    ///    def task_echo(phrase):
    ///        print(phrase)
    /// ```
    ///
    /// ```python
    /// import angreal
    ///
    /// def echo(phrase):
    ///     print(phrase)
    ///
    /// angreal.Command(name='echo',about='a needless echo replacement', func=test-message)
    /// angreal.Arg(name="phrase", help="the phrase to echo", required=True, command_name='echo')
    /// ```
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (name, command_name, default_value=None, is_flag=None, require_equals=None, multiple_values=None, number_of_values=None, max_values=None, min_values=None, short=None, long=None, long_help=None, help=None, required=None, takes_value=None, python_type=None))]
    fn __new__(
        name: &str,
        command_name: &str,
        default_value: Option<&str>,
        is_flag: Option<bool>,
        require_equals: Option<bool>,
        multiple_values: Option<bool>,
        number_of_values: Option<u32>,
        max_values: Option<u32>,
        min_values: Option<u32>,
        short: Option<char>,
        long: Option<&str>,
        long_help: Option<&str>,
        help: Option<&str>,
        required: Option<bool>,
        takes_value: Option<bool>,
        python_type: Option<&str>,
    ) -> Self {
        debug!(
            "Creating new AngrealArg '{}' for command '{}'",
            name, command_name
        );

        // Get the current command path or fallback to command_name if not available
        let command_path = get_current_command_path().unwrap_or_else(|| command_name.to_string());

        let arg = AngrealArg {
            name: name.to_string(),
            command_name: command_name.to_string(),
            command_path: command_path.clone(),
            takes_value: Some(takes_value.unwrap_or(true)),
            default_value: default_value.map(|i| i.to_string()),
            is_flag: Some(is_flag.unwrap_or(false)),
            require_equals,
            multiple_values,
            number_of_values,
            max_values,
            min_values,
            python_type: Some(python_type.unwrap_or("str").to_string()),
            short,
            long: long.map(|i| i.to_string()),
            long_help: long_help.map(|i| i.to_string()),
            help: help.map(|i| i.to_string()),
            required,
        };

        // Insert into HashMap using command path as key
        let mut args_registry = ANGREAL_ARGS.lock().unwrap();
        args_registry
            .entry(command_path.clone())
            .or_default()
            .push(arg.clone());

        debug!(
            "Registered new argument '{}' for command path '{}'",
            name, command_path
        );
        debug!(
            "Current ANGREAL_ARGS registry has {} command paths",
            args_registry.len()
        );
        arg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::Python;

    #[test]
    fn test_hierarchical_command_registration() {
        Python::attach(|py| {
            // Save and restore global state for test isolation
            let original_tasks = ANGREAL_TASKS.lock().unwrap().clone();
            let original_args = ANGREAL_ARGS.lock().unwrap().clone();

            // Clear any existing commands for clean test
            ANGREAL_TASKS.lock().unwrap().clear();
            ANGREAL_ARGS.lock().unwrap().clear();

            // Create two groups
            let group1 = AngrealGroup {
                name: "group1".to_string(),
                about: Some("First group".to_string()),
            };

            let group2 = AngrealGroup {
                name: "group2".to_string(),
                about: Some("Second group".to_string()),
            };

            // Create two commands with the same name but in different groups
            let cmd1 = AngrealCommand {
                name: "all".to_string(),
                about: Some("Run all tests in group1".to_string()),
                long_about: None,
                group: Some(vec![group1.clone()]),
                func: py.None(),
                tool: None,
            };

            let cmd2 = AngrealCommand {
                name: "all".to_string(),
                about: Some("Run all tests in group2".to_string()),
                long_about: None,
                group: Some(vec![group2.clone()]),
                func: py.None(),
                tool: None,
            };

            // Register both commands
            let path1 = generate_command_path_key(&cmd1);
            let path2 = generate_command_path_key(&cmd2);

            ANGREAL_TASKS.lock().unwrap().insert(path1.clone(), cmd1);
            ANGREAL_TASKS.lock().unwrap().insert(path2.clone(), cmd2);

            // Verify both commands are registered with different paths
            assert_eq!(path1, "group1.all");
            assert_eq!(path2, "group2.all");
            assert_eq!(ANGREAL_TASKS.lock().unwrap().len(), 2);

            // Verify we can retrieve both commands
            assert!(ANGREAL_TASKS.lock().unwrap().get("group1.all").is_some());
            assert!(ANGREAL_TASKS.lock().unwrap().get("group2.all").is_some());

            // Verify they have different about texts
            let retrieved_cmd1 = ANGREAL_TASKS
                .lock()
                .unwrap()
                .get("group1.all")
                .unwrap()
                .clone();
            let retrieved_cmd2 = ANGREAL_TASKS
                .lock()
                .unwrap()
                .get("group2.all")
                .unwrap()
                .clone();

            assert_eq!(
                retrieved_cmd1.about,
                Some("Run all tests in group1".to_string())
            );
            assert_eq!(
                retrieved_cmd2.about,
                Some("Run all tests in group2".to_string())
            );

            // Restore original state
            *ANGREAL_TASKS.lock().unwrap() = original_tasks;
            *ANGREAL_ARGS.lock().unwrap() = original_args;
        });
    }

    #[test]
    fn test_argument_collision_resolution() {
        Python::attach(|py| {
            // Save and restore global state for test isolation
            let original_tasks = ANGREAL_TASKS.lock().unwrap().clone();
            let original_args = ANGREAL_ARGS.lock().unwrap().clone();

            // Clear registries for clean test
            ANGREAL_TASKS.lock().unwrap().clear();
            ANGREAL_ARGS.lock().unwrap().clear();

            // Create commands and register them
            let group1 = AngrealGroup {
                name: "group1".to_string(),
                about: None,
            };

            let group2 = AngrealGroup {
                name: "group2".to_string(),
                about: None,
            };

            let cmd1 = AngrealCommand {
                name: "test".to_string(),
                about: None,
                long_about: None,
                group: Some(vec![group1]),
                func: py.None(),
                tool: None,
            };

            let cmd2 = AngrealCommand {
                name: "test".to_string(),
                about: None,
                long_about: None,
                group: Some(vec![group2]),
                func: py.None(),
                tool: None,
            };

            let path1 = generate_command_path_key(&cmd1);
            let path2 = generate_command_path_key(&cmd2);

            ANGREAL_TASKS.lock().unwrap().insert(path1.clone(), cmd1);
            ANGREAL_TASKS.lock().unwrap().insert(path2.clone(), cmd2);

            // Create arguments for each command
            let arg1 = AngrealArg {
                name: "verbose".to_string(),
                command_name: "test".to_string(),
                command_path: path1.clone(),
                takes_value: Some(false),
                default_value: None,
                is_flag: Some(true),
                require_equals: None,
                multiple_values: None,
                number_of_values: None,
                max_values: None,
                min_values: None,
                python_type: Some("bool".to_string()),
                short: Some('v'),
                long: Some("verbose".to_string()),
                long_help: None,
                help: Some("Verbose output".to_string()),
                required: Some(false),
            };

            let arg2 = AngrealArg {
                name: "force".to_string(),
                command_name: "test".to_string(),
                command_path: path2.clone(),
                takes_value: Some(false),
                default_value: None,
                is_flag: Some(true),
                require_equals: None,
                multiple_values: None,
                number_of_values: None,
                max_values: None,
                min_values: None,
                python_type: Some("bool".to_string()),
                short: Some('f'),
                long: Some("force".to_string()),
                long_help: None,
                help: Some("Force operation".to_string()),
                required: Some(false),
            };

            // Register arguments using the HashMap structure
            ANGREAL_ARGS
                .lock()
                .unwrap()
                .entry(path1.clone())
                .or_default()
                .push(arg1);
            ANGREAL_ARGS
                .lock()
                .unwrap()
                .entry(path2.clone())
                .or_default()
                .push(arg2);

            // Verify arguments are correctly separated by command path
            let args1 = crate::builder::select_args(&path1);
            let args2 = crate::builder::select_args(&path2);

            assert_eq!(args1.len(), 1);
            assert_eq!(args2.len(), 1);

            assert_eq!(args1[0].name, "verbose");
            assert_eq!(args2[0].name, "force");

            // Verify that arguments don't cross-contaminate
            assert_eq!(args1[0].command_path, "group1.test");
            assert_eq!(args2[0].command_path, "group2.test");

            // Restore original state
            *ANGREAL_TASKS.lock().unwrap() = original_tasks;
            *ANGREAL_ARGS.lock().unwrap() = original_args;
        });
    }

    #[test]
    fn test_path_key_generation() {
        // Test top-level command (no group)
        let key = generate_path_key_from_parts(&[], "build");
        assert_eq!(key, "build");

        // Test single group
        let key = generate_path_key_from_parts(&["docker".to_string()], "run");
        assert_eq!(key, "docker.run");

        // Test nested groups
        let key =
            generate_path_key_from_parts(&["docker".to_string(), "compose".to_string()], "up");
        assert_eq!(key, "docker.compose.up");
    }
}
