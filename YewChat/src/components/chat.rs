use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                    .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => false,
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen h-screen bg-slate-50">
                /* Sidebar Users */
                <div class="flex-none w-64 bg-slate-900 text-white shadow-xl flex flex-col">
                    <div class="p-6 border-b border-slate-800">
                        <div class="text-2xl font-bold text-indigo-400">{"Online Users"}</div>
                    </div>
                    <div class="grow overflow-y-auto p-4 space-y-3">
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex items-center space-x-3 bg-slate-800/50 p-3 rounded-xl border border-slate-700/50">
                                    <img class="w-10 h-10 rounded-full border-2 border-indigo-500" src={u.avatar.clone()} alt="avatar"/>
                                    <div class="text-sm font-medium">{u.name.clone()}</div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                    </div>
                </div>

                /* Main Chat Area */
                <div class="grow flex flex-col">
                    /* Header */
                    <div class="h-16 bg-white border-b flex items-center px-6 justify-between shadow-sm">
                        <div class="flex items-center space-x-2">
                            <span class="text-2xl">{"💬"}</span>
                            <span class="text-xl font-bold text-slate-800">{"Fasilkom Global Chat"}</span>
                        </div>
                    </div>

                    /* Messages Area */
                    <div class="grow overflow-y-auto p-6 space-y-4 bg-slate-50">
                        {
                            self.messages.iter().map(|m| {
                                let avatar = self.users.iter()
                                    .find(|u| u.name == m.from)
                                    .map(|u| u.avatar.clone())
                                    .unwrap_or_else(|| "https://avatars.dicebear.com/api/adventurer-neutral/default.svg".to_string());

                                html!{
                                    <div class="flex items-start space-x-3">
                                        <img class="w-8 h-8 rounded-full shadow-sm mt-1" src={avatar} alt="avatar"/>
                                        <div class="flex flex-col">
                                            <div class="text-xs font-bold text-slate-500 ml-1 mb-1">{m.from.clone()}</div>
                                            <div class="bg-white p-3 rounded-2xl rounded-tl-none shadow-sm border border-slate-200 max-w-md">
                                                if m.message.ends_with(".gif") {
                                                    <img class="rounded-lg" src={m.message.clone()}/>
                                                } else {
                                                    <div class="text-sm text-slate-700">{m.message.clone()}</div>
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>

                    /* Input Area */
                    <div class="h-20 bg-white border-t flex items-center px-6 space-x-4">
                        <input
                            ref={self.chat_input.clone()}
                            type="text"
                            placeholder="Tulis pesan asinkronus..."
                            class="grow bg-slate-100 rounded-xl px-5 py-3 outline-none focus:ring-2 focus:ring-indigo-500/20 transition-all text-slate-900"
                            onkeydown={ctx.link().callback(|e: KeyboardEvent| {
                                if e.key() == "Enter" { Msg::SubmitMessage } else { Msg::HandleMsg("".to_string()) }
                            })}
                        />
                        <button onclick={submit} class="bg-indigo-600 hover:bg-indigo-500 text-white p-3 rounded-xl transition-colors shadow-lg shadow-indigo-200">
                            <svg viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <line x1="22" y1="2" x2="11" y2="13"></line>
                                <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}