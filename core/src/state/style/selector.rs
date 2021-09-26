use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::string::ToString;

use crate::Specificity;

use bitflags::bitflags;


bitflags! {
    pub struct PseudoClasses: u8 {
        const HOVER = 1;
        const OVER = 1 << 1;
        const ACTIVE = 1 << 2;
        const FOCUS = 1 << 3;
        const DISABLED = 1 << 4;
        const CHECKED = 1 << 5;
        const SELECTED = 1 << 6;
        const CUSTOM = 1 << 7;
    }
}

impl Default for PseudoClasses {
    fn default() -> Self {
        PseudoClasses::empty()
    }
}

impl std::fmt::Display for PseudoClasses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        if self.contains(PseudoClasses::HOVER) {
            write!(f, ":hover")?;
        }
        if self.contains(PseudoClasses::OVER) {
            write!(f, ":over")?;
        }
        if self.contains(PseudoClasses::ACTIVE) {
            write!(f, ":active")?;
        }
        if self.contains(PseudoClasses::FOCUS) {
            write!(f, ":focus")?;
        }
        if self.contains(PseudoClasses::DISABLED) {
            write!(f, ":disabled")?;
        }
        if self.contains(PseudoClasses::CHECKED) {
            write!(f, ":checked")?;
        }
        if self.contains(PseudoClasses::SELECTED) {
            write!(f, ":selected")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Relation {
    None,
    Ancestor,
    Parent,
}

#[derive(Clone, Debug)]
pub struct Selector {
    pub id: Option<u64>,
    pub element: Option<String>,
    pub classes: HashSet<String>,
    pub pseudo_classes: PseudoClasses,
    pub relation: Relation,
    pub asterisk: bool,
}

impl Default for Selector {
    fn default() -> Selector {
        Selector {
            id: None,
            element: None,
            classes: HashSet::new(),
            pseudo_classes: PseudoClasses::empty(),
            relation: Relation::None,
            asterisk: false,
        }
    }
}

impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.asterisk {
            write!(f, "*")?;
        }

        if let Some(element) = &self.element {
            write!(f, "{}", element)?;
        }

        for class_name in self.classes.iter() {
            write!(f, ".{}", class_name)?;
        }

        write!(f, "{}", self.pseudo_classes);

        match self.relation {
            Relation::None => {}
            Relation::Ancestor => write!(f, " ")?,
            Relation::Parent => write!(f, ">")?,
        }

        Ok(())
    }
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            id: None,
            element: None,
            classes: HashSet::new(),
            pseudo_classes: PseudoClasses::empty(),
            relation: Relation::None,
            asterisk: false,
        }
    }

    pub fn element(element: &str) -> Self {
        //let mut s = DefaultHasher::new();
        //element.hash(&mut s);

        Selector {
            id: None,
            element: Some(element.to_owned()),
            classes: HashSet::new(),
            pseudo_classes: PseudoClasses::empty(),
            relation: Relation::None,
            asterisk: false,
        }
    }

    // TO BE REMOVED
    pub fn matches(&self, entity_selector: &Selector) -> bool {
        // Universal selector always matches
        if self.asterisk {
            if !self.pseudo_classes.is_empty()
                && !self.pseudo_classes.intersects(entity_selector.pseudo_classes)
            {
                return false;
            } else {
                return true;
            }
        }

        // Check for ID match
        if self.id.is_some() && self.id != entity_selector.id {
            return false;
        }

        // Check for element name match 
        if self.element.is_some() && self.element != entity_selector.element {
            return false;
        }

        // Check for classes match
        if !self.classes.is_subset(&entity_selector.classes) {
            return false;
        }

        if !self.pseudo_classes.is_empty()
            && !self.pseudo_classes.intersects(entity_selector.pseudo_classes)
        {
            return false;
        }

        if self.asterisk != entity_selector.asterisk {
            return false;
        }

        true
    }

    pub fn specificity(&self) -> Specificity {
        Specificity([
            if self.id.is_some() { 1 } else { 0 },
            (self.classes.len() + self.pseudo_classes.bits().count_ones() as usize) as u8,
            if self.element.is_some() { 1 } else { 0 },
        ])
    }

    pub fn id(mut self, id: &str) -> Self {
        let mut s = DefaultHasher::new();
        id.hash(&mut s);
        self.id = Some(s.finish());
        self
    }

    pub fn class(mut self, class: &str) -> Self {
        self.classes.insert(class.to_string());
        self
    }

    pub fn replace_class(&mut self, old: &str, new: &str) -> &mut Self {
        self.classes.remove(old);
        self.classes.insert(new.to_string());

        self
    }

    pub fn set_id(&mut self, id: &str) -> &mut Self {
        let mut s = DefaultHasher::new();
        id.hash(&mut s);
        self.id = Some(s.finish());
        self
    }

    pub fn set_element(&mut self, element: &str) -> &mut Self {
        self.element = Some(element.to_owned());
        self
    }
}

impl PartialEq for Selector {
    fn eq(&self, other: &Selector) -> bool {
        self.matches(other)
    }
}
