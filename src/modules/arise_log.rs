//! ___________________________________________________________________________________________________________________________
//! **`PROJECT:    `** arise    
//! **`HOME:       `** [arise on GitHub](https://github.com/clunion/arise)    
//! **`SYNOPSIS:   `** A Rainmeter (tm) Skin Evolver, a parameterized generator for rainmeter ini-files       
//! ___________________________________________________________________________________________________________________________
//! **`FILE:       `** arise_log.rs 🦀   
//! **`DESCRIPTION:`** configuration and additional functions for logging. Requires the crates log and flexi_logger   
//! ___________________________________________________________________________________________________________________________
//! **`LICENSE:    `**   
//! Copyright 2020 by Christian Lunau (clunion), Julian Lunau (someone-out-there) and Jaron Lunau (endless-means).   
//! MIT-License, see LICENSE.md file    
//! ___________________________________________________________________________________________________________________________
//! VERSION: | DATE:      | AUTHOR:   | CHANGES:   
//! :---     | :---       | :---:     | :---   
//! 0.1      | 2020-07-10 | Clunion   | creation   
//! ___________________________________________________________________________________________________________________________
//! **`TODO:       `**   
//! * add more tests
//! ___________________________________________________________________________________________________________________________
//!    

//___ DECLARATIONS OF SUBMODULES: _____________________________________________________________________________________________
//___ none ___

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________
use flexi_logger::{Record, DeferredNow};
use yansi::{Color, Style};

use std::path::MAIN_SEPARATOR;

//use crate::modules::*;      // crate::<dirname>::*

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
/// **`FUNCTION:   `**  basename   
/// **`TYPE:       `**  public, common helper function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` path          `** a str, which is expected to contain a path with a file name   
/// **`RETURNS:    `** **` Result -->    `** a str containing only the file name   
/// **`            `** **`     or -->    `** gives back the provided str   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// small helper function, extracting the basename of a given &str. For occasions where the path can not be provided in a 
/// a variable of type Path.
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-07-08 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// * nothing    
/// ___________________________________________________________________________________________________________________________
pub(crate) fn basename(path: &str) -> &str 
{
let mut pieces = path.rsplitn(2, MAIN_SEPARATOR);
match pieces.next() 
    {
    Some(p) => p,
    None    => path,
    }
}


/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  console_line_format   
/// **`TYPE:       `**  public, callback for flexi_logger   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` w             `** (output stream?)   
/// **`            `** **` now           `** the timestamp of the moment the log macro is called   
/// **`            `** **` record        `** a struct with the log metadata and the message   
/// **`RETURNS:    `** **` Result -->    `** OK()   
/// **`            `** **`     or -->    `** Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// logline-formatter that produces log lines with timestamp and file location, like
/// <br>
/// ```[15:25:01  INFO       file.rs[  26] This is an info message```
/// <br>
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 0.1     | 2020-07-08 | Clunion   | initial version, based on flexi_logger::colored_opt_format   
/// 0.2     | 2021-11-06 | Clunion   | umpf, crate 'chrono' got dropped, thus flexi_logger switched to std::time, which has no format for times.    
/// ___________________________________________________________________________________________________________________________
pub(crate) fn console_line_format( w: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record, ) -> Result<(), std::io::Error> 
{
let level = record.level();
let arise_style: Style;

let  error_style: Style = Style::new(Color::Red).bold().italic();  // todo: move to a one-time initializer or change into static
let   warn_style: Style = Style::new(Color::Yellow).italic()    ;  // todo: move to a one-time initializer or change into static
let   info_style: Style = Style::new(Color::Cyan)               ;  // todo: move to a one-time initializer or change into static
let  debug_style: Style = Style::new(Color::Default)            ;  // todo: move to a one-time initializer or change into static
let  trace_style: Style = Style::new(Color::Blue)               ;  // todo: move to a one-time initializer or change into static

match level 
    {
    log::Level::Error => {arise_style = error_style;},
    log::Level::Warn  => {arise_style =  warn_style;},
    log::Level::Info  => {arise_style =  info_style;},
    log::Level::Debug => {arise_style = debug_style;},
    log::Level::Trace => {arise_style = trace_style;},
    };

write!( w, 
        "{} {:5}{:>18}[{:4}] {}",
        now.now().to_string().split('.').next().unwrap_or("time unknown"), //.fomat("%H:%M:%S"),
        arise_style.paint(record.level()),
        basename(record.file().unwrap_or("<unnamed>")),
        record.line().unwrap_or(0),
        &record.args()
      )
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  file_line_format   
/// **`TYPE:       `**  public, callback for flexi_logger   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` w             `** (output stream?)   
/// **`            `** **` now           `** the timestamp of the moment the log macro is called   
/// **`            `** **` record        `** a struct with the log metadata and the message   
/// **`RETURNS:    `** **` Result -->    `** OK()   
/// **`            `** **`     or -->    `** Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// A logline-formatter that produces log lines like
/// <br>
/// ```[2016-01-13 15:25:01.640870]  INFO  src/foo/bar.rs[  26] This is an info message```
/// <br>
/// i.e. with timestamp and file location.
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 0.1     | 2020-07-08 | Clunion   | initial version, based on flexi_logger::detailed_format   
/// 0.2     | 2021-11-06 | Clunion   | crate 'chrono' got dropped, thus flexi_logger switched to std::time, which has no format for times.    
/// ___________________________________________________________________________________________________________________________
pub(crate) fn file_line_format( w: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record, ) -> Result<(), std::io::Error> 
{
write!( w,
        "{:28}{:5}{:>32}[{:4}]: {}",
        now.now().to_string().split('+').next().unwrap_or("time unknown"), //.fmt("%Y-%m-%d %H:%M:%S%.6f %:z"),
        record.level(),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        &record.args()
      )
}



// /// ___________________________________________________________________________________________________________________________
// /// **`TESTMODULE: `** for arise_log   
// /// **`TYPE:       `** unit test functions   
// /// ___________________________________________________________________________________________________________________________
// #[cfg(test)]
// mod tests 
// {
//   // importing names from outer (for mod tests) scope:
//   use super::cursors::*;
//   
//   /// ___________________________________________________________________________________________________________________________
//   /// **`FUNCTION:   `**  test_load()   
//   /// **`TYPE:       `**  unit test function   
//   /// ___________________________________________________________________________________________________________________________
//   /// **`PARAMETER:  `** **`<none>        `**    
//   /// **`RETURNS:    `** **`<none>        `**    
//   /// ___________________________________________________________________________________________________________________________
// 
//   #[test]
//   fn test_load() 
//   {
//     let result = load();
//     assert!(result.is_ok());
//   }
//   
// } // End of: mod test



