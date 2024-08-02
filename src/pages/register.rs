use std::ops::Deref;

use gloo_console::log;
use gloo_net::http::Request;
use serde::{ Deserialize, Serialize };
use serde_json::json;
use wasm_bindgen_futures::js_sys::JSON;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew::{ function_component, prelude::* };
use yewdux::dispatch;
use yewdux::use_store;
use yewdux::prelude::*;

use crate::{ api, app::SERVER_URL };

#[derive(Serialize, Deserialize)]
struct CreateUserBody {}

#[derive(Default, PartialEq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")]
// #[store(storage = "local", storage_tab_sync)]
pub struct RegisterState {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub success: bool,
    pub passwords_equal: bool,
}
#[function_component]
pub fn RegisterPage() -> Html {
    // let s = RegisterForm{email: "ss".to_string()};
    let (state, dispatch) = use_store::<RegisterState>();

    let name_changed = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        dispatch.reduce_mut(|state| {
            state.first_name = value;
        })
    });
    let (state, dispatch) = use_store::<RegisterState>();
    let surname_changed = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        dispatch.reduce_mut(|state| {
            state.last_name = value;
        })
    });
    let (state, dispatch) = use_store::<RegisterState>();
    let email_changed = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        dispatch.reduce_mut(|state| {
            state.email = value;
        })
    });
    let (state, dispatch) = use_store::<RegisterState>();
    let password_changed = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        dispatch.reduce_mut(|state| {
            state.password = value;
        })
    });
    let (state, dispatch) = use_store::<RegisterState>();
    let password_again_changed = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        // log!("{} {}", state.password, value);
        dispatch.reduce_mut(|state| {
            log!("{} {} {}", state.password.clone(), value.clone(), state.password.eq(&value));
            state.passwords_equal = state.password.eq(&value);
        })
    });
    let (state, dispatch) = use_store::<RegisterState>();
    let handle_form_submit = dispatch.future_callback(|dispatch| async move {
        wasm_bindgen_futures::spawn_local(async move {
            let resp = api::api_register(
                dispatch.get().first_name.clone(),
                dispatch.get().last_name.clone(),
                dispatch.get().password.clone(),
                dispatch.get().email.clone()
            ).await;
            dispatch.reduce_mut(|state| {
                state.success = resp;
            });
        });
    });


    html! {<>
    <div class="container reg-container">
     <div class="field">
  <label class="label">{"Имя"}</label>
  <div class="control">
    <input class="input" type="text" placeholder="Иван" onchange = {name_changed}/>
  </div>
</div>

<div class="field">
  <label class="label">{"Фамилия"}</label>
  <div class="control">
    <input class="input" type="text" placeholder="Иванов"  onchange={surname_changed}/>
  </div>
</div>

<div class="field">
  <label class="label">{"Email"}</label>
  <div class="control">
    <input class="input" type="email" placeholder="Электронная почта" onchange={email_changed}/>
  </div>
</div>
<div class="field">
  <label class="label">{"Пароль"}</label>
  <div class="control">
    <input class="input" type="password" onchange={password_changed}/>
  </div>
</div>
<div class="field">
  <label class="label">{"Повтор пароля"}</label>
  <div class="control">
    <input class="input" type="password" placeholder="" onchange={password_again_changed}/>
  </div>
</div>
{
  if state.passwords_equal {
    html!{}
  }
  else {
    html! {
      <div class="notification is-danger is-centered">
        {"Пароли не совпадают!"}
      </div>
      }
  }
}
{
  if state.success {
    html! {
    <div class="notification is-success is-centered">
      {"Регистрация прошла успешно!"}
    </div>
    }
  }
  else {
    html!{}
  }
}
<div class="field is-grouped is-grouped-centered">
  <p class="control">
    <button class="button is-primary" onclick= {handle_form_submit}>
      {"Регистрация"}
    </button>
  </p>
  <p class="control">
    <a class="button is-light">
      {"Вход"}
    </a>
  </p>

</div>

  </div>
    </>
    }
}
