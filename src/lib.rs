// TODO: Remove
// #![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use std::collections::BTreeMap;
use ulid::Ulid;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        todos: BTreeMap::new(),
        new_todo_title: String::new(),
        selected_todo: None,
        filter: Filter::All,
        base_url: Url::new(),
    }.add_mock_data()
}


// ------ ------
//     Model
// ------ ------

struct Model {
    todos: BTreeMap<Ulid, Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>,
    filter: Filter,
    base_url: Url,
}

// TODO: Remove
impl Model {
    fn add_mock_data(mut self) -> Self {
        let (id_a, id_b) = (Ulid::new(), Ulid::new());

        self.todos.insert(id_a, Todo {
            id: id_a,
            title: "I'm todo A".to_owned(),
            completed: false,
        });

        self.todos.insert(id_b, Todo {
            id: id_b,
            title: "I'm todo B".to_owned(),
            completed: true,
        });


        self.new_todo_title = "I'm a new todo title.".to_owned();

        self.selected_todo = Some(SelectedTodo {
            id: id_b,
            title: "I'm better todo B".to_owned(),
            input_element: ElRef::new(),
        });
        self
    }
}

struct Todo {
    id: Ulid,
    title: String,
    completed: bool,
}

struct SelectedTodo {
    id: Ulid,
    title: String,
    input_element: ElRef<web_sys::HtmlInputElement>,
}

enum Filter {
    All,
    Active,
    Completed,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
    NewTodoTitleChanged(String),

    // ------ Basic Todo operations ------

    CreateTodo,
    ToggleTodo(Ulid),
    RemoveTodo(Ulid),

    // ------ Bulk operations ------

    CheckOrUncheckAll,
    ClearCompleted,

    // ------ Selection ------

    SelectTodo(Option<Ulid>),
    SelectedTodoTitleChanged(String),
    SaveSelectedTodo,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("UrlChanged", url);
        }
        Msg::NewTodoTitleChanged(title) => {
            log!("NewTodoTitleChanged", title);
        }

        // ------ Basic Todo operations ------

        Msg::CreateTodo => {
            log!("CreateTodo");
        }
        Msg::ToggleTodo(id) => {
            log!("ToggleTodo");
        }
        Msg::RemoveTodo(id) => {
            log!("RemoveTodo");
        }

        // ------ Bulk operations ------

        Msg::CheckOrUncheckAll => {
            log!("CheckOrUncheckAll");
        }
        Msg::ClearCompleted => {
            log!("ClearCompleted");
        }

        // ------ Selection ------

        Msg::SelectTodo(opt_id) => {
            log!("SelectTodo");
        }
        Msg::SelectedTodoTitleChanged(title) => {
            log!("SelectedTodoTitleChanged", title);
        }
        Msg::SaveSelectedTodo => {
            log!("SaveSelectedTodo");
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    raw![include_str!("../template.html")]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let root_element = document()
        .get_elements_by_class_name("todoapp").item(0).expect("element with the class `todoapp`");

    App::start(root_element, init, update, view);
}
