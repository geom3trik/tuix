#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;
use crate::{BuildHandler, Event, EventHandler, Length, WindowEvent};
use crate::{PropSet, State};

use crate::state::style::*;

use crate::widgets::{Element, Button};


pub struct Dialogue {
    
}

impl Dialogue {
    pub fn new() -> Self {
        Dialogue {

        }
    }
}

impl BuildHandler for Dialogue {

}

impl EventHandler for Dialogue {
    
}