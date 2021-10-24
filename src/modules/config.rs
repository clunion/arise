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
//! 0.1      | 2021-10-16 | Clunion   | creation
//! ___________________________________________________________________________________________________________________________
//!# Examples
//!```
//! ...
//!     
//!```
//! ___________________________________________________________________________________________________________________________
//!    


//___ DECLARATIONS OF SUBMODULES: _____________________________________________________________________________________________
//___ none ___

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________
use std::clone::Clone;

use std::path::PathBuf;

#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};


//___ CONSTANTS: ______________________________________________________________________________________________________________
pub (crate) const ARISE_FILE_EXTENSION: &str = "arise";
pub (crate) const SKIN_FILE_EXTENSION:  &str = "ini";

// Default values for configuration:
pub (crate) const DEFAULT_VERBOSITY:                   u8 = 0;
pub (crate) const DEFAULT_DEBUG_MODE:                 bool = false;
pub (crate) const DEFAULT_TEST_MODE:                  bool = false;
pub (crate) const DEFAULT_BASE_PATHPART:              &str = ".";
pub (crate) const DEFAULT_RES_PATHPART:               &str = "resources";
pub (crate) const DEFAULT_INP_PATHPART:               &str = "input";
pub (crate) const DEFAULT_OUT_PATHPART:               &str = "output";
pub (crate) const DEFAULT_SKIN_NAME:                  &str = "default";
pub (crate) const DEFAULT_INSTALL_SKIN_FOLDER:        &str = "C:\\Users\\YourName\\Documents\\Rainmeter\\Skins";
pub (crate) const DEFAULT_RAINMETER_EXE:              &str = "C:\\Program Files\\Rainmeter\\Rainmeter.exe";
pub (crate) const DEFAULT_RAINMETER_PARAM_REFRESHAPP: &str = "!RefreshApp";                               // see: https://forum.rainmeter.net/viewtopic.php?t=11627
pub (crate) const DEFAULT_RAINMETER_PARAM_MANAGE:     &str = "!Manage Skins <ConfigName> <SkinIniFile>";  
            
//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ MACROS: _________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: ________________________________________________________________________________________________________________
#[derive(Debug, Clone)]
pub(crate) struct AriseConfig 
{
    pub(crate) verbosity:                  u8,
    pub(crate) debug:                      bool,
    pub(crate) test:                       bool,
    pub(crate) base_pathpart:              PathBuf,
    pub(crate) res_pathpart:               PathBuf,
    pub(crate) inp_pathpart:               PathBuf,
    pub(crate) out_pathpart:               PathBuf,
    pub(crate) skin_name:                  PathBuf,
    pub(crate) arise_file_name:            PathBuf,
    pub(crate) skin_file_name:             PathBuf,
    pub(crate) install_skin_folder:        PathBuf,
    pub(crate) rainmeter_exe:              PathBuf,
    pub(crate) rainmeter_param_refreshapp: String,
    pub(crate) rainmeter_param_manage:     String,
}


//___ METHODS: ________________________________________________________________________________________________________________

