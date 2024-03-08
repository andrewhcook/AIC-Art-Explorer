use std::{str::FromStr};
use web_sys::{Event, EventTarget};

use yew::prelude::*;
mod services;
use web_sys::HtmlInputElement;
use crate::services::*;

/// todo: add a use_state for item_focus
/// todo: edit search_results_props and pass new use_state in with those props
/// todo: add an onclick on each search result to set item_focus

/// make Search Results a self-contained container
/// format to look pretty with css
/// remove hello world (replace with website header)
/// 



#[function_component]
fn App() -> Html {
    let search_results_handle = use_state(|| serde_json::Value::from_str("{}").unwrap());
    let search_terms = use_state(|| search_terms {search_type: search_endpoint::artworks, query: "".to_string()});
    let selected_item = use_state(|| serde_json::Value::from_str("{}").unwrap());
  html!{  <>
    <h2> {"Explore the Art Institute of Chicago's Collections!"} </h2>
    <div id = "container">
    <SearchBar search_type = {search_terms.search_type.clone()} query =  {search_terms.query.clone()} search_results = {search_results_handle.clone()}> </SearchBar>
    <ItemViewer item = {(*selected_item).clone()}> </ItemViewer>
    <SearchResults search_results = {search_results_handle.clone()} selected_item= {selected_item}> </SearchResults>
    </div>
     </>
    }
}


#[function_component]
pub fn SearchBar(SearchBarProps {search_type, query, search_results}: &SearchBarProps) -> Html {
    let search_type_clone = search_type.clone();
    let search_results_clone = search_results.clone();
    let search_query_handle = use_state(|| query.clone());
    let search_query = search_query_handle.clone();

    let handle_input = Callback::from(move |input_event: InputEvent| {
        let option_elem= input_event.target_dyn_into::<HtmlInputElement>();
        if let Some(option_elem) = option_elem {

            search_query_handle.set(option_elem.value())
        }
        
    });
    let on_submit_handler = Callback::from( move |_| {
        
        search(search_results_clone.clone(), search_terms {search_type: search_type_clone.clone(), query: search_query.clone().to_string()})
    });

    html! {
        <>
        <div id = "searchBar">
        <input oninput = {handle_input} /> 
        <button onclick = {
            on_submit_handler
        } > {"Search"} </button>
</div>
       
         </>
    }
}



#[function_component]
pub fn SearchResults( SearchResultsProps{search_results, selected_item }: &SearchResultsProps ) -> Html {
    let data= (*search_results)["data"].as_array();
    
    //let onclick = Callback::from(move |_| selected_item_id.set(item["id"].copy()));
    if let Some(unpacked_data) = data {
        
        html! {
            <> 
            <div id = "searchResults">
            <h3> {"Search Results"} </h3>
            <ul class = "item-list">
            {unpacked_data.iter().map(| item| html! {<li> <IndiviudalResult item = {item.clone()} callback = {selected_item.clone()}> </IndiviudalResult>  </li> }).collect::<Html>()} 
        </ul>
        
    </div>
    </>
        
        }
    } else {
        html! {<> </>}
    }
}

#[function_component]
fn IndiviudalResult(IndividualResultProps {item, callback}: &IndividualResultProps) -> Html {
    
    let callback = callback.clone();
    let other_copy = item.clone();
    let item = item.clone();
    let link_attempt = item["api_link"].as_str();
    let item_handler = use_state_eq(|| serde_json::Value::from_str("{}").unwrap());
    if let Some(link) = link_attempt {
        get_main_item(item_handler.clone(), link)
    };
    let mut full_url = "".to_owned();
    if let Some(img_base_url) = (*item_handler).clone()["config"]["iiif_url"].as_str() {
        
        let image_id = (*item_handler).clone()["data"]["image_id"].as_str().unwrap().to_owned();
        full_url = format!("{}/{}/full/200,/0/default.jpg",img_base_url, image_id.clone());



    }
    html! {
        <div class = "indie-result">
        <div onclick = { move |_| callback.set(item.clone())}> {
            html! {
                <>
            <div> {format!("Title: {}", other_copy.clone()["title"].as_str().unwrap_or_default())} </div>
            <div> {format!("Artist: {}", other_copy.clone()["artist"].as_str().unwrap_or_default())} </div>
            <div> {"Thumbnail: "} <img src = {full_url} /> </div>
         
            </>
            }
        }
             </div>
             </div>
    }
}

#[function_component]
fn ItemViewer(ItemViewerProps {item} : &ItemViewerProps) -> Html {
    //let item = item.clone();
    let link_attempt = item["api_link"].as_str();
    let item_handler = use_state_eq(|| serde_json::Value::from_str("{}").unwrap());
    if let Some(link) = link_attempt {
        get_main_item(item_handler.clone(), link)
    };
    let mut full_url = "".to_owned();
    if let Some(img_base_url) = (*item_handler).clone()["config"]["iiif_url"].as_str() {
        
        let image_id = (*item_handler).clone()["data"]["image_id"].as_str().unwrap().to_owned();
        full_url = format!("{}/{}/full/200,/0/default.jpg",img_base_url, image_id.clone());



    }
    //dimension title artist nationality year medium description thumbnail
    html! {
        <>
        <div id  = "ItemViewer">
        <div> {format!("Title: {}",(*item_handler.clone())["data"]["title"].as_str().unwrap_or_default())} </div>
        <div> {format!("Artist: {}",(*item_handler.clone())["data"]["artist"].as_str().unwrap_or_default())} </div>
        <div> {format!("Date First Displayed: {}",(*item_handler.clone())["data"]["date_display"].as_str().unwrap_or_default())} </div>
        <div> {format!("Place of Origin: {}",(*item_handler.clone())["data"]["place_of_origin"].as_str().unwrap_or_default())} </div>
        <div> {format!("Medium: {}",(*item_handler.clone())["data"]["medium_display"].as_str().unwrap_or_default())} </div>
        <div> {"Thumbnail: "}<img src= {full_url} / >  </div>
        <div> {format!("Description: {}",(*item_handler.clone())["data"]["description"].as_str().unwrap_or_default())} </div>
        <div> {format!("api-link: {}",(*item_handler.clone())["data"]["api_link"].as_str().unwrap_or_default())} </div>
        </div>
        </>
    }
    //{format!("{}", *item_handler.clone())}
}

fn main() {
    yew::Renderer::<App>::new().render();
}