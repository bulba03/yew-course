use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: AttrValue,
    pub surname: AttrValue,
    pub email: AttrValue,
    pub role: AttrValue
}

#[function_component]
pub fn UserInfo(props: &Props) -> Html {
html!{  
    <div class="cell">
    <div class="columns">
        <div class="column">{props.name.clone()}</div>
        <div class="column">{props.surname.clone()}</div>
        <div class="column">{props.role.clone()}</div>
        <div class="column">{props.email.clone()}</div>
        <div class="column">

            <button class="button is-info">
                {"Изменить"}
            </button>
           
        </div>
      </div>
    </div>
}
}