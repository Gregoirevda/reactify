import('../../pkg/reactify')
  .then(module => {
    console.log("module", module);
    module.run();
    Object.entries(module)
    .map(([key, value]) => {
      if(isReactifyFunction(key)) {
        console.log(key, value)
        window[key] = value;
      }
    });    
  })
  .catch(console.error);


function isReactifyFunction(name) {
  return name.startsWith("__reactify__");
}
