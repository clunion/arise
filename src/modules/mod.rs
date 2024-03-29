//! ___________________________________________________________________________________________________________________________
//! **`PROJECT:    `** Shardoverse    
//! **`HOME:       `** [Shardoverse on GitHub](https://github.com/clunion/shardoverse)    
//! **`SYNOPSIS:   `** A Roguelike Peer-to-Peer Multi Player Dungeon Explorer Game written in Rust    
//! ___________________________________________________________________________________________________________________________
//! **`FILE:       `** mod.rs 🦀   
//! **`DESCRIPTION:`** this file contains the bindings/paths-to-the-sources of project shardoverse   
//! The mod.rs-file is (at least, seems to be) handled in a special way. We use it solely for providing access to our own submodules.   
//! ___________________________________________________________________________________________________________________________
//! **`LICENSE:    `**   
//! Copyright 2020 by Christian Lunau (clunion), Julian Lunau (someone-out-there) and Jaron Lunau (endless-means).   
//! MIT-License, see LICENSE.md file    
//! ___________________________________________________________________________________________________________________________
//! VERSION: | DATE:      | AUTHOR:   | CHANGES:   
//! :---     | :---       | :---:     | :---   
//! 0.1      | 2020-06-03 | Clunion   | creation   
//! ___________________________________________________________________________________________________________________________
//! **`TODO:       `**   
//! * understand what is really happening with the Rust module access
//! ___________________________________________________________________________________________________________________________
//!    

//___ Activate additional WARNINGS:
#![ warn
(
   anonymous_parameters          ,
   missing_copy_implementations  ,
   missing_debug_implementations ,
   missing_docs                  ,
   nonstandard_style             ,
// rust_2018_idioms              ,
   single_use_lifetimes          ,
   trivial_casts                 ,
   trivial_numeric_casts         ,
   unreachable_pub               ,
   unused_extern_crates          ,
   unused_qualifications         ,
   variant_size_differences      ,
)]


//___ DECLARATIONS OF SUBMODULES: _____________________________________________________________________________________________
pub(crate) mod config;        // <filename>
pub(crate) mod core_logic;    // <filename>
pub(crate) mod arise_log;     // <filename>

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

