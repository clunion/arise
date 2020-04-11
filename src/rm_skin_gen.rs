/*
## ---------------------------------------------------------------------------------------------------------------------------
## PROJECT:                      Rainmeter Skin Generator
## HOME:
## ---------------------------------------------------------------------------------------------------------------------------
## FILE:     main.rs
## SYNOPSIS: main, start and entry point of the program
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## Rainmeter (tm) Skin Generator, a parameterized generator for rainmeter skins (ini-files)
##----------------------------------------------------------------------------------------------------------------------------
## COPYRIGHT:
## Copyright 2018-20xx by Christian Lunau.
##
## This sourcecode is free software; you can redistribute it and/or
## modify it under the terms of the GNU Library General Public
## License as published by the Free Software Foundation; either
## version 2 of the License, or (at your option) any later version.
##
## This sourcecode is distributed in the hope that it will be useful,
## but WITHOUT ANY WARRANTY; without even the implied warranty of
## MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
## Library General Public License for more details.
##
## You should have received a copy of the GNU Library General Public
## License along with this library; if not, write to the Free Software
## Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307
## USA.
##
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:  DATE:       AUTHOR: CHANGES:
## 0.1       2018-04-03  CLu     creation
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
##    - everything
## ---------------------------------------------------------------------------------------------------------------------------
*/

//--- MODULES EXTERNAL: ------------------------------------------------------------------------------------------------------
extern crate rand;

//--- MODULES LOCAL: ---------------------------------------------------------------------------------------------------------
//--- none ---

//--- MODULE USES: -----------------------------------------------------------------------------------------------------------
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

// old C-Style section comments, to be replaced by Rust-equivalents:
//--- DEFINES: ---------------------------------------------------------------------------------------------------------------
//--- none ---

//--- TYPE DEFINITIONS: ------------------------------------------------------------------------------------------------------
//--- none ---

//--- CLASS DECLARATIONS: ----------------------------------------------------------------------------------------------------
//--- none ---

//--- GLOBAL VARS: -----------------------------------------------------------------------------------------------------------
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

const OFFSET_VARAIBLES_BEGIN  : &str = "<offset varaibles begin>";
const OFFSET_VARAIBLES_END    : &str = "<offset varaibles end>";

const SECTION_HEADER_BEGIN    : &str = "<header begin>";
const SECTION_HEADER_END      : &str = "<header end>";
const SECTION_MEASURES_BEGIN  : &str = "<measures begin>";
const SECTION_MEASURES_END    : &str = "<measures end>";
const SECTION_METERS_BEGIN    : &str = "<meters begin>";
const SECTION_METERS_END      : &str = "<meters end>";
const SECTION_FOOTER_BEGIN    : &str = "<footer begin>";
const SECTION_FOOTER_END      : &str = "<footer end>";


//--- EXPORTS: ---------------------------------------------------------------------------------------------------------------
//--- none ---

//--- PROTOTYPES: ------------------------------------------------------------------------------------------------------------
//--- callbacks:


/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   cat
## TYPE:       simple function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  path
## RETURNS:    io::Result<String>
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## A simple implementation of `% cat path`
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:  DATE:       AUTHOR: CHANGES:
## 1.0       2018        ?       initial version, based on an example from 'Rust By Example'
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:     rework it to a more usefull utility function
## ---------------------------------------------------------------------------------------------------------------------------
*/
#[allow(dead_code)]
fn cat(path: &Path) -> io::Result<String>
{
let mut f = File::open(path)?;
let mut s = String::new();
match f.read_to_string(&mut s)
    {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
    }
}

/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   echo
## TYPE:       simple function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  str      - text content to be written to the newly created file
##             path     - full path and filename of the file to be created
## RETURNS:    io::Result<()>
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## A simple implementation of `% echo s > path`
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:  DATE:       AUTHOR: CHANGES:
## 1.0       2018        ?       initial version, based on an example from 'Rust By Example'
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:     rework it to a more usefull utility function
## ---------------------------------------------------------------------------------------------------------------------------
*/
#[allow(dead_code)]
fn echo(s: &str, path: &Path) -> io::Result<()>
{
let mut f = File::create(path)?;
f.write_all(s.as_bytes())
}

