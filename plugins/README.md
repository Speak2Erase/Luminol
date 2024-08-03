This folder contains WIT files for Luminol's plugin API, as well as the current version of WASI Luminol implements.
Luminol supports plugins written in any language that supports webassembly, and the webassembly component model.

# Plugin structure
<VERY WIP!>

General filesystem structure:
```
<plugin id>/:
  assets/
    <assets go here>
  bin/
    plugin.wasm
  icon.png (optional)
  plugin.toml
```

plugin.toml:
```toml
[plugin]
# Environment plugin supports running in (default "both")
# Valid values: native, web, both
# (optional)
environment = "both"
# List of names, required
authors = [] 
# Filename of the plugin description used in Luminol's UI.
description_file = "README.md"
# Semver compatible version
version = "1.0.0"
# What setting this plugin can run in
# Valid values are: editor, project (default "project")
# If set to editor, this plugin is not copied to projects and is always running when the editor is opened.
# If set to plugin, this plugin is copied to projects and is only run when a project containing it is opened.
# (optional)
setting = "project"

# Required configuration if this plugin is a "runner", which can run other plugins
[runner]
# Format string to specify how Luminol should format the command line arguments.
# Command line arguments don't match what Luminol's arguments are (ever!) and are always per-plugin.
command_line_fmt = "${file}"

# Optional setting for a "runner", another plugin which can run this plugin.
# Plugins that require runners cannot contain a plugin.wasm.
[needs_runner]
# Required plugin ID to use when running a program
plugin_id = "ruby"
# Optional Semver compatible version
version = "3.3.0"
# Optional load path, when using something like ruby's "require"
# it'll look in this load path. 
load_path = "bin/"
binary = "plugin.rb"
```

Currently, plugins must be based on WASI preview 2 (shortened to WASIp2).
Right now, not a lot supports WASIp2, but luckily there are tools to convert between WASIp1 (earlier version of WASI) and WASIp2.

WASI is an interface that allows webassembly programs to interface with the system. 
With WASI, webassembly programs can interact with the filesystem, and even spawn other processes. 
However, that functionality must be implemented by the program that runs the webassembly program. (in this case Luminol)
WASI is designed to map closely to native APIs- but such APIs are not availible in the browser! 

Instead, Luminol polyfills WASI APIs it can't feasibly support in the browser with dud functions that either error or do nothing.
If plugins require the use of these APIs, they can mark themselves as only compatible with native builds of Luminol.