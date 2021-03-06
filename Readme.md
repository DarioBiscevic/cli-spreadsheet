# cli-spreadsheet - WORK IN PROGRESS
## General Concept
This project aims to create a spreadsheet program completely based on the command line.
As in every such office project, each cell holds an expression, which is then translated
to a value; the way the expression is written, in this project, is a little bit different
compared to the famous Excel, since it uses RPN: insted of typing "=42 + 7", as you would
in any other spreadsheet system, in cli-spreadsheet you would write it as "=42 7 +", as it
is easier to compute and, in a programming way, more logical. 

## Visual Idea
```

> draw A..C 1..3

   |        A       |        B       |        C       |
---+----------------+----------------+----------------+
 1 |                |                |                |
---+----------------+----------------+----------------+
 2 |                |                |                |
---+----------------+----------------+----------------+
 3 |                |                |                |
---+----------------+----------------+----------------+

> set A2 9
A2: 9

> draw A..C 1..3

   |        A       |        B       |        C       |
---+----------------+----------------+----------------+
 1 |                |                |                |
---+----------------+----------------+----------------+
 2 |              9 |                |                |
---+----------------+----------------+----------------+
 3 |                |                |                |
---+----------------+----------------+----------------+

> set B1 "=A2+3"
B1: "=A2+3"

> draw A..C 1..3

   |        A       |        B       |        C       |
---+----------------+----------------+----------------+
 1 |                |             12 |                |
---+----------------+----------------+----------------+
 2 |              9 |                |                |
---+----------------+----------------+----------------+
 3 |                |                |                |
---+----------------+----------------+----------------+

> set A2 5
A2: 5

> draw A..C 1..3

   |        A       |        B       |        C       |
---+----------------+----------------+----------------+
 1 |                |              8 |                |
---+----------------+----------------+----------------+
 2 |              5 |                |                |
---+----------------+----------------+----------------+
 3 |                |                |                |
---+----------------+----------------+----------------+

> view A2
A2: 5

> view B1
B1: "=A2+3" --> 8

> set B3 "=B1+A2"
B3: "=B1+A2"

> view B3
B3: "=B1+A2" --> 13

> draw A..C 1..3

   |        A       |        B       |        C       |
---+----------------+----------------+----------------+
 1 |                |              8 |                |
---+----------------+----------------+----------------+
 2 |              5 |                |                |
---+----------------+----------------+----------------+
 3 |                |             13 |                |
---+----------------+----------------+----------------+

> 

```