/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   touch
## TYPE:       simple function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  path
## RETURNS:    io::Result<()>
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## A simple implementation of `% touch path` (ignores existing files)
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:  DATE:       AUTHOR: CHANGES:
## 1.0       2018        ?       initial version, based on an example from 'Rust By Example'
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:     rework it to a more usefull utility function
## ---------------------------------------------------------------------------------------------------------------------------
*/
#[allow(dead_code)]
fn touch(path: &Path) -> io::Result<()>
{
match OpenOptions::new().create(true).write(true).open(path)
    {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
    }
}

/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   rmsg_create_dir
## TYPE:       simple function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  dir_p          - reference to a PathBuf containing the name of the directory to be created
## RETURNS:    io::Result<()> - status
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## Creates a directory, returns `io::Result<()>`
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:  DATE:       AUTHOR: CHANGES:
## 1.0       2018        ?       initial version, based on an example from 'Rust By Example'
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:     rework it to a more usefull utility function
## ---------------------------------------------------------------------------------------------------------------------------
*/
fn rmsg_create_dir(dir_p: &PathBuf) -> io::Result<()>
{
match fs::create_dir(dir_p)
    {
    Ok(_)      => {println!("OK, created dir: {}", dir_p.display());Ok(())},
    Err(ref error) if error.kind() == ErrorKind::AlreadyExists => {println!("OK, dir {} already exists.", dir_p.display());Ok(())},
    Err(error) => {println!("couldn't create dir '{}': {}", dir_p.display(), error); Err(error)},
    }
}

/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   rmsg_create_path
## TYPE:       simple function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  new_path_p     - name of the (multiple) directories to be created
## RETURNS:    io::Result<()> - status
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## Recursively create directories, returns `io::Result<()>`
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:  DATE:       AUTHOR: CHANGES:
## 1.0       2018        CLu     initial version, based on an example from 'Rust By Example'
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:     rework it to a more usefull utility function
## ---------------------------------------------------------------------------------------------------------------------------
*/
#[allow(dead_code)]
fn rmsg_create_path(new_path_p: &PathBuf) -> io::Result<()>
{
match fs::create_dir_all(new_path_p)
    {
    Ok(_)    => {println!("OK, all dirs created: '{}'",new_path_p.display());                                             Ok(()) },
    Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {println!("OK, path {} already exists.", new_path_p.display()); Ok(()) },
    Err(error) => {println!("Error, creating dirs '{}' failed with: {}",new_path_p.display(), error);          Err(error)},
    }
}

/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   rmsg_exists_file
## TYPE:       simple function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  file_p          - name of the file to be checked for existance
## RETURNS:    bool   -> true  = path exists
##                       false = path does not exist
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## Checks if the given directory exists, returns `io::Result<()>`
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:  DATE:       AUTHOR: CHANGES:
## 1.0       2018        ?       initial version, based on an example from 'Rust By Example'
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
## ---------------------------------------------------------------------------------------------------------------------------
*/
#[allow(dead_code)]
fn rmsg_exists_file(file_p: &PathBuf) -> bool
{
let retval = Path::new(file_p).exists();

if retval {println!("OK, file exists: '{}'"          ,file_p.display());}
else      {println!("NOPE, file does not exist: '{}'",file_p.display());}

return retval;
}



/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   rmsg_exists_dir
## TYPE:       simple function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  path_p          - name of the directory to be checked for existance
## RETURNS:    bool   -> true  = path exists
##                       false = path does not exist
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## Checks if the given directory exists, returns `io::Result<()>`
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:  DATE:       AUTHOR: CHANGES:
## 1.0       2018        ?       initial version, based on an example from 'Rust By Example'
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
## ---------------------------------------------------------------------------------------------------------------------------
*/
#[allow(dead_code)]
fn rmsg_exists_dir(path_p: &PathBuf) -> bool
{
let retval = Path::new(path_p).exists();

if retval {println!("OK, dir exists: '{}'"          ,path_p.display());}
else      {println!("NOPE, dir does not exist: '{}'",path_p.display());}

return retval;
}

