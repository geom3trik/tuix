use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::string::ToString;

use crate::Specificity;

// #[derive(Clone, PartialEq, Eq, Hash, Debug)]
// pub enum PseudoClass {
//     None,
//     Hover,
//     Over,
//     Active,
//     Focus,
//     Enabled,
//     Disabled,
//     Checked,
// }

// 0 - Hover
// 1 - Over
// 2 - Active
// 3 - Focus
// 4 - Enabled
// 5 - Disabled
// 6 - Checked
// 7 - Unassigned

#[derive(Debug, Clone)]
pub struct PseudoClasses(u8);

impl Default for PseudoClasses {
    fn default() -> Self {
        PseudoClasses(0)
    }
}

impl std::fmt::Display for PseudoClasses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.get_hover() {
            write!(f, ":hover")?;
        }
        if self.get_over() {
            write!(f, ":over")?;
        }
        if self.get_active() {
            write!(f, ":active")?;
        }
        if self.get_focus() {
            write!(f, ":focus")?;
        }
        if self.get_enabled() {
            write!(f, ":enabled")?;
        }
        if self.get_disabled() {
            write!(f, ":disabled")?;
        }
        if self.get_checked() {
            write!(f, ":checked")?;
        }

        Ok(())
    }
}

impl PseudoClasses {
    pub fn new() -> Self {
        PseudoClasses(0)
    }

    pub fn set_hover(&mut self, flag: bool) {
        if flag {
            self.0 |= 1;
        } else {
            self.0 &= !1;
        }
    }

    pub fn set_over(&mut self, flag: bool) {
        if flag {
            self.0 |= (1 << 1);
        } else {
            self.0 &= !(1 << 1);
        }
    }

    pub fn set_active(&mut self, flag: bool) {
        if flag {
            self.0 |= (1 << 2);
        } else {
            self.0 &= !(1 << 2);
        }
    }

    pub fn set_focus(&mut self, flag: bool) {
        if flag {
            self.0 |= (1 << 3);
        } else {
            self.0 &= !(1 << 3);
        }
    }

    pub fn set_enabled(&mut self, flag: bool) {
        if flag {
            self.0 |= (1 << 4);
        } else {
            self.0 &= !(1 << 4);
        }
    }

    pub fn set_disabled(&mut self, flag: bool) {
        if flag {
            self.0 |= (1 << 5);
        } else {
            self.0 &= !(1 << 5);
        }
    }

    pub fn set_checked(&mut self, flag: bool) {
        if flag {
            self.0 |= (1 << 6);
        } else {
            self.0 &= !(1 << 6);
        }
    }

    pub fn get_hover(&self) -> bool {
        (self.0 & 1) != 0
    }

    pub fn get_over(&self) -> bool {
        (self.0 & (1 << 1)) >> 1 != 0
    }

    pub fn get_active(&self) -> bool {
        (self.0 & (1 << 2)) >> 2 != 0
    }

    pub fn get_focus(&self) -> bool {
        (self.0 & (1 << 3)) >> 3 != 0
    }

    pub fn get_enabled(&self) -> bool {
        (self.0 & (1 << 4)) >> 4 != 0
    }

    pub fn get_disabled(&self) -> bool {
        (self.0 & (1 << 5)) >> 5 != 0
    }

    pub fn get_checked(&self) -> bool {
        (self.0 & (1 << 6)) >> 6 != 0
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
    //pub pseudo_classes: HashSet<PseudoClass>,
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
            //pseudo_classes: HashSet::new(),
            pseudo_classes: PseudoClasses::default(),
            relation: Relation::None,
            asterisk: false,
        }
    }
}

impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        if self.asterisk {
            write!(f,"*")?;
        }
        
        if let Some(element) = &self.element {
            write!(f, "{}", element)?;
        }

        for class_name in self.classes.iter() {
            write!(f, ".{}", class_name)?;
        }

        write!(f,"{}",self.pseudo_classes);

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
            //pseudo_classes: HashSet::new(),
            pseudo_classes: PseudoClasses::default(),
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
            //pseudo_classes: HashSet::new(),
            pseudo_classes: PseudoClasses::default(),
            relation: Relation::None,
            asterisk: false,
        }
    }

    pub fn matches(&self, entity_selector: &Selector) -> bool {
        // Universal selector always matches
        if self.asterisk {
            if self.pseudo_classes.0 != 0
                && (self.pseudo_classes.0 & entity_selector.pseudo_classes.0) == 0
            {
                return false;
            } else {
                return true;
            }
        }

        if self.id.is_some() && self.id != entity_selector.id {
            return false;
        }

        if self.element.is_some() && self.element != entity_selector.element {
            return false;
        }

        if !self.classes.is_subset(&entity_selector.classes) {
            return false;
        }

        if self.pseudo_classes.0 != 0
            && (self.pseudo_classes.0 & entity_selector.pseudo_classes.0) == 0
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
            (self.classes.len() + self.pseudo_classes.0.count_ones() as usize) as u8,
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
        //let mut s = DefaultHasher::new();
        //element.hash(&mut s);
        self.element = Some(element.to_owned());
        self
    }

    pub fn set_hover(mut self) -> Self {
        self.pseudo_classes.set_hover(true);
        self
    }

    pub fn set_active(mut self) -> Self {
        self.pseudo_classes.set_active(true);
        self
    }
}

impl PartialEq for Selector {
    fn eq(&self, other: &Selector) -> bool {
        self.matches(other)
    }
}
