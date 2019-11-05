const rust = import('./pkg/wasm')

rust.then(m => m.greet('World!'))
  .catch(console.error);

