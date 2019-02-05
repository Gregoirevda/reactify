extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;

use wasm_bindgen::prelude::*;


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
    props: Vec<(String, String)>,
    children: Option<Vec<VElement>>
}

struct Instance<'a> {
    dom: web_sys::Node,
    element: &'a VElement,
    child_instances: Vec<Instance<'a>>
}

#[wasm_bindgen(start)]
pub fn run() {
    // create primitive components
    let div = |props, children| primitive_component("div".to_string(), props, children);
    let span = |props, children| primitive_component("span".to_string(), props, children);
    let text = |value| primitive_component("text".to_string(), vec![("nodeValue".to_string(), value)], vec![]);

    let v_element = 
        div(vec![id("container")], vec![
            span(vec![id("child"), on_click("console.log(arguments)".to_string())], vec![
                text("Hello World".to_string())
            ])
        ]);

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");
    let root_dom_opt = document.get_element_by_id("root");

    if let Some(root_dom) = root_dom_opt {
        // User passed a root dom that was found on the document
       let root_instance = None;
       tick(&v_element, &root_dom, 0, root_instance);
    }
}

fn tick(v_element: &VElement, root_dom: &web_sys::Element, count: u32, root_instance: Option<Instance>){
    let next_root_instance = render(v_element, root_dom, root_instance);
    if count < 10 { 
        tick(v_element, root_dom, count + 1, next_root_instance);
    }
}

fn render<'a>(
    v_element: &'a VElement, 
    container: &web_sys::Element, 
    root_instance: Option<Instance<'a>>
) -> Option<Instance<'a>> {
    let previous_instance = root_instance;
    let next_instance = reconcile(container, previous_instance, Some(v_element));
    next_instance
}

fn reconcile<'a>(
    parent_dom: &web_sys::Element, 
    instance: Option<Instance<'a>>,
    v_element: Option<&'a VElement>
) -> Option<Instance<'a>> {
    if instance.is_none() {
        log("create instance");
        // Create instance
        let new_instance = instantiate(v_element);
        match new_instance {
            None => None,
            Some(new_instance) => {
             parent_dom.append_child(&new_instance.dom);
             Some(new_instance)
            }
        }
   } else if v_element.is_none() {
        log("remove instance");
        // Remove instance 
        if let Some(instance) = instance {
            parent_dom.remove_child(&instance.dom);
        }
        None
   } else if same_type(&instance, v_element) {
        log("update instance");
        // Update instance
        match (instance, v_element) {
            (Some(instance), Some(v_element)) => {
                // TODO update_dom_properties is called with a web_sys::Node and web_sys::Element (down)
                // The function needs to call set_attribute, which only exist on Element.
                // there is no way to know if the param is a Node or Element, Element and Text
                // should be divided in 2 enums
                // update_dom_properties(instance.dom, instance.element.props, element.props);
                Some(instance)
            },
            _ => None
        }
   } else {
       log("replace_instance");
        // replace instance
        let new_instance = instantiate(v_element);
        match (new_instance, instance) {
            (Some(new_instance), Some(instance)) => {
                parent_dom.replace_child(&new_instance.dom, &instance.dom);
                Some(new_instance)
            },
            _ => None
        }
   }
}

fn same_type(instance: &Option<Instance>, v_element: Option<&VElement>) -> bool {
    match instance {
        None => false,
        Some(instance) => 
            match v_element {
                None => false,
                Some(v_element) => v_element.type_ == instance.element.type_
            }
    }
}

fn instantiate<'a>(v_element: Option<&'a VElement>) -> Option<Instance<'a>> {
    match v_element {
        None => None,
        Some(v_element) => {
            let VElement { type_, props, children } = v_element;
            
            let window = web_sys::window().expect("should have a Window");
            let document = window.document().expect("should have a Document");

            match type_.as_ref() {
                "text" => {
                    let mut node_value = "";
                    for (name, value) in props {
                      if name == "nodeValue" {
                        node_value = value;  
                      }  
                    }
                    let text_node = document.create_text_node(node_value);
                    let instance = Instance {
                        dom: web_sys::Node::from(text_node),
                        element: v_element,
                        child_instances: vec![]
                    };
                    Some(instance)
                },
                _ => {
                    let dom = document.create_element(&type_)
                        .expect("it to be there");

                    update_dom_properties(&dom, &vec![], &props);

                    match children {
                        None => None,
                        Some(child_elements) => {
                            let mut child_instances = vec![];
                            for child_element in child_elements {
                                if let Some(instance) = instantiate(Some(child_element)) {
                                    child_instances.push(instance);
                                }
                            };
                            let mut child_doms = vec![];
                            for child_instance in &child_instances {
                                child_doms.push(Some(&child_instance.dom));
                            };

                            for child_dom in child_doms {
                                if let Some(child_dom) = child_dom {
                                    dom.append_child(&child_dom);
                                };
                            };

                            let instance = Instance {
                                dom: web_sys::Node::from(dom),
                                element: v_element,
                                child_instances: child_instances
                            };
                            Some(instance)
                        }
                    }
                }
            }
        }
    }
}

fn update_dom_properties(
    dom: &web_sys::Element, 
    prev_props: &Vec<(String, String)>, 
    next_props: &Vec<(String, String)>
) {
    // Add attributes
    for (name, value) in prev_props {
        if is_listener(&name) {
            let callback = js_sys::Function::new_no_args(&value);
            dom.add_event_listener_with_callback(&name.to_lowercase()[2..], &callback); 
        } else {
            dom.set_attribute(&name, &value);
        }
    }
}

// create primitive components
fn primitive_component(type_: String, props: Vec<(String, String)>, children: Vec<VElement>) -> VElement {
    // Convert [(String, String)] -> HashMap<String, String>;
    // VElement.props can't be a &[(String, String)]

    let children = match children.len() {
        0 => None,
        _ => Some(children)
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
