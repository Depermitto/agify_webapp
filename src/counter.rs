use yew::{Component, Context, Html, html};

pub enum Message {
    Increment,
    Decrement
}

pub struct Counter {
    value: i32
}


impl Component for Counter {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: 0
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <div>
                <button onclick={ctx.link().callback(|_| Message::Increment)}>{"Increment"}</button>
                <p>{self.value}</p>
                <button onclick={ctx.link().callback(|_| Message::Decrement)}>{"Decrement"}</button>
            </div>
        }
    }
}
