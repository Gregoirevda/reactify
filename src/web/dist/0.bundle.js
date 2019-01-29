(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../../pkg/reactify.js":
/*!******************************************************************!*\
  !*** /Users/gregoirevda/dev/opensource/reactify/pkg/reactify.js ***!
  \******************************************************************/
/*! exports provided: __wbg_createelement_afabc603f686561f, __wbg_appendchild_274ae14d62037d85, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_createelement_afabc603f686561f\", function() { return __wbg_createelement_afabc603f686561f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_appendchild_274ae14d62037d85\", function() { return __wbg_appendchild_274ae14d62037d85; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _reactify_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./reactify_bg */ \"../../pkg/reactify_bg.wasm\");\n/* harmony import */ var _src_web_dom__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../src/web/dom */ \"./dom.js\");\n/* harmony import */ var _src_web_dom__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(_src_web_dom__WEBPACK_IMPORTED_MODULE_1__);\n/* tslint:disable */\n\n\n\n\nlet cachedTextDecoder = new TextDecoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _reactify_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_reactify_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nlet cachedTextEncoder = new TextEncoder('utf-8');\n\nlet WASM_VECTOR_LEN = 0;\n\nfunction passStringToWasm(arg) {\n\n    if (typeof(arg) !== 'string') throw new Error('expected a string argument');\n\n    const buf = cachedTextEncoder.encode(arg);\n    const ptr = _reactify_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"](buf.length);\n    getUint8Memory().set(buf, ptr);\n    WASM_VECTOR_LEN = buf.length;\n    return ptr;\n}\n\nlet cachegetUint32Memory = null;\nfunction getUint32Memory() {\n    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== _reactify_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint32Memory = new Uint32Array(_reactify_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint32Memory;\n}\n\nfunction __wbg_createelement_afabc603f686561f(ret, arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    try {\n\n        const retptr = passStringToWasm(Object(_src_web_dom__WEBPACK_IMPORTED_MODULE_1__[\"create_element\"])(varg0));\n        const retlen = WASM_VECTOR_LEN;\n        const mem = getUint32Memory();\n        mem[ret / 4] = retptr;\n        mem[ret / 4 + 1] = retlen;\n\n    } catch (e) {\n        console.error(\"wasm-bindgen: imported JS function that was not marked as `catch` threw an error:\", e);\n        throw e;\n    }\n}\n\nfunction __wbg_appendchild_274ae14d62037d85(ret, arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    try {\n\n        const retptr = passStringToWasm(Object(_src_web_dom__WEBPACK_IMPORTED_MODULE_1__[\"append_child\"])(varg0));\n        const retlen = WASM_VECTOR_LEN;\n        const mem = getUint32Memory();\n        mem[ret / 4] = retptr;\n        mem[ret / 4 + 1] = retlen;\n\n    } catch (e) {\n        console.error(\"wasm-bindgen: imported JS function that was not marked as `catch` threw an error:\", e);\n        throw e;\n    }\n}\n\nfunction __wbindgen_throw(ptr, len) {\n    throw new Error(getStringFromWasm(ptr, len));\n}\n\n\n\n//# sourceURL=webpack:////Users/gregoirevda/dev/opensource/reactify/pkg/reactify.js?");

/***/ }),

/***/ "../../pkg/reactify_bg.wasm":
/*!***********************************************************************!*\
  !*** /Users/gregoirevda/dev/opensource/reactify/pkg/reactify_bg.wasm ***!
  \***********************************************************************/
/*! exports provided: memory, __rustc_debug_gdb_scripts_section__, main, __wbindgen_malloc */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./reactify */ \"../../pkg/reactify.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:////Users/gregoirevda/dev/opensource/reactify/pkg/reactify_bg.wasm?");

/***/ }),

/***/ "./dom.js":
/*!****************!*\
  !*** ./dom.js ***!
  \****************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("function create_element(type) {\n  return document.createElement(type);\n}\n\nfunction append_child(dom) {\n  const parentDom = document.getElementById(\"root\");\n  return parentDom.appendChild(dom);\n}\n\n\n//# sourceURL=webpack:///./dom.js?");

/***/ })

}]);