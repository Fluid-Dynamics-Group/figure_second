import os
from typing import Union
from typing import Dict 
from typing import List
from typing import Optional

class VisibleMethod():
    Id = "Id"
    Name = "Name"

class Updater():
    def __new__(cls, base_file: Union[os.PathLike, str], output_file: Optional[Union[os.PathLike, str]] = None): ...

    def ids(self) -> List[str]: ...

    def layer_names(self) -> List[str]: ...

    def update(self, mapping: Dict[str, Union[os.PathLike, str]]): ...

    def dimensions(self, id: str): ...
    
    def relative_dimensions(self, id: str, height: float): ...

    def hide_layers(self, name_or_ids: list[str], method : VisibleMethod = VisibleMethod.Name): ...

    def show_layers(self, name_or_ids: list[str], method: VisibleMethod = VisibleMethod.Name): ...

    def show_all_layers(self): ...

    def hide_all_layers(self): ...

    def to_png(self, output_path: Union[os.PathLike, str], dpi : int=96): ...


class Dimensions():
    def width(self) -> float: ...

    def height(self) -> float: ...
