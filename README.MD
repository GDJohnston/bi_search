# AVL Tree Project

```
     A
   /   \
  V     L
 / \   / \
T   R E   E
```

This project is an implementation of an [AVL Tree](https://en.wikipedia.org/wiki/AVL_tree), a self-balancing binary tree.
The purpose of this project was to further explore the `box` container type, introduced by [The Rust book](https://doc.rust-lang.org/book/ch15-01-box.html#using-boxt-to-point-to-data-on-the-heap).
The `box` type is the simplest form of smart pointer and is used here to allow type recursion which not possible without some indirection.
There is an implementation of binary search as well, I made this for comparison.

I started this project in December 2023 and I had left it in an unfinished and broken state as I had gotten what I wanted out of it.
When I was thinking of projects to add to my GitHub showcase this came to mind.

Coming back to it a year later with more experience, I was quickly able to get the code working and I found plenty of things I wanted to improve upon.

- All the code was in one file, as I was not confident with modules yet.
- I didn't like how I needed to use data to construct the tree, as opposed to having an empty tree.
- There was no way of displaying the tree, which I found dissatisfying.
- The main function has tests/example data in it.
- There are no function comments.
- There is no search function for the binary tree.
- The project might be more useful if it was a libary with an excutable as an example
- There is no way of interacting with the compiled code, i.e. no processing of command line arguments
- The rotate functions drop may be dropping heap memory (box) only to allocate more for the same variable.

Some of these I have implemented already, the rest I plan to get to _someday_.

## Reference material

This AVL tree is an implementation based on code from the [geeksforgeeks](https://www.geeksforgeeks.org/java/avl-tree-program-in-java/) webpage exploring an implementation of an AVL tree in Java. It was copied into the project to allow for easy referencing however it is not included in the project as I don't know if I have permission to reproduce.
It was an fun and interesting challenge trying to "rustify" the Java implementation, and I think I did well for the most part.
