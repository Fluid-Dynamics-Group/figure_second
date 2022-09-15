# Installing

You can install the package from git (for now):

```
using Pkg
Pkg.add(url="https://github.com/Fluid-Dynamics-Group/figure_second", subdir="julia/figure_second")
```

Since the julia library currently wraps the python library, you must also have the python package:

```
pip install figure-second
```

and update the package as you would any other:

```
using Pkg
Pkg.update("figure_second")
```

and also update the python package:

```
pip install -U figure-second
```

Note that the `pip` package builds from a high performance rust library. Therefore, you will need
a recent rust compiler which can be installed [here](https://www.rust-lang.org/tools/install).
