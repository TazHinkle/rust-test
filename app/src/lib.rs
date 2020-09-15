use seed::prelude::*;
use seed::*;
use seed::browser::fetch as fetch;

#[derive(Default)]
struct Model {
    items: Vec<String>,
    error: Option<String>,
}

enum Msg {
    FetchedItems(fetch::Result<Vec<String>>),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        FetchedItems(resp) => match resp {
            Ok(items) => model.items = items,
            Err(e) => model.error = Some(format!("{:?}", e)),
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        ul![
            model.items.iter().map(|item| {
                li![item]
            })
        ]
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

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::FetchedItems(get_todo_items().await) });
    Model::default()
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
