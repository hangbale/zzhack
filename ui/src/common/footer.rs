use crate::contact::{ContactType, Contacts};
use crate::container::Container;
use utils::use_style;
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let style = use_style!(
        r"
        height: 139px;
        width: 100%;
        background: var(--base-color);

        .contacts {
            margin-top: 31px;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }

        .copyright {
            display: flex;
            justify-content: center;
        }

        .text {
            font-size: 14px;
        }
    "
    );

    html! {
        <div class={style}>
            <Container>
                <div>
                    <div class="contacts">
                        <div>
                            <div class="text">{"Powered by Rust & Yew"}</div>
                            <div class="text">{"Illustration by Icons 8 from Ouch!"}</div>
                        </div>
                        <div>
                            <Contacts source={vec![ContactType::GitHub, ContactType::Twitter, ContactType::Discord, ContactType::Email, ContactType::WeChat]} />
                        </div>
                    </div>
                    <div class="copyright text">{"Copyright © 2021 Mist"}</div>
                </div>
            </Container>
        </div>
    }
}