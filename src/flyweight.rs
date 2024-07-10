/// Flyweight design pattern
/// for saving time or space

use std::{collections::HashMap, rc::Rc};

pub enum Color {
    RED,
    BLACK,
}

pub struct ChessBase {
    id: i8,
    color: Color, 
    text: String,
}

impl ChessBase {
    pub fn new(id: i8, color: Color, text: String) -> Self {
        Self {
            id,
            color,
            text,
        }
    }
}

pub struct ChessBaseFactory {
    bases: HashMap<i8, ChessBase>
}

impl ChessBaseFactory {
    pub fn new() -> Self {
        let mut bases = HashMap::new();
        let base1 = ChessBase::new(1, Color::RED, "king".into());
        let base2 = ChessBase::new(2, Color::BLACK, "king".into());
        let base3 = ChessBase::new(3, Color::RED, "general".into());
        let base4 = ChessBase::new(4, Color::BLACK, "general".into());
        bases.insert(1, base1);
        bases.insert(2, base2);
        bases.insert(3, base3);
        bases.insert(4, base4);
        Self { bases }
    }

    pub fn get_base(&self, idx: &i8) -> &ChessBase {
        self.bases.get(idx).unwrap()
    }
}

pub struct ChessUnit<'a> {
    base: Rc<&'a ChessBase>,
    pos_x: i8,
    pos_y: i8,
}

impl<'a> ChessUnit<'a> {
    pub fn new(base: Rc<&'a ChessBase>, pos_x: i8, pos_y: i8) -> Self {
        Self { base, pos_x, pos_y }
    }
}


pub struct ChessBoard<'a> {
    units: HashMap<i8, ChessUnit<'a>>
}

impl<'a> ChessBoard<'a> {
    pub fn new() -> Self {
        let units = HashMap::new();
        Self { units }
    }

    pub fn add_unit(&mut self, idx: i8, unit: ChessUnit<'a>) {
        self.units.insert(idx, unit);
    }

    pub fn print(&self) {
        for (key, val) in self.units.iter() {
            println!("{}'s x: {}, y: {}", val.base.text, val.pos_x, val.pos_y);
        }
    }
}
