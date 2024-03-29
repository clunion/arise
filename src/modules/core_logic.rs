#![deny(clippy::all)]
//#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(clippy::suspicious_else_formatting)]
#![allow(clippy::collapsible_if)]

//! ___________________________________________________________________________________________________________________________
//! **`PROJECT:    `** ARISE - A RaInmeter Skin Evolver    
//! **`HOME:       `** [arise on GitHub](https://github.com/clunion/arise)    
//! **`SYNOPSIS:   `** A Rainmeter (tm) Skin Evolver, a parameterized generator for rainmeter ini-files   
//! ___________________________________________________________________________________________________________________________
//! **`FILE:       `** arise.rs 🦀   
//! **`DESCRIPTION:`** the core of th skin generator, contains the logic and some helper functions, ...   
//! ___________________________________________________________________________________________________________________________
//! **`LICENSE:    `**   
//! Copyright 2020 by Christian Lunau (clunion)   
//! MIT-License, see LICENSE.md file    
//! ___________________________________________________________________________________________________________________________
//! VERSION: | DATE:      | AUTHOR:   | CHANGES:   
//! :---     | :---       | :---:     | :---   
//! 0.1      | 2021-05-23 | Clunion   | creation
//! ___________________________________________________________________________________________________________________________
//!# Examples
//!```
//! Hmm, this is a logic module, have to think about what examples could be here...
//!     xxx
//!```
//! ___________________________________________________________________________________________________________________________
//!    

//___ DECLARATIONS OF SUBMODULES TO INCLUDE: __________________________________________________________________________________
// mod modules;                              // <dirname>

use crate::modules::config::*;            // crate::<filename>::*

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________

//use std::env;
use std::io;
use std::io::prelude::*;

use std::path::PathBuf;

use std::fs;
//use std::fs::{File, OpenOptions};
use std::fs::File;
use std::fs::OpenOptions;

use std::path::Path;

// use std::cmp::Ordering;

use std::error::Error;
use std::io::ErrorKind;

#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};


//___ CONSTANTS: ______________________________________________________________________________________________________________
const COMMENT_SINGLELINE      : &str = ";";
const COMMENT_MULTILINE_BEGIN : &str = "/*";
const COMMENT_MULTILINE_END   : &str = "*/";

const OPERATOR_ASSIGN         : &str = ":=";
const OPERATOR_PLUS           : &str = "+";
const OPERATOR_MINUS          : &str = "-";

const KEY_NAME_BEGIN          : &str = "<:";
const KEY_NAME_END            : &str = ":>";

const MULTIPLIER_LIST_BEGIN   : &str = "<multiplier list begin>";
const MULTIPLIER_LIST_END     : &str = "<multiplier list end>";

const OFFSET_VARIABLES_BEGIN  : &str = "<offset variables begin>";
const OFFSET_VARIABLES_END    : &str = "<offset variables end>";

const SECTION_HEADER_BEGIN    : &str = "<header begin>";
const SECTION_HEADER_END      : &str = "<header end>";
const SECTION_MEASURES_BEGIN  : &str = "<measures begin>";
const SECTION_MEASURES_END    : &str = "<measures end>";
const SECTION_METERS_BEGIN    : &str = "<meters begin>";
const SECTION_METERS_END      : &str = "<meters end>";
const SECTION_FOOTER_BEGIN    : &str = "<footer begin>";
const SECTION_FOOTER_END      : &str = "<footer end>";

//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ MACROS: _________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: ________________________________________________________________________________________________________________
#[derive(Debug, Clone)]
struct AriseBucket   
{
// counter for defined Literals, which are found:
    comment_singleline_cnt      : i32,
    comment_multiline_begin_cnt : i32,
    comment_multiline_end_cnt   : i32,
    
    operator_assign_cnt         : i32,
    operator_plus_cnt           : i32,
    operator_minus_cnt          : i32,
    
    key_name_begin_cnt          : i32,
    key_name_end_cnt            : i32,
    
    multiplier_list_begin_cnt   : i32,
    multiplier_list_end_cnt     : i32,
    
    offset_variables_begin_cnt  : i32,
    offset_variables_end_cnt    : i32,
    
