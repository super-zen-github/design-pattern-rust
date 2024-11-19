/// Command design pattern [action]
/// The command pattern encapsulates a request as an object, thereby letting us parameterize other objects
/// with different requests, queue or log requests, and support undoable operations.
///
/// The pattern is useful when you need to:
/// - Parameterize objects with operations
/// - Queue operations
/// - Support undoable operations
/// - Support logging of changes
/// - Structure a system around high-level operations
use std::rc::Rc;
use std::cell::RefCell;

pub trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
}

#[derive(Clone)]
pub struct CommandTextEditor {
    content: String,
}

impl CommandTextEditor {
    pub fn new() -> Self {
        CommandTextEditor {
            content: String::new(),
        }
    }

    fn insert_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn delete_text(&mut self, length: usize) {
        let new_len = if length > self.content.len() {
            0
        } else {
            self.content.len() - length
        };
        self.content.truncate(new_len);
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
}

pub struct InsertTextCommand {
    editor: Rc<RefCell<CommandTextEditor>>,
    text: String,
}

impl InsertTextCommand {
    pub fn new(editor: Rc<RefCell<CommandTextEditor>>, text: String) -> Self {
        InsertTextCommand {
            editor,
            text,
        }
    }
}

impl Command for InsertTextCommand {
    fn execute(&mut self) {
        self.editor.borrow_mut().insert_text(&self.text);
    }

    fn undo(&mut self) {
        self.editor.borrow_mut().delete_text(self.text.len());
    }
}

pub struct DeleteTextCommand {
    editor: Rc<RefCell<CommandTextEditor>>,
    length: usize,
    deleted_text: String,
}

impl DeleteTextCommand {
    pub fn new(editor: Rc<RefCell<CommandTextEditor>>, length: usize) -> Self {
        let deleted_text = editor.borrow().content[editor.borrow().content.len().saturating_sub(length)..].to_string();
        DeleteTextCommand {
            editor,
            length,
            deleted_text,
        }
    }
}

impl Command for DeleteTextCommand {
    fn execute(&mut self) {
        self.editor.borrow_mut().delete_text(self.length);
    }

    fn undo(&mut self) {
        self.editor.borrow_mut().insert_text(&self.deleted_text);
    }
}

pub struct TextEditorInvoker {
    pub editor: Rc<RefCell<CommandTextEditor>>,
    command_history: Vec<Box<dyn Command>>,
    undo_history: Vec<Box<dyn Command>>,
}

impl TextEditorInvoker {
    pub fn new() -> Self {
        TextEditorInvoker {
            editor: Rc::new(RefCell::new(CommandTextEditor::new())),
            command_history: Vec::new(),
            undo_history: Vec::new(),
        }
    }

    pub fn execute_command(&mut self, mut command: Box<dyn Command>) {
        command.execute();
        self.command_history.push(command);
        self.undo_history.clear();
    }

    pub fn undo(&mut self) {
        if let Some(mut command) = self.command_history.pop() {
            command.undo();
            self.undo_history.push(command);
        }
    }

    pub fn redo(&mut self) {
        if let Some(mut command) = self.undo_history.pop() {
            command.execute();
            self.command_history.push(command);
        }
    }
}
