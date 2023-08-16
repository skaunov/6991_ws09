This is a solution for [workshop]. Since this one turned out for me to be not as straightforward as others I'm asking an ([independent]) review for this one.

I want to thank all reviewers, especially <https://users.rust-lang.org/u/nerditation> whos comments led to considerably different approach to solution which I left in a separate <branch:ditch-`map`-from-`read_string`>.

### few notes/comments on the solution

I feel like there should be _safety arguments_ where `unsafe` is used. The only reason of their absense is that I don't actually understand what is going on in `libc` and FFI, and studying it to add the arguments is to not only learn Rustonomicon (which isn't *that* bad), but also become C programmer (which isn't on the schedule).

_Starter_ code mentioned in the exercise and useful to understand its scope is represented via <commit:80fbd6b4fe71557e3bbaf32516b9b87aa01dc178>.

The solution is a bit boilerplaty. I would tackle this with macros, but it was overstretch to me to focus both on FFI part and do this improvement. So this code stays a good object if I will need some macros practise.

# the exercise
Here's copy of relevant text from the [workshop] (to archive it and for convinience).

(Note that <data/test_file.txt> for `read`ing is provided as part of the downloaded _starter_ code.)

After the exercise text there's a section with my answers to the theoretical part concluding the given exercise.
## Exercise:
## Workshop 9 - Safety Shmafety
...
> This week, we'll be looking at some unsafe code. Before the workshop, we'll have a quick refresher on the following topics:
> 
> -    Safe abstractions over unsafe code
> -    The `unsafe` keyword
> -    `*const T` / `*mut T`

...

### Week 9 Code

In groups, your task for this week is to build a `File` struct, using primitives from the `libc` crate. These use functions which are common in the C language, but which Rust does not use.

You should implement the following on the starter code:

1.    Opening a file to read with the `read` function.
2.    Reading a string a file with `read`.
3.    Reading a type from a file with `read_i64`.
4.    Reading a type from a file with `read_f64`.
5.    Reading a type from a file with `read_char`.
6.    When your file goes out of scope, close it automatically.

To do this, you will need to know about the following functions/enums/traits, mostly from `libc`:

*    The `FILE` enum represents an open file.
*    fopen opens a file (called `filename`). To open in "read" mode, you should use `"r"` as the mode. This returns a (possibly null) file-pointer, which represents an open file.
*    fgets reads a whole line from a file. `buf` is a pointer to memory, which must be at least `n` bytes big. `stream` is a file-pointer. Note that fgets returns null if opening the file failed.
*    fscanf reads a different type from the file `stream`, depending on `format`:
  * If the string is `"%d"`, the third argument to `fscanf` should be a `&mut libc::c_int`.
  * If the string is `" %c"` (note the leading space), the third argument to `fscanf` should be a `&mut libc::c_char`.
  * If the string is `"%lf"`, the third argument to `fscanf` should be a `&mut libc::c_double`.
*    fclose should close the file pointer given to it. The Drop trait may be useful here.
*    There are a variety of types (`c_int`, `c_char`, `c_double`) which correspond to types from C. You will need these.

You should assume this code will only be run on unix-like systems, and that all paths will be ascii.

At the end of the activity, if there's time, look into the errno crate. This allows you to give the user more information about why an operation failed.

Afterwards, you might like to discuss the following points:

*    Could your type be `Copy`, `Clone`, `Send` or `Sync`?
*    Did you have to think about memory safety at all? Could you modify your code to cause a dangling pointer?
*    How easy was interoperating with C functions?

## answers to the theoretical part
### Could your type be `Copy`, `Clone`, `Send` or `Sync`?
Practically after adding all three of possible traits the output of the toy/exercise app doesn't change (including Miri, which for this exercise on my system just complaining that particular C function shouldn't be run on OS "linux"). Ofc it's way too shallow and not indicative, but at least it doesn't immediately contradict theoretical rationale/reasoning which is put below.

(Technically for `Send` or `Sync` it's enough to (unsafely) mark the `struct` with respective `trait` name.)
#### `Copy`
My arguments for `Copy` and `Clone` were same, but experiment shows that there's a formal obstacle to be `Copy`.

Never thought about it, but that makes sense. \
"the trait `Copy` cannot be implemented for this type; the type has a destructor" \

"`Copy` not allowed on types with destructors"
#### `Clone`
This `struct`/type can be `Clone`, but must not.

"Can" since pointer at the end of the day is just a number(s), which are even `Copy` in their Rust essense. It's easy just to `derive` the `trait`.

It's really crazy to manage (and deallocate) this `FILE` stream pointer while cloning it around.
#### `Send`
Seems like the only sane trait here, since it transfer ownership. I guess it should be noticed people generally should watch if anything is still using the `File` before `Send`ing it, but since everything is `unsafe` here, this is the way to deal with things.
#### `Sync`
It seems as an awful idea to me: like `Clone`, but on an order of magnitude.
### Did you have to think about memory safety at all? Could you modify your code to cause a dangling pointer?
Yes & yes; it's enough to first try to acquire `CString` `from_raw` and *then* read its usage limitation documentation.
### How easy was interoperating with C functions?
Near to a nightmare --- it's the first time I touched C since university, and I did avoid to delve it back then already. It's to personal, so better ask a person who was exposed to C previously.

# PS 
AFAIU the only thing that shouldn't be publicly shared are solutions to the graded exercises/activities, which I keep access restricted. If I got anything wrong and this code shouldn't be public as well, pls approach me by any mean you like, and I'll remove it without hesitations. Tom received couple of my contacts due to distant interactions (`macrokata`, <https://discord.com/channels/1075940806004838470/1075940806004838475/1101303841531646092>).

# references
[workshop]: https://cgi.cse.unsw.edu.au/~cs6991/23T1/workshop/09/questions
[independent]: in the sense that it's outside and not anyhow connected to NSWU.