const rust = import('./pkg/index')

rust.then((m) => {
  m.run(1).then((res) => console.log(res));
  m.run(365).then(console.log);
}).catch(console.error);

