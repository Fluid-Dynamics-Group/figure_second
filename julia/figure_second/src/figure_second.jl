module figure_second
    include("./python_bindings.jl")

    using .python_bindings: Updater, updater, ids, update, dimensions, relative_dimensions
    export Updater, updater, ids, update, dimensions, relative_dimensions
end