/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   rmsg_open_file
## TYPE:       simple function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  file_name       - Reference to a PathBuf containing the name of the Fle to be opened, including path
## RETURNS:    std::fs::File   - a handle of the successfully opened file
##             std::io::Error  - status in case of a failure
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## Opens the file with the given file_name. The File has to exist, oterwise an error is returned.
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:  DATE:       AUTHOR: CHANGES:
## 1.0       2018-05-23  CLu     initial version, based on an example from 'Rust By Example'
## 1.1       2020-01-17  CLu     changed: parameter to PathBuf refference, return-types to File and io::Error,
##                               added    println outputs
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
## ---------------------------------------------------------------------------------------------------------------------------
*/
#[allow(dead_code)]
fn rmsg_open_file(file_name: &PathBuf) -> Result<std::fs::File, std::io::Error>
{
let _file = match File::open(file_name)
    {
    Ok(f)    => { println!("OK, file opened: {}", file_name.display());
                  return  Ok(f)},
    Err(error) => { println!("Error, couldn't open file '{}': {}", file_name.display(), error);
                  return Err(error)},
    };
}

/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   rmsg_read_file_fully
## TYPE:       simple function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  file_path      - Reference to a PathBuf containing the name of the Fle to be read, including path
## RETURNS:    String         - on Success: A String containing the whole content of the file
##             std::io::Error - status in case of a failure
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## Reads the whole file with the given file_name into a String.
## The File has to exist, oterwise an error is returned.
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:  DATE:       AUTHOR: CHANGES:
## 1.1       2020-01-17  CLu     created, initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
## ---------------------------------------------------------------------------------------------------------------------------
*/
fn rmsg_read_file_fully(file_path: &PathBuf) -> Result<String, std::io::Error>
{
let mut data = String::new();

let mut file = File::open(file_path)?;

match file.read_to_string(&mut data)
    {
    Ok(_)    => return Ok(data),
    Err(err) => return Err(err),
    }
}

