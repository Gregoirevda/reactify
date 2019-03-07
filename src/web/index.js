import('../../pkg/reactify')
  .then(module => {
    console.log("module", module);
    module.run();
  })
  .catch(console.error);

