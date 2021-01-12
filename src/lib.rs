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

use fixed_vec_deque::FixedVecDeque;
use generated::css_classes::C;
use seed::{prelude::*, *};
use Visibility::*;

const TITLE_SUFFIX: &str = "TODO";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";

const ABOUT: &str = "about";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        base_url: url.to_base_url(),
        page: Page::init(url),
        scroll_history: ScrollHistory::new(),
        menu_visibility: Hidden,
        in_prerendering: is_in_prerendering(),
    }
}

fn is_in_prerendering() -> bool {
    let user_agent =
        window().navigator().user_agent().expect("cannot get user agent");

    user_agent == USER_AGENT_FOR_PRERENDERING
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

// We need at least 3 last values to detect scroll direction,
// because neighboring ones are sometimes equal.
type ScrollHistory = FixedVecDeque<[i32; 3]>;

pub struct Model {
    pub base_url: Url,
    pub page: Page,
    pub scroll_history: ScrollHistory,
    pub menu_visibility: Visibility,
    pub in_prerendering: bool,
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

pub enum Msg {}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {}

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
            new_todo_view()
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

fn new_todo_view() -> Node<Msg> {
    div![
        C![C.rounded_md, C.bg_light_1, C.w_full, C.p_5, C.flex],
        div![C![
            C.rounded_full,
            C.justify_center,
            C.h_6,
            C.w_6,
            C.flex,
            C.border_light_3,
            C.border_2
        ]],
        div![
            C![C.ml_4, C.font_display, C.text_light_4],
            "Create a new todo..."
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
