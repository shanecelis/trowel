(function() {var implementors = {
"either":[["impl&lt;L, R, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;A&gt; for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;A&gt;,</span>"]],
"heapless":[["impl&lt;T, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;T&gt; for <a class=\"struct\" href=\"heapless/struct.HistoryBuffer.html\" title=\"struct heapless::HistoryBuffer\">HistoryBuffer</a>&lt;T, N&gt;"],["impl&lt;'a, T, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.reference.html\">&amp;'a </a>T&gt; for <a class=\"struct\" href=\"heapless/struct.HistoryBuffer.html\" title=\"struct heapless::HistoryBuffer\">HistoryBuffer</a>&lt;T, N&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;K, V, S, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.tuple.html\">(K, V)</a>&gt; for <a class=\"struct\" href=\"heapless/struct.IndexMap.html\" title=\"struct heapless::IndexMap\">IndexMap</a>&lt;K, V, S, N&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;'a, K, V, S, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.reference.html\">&amp;'a </a>K, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.reference.html\">&amp;'a </a>V)&gt; for <a class=\"struct\" href=\"heapless/struct.IndexMap.html\" title=\"struct heapless::IndexMap\">IndexMap</a>&lt;K, V, S, N&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;T, S, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;T&gt; for <a class=\"struct\" href=\"heapless/struct.IndexSet.html\" title=\"struct heapless::IndexSet\">IndexSet</a>&lt;T, S, N&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;'a, T, S, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.reference.html\">&amp;'a </a>T&gt; for <a class=\"struct\" href=\"heapless/struct.IndexSet.html\" title=\"struct heapless::IndexSet\">IndexSet</a>&lt;T, S, N&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;T, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;T&gt; for <a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt;"],["impl&lt;'a, T, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.0/std/primitive.reference.html\">&amp;'a </a>T&gt; for <a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,</span>"]],
"png":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"png/struct.Transformations.html\" title=\"struct png::Transformations\">Transformations</a>&gt; for <a class=\"struct\" href=\"png/struct.Transformations.html\" title=\"struct png::Transformations\">Transformations</a>"]],
"proc_macro2":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"enum\" href=\"proc_macro2/enum.TokenTree.html\" title=\"enum proc_macro2::TokenTree\">TokenTree</a>&gt; for <a class=\"struct\" href=\"proc_macro2/struct.TokenStream.html\" title=\"struct proc_macro2::TokenStream\">TokenStream</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"proc_macro2/struct.TokenStream.html\" title=\"struct proc_macro2::TokenStream\">TokenStream</a>&gt; for <a class=\"struct\" href=\"proc_macro2/struct.TokenStream.html\" title=\"struct proc_macro2::TokenStream\">TokenStream</a>"]],
"sdl2":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"sdl2/keyboard/struct.Mod.html\" title=\"struct sdl2::keyboard::Mod\">Mod</a>&gt; for <a class=\"struct\" href=\"sdl2/keyboard/struct.Mod.html\" title=\"struct sdl2::keyboard::Mod\">Mod</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"sdl2/messagebox/struct.MessageBoxFlag.html\" title=\"struct sdl2::messagebox::MessageBoxFlag\">MessageBoxFlag</a>&gt; for <a class=\"struct\" href=\"sdl2/messagebox/struct.MessageBoxFlag.html\" title=\"struct sdl2::messagebox::MessageBoxFlag\">MessageBoxFlag</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"sdl2/messagebox/struct.MessageBoxButtonFlag.html\" title=\"struct sdl2::messagebox::MessageBoxButtonFlag\">MessageBoxButtonFlag</a>&gt; for <a class=\"struct\" href=\"sdl2/messagebox/struct.MessageBoxButtonFlag.html\" title=\"struct sdl2::messagebox::MessageBoxButtonFlag\">MessageBoxButtonFlag</a>"]],
"trowel":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"trowel/struct.Buttons.html\" title=\"struct trowel::Buttons\">Buttons</a>&gt; for <a class=\"struct\" href=\"trowel/struct.Buttons.html\" title=\"struct trowel::Buttons\">Buttons</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"trowel/flipped/struct.Axes.html\" title=\"struct trowel::flipped::Axes\">Axes</a>&gt; for <a class=\"struct\" href=\"trowel/flipped/struct.Axes.html\" title=\"struct trowel::flipped::Axes\">Axes</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()