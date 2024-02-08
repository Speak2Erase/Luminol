#![feature(min_specialization)]
#![allow(non_upper_case_globals)]

// Editor specific types
pub mod rmxp;

// Shared structs with the same layout
mod shared;

mod option_vec;

mod rgss_structs;

pub mod helpers;

pub mod commands;

pub use helpers::*;
pub use option_vec::OptionVec;
pub use rgss_structs::{Color, Table1, Table2, Table3, Tone};

#[derive(Debug, Default)]
pub struct ChangeDetection<T> {
    data: T,
    modified: bool,
}

impl<T> ChangeDetection<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            modified: false,
        }
    }

    pub fn modified(&self) -> bool {
        self.modified
    }

    /// Use sparingly!
    #[cfg_attr(feature = "debug-modification-locations", track_caller)]
    pub fn bypass_change_detection(&mut self) -> &mut T {
        #[cfg(feature = "debug-modification-locations")]
        {
            let caller = std::panic::Location::caller();
            let type_name = std::any::type_name::<T>();
            println!("{type_name}:{self:p} change detection was bypassed at {caller}");
        }

        &mut self.data
    }

    /// Use sparingly!
    #[cfg_attr(feature = "debug-modification-locations", track_caller)]
    pub fn reset_modified(&mut self) {
        #[cfg(feature = "debug-modification-locations")]
        {
            let caller = std::panic::Location::caller();
            let type_name = std::any::type_name::<T>();
            println!("{type_name}:{self:p} modified flag was reset at {caller}");
        }

        self.modified = false;
    }

    #[cfg_attr(feature = "debug-modification-locations", track_caller)]
    pub fn set_modified(&mut self) {
        #[cfg(feature = "debug-modification-locations")]
        {
            let caller = std::panic::Location::caller();
            let type_name = std::any::type_name::<T>();
            println!("{type_name}:{self:p} modified flag was set at {caller}");
        }

        self.modified = true;
    }

    pub fn to_inner(self) -> T {
        self.data
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T> std::ops::Deref for ChangeDetection<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> std::ops::DerefMut for ChangeDetection<T> {
    #[cfg_attr(feature = "debug-modification-locations", track_caller)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[cfg(feature = "debug-modification-locations")]
        {
            let caller = std::panic::Location::caller();
            let type_name = std::any::type_name::<T>();
            println!("{type_name}:{self:p} was mutably dereferenced at at {caller}");
        }

        self.modified = true;
        &mut self.data
    }
}

impl<T: Clone> Clone for ChangeDetection<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            modified: false,
        }
    }
}

pub mod rpg {
    pub use crate::rmxp::*;
    pub use crate::shared::*;

    macro_rules! basic_wrapper {
    ($($parent:ident, $child:ident),* $(,)?) => {
        $(
            pub type $parent = crate::ChangeDetection<Vec<$child>>;
         )*
    };
}

    basic_wrapper! {
        Actors, Actor,
        Animations, Animation,
        Armors, Armor,
        Classes, Class,
        CommonEvents, CommonEvent,
        Enemies, Enemy,
        Items, Item,
        Scripts, Script,
        Skills, Skill,
        States, State,
        Tilesets, Tileset,
        Troops, Troop,
        Weapons, Weapon,
    }

    pub type CSystem = crate::ChangeDetection<System>;
    pub type CMap = crate::ChangeDetection<Map>;
    pub type MapInfos = crate::ChangeDetection<std::collections::HashMap<usize, MapInfo>>;
}

pub use shared::BlendMode;

pub type Path = Option<camino::Utf8PathBuf>;
