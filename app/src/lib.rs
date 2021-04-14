use seed::prelude::*;
use seed::*;
use seed::browser::fetch as fetch;
use web_sys::window;


#[derive(Default)]
struct Model {
    items: Vec<String>,
    error: Option<String>,
}

enum Msg {
    FetchedItems(fetch::Result<Vec<String>>),
    AddItem(),
    ClearAll(),
    NewWords(String),
    ClearOne(web_sys::Event),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::{ClearOne, NewWords, AddItem, ClearAll, FetchedItems};

    let mut word = "goats".to_string();
    
    match msg {
        FetchedItems(resp) => match resp {
            Ok(items) => model.items = items,
            Err(e) => model.error = Some(format!("{:?}", e)),
        },
        AddItem() => {           
            model.items.push(word)
        },
        ClearAll() => {
            model.items.clear();
        },
        NewWords(text) => {
            word = text.to_string();
        },
        ClearOne(thingy) => {
            // this takes the last item, not the current item.
            log(thingy);
            model.items.pop();
            // model.items.remove(number);
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
            model.items.iter().map(|item| {
                li![
                    item, 
                    button![
                        "Delete",
                        style!{St::Margin => px(5);},
                        ev(Ev::Click, |thingy| Msg::ClearOne(thingy)),
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

// async fn post_todo_item() -> fetch::Result<Vec<String>> {
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
