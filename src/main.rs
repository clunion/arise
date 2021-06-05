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
// mod modules;                              // <dirname>
mod arise;                             // <filename>

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________
//use std::env;
use std::io;

use clap::Arg;

// use std::cmp::Ordering;

// use std::error::Error;
// use std::io::ErrorKind;

// use std::path::Path;
use std::path::PathBuf;

use arise::*; 

//___ CONSTANTS: ______________________________________________________________________________________________________________
const ARISE_FILE_EXTENSION:      &str = ".arise";
const RAINMETER_FILE_EXTENSION:  &str = ".ini";
const DEFAULT_SKIN_NAME:         &str = "StorageMon";

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
let     _bret: bool   = false;                                 // common boolean return value
let     res_path      = PathBuf::from("resources");
let     inp_path      = PathBuf::from("input");
let     out_path      = PathBuf::from("output");

let mut arise_filename = format!("{}{}",DEFAULT_SKIN_NAME,ARISE_FILE_EXTENSION);
let mut skin_filename  = format!("{}{}",DEFAULT_SKIN_NAME,RAINMETER_FILE_EXTENSION);          // Name of the current Rainmeter-Skin to generate 

// Parse the command line using clap:
let cmd_line = clap::App::new("Arise")
                   .version("0.1")
                   .author("Clunion <Christian.Lunau@gmx.de>")
                   .about("A RaInmeter Skin Evolver")
                   .arg(Arg::with_name("arisefile")                   // <--arise-File (=Source)-----------------------
                       .short("f")
                       .long("arisefile")
                       .value_name("ARISEFILE")
                       .help("Sets a specific input file as source.")
                       .takes_value(true))
                   .arg(Arg::with_name("verbosity")                    // <--VERBOSITY --------------------------------
                       .short("v")
                       .multiple(true)
                       .help("Sets the level of verbosity, more vs, more chatter."))
                   .arg(Arg::with_name("test-mode")                    // <--TEST-MODE---------------------------------
                       .help("Starts the program in testing mode.")
                       .short("t")
                       .long("test")
                       .takes_value(false))
                   .arg(Arg::with_name("debug-mode")                   // <--DEBUG-MODE--------------------------------
                       .short("d")
                       .long("debug")
                       .help("Starts the program in debug mode.")
                       .takes_value(false))
                   .get_matches();

// Get the name of a config-file, if supplied on command line, or defaults to config::INI_FILE_NAME
// let arise_filename = cmd_line.value_of("arisefile").unwrap_or(DEFAULT_SKIN_NAME);

println!("res_path:       {}", res_path.display());
println!("inp_path:       {}", inp_path.display());
println!("out_path:       {}", out_path.display());

// Check preconditions to run:
if !exists_dir(&res_path)  {match create_dir(&res_path) {Ok(_) => println!("created: '{}'",res_path.display()), Err(error) => panic!("couldn't create dir '{}': {}", res_path.display(), error),}; }
if !exists_dir(&inp_path)  {match create_dir(&inp_path) {Ok(_) => println!("created: '{}'",inp_path.display()), Err(error) => panic!("couldn't create dir '{}': {}", inp_path.display(), error),}; }
if !exists_dir(&out_path)  {match create_dir(&out_path) {Ok(_) => println!("created: '{}'",out_path.display()), Err(error) => panic!("couldn't create dir '{}': {}", out_path.display(), error),}; }


let mut inp_full_filename = PathBuf::from(&inp_path); 
        inp_full_filename.push(&arise_filename);

let mut out_full_filename = PathBuf::from(&out_path); 
        out_full_filename.push(&skin_filename);

println!("arise-filename:       {}", arise_filename);
println!("skin-filename:        {}", skin_filename);
println!("input-full-filename:  {}", inp_full_filename.display());
println!("output-full-filename: {}", out_full_filename.display());

if !exists_file(&inp_full_filename) {panic!("Error, input arise file not found '{}'", inp_full_filename.display());}

match core_logic(&res_path, &inp_full_filename, &out_full_filename)
    {
        Ok(stat)   => { println!("got {}",stat); Ok(()) },
        Err(error) => { println!("Error saving config: {:?}", error); Err(error) },
    }

}
