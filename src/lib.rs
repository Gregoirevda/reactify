extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::cmp;
use std::iter;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Clone)]
struct VElement {
    type_: String,
    props: Vec<(String, String)>,
    children: Vec<VElement>
}

#[derive(Clone)]
enum Node {
    Element(web_sys::Element),
    Text(web_sys::Text),
}

#[derive(Clone)]
struct Instance {
    dom: Node,
    element: VElement,
    child_instances: Vec<Instance>
}

#[wasm_bindgen(start)]
pub fn run() {
    // create primitive components
    let div = |props, children| primitive_component("div".to_string(), props, children);
    let span = |props, children| primitive_component("span".to_string(), props, children);
    let li = |props, children| primitive_component("span".to_string(), props, children);
    let button = |props, children| primitive_component("button".to_string(), props, children);

    let a = |props, children| primitive_component("a".to_string(), props, children);

    let text = |value| primitive_component("text".to_string(), vec![("nodeValue".to_string(), value)], vec![]);

    struct Story {
        name: String,
        url: String,
        likes: i32
    }

    let stories = vec![Story {
        name: "Hello".to_string(),
        url: "http://bit.ly/2pX7HNn".to_string(),
        likes: 32
    }, Story {
        name: "World".to_string(),
        url: "http://bit.ly/2pX7HNn".to_string(),
        likes: 42
    }];

    let v_element = 
        div(vec![id("container")], vec![
            li(vec![], vec![
               button(vec![], vec![
                  text("Click me".to_string())
               ]),
               a(vec![href("link")], vec![
                  text("href".to_string())
               ])
            ])
        ]);

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");
    let root_dom_opt = document.get_element_by_id("root");

    if let Some(root_dom) = root_dom_opt {
        // User passed a root dom that was found on the document
       let root_instance = None;
       let next_root_instance = render(v_element, root_dom, root_instance);
    }
}

/*
fn tick(v_element: &VElement, root_dom: &web_sys::Element, count: u32, root_instance: Option<Instance>){
    if count < 10 { 
        let next_root_instance = render(v_element, root_dom, root_instance);
        tick(v_element, root_dom, count + 1, next_root_instance);
    }
}
*/

fn render(
    v_element: VElement, 
    container: web_sys::Element, 
    root_instance: Option<Instance>
) -> Option<Instance> {
    let previous_instance = root_instance;
    // container needs to be an element, but reconcile can be called with any node
    let next_instance = reconcile(&web_sys::Node::from(container), previous_instance, Some(v_element));
    next_instance
}

fn reconcile(
    parent_dom: &web_sys::Node, 
    instance: Option<Instance>,
    v_element: Option<VElement>
) -> Option<Instance> {
    if instance.is_none() {
        log("create instance");
        // Create instance
        let new_instance = instantiate(&v_element);
        match new_instance {
            None => None,
            Some(new_instance) => {
                log("append to parent");
                match &new_instance.dom {
                    Node::Element(dom_element) => parent_dom.append_child(dom_element),
                    Node::Text(dom_text) => parent_dom.append_child(dom_text),
                };
                Some(new_instance)
            }
        }
   } else if v_element.is_none() {
        log("remove instance");
        // Remove instance 
        if let Some(instance) = instance {
            match &instance.dom {
                Node::Element(dom_element) => parent_dom.remove_child(&dom_element),
                Node::Text(text_element) => parent_dom.remove_child(&text_element),
            };
        }
        None
   } else if same_type(&instance, &v_element) {
        log("update instance");
        // Update instance
        match (instance, v_element) {
            (Some(instance), Some(v_element)) => {
                // TODO pass instance and return instance with attrs and all set on instance.dom
                let instance = update_dom_properties_instance(instance, &v_element.props);
                let child_instances = reconcile_children(instance.clone(), v_element.clone());
                Some(Instance { dom: instance.dom, element: v_element, child_instances })
            },
            _ => None
        }
   } else {
       log("replace_instance");
        // replace instance
        let new_instance = instantiate(&v_element);
        match (new_instance, instance) {
            (Some(new_instance), Some(instance)) => {
                match (&new_instance.dom, &instance.dom) {
                    (Node::Element(n_dom_element), Node::Element(dom_element)) => parent_dom.replace_child(&n_dom_element, &dom_element),
                    (Node::Text(n_dom_text), Node::Text(dom_text)) => parent_dom.replace_child(&n_dom_text, &dom_text),
                    (Node::Element(n_dom_element), Node::Text(dom_text)) => parent_dom.replace_child(&n_dom_element, &dom_text),
                    (Node::Text(n_dom_text), Node::Element(dom_element)) => parent_dom.replace_child(&n_dom_text, &dom_element),
                };
                Some(new_instance)
            },
            _ => None
        }
   }
}

