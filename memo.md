# Memo

## Usage

To utilize learn-rs, you should learn the following:
1. Each mod is a seperate little project and should contain a main function.
   - The mod should be either a file or a mod-style folder.
   - `crate::utils` can provide / be added with common modules, e.g. std.
2. A Wrapper struct is presented to deal with the testing logic.
   - new("Description", main_function) -> create a test item.
   - `using(bool)` -> an action toggle, perform if true; always leaves a mark.
     - prints ">>> {name} <<<" as op of a test.
     - prints "^^^ {name} ^^^\n" as ed of a test. 
     - prints "xxx {name} xxx\n" if skipped.

## Git

Commit messages obey the following notation:
```
[- <verb>[[:|!] <explain>|.]]*
```
where `[]` is "word group", `|` is "or", `*` is "repeat precedent", and `<>` is "label".

Use '!' instead of ':' to declare importance.

The following may serve as an example:
```
- impl: Tree.
- done: data model.
- try! RefCell.
- clean.
- memo.
```

You should keep in mind that as long as the notation is readable and detailed, it's fine to invent expressions.

// Todo: vocabulary list.

### Possible `<verb>`
```
- init.     Any init behavior. Always the root commit containing a single .gitignore.
- impl.     Aggressive or constructive outcome.
- done.     General "task complete" verb.
- rapid.    Quickly build a structure without detailed implementations.
- frame.    Same as "rapid" but emphasize on a well-defined structure.
- reframe.  Alter old structures.
- try.      Make an uncertain attempt.
- failing.  Not working, but progressive.
- revert.   Not working, fuck this patch.
- debug.    Working on an ill feature and it's getting better.
- fix.      Bug gone.
- chore.    Some unimportant-but-worthy-enough-to-be-recorded jobs.
- misc.     Anything unimportant. A reminder for impure commits: "not as it sounds".
- clean.    Clean up some messy code / comments.
- memo.     Update memo.
- polish.   Just a little more tire up...
- ver.      New version release!
- log.      Log the version.
```

### Rules for `<explain>`
