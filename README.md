# Reactify

### Introduction
The goal of this project is to port the React reconciler to Rust so we can compile it to Web Assembly and WASI. Meaning fastest execution thanks to Web Assembly on the web and WASI for macos, linux and windows.
The reconciler is just the tree diffing part and can be used with a Rust like React or any other language React (Including Javascript).
The reconciler is not platform agnostic. Each platform has it's renderer, Web Assembly for the web (DOM) and Rust ffi for macos/linux/windows and IOS/Android renderers.


## TODO
- useState and other hooks
- extract DOM and React like Virtual tree generator function
