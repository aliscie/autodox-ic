use web_sys::{Document, Element, MouseEvent, window};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::{TitleBar, TitleBarButton};
use crate::components::TreeList;
use crate::utils::{FileNode, FileTree};
use yew::Html;

// use yew_router::prelude::*;


// use crate::router::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub id: u64,
}

#[function_component]
pub fn App(props: &Props) -> Html {
    let dispatch = Dispatch::<FileTree>::new();
    dispatch.reduce_mut(|r| {
        r.push_vertex(
            234,
            FileNode {
                id: 234,
                name: "FileOne".into(),
                data: "File one".into(),
            },
        );
        r.push_vertex(
            235,
            FileNode {
                id: 235,
                name: "FileTwo".into(),
                data: "File tow".into(),
            },
        );
        r.push_vertex(
            225,
            FileNode {
                id: 225,
                name: "FileThree".into(),
                data: "File three".into(),
            },
        );
    });
    dispatch.reduce_mut(|r| {
        r.push_edge(0, 234);
        r.push_edge(234, 235);
        r.push_edge(0, 225);
    });

    html! {
        // <BrowserRouter>
        <div>
        {"hello world"}
        // { super::utils::get_titlebar(article_position.clone(), x) }
        // <aside style={format!("{}",(*aside_bar_taggol).clone())}>

        <ul  id="myUL">
            <TreeList/>
        </ul>
        // </aside>

        // <article style={format!("{}",(*article_position).clone())}>
        // <h2 contenteditable="true" class={"heading"}>
        //   <Switch<Route> render={Switch::render(switch)} />
        //   </h2>
        //   <Editor/>
        // </article>
        </div>
        // </BrowserRouter>
    }
}


