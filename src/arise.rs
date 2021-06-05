#![deny(clippy::all)]
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
// mod central_core;                         // <filename>
// mod arise;                                // <filename>

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________

//use std::env;
use std::io;
use std::io::prelude::*;

use std::fs;
//use std::fs::{File, OpenOptions};
use std::fs::File;
use std::fs::OpenOptions;

use std::path::Path;
use std::path::PathBuf;

// use std::cmp::Ordering;

// use std::error::Error;
use std::io::ErrorKind;


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
//___ none ___

//___ METHODS: ________________________________________________________________________________________________________________
//___ none ___



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
    Ok(_)      => {println!("OK, created dir: {}", dir_p.display());Ok(())},
    Err(ref error) if error.kind() == ErrorKind::AlreadyExists => {println!("OK, dir {} already exists.", dir_p.display());Ok(())},
    Err(error) => {println!("couldn't create dir '{}': {}", dir_p.display(), error); Err(error)},
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
    Ok(_)    => {println!("OK, all dirs created: '{}'",new_path_p.display());                                             Ok(()) },
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

if retval {println!("OK, file exists: '{}'"          ,file_p.display());}
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

if retval {println!("OK, dir exists: '{}'"          ,path_p.display());}
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
pub(crate) fn open_file(file_name: &Path) -> Result<std::fs::File, std::io::Error>
{
let _file = match File::open(file_name)
    {
    Ok(f)    => { println!("OK, file opened: {}", file_name.display());
                  return  Ok(f)},
    Err(error) => { println!("Error, couldn't open file '{}': {}", file_name.display(), error);
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
/// **`PARAMETER:  `** **` file_path_p    `** - full path and filename of the file to be read   
/// **`RETURNS:    `** **` Result -->     `** OK(status flag: true = successful, false = failed)   
/// **`            `** **`     or -->     `** Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Reads the whole file with the given file_name into a String.   
/// The File has to exist, otherwise an error is returned.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-01-17 | Clunion   | created, initial version   
/// ___________________________________________________________________________________________________________________________
pub(crate) fn core_logic(res_path_p: &Path, inp_full_filename_p: &Path, out_full_filename_p: &Path) -> Result<bool, io::Error>
{
let     _bret: bool  = false;                                 // common boolean return value

// counter for defined Literals, which are found:
let mut comment_singleline_cnt      : i32 = 0;
let mut comment_multiline_begin_cnt : i32 = 0;
let mut comment_multiline_end_cnt   : i32 = 0;

let mut operator_assign_cnt         : i32 = 0;
let mut operator_plus_cnt           : i32 = 0;
let mut operator_minus_cnt          : i32 = 0;

let mut key_name_begin_cnt          : i32 = 0;
let mut key_name_end_cnt            : i32 = 0;

let mut multiplier_list_begin_cnt   : i32 = 0;
let mut multiplier_list_end_cnt     : i32 = 0;

let mut offset_variables_begin_cnt  : i32 = 0;
let mut offset_variables_end_cnt    : i32 = 0;

let mut section_header_begin_cnt    : i32 = 0;
let mut section_header_end_cnt      : i32 = 0;
let mut section_measures_begin_cnt  : i32 = 0;
let mut section_measures_end_cnt    : i32 = 0;
let mut section_meters_begin_cnt    : i32 = 0;
let mut section_meters_end_cnt      : i32 = 0;
let mut section_footer_begin_cnt    : i32 = 0;
let mut section_footer_end_cnt      : i32 = 0;

println!(">>> START OF: core_logic({},{},{})", res_path_p.display(), inp_full_filename_p.display(), out_full_filename_p.display());

let s_gen = match read_file_fully(&inp_full_filename_p)
    {
    Ok(s_gen)    => {println!("OK, read from file {}", inp_full_filename_p.display());s_gen},
    Err(error)   => {panic!("Read from file {} failed with {}",inp_full_filename_p.display(),error)},
    };

let s_gen_len      = s_gen.len();
let s_gen_capacity = s_gen.capacity();

println!("the generator-source code has len={}, capacity={}", s_gen_len,s_gen_capacity);

let s_gen_lines = s_gen.lines();

println!("-----------------------------------------------------------");
for s_cur_line in s_gen_lines
    {
    if s_cur_line.contains(COMMENT_SINGLELINE     ) {comment_singleline_cnt      += 1;}
    if s_cur_line.contains(COMMENT_MULTILINE_BEGIN) {comment_multiline_begin_cnt += 1;}
    if s_cur_line.contains(COMMENT_MULTILINE_END  ) {comment_multiline_end_cnt   += 1;}

    if s_cur_line.contains(OPERATOR_ASSIGN         ) {operator_assign_cnt        += 1;}
    if s_cur_line.contains(OPERATOR_PLUS           ) {operator_plus_cnt          += 1;}
    if s_cur_line.contains(OPERATOR_MINUS          ) {operator_minus_cnt         += 1;}

    if s_cur_line.contains(KEY_NAME_BEGIN          ) {key_name_begin_cnt         += 1;}
    if s_cur_line.contains(KEY_NAME_END            ) {key_name_end_cnt           += 1;}

    if s_cur_line.contains(MULTIPLIER_LIST_BEGIN   ) {multiplier_list_begin_cnt  += 1;}
    if s_cur_line.contains(MULTIPLIER_LIST_END     ) {multiplier_list_end_cnt    += 1;}

    if s_cur_line.contains(OFFSET_VARIABLES_BEGIN  ) {offset_variables_begin_cnt += 1;}
    if s_cur_line.contains(OFFSET_VARIABLES_END    ) {offset_variables_end_cnt   += 1;}

    if s_cur_line.contains(SECTION_HEADER_BEGIN    ) {section_header_begin_cnt   += 1;}
    if s_cur_line.contains(SECTION_HEADER_END      ) {section_header_end_cnt     += 1;}
    if s_cur_line.contains(SECTION_MEASURES_BEGIN  ) {section_measures_begin_cnt += 1;}
    if s_cur_line.contains(SECTION_MEASURES_END    ) {section_measures_end_cnt   += 1;}
    if s_cur_line.contains(SECTION_METERS_BEGIN    ) {section_meters_begin_cnt   += 1;}
    if s_cur_line.contains(SECTION_METERS_END      ) {section_meters_end_cnt     += 1;}
    if s_cur_line.contains(SECTION_FOOTER_BEGIN    ) {section_footer_begin_cnt   += 1;}
    if s_cur_line.contains(SECTION_FOOTER_END      ) {section_footer_end_cnt     += 1;}

    println!("{}", s_cur_line)
    }

println!("-----------------------------------------------------------");
println!("Found Literals, Operators and Keys with counts of:");
println!("{:>26} = {:3}",format!("\"{}\"",COMMENT_SINGLELINE     ), comment_singleline_cnt      );
println!("{:>26} = {:3}",format!("\"{}\"",COMMENT_MULTILINE_BEGIN), comment_multiline_begin_cnt );
println!("{:>26} = {:3}",format!("\"{}\"",COMMENT_MULTILINE_END  ), comment_multiline_end_cnt   );
println!("{:>26} = {:3}",format!("\"{}\"",OPERATOR_ASSIGN        ), operator_assign_cnt         );
println!("{:>26} = {:3}",format!("\"{}\"",OPERATOR_PLUS          ), operator_plus_cnt           );
println!("{:>26} = {:3}",format!("\"{}\"",OPERATOR_MINUS         ), operator_minus_cnt          );
println!("{:>26} = {:3}",format!("\"{}\"",KEY_NAME_BEGIN         ), key_name_begin_cnt          );
println!("{:>26} = {:3}",format!("\"{}\"",KEY_NAME_END           ), key_name_end_cnt            );
println!("{:>26} = {:3}",format!("\"{}\"",MULTIPLIER_LIST_BEGIN  ), multiplier_list_begin_cnt   );
println!("{:>26} = {:3}",format!("\"{}\"",MULTIPLIER_LIST_END    ), multiplier_list_end_cnt     );
println!("{:>26} = {:3}",format!("\"{}\"",OFFSET_VARIABLES_BEGIN ), offset_variables_begin_cnt  );
println!("{:>26} = {:3}",format!("\"{}\"",OFFSET_VARIABLES_END   ), offset_variables_end_cnt    );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_HEADER_BEGIN   ), section_header_begin_cnt    );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_HEADER_END     ), section_header_end_cnt      );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_MEASURES_BEGIN ), section_measures_begin_cnt  );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_MEASURES_END   ), section_measures_end_cnt    );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_METERS_BEGIN   ), section_meters_begin_cnt    );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_METERS_END     ), section_meters_end_cnt      );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_FOOTER_BEGIN   ), section_footer_begin_cnt    );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_FOOTER_END     ), section_footer_end_cnt      );

    let path = Path::new("write_output.png");

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) 
        {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
        };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
     match file.write_all(s_gen.as_bytes()) 
        {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
        Ok(_)    => println!("successfully wrote to {}", path.display()),
        }

Ok(true)
// file_handle1 goes out of scope, and the new_file1 file gets closed.
}
