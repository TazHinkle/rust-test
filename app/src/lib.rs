use seed::prelude::*;
use seed::*;
use seed::browser::fetch as fetch;
use web_sys::window;


#[derive(Default)]
struct Model {
    items: Vec<String>,
    error: Option<String>,
    word: String,
}

enum Msg {
    FetchedItems(fetch::Result<Vec<String>>),
    AddItem(),
    ClearAll(),
    NewWords(String),
    ClearOne(usize),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::{ClearOne, NewWords, AddItem, ClearAll, FetchedItems};
    
    match msg {
        FetchedItems(resp) => match resp {
            Ok(items) => model.items = items,
            Err(e) => model.error = Some(format!("{:?}", e)),
        },
        AddItem() => {           
            model.items.push(model.word.to_owned())
        },
        ClearAll() => {
            model.items.clear();
        },
        NewWords(text) => {
            model.word = text.to_string();
        },
        ClearOne(index) => {

            // this takes the last item, not the current item.
            // model.items.pop();
            model.items.remove(index);
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    let count = 0;

    div![
        div![
            input![
                attrs!{At::Placeholder => "Item Name"},
                input_ev("input", |text| Msg::NewWords(text)),
            ],
            button![
                "Add",
                ev(Ev::Click, |_| Msg::AddItem()),
            ],
            div![
                button![
                    "Clear All",
                    ev(Ev::Click, |_| Msg::ClearAll()),
                ]
            ]
        ],
        ul![
            model.items.iter()
            .enumerate()
            .map(|(index, item)| {
                let label = format!("{}, {}", index, item);
                let index_copy = index.to_owned();
                li![
                    label, 
                    button![
                        "Delete",
                        style!{St::Margin => px(5);},
                        ev(Ev::Click, move |_| Msg::ClearOne(index_copy)),
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

// async fn post_todo_item(String string) -> fetch::Result<Vec<String>> {
//     Request::new("/api/todo")
//         .method(fetch::Method::Post)
//         .fetch()
//         .await?
//         .check_status()?
//         .json()
//         .await
// }

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::FetchedItems(get_todo_items().await) });
    Model::default()
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
