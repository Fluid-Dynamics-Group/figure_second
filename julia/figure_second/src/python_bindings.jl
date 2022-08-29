module python_bindings
    using PyCall

    export Updater
    export updater, ids, update, dimensions, relative_dimensions

    struct Updater
        updater::PyCall.PyObject
    end

    struct Dimensions
        width::Float64
        height::Float64
    end
    
    function updater(base_path::String, output_file::String)::Updater
        figure_second = pyimport("figure_second")
        class_initialization = figure_second.Updater(base_path, output_file)

        return Updater(class_initialization)
    end

    function ids(updater::Updater)::Vector{String}
        return updater.updater.ids()
    end

    function update(updater::Updater, map::Dict{String, String})
        updater.updater.update(map)
    end

    function dimensions(updater::Updater, id::String)::Dimensions
        dims = updater.updater.dimensions(id)

        Dimensions(dims.width(), dims.height())
    end

    function relative_dimensions(updater::Updater, id::String, height::Float64)::Tuple{Float64, Float64}
        (width, height) = updater.updater.relative_dimensions(id, height)

        return (width, height)
    end
end
