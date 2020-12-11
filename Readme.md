# cli-spreadsheet
## General Concept
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