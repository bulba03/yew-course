use std::ops::Deref;

use gloo_console::log;
use serde::{ Deserialize, Serialize };
use yew::{ function_component, prelude::* };
use yewdux::{ use_store, Store };

use crate::{ api::{ self, ApiCourseInfoData }, components::{admin_course_card::AdminCourseCard, my_course_card::MyCourseCard} };
pub struct AdminCourses {
    courses: Vec<ApiCourseInfoData>,
}
pub enum Msg {
    FetchCourses,
    SetCourses(Vec<ApiCourseInfoData>),
}

impl Component for AdminCourses {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        let data = Self {
            courses: Vec::new(),
        };
        _ctx.link().send_message(Msg::FetchCourses);
        data
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!(self.courses.len());
        let course_cards = self.courses.iter().map(|data| {
            html! { <AdminCourseCard id={data.id as i128} description={data.short_desc.clone().unwrap()} difficulty=1 title={data.name.clone()} image_url={data.image.clone().unwrap()}/> }
        });
        html! {
        <div class="container is-fluid">
                    <div class="fixed-grid has-4-cols">
                        <div class="grid">
                        {
                            for course_cards
                        }
                        // <MyCourseCard id = 0 title="C++ с нуля" description="Курс для базового ознакомления с одним из главных языков в программировании" difficulty=1 image_url="https://ip-calculator.ru/blog/wp-content/uploads/2021/02/6038586442907648.png"/>
                        // <AdminCourseCard id = 1 title="Язык программирования Rust" description="Курс подойдет программистам любого уровня, которые хотят изучить новые возможности в программировании" difficulty=1 image_url="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg"/>
                        // // <MyCourseCard id = 0 title="Лидерство в коллективе" description="Короткий курс с практическим обучением. Без воды и долгих речей. Только основные навыки." difficulty=1 image_url="https://static.tildacdn.com/tild3039-3134-4032-a637-663335343332/-7-.jpg"/>
                        // <AdminCourseCard id = 1 title="Программирование на C#: от новичка до специалиста" description="Изучите C# и платформу .NET, включая .NET Core и начните практиковать объектно-ориентированное программирование (ООП)." difficulty=1 image_url="https://248006.selcdn.ru/main/iblock/9dd/9dd3c730431af9dbab7befd41814fcea/13eaa548ad580af37b96dfbd474da753.png"/>
                     </div>
                    </div>
                 </div>
                }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchCourses => {
                ctx.link().send_future(async {
                    let resp = api::api_get_courses().await;
                    return Msg::SetCourses(resp);
                });

            }
            Msg::SetCourses(data) => {
                self.courses.extend(data);
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        self.courses.clear()
    }
}