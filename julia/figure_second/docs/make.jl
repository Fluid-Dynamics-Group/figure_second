using Documenter
using Pkg
using Makie

Pkg.activate("../")
using figure_second

makedocs(
    sitename="figure_second",
    html_prettyurls = true,
    pages = [
        "Home" => "index.md",
        "api.md"
    ]
)
