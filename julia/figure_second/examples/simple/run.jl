import Pkg;

Pkg.activate("../../")

using figure_second

up = updater("./simple.svg", "./simple-output.svg")

println(ids(up))
println(dimensions(up, "A"))
println(relative_dimensions(up, "A", 4.))