//-- Arise-Config -------------------------------------------
impl <'lt_ariseconf> Default for AriseConfig
{
/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  default   
/// **`TYPE:       `**  method of AriseConfig   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **`               `** None   
/// **`RETURNS:    `** **`               `** None   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Initializes a AriseConfig struct with default values   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2021-10-13 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________
    fn default() -> Self 
    {
        AriseConfig 
        {
        verbosity:                  DEFAULT_VERBOSITY,
        debug:                      DEFAULT_DEBUG_MODE,
        test:                       DEFAULT_TEST_MODE,
        base_pathpart:              PathBuf::from(DEFAULT_BASE_PATHPART),
        res_pathpart:               PathBuf::from(DEFAULT_RES_PATHPART),
        inp_pathpart:               PathBuf::from(DEFAULT_INP_PATHPART),
        out_pathpart:               PathBuf::from(DEFAULT_OUT_PATHPART),
        skin_name:                  PathBuf::from(DEFAULT_SKIN_NAME),                                       // skin-name of the current Rainmeter-Skin to generate 
        arise_file_name:            PathBuf::from(format!("{}.{}",DEFAULT_SKIN_NAME,ARISE_FILE_EXTENSION)), // file-name of the generator-source file (.arise)
        skin_file_name:             PathBuf::from(format!("{}.{}",DEFAULT_SKIN_NAME,SKIN_FILE_EXTENSION)),  // file-name of the current Rainmeter-Skin (.ini) to generate 
        install_skin_folder:        PathBuf::from(DEFAULT_INSTALL_SKIN_FOLDER),
        rainmeter_exe:              PathBuf::from(DEFAULT_RAINMETER_EXE),
        rainmeter_param_refreshapp:  String::from(DEFAULT_RAINMETER_PARAM_REFRESHAPP),
        rainmeter_param_manage:      String::from(DEFAULT_RAINMETER_PARAM_MANAGE),
        }
    }
}

//-- Arise-Config -------------------------------------------
impl <'lt_ariseconf> AriseConfig 
{
/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  new   
/// **`TYPE:       `**  method of AriseConfig   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` <none>       `**    
/// **`RETURNS:    `** **` AriseConfig  `** a newly created struct   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Create an AriseConfig struct with default values   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2021-10-17 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________
#[allow(dead_code)]    
fn new() -> AriseConfig
    {
        AriseConfig 
        {
            verbosity:                  DEFAULT_VERBOSITY,
            debug:                      DEFAULT_DEBUG_MODE,
            test:                       DEFAULT_TEST_MODE,
            base_pathpart:              PathBuf::from(DEFAULT_BASE_PATHPART),
            res_pathpart:               PathBuf::from(DEFAULT_RES_PATHPART),
            inp_pathpart:               PathBuf::from(DEFAULT_INP_PATHPART),
            out_pathpart:               PathBuf::from(DEFAULT_OUT_PATHPART),
            skin_name:                  PathBuf::from(DEFAULT_SKIN_NAME),                                       // skin-name of the current Rainmeter-Skin to generate 
            arise_file_name:            PathBuf::from(format!("{}.{}",DEFAULT_SKIN_NAME,ARISE_FILE_EXTENSION)), // file-name of the generator-source file (.arise)
            skin_file_name:             PathBuf::from(format!("{}.{}",DEFAULT_SKIN_NAME,SKIN_FILE_EXTENSION)),  // file-name of the Rainmeter-skin (.ini) to generate 
            install_skin_folder:        PathBuf::from(DEFAULT_INSTALL_SKIN_FOLDER),
            rainmeter_exe:              PathBuf::from(DEFAULT_RAINMETER_EXE),
            rainmeter_param_refreshapp:  String::from(DEFAULT_RAINMETER_PARAM_REFRESHAPP),
            rainmeter_param_manage:      String::from(DEFAULT_RAINMETER_PARAM_MANAGE),
            }
    }
} // End of struct: AriseConfig


/// ___________________________________________________________________________________________________________________________
/// **`TESTMODULE: `** for config   
/// **`TYPE:       `** unit tests   
/// ___________________________________________________________________________________________________________________________
#[cfg(test)]
mod tests 
{
  use super::*;            // importing names from outer (for mod tests) scope