    section_header_begin_cnt    : i32,
    section_header_end_cnt      : i32,
    section_measures_begin_cnt  : i32,
    section_measures_end_cnt    : i32,
    section_meters_begin_cnt    : i32,
    section_meters_end_cnt      : i32,
    section_footer_begin_cnt    : i32,
    section_footer_end_cnt      : i32,
    arise_in: String,  // will be shortened from the head      by each section-function (empty at end)
    skin_out: String,  // will get newly evolved code appended by each section-function (empty at start)
}

//___ METHODS: ________________________________________________________________________________________________________________

impl  AriseBucket
{
/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  new   
/// **`TYPE:       `**  method of AriseBucket   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` <none>       `**    
/// **`RETURNS:    `** **` AriseBucket  `** a newly created struct   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Create an AriseBucket struct with default values (which are empty at first, thus None)   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2021-11-03 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________
#[allow(dead_code)]    
fn new() -> AriseBucket
    {
        AriseBucket
        {
        // counter for defined Literals when found:
        comment_singleline_cnt     : 0,
        comment_multiline_begin_cnt: 0,
        comment_multiline_end_cnt  : 0,

        operator_assign_cnt        : 0,
        operator_plus_cnt          : 0,
        operator_minus_cnt         : 0,

        key_name_begin_cnt         : 0,
        key_name_end_cnt           : 0,

        multiplier_list_begin_cnt  : 0,
        multiplier_list_end_cnt    : 0,

        offset_variables_begin_cnt : 0,
        offset_variables_end_cnt   : 0,

        section_header_begin_cnt   : 0,
        section_header_end_cnt     : 0,
        section_measures_begin_cnt : 0,
        section_measures_end_cnt   : 0,
        section_meters_begin_cnt   : 0,
        section_meters_end_cnt     : 0,
        section_footer_begin_cnt   : 0,
        section_footer_end_cnt     : 0,
        arise_in: "uninitialized".to_string(), // ugly, todo: replace with Option (?)
        skin_out: "uninitialized".to_string(), // ugly, todo: replace with Option (?)
        }
    }

} // End of impl: AriseBucket



