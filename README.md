# tree

Draw the structure of an instance like 'tree' command in shell.

## Notice

- Should implement a callable object and pass it to parameter `iterfunc` in order to indicate how to iter the instance given. 
- Be aware that `iterfunc` should **always** return a **finite** iterable and be able to **recursively** take every elements of the iterable as its argument, unless the elements is `None` or an empty iterable.

    For example:

    ```python
    foo = (0, 1, 2)
    
    def bar(foo):
        if isinstance(foo, int):  # Please make sure that `bar` is valid when taking an `int`.
            return None
        return foo
    ```
- If parameter `iterfunc` is `None` , `Tree.generate` will try to call the given instance's `__iter__` method instead. Different from callable object `iterfunc` , `__iter__` method is only required to return a finite iterable.
- If `__iter__` is not implemented as well, `Tree.generate` returns the same as built-in function `str` .
- `tree` is the factory function, of which the argument `indent` determines output style.

## Example

```python
from tree import Tree, tree


class Node:

    def __init__(self, i: int):
        self.i = i
        self.j = i

    def __iter__(self):
        return self

    def __next__(self):
        if self.j > 0:
            self.j -= 1
            return self.__class__(self.j)
        else:
            raise StopIteration

    def __repr__(self):
        return f"Node({self.i})"


print(Tree(Node(4)).generate())
# Result:
# Node(4)
# ├── Node(3)
# │   ├── Node(2)
# │   │   ├── Node(1)
# │   │   │   └── Node(0)
# │   │   └── Node(0)
# │   ├── Node(1)
# │   │   └── Node(0)
# │   └── Node(0)
# ├── Node(2)
# │   ├── Node(1)
# │   │   └── Node(0)
# │   └── Node(0)
# ├── Node(1)
# │   └── Node(0)
# └── Node(0)

print(tree(Node(4), indent=2))
# Node(4)
# ├─Node(3)
# │ ├─Node(2)
# │ │ ├─Node(1)
# │ │ │ └─Node(0)
# │ │ └─Node(0)
# │ ├─Node(1)
# │ │ └─Node(0)
# │ └─Node(0)
# ├─Node(2)
# │ ├─Node(1)
# │ │ └─Node(0)
# │ └─Node(0)
# ├─Node(1)
# │ └─Node(0)
# └─Node(0)

print(tree(Node(4), indent=8))
# Result:
# Node(4)
# ├────── Node(3)
# │       ├────── Node(2)
# │       │       ├────── Node(1)
# │       │       │       └────── Node(0)
# │       │       └────── Node(0)
# │       ├────── Node(1)
# │       │       └────── Node(0)
# │       └────── Node(0)
# ├────── Node(2)
# │       ├────── Node(1)
# │       │       └────── Node(0)
# │       └────── Node(0)
# ├────── Node(1)
# │       └────── Node(0)
# └────── Node(0)
```
