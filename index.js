const rust = import('./pkg/index')

rust.then(m => m.greet('World!'))
  .catch(console.error);

