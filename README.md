# Snippet Search

Desktop app for managing and quickly looking-up personal documentation for code snippets, dev concepts, syntax etc.

Cross-platform, fast search, performant, snappy UX, ultra-lightweight on RAM, minimal disk-size.

## Table of Contents
- [Progress](#about)
- [Plan](#plan)
- [Opinionated Design Choices](#opinionated-design-choices)
- [FAQs](#faqs-not-so-frequently-asked)
- [Stack](#stack)
- [Download the App](#download-the-app)
- [Info for Users](#info-for-users)
- [Usage Guide for Developers](#usage-guide-for-developers)
- [Contribute](#contribute)
- [Support](#support)

## Progress
**not ready for usage**

## Plan
- v0.1 : CRUD md files, code syntax highlighting
- v0.2 : add keyword-based search
- v0.3 : add LLM-based semantic search
- v0.4 : add support for pure code & txt files
- v0.5 : not decided yet

## Opinionated Design Choices

- No nesting, all files flat in one folder with no sub folder file detection. Focus is quick search not deep organization

- fork and customize > pull request and complicate

- pull requests are most welcome for bug-fixes, error-handling, quality-control; not for style customization, feature addition not listed in feature plan

## FAQs (not so frequently asked)
- Yes, you can keep it running in the background 'all the time'
- Yes, you can use it as notepad alternative, but it is opinionated by design for dev usage; your choice.
- No, it doesn't uses tauri. It is NOT web-based. It compiles into native binaries.

## Stack
- rust
- `iced` crate for native GUI

## Download the App
Releases link :
Website link :

## Info for Users

- Search currently uses keyword-based content matching. (I have future plans to provide optional choice to integrate local ollama/LM-studio-based or API-based cloud LLM models or both for semantic search)

- Files are stored in $HOME directory on linux and mac and users/yourname on windows, by default, hard-coded. (will add custom location choice feature in later version)


## Usage Guide for Developers

- clone the repo with `git clone <repo_link>` and run with cargo package manager.

```shell
cargo run
```

## Contribute
soon...

## Support
Buy me a Chai (INR / UPI supported) :
Buy me a Coffee (International) :