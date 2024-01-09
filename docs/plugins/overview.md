# Plugins

Plugins are supported by RAstra in many different languages that is thanks to it's really modular building techniques, it uses many modern technologies to accomplish something like that.

> _RAstra plugins are build in a certain way, which is as follows:_

````
plugin/
    main.wasm       # The WASM file containing the code
    Config.toml     # The toml file containing most of the meta information
    assets/         # An assets folder containg extra needed content
        ...
````

!!! note annotate
    This folder/dir **MUST** be zipped and the extension renamed from ```.zip``` to ```.rpl``` (1) (my_plugin.zip --> my_plugin.rpl)

1.  Stands for **R**-_Astra_ **Pl**-_ugin_

## --> main.wasm
- wasm (also known as [WebAssembly](https://webassembly.org)) is the heart of a plugin, it contains all the code.
- WebAssembly is a portable bytecode format which allows to write programs in any language and then compile them to it.
- The best supported languages for wasm are:
    - [Rust](https://github.com/appcypher/awesome-wasm-langs#rust)
    - [Go(lang)](https://github.com/appcypher/awesome-wasm-langs#go)
    - [C++](https://github.com/appcypher/awesome-wasm-langs#c-top-2) / [C](https://github.com/appcypher/awesome-wasm-langs#c) 
- Other's which are also usable:
    - [Kotlin](https://github.com/appcypher/awesome-wasm-langs#kotlin)
    - [C#](https://github.com/appcypher/awesome-wasm-langs#csharp)
    - [TypeScript](https://github.com/appcypher/awesome-wasm-langs#typescript)
    - [Lua](https://github.com/appcypher/awesome-wasm-langs#lua)
    - [Brainfuck](https://github.com/appcypher/awesome-wasm-langs#brainfuck-top)
- The functions are all documented [here](functions.md)

## --> Config.toml
- The file looks about like this:
````toml
[plugin]
name = "Plugin Name" # The name of the plugin
author = [ # All the authors
    "I'm the author!",
    "No I am..",
    "Maybe I am?"
]
version = "1.0.1" # The intern version (not used by RAstra, it is just for you)
description = "This is a fancy plugin!"

[techincal]
arch = "WASM32" # Either WASM32 or WASM64, it is the architecture of the plugin's wasm file
api = "1.0.1" # The Api RAstra Version it uses
force-api = false # If the plugin should throw an error if it is used with the wrong api
````
- If any fields are missing it won't work, and you'll get an error.

## --> assets/
- This dir can contain assets which might be used for the plugin.
- You can get these function from the function [func be added here]



