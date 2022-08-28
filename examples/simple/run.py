import figure_second

print(dir(figure_second))

print(dir(figure_second.figure_second))

print(figure_second.Updater)
print(figure_second.Updater("./static/simple-inkscape-drawing.svg", "./static/simple-inkscape-drawing-output.svg"))

figure_second.other_fn()
