use std::{rc::Rc, cell::RefCell};

pub type RcComponent = Rc<RefCell<Component>>;
pub type RcWire = Rc<RefCell<Wire>>;


pub enum Component {
    Input(usize),
    AndGate(RcWire, RcWire, RcWire),
    OrGate(RcWire, RcWire, RcWire),
    NotGate(RcWire, RcWire),
    LshiftGate(RcWire, usize, RcWire),
    RshiftGate(RcWire, usize, RcWire),
}

pub struct Wire {
    pub name: String,
    pub input: Option<RcComponent>,
    pub outputs: Vec<RcComponent>
}

impl Wire {
    pub fn new(name: String, input: Option<RcComponent>, outputs: Vec<RcComponent>) -> RcWire {
        Rc::new(RefCell::new(Wire { name, input, outputs }))
    }
    pub fn new_component(cmp: Component) -> RcComponent {
        Rc::new(RefCell::new(cmp))
    }
}

impl core::fmt::Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wire").field("name", &self.name).finish()
    }
}