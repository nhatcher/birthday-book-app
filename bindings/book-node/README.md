# Node.js bindings for Book

We use [napi-rs](https://napi.rs/) to manage the Node.js bindings.

## Build

Make sure your Node.js version is up to date (^16.17 or >=18.6).

```bash
$ npm install
$ npm run build
```

And then, this should confirm everything is working:

```
$ node
Welcome to Node.js v19.8.1.
Type ".help" for more information.
> const book = require("./index");
undefined
```

## Tests

To run the tests:

```
$ npm run test
```
