//! ___________________________________________________________________________________________________________________________
//! **`PROJECT:    `** ARISE - A RaInmeter Skin Evolver    
//! **`HOME:       `** [arise on GitHub](https://github.com/clunion/arise)    
//! **`SYNOPSIS:   `** A Rainmeter (tm) Skin Generator, a parameterized generator for rainmeter skins (ini-files)
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
//! ___________________________________________________________________________________________________________________________
//!# Examples
//!```
//! arise(.exe)
//!     Starts the program arise. 
//!     Depending on the operating system the name may differ: on MS-Windows the program file has the extension '.exe'.
//!
//! arise(.exe) --help
//!     Writes a short help text to the console window, which shows all available command line parameters and their meaning.
//!
//! arise(.exe) --generate=<an_existing_generator_definition_file>
//!     Reads the generator definition (<name>.rms_sin_gen) from the given definition file, interprets it and generates new 
//!     Rainmeter-ini file based on it.
//!```
//! ___________________________________________________________________________________________________________________________
//!    

//___ CRATES EXTERNAL: ________________________________________________________________________________________________________
extern crate clap;

//___ DECLARATIONS OF SUBMODULES TO INCLUDE: __________________________________________________________________________________
// mod modules;                              // <dirname>
mod arise;                             // <filename>

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________
//use std::env;
use std::io;

use clap::Arg;

// use std::cmp::Ordering;

// use std::error::Error;
use std::io::ErrorKind;

use std::path::Path;
use std::path::PathBuf;

use arise::*; 

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
///  * add recognition and handling of debug mode (compile switch/definition?)   
///  * add recognition and handling of testing mode   
/// ___________________________________________________________________________________________________________________________

fn main() -> Result<(), io::Error>
{
let     _bret: bool  = false;                                 // common boolean return value
let     res_path     = PathBuf::from("resources");
let     in_path      = PathBuf::from("input");
let     out_path     = PathBuf::from("output");
let     ill_path     = PathBuf::from("il:legal");             // illegal, not creatable Path (only for testing of the error handling)
let     non_path     = PathBuf::from("does not exist");       // path does not exist         (only for testing of the error handling)

let     skin_name    = "StorageMon";                          // Name of the current Rainmeter-Skin to generate 

let     gen_filename = PathBuf::from("StorageMon.arise");

let mut in_filename  = PathBuf::from(&in_path ); 


// Parse the command line using clap:
let cmd_line = clap::App::new("Arise")
                   .version("0.1")
                   .author("Clunion <Christian.Lunau@gmx.de>")
                   .about("A RaInmeter Skin Evolver")
                   .arg(Arg::with_name("file")                         // <--CONFIG-File-------------------------------
                       .short("c")
                       .long("file")
                       .value_name("FILE")
                       .help("Sets a specific input file.")
                       .takes_value(true))
                   .arg(Arg::with_name("verbosity")                    // <--VERBOSITY --------------------------------
                       .short("v")
                       .multiple(true)
                       .help("Sets the level of verbosity, more vs, more chatter."))
                   .arg(Arg::with_name("test-mode")                    // <--TEST-MODE---------------------------------
                       .help("Lets the program run in testing mode.")
                       .short("t")
                       .long("test")
                       .takes_value(false))
                   .arg(Arg::with_name("debug-mode")                   // <--DEBUG-MODE--------------------------------
                       .short("d")
                       .long("debug")
                       .help("Lets the program run in debug mode.")
                       .takes_value(false))
                   .get_matches();

// Get the name of a config-file, if supplied on commandline, or defaults to config::INI_FILE_NAME
let config_filename = cmd_line.value_of("configfile").unwrap_or(config::INI_FILE_NAME);
shard_config.ini_file_name   = config_filename.to_string();
info!("config-file: {}", shard_config.ini_file_name);









in_filename.push(&gen_filename);

// Check preconditions to run:
if !rmsg_exists_dir(&non_path)    {println!("wont be created now"); }
if !rmsg_exists_dir(&res_path)    {match rmsg_create_dir(&res_path) {Ok(_) => println!("created: '{}'",res_path.display()), Err(error) =>   panic!("couldn't create dir '{}': {}", res_path.display(), error),}; }
if !rmsg_exists_dir(&in_path )    {match rmsg_create_dir(&in_path ) {Ok(_) => println!("created: '{}'",in_path .display()), Err(error) =>   panic!("couldn't create dir '{}': {}", in_path .display(), error),}; }
if !rmsg_exists_dir(&out_path)    {match rmsg_create_dir(&out_path) {Ok(_) => println!("created: '{}'",out_path.display()), Err(error) =>   panic!("couldn't create dir '{}': {}", out_path.display(), error),}; }
if !rmsg_exists_dir(&ill_path)    {match rmsg_create_dir(&ill_path) {Ok(_) => println!("created: '{}'",ill_path.display()), Err(error) => println!("couldn't create dir '{}': {}", ill_path.display(), error),}; }

println!("Input-File: '{}'", in_filename.display());

if !rmsg_exists_file(&in_filename) {panic!("Error, input file not found '{}'", in_filename.display());}

match rmsg_core_logic(&in_filename)
    {
        Ok(stat)   => { println!("got {}",stat); Ok(()) },
        Err(error) => { println!("Error saving config: {:?}", error); Err(error) },
    }

}