  use crate::config::AriseConfig;
  #[allow(unused_imports)]



  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `** test_default_values()   
  /// **`TYPE:       `** unit test   
  /// **`TESTS:      `** checks if method AriseConfig::default() indeed sets the DEFAULT-VALUES   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_default_values() 
  {
  let defaults = AriseConfig::default();
  
  assert_eq!(defaults.verbosity                  , DEFAULT_VERBOSITY);
  assert_eq!(defaults.debug                      , DEFAULT_DEBUG_MODE);
  assert_eq!(defaults.test                       , DEFAULT_TEST_MODE);
  assert_eq!(defaults.base_pathpart              , PathBuf::from(DEFAULT_BASE_PATHPART));
  assert_eq!(defaults.res_pathpart               , PathBuf::from(DEFAULT_RES_PATHPART));
  assert_eq!(defaults.inp_pathpart               , PathBuf::from(DEFAULT_INP_PATHPART));
  assert_eq!(defaults.out_pathpart               , PathBuf::from(DEFAULT_OUT_PATHPART));
  assert_eq!(defaults.skin_name                  , PathBuf::from(DEFAULT_SKIN_NAME));                                       // skin-name of the current Rainmeter-Skin to generate 
  assert_eq!(defaults.arise_file_name            , PathBuf::from(format!("{}.{}",DEFAULT_SKIN_NAME,ARISE_FILE_EXTENSION))); // file-name of the generator-source file (.arise)
  assert_eq!(defaults.skin_file_name             , PathBuf::from(format!("{}.{}",DEFAULT_SKIN_NAME,SKIN_FILE_EXTENSION)));  // file-name of the current Rainmeter-Skin (.ini) to generate 
  assert_eq!(defaults.install_skin_folder        , PathBuf::from(DEFAULT_INSTALL_SKIN_FOLDER));
  assert_eq!(defaults.rainmeter_exe              , PathBuf::from(DEFAULT_RAINMETER_EXE));
  assert_eq!(defaults.rainmeter_param_refreshapp , String::from(DEFAULT_RAINMETER_PARAM_REFRESHAPP));
  assert_eq!(defaults.rainmeter_param_manage     , String::from(DEFAULT_RAINMETER_PARAM_MANAGE));
  }

  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `** set_and_check_all_values()   
  /// **`TYPE:       `** unit test   
  /// **`TESTS:      `** checks if complete cycle (of storing configuration to file and reloading) returns all values intact   
  /// ___________________________________________________________________________________________________________________________
    #[test]
    fn set_and_check_all_values() 
    {
    let a_conf = AriseConfig::new();

    assert_eq!(a_conf.verbosity                  , DEFAULT_VERBOSITY);
    assert_eq!(a_conf.debug                      , DEFAULT_DEBUG_MODE);
    assert_eq!(a_conf.test                       , DEFAULT_TEST_MODE);
    assert_eq!(a_conf.base_pathpart              , PathBuf::from(DEFAULT_BASE_PATHPART));
    assert_eq!(a_conf.res_pathpart               , PathBuf::from(DEFAULT_RES_PATHPART));
    assert_eq!(a_conf.inp_pathpart               , PathBuf::from(DEFAULT_INP_PATHPART));
    assert_eq!(a_conf.out_pathpart               , PathBuf::from(DEFAULT_OUT_PATHPART));
    assert_eq!(a_conf.skin_name                  , PathBuf::from(DEFAULT_SKIN_NAME));                                       // skin-name of the current Rainmeter-Skin to generate 
    assert_eq!(a_conf.arise_file_name            , PathBuf::from(format!("{}.{}",DEFAULT_SKIN_NAME,ARISE_FILE_EXTENSION))); // file-name of the generator-source file (.arise)
    assert_eq!(a_conf.skin_file_name             , PathBuf::from(format!("{}.{}",DEFAULT_SKIN_NAME,SKIN_FILE_EXTENSION)));  // file-name of the current Rainmeter-Skin (.ini) to generate 
    assert_eq!(a_conf.install_skin_folder        , PathBuf::from(DEFAULT_INSTALL_SKIN_FOLDER));
    assert_eq!(a_conf.rainmeter_exe              , PathBuf::from(DEFAULT_RAINMETER_EXE));
    assert_eq!(a_conf.rainmeter_param_refreshapp , String::from(DEFAULT_RAINMETER_PARAM_REFRESHAPP));
    assert_eq!(a_conf.rainmeter_param_manage     , String::from(DEFAULT_RAINMETER_PARAM_MANAGE));
    }

} // End of: mod test
