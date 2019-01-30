extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

struct VElement {
    type_: String,
    props: HashMap<String, String>,
    children: Option<Vec<VElement>>
}

#[wasm_bindgen(start)]
pub fn run() {
    // create primitive components
    let div = |props, children| primitive_component("div".to_string(), props, children);
    let span = |props, children| primitive_component("span".to_string(), props, children);
    let text = |value| primitive_component("text".to_string(), vec![("value".to_string(), value)], vec![]);

    let v_element = div(vec![id("container")], vec![
                span(vec![id("child"), on_click("console.log(arguments)".to_string())], vec![
                    text("Hello World".to_string())
                ])
            ]
        );

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");
    let root_dom_opt = document.get_element_by_id("root");

    if let Some(root_dom) = root_dom_opt {
        render(v_element, &root_dom);
    }
}

fn render(v_element: VElement, parent_dom: &web_sys::Element) -> Result<(), JsValue> {
    let VElement { type_, props, children } = v_element;
    
    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    match type_.as_ref() {
        "text" => {
           if let Some(text) = props.get("value") {
            let txt_dom = document.create_text_node(text);
            parent_dom.append_child(&txt_dom);
           }
        },
        _ => {
            let dom = document.create_element(&type_)?;

            // Add attributes
            for (name, value) in props {
                if is_listener(&name) {
                    let callback = js_sys::Function::new_no_args(&value);
                    dom.add_event_listener_with_callback(&name.to_lowercase()[2..], &callback); 
                } else {
                    dom.set_attribute(&name, &value);
                }
            }

            if let Some(childElements) = children {
                for childElement in childElements {
                    render(childElement, &dom);
                }
            }

            parent_dom.append_child(&dom)?;
        }
    };

    Ok(())
}

// create primitive components
fn primitive_component(type_: String, array_prop: Vec<(String, String)>, _children: Vec<VElement>) -> VElement {
    // Convert [(String, String)] -> HashMap<String, String>;
    // VElement.props can't be a &[(String, String)]
    let mut props = HashMap::new();
    for prop in array_prop {
        let (name, value) = prop;
        props.insert(name.to_string(), value.to_string());
    }

    let children = match _children.len() {
        0 => None,
        _ => Some(_children)
    };

   VElement {
        type_: type_,
        props: props,
        children: children
    }
}

// Attribute functions
fn on_click(js_fn: String) -> (String, String) {
    ("onClick".to_string(), js_fn)
}

fn id(name: &str) -> (String, String) {
    ("id".to_string(), name.to_string())
}

// Helper functions
fn is_listener(attribute_name: &str) -> bool {
    attribute_name.starts_with("on")
}
