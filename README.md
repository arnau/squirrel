# Squirrel

A meta catalogue for Lightroom catalogues.

## Arquitecture

**Squirrel** is a [Tauri] application with a [Solid.js] frontend and a [Sqlite] database.
The business logic is contained in the [nut](./crates/nut/) library.

## Development

```sh
npm run tauri dev
```

## Licence

This is private code and is not intended to be reused.


[Tauri]: https://tauri.app/
[Sqlite]: https://sqlite.org/
[Solid.js]: https://www.solidjs.com/
