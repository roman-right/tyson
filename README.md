## TySON

![Neil deGrasse Tyson](https://pbs.twimg.com/profile_images/74188698/NeilTysonOriginsA-Crop_400x400.jpg)

Typed Simple Object Notation (TySON) is a text-based type-customizable data-serialization format. The key feature of this format is typing. Each value (primitive and container) contains a type, which can be mapped to a specific data structure during implementation. This expands the ways of processing and storing data.

## Grammar

A TySON text is a sequence of tokens wrapped into a document. There are 3 types of tokens: vector, map, and primitive

### Document

The document is key/value pairs separated with commas or semicolons. Key is primitive, value could be any TySON token.  A single colon comes after each key. Keys can be not unique.

```
k|foo|: v|bar|;
k|one|: l[n|1|, n|2|, n|3|];
k|two|: o{n|1|:s|uno|, n|2|:s|dos|};
```

### Map

The map consists of a type and a pair of curly brackets surrounding zero or more key/value pairs. Type represented as strings, which consists of letters. Key is primitive, value could be any TySON token. A single colon comes after each key. Keys can be not unique.

```
k|two|: o{n|1|:s|uno|, n|2|:s|dos|};
```

### Vector

The vector consists of a type and a pair of square brackets surrounding zero or more values. Type represented as strings, which consists of letters. Value could be any TySON token.

```
k|one|: l[n|1|, n|2|, n|3|];
```

### Primitive

Primitive consists of a type and is surrounded by vertical bars value. If the value is empty, vertical bars don't exist. If the type is empty, the value must be surrounded by bars. Type is a string of letters, value is any string.

Valid primitives
```
type|value|
type
|value|
```
