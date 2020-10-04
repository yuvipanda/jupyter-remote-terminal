# Guessing Game

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

1. `cargo run` is quite nice
2. You need to `use` traits to be able to access some *methods* on some types. This is a little
   different from regular imports - maybe close to C#'s extension methods? Something to watch
   out for.
3. Does rust do type inference on return values? Dispatch to methods not just based on the types
   of params passed in, but of return value! My understanding now is *yes*, although that sounds
   kinda mindblowing?
   - I just tried it, I couldn't have two functions with the same name and different return types
   - I can't have two functions with the same name, period. So no function dispatch semantics
     at all?
4. Shadowing variable names is considered ok, and even encouraged? First `guess` is a `&mut String`,
   and then you parse a `u32` into it. This is fine, instead of a separate `guess_str`. I'm not
   sure how I feel about it?
5. In my personal experience, I'm not sure how many bugs were introduced by mutability as such.
   But that's probably an indictment of how little code I've been writing recently.
6. Underscores in numbers for readability is something I've never used but seem to instantly like.
   I have to count 'ones, tens, hundreds, thousands..' too many times. Maybe they can also be used
   to distinguish GB / MB at a glance?
7. Rust is statically typed, so needs to know types of all *variables* at compile time.
8. Integer overflow checking happens in debug builds, but *not* in prod builds. I'm sure this causes
   some fun errors.
9. Rust's `char` is 4 bytes and represents a Unicode Scalar Value. But strings aren't just arrays
   of `char`s, so this is probably fine. But then what happens when you iterate over strings? I
   guess maybe that's not a thing you can do - you have to explicitly get `char`s out, or iterate
   over UTF-8 points?
10. Rust `array` types have a fixed length, so not the same as dynamic languages'. 
11. Array out of bounds access is checked, with a runtime error. Not sure what the performance
    implications of this are?
12. `snake_case` for function names. Me likey. A far cry from my fave back in the day, VB6's
    case-insensitive InitialCapCamelCase.
13. Statements & expressions are distinct, similar to Python - unlike Ruby. But many many things
    are expressions - function calls, macros, creating new scopes. Semicolons are for statements
    only - expressions don't have 'em? TODO more investigation here, especially in relation to
    how Python handles this. This also means that you need to *pay attention* to where you put
    semicolons, in a way that's more meaningful than in JS.
    - `if` and other conditionals (like `match`) are expressions, which is great. I'm guessing
      this will be very hard to do in python because of significant whitespace? Ternaries in
      Python are super awkward.
14. Only `boolean`s are true or false - none of this `truthy` business. Fits in with the rest
    of the language.
