use std::ops::Deref;

use gloo_console::log;
use yew::{ prelude::*, function_component, html, AttrValue, Html, Properties };
use yew_router::hooks::use_navigator;
use yewdux::{use_store, Store};

use crate::app::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i128,
    pub title: AttrValue,
    pub description: AttrValue,
    pub difficulty: i8,
    pub image_url: AttrValue,
}

#[function_component]
pub fn MyCourseCard(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let id = props.id;
    
    let onclick = Callback::from(move |_| navigator.push(&Route::Lesson { course_id: id.clone() as i32, lesson_id: 0 }));
    html! {
        <div class="card equal-height">
        <div class="card-image">
            <figure class="image is-4by3">
            <img
                src={props.image_url.clone()}
                alt="Placeholder image"
            />
            </figure>
        </div>

        <div class="card-content ">
            <div class="media">
            
            <div class="media-content">
                <p class="title is-4">{props.title.clone()}</p>
            </div>
            </div>

            <div class="content">
            // {"ss asodkasd askdpoaskdask sadkaspokdpsaokd akdpo askdpokaskpo pokpod askpdoska kpasokd oa"}
            {props.description.clone()}
            
            </div>
        </div>
        <div class="card-footer is-centered">
                <button class="card-footer-item button has-addons is-dark" onclick ={onclick}>{"Продолжить"}</button>
        
    </div>

        // </footer>
        </div>
        
        // </div>
    }
}
