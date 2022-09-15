using Documenter
using Pkg
using Makie

Pkg.activate("../")
using figure_second

makedocs(
    sitename="figure_second",
    pages = [
        "Home" => ["index.md", "install.md", "example.md"],
        "api.md"
    ]
)
