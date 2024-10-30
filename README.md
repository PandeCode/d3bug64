###### Personal Site

Over engineered personal site written in the yew framework in rust. Using taildwindcss classes for styling.

On push, a github workflow is run to compile and bundle the tailwind, css, assets and lastly the rust-yew code into wasm.
The compiled and bundled output is then pushed to my github pages repo.

###### Running locally

```bash
trunk serve --open
```
