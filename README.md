# jack

A JSON configuration language like [jsonnet](https://jsonnet.org/).


## Example

```
expr> // JSON object.
....| {
....|   "name": "Alice",
....|   "age": 20,
....|   "friends": ["Bob", "Charlie"],
....| }
=> {
  "friends": [
    "Bob",
    "Charlie"
  ],
  "age": 20.0,
  "name": "Alice"
}

expr> // A function that returns an object.
....| local Person(name) = {
....|   name: name,
....|   welcome: "Hello " + name + "!",
....| };
....| {
....|   person1: Person("Alice"),
....|   person2: Person("Bob"),
....| }
=> {
  "person1": {
    "welcome": "Hello Alice!",
    "name": "Alice"
  },
  "person2": {
    "name": "Bob",
    "welcome": "Hello Bob!"
  }
}

expr> // Recursive function.
....| local fact(n) =
....|   if n == 0 then
....|     1
....|   else
....|     n * fact(n-1);
....| fact(5)
=> 120.0

expr> // Lazy evaluation.
....| local cons(x, xs) = [x, xs];
....| local head(ls) = ls[0];
....| local tail(ls) = ls[1];
....| local take(n, ls) =
....|   if n == 0 then
....|     null
....|   else
....|     cons(head(ls), take(n-1, tail(ls)));
....| local mugen = cons("∞", mugen); // Infinite list!!
....| take(3, mugen)
=> [
  "∞",
  [
    "∞",
    [
      "∞",
      null
    ]
  ]
]
```
