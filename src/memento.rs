/// Memento design pattern [action]
/// Captures and externalizes an object's internal state
/// so that it can be restored later, all without violating encapsulation.
///
/// This pattern is useful when you need to:
/// - Implement undo/redo functionality
/// - Save and restore object states
/// - Create snapshots of object states
/// - Need to restore an object to a previous state while keeping encapsulation

// Memento: Contains the state that needs to be saved and restored
#[derive(Clone)]
pub struct EditorMemento {
    content: String,
    cursor_position: usize,
}

// Originator: The object whose state needs to be saved
pub struct TextEditor {
    content: String,
    cursor_position: usize,
}

// Caretaker: Manages and stores mementos
pub struct History {
    states: Vec<EditorMemento>,
}

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            content: String::new(),
            cursor_position: 0,
        }
    }

    pub fn write(&mut self, text: &str) {
        self.content.push_str(text);
        self.cursor_position += text.len();
    }

    pub fn move_cursor(&mut self, position: usize) {
        if position <= self.content.len() {
            self.cursor_position = position;
        }
    }

    pub fn save(&self) -> EditorMemento {
        EditorMemento {
            content: self.content.clone(),
            cursor_position: self.cursor_position,
        }
    }

    pub fn restore(&mut self, memento: EditorMemento) {
        self.content = memento.content;
        self.cursor_position = memento.cursor_position;
    }

    pub fn get_content(&self) -> &str {
        &self.content.get(..self.cursor_position).unwrap()
    }

    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position
    }
}

impl History {
    pub fn new() -> Self {
        History {
            states: Vec::<EditorMemento>::new(),
        }
    }

    pub fn push(&mut self, memento: EditorMemento) {
        self.states.push(memento);
    }

    pub fn pop(&mut self) -> Option<EditorMemento> {
        self.states.pop()
    }
}