/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   main
## TYPE:       entry point,
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  ?
## RETURNS:    ?
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## The one and only main: startup and entry point of this program
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:    DATE:       AUTHOR: CHANGES:
## 1.0         2018        CLu     initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
## ---------------------------------------------------------------------------------------------------------------------------
*/
#[allow(unused_variables)]
fn main()
{
let     _bret: bool      = false;                             // common bollean return value
let     res_path     = PathBuf::from("ressources");
let     gen_path     = PathBuf::from("generator");
let     out_path     = PathBuf::from("output");
let     ill_path     = PathBuf::from("il:legal");             // Illegaler, nicht-anlegbarer Pfad (nur zum testen der Fehlerfall-Behandlung)
let     non_path     = PathBuf::from("dies dir gibs nich");   // Pfad existiret nicht             (nur zum testen der Fehlerfall-Behandlung)

let     skin_name    = "StorageMon";                          // Name des aktuell zu generierenden Rainmeter-Skins

let     gen_filename = PathBuf::from("StorageMon.rm_skin_gen");
let     out_filename = PathBuf::from("StorageMon.ini");

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

let mut offset_varaibles_begin_cnt  : i32 = 0;
let mut offset_varaibles_end_cnt    : i32 = 0;

let mut section_header_begin_cnt    : i32 = 0;
let mut section_header_end_cnt      : i32 = 0;
let mut section_measures_begin_cnt  : i32 = 0;
let mut section_measures_end_cnt    : i32 = 0;
let mut section_meters_begin_cnt    : i32 = 0;
let mut section_meters_end_cnt      : i32 = 0;
let mut section_footer_begin_cnt    : i32 = 0;
let mut section_footer_end_cnt      : i32 = 0;


let mut  in_filename = PathBuf::from(&gen_path); in_filename.push(&gen_filename);

// Check preconditions to run:
if !rmsg_exists_dir(&non_path)    {println!("wird jetzt auch nicht angelegt"); }
if !rmsg_exists_dir(&res_path)    {match rmsg_create_dir(&res_path) {Ok(_) => println!("created: '{}'",res_path.display()), Err(error) =>   panic!("couldn't create dir '{}': {}", res_path.display(), error),}; }
if !rmsg_exists_dir(&gen_path)    {match rmsg_create_dir(&gen_path) {Ok(_) => println!("created: '{}'",gen_path.display()), Err(error) =>   panic!("couldn't create dir '{}': {}", gen_path.display(), error),}; }
if !rmsg_exists_dir(&out_path)    {match rmsg_create_dir(&out_path) {Ok(_) => println!("created: '{}'",out_path.display()), Err(error) =>   panic!("couldn't create dir '{}': {}", out_path.display(), error),}; }
if !rmsg_exists_dir(&ill_path)    {match rmsg_create_dir(&ill_path) {Ok(_) => println!("created: '{}'",ill_path.display()), Err(error) => println!("couldn't create dir '{}': {}", ill_path.display(), error),}; }

println!("Input-File: '{}'", in_filename.display());

if !rmsg_exists_file(&in_filename) {panic!("Error, input file not found '{}'", in_filename.display());}

let s_gen = match rmsg_read_file_fully(&in_filename)
    {
    Ok(s_gen)   => {println!("OK, read from file {}", in_filename.display());s_gen},
    Err(error)    => {panic!("Read from file {} failed with {}",in_filename.display(),error)},
    };

let s_gen_len      = s_gen.len();
let s_gen_capacity = s_gen.capacity();

println!("the generator-sourcecode has len={}, capacity={}", s_gen_len,s_gen_capacity);

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

    if s_cur_line.contains(OFFSET_VARAIBLES_BEGIN  ) {offset_varaibles_begin_cnt += 1;}
    if s_cur_line.contains(OFFSET_VARAIBLES_END    ) {offset_varaibles_end_cnt   += 1;}

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
println!("{:>26} = {:3}",format!("\"{}\"",OFFSET_VARAIBLES_BEGIN ), offset_varaibles_begin_cnt  );
println!("{:>26} = {:3}",format!("\"{}\"",OFFSET_VARAIBLES_END   ), offset_varaibles_end_cnt    );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_HEADER_BEGIN   ), section_header_begin_cnt    );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_HEADER_END     ), section_header_end_cnt      );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_MEASURES_BEGIN ), section_measures_begin_cnt  );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_MEASURES_END   ), section_measures_end_cnt    );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_METERS_BEGIN   ), section_meters_begin_cnt    );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_METERS_END     ), section_meters_end_cnt      );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_FOOTER_BEGIN   ), section_footer_begin_cnt    );
println!("{:>26} = {:3}",format!("\"{}\"",SECTION_FOOTER_END     ), section_footer_end_cnt      );


/*
println!("`cat a/c/b.txt`");
match cat(&Path::new("a/c/b.txt"))
    {
    Err(error) => println!("! {:?}", error.kind()),
    Ok(s) => println!("> {}", s),
    }

println!("`ls a`");
// Read the contents of a directory, returns `io::Result<Vec<Path>>`
match fs::read_dir("a")
    {
    Err(error) => println!("! {:?}", error.kind()),
    Ok(paths) => for path in paths {
                                   println!("> {:?}", path.unwrap().path());
                                   },
    }

println!("`rm a/c/e.txt`");
// Remove a file, returns `io::Result<()>`
match fs::remove_file("a/c/e.txt")
    {
    Ok(_)    => print!("created:{}\n",new_dir),
    Err(error) => panic!("couldn't create dir {}: {}", new_dir, error),
    }


println!("`rmdir a/c/d`");
// Remove an empty directory, returns `io::Result<()>`
match fs::remove_dir("a/c/d")
    {
    Ok(_)    => print!("created:{}\n",new_dir),
    Err(error) => panic!("couldn't create dir {}: {}", new_dir, error),
    }
*/

// file_handle1 goes out of scope, and the new_file1 file gets closed.
}
