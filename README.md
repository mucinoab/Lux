# Lux
A toy interpreter of `Lux` (based on [Lox](https://craftinginterpreters.com/the-lox-language.html/)), a fully featured dynamically typed language. 

You can try the REPL [here.](https://mucinoab.github.io/lux/lux_demo.html)

# Features

### Control Flow
```c#
if (true and false or true and false)
  print "branch";
else if (false)
  print "other branch";
else 
  print "final branch";
```

### Loops
```c#
// Iterative fibonacci
var a = 0;
var temp;

for (var b = 1; a < 100; b = temp + b) {
  print a;
  temp = a;
  a = b;
}

var iter = 0;
while (iter < 10) {
  print iter;
  iter = iter + 1;
}

```

### Functions
```c#
// User defined functions
fn fibonacci(n) {
  if (n <= 1) { return n; }
  return fibonacci(n - 2) + fibonacci(n - 1);
}

// Nested functions
fn makeCounter() {
  var i = 0;
  fn count() {
    i = i + 1;
    print i;
  }

  return count;
}

var counter = makeCounter();
counter(); // 1
counter(); // 2
counter(); // 3

// Native functions (baked into the language)
clock(); // milliseconds since the unix epoch
```

### Variable Scopes
```c#
var a = "global a";
var b = "global b";
{
  var a = "outer a";
  var b = "outer b";
  {
    var a = "inner a";
    print a;
    print b;
  }
  print a;
  print b;
}
print a;
print b;
```

### REPL mode
### Nice Error Messages
