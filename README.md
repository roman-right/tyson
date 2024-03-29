## TySON

![Neil deGrasse Tyson](https://upload.wikimedia.org/wikipedia/commons/thumb/8/82/Neil_deGrasse_Tyson_in_June_2017_(cropped).jpg/200px-Neil_deGrasse_Tyson_in_June_2017_(cropped).jpg)

*Neil deGrasse Tyson - an astrophysicist, planetary scientist, author, and science communicator.*

**Typed Simple Object Notation (TySON)** is a text-based type-customizable data-serialization format. The key feature of this format is typing. Each value (primitive and container) contains a type, which can be mapped to a specific data structure during implementation. This expands the ways of processing and storing data.


## Grammar

A TySON text is a sequence of tokens wrapped into a journal. There are 4 types of tokens: vector, map, modifier, and primitive

### Journal

The journal is key/value pairs separated with commas or semicolons. Key is primitive, value could be any TySON token.  A single colon comes after each key. Keys can be not unique.

```
k|foo|: v|bar|;
k|one|: abs(n|3.14|);
k|two|: l[n|1|, n|2|, n|3|];
k|three|: o{n|1|:s|uno|, n|2|:s|dos|};
```

### Map

The map consists of a type and a pair of curly brackets surrounding zero or more key/value pairs. The type represented as strings, which consists of letters. Key is primitive, value could be any TySON token. A single colon comes after each key. Keys can be not unique.

```
o{n|1|:s|uno|, n|2|:s|dos|}
```

### Vector

The vector consists of a type and a pair of square brackets surrounding zero or more values. The type represented as strings, which consists of letters. Value could be any TySON token.

```
l[n|1|, n|2|, n|3|]
```

### Modifier

The modifier consist of type and a pair of round brackets surrounding a single value. The type represented as strings, which consists of letters. Value could be any TySON token. 
```
abs(n|3.14|)
```

### Primitive

Primitive consists of a type and is surrounded by vertical bars value. If the value is empty, vertical bars don't exist. If the type is empty, the value must be surrounded by bars. The type is a string of letters, value is any string.

Valid primitives
```
type|value|
type
|value|
```