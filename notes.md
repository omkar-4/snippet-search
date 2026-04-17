## Elm

### What is Elm?
Functional language for building UIs. It enforces strict pattern called 'the elm architecture (TEA)' -> makes apps predictable and bug-resistant.

### Core Idea
1. Model = app state
2. Message = events (user/system actions)
3. Update = function: (msg, model) -> new model
4. View = model -> UI

App has a current state and UI. When a user/system performs an action it fires an event/message which triggers update function that returns the new state which runtime treats as state change; updated state makes runtime call view function after state change which redraws UI

explicit state replacement -> re-render

### Flow
user action -> Message -> Update changes model -> view redraws UI

### Elm arch in Iced
- struct state -> model
- enum Message -> events/messages
- fn update(&mut self, message) -> changes state
- fn view(&self) -> repaints UI

---

## Event / Message System

It's a controlled pipeline for changes where nothing happens silently.

Instead of mutating state directly, every change is turned into message/event and one authority (function) decides how state changes and when screen repaints.

events are async.

Message is data, not action.

function interprets and returns new state.

UI is pure reflection of state.


### Mental Model
x 'delete button' deletes file
✓ clicking 'delete button' emits DeletePressed; update decides what it means and 'acts'; view/UI only 'reports'.

---

## Rust Concepts

### enum (enumeration)

'Custom Type' that can be one of a fixed set of options called variants. Variants are not-subtypes, they are values/states.

if `enum is container` then its `variant is its current shape/value`

There can be exactly one variant at a time, never many, never outside the defined set.

### String vs &str
- String : owned box of bytes on heap, heap buffer (can grow/shrink)
- &str : pointer to some bytes, no ownership (look at someone else's memory)

```rs

s.chars().take(max).collect()
// chars() → turn string into a stream of characters
// take(max) → stop after max characters
// collect() → build a new String from them

```

### Option
Safe box for 'maybe value exists'. handler to prevent invalid values causing panics.

### Some & None
Many functions in rust don't return values, they return:
Some(value) : value exists
None : no value

```rs
enum Option<T> {
  Some(T),
  None
}
```

```rs
fn truncate(s: &str, max: usize) -> String {
    match s.char_indices().nth(max) {
        Some((idx, _)) => format!("{}...", &s[..idx]),
        None => s.to_string(),
    }
}

/* returns
* Some((idx, char)) -> If position exists
* None -> If string too short
*/
```

### super
goes 1 level up in the module tree

```rs
use super::file_card::FileCard;

// super would be 'ui' > file_card > FileCard
```
