use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="flex h-screen w-screen items-center justify-center bg-gradient-to-br from-slate-900 to-indigo-950">
            <div class="w-full max-w-md rounded-2xl bg-white/10 p-8 shadow-2xl backdrop-blur-md border border-white/20">
                <div class="mb-8 text-center">
                    <h1 class="text-4xl font-extrabold text-white mb-2">{"Andi's Chat"}</h1>
                    <p class="text-indigo-200">{"Fasilkom UI - Module 10 Async"}</p>
                </div>

                <div class="flex flex-col space-y-4">
                    <input
                        {oninput}
                        class="w-full rounded-xl bg-slate-800/50 p-4 text-white placeholder-slate-400 outline-none ring-2 ring-indigo-500/50 focus:ring-indigo-400"
                        placeholder="Masukkan Username..."
                    />

                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            class="w-full rounded-xl bg-indigo-600 p-4 font-bold text-white transition-all hover:bg-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg shadow-indigo-900/40"
                        >
                            {"JOIN CHAT"}
                        </button>
                    </Link<Route>>
                </div>

                <div class="mt-6 text-center text-xs text-slate-500">
                    {"Tugas Modul 10 - Pemrograman Lanjut"}
                </div>
            </div>
        </div>
    }
}