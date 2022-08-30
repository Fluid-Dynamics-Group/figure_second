module python_bindings
    using PyCall

    export Updater
    export updater, ids, update, dimensions, relative_dimensions

    """
        Updater

    representation of an inkscape document. An `Updater` can either
    mutate an the underlying inkscape file.
    or export to a new file See documentation on the constructor ([`updater`](@ref))
    """
    struct Updater
        updater::PyCall.PyObject
    end

    """
        Dimensions(width::Float64, height::Float64)

    raw dimensions of an inkscape object. 
    """
    struct Dimensions
        width::Float64
        height::Float64
    end
    
    """
        updater(base_path::String, output_file::String)::Updater

    `Updater` constructor for a workflow that reads in a file `base_path` and 
    exports all changes under a new file `output_file`. 

    This `Updater` instance
    will *not* mutate the underlying `base_path` inkscape file
    """
    function updater(base_path::String, output_file::String)::Updater
        figure_second = pyimport("figure_second")
        class_initialization = figure_second.Updater(base_path, output_file)

        return Updater(class_initialization)
    end

    """
        updater(base_path::String)::Updater

    `Updater` constructor for a workflow that reads in a file `base_path` and 
    mutates it in place with all plotting changes.
    """
    function updater(base_path::String)::Updater
        figure_second = pyimport("figure_second")
        class_initialization = figure_second.Updater(base_path)

        return Updater(class_initialization)
    end

    """
        ids(updater::Updater)::Vector{String}

    fetch all valid `id`s from the `base_path` document of an `Updater`
    """
    function ids(updater::Updater)::Vector{String}
        return updater.updater.ids()
    end

    """
        update(updater::Updater, map::Dict{String, String})

    low level function to update the contents of an inkscape file. The `map` argument
    maps an inkscape object's `id` to a file path containing a `.png` encoded image.
    The `.png` encoded image (dictionary value) will be placed in the inkscape object
    with the id of the key.
    """
    function update(updater::Updater, map::Dict{String, String})
        updater.updater.update(map)
    end

    """
        dimensions(updater::Updater, id::String)::Dimensions


    parse the raw dimensions of an inkscape object with id `id` and return them 
    as a [`Dimensions`](@ref) object
    """
    function dimensions(updater::Updater, id::String)::Dimensions
        dims = updater.updater.dimensions(id)

        Dimensions(dims.width(), dims.height())
    end

    """
        relative_dimensions(updater::Updater, id::String, height::Float64)::Tuple{Float64, Float64}

    Calculate the required dimension pair `(width, height)` to maintain the aspect ratio of 
    the inkscape object. Returns result in fractional units.
    """
    function relative_dimensions(updater::Updater, id::String, height::Float64)::Tuple{Float64, Float64}
        (width, height) = updater.updater.relative_dimensions(id, height)

        return (width, height)
    end

    """
        relative_dimensions(updater::Updater, id::String, height::Int)::Tuple{Int, Int}

    Calculate the required dimension pair `(width, height)` to maintain the aspect ratio of 
    the inkscape object. Rounds the otherwise float point value and converts to `Int`. 

    This is useful for plotting functions that expect an interger number for pixel dimensions
    """
    function relative_dimensions(updater::Updater, id::String, height::Int)::Tuple{Int, Int}
        (width, height) = updater.updater.relative_dimensions(id, Float64(height))

        return (Int(round(width)), Int(round(height)))
    end
end
