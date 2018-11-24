#![windows_subsystem = "windows"]

extern crate azul;
extern crate comrak;

use azul::{prelude::*, widgets::{label::Label, button::Button}};
use azul::dialogs::open_directory_dialog;

#[derive(Debug, Default)]
struct DataModel {
    open_directory: Option<OpenedDirectory>,
    is_sidebar_open: bool,
}

#[derive(Debug, Clone)]
struct OpenedDirectory {
    root_path: String,
    files: Vec<String>,
    opened_file: Option<usize>,
}

impl Layout for DataModel {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        match &self.open_directory {
            Some(dir) => render_project(self.is_sidebar_open, info.window.state.size.dimensions.width > 750.0, dir),
            None => render_welcome_screen(),
        }
    }
}

fn render_welcome_screen() -> Dom<DataModel> {
    Dom::new(NodeType::Div).with_id("welcome_screen")
        .with_child(Label::new("No directory loaded!").dom().with_id("no_directory_label"))
        .with_child(Button::with_label("Load directory...").dom().with_id("load_directory_btn")
                    .with_callback(On::MouseUp, Callback(load_directory_callback))
    )
}

fn render_project(_sidebar_open: bool, _preview_open: bool, dir: &OpenedDirectory) -> Dom<DataModel> {
    Dom::new(NodeType::Label(format!("Directory loaded: {}", dir.root_path)))
}

fn load_directory_callback(app_state: &mut AppState<DataModel>, event: WindowEvent<DataModel>) -> UpdateScreen {
    fn load_directory_callback_inner(app_state: &mut AppState<DataModel>, _event: WindowEvent<DataModel>) -> Option<()> {
        let file_path = open_directory_dialog(None)?;
        let dir = load_directory(&file_path)?;
        let state = &mut *app_state.data.lock().ok()?;

        state.open_directory = Some(dir);
        Some(())
    }

    load_directory_callback_inner(app_state, event).into()
}

fn load_directory(root_path: &str) -> Option<OpenedDirectory> {
    println!("path: {:?}", root_path);

    None
}

struct MarkdownDocument {
    text: String,
}

use comrak::nodes::AstNode;

fn parse_markdown(text: &str) -> Option<MarkdownDocument> {
    use comrak::{Arena, parse_document, ComrakOptions};
    // use comrak::nodes::{AstNode, NodeValue};

    let arena = Arena::new();

    let root = parse_document(
        &arena,
        text,
        &ComrakOptions::default());

    let callback = |node| { println!("node: {:?}", node); };

    iter_nodes(root, &callback);

    None
}

fn iter_nodes<'a, F: Fn(&'a AstNode<'a>)>(node: &'a AstNode<'a>, callback: &F) {
    callback(node);
    for c in node.children() {
        iter_nodes(c, callback);
    }
}

fn main() {

    macro_rules! CSS_PATH { () => (concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.css")) }

    #[cfg(debug_assertions)]
    let css = Css::hot_reload(CSS_PATH!()).unwrap();
    #[cfg(not(debug_assertions))]
    let css = Css::new_from_str(include_str!(CSS_PATH!())).unwrap();

    let app = App::new(DataModel::default(), AppConfig::default());
    app.run(Window::new(WindowCreateOptions::default(), css).unwrap()).unwrap();
}