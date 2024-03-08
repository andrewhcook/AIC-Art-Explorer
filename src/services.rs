

use std::str::FromStr;

use serde_json::{Value};
use yew::{platform::spawn_local, prelude::*};
use gloo_net::http::Request;

#[derive(PartialEq, Eq, Clone)]
pub struct search_results {
    pub search_results: serde_json::Value
}

#[derive(PartialEq, Eq, Clone)]
pub enum search_endpoint {
    artworks,
    agents,
    places,
    galleries,
    exhibitions,
    category_terms
}

pub trait GetName {
    fn get_name(self) -> String;
} 

impl GetName for search_endpoint {
    fn get_name(self) -> String {
        match self {
            search_endpoint::artworks => {"artworks".to_string()},
            search_endpoint::agents => {"agents".to_string()},
            search_endpoint::places => {"places".to_string()},
            search_endpoint::galleries => {"galleries".to_string()},
            search_endpoint::exhibitions => {"exhibitions".to_string()},
            search_endpoint::category_terms => {"category-terms".to_string()}
        }
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct search_terms {
    pub search_type: search_endpoint,
    pub query: String,

}


#[derive(PartialEq, Properties, Clone)]
pub struct SearchBarProps {
    pub search_type: search_endpoint,
    pub query: String,
    pub search_results: UseStateHandle<Value>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct SearchResultsProps {
    pub search_results: UseStateHandle<Value>,
    pub selected_item: UseStateHandle<Value>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct IndividualResultProps {
    pub item: serde_json::Value,
    pub callback: UseStateHandle<Value>
}

#[derive(PartialEq, Properties, Clone)]
pub struct ItemViewerProps {
    pub item: serde_json::Value
}

pub fn search(search_state: UseStateHandle<Value>, search_terms: search_terms ) {
    
    let RAW_AIC_URL= format!("https://api.artic.edu/api/v1/{}/search?q={}", search_terms.search_type.get_name(), search_terms.query);
    spawn_local(async move {
        
            
            let response =
                Request::get(&RAW_AIC_URL)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
            if let Ok(search_results) = serde_json::Value::from_str(&response) {

                search_state.set(search_results);
            }
            
        
    });
}

pub fn get_main_item(item_handler: UseStateHandle<Value>,link: &str){
    let link = link.to_owned();
    
    spawn_local(async move {
        
        let link = link.clone();
        let response = Request::get(&link)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    
    if let Ok(search_result) = serde_json::Value::from_str(&response) {
    item_handler.set(search_result)
    }
}
);

}

pub fn fetch_image(image_handler: UseStateHandle<&str>, link: &str) {
    
}