fn same_type(instance: &Option<Instance>, v_element: &Option<VElement>) -> bool {
    match instance {
        None => false,
        Some(instance) => 
            match v_element {
                None => false,
                Some(v_element) => v_element.type_ == instance.element.type_
            }
    }
}

fn instantiate(v_element: &Option<VElement>) -> Option<Instance> {
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
                    let text_node = document.create_text_node(&node_value);
                    let instance = Instance {
                        dom: Node::Text(text_node),
                        element: v_element.clone(),
                        child_instances: vec![]
                    };
                    Some(instance)
                },
                _ => {
                    log("create div");
                    let dom = document.create_element(&type_)
                        .expect("it to be there");

                    let dom = update_dom_properties(Node::Element(dom), &vec![], &props);

                    let child_elements = children;

                    let mut child_instances = vec![];
                    for child_element in child_elements {
                        log("instantiate children");
                        if let Some(instance) = instantiate(&Some(child_element.clone())) {
                            if let Node::Element(dom) = &dom {
                                match &instance.dom {
                                    Node::Element(child_dom) => dom.append_child(child_dom),
                                    Node::Text(child_dom) => dom.append_child(child_dom),
                                };
                            };
                            child_instances.push(instance);
                        }
                    };

                    let instance = Instance {
                        dom: dom,
                        element: v_element.clone(),
                        child_instances: child_instances
                    };
                    Some(instance)
                }
            }
        }
    }
}

fn update_dom_properties_instance(
    instance: Instance,
    next_props: &Vec<(String, String)>
) -> Instance {
    // TODO diff changes
    // Add attributes
    for (name, value) in &instance.element.props {
        if is_listener(&name) {
            let callback = js_sys::Function::new_no_args(&value);
            match &instance.dom {
                Node::Element(dom_element) => dom_element.add_event_listener_with_callback(&name.to_lowercase()[2..], &callback),
                Node::Text(dom_text) => dom_text.add_event_listener_with_callback(&name.to_lowercase()[2..], &callback),
            };
        } else {
            if let Node::Element(dom_element) = &instance.dom {
                dom_element.set_attribute(&name, &value);
            }
        }
    };
    instance
}
fn update_dom_properties(
    dom: Node, 
    prev_props: &Vec<(String, String)>, 
    next_props: &Vec<(String, String)>
) -> Node {
    // TODO diff changes
    // Add attributes
    for (name, value) in prev_props {
        if is_listener(&name) {
            let callback = js_sys::Function::new_no_args(&value);
            match &dom {
                Node::Element(dom_element) => dom_element.add_event_listener_with_callback(&name.to_lowercase()[2..], &callback),
                Node::Text(dom_text) => dom_text.add_event_listener_with_callback(&name.to_lowercase()[2..], &callback),
            };
        } else {
            if let Node::Element(dom_element) = &dom {
                dom_element.set_attribute(&name, &value);
            }
        }
    };
    dom
}

fn reconcile_children(instance: Instance, element: VElement) -> Vec<Instance> {
    let dom = instance.dom;
    let child_instances = instance.child_instances;
    let next_child_elements = element.children;
    let mut new_child_instances: Vec<Instance> = vec![];

    let dom: web_sys::Node = match dom {
        Node::Element(dom_element) => web_sys::Node::from(dom_element),
        Node::Text(dom_text) => web_sys::Node::from(dom_text)
    };

    let max_children = cmp::max(child_instances.len(), next_child_elements.len());
    // TODO should children by Optional?, would avoid extra mapping cost
    let iter = child_instances
            .into_iter()
            .map(|n| Some(n))
            .chain(iter::repeat(None))
        .zip(
            next_child_elements
            .into_iter()
            .map(|n| Some(n))
            .chain(iter::repeat(None))
        )
        .take(max_children);

    for (child_instance, child_element) in iter {
        let new_child_instance = reconcile(&dom, child_instance, child_element);
        if let Some(new_child_instance) = new_child_instance {
            new_child_instances.push(new_child_instance);
        };
    }

    new_child_instances
}

// create primitive components
fn primitive_component(type_: String, props: Vec<(String, String)>, children: Vec<VElement>) -> VElement {
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

fn href(value: &str) -> (String, String) {
    ("href".to_string(), value.to_string())
}

// Helper functions
fn is_listener(attribute_name: &str) -> bool {
    attribute_name.starts_with("on")
}
