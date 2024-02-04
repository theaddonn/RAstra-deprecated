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

### main.wasm
- wasm (also known as [WebAssembly](https://webassembly.org)) is the heart of a plugin, it contains all the code.
- WebAssembly is a portable bytecode format which allows to write programs in any language and then compile them to it.
- The best supported languages for wasm are:
    - [Rust](https://github.com/appcypher/awesome-wasm-langs#rust)
    - [Go(lang)](https://github.com/appcypher/awesome-wasm-langs#go)
    - [C++](https://github.com/appcypher/awesome-wasm-langs#c-top-2) / [C](https://github.com/appcypher/awesome-wasm-langs#c) 
    - [Kotlin](https://github.com/appcypher/awesome-wasm-langs#kotlin)
    - [C#](https://github.com/appcypher/awesome-wasm-langs#csharp)
    - [AssemblyScript](https://github.com/appcypher/awesome-wasm-langs#assemblyscript)
    - [Lua](https://github.com/appcypher/awesome-wasm-langs#lua)
- The functions are all documented [here](functions.md)

### Config.toml
- The configuration file for the plugin
- The file looks about like this:
````toml
# The name of the plugin
name = "Plugin Name" 

# All the authors
author = [
    "PotatoMan232",
    "Adrian8115"
]

# The Plugin version
version = "1.0.1"

# The plugin description
description = "This is a fancy plugin!"

# Either WASM32 or WASM64,
# it is the architecture of the plugin's wasm file
arch = "WASM32"

# The Api RAstra Version it uses
api = "1.0.1"

# Should the plugin should throw an error if
# it is used with the wrong api version
# Can be overwritten by the server's rastra.toml
force-api = false
````
- If any fields are missing it won't work, and you'll get an error.

### assets/...
- This dir can contain assets which might be used for the plugin.
- You can get these function from the function `[func be added here]`



