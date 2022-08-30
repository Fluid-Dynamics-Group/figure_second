# Welcome

`figure_second` is a layout first approach to plotting based on the ideas of the python
library [figurefirst](https://flyranch.github.io/figurefirst/). The general workflow of the library
is to define a layout of graphs in inkscape, label each object with an XML `id`, and then plot 
*into* these objects using common julia plotting libraries (`Makie.jl`, `Plots.jl`, etc).

```@contents
Depth = 2
```

## Example

TODO

## API Reference

For figure layout, the most useful functions will be [`updater`](@ref) to construct a common object,
[`plot_figures`](@ref) to place figures in the document, [`relative_dimensions`](@ref) for sizing 
plots correctly. 

The [`ids`](@ref) function can sometimes be useful for debugging purposes.

```@autodocs
Modules = [figure_second, figure_second.python_bindings]
```
