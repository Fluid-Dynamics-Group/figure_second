### A Pluto.jl notebook ###
# v0.19.9

using Markdown
using InteractiveUtils

# ╔═╡ 205ae4d7-4be3-4605-99c3-3cf06ae8eb1b
using Pkg

# ╔═╡ e610e7f3-612c-43cb-b417-398cd310d119
Pkg.activate("../../../figure_second")

# ╔═╡ b8b3d7de-273f-11ed-3c47-cd1ab0eba7d9
using Revise

# ╔═╡ f1dc6bad-45ac-4592-bc3e-b681a858b97e
using figure_second

# ╔═╡ 9054e3ea-bc87-4bc1-930a-4dd986e61bdc
using Makie

# ╔═╡ 40df593b-9dd9-4a7e-ab8b-bc34bb687aab
using CairoMakie

# ╔═╡ f22833f3-9040-40cf-bfc9-9dbb8ed7cca6
up = updater("./simple.svg", "./simple-output.svg")

# ╔═╡ 98570949-8d81-484f-a983-b9f5f09bb4be
ids(up)

# ╔═╡ 3fcf6251-be75-408e-a23a-31e56e69addf
x = 0:.1:2*pi

# ╔═╡ a3d6946c-c6e6-4de8-ba5b-c74337ddbef3
function make_figure(id::String, y_function::Function; extra...)::Makie.Figure
	y = y_function.(x)
	
	res = relative_dimensions(up, id, 600)
	
	fig = Figure(resolution = res)
	
	ax = Axis(
		fig[1,1], 
		xlabel = "x", 
		ylabel = "y", 
		title = id, 
		backgroundcolor=:white
	)
	lines!(ax,x, y)

	return fig
end

# ╔═╡ 9828aae3-e72e-4509-8598-4ccc70ef4e71
function make_figure(id::String, data::Matrix{Float64})::Makie.Figure
	res = relative_dimensions(up, id, 600)
	fig = Figure(resolution = res)
	
	ax = Axis(
		fig[1,1], 
		xlabel = "x", 
		ylabel = "y"
	)

	hidedecorations!(ax)

	lx, ly = size(data)
	
	heatmap!(ax,1:lx, 1:ly, data)

	return fig
end

# ╔═╡ 865e547b-e6ae-4c70-9f83-d38885f71f92
figA = make_figure("A", x -> sin.(x))

# ╔═╡ 378cd22c-2021-44c9-84ed-24e93880044c
figB = make_figure("B", x -> sin.(x) + 0.5)

# ╔═╡ b6b2a33e-df7d-40df-a4db-64246046fcb1
figC = make_figure("C", x -> cos.(x+pi/3))

# ╔═╡ 64038360-8f38-45d7-9b05-8732f1c49613
relative_dimensions(up, "A", 600)

# ╔═╡ ea72f619-d1b3-42d5-b401-3c73d310b641
figSlice = make_figure("slice", randn(20,20))

# ╔═╡ f9cfaaaa-a508-4d0e-96f4-ac53ea5af7e1
figure_map = Dict(
	"A"=> figA,
	"B"=> figB,
	"C"=> figC,
	"slice" =>figSlice
)

# ╔═╡ eee082aa-6001-49bf-939e-613a497309a5
plot_figures(up, figure_map)

# ╔═╡ Cell order:
# ╠═b8b3d7de-273f-11ed-3c47-cd1ab0eba7d9
# ╠═205ae4d7-4be3-4605-99c3-3cf06ae8eb1b
# ╠═e610e7f3-612c-43cb-b417-398cd310d119
# ╠═f1dc6bad-45ac-4592-bc3e-b681a858b97e
# ╠═9054e3ea-bc87-4bc1-930a-4dd986e61bdc
# ╠═40df593b-9dd9-4a7e-ab8b-bc34bb687aab
# ╠═f22833f3-9040-40cf-bfc9-9dbb8ed7cca6
# ╠═98570949-8d81-484f-a983-b9f5f09bb4be
# ╠═3fcf6251-be75-408e-a23a-31e56e69addf
# ╠═a3d6946c-c6e6-4de8-ba5b-c74337ddbef3
# ╠═9828aae3-e72e-4509-8598-4ccc70ef4e71
# ╠═865e547b-e6ae-4c70-9f83-d38885f71f92
# ╠═378cd22c-2021-44c9-84ed-24e93880044c
# ╠═b6b2a33e-df7d-40df-a4db-64246046fcb1
# ╠═64038360-8f38-45d7-9b05-8732f1c49613
# ╠═ea72f619-d1b3-42d5-b401-3c73d310b641
# ╠═f9cfaaaa-a508-4d0e-96f4-ac53ea5af7e1
# ╠═eee082aa-6001-49bf-939e-613a497309a5
