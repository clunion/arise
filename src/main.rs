//! ___________________________________________________________________________________________________________________________
//! **`PROJECT:    `** ARISE - A RaInmeter Skin Evolver    
//! **`HOME:       `** [arise on GitHub](https://github.com/clunion/arise)    
//! **`SYNOPSIS:   `** A Rainmeter (tm) Skin Evolver, a parameterized generator for rainmeter ini-files   
//! ___________________________________________________________________________________________________________________________
//! **`FILE:       `** main.rs 🦀   
//! **`DESCRIPTION:`** the main of arise, contains the one and only start and entry point of the program.   
//! ___________________________________________________________________________________________________________________________
//! **`LICENSE:    `**   
//! Copyright 2020 by Christian Lunau (Clunion)
//! MIT-License, see LICENSE.md file    
//! ___________________________________________________________________________________________________________________________
//! VERSION: | DATE:      | AUTHOR:   | CHANGES:   
//! :---     | :---       | :---:     | :---   
//! 0.1      | 2018-04-03 | Clunion   | creation
//! 0.2      | 2021-05-24 | Clunion   | building up some source structure, renamed from rm_skin_gen to arise
//! ___________________________________________________________________________________________________________________________
//!# Examples
//!```text
//! arise(.exe)
//!     Starts the program arise. 
//!     Depending on the operating system the name may differ: on MS-Windows the program file has the extension '.exe'.
//!
//! arise(.exe) --help
//!     Writes a short help text to the console window, which shows all available command line parameters and their meaning.
//!
//! arise(.exe) --sourcefile=<an_existing_arise_generator_definition_file>
//!     Reads the generator definition (<name>.arise) from the given definition file, interprets it and generates new 
//!     Rainmeter-ini file based on it.
//!```
//! ___________________________________________________________________________________________________________________________
//!    


//___ CRATES EXTERNAL: ________________________________________________________________________________________________________
extern crate clap;

//___ DECLARATIONS OF SUBMODULES TO INCLUDE: __________________________________________________________________________________
mod modules;                              // <dirname> (necessary for the explicit uses below...)

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________
use main_error::MainError;
use std::path::PathBuf;

use log::{trace, debug, info, warn, error};
use flexi_logger::{Logger, FileSpec, Duplicate, Cleanup, Criterion, Naming};

use clap::Arg;

use crate::modules::*;                    // crate::<dirname>::*
use crate::modules::core_logic::*;        // crate::<filename>::*
use crate::modules::config::*;            // crate::<filename>::*

//___ CONSTANTS: ______________________________________________________________________________________________________________
//___ none ___

//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ MACROS: _________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: ________________________________________________________________________________________________________________
//___ none ___

//___ METHODS: ________________________________________________________________________________________________________________
//___ none ___


/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  main   
/// **`TYPE:       `**  program entry point   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **`<none>     `**    
/// **`RETURNS:    `** **`Result --> `** - OK(state)   
/// **`            `** **`       --> `** - Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// The one and only main: startup and entry point of this program.   
/// Here the handling of command line parameters and calls to initialize und de-initialize are done.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-04-## | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
///  * define command line arguments for all configuration switches and variables    
///  * add handling of testing mode   
/// ___________________________________________________________________________________________________________________________

