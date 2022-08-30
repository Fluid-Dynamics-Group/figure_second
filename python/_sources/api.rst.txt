API Reference
=============

.. class:: Updater(base_file: Union[str, os.PathLike], output_file: Optional[Union[str, os.PathLike]] = None)

   :param base_file: a path to the template file that contains rectangles / images with their respective XML ids set
   :param output_file: the file that will be generated from ``base_file`` with figures substituted in. If this is none, ``base_file`` will be mutated in palce.
   :return: ``Updater`` class

    .. function:: Updater.ids(self) -> List[str]

        :return: A list of available ids belonging to either `rect` or `image` objects in the inkscape ``base_file`` ``.svg``

    .. function:: Updater.update(self, map: Dict[str, Union[str, os.PathList]])

        update the svg in ``base_path`` to the png images in the keys of ``map``. All input images must be 
        png encoded.
        Note that most of the time, you want :func:`plot_figs()` to handle this in a simple manner.
        The resulting svg file will be stored at ``output_file``. 

        This function may be called more than once for the same ``Updater`` instance.

        :param map: A dictionary of valid XML ids keys and ``.png`` image path values. 
        :return: None

        The id of the xml object must be valid. Ensure that it is present in the output of ``Updater.ids()``.


    .. function:: Updater.dimensions(id: str) -> :class:`Dimensions`

        fetch the dimensions of the object in inkscape. Note that the output results
        are in inkscape units, but ``matplotlib`` figure dimensions are in inches.
        
        :param id: the XML object id whose dimensions we are parsing
        :return: :class:`Dimensions` object with width and height information

    .. function:: Updater.relative_dimensions(id: str, height: float) -> Tuple[float, float]

        A helper method to calculate what width a matplotlib figure should be for a given height,
        given that you want to preserve the aspect ratio with the geometry from inkscape.

        This method simply solves a proportion:

        .. code-block::

            matplotlib width     matplotlib height
            ----------------  =  -----------------
            inkscape width       inkscape height

        :param height: the height of the figure (inches)
        :return: a tuple of ``(width, height)``, ready to be passed into a matplotlib ``Figure(figsize = _)`` constructor

        

.. class:: Dimensions

    .. function:: Dimensions.width(self) -> float
        
        :return: the width of the object (inkscape units)

    .. function:: Dimensions.height(self) -> float
        
        :return: the height of the object (inkscape units)
    
.. function:: plot_figs(updater: Updater, fig_map: Dict[str, matplotlib.figure.Figure], *args, **kwargs)

    Helper function to substitute all figures in the values of ``fig_map`` into the corresponding inkscape ids
    in ``updater.base_path``.

    :param updater: an :class:`Updater()` class
    :param fig_map: a dictionary with values matplotlib ``Figure`` objects, and keys of inkscape ids
    :param \*args: passed into matplotlib ``Figure.savefig``
    :param \*\*kwargs: passed into matplotlib ``Figure.savefig``
