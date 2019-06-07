extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};


#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct ClosureHandle(Closure<FnMut()>);

enum Prop {
    Attr(String, String),
    Listener(String, wasm_bindgen::prelude::Closure<Fn(web_sys::Event)>)
}

// Virtual element representation
#[wasm_bindgen]
pub struct VElement {
    type_: String,
    props: Vec<Prop>,
    children: Vec<VElement>,
}

// Keeps the relation between the virtual element and it's instantiation
// The users virtual tree is created, then only we instantiate it as a dom representation. 
// For this reason, we keep a reference to the virtual dom
// 
struct Instance<'a> {
    dom: web_sys::Node,
    element: &'a VElement,
    child_instances: Vec<Instance<'a>>,
}

struct Story {
    id: u32,
    name: String,
    url: String,
    likes: i32,
}

pub fn __reactify__increment_likes(story_id: u32) {
    log("Rust code")
    // render(app(), root_dom, root_instance);
}

fn app() -> VElement {
    let mut stories = vec![
        Story {
            id: 2,
            name: "World".to_string(),
            url: "http://bit.ly/2pX7HNn".to_string(),
            likes: 42,
        },
    ];

    fn handle_click(e: web_sys::Event) {
        log("clicked");
    }

    fn story_element(story: Story) -> VElement {
        li(
            vec![],
            vec![
                button(vec![on_click(handle_click)], vec![text(story.likes.to_string())]),
                a(vec![href(story.url)], vec![text(story.name)]),
            ],
        )
    }

    let app = div(
        vec![id("container".to_string())],
        vec![ul(
            vec![],
            stories
                .into_iter()
                .map(|story| story_element(story))
                .collect(),
        )],
    );
    app
}

fn get_element_by_id(id: &str) -> Option<web_sys::Element> {
    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");
    document.get_element_by_id(id)
}

#[wasm_bindgen]
pub fn run() -> VElement {
    let root_dom_opt = get_element_by_id("root");
    if let Some(root_dom) = root_dom_opt {
        // User passed a root dom that was found on the document
        let root_instance = None;
        let v_tree = render(app(), root_dom, root_instance);
        v_tree
    } else {
        VElement { type_: String::from(""), props: vec![], children: vec![] }
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
    root_instance: Option<Instance>,
) -> VElement {
    let previous_instance = root_instance;
    // container needs to be an element, but reconcile can be called with any node
    let next_instance = reconcile(
        &web_sys::Node::from(container),
        previous_instance,
        Some(&v_element),
    );
    v_element
}

fn reconcile<'a>(
    parent_dom: &web_sys::Node,
    instance: Option<Instance>,
    v_element: Option<&'a VElement>,
) -> Option<Instance<'a>> {
    if instance.is_none() {
        log("create instance");
        // Create instance
        let new_instance = instantiate(v_element);
        match new_instance {
            None => None,
            Some(new_instance) => {
                log("append to parent");
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
                let instance = update_dom_properties_instance(instance, &v_element.props);
                let instance = reconcile_children(instance, v_element);
                Some(Instance {
                    dom: instance.dom,
                    element: v_element,
                    child_instances: instance.child_instances,
                })
            }
            _ => None,
        }
    } else {
        log("replace_instance");
        // replace instance
        let new_instance = instantiate(v_element);
        match (new_instance, instance) {
            (Some(new_instance), Some(instance)) => {
                parent_dom.replace_child(&new_instance.dom, &instance.dom);
                Some(new_instance)
            }
            _ => None,
        }
    }
}

fn same_type(instance: &Option<Instance>, v_element: Option<&VElement>) -> bool {
    match instance {
        None => false,
        Some(instance) => match v_element {
            None => false,
            Some(v_element) => v_element.type_ == instance.element.type_,
        },
    }
}

// Creates the Instance Struct holding the Virtual Element and it's dom representatoin
// but doesn't append the created dom to the parent dom 
// Reconciler create branch calls instantiate and then appends it to the parent dom
// Reconciler replace branch calls instantiate and then updates it on the parent dom
// Child instances calls instantiate and appends it to the parent dom
fn instantiate<'a>(v_element: Option<&'a VElement>) -> Option<Instance> {
    match v_element {
        None => None,
        Some(v_element) => {
            let VElement {
                type_,
                props,
                children,
            } = v_element;

            let window = web_sys::window().expect("should have a Window");
            let document = window.document().expect("should have a Document");

            match type_.as_ref() {
                "text" => {
                    log("create a text node");
                    let mut node_value = "";
                    for prop in props {
                        if let Prop::Attr(name, value) = prop {
                            if name == "nodeValue" {
                                node_value = value;
                            }
                        }
                    }
                    let text_node = document.create_text_node(node_value);
                    let instance = Instance {
                        dom: web_sys::Node::from(text_node),
                        element: v_element,
                        child_instances: vec![],
                    };
                    Some(instance)
                }
                _ => {
                    log("create a node");
                    // Create the node with the type
                    let dom = document.create_element(&type_).expect("it to be there");

                    for child in children {
                        log("child");
                    }

                    // Currently simply adds the next props to the dom element
                    let dom = update_dom_properties(web_sys::Node::from(dom), &vec![], &props);
                    //Creates an instance holding the dom representation of the virtual dom and
                    //child instances
                    let instance = Instance {
                        child_instances: child_instances(&dom, children),
                        dom: dom,
                        element: v_element,
                    };
                    Some(instance)
                }
            }
        }
    }
}

