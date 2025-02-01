# JavaScript bindings for Book

You will need to have recent versions of [Rust](https://www.rust-lang.org/tools/install), [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), [TypeScript](https://www.typescriptlang.org/download) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed in your system.

## Build and test JavaScript

Run (this may take several minutes):

```bash
$ wasm-pack build --target web
```

Then:

```bash
$ python3 -m http.server -b 127.0.0.1 8000
```

Open http://127.0.0.1:8000 in your browser, and you should see:

```
Have a look at the console!
```

The console should contain something like:

```
Friends by name Uhlenbeck:  2
First of the Uhlenbecks: Object
Object
Exception raised as expected:  You already have that friend
```

## Build and test TypeScript bindings

You can optionally build bindings for TypeScript. Run:

```
$ make all
```

And then:

```
$ make serve
```

Open http://127.0.0.1:8000 in your browser, and you should see similar results as in the JavaScript test. Specifically,
the following text in the browser:


```
Have a look at the console!
```

And the console should contain something like:

```
Friends by name Uhlenbeck:  2
First of the Uhlenbecks: Object
Object
Exception raised as expected:  You already have that friend
```
