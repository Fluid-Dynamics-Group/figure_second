module figure_second
    include("./python_bindings.jl")

    using .python_bindings: Updater, updater, ids, update, dimensions, relative_dimensions
    export Updater, updater, ids, update, dimensions, relative_dimensions
    export plot_figures

    using Makie

    """
        plot_figures(updater::Updater, figure_map::Dict{String, Makie.Figure})

    **updater** is a wrapper around the python class. It represents the inkscape svg document
    we are plotting on.

    **figure_map** is a dictionary with keys containing `id`s of inkscape objects 
    (rectangles or images) to plot on top of. The values of `figure_map` are 
    figures to render. 

    The user is responsible for filling each figure with content, and sizing it appropriately 
    to the dimensions of the inkscape object. See [`relative_dimensions`](@ref) for sizing
    considerations.
    """
    function plot_figures(updater::Updater, figure_map::Dict{String, Makie.Figure})

        path_map::Dict{String, String} = Dict()

        for (inkscape_id, figure) in figure_map
            file_path = Base.Filesystem.tempname(cleanup=true) * ".png"

            Makie.save(file_path, figure)

            path_map[inkscape_id] = file_path
        end


        update(updater, path_map)
    end
end
