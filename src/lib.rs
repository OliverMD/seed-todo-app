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
use seed::{prelude::*, *};
use Visibility::*;

const TITLE_SUFFIX: &str = "TODO";
const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";

const ABOUT: &str = "about";

const ENTER_KEY: &str = "Enter";

// ------ ------
//     Init
// ------ ------

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {
        new_todo: String::new(),
        todos: vec![
            String::from("Pick up groceries"),
            String::from("10 minutes meditation"),
        ],
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

pub struct Model {
    new_todo: String,
    todos: Vec<String>,
}

// ------ Page ------

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    About,
    NotFound,
}

impl Page {
    pub fn init(mut url: Url) -> Self {
        let (page, title) = match url.remaining_path_parts().as_slice() {
            [] => (Self::Home, TITLE_SUFFIX.to_owned()),
            [ABOUT] => (Self::About, format!("About - {}", TITLE_SUFFIX)),
            _ => (Self::NotFound, format!("404 - {}", TITLE_SUFFIX)),
        };
        document().set_title(&title);
        page
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }

    pub fn about(self) -> Url {
        self.base_url().add_path_part(ABOUT)
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    NewTodoChanged(String),
    CreateTodo,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NewTodoChanged(new_todo) => {
            model.new_todo = new_todo;
        }
        Msg::CreateTodo => {
            model.todos.push(model.new_todo.to_owned());
            model.new_todo = String::new();
        }
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
            todo_list_view(&model.todos)
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
                C.text_base
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

fn todo_list_view(todos: &Vec<String>) -> Node<Msg> {
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
            todos.iter().enumerate().map(|(idx, todo)| {
                li![
                    el_key(&idx),
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
                        div![C![C.ml_4, C.text_base], todo]
                    ]
                ]
            })
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
