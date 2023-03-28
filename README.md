# JSON Merge Patch

Rust implementation of [RFC 7396](https://www.rfc-editor.org/rfc/rfc7396)

> This specification defines the JSON merge patch format and processing
> rules. The merge patch format is primarily intended for use with the
> HTTP PATCH method as a means of describing a set of modifications to
> a target resource's content.

For example, given the following original JSON document:

```json
{
  "a": "b",
  "c": {
    "d": "e",
    "f": "g"
  }
}
```

Changing the value of "a" and removing "f" can be achieved by
sending:

```
{
  "a":"z",
  "c": {
    "f": null
  }
}
```

When applied to the target resource, the value of the "a" member is
replaced with "z" and "f" is removed, leaving the remaining content
untouched.

Example Test Cases:

| ORIGINAL           | PATCH                     | RESULT             |
|--------------------|---------------------------|--------------------|
| {"a":"b"}          | {"a":"c"}                 | {"a":"c"}          |
| {"a":"b"}          | {"b":"c"}                 | {"a":"b", "b":"c"} |
| {"a":"b"}          | {"a":null}                | {}                 |
| {"a":"b","b":"c"}  | {"a":null}                | {"b":"c"}          |
| {"a":["b"]}        | {"a":"c"}                 | {"a":"c"}          |
| {"a":"c"}          | {"a":["b"]}               | {"a":["b"]}        |
| {"a":{"b":"c"}}    | {"a":{"b":"d","c":null}}  | {"a":{"b":"d"}}    |
| {"a":[ {"b":"c"}]} | {"a":[1]}                 | {"a":[1]}          |
| ["a","b"]          | ["c","d"]                 | ["c","d"]          |
| {"a":"b"}          | ["c"]                     | ["c"]              |
| {"a":"b"}          | ["c"]                     | ["c"]              |
| {"a":"foo"}        | null                      | null               |
| {"a":"foo"}        | "bar"                     | "bar"              |
| {"e":null}         | {"a":1}                   | {"e":null, "a":1}  |
| [1,2]              | {"a":"b", "c":null}       | {"a":"b"}          |
| {}                 | {"a":{"bb":{"ccc":null}}} | {"a":{"bb":{}}}    |

