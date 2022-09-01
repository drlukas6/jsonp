# JSONP

## Pretty print JSON from standard input. Whole or a specified field

---

## Usage

Example JSON:

```json
{"name":"John", "age":30, "car":null}
```

To print a whole JSON from clipboard:

``` bash
pbpaste | jsonp
```

Output:

```json
{
  "age": 30,
  "car": null,
  "name": "John"
}
```

To print only a specified field (eg. "age"):

``` bash
pbpaste | jsonp age
```

Output:

```json
age => 30
```
