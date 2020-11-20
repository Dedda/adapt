# Adapt

Very simplistic esoteric programming language that runs on its source file. 
While the code is executed, it may rewrite itself. Currently, there are only a few allowed operations.

| | |
|---|---|
| | Blank lines are ignored |
| `_123` | Integer data is displayed as passive code preceded by `_`. There are no negative integers. |
| `a` | Single characters are also passive code and represent itself as char data |
| `flip type 3` | Tries to read an integer from address `3` (4th line in the code) and flip its data type. Int to Char and Char to Int. |
| `jump 4` | Jumps to the 5th line in the code |
| `jump addr 2` | Tries to read an integer from address `2` and uses this integer as the address for a normal `jump` instruction |
| `jump addr cmp 1 2 3 4 5` | This one is a bit complicated. All arguments as treated as addresses for addresses. The value resolved from the first and second address addresses are compared. If the first is lower, it jumps to the address behind the third argument. If it is equal, it jumps to the address behind the fourth argument. And if it is greater, it jumps to the address behind the fifth argument. |
| `swap 3 4` | Tries to read integers from addresses `3` and `4`. These two integers are used as addresses to swap the two corresponding instructions in code. |
| `copy 1 2` | Tries to read integers from addresses `1` and `2`. The instruction at the first address is then copied to the second one. |
| `add 3 5` | Tries to read integers from addresses `3` and `5`. Both corresponding addresses have to be integers. The first is added to the second |
| `sub 3 5` | Equivalent to `add` |
| `mul 3 5` | Equivalent to `add` |
| `div 3 5` | Equivalent to `add` |
| `del 4` | Deletes the instruction at the address that is found at address `4` |
| `print 5` | Tries to read an integer from address `5` and tries to print the underlying data. Has to be char or int. |
| `Exit` | Exits with code 0 |
| `Exit 1` | Exits with code 1 |

## Example code:

```
jump 4
_0
_1
_2
add 3 2
print 2
```

This program will output a sequence of numbers but only one per execution. 
Every time it is run it prints the next higher number and stores it in its source code.
After 3 executions the source file will look like this:

```
jump 4
_3
_1
_2
add 3 2
print 2
```

Be careful with newlines as they alter the source file when they are converted from int to char but they don't update
the runtime. you basically create a line you cannot address in between the existing instructions. When you execute the
file again, those new empty lines are parsed as NOP operations though so you should cleanup any possibly created 
newlines while the program is still running. Take a look at the `test.ada` file where a newline is created and also 
deleted again in the lines 15-24.