# Lux
A toy interpreter of `Lux` (based on [Lox](https://craftinginterpreters.com/the-lox-language.html/)), a fully featured dynamically typed language. 

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

### Functions
```c#
// User defined
fn count(n) {
  if (n > 1) {
    count(n - 1);
  }

  print n;
}

count(100);

// Baked into the language
clock(); // milliseconds since the unix epoch
```

### REPL mode
### Nice Error Messages
