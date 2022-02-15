# Crisp documentation

## Introduction

`crisp` is a programming language based on Lisp, but takes more inspiration
from Rust (keywords for instance).

## Table of contents

- [Basics](#basics)
- [Variables](#variables)
- [Functions](#functions)

### Basics

```lisp
>> "Hello, world!"
"Hello, world!"
>> (+ 1 2 3 4 5)
15
>> (* 4 5 6)
120
>> (- 5 15)
-10
>> (* 13 (+ 1 2 3) 5)
390
```

### Variables

```lisp
>> (let x 10)
>> (let y 15)
>> (+ x y)
25
```

### Functions

```lisp
>> (let triple (fn (x) (* x 3)))
>> (triple 5)
15
```

Above can also be written like so:

```lisp
>> (let (triple x) (* x 3))
>> (triple 5)
15
```

Functions can also be recursive:

```lisp
>> (let (sum x) (if (> x 0) (+ x (sum (- x 1))) x))
>> (sum 10)
55
```
