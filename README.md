# rustlings

Small exercises to get you used to reading and writing Rust code. Includes practice reading and responding to
compiler messages!

This repo is very much the smallest thing that could possibly work :)

## To do these exercises

Thanks to [btbytes'](https://twitter.com/btbytes) [prlinks](https://github.com/btbytes/prlink), you can now click on the links below to load the exercises in the rust playground!

There are infinite correct answers-- the exercises are sometimes left very open-ended. Scroll down in the playground to find comments that have hints.

If you need more help or would like to compare solutions, you can ask in [#rust-beginners on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners ), the [user forum](https://users.rust-lang.org/), or [the subreddit](https://reddit.com/r/rust). If an exercise could be improved in any way, please [create an issue](https://github.com/carols10cents/rustlings/issues/new) or submit a pull request!

### Variable bindings

[Relevant chapter in The Rust Programming Language](https://doc.rust-lang.org/stable/book/variable-bindings.html)

- ["variables1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++x+%3D+5%3B%0A++++println%21%28%22x+has+the+value+%7B%7D%22%2C+x%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Hint%3A+The+declaration+on+line+4+is+missing+a+keyword+that+is+needed+in+Rust%0A%2F%2F+to+create+a+new+variable+binding.%0A)
- ["variables2.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++let+x%3B%0A++++if+x+%3D%3D+10+%7B%0A++++++++println%21%28%22Ten%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22Not+ten%21%22%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+The+compiler+message+is+saying+that+Rust+cannot+infer+the+type+that+the%0A%2F%2F+variable+binding+%60x%60+has+with+what+is+given+here.%0A%2F%2F+What+happens+if+you+annotate+line+4+with+a+type+annotation%3F%0A%2F%2F+What+if+you+give+x+a+value%3F%0A%2F%2F+What+if+you+do+both%3F%0A%2F%2F+What+type+should+x+be%2C+anyway%3F%0A%2F%2F+What+if+x+is+the+same+type+as+10%3F+What+if+it%27s+a+different+type%3F%0A)
- ["variables3.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++let+x+%3D+3%3B%0A++++println%21%28%22Number+%7B%7D%22%2C+x%29%3B%0A++++x+%3D+5%3B%0A++++println%21%28%22Number+%7B%7D%22%2C+x%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+In+Rust%2C+variable+bindings+are+immutable+by+default.+But+here+we%27re+trying%0A%2F%2F+to+reassign+a+different+value+to+x%21+There%27s+a+keyword+we+can+use+to+make%0A%2F%2F+a+variable+binding+mutable+instead.%0A)
- ["variables4.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++let+x%3A+i32%3B%0A++++println%21%28%22Number+%7B%7D%22%2C+x%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Oops%21+In+this+exercise%2C+we+have+a+variable+binding+that+we%27ve+created+on%0A%2F%2F+line+4%2C+and+we%27re+trying+to+use+it+on+line+5%2C+but+we+haven%27t+given+it+a%0A%2F%2F+value.+We+can%27t+print+out+something+that+isn%27t+there%3B+try+giving+x+a+value%21%0A%2F%2F+This+is+an+error+that+can+cause+bugs+that%27s+very+easy+to+make+in+any%0A%2F%2F+programming+language+--+thankfully+the+Rust+compiler+has+caught+this+for+us%21%0A)

### Functions

[Relevant chapter in The Rust Programming Language](https://doc.rust-lang.org/stable/book/functions.html)

- ["functions1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++call_me%28%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+This+main+function+is+calling+a+function+that+it+expects+to+exist%2C+but+the%0A%2F%2F+function+doesn%27t+exist.+It+expects+this+function+to+have+the+name+%60call_me%60.%0A%2F%2F+It+expects+this+function+to+not+take+any+arguments+and+not+return+a+value.%0A%2F%2F+Sounds+a+lot+like+%60main%60%2C+doesn%27t+it%3F%0A)
- ["functions2.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++call_me%283%29%3B%0A%7D%0A%0Afn+call_me%28num%29+%7B%0A++++for+i+in+0..num+%7B%0A++++++++println%21%28%22Ring%21+Call+number+%7B%7D%22%2C+i+%2B+1%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Rust+requires+that+all+parts+of+a+function%27s+signature+have+type+annotations%2C%0A%2F%2F+but+%60call_me%60+is+missing+the+type+annotation+of+%60num%60.%0A)
- ["functions3.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++call_me%28%29%3B%0A%7D%0A%0Afn+call_me%28num%3A+i32%29+%7B%0A++++for+i+in+0..num+%7B%0A++++++++println%21%28%22Ring%21+Call+number+%7B%7D%22%2C+i+%2B+1%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+This+time%2C+the+function+*declaration*+is+okay%2C+but+there%27s+something+wrong%0A%2F%2F+with+the+place+where+we%27re+calling+the+function.%0A)
- ["functions4.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0A%2F%2F+This+store+is+having+a+sale+where+if+the+price+is+an+even+number%2C+you+get%0A%2F%2F+10+%28money+unit%29+off%2C+but+if+it%27s+an+odd+number%2C+it%27s+3+%28money+unit%29+less.%0A%0Afn+main%28%29+%7B%0A++++let+original_price+%3D+51%3B%0A++++println%21%28%22Your+sale+price+is+%7B%7D%22%2C+sale_price%28original_price%29%29%3B%0A%7D%0A%0Afn+sale_price%28price%3A+i32%29+-%3E+%7B%0A++++if+is_even%28price%29+%7B%0A++++++++price+-+10%0A++++%7D+else+%7B%0A++++++++price+-+3%0A++++%7D%0A%7D%0A%0Afn+is_even%28num%3A+i32%29+-%3E+bool+%7B%0A++++num+%25+2+%3D%3D+0%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+The+error+message+points+to+line+10+and+says+it+expects+a+type+after+the%0A%2F%2F+%60-%3E%60.+This+is+where+the+function%27s+return+type+should+be--+take+a+look+at%0A%2F%2F+the+%60is_even%60+function+for+an+example%21%0A)
- ["functions5.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++let+answer+%3D+square%283%29%3B%0A++++println%21%28%22The+answer+is+%7B%7D%22%2C+answer%29%3B%0A%7D%0A%0Afn+square%28num%3A+i32%29+-%3E+i32+%7B%0A++++num+*+num%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+This+is+a+really+common+error+that+can+be+fixed+by+removing+one+character.%0A%2F%2F+It+happens+because+Rust+distinguishes+between+expressions+and+statements%3A+expressions+return%0A%2F%2F+a+value+and+statements+don%27t.+We+want+to+return+a+value+from+the+%60square%60+function%2C+but+it%0A%2F%2F+isn%27t+returning+one+right+now...%0A)

### Primitive types

[Relevant chapter in The Rust Programming Language](https://doc.rust-lang.org/stable/book/primitive-types.html)

- ["primitive_types1.rs"](http://play.rust-lang.org/?code=%2F%2F+Fill+in+the+rest+of+the+line+that+has+code+missing%21%0A%2F%2F+No+hints%2C+there%27s+no+tricks%2C+just+get+used+to+typing+these+%3A%29%0A%0Afn+main%28%29+%7B%0A++++%2F%2F+Booleans+%28%60bool%60%29%0A%0A++++let+is_morning+%3D+true%3B%0A++++if+is_morning+%7B%0A++++++++println%21%28%22Good+morning%21%22%29%3B%0A++++%7D%0A%0A++++let+%2F%2F+Finish+the+rest+of+this+line+like+the+example%21+Or+make+it+be+false%21%0A++++if+is_evening+%7B%0A++++++++println%21%28%22Good+evening%21%22%29%3B%0A++++%7D%0A%7D%0A)
- ["primitive_types2.rs"](http://play.rust-lang.org/?code=%2F%2F+Fill+in+the+rest+of+the+line+that+has+code+missing%21%0A%2F%2F+No+hints%2C+there%27s+no+tricks%2C+just+get+used+to+typing+these+%3A%29%0A%0Afn+main%28%29+%7B%0A++++%2F%2F+Characters+%28%60char%60%29%0A%0A++++let+my_first_initial+%3D+%27C%27%3B%0A++++if+my_first_initial.is_alphabetic%28%29+%7B%0A++++++++println%21%28%22Alphabetical%21%22%29%3B%0A++++%7D+else+if+my_first_initial.is_numeric%28%29+%7B%0A++++++++println%21%28%22Numerical%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22Neither+alphabetic+nor+numeric%21%22%29%3B%0A++++%7D%0A%0A++++let+%2F%2F+Finish+this+line+like+the+example%21+What%27s+your+favorite+character%3F%0A++++%2F%2F+Try+a+letter%2C+try+a+number%2C+try+a+special+character%2C+try+a+character%0A++++%2F%2F+from+a+different+language+than+your+own%2C+try+an+emoji%21%0A++++if+your_character.is_alphabetic%28%29+%7B%0A++++++++println%21%28%22Alphabetical%21%22%29%3B%0A++++%7D+else+if+your_character.is_numeric%28%29+%7B%0A++++++++println%21%28%22Numerical%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22Neither+alphabetic+nor+numeric%21%22%29%3B%0A++++%7D%0A%7D%0A)
- ["primitive_types3.rs"](http://play.rust-lang.org/?code=%2F%2F+Create+an+array+with+at+least+100+elements+in+it+where+the+%3F%3F%3F+is.+%0A%2F%2F+Scroll+down+for+hints%21%0A%0Afn+main%28%29+%7B%0A++++let+a+%3D+%3F%3F%3F%0A%0A++++if+a.len%28%29+%3E%3D+100+%7B%0A++++++++println%21%28%22Wow%2C+that%27s+a+big+array%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22Meh%2C+I+eat+arrays+like+that+for+breakfast.%22%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+There%27s+a+shorthand+to+initialize+Arrays+with+a+certain+size+that+does+not+%0A%2F%2F+require+you+to+type+in+100+items+%28but+you+certainly+can+if+you+want%21%29%0A%2F%2F+Check+out+the+Primitive+Types+-%3E+Arrays+section+of+the+book%3A%0A%2F%2F+http%3A%2F%2Fdoc.rust-lang.org%2Fstable%2Fbook%2Fprimitive-types.html%23arrays%0A%2F%2F+Bonus%3A+what+are+some+other+things+you+could+have+that+would+return+true%0A%2F%2F+for+%60a.len%28%29+%3E%3D+100%60%3F%0A)
- ["primitive_types4.rs"](http://play.rust-lang.org/?code=%2F%2F+Get+a+slice+out+of+Array+a+where+the+%3F%3F%3F+is+so+that+the+%60if%60+statement%0A%2F%2F+returns+true.+Scroll+down+for+hints%21%21%0A%0Afn+main%28%29+%7B%0A++++let+a+%3D+%5B1%2C+2%2C+3%2C+4%2C+5%5D%3B%0A%0A++++let+nice_slice+%3D+%3F%3F%3F%0A%0A++++if+nice_slice+%3D%3D+%5B2%2C+3%2C+4%5D+%7B%0A++++++++println%21%28%22Nice+slice%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22Not+quite+what+I+was+expecting...+I+see%3A+%7B%3A%3F%7D%22%2C+nice_slice%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Take+a+look+at+the+Primitive+Types+-%3E+Slices+section+of+the+book%3A%0A%2F%2F+http%3A%2F%2Fdoc.rust-lang.org%2Fstable%2Fbook%2Fprimitive-types.html%23slices%0A%2F%2F+and+use+the+starting+and+ending+indices+of+the+items+in+the+Array%0A%2F%2F+that+you+want+to+end+up+in+the+slice.%0A%0A%2F%2F+If+you%27re+curious+why+the+right+hand+of+the+%60%3D%3D%60+comparison+does+not%0A%2F%2F+have+an+ampersand+for+a+reference+since+the+left+hand+side+is+a%0A%2F%2F+reference%2C+take+a+look+at+the+Deref+coercions+chapter%3A%0A%2F%2F+http%3A%2F%2Fdoc.rust-lang.org%2Fstable%2Fbook%2Fderef-coercions.html%0A)
- ["primitive_types5.rs"](http://play.rust-lang.org/?code=%2F%2F+Destructure+the+%60cat%60+tuple+so+that+the+println+will+work.%0A%2F%2F+Scroll+down+for+hints%21%0A%0Afn+main%28%29+%7B%0A++++let+cat+%3D+%28%22Furry+McFurson%22%2C+3.5%29%3B%0A++++let+%2F*+your+pattern+here+*%2F+%3D+cat%3B%0A%0A++++println%21%28%22%7B%7D+is+%7B%7D+years+old.%22%2C+name%2C+age%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Take+a+look+at+the+Primitive+Types+-%3E+Tuples+section+of+the+book%3A%0A%2F%2F+http%3A%2F%2Fdoc.rust-lang.org%2Fstable%2Fbook%2Fprimitive-types.html%23tuples%0A%2F%2F+Particularly+the+part+about+%22destructuring+lets%22.+You%27ll+need+to%0A%2F%2F+make+a+pattern+to+bind+%60name%60+and+%60age%60+to+the+appropriate+parts%0A%2F%2F+of+the+tuple.+You+can+do+it%21%21%0A)
- ["primitive_types6.rs"](http://play.rust-lang.org/?code=%2F%2F+Use+a+tuple+index+to+access+the+second+element+of+%60numbers%60.%0A%2F%2F+You+can+put+this+right+into+the+%60println%21%60+where+the+%3F%3F%3F+is.%0A%2F%2F+Scroll+down+for+hints%21%0A%0Afn+main%28%29+%7B%0A++++let+numbers+%3D+%281%2C+2%2C+3%29%3B%0A++++println%21%28%22The+second+number+is+%7B%7D%22%2C+%3F%3F%3F%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+While+you+could+use+a+destructuring+%60let%60+for+the+tuple+here%2C+try+%0A%2F%2F+indexing+into+it+instead%2C+as+explained+here%3A%0A%2F%2F+http%3A%2F%2Fdoc.rust-lang.org%2Fstable%2Fbook%2Fprimitive-types.html%23tuple-indexing%0A%2F%2F+Now+you+have+another+tool+in+your+toolbox%21%0A)

### Tests

Going out of order from the Syntax and Semantics section of the book to cover tests-- many of the
following exercises will ask you to make tests pass!

[Testing chapter from the Effective Rust section of the book](https://doc.rust-lang.org/stable/book/testing.html)

- ["tests1.rs"](http://play.rust-lang.org/?code=%2F%2F+This+test+has+a+problem+with+it+--+make+the+test+compile%21+Make+the+test%0A%2F%2F+pass%21+Make+the+test+fail%21+Scroll+down+for+hints+%3A%29%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++%23%5Btest%5D%0A++++fn+you_can_assert%28%29+%7B%0A++++++++assert%21%28%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+You+don%27t+even+need+to+write+any+code+to+test+--+you+can+just+test+values+and+run+that%2C+even%0A%2F%2F+though+you+wouldn%27t+do+that+in+real+life+%3A%29+%60assert%21%60+is+a+macro+that+needs+an+argument.%0A%2F%2F+Depending+on+the+value+of+the+argument%2C+%60assert%21%60+will+do+nothing+%28in+which+case+the+test+will%0A%2F%2F+pass%29+or+%60assert%21%60+will+panic+%28in+which+case+the+test+will+fail%29.+So+try+giving+different+values%0A%2F%2F+to+%60assert%21%60+and+see+which+ones+compile%2C+which+ones+pass%2C+and+which+ones+fail+%3A%29%0A)
- ["tests2.rs"](http://play.rust-lang.org/?code=%2F%2F+This+test+has+a+problem+with+it+--+make+the+test+compile%21+Make+the+test%0A%2F%2F+pass%21+Make+the+test+fail%21+Scroll+down+for+hints+%3A%29%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++%23%5Btest%5D%0A++++fn+you_can_assert_eq%28%29+%7B%0A++++++++assert_eq%21%28%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Like+the+previous+exercise%2C+you+don%27t+need+to+write+any+code+to+get+this+test+to+compile+and%0A%2F%2F+run.+%60assert_eq%21%60+is+a+macro+that+takes+two+arguments+and+compares+them.+Try+giving+it+two%0A%2F%2F+values+that+are+equal%21+Try+giving+it+two+arguments+that+are+different%21+Try+giving+it+two+values%0A%2F%2F+that+are+of+different+types%21+Try+switching+which+argument+comes+first+and+which+comes+second%21%0A)
- ["tests3.rs"](http://play.rust-lang.org/?code=%2F%2F+This+test+isn%27t+testing+our+function+--+make+it+do+that+in+such+a+way+that%0A%2F%2F+the+test+passes.+Then+write+a+second+test+that+tests+that+we+get+the+result%0A%2F%2F+we+expect+to+get+when+we+call+%60is_even%285%29%60.+Scroll+down+for+hints%21%0A%0Apub+fn+is_even%28num%3A+i32%29+-%3E+bool+%7B%0A++++num+%25+2+%3D%3D+0%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+is_true_when_even%28%29+%7B%0A++++++++assert%21%28false%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+You+can+call+a+function+right+where+you%27re+passing+arguments+to+%60assert%21%60+--+so+you+could+do%0A%2F%2F+something+like+%60assert%21%28having_fun%28%29%29%60.+If+you+want+to+check+that+you+indeed+get+false%2C+you%0A%2F%2F+can+negate+the+result+of+what+you%27re+doing+using+%60%21%60%2C+like+%60assert%21%28%21having_fun%28%29%29%60.%0A)
- ["tests4.rs"](http://play.rust-lang.org/?code=%2F%2F+This+test+isn%27t+testing+our+function+--+make+it+do+that+in+such+a+way+that%0A%2F%2F+the+test+passes.+Then+write+a+second+test+that+tests+that+we+get+the+result%0A%2F%2F+we+expect+to+get+when+we+call+%60times_two%60+with+a+negative+number.%0A%2F%2F+No+hints%2C+you+can+do+this+%3A%29%0A%0Apub+fn+times_two%28num%3A+i32%29+-%3E+i32+%7B%0A++++num+*+2%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+returns_twice_of_positive_numbers%28%29+%7B%0A++++++++assert_eq%21%284%2C+4%29%3B%0A++++%7D%0A%7D%0A)

### If

[Relevant chapter in The Rust Programming Language](https://doc.rust-lang.org/stable/book/if.html)

- ["if1.rs"](http://play.rust-lang.org/?code=fn+bigger%28a%3A+i32%2C+b%3Ai32%29+-%3E+i32+%7B%0A++++%2F%2F+Complete+this+function+to+return+the+bigger+number%21%0A++++%2F%2F+Do+not+use%3A%0A++++%2F%2F+-+return%0A++++%2F%2F+-+another+function+call%0A++++%2F%2F+-+additional+variables%0A++++%2F%2F+Scroll+down+for+hints.%0A%7D%0A%0Afn+main%28%29+%7B%0A++++assert_eq%21%2810%2C+bigger%2810%2C+8%29%29%3B%0A++++assert_eq%21%2842%2C+bigger%2832%2C+42%29%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+It%27s+possible+to+do+this+in+one+line+if+you+would+like%21%0A%2F%2F+Some+similar+examples+from+other+languages%3A%0A%2F%2F+-+In+C%28%2B%2B%29+this+would+be%3A+%60a+%3E+b+%3F+a+%3A+b%60%0A%2F%2F+-+In+Python+this+would+be%3A++%60a+if+a+%3E+b+else+b%60%0A%2F%2F+Remember+in+Rust+that%3A%0A%2F%2F+-+the+%60if%60+condition+does+not+need+to+be+surrounded+by+parentheses%0A%2F%2F+-+%60if%60%2F%60else%60+conditionals+are+expressions%0A%2F%2F+-+Each+condition+is+followed+by+a+%60%7B%7D%60+block.%0A)

### Strings

[Relevant chapter in The Rust Programming Language](https://doc.rust-lang.org/stable/book/strings.html)

- ["strings1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile+without+changing+the+function+signature%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++let+answer+%3D+current_favorite_color%28%29%3B%0A++++println%21%28%22My+current+favorite+color+is+%7B%7D%22%2C+answer%29%3B%0A%7D%0A%0Afn+current_favorite_color%28%29+-%3E+String+%7B%0A++++%22blue%22%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+The+%60current_favorite_color%60+function+is+currently+returning+a+string+slice+with+the+%60%27static%60%0A%2F%2F+lifetime.+We+know+this+because+the+data+of+the+string+lives+in+our+code+itself+--+it+doesn%27t%0A%2F%2F+come+from+a+file+or+user+input+or+another+program+--+so+it+will+live+as+long+as+our+program%0A%2F%2F+lives.+But+it+is+still+a+string+slice.+There%27s+one+way+to+create+a+%60String%60+by+converting+a%0A%2F%2F+string+slice+covered+in+the+Strings+chapter+of+the+book%2C+and+another+way+that+uses+the+%60From%60%0A%2F%2F+trait.%0A)
- ["strings2.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile+without+changing+the+function+signature%21+Scroll+down+for+hints+%3A%29%0A%0Afn+main%28%29+%7B%0A++++let+guess1+%3D+%22blue%22.to_string%28%29%3B+%2F%2F+Try+not+changing+this+line+%3A%29%0A++++let+correct+%3D+guess_favorite_color%28guess1%29%3B%0A++++if+correct+%7B%0A++++++++println%21%28%22You+guessed+correctly%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22Nope%2C+that%27s+not+it.%22%29%3B%0A++++%7D%0A%7D%0A%0Afn+guess_favorite_color%28attempt%3A+%26str%29+-%3E+bool+%7B%0A++++attempt+%3D%3D+%22green%22%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Yes%2C+it+would+be+really+easy+to+fix+this+by+just+changing+the+value+bound+to+%60guess1%60+to+be+a%0A%2F%2F+string+slice+instead+of+a+%60String%60%2C+wouldn%27t+it%3F%3F+There+is+a+way+to+add+one+character+to+line%0A%2F%2F+5%2C+though%2C+that+will+coerce+the+%60String%60+into+a+string+slice.%0A)
- ["strings3.rs"](http://play.rust-lang.org/?code=%2F%2F+Ok%2C+here+are+a+bunch+of+values--+some+are+%60Strings%60%2C+some+are+%60%26strs%60.+Your%0A%2F%2F+task+is+to+call+one+of+these+two+functions+on+each+value+depending+on+what%0A%2F%2F+you+think+each+value+is.+That+is%2C+add+either+%60string_slice%60+or+%60string%60%0A%2F%2F+before+the+parentheses+on+each+line.+If+you%27re+right%2C+it+will+compile%21%0A%0Afn+string_slice%28arg%3A+%26str%29+%7B+println%21%28%22%7B%7D%22%2C+arg%29%3B+%7D%0Afn+string%28arg%3A+String%29+%7B+println%21%28%22%7B%7D%22%2C+arg%29%3B+%7D%0A%0Afn+main%28%29+%7B%0A++++%28%22blue%22%29%3B%0A++++%28%22red%22.to_string%28%29%29%3B%0A++++%28String%3A%3Afrom%28%22hi%22%29%29%3B%0A++++%28format%21%28%22Interpolation+%7B%7D%22%2C+%22Station%22%29%29%3B%0A++++%28%26String%3A%3Afrom%28%22abc%22%29%5B0..1%5D%29%3B%0A++++%28%22++hello+there+%22.trim%28%29%29%3B%0A++++%28%22Happy+Monday%21%22.to_string%28%29.replace%28%22Mon%22%2C+%22Tues%22%29%29%3B%0A++++%28%22mY+sHiFt+KeY+iS+sTiCkY%22.to_lowercase%28%29%29%3B%0A%7D%0A)

### Move semantics

These exercises are adapted from [pnkfelix]()'s [Rust Tutorial](http://pnkfelix.github.io/rust-examples-icfp2014/) -- thank you Felix!!!

Relevant chapters in the book:
- [Ownership](https://doc.rust-lang.org/stable/book/ownership.html)
- [References and borrowing](https://doc.rust-lang.org/stable/book/references-and-borrowing.html)

Note that the exercises in this section may look similar to each other but they are subtly different :)

- ["move_semantics1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Apub+fn+main%28%29+%7B%0A++++let+vec0+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++let+vec1+%3D+fill_vec%28vec0%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A++++vec1.push%2888%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A%7D%0A%0Afn+fill_vec%28vec%3A+Vec%3Ci32%3E%29+-%3E+Vec%3Ci32%3E+%7B%0A++++let+mut+vec+%3D+vec%3B%0A%0A++++vec.push%2822%29%3B%0A++++vec.push%2844%29%3B%0A++++vec.push%2866%29%3B%0A%0A++++vec%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+So+you%27ve+got+the+%22cannot+borrow+immutable+local+variable+%60vec1%60+as+mutable%22+error+on+line+10%2C%0A%2F%2F+right%3F+The+fix+for+this+is+going+to+be+adding+one+keyword%2C+and+the+addition+is+NOT+on+line+10%0A%2F%2F+where+the+error+is.%0A)
- ["move_semantics2.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile+without+changing+line+9%21+Scroll+down+for+hints+%3A%29%0A%0Apub+fn+main%28%29+%7B%0A++++let+vec0+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++let+mut+vec1+%3D+fill_vec%28vec0%29%3B%0A%0A++++%2F%2F+Do+not+change+the+following+line%21%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec0%22%2C+vec0.len%28%29%2C+vec0%29%3B%0A%0A++++vec1.push%2888%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A%7D%0A%0Afn+fill_vec%28vec%3A+Vec%3Ci32%3E%29+-%3E+Vec%3Ci32%3E+%7B%0A++++let+mut+vec+%3D+vec%3B%0A%0A++++vec.push%2822%29%3B%0A++++vec.push%2844%29%3B%0A++++vec.push%2866%29%3B%0A%0A++++vec%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+So+%60vec0%60+is+being+*moved*+into+the+function+%60fill_vec%60+when+we+call+it+on%0A%2F%2F+line+6%2C+which+means+it+gets+dropped+at+the+end+of+%60fill_vec%60%2C+which+means+we%0A%2F%2F+can%27t+use+%60vec0%60+again+on+line+9+%28or+anywhere+else+in+%60main%60+after+the%0A%2F%2F+%60fill_vec%60+call+for+that+matter%29.+We+could+fix+this+in+a+few+ways%2C+try+them%0A%2F%2F+all%21%0A%2F%2F+1.+Make+another%2C+separate+version+of+the+data+that%27s+in+%60vec0%60+and+pass+that%0A%2F%2F+to+%60fill_vec%60+instead.%0A%2F%2F+2.+Make+%60fill_vec%60+borrow+its+argument+instead+of+taking+ownership+of+it%2C%0A%2F%2F+and+then+copy+the+data+within+the+function+in+order+to+return+an+owned%0A%2F%2F+%60Vec%3Ci32%3E%60%0A%2F%2F+3.+Make+%60fill_vec%60+*mutably*+borrow+its+argument+%28which+will+need+to+be%0A%2F%2F+mutable%29%2C+modify+it+directly%2C+then+not+return+anything.+Then+you+can+get+rid%0A%2F%2F+of+%60vec1%60+entirely+--+note+that+this+will+change+what+gets+printed+by+the%0A%2F%2F+first+%60println%21%60%0A)
- ["move_semantics3.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile+without+adding+new+lines--+just+changing+existing+lines%21%0A%2F%2F+%28no+lines+with+multiple+semicolons+necessary%21%29%0A%2F%2F+Scroll+down+for+hints+%3A%29%0A%0Apub+fn+main%28%29+%7B%0A++++let+vec0+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++let+mut+vec1+%3D+fill_vec%28vec0%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A++++vec1.push%2888%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A%7D%0A%0Afn+fill_vec%28vec%3A+Vec%3Ci32%3E%29+-%3E+Vec%3Ci32%3E+%7B%0A++++vec.push%2822%29%3B%0A++++vec.push%2844%29%3B%0A++++vec.push%2866%29%3B%0A%0A++++vec%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+The+difference+between+this+one+and+the+previous+ones+is+that+the+first+line%0A%2F%2F+of+%60fn+fill_vec%60+that+had+%60let+mut+vec+%3D+vec%3B%60+is+no+longer+there.+You+can%2C%0A%2F%2F+instead+of+adding+that+line+back%2C+add+%60mut%60+in+one+place+that+will+change%0A%2F%2F+an+existing+binding+to+be+a+mutable+binding+instead+of+an+immutable+one+%3A%29%0A)
- ["move_semantics4.rs"](http://play.rust-lang.org/?code=%2F%2F+Refactor+this+code+so+that+instead+of+having+%60vec0%60+and+creating+the+vector%0A%2F%2F+in+%60fn+main%60%2C+we+instead+create+it+within+%60fn+fill_vec%60+and+transfer+the%0A%2F%2F+freshly+created+vector+from+fill_vec+to+its+caller.+Scroll+for+hints%21%0A%0Apub+fn+main%28%29+%7B%0A++++let+vec0+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++let+mut+vec1+%3D+fill_vec%28vec0%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A++++vec1.push%2888%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A%7D%0A%0Afn+fill_vec%28vec%3A+Vec%3Ci32%3E%29+-%3E+Vec%3Ci32%3E+%7B%0A++++let+mut+vec+%3D+vec%3B%0A%0A++++vec.push%2822%29%3B%0A++++vec.push%2844%29%3B%0A++++vec.push%2866%29%3B%0A%0A++++vec%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Stop+reading+whenever+you+feel+like+you+have+enough+direction+%3A%29+Or+try%0A%2F%2F+doing+one+step+and+then+fixing+the+compiler+errors+that+result%21%0A%2F%2F+So+the+end+goal+is+to%3A%0A%2F%2F+-+get+rid+of+the+first+line+in+main+that+creates+the+new+vector%0A%2F%2F+-+so+then+%60vec0%60+doesn%27t+exist%2C+so+we+can%27t+pass+it+to+%60fill_vec%60%0A%2F%2F+-+we+don%27t+want+to+pass+anything+to+%60fill_vec%60%2C+so+its+signature+should%0A%2F%2F+++reflect+that+it+does+not+take+any+arguments%0A%2F%2F+-+since+we%27re+not+creating+a+new+vec+in+%60main%60+anymore%2C+we+need+to+create%0A%2F%2F+++a+new+vec+in+%60fill_vec%60%2C+similarly+to+the+way+we+did+in+%60main%60%0A)

### Modules

[Relevant chapter in The Rust Programming Language](https://doc.rust-lang.org/stable/book/crates-and-modules.html)

- ["modules1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Amod+sausage_factory+%7B%0A++++fn+make_sausage%28%29+%7B%0A++++++++println%21%28%22sausage%21%22%29%3B%0A++++%7D%0A%7D%0A%0Afn+main%28%29+%7B%0A++++sausage_factory%3A%3Amake_sausage%28%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Everything+is+private+in+Rust+by+default--+but+there%27s+a+keyword+we+can+use%0A%2F%2F+to+make+something+public%21+The+compiler+error+should+point+to+the+thing+that%0A%2F%2F+needs+to+be+public.%0A)
- ["modules2.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Amod+us_presidential_frontrunners+%7B%0A++++use+self%3A%3Ademocrats%3A%3AHILLARY_CLINTON+as+democrat%3B%0A++++use+self%3A%3Arepublicans%3A%3ADONALD_TRUMP+as+republican%3B%0A%0A++++mod+democrats+%7B%0A++++++++pub+const+HILLARY_CLINTON%3A+%26%27static+str+%3D+%22Hillary+Clinton%22%3B%0A++++++++pub+const+BERNIE_SANDERS%3A+%26%27static+str+%3D+%22Bernie+Sanders%22%3B%0A++++%7D%0A%0A++++mod+republicans+%7B%0A++++++++pub+const+DONALD_TRUMP%3A+%26%27static+str+%3D+%22Donald+Trump%22%3B%0A++++++++pub+const+JEB_BUSH%3A+%26%27static+str+%3D+%22Jeb+Bush%22%3B%0A++++%7D%0A%7D%0A%0Afn+main%28%29+%7B%0A++++println%21%28%22candidates%3A+%7B%7D+and+%7B%7D%22%2C%0A+++++++++++++us_presidential_frontrunners%3A%3Ademocrat%2C%0A+++++++++++++us_presidential_frontrunners%3A%3Arepublican%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+The+us_presidential_frontrunners+module+is+trying+to+present+an+external%0A%2F%2F+interface+%28the+%60democrat%60+and+%60republican%60+constants%29+that+is+different+than%0A%2F%2F+its+internal+structure+%28the+%60democrats%60+and+%60republicans%60+modules+and%0A%2F%2F+associated+constants%29.+It%27s+almost+there+except+for+one+keyword+missing+for%0A%2F%2F+each+constant.%0A%2F%2F+One+more+hint%3A+I+wish+the+compiler+error%2C+instead+of+saying+%22unresolved+name%0A%2F%2F+%60us_presidential_frontrunners%3A%3Ademocrat%60%22%2C+could+say++%22constant%0A%2F%2F+%60us_presidential_frontrunners%3A%3Ademocrat%60+is+private%22%21)

### Error Handling

The [Error Handling](https://doc.rust-lang.org/stable/book/error-handling.html) and [Generics](https://doc.rust-lang.org/stable/book/generics.html) sections are relevant.

- ["result1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+this+test+pass%21+Scroll+down+for+hints+%3A%29%0A%0A%23%5Bderive%28PartialEq%2CDebug%29%5D%0Astruct+PositiveNonzeroInteger%28u64%29%3B%0A%0A%23%5Bderive%28PartialEq%2CDebug%29%5D%0Aenum+CreationError+%7B%0A++++Negative%2C%0A++++Zero%2C%0A%7D%0A%0Aimpl+PositiveNonzeroInteger+%7B%0A++++fn+new%28value%3A+i64%29+-%3E+Result%3CPositiveNonzeroInteger%2C+CreationError%3E+%7B%0A++++++++Ok%28PositiveNonzeroInteger%28value+as+u64%29%29%0A++++%7D%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_creation%28%29+%7B%0A++++assert%21%28PositiveNonzeroInteger%3A%3Anew%2810%29.is_ok%28%29%29%3B%0A++++assert_eq%21%28Err%28CreationError%3A%3ANegative%29%2C+PositiveNonzeroInteger%3A%3Anew%28-10%29%29%3B%0A++++assert_eq%21%28Err%28CreationError%3A%3AZero%29%2C+PositiveNonzeroInteger%3A%3Anew%280%29%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+%60PositiveNonzeroInteger%3A%3Anew%60+is+always+creating+a+new+instance+and+returning+an+%60Ok%60+result.%0A%2F%2F+It+should+be+doing+some+checking%2C+returning+an+%60Err%60+result+if+those+checks+fail%2C+and+only%0A%2F%2F+returning+an+%60Ok%60+result+if+those+checks+determine+that+everything+is...+okay+%3A%29%0A)

### Standard library types

#### `Arc`

The [Concurrency](https://doc.rust-lang.org/stable/book/concurrency.html) section is relevant.

- ["arc1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+this+code+compile+by+filling+in+a+value+for+%60shared_numbers%60+where+the%0A%2F%2F+TODO+comment+is+and+creating+an+initial+binding+for+%60child_numbers%60%0A%2F%2F+somewhere.+Try+not+to+create+any+copies+of+the+%60numbers%60+Vec%21%0A%2F%2F+Scroll+down+for+hints+%3A%29%0A%0Ause+std%3A%3Async%3A%3AArc%3B%0Ause+std%3A%3Athread%3B%0A%0Afn+main%28%29+%7B%0A++++let+numbers%3A+Vec%3C_%3E+%3D+%280..100u32%29.collect%28%29%3B%0A++++let+shared_numbers+%3D+%2F%2F+TODO%0A++++let+mut+joinhandles+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++for+offset+in+0..8+%7B%0A++++++++joinhandles.push%28%0A++++++++thread%3A%3Aspawn%28move+%7C%7C+%7B%0A++++++++++++let+mut+i+%3D+offset%3B%0A++++++++++++let+mut+sum+%3D+0%3B%0A++++++++++++while+i+%3C+child_numbers.len%28%29+%7B%0A++++++++++++++++sum+%2B%3D+child_numbers%5Bi%5D%3B%0A++++++++++++++++i+%2B%3D+5%3B%0A++++++++++++%7D%0A++++++++++++println%21%28%22Sum+of+offset+%7B%7D+is+%7B%7D%22%2C+offset%2C+sum%29%3B%0A++++++++%7D%29%29%3B%0A++++%7D%0A++++for+handle+in+joinhandles.into_iter%28%29+%7B%0A++++++++handle.join%28%29.unwrap%28%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Make+%60shared_numbers%60+be+an+%60Arc%60+from+the+numbers+vector.+Then%2C+in+order%0A%2F%2F+to+avoid+creating+a+copy+of+%60numbers%60%2C+you%27ll+need+to+create+%60child_numbers%60%0A%2F%2F+inside+the+loop+but+still+in+the+main+thread.%0A%0A%2F%2F+%60child_numbers%60+should+be+a+clone+of+the+Arc+of+the+numbers+instead+of+a%0A%2F%2F+thread-local+copy+of+the+numbers.%0A)

### Threads

See [the Dining Philosophers example](https://doc.rust-lang.org/stable/book/dining-philosophers.html) and the [Concurrency Chapter](https://doc.rust-lang.org/stable/book/concurrency.html) from the book.

- ["threads1.rs"](<https://play.rust-lang.org/?code=%2F%2F%20Make%20this%20compile!%20Scroll%20down%20for%20hints%20%3A\)%20The%20idea%20is%20the%20thread%0A%2F%2F%20spawned%20on%20line%2017%20is%20completing%20jobs%20while%20the%20main%20thread%20is%0A%2F%2F%20monitoring%20progress%20until%2010%20jobs%20are%20completed.%20If%20you%20see%206%20lines%0A%2F%2F%20of%20%22waiting...%22%20and%20the%20program%20ends%20without%20timing%20out%20the%20playground%2C%0A%2F%2F%20you%27ve%20got%20it%20%3A\)%0A%0Ause%20std%3A%3Async%3A%3AArc%3B%0Ause%20std%3A%3Athread%3B%0Ause%20std%3A%3Atime%3A%3ADuration%3B%0A%0Astruct%20JobStatus%20%7B%0A%20%20%20%20jobs_completed%3A%20u32%2C%0A%7D%0A%0Afn%20main\(\)%20%7B%0A%20%20%20%20let%20status%20%3D%20Arc%3A%3Anew\(JobStatus%20%7B%20jobs_completed%3A%200%20%7D\)%3B%0A%20%20%20%20let%20status_shared%20%3D%20status.clone\(\)%3B%0A%20%20%20%20thread%3A%3Aspawn\(move%20%7C%7C%20%7B%0A%20%20%20%20%20%20%20%20for%20_%20in%200..10%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20thread%3A%3Asleep\(Duration%3A%3Afrom_millis\(250\)\)%3B%0A%20%20%20%20%20%20%20%20%20%20%20%20status_shared.jobs_completed%20%2B%3D%201%3B%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D\)%3B%0A%20%20%20%20while%20status.jobs_completed%20%3C%2010%20%7B%0A%20%20%20%20%20%20%20%20println!\(%22waiting...%20%22\)%3B%0A%20%20%20%20%20%20%20%20thread%3A%3Asleep\(Duration%3A%3Afrom_millis\(500\)\)%3B%0A%20%20%20%20%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F%20%60Arc%60%20is%20an%20Atomic%20Reference%20Counted%20pointer%20that%20allows%20safe%2C%20shared%20access%0A%2F%2F%20to%20**immutable**%20data.%20But%20we%20want%20to%20*change*%20the%20number%20of%20%60jobs_completed%60%0A%2F%2F%20so%20we%27ll%20need%20to%20also%20use%20another%20type%20that%20will%20only%20allow%20one%20thread%20to%0A%2F%2F%20mutate%20the%20data%20at%20a%20time.%20Take%20a%20look%20at%20this%20section%20of%20the%20book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fstable%2Fbook%2Fconcurrency.html%23safe-shared-mutable-state%0A%2F%2F%20and%20keep%20scrolling%20if%20you%27d%20like%20more%20hints%20%3A\)%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F%20Do%20you%20now%20have%20an%20%60Arc%60%20%60Mutex%60%20%60JobStatus%60%20at%20the%20beginning%20of%20main%3F%20Like%3A%0A%2F%2F%20%60let%20status%20%3D%20Arc%3A%3Anew\(Mutex%3A%3Anew\(JobStatus%20%7B%20jobs_completed%3A%200%20%7D\)\)%3B%60%0A%2F%2F%20Similar%20to%20the%20code%20in%20the%20example%20in%20the%20book%20that%20happens%20after%20the%20text%0A%2F%2F%20that%20says%20%22We%20can%20use%20Arc%3CT%3E%20to%20fix%20this.%22.%20If%20not%2C%20give%20that%20a%20try!%20If%20you%0A%2F%2F%20do%20and%20would%20like%20more%20hints%2C%20keep%20scrolling!!%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F%20Make%20sure%20neither%20of%20your%20threads%20are%20holding%20onto%20the%20lock%20of%20the%20mutex%0A%2F%2F%20while%20they%20are%20sleeping%2C%20since%20this%20will%20prevent%20the%20other%20thread%20from%0A%2F%2F%20being%20allowed%20to%20get%20the%20lock.%20Locks%20are%20automatically%20released%20when%0A%2F%2F%20they%20go%20out%20of%20scope.%0A%0A%2F%2F%20Ok%2C%20so%2C%20real%20talk%2C%20this%20was%20actually%20tricky%20for%20*me*%20to%20do%20too.%20And%0A%2F%2F%20I%20could%20see%20a%20lot%20of%20different%20problems%20you%20might%20run%20into%2C%20so%20at%20this%0A%2F%2F%20point%20I%27m%20not%20sure%20which%20one%20you%27ve%20hit%20%3A\)%20Please%20see%20a%20few%20possible%0A%2F%2F%20answers%20on%20https%3A%2F%2Fgithub.com%2Fcarols10cents%2Frustlings%2Fissues%2F3%20--%0A%2F%2F%20mine%20is%20a%20little%20more%20complicated%20because%20I%20decided%20I%20wanted%20to%20see%0A%2F%2F%20the%20number%20of%20jobs%20currently%20done%20when%20I%20was%20checking%20the%20status.%0A%0A%2F%2F%20Please%20open%20an%20issue%20if%20you%27re%20still%20running%20into%20a%20problem%20that%0A%2F%2F%20these%20hints%20are%20not%20helping%20you%20with%2C%20or%20if%20you%27ve%20looked%20at%20the%20sample%0A%2F%2F%20answers%20and%20don%27t%20understand%20why%20they%20work%20and%20yours%20doesn%27t.%0A%0A%2F%2F%20If%20you%27ve%20learned%20from%20the%20sample%20solutions%2C%20I%20encourage%20you%20to%20come%0A%2F%2F%20back%20to%20this%20exercise%20and%20try%20it%20again%20in%20a%20few%20days%20to%20reinforce%0A%2F%2F%20what%20you%27ve%20learned%20%3A\)%0A&version=stable>)

### Uncategorized

A few exercises based on things I've encountered or had trouble with getting used to.

- ["ex1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21%0A%0Afn+main%28%29+%7B%0A++++println%21%28%29%3B%0A%7D%0A)
- ["ex2.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21%0A%0Afn+something%28%29+-%3E+String+%7B%0A++++%22hi%21%22%0A%7D%0A%0Afn+main%28%29+%7B%0A++++println%21%28%22%7B%7D%22%2C+something%28%29%29%3B%0A%7D%0A)
- ["ex3.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21%0A%0Astruct+Foo+%7B%0A++++capacity%3A+i32%2C%0A%7D%0A%0Afn+main%28%29+%7B%0A++++println%21%28%22%7B%3A%3F%7D%22%2C+Foo+%7B+capacity%3A+3+%7D%29%3B%0A%7D%0A)
- ["ex4.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21%0A%0Afn+something%28%29+-%3E+Result%3Ci32%2C+std%3A%3Anum%3A%3AParseIntError%3E+%7B%0A++++let+x%3Ai32+%3D+%223%22.parse%28%29%3B%0A++++Ok%28x+*+4%29%0A%7D%0A%0Afn+main%28%29+%7B%0A++++match+something%28%29+%7B%0A++++++++Ok%28..%29+%3D%3E+println%21%28%22You+win%21%22%29%2C%0A++++++++Err%28e%29+%3D%3E+println%21%28%22Oh+no+something+went+wrong%3A+%7B%7D%22%2C+e%29%2C%0A++++%7D%0A%7D%0A)
- ["ex5.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21%0A%0Aenum+Reaction%3C%27a%3E+%7B%0A++++Sad%28%26%27a+str%29%2C%0A++++Happy%28%26%27a+str%29%2C%0A%7D%0A%0Afn+express%28sentiment%3A+Reaction%29+%7B%0A++++match+sentiment+%7B%0A++++++++Reaction%3A%3ASad%28s%29+%3D%3E+println%21%28%22%3A%28+%7B%7D%22%2C+s%29%2C%0A++++++++Reaction%3A%3AHappy%28s%29+%3D%3E+println%21%28%22%3A%29+%7B%7D%22%2C+s%29%2C%0A++++%7D%0A%7D%0A%0Afn+main+%28%29+%7B%0A++++let+x+%3D+Reaction%3A%3AHappy%28%22It%27s+a+great+day+for+Rust%21%22%29%3B%0A++++express%28x%29%3B%0A++++express%28x%29%3B%0A++++let+y+%3D+Reaction%3A%3ASad%28%22This+code+doesn%27t+compile+yet.%22%29%3B%0A++++express%28y%29%3B%0A%7D%0A)

## To help with this repo/TODO list

* File issues for problems or suggestions!
* Contribute more exercises! Anything that took you time to get used to, or that you had trouble with, or that deserves practice would be a good exercise!
* How could the process of doing these exercises work better? This is an open-ended question :) Are the playground links good enough? Are there ways that we could make going to the next exercise easier without forking the playground??
