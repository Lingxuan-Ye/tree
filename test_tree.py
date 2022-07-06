import unittest

from tree import Tree, tree


class _Node:

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

    def __eq__(self, _node):
        return True if self.i == _node.i else False

    def __repr__(self):
        return f"_Node({self.i})"


def _iterfunc(_node: _Node):
    if not isinstance(_node, _Node):
        return None
    return [0, 1, 2]


class TestTree(unittest.TestCase):

    tree = Tree(_Node(2))
    tree_ = Tree(_Node(2), _iterfunc)
    tree_.indent = 1
    tree__ = Tree((("0", "1", "2", ("3", "4")), ("5", "6"), "7",
                   ("8", ("9", "a", ("b", "c", ("d", ))))))

    def test_generate(self):
        self.assertEqual(
            self.tree.generate(),
            "_Node(2)\n"
            "├── _Node(1)\n"
            "│   └── _Node(0)\n"
            "└── _Node(0)"
        )
        self.assertEqual(
            self.tree_.generate(),
            "_Node(2)\n"
            "├0\n"
            "├1\n"
            "└2"
        )
        print(self.tree__.generate())

    def test_tree(self):
        self.assertEqual(
            tree(_Node(2)),
            "_Node(2)\n"
            "├── _Node(1)\n"
            "│   └── _Node(0)\n"
            "└── _Node(0)"
        )
        with self.assertRaises(ValueError):
            tree(_Node(2), indent= -1)
