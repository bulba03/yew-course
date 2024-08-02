use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::{function_component, html, AttrValue, Callback, Event, Html, Properties};
use yewdux::{use_store, Store};use wasm_bindgen::JsCast;

use crate::api;

use super::login::LoginState;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}
#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
pub struct State {
    name: String,
    short_desc: String,
    desc: String,
    image_url: String,
    id: i32
}
#[function_component]
pub fn CourseEdit(props: &Props) -> Html {
  let (state, dispatch) = use_store::<State>();
  dispatch.reduce_mut(|state| state.id = props.id);
  let name_changed = Callback::from(move |event: Event| {
    let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
    dispatch.reduce_mut(|state| {
        state.name = value;
    })
  });
  let (state, dispatch) = use_store::<State>();

  let short_desc_changed = Callback::from(move |event: Event| {
    let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
    dispatch.reduce_mut(|state| {
        state.short_desc = value;
    })
  });
  let (state, dispatch) = use_store::<State>();

  let desc_changed = Callback::from(move |event: Event| {
    let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
    dispatch.reduce_mut(|state| {
        state.desc = value;
    })
  });
  let (state, dispatch) = use_store::<State>();

  let img_changed = Callback::from(move |event: Event| {
    let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
    dispatch.reduce_mut(|state| {
        state.image_url = value;
    })
  });
  let (state, dispatch) = use_store::<State>();
    let (l_state, _) = use_store::<LoginState>();
    
  let handle_form_submit = dispatch.future_callback(|dispatch| async move {
    wasm_bindgen_futures::spawn_local(async move {
        let resp = api::api_change_course_info(
            dispatch.get().name.clone(),
            dispatch.get().desc.clone(),
            dispatch.get().short_desc.clone(),
            dispatch.get().image_url.clone(),
            dispatch.get().id.into()
        ).await;
    });
});
  let mut btn_name: String = String::from("Изменить информацию");
  if props.id == -1 {
    btn_name = String::from("Создать курс");
  }
    html! {
    <>
    <div class="container ">
    <div class="field">
  <p class="control has-icons-left has-icons-right">
    <label class="label">{"Название курса "}</label>
    <input class="input" type="text" onchange={name_changed}/>
    <span class="icon is-small is-left">
      <i class="fas fa-envelope"></i>
    </span>
    <span class="icon is-small is-right">
      <i class="fas fa-check"></i>
    </span>
  </p>
</div>
  <div class="field">
  <p class="control has-icons-left">
    <label class="label">{"Короткое описание"}</label>
    <textarea class="textarea" placeholder="" onchange={short_desc_changed}></textarea>
    <span class="icon is-small is-left">
      <i class="fas fa-lock"></i>
    </span>
  </p>
</div>
<div class="field">
  <p class="control has-icons-left">
    <label class="label">{"Полное описание"}</label>
    <textarea class="textarea is-large" placeholder="" onchange={desc_changed}></textarea>
    <span class="icon is-small is-left">
      <i class="fas fa-lock"></i>
    </span>
  </p>
</div>
<div class="field">
  <p class="control ">
    <label class="label">{"Ссылка на картинку"}</label>
    <input class="input" placeholder="" onchange={img_changed}/>
    <span class="icon is-small is-left">
      <i class="fas fa-lock"></i>
    </span>
  </p>
</div>
<div class="field">
  <p class="control">
    <button class="button is-success" onclick={handle_form_submit}>
      {btn_name}
    </button>
  </p>
</div>
</div>
    </>
    
    }
}
