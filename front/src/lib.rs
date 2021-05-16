use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::FetchService;
use yew::services::fetch::{Request, Response, FetchTask};
use yew::format::{Json, Nothing};
use chrono::{DateTime, Utc};
use serde::{Deserialize};
use anyhow;

#[derive(Deserialize)]
struct Todo {
    id: String,
    description: String,
    done: bool,
    datetime: DateTime<Utc>
}

struct Model {
    link: ComponentLink<Self>,
    todo_list: Vec<Todo>,
    fetch_task: Option<FetchTask>,
    error: Option<String>
}

enum Msg {
    GetList,
    SetList(Result<Vec<Todo>, anyhow::Error>)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetList);
        Self {
            link,
            todo_list: Vec::new(),
            fetch_task: None,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetList => {

                let request = Request::get("http://localhost:8080/api/todo").body(Nothing).expect("Could not build request.");
                let callback = self.link
                    .callback(|response: Response<Json<Result<Vec<Todo>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::SetList(data)
                    });

                let task = FetchService::fetch(request, callback).expect("failed to start request.");
                self.fetch_task = Some(task);
                false
            }
            Msg::SetList(result) => {
                match result {
                    Ok(todos) => {
                        self.todo_list = todos
                    }
                    Err(error) => {
                        self.error = Some(error.to_string())
                    }
                }
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
         <div>
            <ul>
                { for self.todo_list.iter().map(|e| html!{<li>{e.description.clone()}</li>}) }
            </ul>
         </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
