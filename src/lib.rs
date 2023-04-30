#![recursion_limit = "256"]

use serde_derive::Deserialize;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::html;
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};
use yew_router::{components::RouterAnchor, router::Router, Switch};


struct TodoApp {
    link: ComponentLink<Self>,
    todos: Option<Vec<Todo>>,
    fetch_task: Option<FetchTask>
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub user_id: u64,
    pub id: u64,
    pub title: String,
    pub completed: bool,
}


enum Msg {
    MakeReq,
    Resp(Result<Vec<Todo>, anyhow::Error>)
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();
    // fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    //     Self {
    //         link,
    //         todos: None,
    //         fetch_task: None,
    //     }
    // }
    fn create(ctx: &Context<Self>) -> Self {
        Self { link: (), todos: (None), fetch_task: (None) }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div></div>
        }
    }
}
