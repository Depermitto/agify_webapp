use std::collections::HashMap;
use std::fmt::Display;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{Component, Context, Html, html, Event};

pub struct Agify {
    entries: HashMap<String, i32>
}

impl Agify {
    pub async fn pull_from_name(name: String) -> Message {
        let response = reqwest::get(Self::format_to_url(name.clone()))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        #[derive(serde::Deserialize)]
        struct AgifyResponse {
            count: i32,
            name: String,
            age: i32
        }

        let response: AgifyResponse = serde_json::from_str(&response).unwrap();

        Message::GetFromName(name, response.age)
    }

    pub fn format_to_url(name: String) -> String {
        format!("https://api.agify.io/?name={}", name)
    }
    
    pub fn format_list_to_html<K, V>(list: HashMap<K, V>) -> Html
    where
        K: Display,
        V: Display
    {
        list.iter().map(|entry| html! {
            <li>
                {format!("{} - {} years old", entry.0, entry.1)}
            </li>
        } ).collect::<Html>()
    }
}

impl Component for Agify {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log!("Successfully created.");
        Self {
            entries: HashMap::new()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::PullFromName(name) => {
                ctx.link().send_future(Self::pull_from_name(name));
                true
            }
            Message::GetFromName(name, age) => {
                self.entries.insert(name, age);
                true
            }
            _ => true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <label for="name_input">{"How old does the hivemind think you are? Enter somebody's name!"}</label>
                // Text input
                <input type="text" name="input" id="name_input" onchange={
                    ctx.link().callback(|event: Event| {
                        let target = event.target().unwrap();
                        let input = target.unchecked_into::<HtmlInputElement>();
                        log!("New entry: ", input.value());
                
                        Message::PullFromName(input.value())
                    })
                }/>
                // Unordered list of age entries
                <ul>
                    {Self::format_list_to_html(self.entries.clone())}
                </ul>
            </div>
        }
    }
}

pub enum Message {
    PullFromName(String),
    PullFromNames(Vec<String>),
    GetFromName(String, i32)
}