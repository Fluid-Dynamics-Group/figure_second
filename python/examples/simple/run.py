import figure_second
from matplotlib.figure import Figure
import numpy as np
from math import pi

up = figure_second.Updater("./simple.svg", "./simple-output.svg")

print("ids available in inkscape document:\n", up.ids())

DPI = 100

def plot_xy_fig(x,y, inkscape_id: str) -> Figure:

    # fetch the dimensions of the inkscape file
    height = 4.
    dims = up.relative_dimensions(inkscape_id, height)

    fig = Figure(figsize=dims, dpi = DPI)
    ax = fig.subplots()

    ax.plot(x,y)
    ax.set_title(inkscape_id)

    return fig

x = np.linspace(0, 2*pi, 100)

y_A = np.cos(x)
y_B = np.sin(x)
y_C = np.cos(x)

mapping = {
    "A": plot_xy_fig(x, y_A, "A"),
    "B": plot_xy_fig(x, y_B, "B"),
    "C": plot_xy_fig(x, y_C, "C"),
}

figure_second.plot_figs(up, mapping, bbox_inches="tight")