/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  cat   
/// **`TYPE:       `**  simple function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` path_p         `** - path of the file to be printed   
/// **`RETURNS:    `** **`     Result --> `** - OK()   
/// **`            `** **`         or --> `** - Error(Error-Message)   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// prints the contents of the given file on the console.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2018-##-## | Clunion   | initial version, based on an example from 'Rust By Example'   
/// ___________________________________________________________________________________________________________________________
#[allow(dead_code)]
pub(crate) fn cat(path: &Path) -> io::Result<String>
{
let mut f = File::open(path)?;
let mut s = String::new();
match f.read_to_string(&mut s)
    {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
    }
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  echo   
/// **`TYPE:       `**  simple function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` str_p          `** - text content to be written to the newly created file   
/// **`            `** **` path_p         `** - full path and filename of the file to be created
/// **`RETURNS:    `** **`     Result --> `** - OK()   
/// **`            `** **`         or --> `** - Error(Error-Message)   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// A simple implementation of `% echo s > path`   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2018-##-## | Clunion   | initial version, based on an example from 'Rust By Example'   
/// ___________________________________________________________________________________________________________________________
#[allow(dead_code)]
pub(crate) fn echo(s: &str, path: &Path) -> io::Result<()>
{
let mut f = File::create(path)?;
f.write_all(s.as_bytes())
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  touch   
/// **`TYPE:       `**  simple function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` path_p         `** - full path and filename of the file to be touched   
/// **`RETURNS:    `** **`     Result --> `** - OK()   
/// **`            `** **`         or --> `** - Error(Error-Message)   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// A simple implementation of `% touch path` (ignores existing files)   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2018-##-## | Clunion   | initial version, based on an example from 'Rust By Example'   
/// ___________________________________________________________________________________________________________________________
#[allow(dead_code)]
pub(crate) fn touch(path: &Path) -> io::Result<()>
{
match OpenOptions::new().create(true).write(true).open(path)
    {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
    }
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  create_dir   
/// **`TYPE:       `**  simple function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` dir_p          `** - reference to a Path containing the name of the directory to be created   
/// **`RETURNS:    `** **`     Result --> `** - OK()   
/// **`            `** **`         or --> `** - Error(Error-Message)   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// A simple implementation of `% touch path` (ignores existing files)   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2018-##-## | Clunion   | initial version, based on an example from 'Rust By Example'   
/// ___________________________________________________________________________________________________________________________
pub(crate) fn create_dir(dir_p: &Path) -> io::Result<()>
{
match fs::create_dir(dir_p)
    {
    Ok(_)      => {debug!("OK, created dir: {}", dir_p.display());Ok(())},
    Err(ref error) if error.kind() == ErrorKind::AlreadyExists => {error!("OK, dir {} already exists.", dir_p.display());Ok(())},
    Err(error) => {error!("couldn't create dir '{}': {}", dir_p.display(), error); Err(error)},
    }
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  create_path   
/// **`TYPE:       `**  simple function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` new_path_p     `** - name of the (multiple) directories to be created   
/// **`RETURNS:    `** **`     Result --> `** - OK()   
/// **`            `** **`         or --> `** - Error(Error-Message)   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// A simple implementation of `% touch path` (ignores existing files)   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2018-##-## | Clunion   | initial version, based on an example from 'Rust By Example'   
/// ___________________________________________________________________________________________________________________________
#[allow(dead_code)]
pub(crate) fn create_path(new_path_p: &Path) -> io::Result<()>
{
match fs::create_dir_all(new_path_p)
    {
    Ok(_)    => {debug!("OK, all dirs created: '{}'",new_path_p.display());                                             Ok(()) },
    Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {println!("OK, path {} already exists.", new_path_p.display()); Ok(()) },
    Err(error) => {println!("Error, creating dirs '{}' failed with: {}",new_path_p.display(), error);          Err(error)},
    }
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  exists_file   
/// **`TYPE:       `**  simple function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` file_p           `** - name of the file to be checked for existence   
/// **`RETURNS:    `** **`     bool -> true `** - file exists   
/// **`            `** **`          -> false`** - file does not exist   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Checks if the given directory exists, returns `io::Result<()>`   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2018-##-## | Clunion   | initial version, based on an example from 'Rust By Example'   
/// ___________________________________________________________________________________________________________________________
#[allow(dead_code)]
pub(crate) fn exists_file(file_p: &Path) -> bool
{
let retval = Path::new(file_p).exists();

if retval {debug!("OK, file exists: '{}'"          ,file_p.display());}
else      {println!("NOPE, file does not exist: '{}'",file_p.display());}

retval
}



/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  exists_dir   
/// **`TYPE:       `**  simple function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` path_p           `** - name of the directory to be checked for existence   
/// **`RETURNS:    `** **`     bool -> true `** - path exists   
/// **`            `** **`          -> false`** - path does not exist   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Checks if the given directory exists, returns `io::Result<()>`   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2018-##-## | Clunion   | initial version, based on an example from 'Rust By Example'   
/// ___________________________________________________________________________________________________________________________
#[allow(dead_code)]
pub(crate) fn exists_dir(path_p: &Path) -> bool
{
let retval = Path::new(path_p).exists();

if retval {debug!("OK, dir exists: '{}'"          ,path_p.display());}
else      {println!("NOPE, dir does not exist: '{}'",path_p.display());}

retval
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  open_file   
/// **`TYPE:       `**  simple function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` file_name_p       `** - Reference to a Path containing the name of the Fle to be opened, including path   
/// **`RETURNS:    `** **` std::fs::File  -> `** - a handle of the successfully opened file   
/// **`            `** **` std::io::Error -> `** - Error(Error-Message) status in case of a failure   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Opens the file with the given file_name. The File has to exist, otherwise an error is returned.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2018-05-23 | Clunion   | initial version, based on an example from 'Rust By Example'   
/// 1.1     | 2020-01-17 | Clunion   | changed: parameter to PathBuf reference, return-types to File and io::Error, added println outputs     
/// ___________________________________________________________________________________________________________________________
#[allow(dead_code)]
pub(crate) fn open_file(file_name: &Path) -> Result<File, std::io::Error>
{
let _file = match File::open(file_name)
    {
    Ok(f)    => { debug!("OK, file opened: {}", file_name.display());
                  return  Ok(f)},
    Err(error) => { error!("Error, couldn't open file '{}': {}", file_name.display(), error);
                  return Err(error)},
    };
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  read_file_fully   
/// **`TYPE:       `**  simple function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` file_path_p    `** - full path and filename of the file to be read   
/// **`RETURNS:    `** **`     String --> `** - on Success: A String containing the whole content of the file   
/// **`            `** **`         or --> `** - Error(Error-Message) status in case of a failure   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Reads the whole file with the given file_name into a String.   
/// The File has to exist, otherwise an error is returned.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-01-17 | Clunion   | created, initial version   
/// ___________________________________________________________________________________________________________________________
pub(crate) fn read_file_fully(file_path: &Path) -> Result<String, std::io::Error>
{
let mut data = String::new();

let mut file = File::open(file_path)?;

match file.read_to_string(&mut data)
    {
    Ok(_)    => Ok(data),
    Err(err) => Err(err),
    }
}


/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  core_logic   
/// **`TYPE:       `**  central core logic function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` conf_p         `** - arise-config to be processed   
/// **`RETURNS:    `** **` Result -->     `** OK(status flag: true = successful, false = failed)   
/// **`            `** **`     or -->     `** Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Does the business logic of arise, based on the provided configuration struct.   
/// The File has to exist, otherwise an error is returned.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-01-17 | Clunion   | created, initial version   
/// ___________________________________________________________________________________________________________________________
//-> Result<AriseBucket, Box<dyn Error>>
//pub(crate) fn core_logic(conf_p: &AriseConfig) -> Result<bool, io::Error>
pub(crate) fn core_logic(conf_p: &AriseConfig) -> Result<bool, Box<dyn Error>>
{
let mut arise : AriseBucket = AriseBucket::new();

// construct the full paths+filenames to work on:
let mut inp_full_filename = PathBuf::from(&conf_p.base_pathpart); 
        inp_full_filename.push(&conf_p.inp_pathpart);
        inp_full_filename.push(&conf_p.arise_file_name);

let mut out_full_filename = PathBuf::from(&conf_p.base_pathpart); 
        out_full_filename.push(&conf_p.out_pathpart);
        out_full_filename.push(&conf_p.skin_file_name);

debug!("--- Config-Values: ---");
debug!("skin-name (command line):   {}",   conf_p.skin_name.display());
debug!("base_pathpart:              {}",   conf_p.base_pathpart.display());
debug!("res_pathpart:               {}",   conf_p.res_pathpart.display());
debug!("inp_pathpart:               {}",   conf_p.inp_pathpart.display());
debug!("out_pathpart:               {}",   conf_p.out_pathpart.display());
debug!("arise-filename:             {}",   conf_p.arise_file_name.display());
debug!("skin-filename:              {}",   conf_p.skin_file_name.display());
debug!("install_skin_folder:        {}",   conf_p.install_skin_folder.display());
debug!("rainmeter_exe:              {}",   conf_p.rainmeter_exe.display());
debug!("rainmeter_param_refreshapp: {:?}", conf_p.rainmeter_param_refreshapp);
debug!("rainmeter_param_manage:     {:?}", conf_p.rainmeter_param_manage    );
debug!("input-full-filename:        {}",   inp_full_filename.display());
debug!("output-full-filename:       {}",   out_full_filename.display());


// Check preconditions to run:
assert!(exists_file(&inp_full_filename), "Error, input arise file not found '{}'", inp_full_filename.display());

arise.arise_in = match read_file_fully(&inp_full_filename)
{
    Err(why)    => {error!("Read file {} failed with {}", inp_full_filename.display(),why); return Err(why.into())},
    Ok(s_arise) => {debug!("Read file {} OK."           , inp_full_filename.display()); s_arise},
};

debug!("-----------------------------------------------------------");

match build_metainfo(arise)
{
    Err(why)           => {error!("couldn't evolve skin metadata: {}", why); return Err(why)}
    Ok(arise_metainfo) => { arise = arise_metainfo }  // be careful here, it's tricky...
};
debug!("ok, lengths now: arise-in {:4}, skin-out {:4}", arise.arise_in.len(), arise.skin_out.len());

match build_skin_header(arise)
{
    Err(why)           => {error!("couldn't evolve skin header: {}", why); return Err(why)}
    Ok(arise_header)   => { arise = arise_header }  // be careful here, it's tricky...
};
debug!("ok, lengths now: arise-in {:4}, skin-out {:4}", arise.arise_in.len(), arise.skin_out.len());

match build_skin_body(arise)
{
    Err(why)           => {error!("couldn't evolve skin body: {}", why); return Err(why)}
    Ok(arise_body)     => { arise = arise_body }  // be careful here, it's tricky...
};
debug!("ok, lengths now: arise-in {:4}, skin-out {:4}", arise.arise_in.len(), arise.skin_out.len());

match build_skin_footer(arise)
{
    Err(why)           => {error!("couldn't evolve skin footer: {}", why); return Err(why)}
    Ok(arise_footer)   => { arise = arise_footer }  // be careful here, it's tricky...
};
debug!("ok, lengths now: arise-in {:4}, skin-out {:4}", arise.arise_in.len(), arise.skin_out.len());


debug!("-----------------------------------------------------------");
debug!("Amounts of Literals, Operators and Keys found:");
debug!("{:>26} = {:3}",format!("\"{}\"",COMMENT_SINGLELINE     ), arise.comment_singleline_cnt      );
debug!("{:>26} = {:3}",format!("\"{}\"",COMMENT_MULTILINE_BEGIN), arise.comment_multiline_begin_cnt );
debug!("{:>26} = {:3}",format!("\"{}\"",COMMENT_MULTILINE_END  ), arise.comment_multiline_end_cnt   );
debug!("{:>26} = {:3}",format!("\"{}\"",OPERATOR_ASSIGN        ), arise.operator_assign_cnt         );
debug!("{:>26} = {:3}",format!("\"{}\"",OPERATOR_PLUS          ), arise.operator_plus_cnt           );
debug!("{:>26} = {:3}",format!("\"{}\"",OPERATOR_MINUS         ), arise.operator_minus_cnt          );
debug!("{:>26} = {:3}",format!("\"{}\"",KEY_NAME_BEGIN         ), arise.key_name_begin_cnt          );
debug!("{:>26} = {:3}",format!("\"{}\"",KEY_NAME_END           ), arise.key_name_end_cnt            );
debug!("{:>26} = {:3}",format!("\"{}\"",MULTIPLIER_LIST_BEGIN  ), arise.multiplier_list_begin_cnt   );
debug!("{:>26} = {:3}",format!("\"{}\"",MULTIPLIER_LIST_END    ), arise.multiplier_list_end_cnt     );
debug!("{:>26} = {:3}",format!("\"{}\"",OFFSET_VARIABLES_BEGIN ), arise.offset_variables_begin_cnt  );
debug!("{:>26} = {:3}",format!("\"{}\"",OFFSET_VARIABLES_END   ), arise.offset_variables_end_cnt    );
debug!("{:>26} = {:3}",format!("\"{}\"",SECTION_HEADER_BEGIN   ), arise.section_header_begin_cnt    );
debug!("{:>26} = {:3}",format!("\"{}\"",SECTION_HEADER_END     ), arise.section_header_end_cnt      );
debug!("{:>26} = {:3}",format!("\"{}\"",SECTION_MEASURES_BEGIN ), arise.section_measures_begin_cnt  );
debug!("{:>26} = {:3}",format!("\"{}\"",SECTION_MEASURES_END   ), arise.section_measures_end_cnt    );
debug!("{:>26} = {:3}",format!("\"{}\"",SECTION_METERS_BEGIN   ), arise.section_meters_begin_cnt    );
debug!("{:>26} = {:3}",format!("\"{}\"",SECTION_METERS_END     ), arise.section_meters_end_cnt      );
debug!("{:>26} = {:3}",format!("\"{}\"",SECTION_FOOTER_BEGIN   ), arise.section_footer_begin_cnt    );
debug!("{:>26} = {:3}",format!("\"{}\"",SECTION_FOOTER_END     ), arise.section_footer_end_cnt      );
    

    // Open a file in write-only mode, returns `io::Result<File>`
let mut file = match File::create(&out_full_filename) 
    {
    Err(why) => {error!("couldn't create {}: {}", out_full_filename.display(), why); return Err(why.into())},
    Ok(file) => {debug!("ok, created {}"        , out_full_filename.display()); file}
    };

// Write the full contents of generated rainmeter-ini-file to the skin-file, return io::Result<()> if successful
 match file.write_all(arise.skin_out.as_bytes()) 
    {
    Err(why) => {error!("couldn't write to {}: {}", out_full_filename.display(), why); Err(why.into())}
    Ok(_)    => {debug!("successfully wrote to {}", out_full_filename.display());      Ok(true) }
    }

// file -handle goes out of scope and the file gets closed.
}


 
/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  ``build_metainfo``   
/// **`TYPE:       `**  local, common function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` arise_p       `** Arise-bucket to be processed, contains (remainder of input and start of output)   
/// **`RETURNS:    `** **` Result -->    `** - OK(status flag: true = successful, false = failed)   
/// **`            `** **`     or -->    `** - Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Processes the metainfo-section of the arise-input, evolves it into a part of the rainmeter-skin output.   
/// The input gets shortened by the processed section, the output is extended with the generated Rainmeter-ini-code.
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2021-11-06 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________

fn build_metainfo(mut arise_p: AriseBucket) -> Result<AriseBucket, Box<dyn Error>>
{
let metainfo : String = "; -- Metainfo-Text --\n".to_owned();

let s_arise_lines = arise_p.arise_in.lines();
let mut line_num = 0;

trace!("-----------------------------------------------------------");
for s_cur_line in s_arise_lines
    {                                                  
    line_num += 1;    
    debug!("analyzing line:[{:4}] {}", line_num, s_cur_line);

    if s_cur_line.contains(COMMENT_SINGLELINE      ) {arise_p.comment_singleline_cnt     += 1; debug!("[{:4}] found COMMENT_SINGLELINE     ", line_num);}
    if s_cur_line.contains(COMMENT_MULTILINE_BEGIN ) {arise_p.comment_multiline_begin_cnt+= 1; debug!("[{:4}] found COMMENT_MULTILINE_BEGIN", line_num);}
    if s_cur_line.contains(COMMENT_MULTILINE_END   ) {arise_p.comment_multiline_end_cnt  += 1; debug!("[{:4}] found COMMENT_MULTILINE_END  ", line_num);}
    if s_cur_line.contains(OPERATOR_ASSIGN         ) {arise_p.operator_assign_cnt        += 1; debug!("[{:4}] found OPERATOR_ASSIGN        ", line_num);}
    if s_cur_line.contains(OPERATOR_PLUS           ) {arise_p.operator_plus_cnt          += 1; debug!("[{:4}] found OPERATOR_PLUS          ", line_num);}
    if s_cur_line.contains(OPERATOR_MINUS          ) {arise_p.operator_minus_cnt         += 1; debug!("[{:4}] found OPERATOR_MINUS         ", line_num);}
    if s_cur_line.contains(KEY_NAME_BEGIN          ) {arise_p.key_name_begin_cnt         += 1; debug!("[{:4}] found KEY_NAME_BEGIN         ", line_num);}
    if s_cur_line.contains(KEY_NAME_END            ) {arise_p.key_name_end_cnt           += 1; debug!("[{:4}] found KEY_NAME_END           ", line_num);}
    if s_cur_line.contains(MULTIPLIER_LIST_BEGIN   ) {arise_p.multiplier_list_begin_cnt  += 1; debug!("[{:4}] found MULTIPLIER_LIST_BEGIN  ", line_num);}
    if s_cur_line.contains(MULTIPLIER_LIST_END     ) {arise_p.multiplier_list_end_cnt    += 1; debug!("[{:4}] found MULTIPLIER_LIST_END    ", line_num);}
    if s_cur_line.contains(OFFSET_VARIABLES_BEGIN  ) {arise_p.offset_variables_begin_cnt += 1; debug!("[{:4}] found OFFSET_VARIABLES_BEGIN ", line_num);}
    if s_cur_line.contains(OFFSET_VARIABLES_END    ) {arise_p.offset_variables_end_cnt   += 1; debug!("[{:4}] found OFFSET_VARIABLES_END   ", line_num);}
    if s_cur_line.contains(SECTION_HEADER_BEGIN    ) {arise_p.section_header_begin_cnt   += 1; debug!("[{:4}] found SECTION_HEADER_BEGIN   ", line_num);}
    if s_cur_line.contains(SECTION_HEADER_END      ) {arise_p.section_header_end_cnt     += 1; debug!("[{:4}] found SECTION_HEADER_END     ", line_num);}
    if s_cur_line.contains(SECTION_MEASURES_BEGIN  ) {arise_p.section_measures_begin_cnt += 1; debug!("[{:4}] found SECTION_MEASURES_BEGIN ", line_num);}
    if s_cur_line.contains(SECTION_MEASURES_END    ) {arise_p.section_measures_end_cnt   += 1; debug!("[{:4}] found SECTION_MEASURES_END   ", line_num);}
    if s_cur_line.contains(SECTION_METERS_BEGIN    ) {arise_p.section_meters_begin_cnt   += 1; debug!("[{:4}] found SECTION_METERS_BEGIN   ", line_num);}
    if s_cur_line.contains(SECTION_METERS_END      ) {arise_p.section_meters_end_cnt     += 1; debug!("[{:4}] found SECTION_METERS_END     ", line_num);}
    if s_cur_line.contains(SECTION_FOOTER_BEGIN    ) {arise_p.section_footer_begin_cnt   += 1; debug!("[{:4}] found SECTION_FOOTER_BEGIN   ", line_num);}
    if s_cur_line.contains(SECTION_FOOTER_END      ) {arise_p.section_footer_end_cnt     += 1; debug!("[{:4}] found SECTION_FOOTER_END     ", line_num);}

    }

arise_p.skin_out = metainfo;

Ok(arise_p)
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  ``build_skin_header``   
/// **`TYPE:       `**  local, common function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` arise_p       `** Arise-bucket to be processed, contains (remainder of input and start of output)   
/// **`RETURNS:    `** **` Result -->    `** - OK(status flag: true = successful, false = failed)   
/// **`            `** **`     or -->    `** - Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Processes the skin_header-section of the arise-input, evolves it into a part of the rainmeter-skin output.   
/// The input gets shortened by the processed section, the output is extended with the generated Rainmeter-ini-code.
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2021-11-06 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________

fn build_skin_header(mut arise_p: AriseBucket) -> Result<AriseBucket, Box<dyn Error>>
{
let header : String = 
  "; --- Skin Header-Start ---\n".to_owned()
+ "[Rainmeter]\n"
+ "Update=10000\n"
+ "\n"
+ "[Metadata]\n"
+ "Name=AriseWorld\n"
+ "Author=clunion\n"
+ "Information=Shows a \"Hello, World!\"-kind text display\n"
+ "Version=0.1\n"
+ "License=Creative Commons Attribution - Non - Commercial - Share Alike 3.0\n"
+ "; --- Skin Header-End -----\n"
+ "\n";

arise_p.skin_out = format!("{}\n{}", arise_p.skin_out, header);

Ok(arise_p)
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  ``build_skin_body``   
/// **`TYPE:       `**  local, common function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` arise_p       `** Arise-bucket to be processed, contains (remainder of input and start of output)   
/// **`RETURNS:    `** **` Result -->    `** - OK(status flag: true = successful, false = failed)   
/// **`            `** **`     or -->    `** - Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Processes the skin_body-section of the arise-input, evolves it into a part of the rainmeter-skin output.   
/// The input gets shortened by the processed section, the output is extended with the generated Rainmeter-ini-code.
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2021-11-06 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________

fn build_skin_body(mut arise_p: AriseBucket) -> Result<AriseBucket, Box<dyn Error>>
{
let body : String = 
  "; --- Skin Body-Start ---\n".to_owned()
+ "[SimpleMeter]\n"
+ "Meter=String\n"
+ "Text=\" Arise, World!\"\n"
+ "AntiAlias=1\n"
+ "FontSize=40\n"
+ "FontFace=SegoeUI\n"
+ "FontColor=200,220,255\n"
+ "SolidColor=64,64,64,128,1\n"
+ "; --- Skin Body-End -----\n"
+ "\n";

arise_p.skin_out = format!("{}\n{}", arise_p.skin_out, body);

Ok(arise_p)
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  ``build_skin_footer``   
/// **`TYPE:       `**  local, common function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` arise_p       `** Arise-bucket to be processed, contains (remainder of input and start of output)   
/// **`RETURNS:    `** **` Result -->    `** - OK(status flag: true = successful, false = failed)   
/// **`            `** **`     or -->    `** - Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Processes the skin_footer-section of the arise-input, evolves it into a part of the rainmeter-skin output.   
/// The input gets shortened by the processed section, the output is extended with the generated Rainmeter-ini-code.
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2021-11-06 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________


fn build_skin_footer(mut arise_p: AriseBucket) -> Result<AriseBucket, Box<dyn Error>>
{
let footer : String = 
  "; --- Skin Footer-Start ---\n".to_owned()
+ "; --- Skin Footer-End -----\n"
+ "\n";

arise_p.skin_out = format!("{}\n{}", arise_p.skin_out, footer);

Ok(arise_p)
}
