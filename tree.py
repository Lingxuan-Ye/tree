from typing import (Any, Callable, Dict, Iterable, Iterator, List, Optional,
                    Tuple)

Inst = Any
IterFunc = Callable[[Inst], Optional[Iterable[Inst]]]
Locator = Tuple[int, ...]
Node = Tuple[Locator, Inst]
Structure = Dict[Locator, Inst]
Stems = Dict[int, str]


def _iter(inst: Inst) -> Optional[Iterator]:
    if isinstance(inst, (str, bytes, bytearray)):
        return None
    try:
        return iter(inst)
    except TypeError:
        return None


class Tree:

    indent: int = 4

    def __init__(self, inst: Inst, iterfunc: IterFunc = None) -> None:
        self.inst: Inst = inst
        self.iterfunc: IterFunc = iterfunc if iterfunc is not None else _iter
        self.__temp: List[Node] = []

    def __sub_nodes(self, node: Node) -> None:
        self.__temp.clear()
        locator = node[0]
        inst = node[1]
        temp = self.iterfunc(inst)
        if temp is None:
            return
        for i, _inst in enumerate(temp):
            _locator = (*locator, i)
            self.__temp.append((_locator, _inst))
        # self.__temp.reverse()  # unnecessary.

    def __get_structure(self) -> Structure:
        stack: List[Node] = [((), self.inst)]
        structure: Dict[Locator, Inst] = {}
        while stack:
            node = stack.pop()
            self.__sub_nodes(node)
            stack.extend(self.__temp)
            structure[node[0]] = node[1]
        self.__temp.clear()
        return structure

    @property
    def stems(self) -> Stems:
        indent = self.indent
        if not isinstance(indent, int) or indent < 1:
            message = "attribute 'indent' must be int and greater than 0"
            raise ValueError(message)
        _stems = {
            0: "│" + " " * (indent - 1),
            1: " " + " " * (indent - 1),
            2: "├" + "─" * (indent - 1),
            3: "└" + "─" * (indent - 1)
        }
        if indent > 2:
            _stems[2] = _stems[2][0:-1] + " "
            _stems[3] = _stems[3][0:-1] + " "
        return _stems

    def generate(self) -> str:
        structure: Structure = self.__get_structure()
        lines_with_locator: List[Tuple[Locator, str]] = []
        for locator in structure:
            locator_len = len(locator)
            prefix_list: List[str] = []
            for i, j in enumerate(locator):
                dummy = (*locator[0:i], j + 1)

                ### for Python 3.10 and later ###

                # match (
                #     i < loc_len - 1,
                #     structure.get(dummy, None) is not None
                # ):
                #     case (True, True):
                #         prefix_list.append(self.stems[0])
                #     case (True, False):
                #         prefix_list.append(self.stems[1])
                #     case (False, True):
                #         prefix_list.append(self.stems[2])
                #     case (False, False):
                #         prefix_list.append(self.stems[3])

                if i < locator_len - 1:
                    if structure.get(dummy, None) is not None:
                        prefix_list.append(self.stems[0])
                    else:
                        prefix_list.append(self.stems[1])
                else:
                    if structure.get(dummy, None) is not None:
                        prefix_list.append(self.stems[2])
                    else:
                        prefix_list.append(self.stems[3])
            line = "".join(prefix_list) + repr(structure[locator])
            lines_with_locator.append((locator, line))
        lines_with_locator.sort(key=lambda i: i[0])
        return "\n".join([i[1] for i in lines_with_locator])


def tree(inst: Inst, iterfunc: IterFunc = None, indent: int = 4) -> str:
    _tree = Tree(inst, iterfunc)
    _tree.indent = indent
    return _tree.generate()
