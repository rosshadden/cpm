# cpm
Cosmic Package Manager

## WTF‽‽‽

Cosmic packages are a language-agnostic format for defining common details about packages.
There are several reasons for this, and while some of them may be individually dismissed as "laziness", I believe that as a whole they make a case for existence.

### Properties

Properties like the name, version, description, authors, and repositories should immediately come to mind, and we can start with these in explaining the theorized void that `cpm` fills.

Many developers---hobbyists and professionals alike---are polyglots.
We work across different languages, each with (or in some cases lacking) their own package ecosystems.
For scripting reasons alone it is beneficial to have a universal definition of a lot of static properties.

### Actions

Perhaps a more useful thing to store in a language-agnostic way are what some other package managers call "scripts" or "targets".

When we moved away from `Makefile`s in most ecosystems, we lost a very important thing:  A universal way to run actions.
Granted, the action-running concept has often been ported to their language-specific package definitions, but we can benefit from keeping these agnostic.

### Dependencies

Many projects necessarily span multiple languages or frameworks.
It is therefore not uncommon to see for example both a `package.json` file and a `bower.json` file, or `package.json` and `Cargo.toml`, etc.
`npm` is in fact a great example even by itself, as there are many packages stored within it that aren't even for JavaScript or Node.js.
In some cases this is because such packages have JavaScript dependencies, and in other cases it is simply for the conveniences that `npm` offers.

Cosmic packages can define dependencies that reside in different places, such as `npm` and `bower`.

## Features

- [x] define arbitrary actions, to be run by the cli
- [ ] get properties from the cli
- [ ] set properties from the cli
- [ ] special operations for certain types
	- [ ] increment/decrement numbers
	- [ ] toggle booleans
	- [ ] update dates to right now
- [ ] define schemas for `cosmic.toml` files
	- [ ] support static properties
	- [ ] support types
	- [ ] support defaults
	- [ ] support dependencies from multiple package manager sources
- [ ] import/fill from existing package files
- [ ] export to specific package formats

## Ideas

- add a `sync` command that keeps package files in sync
- maybe add an opt-in git hook or something, that calls `sync`

## Goals

- draw inspiration from the good package managers of today, normalizing the common things and introducing the uncommon
- support common package managers
	- `npm`, `pip`, `cargo`, `gem`, `bower`, `luarocks`
- replace or support common task runners
	- `Makefile`, `PKGBUILD`, `grunt`, `gulp`

Maybe some of these goals (namely the task-runner one) are stupid, or entirely unnecessary.
I don't really know yet.
The idea is not necessarily solidified in my mind, and will certainly evolve to encompass more or possibly even less than I have currently decided.
