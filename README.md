# Bookbag

Bookbag is a simple server designed to serve and publish websites. Anyone can send a website bundle to the `hb.sbs/<name>/publish` endpoint, which will begin serving the website at `<name>.hb.sbs`.

All websites are backed by a simple KV store (powered by `sled` and persisted in the browser's `IndexedDB`), which streams updates live through websockets to allow for the creation of real-time applications.

The most important bookbag application is the `handbook` editor, avaliable at `edit.hb.sbs`, which is an interactive online editor for developing and publishing websites. The `handbook` editor is a bundle, meaning it can be used to edit and publish improved versions of itself.

It's important to note that in the future we plan to have many editors, which provide 'runtimes' that automatically compile projects. For example, a `zola` editor for blogging or a `rust` editor for compiling applications to WebAssembly (we'd have to start some sort of Compiler-as-a-Service).
