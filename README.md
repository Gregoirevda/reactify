// How components should be build

// Like current JS implementation. 
// Not possible to have unspecified number of arguments
// This is how promitives and custom components are made
fn React.createElement(type, props, children1, children2, ...) {}

// Like ReasonML implementation
// simple component, not really possible since too many opt args. OCaml has labbelled and optional
// args
fn div(foo, bar, [children]) {}
// Custom components:
fn MyComponent.make(foo, bar, [children]) {}

// Elm style
fn div([("foo", "bar")], [children])

// Builder pattern
fn div() {
    props: () => (),
    children: () => ()
};

enum Div([("foo", "bar")], ["children"])

// usage 
    div({style: {fontColor: 'red'}})
        .children(
            span('Text'),
        )
    .div({style: {fontColor: 'blue'}})
    
    div({style: {fontColor: 'red'}}, (
        span('Text'),
    ))
    .div({style: {fontColor: 'blue'}})

    div [ style [(fontColor, 'red')]] [
        span [ text "Text"]
    ]
    div [] []

    Div({style: fontColor: 'red'}, (
        Span("Text")        
    ))
    Div()

// Note on ReasonML JSX
<div foo={bar}> child1 child2 </div>;
// ->
([@JSX] div(~foo=bar, ~children=[child1, child2], ()));

ReactDOMRe.createElement("div", ~props={"foo": "bar"}, [|child1, child2|])

// Is the reason why Elm represents it with arrays because the language doesn't support optional
// and labelled arguments?
div([("foo", "bar")], [child1, child2]);

// Alternative in Rust
div
 .prop("foo", bar)
 .children([
    child1,
    child2
 ])

## Why JSX can not immediately be represented as a tree

Why not:
const element = <div>;
-> jsx preprocessor
const element = {
    type: "div",
    props: {}
};

Instead of:
const element = (bar) => <div foo={bar}> </div>;

-> jsx preprocessor

const element = (bar) => h("div", {foo: bar}, []);

-> runtime execution

const element = (bar) => {
    type: "div",
    props: {
        foo: bar
    }
};

=> the tree can contain state at runtime


let element = Div(Span())

// In Elm
div : List (Attribute msg) -> List (Html msg) -> Html msg

div attributes children = node "div" attributes children

node : String -> List (Attribute msg) -> List (Html msg) -> Html msg

node tag =
  Elm.Kernel.VirtualDom.node (Elm.Kernel.VirtualDom.noScript tag)



let div = node "div" [][]:

Html.div [] []
=
{ type = "node", tag = "div", facts = {}, children = {}, namespace = <internal structure>, descendantsCount = 0 }

in { type: str, props: { foo: "bar", children: [] } };

children is in props, and not
{ type: str, props: {}, children: [] }
because you you to give access to children in a custom component, like this.props.children


// Revery 
fn div([attributes], [children]) {
    primitiveComponent(Div(attributes), children)
}

// div([ class "test"], [])


// Or enums?
// Div([class "test" ], [])
