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
    // Generate state representation
    let mut props1 = HashMap::new();
    props1.insert(String::from("id"), String::from("container"));
    props1.insert(String::from("onClick"), "console.log('click')".to_string());

    let mut props2 = HashMap::new();
    props2.insert(String::from("id"), String::from("child"));

    let mut txtProps = HashMap::new();
    txtProps.insert("value".to_string(), "click me".to_string());

    let text = VElement {
        type_: String::from("text"),
        props: txtProps,
        children: None
    };

    let child = VElement {
        type_: String::from("span"),
        props: props2,
        children: Some(vec![text])
    };

    let v_element = VElement {
        type_: String::from("div"),
        props: props1,
        children: Some(vec![child])
    };

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
                if isListener(&name) {
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

// Helper functions
fn isListener(attribute_name: &str) -> bool {
    attribute_name.starts_with("on")
}
