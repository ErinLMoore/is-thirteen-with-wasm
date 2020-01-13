<img src="isthirteen-logo-small.png">

# find out if it's 13...... with near-native performance 

## how to use this library
Give a value to the function, `is_thirteen(var)`, and it will return a boolean that tells you if the value is 13.

run tests: `cargo test`

npm install: `npm i @erin-l-moore/is-thirteen-with-wasm`

sample import and usage:
```
const find_out = async possibly_thirteen => {
  var wasm = await import("@erin-l-moore/is-thirteen-with-wasm")
  return wasm.is_thirteen(possibly_thirteen)
}
```

`find_out("13")` -> returns `true`
`find_out("thirteen")` -> returns `true`
`find_out("12")` -> returns `false`

inspired by (and canonical list of thirteens borrowed under license from): [is-thirteen](https://github.com/jezen/is-thirteen)