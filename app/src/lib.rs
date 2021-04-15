use seed::prelude::*;
use seed::*;
use seed::browser::fetch as fetch;
use serde_derive::*;


#[derive(Default)]
struct Model {
    items: Vec<String>,
    error: Option<String>,
    word: String,
}

#[derive(Serialize)]
struct PostTodo {
    name: String,
}

enum Msg {
    FetchedItems(fetch::Result<Vec<String>>),
    AddItem(),
    ClearAll(),
    NewWords(String),
    DeleteOne(usize),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    use Msg::{DeleteOne, NewWords, AddItem, ClearAll, FetchedItems};
    
    match msg {
        FetchedItems(resp) => match resp {
            Ok(items) => model.items = items,
            Err(e) => model.error = Some(format!("{:?}", e)),
        },
        AddItem() => {
            let new_items = post_todo_item(model.word.to_owned());
            model.word = "".to_string();
            orders.perform_cmd(async { Msg::FetchedItems(new_items.await) });
        },
        ClearAll() => {
            let new_items = clear_all_todo_items();
            orders.perform_cmd(async { Msg::FetchedItems(new_items.await) });
        },
        NewWords(text) => {
            model.word = text.to_string();
        },
        DeleteOne(index) => {
            let new_items = delete_todo_item(index);
            orders.perform_cmd(async { Msg::FetchedItems(new_items.await) });
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            input![
                attrs!{At::Placeholder => "Item Name"},
                attrs!{At::Value => model.word},
                input_ev("input", |text| Msg::NewWords(text)),
            ],
            button![
                "Add",
                ev(Ev::Click, |_| Msg::AddItem()),
            ],
            div![
                button![
                    C!{"clear-button"},
                    "Clear All",
                    ev(Ev::Click, |_| Msg::ClearAll()),
                ]
            ]
        ],
        ul![
            C!["title-list"],
            model.items.iter()
            .enumerate()
            .map(|(index, item)| {
                let label = format!("{}", item);
                let index_copy = index.to_owned();
                li![
                    C!["title-list-item"],
                    span![
                        label,
                        C!["title-list-item-label"],
                    ],
                    button![
                        C!["title-list-item-delete"],
                        "×",
                        attrs!{At::Title => "Delete"},
                        ev(Ev::Click, move |_| Msg::DeleteOne(index_copy)),
                    ]
                ]
            })
        ],
    ]
}

async fn get_todo_items() -> fetch::Result<Vec<String>> {
    Request::new("/api/todo")
        .method(fetch::Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn post_todo_item(string: String) -> fetch::Result<Vec<String>> {
    Request::new("/api/todo")
        .method(fetch::Method::Post)
        .json(&PostTodo {
            name: string.to_owned()
        })?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn delete_todo_item(index: usize) -> fetch::Result<Vec<String>> {
    Request::new(format!("/api/todo/{}", index.to_string()))
        .method(fetch::Method::Delete)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn clear_all_todo_items() -> fetch::Result<Vec<String>> {
    Request::new("/api/todo")
        .method(fetch::Method::Delete)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::FetchedItems(get_todo_items().await) });
    Model::default()
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
