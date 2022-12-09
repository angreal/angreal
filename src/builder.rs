
use clap::{App, AppSettings, Arg, Command};
use crate::task::{AngrealArg, ANGREAL_ARGS, ANGREAL_TASKS};


pub fn select_args(name: String) -> Vec<AngrealArg> {
    let this = ANGREAL_ARGS.lock().unwrap().clone();

    this.iter()
        .cloned()
        .filter(|a| a.command_name == Some(name.clone()))
        .collect()
}

pub fn build_app() -> App<'static>{

    // Build the initial App with angreal sub commands
    let mut app = App::new("angreal")
    .setting(AppSettings::NoBinaryName)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .allow_external_subcommands(true)
    .subcommand(Command::new("init").about("Initialize an Angreal template from source."))
    .version(version!());


    let commands = ANGREAL_TASKS.lock().unwrap().clone();
    for cmd in commands.into_iter() {
        
        let mut sc = Command::new(cmd.name.as_str());
        attr_copy_str!(sc,about,cmd);
        attr_copy_str!(sc,long_about,cmd);


        let args = select_args(cmd.name.clone());
        for arg in args.into_iter() {
        
            let name = &*Box::leak(Box::new(arg.name));
            
            let mut a = Arg::new(name.as_str());
            
            attr_copy_bool!(a,takes_value,arg);
            attr_copy_str!(a,default_value,arg);
            attr_copy_bool!(a,require_equals,arg);
            attr_copy_bool!(a,multiple_values,arg);
            attr_copy_u64!(a,number_of_values,arg);
            attr_copy_u64!(a,max_values,arg);
            attr_copy_u64!(a,min_values,arg);
            attr_copy_char!(a,short,arg);
            attr_copy_str!(a,long,arg);
            attr_copy_str!(a,long_help,arg);
            attr_copy_str!(a,help,arg);
            attr_copy_bool!(a,required,arg);
            sc = sc.arg(a);
        }

        app = app.subcommand(sc);
    }
    app
}

    



// #[cfg(test)]
// #[path = "../tests"]
// mod tests {
//     mod common;

//     #[test]
//     fn test_generate_command(){
//         crate::builder::build_task();
//     }

// }
