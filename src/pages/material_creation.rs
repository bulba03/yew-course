use yew::{function_component, html, Html};


#[function_component]
pub fn MaterialCreation() -> Html {
    html! {
    <>
    <div class="container ">
    <div class="field">
    <label class="label">{"ID курса"}</label>
    <input class="input" type="text" placeholder=""/>
  <p class="control has-icons-left has-icons-right">
    <label class="label">{"Название урока (главы) "}</label>
    <input class="input" type="text" placeholder=""/>
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
    <label class="label">{"Содержимое урока (в формате Markdown)"}</label>
    <textarea class="textarea" placeholder=""></textarea>
    <span class="icon is-small is-left">
      <i class="fas fa-lock"></i>
    </span>
  </p>
</div>
<div class="field">
  <p class="control">
    <button class="button is-success">
      {"Добавить урок"}
    </button>
  </p>
</div>
</div>
    </>
    
    }
}
