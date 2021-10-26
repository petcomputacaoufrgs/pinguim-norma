# Pinguim Norma Machine Simulator

# Building the Site

First, ensure WASM module is up to date. If not up to date, then:

```sh
cd norma-wasm
wasm-pack build
cd ..
```

After that, get into site source directory:
```sh
cd www
```

Then, ensure dependencies are up to date. If not up to date, then:

```sh
npm install
```

Finally, we have two commands for building:

```sh
# Starts a development server
npm run start

# Builds the site into dist/
npm run build
```
