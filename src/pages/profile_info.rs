use gloo_console::log;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::{function_component, html::IntoHtmlResult, prelude::*};
use yewdux::{dispatch, use_store, Store};
use wasm_bindgen::JsCast;

use crate::{api, app::Role, pages::{login::LoginState, register::RegisterState}};


#[function_component]
pub fn ProfileInfo() -> Html {
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
    let (state, dispatch) = use_store::<RegisterState>();
    let email = state.email.clone();
    let name = state.first_name.clone();
    let surname = state.last_name.clone();

    html! {<>
    <div class="container reg-container">
     <div class="field">
  <label class="label">{"Имя"}</label>
  <div class="control">
    <input class="input" type="text" value={name.clone()} onchange = {name_changed}/>
  </div>
</div>

<div class="field">
  <label class="label">{"Фамилия"}</label>
  <div class="control">
    <input class="input" type="text" value={surname.clone()}  onchange={surname_changed}/>
  </div>
</div>

<div class="field">
  <label class="label">{"Email"}</label>
  <div class="control">
    <input class="input" type="email" value={email.clone()} onchange={email_changed}/>
  </div>
</div>
<div class="field">
  <label class="label">{"Текущий пароль"}</label>
  <div class="control">
    <input class="input" type="password"/>
  </div>
</div>
<div class="field">
  <label class="label">{"Новый пароль"}</label>
  <div class="control">
    <input class="input" type="password" placeholder=""/>
  </div>
</div>
<div class="field is-grouped is-grouped-centered">
  <p class="control">
    <button class="button is-primary" onclick= {handle_form_submit}>
      {"Изменить информацию"}
    </button>
  </p>
</div>

  </div>
    </>
    }
}