// @TODO: uncomment once https://github.com/rust-lang/rust/issues/54726 stable
//#![rustfmt::skip::macros(class)]

#![allow(
    clippy::used_underscore_binding,
    clippy::non_ascii_literal,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

mod generated;

use generated::css_classes::C;
use seed::{
    prelude::{wasm_bindgen::__rt::std::collections::BTreeMap, *},
    *,
};
use ulid::Ulid;
use Visibility::*;

const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";

const ENTER_KEY: &str = "Enter";

// ------ ------
//     Init
// ------ ------

#[allow(clippy::needless_pass_by_value)]
fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    let mut initial_todos = BTreeMap::new();

    initial_todos
        .insert(Ulid::new(), Todo::new(String::from("Pick up groceries")));
    initial_todos
        .insert(Ulid::new(), Todo::new(String::from("10 minutes meditation")));
    initial_todos.insert(
        Ulid::new(),
        Todo::new_completed(String::from("This is a completed todo")),
    );
    Model {
        new_todo: String::new(),
        todos: initial_todos,
    }
}

// ------ ------
//     Model
// ------ ------

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
}

impl Visibility {
    pub fn toggle(&mut self) {
        *self = match self {
            Visible => Hidden,
            Hidden => Visible,
        }
    }
}

struct Todo {
    content: String,
    is_complete: bool,
}

impl Todo {
    const fn new(content: String) -> Self {
        Self {
            content,
            is_complete: false,
        }
    }

    const fn new_completed(content: String) -> Self {
        Self {
            content,
            is_complete: true,
        }
    }
}

pub struct Model {
    new_todo: String,
    todos: BTreeMap<Ulid, Todo>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    NewTodoChanged(String),
    CreateTodo,
    ToggleComplete(Ulid),
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NewTodoChanged(new_todo) => {
            model.new_todo = new_todo;
        },
        Msg::CreateTodo => {
            model
                .todos
                .insert(Ulid::new(), Todo::new(model.new_todo.to_owned()));
            model.new_todo = String::new();
        },
        Msg::ToggleComplete(id) => {
            model
                .todos
                .entry(id)
                .and_modify(|todo| todo.is_complete = !todo.is_complete);
        },
    }
}

// ------ ------
//     View
// ------ ------

// Notes:
// - \u{00A0} is the non-breaking space
//   - https://codepoints.net/U+00A0
//
// - "▶\u{fe0e}" - \u{fe0e} is the variation selector, it prevents ▶ to change to emoji in some browsers
//   - https://codepoints.net/U+FE0E

pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        C![C.justify_center, C.items_center, C.flex],
        div![
            C![C.absolute, C.w_screen, C.h_full, C.top_0],
            img![attrs! { At::Src => image_src("bg-desktop-light.jpg")}]
        ],
        div![
            C![C.px_56, C.mt_16, C.container, C.z_10],
            header_view(),
            new_todo_view(&model.new_todo),
            todo_list_view(&model.todos),
            div![
                C![
                    C.font_display,
                    C.flex,
                    C.justify_center,
                    C.text_light_3,
                    C.mt_16,
                    C.text_sm
                ],
                "Drag and drop to reorder list"
            ]
        ],
    ]
}

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn image_src_url(image: &str) -> String {
    format!("url({}/{})", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}

fn header_view() -> Node<Msg> {
    div![
        C![C.relative, C.flex, C.flex_row, C.mb_12],
        h1![
            C![
                C.flex_1,
                C.font_display,
                C.text_light_1,
                C.font_bold,
                C.tracking_widest,
                C.text_4xl,
            ],
            "TODO"
        ],
        div![
            C![C.self_center],
            img![C![], attrs! {At::Src => image_src("icon-moon.svg")}]
        ]
    ]
}

fn new_todo_view(new_todo: &str) -> Node<Msg> {
    div![
        C![C.rounded_md, C.bg_light_6, C.w_full, C.p_5, C.flex],
        div![C![
            C.rounded_full,
            C.justify_center,
            C.h_6,
            C.w_6,
            C.flex,
            C.border_light_2,
            C.border_2
        ]],
        input![
            C![
                C.ml_4,
                C.font_display,
                C.text_light_4,
                C.bg_light_6,
                C.w_full,
                C.text_base,
                C.focus__outline_none
            ],
            attrs! {
                At::Placeholder => "Create a new todo...",
                At::AutoFocus => AtValue::None,
                At::Value => new_todo
            },
            input_ev(Ev::Input, Msg::NewTodoChanged),
            keyboard_ev(
                Ev::KeyDown,
                |event| IF!(event.key() == ENTER_KEY => Msg::CreateTodo)
            )
        ]
    ]
}

fn todo_view(id: &Ulid, todo: &Todo) -> Node<Msg> {
    let id = *id;
    if todo.is_complete {
        li![
            el_key(&id),
            C![
                C.p_5,
                C.flex,
                C.w_full,
                C.font_display,
                C.text_light_3,
                C.border_light_4
            ],
            div![
                C![
                    C.rounded_full,
                    C.justify_center,
                    C.h_6,
                    C.w_6,
                    C.flex,
                    C.bg_gradient_to_r,
                    C.from_light_5,
                    C.to_blue
                ],
                img![
                    C![C.w_3, C.h_3, C.flex, C.self_center],
                    attrs! {At::Src => image_src("icon-check.svg")}
                ],
                mouse_ev(Ev::Click, move |_| Msg::ToggleComplete(id))
            ],
            div![C![C.ml_4, C.text_base, C.line_through], &todo.content]
        ]
    } else {
        li![
            el_key(&id),
            div![
                C![
                    C.p_5,
                    C.flex,
                    C.w_full,
                    C.font_display,
                    C.text_light_5,
                    C.border_light_4
                ],
                div![C![
                    C.rounded_full,
                    C.justify_center,
                    C.h_6,
                    C.w_6,
                    C.flex,
                    C.border_light_2,
                    C.border_2
                ]],
                div![C![C.ml_4, C.text_base], &todo.content],
                mouse_ev(Ev::Click, move |_| Msg::ToggleComplete(id))
            ]
        ]
    }
}

fn todo_list_view(todos: &BTreeMap<Ulid, Todo>) -> Node<Msg> {
    div![
        C![
            C.mt_8,
            C.flex,
            C.rounded_md,
            C.bg_light_6,
            C.w_full,
            C.shadow,
            C.flex_col,
            C.divide_y,
            C.divide_light_3
        ],
        ul![
            C![C.flex, C.w_full, C.divide_y, C.divide_light_3, C.flex_col],
            todos.iter().map(|(id, todo)| { todo_view(id, todo) }),
        ],
        div![
            C![
                C.flex,
                C.flex_row,
                C.font_display,
                C.p_5,
                C.text_light_3,
                C.text_sm
            ],
            div![C![C.flex_auto], format!("{} items left", todos.len())],
            div![
                C![
                    C.flex_auto,
                    C.flex,
                    C.justify_center,
                    C.space_x_5,
                    C.font_display,
                    C.font_bold
                ],
                div![C![C.text_blue], "All"],
                div!["Active"],
                div!["Completed"]
            ],
            div![C![C.flex_auto, C.flex, C.justify_end], "Clear Completed"]
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");

    App::start("app", init, update, view);

    log!("App started.");
}
