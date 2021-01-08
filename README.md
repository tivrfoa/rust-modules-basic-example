# rust-modules-basic-example
Rust modules basic example. Shows how to use 'mod' and 'use'

From:
https://users.rust-lang.org/t/i-love-rust-but-one-thing-about-modules-is-aweful/2930/12

>kstep 2 Sep '15
>
>Seems like you badly misunderstand modules in Rust. They are not like namespaces, they are a little closer to C’s #include but with better semantics.
>
>When you write your code, you implicitly create a crate, and your main.rs is the root of your crate. This is an entry point to your whole application, and when you put mod mymodule; into it, it just means something like #include "mymodule.c"; in C, that is you declare you have a module, and the compiler should look for it in mymodule.rs or mymodule/mod.rs. Thanks to holy Ferris 50 you need to do it only once in the crate’s root, unlike in C. Once the module is declared (read “included from other file”) in the crate root, you can use it from any module in the same crate however you want. And, unlike in C, all files included with mod, are put into separate “namespace”, so names from these other files don’t mess up with your current file.
>
>When you write use mymodule;, you don’t declare (“include”) it, you just tell compiler to move some names from the module into your current namespace, so you don’t ::have::to::write::full::path::to::your::names::in::some::module::just::to::call::some::function(). You don’t have to use thingies from other modules with use, you can ::always::use::full::paths, but it’s just annoying.
>
>Hope it helps.

It sure does! =)

-------------

https://users.rust-lang.org/t/i-love-rust-but-one-thing-about-modules-is-aweful/2930/27

```
pixel Sep '15

I may be missing the question but when I’m writing applications I usually end up with a structure like:

main.rs
mod_a\mod.rs
mod_a\struct_aa.rs
mod_a\struct_ab.rs
mod_b\mod.rs
mod_b\struct_ba.rs
mod_b\struct_bb.rs
And yes main.rs 2 has the:

mod mod_a;
mod mod_b;
And mod_a\mod.rs 2 has:

mod struct_aa.rs
mod struct_ab.rs
But it’s only main.rs 2 and the various mod.rs 2 files that have the “mod” commands. My main.rs 2 also ends up being very small because pub fn main() usually creates a few objects and then jumps into one of those objects to perform the actual work. So the number of “use” commands in main.rs 2 ends up being quite small.

Then where the real work is going on like in struct_aa.rs I have the various includes:

use mod_a::struct_aa::ObjectAaa;
use mod_b::struct_bb::{ObjectBba, ObjecBbb};

For the objects that source file needs to use/reference. But no “mod” commands there.
```
