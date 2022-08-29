from matplotlib.figure import Figure
from .figure_second import Updater
from typing import Dict, Tuple
import tempfile

def plot_figs(updater: Updater, fig_map: Dict[str, Figure], *args, **kwargs):
    plotting_map = {}
    tempfile_history =  []

    for inkscape_id in fig_map.keys():
        fig = fig_map[inkscape_id]

        # create a tempfile 
        file = tempfile.NamedTemporaryFile(suffix=".png")
        filename = file.name

        # ensure we store this in a list of file handles 
        # so that it does not become garbage collected
        # (and therefore delted before we read it from the 
        # rust side
        tempfile_history.append(file)

        fig.savefig(filename, *args, **kwargs)

        plotting_map[inkscape_id] = filename

    # now we have exported all the figures, use the updater to produce a change
    # in the SVG
    updater.update(plotting_map)