fn main() -> Result<(), MainError>
{
let mut arise_config: AriseConfig = AriseConfig::default();

// Initialize flexi_logger, see documentation of Struct flexi_logger::LogSpecification:
match Logger::try_with_env_or_str("arise=debug, arise::modules::core_logic=debug, arise::modules::arise_log=debug")
            .unwrap_or_else(|e| panic!("Logger initialization failed with {:?}", e))
            .rotate(Criterion::Size(100_000), Naming::Timestamps, Cleanup::KeepLogAndCompressedFiles(4,10))
            .duplicate_to_stderr(Duplicate::Trace)
            .format_for_stderr(arise_log::console_line_format)
            .format_for_files( arise_log::file_line_format)     
            .log_to_file(FileSpec::default()
                                  .directory("log")
                                  .basename("arise")
                                  .suppress_timestamp()
                                  .suffix("log"),
                        )
            .start()
    {
    Ok(_reconf_handle) => {},
    Err(error)         => { println!("ERROR initializing flexi_logger: {:?}", error); return Err(error.into()) },
    }

if   cfg!(debug_assertions) {println!("compiled in DEBUG mode");   }
else                        {println!("compiled in RELEASE mode"); }

trace!("this is a  trace message");
debug!("this is a  debug {}", "message");
info!( "this is an info"); 
warn!( "this is a  warn message");
error!("this is an error");


// Parse the command line using clap:
let cmd_line = clap::App::new("Arise")
                   .version("0.1")
                   .author("Clunion <Christian.Lunau@gmx.de>")
                   .about("A RaInmeter Skin Evolver")
                   .arg(Arg::with_name("verbosity")                    // <--VERBOSITY --------------------------------------------
                       .short("v")
                       .multiple(true)
                       .help("Sets the level of verbosity, more vs, more chatter."))
                   .arg(Arg::with_name("test-mode")                    // <--TEST-MODE---------------------------------------------
                       .help("Starts the program in testing mode.")
                       .short("t")
                       .long("test")
                       .takes_value(false))
                   .arg(Arg::with_name("debug-mode")                   // <--DEBUG-MODE--------------------------------------------
                       .short("d")
                       .long("debug")
                       .help("Starts the program in debug mode.")
                       .takes_value(false))
                   .arg(Arg::with_name("skin")                         // <--name of Skin (=basename of source- and target-files)--
                       .short("s")
                       .long("skin")
                       .value_name("SKIN")
                       .help("Sets the name of the skin, used as basename for source- and target-files, extensions are '.arise' and '.ini'.")
                       .takes_value(true))
                   .get_matches();

// Increase the amount of logging based on how many times the user used the "verbose" flag (i.e. 'myprog -v' or 'myprog -vv' or 'myprog -v -v -v':
match cmd_line.occurrences_of("verbosity") 
    {
    0 => {arise_config.verbosity = 0; info!("Verbosity={}, No verbose info"     ,arise_config.verbosity); }, 
    1 => {arise_config.verbosity = 1; info!("Verbosity={}, Some verbose info"   ,arise_config.verbosity); },
    2 => {arise_config.verbosity = 2; info!("Verbosity={}, Tons of verbose info",arise_config.verbosity); },
    3 => {arise_config.verbosity = 3; info!("Verbosity={}, Don't be crazy"      ,arise_config.verbosity); },
    _ => {arise_config.verbosity = 9; info!("Verbosity={}, Maximum verbosity"   ,arise_config.verbosity); },
    }

// Handle the existence of command line parameters by matching over name:
if  cmd_line.is_present("test-mode")     {info!("Test Mode enabled")    ; arise_config.test        = true; }
if  cmd_line.is_present("debug-mode")    {info!("Debug Mode enabled")   ; arise_config.debug       = true; } 

// Get the values of the known parameters, if supplied on command line, or set them with the defaults:
arise_config.skin_name        = PathBuf::from(cmd_line.value_of("skin").unwrap_or(DEFAULT_SKIN_NAME));
arise_config.arise_file_name  = arise_config.skin_name.clone();
arise_config.arise_file_name.set_extension(ARISE_FILE_EXTENSION);
arise_config.skin_file_name   = arise_config.skin_name.clone();
arise_config.skin_file_name.set_extension(SKIN_FILE_EXTENSION);

debug!("command line: skin-name:    {}",   arise_config.skin_name.display());
debug!("base_pathpart:              {}",   arise_config.base_pathpart.display());
debug!("res_pathpart:               {}",   arise_config.res_pathpart.display());
debug!("inp_pathpart:               {}",   arise_config.inp_pathpart.display());
debug!("out_pathpart:               {}",   arise_config.out_pathpart.display());
debug!("skin-name:                  {}",   arise_config.skin_name.display());
debug!("arise-filename:             {}",   arise_config.arise_file_name.display());
debug!("skin-filename:              {}",   arise_config.skin_file_name.display());
debug!("install_skin_folder:        {}",   arise_config.install_skin_folder .display());
debug!("rainmeter_exe:              {}",   arise_config.rainmeter_exe       .display());
debug!("rainmeter_param_refreshapp: {:?}", arise_config.rainmeter_param_refreshapp);
debug!("rainmeter_param_manage:     {:?}", arise_config.rainmeter_param_manage    );

// Check directories, create them if missing:
if !exists_dir(&arise_config.res_pathpart) {match create_dir(&arise_config.res_pathpart) {Ok(_) => info!("created: '{}'",arise_config.res_pathpart.display()), Err(error) => panic!("couldn't create dir '{}': {}", arise_config.res_pathpart.display(), error),}; }
if !exists_dir(&arise_config.inp_pathpart) {match create_dir(&arise_config.inp_pathpart) {Ok(_) => info!("created: '{}'",arise_config.inp_pathpart.display()), Err(error) => panic!("couldn't create dir '{}': {}", arise_config.inp_pathpart.display(), error),}; }
if !exists_dir(&arise_config.out_pathpart) {match create_dir(&arise_config.out_pathpart) {Ok(_) => info!("created: '{}'",arise_config.out_pathpart.display()), Err(error) => panic!("couldn't create dir '{}': {}", arise_config.out_pathpart.display(), error),}; }

// do the real work:
match core_logic(&arise_config)
    {
        Err(error) => { error!("Error executing core-logic: {:?}", error); Err(error.into()) },
        Ok(stat)   => { debug!("OK, got {} from logic-module.",stat);      Ok(()) },
    }
}
