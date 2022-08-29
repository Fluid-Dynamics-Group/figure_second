module figure_second
    include("./python_bindings.jl")

    using .python_bindings: Updater, updater, ids, update, dimensions, relative_dimensions
    export Updater, updater, ids, update, dimensions, relative_dimensions
    export plot_figures

    using Makie

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
