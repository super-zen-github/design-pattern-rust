/// Visitor design pattern [action]
/// The Visitor pattern is useful when:
/// - You have a complex object structure
/// - You need to perform operations on objects of different types
/// - You want to add new operations without modifying the object classes
/// - You want to keep related operations together
///
/// This pattern helps separate algorithms from the objects on which they operate, making it
/// easier to add new operations without modifying existing code.
pub trait Element {
    fn accept(&self, visitor: &dyn Visitor);
}

pub trait Visitor {
    fn visit_text(&self, text: &Text);
    fn visit_image(&self, image: &Image);
}

pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Text {
            content: content.to_string(),
        }
    }
}

impl Element for Text {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_text(self);
    }
}

pub struct Image {
    path: String,
}

impl Image {
    pub fn new(path: &str) -> Self {
        Image {
            path: path.to_string(),
        }
    }
}

impl Element for Image {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_image(self);
    }
}

// Concrete visitors
pub struct HTMLExportVisitor;

impl Visitor for HTMLExportVisitor {
    fn visit_text(&self, text: &Text) {
        println!("<p>{}</p>", text.content);
    }

    fn visit_image(&self, image: &Image) {
        println!("<img src=\"{}\" >", image.path);
    }
}

pub struct PlainTextExportVisitor;

impl Visitor for PlainTextExportVisitor {
    fn visit_text(&self, text: &Text) {
        println!("Text: {}", text.content);
    }

    fn visit_image(&self, image: &Image) {
        println!("Image: {}", image.path);
    }
}

// Document structure
pub struct Document {
    elements: Vec<Box<dyn Element>>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: Box<dyn Element>) {
        self.elements.push(element);
    }

    pub fn accept(&self, visitor: &dyn Visitor) {
        for element in &self.elements {
            element.accept(visitor);
        }
    }
}