// Instantiate all Virtual elements children and appends the created dom element to the parent dom
fn child_instances<'a>(dom: &web_sys::Node, children: &'a Vec<VElement>) -> Vec<Instance<'a>>{
    let mut child_instances = vec![];

    for child_element in children {
        if let Some(instance) = instantiate(Some(child_element)) {
            // Only Element has child instances, not the Text node. 
            // Public API doesn't accept text to have children
            dom
            .dyn_ref::<web_sys::Element>()
            .expect("Problem casting Node as Element")
            .append_child(&instance.dom);

            child_instances.push(instance);
        }
    }
    child_instances
}

// Adds next properties on the dom of the Instance struct
fn update_dom_properties_instance<'a>(
    instance: Instance<'a>,
    next_props: &Vec<Prop>,
) -> Instance<'a> {
    // TODO diff changes
    // Add attributes
    for prop in next_props {
        match prop {
            Prop::Listener(name, callback) => {
                instance.dom
                    .add_event_listener_with_callback(&name, callback.as_ref().unchecked_ref());
                //callback.forget();
            },
            Prop::Attr(name, value) => {
                // Attributes can only be set on Element, the public API only allows to set an
                // attribute on an Element. Not a text node
                instance.dom
                    .dyn_ref::<web_sys::Element>()
                    .expect("Problem casting Node as Element")
                    .set_attribute(&name, &value);
            },
        }
    }
    instance
}

// Adds next props to dom element
fn update_dom_properties(
    dom: web_sys::Node,
    prev_props: &Vec<Prop>,
    next_props: &Vec<Prop>,
) -> web_sys::Node {
    // TODO diff changes
    // Add attributes
    for prop in next_props {
        match prop {
            Prop::Attr(name, value) => {
                // Attributes can only be set on Element, the public API only allows to set an
                // attribute on an Element. Not a text node
                dom
                    .dyn_ref::<web_sys::Element>()
                    .expect("Problem casting Node as Element")
                    .set_attribute(&name, &value);
            },
            Prop::Listener(name, callback) => {
                dom
                    .add_event_listener_with_callback(&name, callback.as_ref().unchecked_ref());
            } 
        }
    }
    dom
}

fn reconcile_children<'a>(instance: Instance, element: &'a VElement) -> Instance<'a> {
    let Instance { dom, element: _, child_instances: _ } = instance;
    let child_instances = instance.child_instances;
    let VElement { type_, props, children: ref next_child_elements } = element;
    let mut new_child_instances: Vec<Instance> = vec![];

    if child_instances.len() < next_child_elements.len() {
        let mut child_instance_iter = child_instances.into_iter();
        for child_element in next_child_elements.into_iter() {
            let child_instance = child_instance_iter.next();

            let new_child_instance = reconcile(&dom, child_instance, Some(child_element));
            if let Some(new_child_instance) = new_child_instance {
                new_child_instances.push(new_child_instance);
            };
        }
    } else {
        let mut next_child_elements_iter = next_child_elements.into_iter();
        for child_instance in child_instances.into_iter() {
            let child_element = next_child_elements_iter.next();

            let new_child_instance = reconcile(&dom, Some(child_instance), child_element);
            if let Some(new_child_instance) = new_child_instance {
                new_child_instances.push(new_child_instance);
            };
        }
    };
    
    Instance {
        child_instances: new_child_instances,
        dom, 
        element
    } 
}

// create primitive components
fn primitive_component(
    type_: String,
    props: Vec<Prop>,
    children: Vec<VElement>,
) -> VElement {
    VElement {
        type_: type_,
        props: props,
        children: children,
    }
}

// helper functions to create primitive components
fn div(props: Vec<Prop>, children: Vec<VElement>) -> VElement {
    primitive_component("div".to_string(), props, children)
}

fn span(props: Vec<Prop>, children: Vec<VElement>) -> VElement {
    primitive_component("span".to_string(), props, children)
}

fn li(props: Vec<Prop>, children: Vec<VElement>) -> VElement {
    primitive_component("li".to_string(), props, children)
}

fn ul(props: Vec<Prop>, children: Vec<VElement>) -> VElement {
    primitive_component("ul".to_string(), props, children)
}

fn button(props: Vec<Prop>, children: Vec<VElement>) -> VElement {
    primitive_component("button".to_string(), props, children)
}

fn a(props: Vec<Prop>, children: Vec<VElement>) -> VElement {
    primitive_component("a".to_string(), props, children)
}

fn text(value: String) -> VElement {
    primitive_component(
        "text".to_string(),
        vec![Prop::Attr("nodeValue".to_string(), value)],
        vec![],
    )
}

// Attribute functions
fn on_click(callback: fn(e: web_sys::Event) -> ()) -> Prop {
    let closure_boxed_callback = Closure::wrap(Box::new(move |e| {
        callback(e);
    }) as Box<Fn(web_sys::Event)>);

    Prop::Listener("click".to_string(), closure_boxed_callback)
}

fn id(name: String) -> Prop {
    Prop::Attr("id".to_string(), name)
}

fn href(value: String) -> Prop {
    Prop::Attr("href".to_string(), value)
}
