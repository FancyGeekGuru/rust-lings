# rustlings

Small exercises to get you used to reading and writing Rust code. Includes practice reading and responding to
compiler messages!

This repo is very much the smallest thing that could possibly work :)

## To do these exercises

Thanks to [btbytes'](https://twitter.com/btbytes) [prlinks](https://github.com/btbytes/prlink), you can now click on the links below to load the exercises in the rust playground!

There are infinite correct answers-- the exercises are sometimes left very open-ended. Scroll down in the playground to find comments that have hints.

If you need more help or would like to compare solutions, you can ask in [#rust on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust), the [user forum](https://users.rust-lang.org/), or [the subreddit](https://reddit.com/r/rust). If an exercise could be improved in any way, please [create an issue](https://github.com/carols10cents/rustlings/issues/new) or submit a pull request!

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

### Tests

Going out of order from the Syntax and Semantics section of the book to cover tests-- many of the
following exercises will ask you to make tests pass!

[Testing chapter from the Effective Rust section of the book](https://doc.rust-lang.org/stable/book/testing.html)

- ["tests1.rs"](http://play.rust-lang.org/?code=%2F%2F+This+test+has+a+problem+with+it+--+make+the+test+compile%21+Make+the+test%0A%2F%2F+pass%21+Make+the+test+fail%21+Scroll+down+for+hints+%3A%29%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++%23%5Btest%5D%0A++++fn+you_can_assert%28%29+%7B%0A++++++++assert%21%28%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+You+don%27t+even+need+to+write+any+code+to+test+--+you+can+just+test+values+and+run+that%2C+even%0A%2F%2F+though+you+wouldn%27t+do+that+in+real+life+%3A%29+%60assert%21%60+is+a+macro+that+needs+an+argument.%0A%2F%2F+Depending+on+the+value+of+the+argument%2C+%60assert%21%60+will+do+nothing+%28in+which+case+the+test+will%0A%2F%2F+pass%29+or+%60assert%21%60+will+panic+%28in+which+case+the+test+will+fail%29.+So+try+giving+different+values%0A%2F%2F+to+%60assert%21%60+and+see+which+ones+compile%2C+which+ones+pass%2C+and+which+ones+fail+%3A%29%0A)
- ["tests2.rs"](http://play.rust-lang.org/?code=%2F%2F+This+test+has+a+problem+with+it+--+make+the+test+compile%21+Make+the+test%0A%2F%2F+pass%21+Make+the+test+fail%21+Scroll+down+for+hints+%3A%29%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++%23%5Btest%5D%0A++++fn+you_can_assert_eq%28%29+%7B%0A++++++++assert_eq%21%28%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Like+the+previous+exercise%2C+you+don%27t+need+to+write+any+code+to+get+this+test+to+compile+and%0A%2F%2F+run.+%60assert_eq%21%60+is+a+macro+that+takes+two+arguments+and+compares+them.+Try+giving+it+two%0A%2F%2F+values+that+are+equal%21+Try+giving+it+two+arguments+that+are+different%21+Try+giving+it+two+values%0A%2F%2F+that+are+of+different+types%21+Try+switching+which+argument+comes+first+and+which+comes+second%21%0A)
- ["tests3.rs"](http://play.rust-lang.org/?code=%2F%2F+This+test+isn%27t+testing+our+function+--+make+it+do+that+in+such+a+way+that%0A%2F%2F+the+test+passes.+Then+write+a+second+test+that+tests+that+we+get+the+result%0A%2F%2F+we+expect+to+get+when+we+call+%60is_even%285%29%60.+Scroll+down+for+hints%21%0A%0Apub+fn+is_even%28num%3A+i32%29+-%3E+bool+%7B%0A++++num+%25+2+%3D%3D+0%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+is_true_when_even%28%29+%7B%0A++++++++assert%21%28false%29%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+You+can+call+a+function+right+where+you%27re+passing+arguments+to+%60assert%21%60+--+so+you+could+do%0A%2F%2F+something+like+%60assert%21%28having_fun%28%29%29%60.+If+you+want+to+check+that+you+indeed+get+false%2C+you%0A%2F%2F+can+negate+the+result+of+what+you%27re+doing+using+%60%21%60%2C+like+%60assert%21%28%21having_fun%28%29%29%60.%0A)
- ["tests4.rs"](http://play.rust-lang.org/?code=%2F%2F+This+test+isn%27t+testing+our+function+--+make+it+do+that+in+such+a+way+that%0A%2F%2F+the+test+passes.+Then+write+a+second+test+that+tests+that+we+get+the+result%0A%2F%2F+we+expect+to+get+when+we+call+%60times_two%60+with+a+negative+number.%0A%2F%2F+No+hints%2C+you+can+do+this+%3A%29%0A%0Apub+fn+times_two%28num%3A+i32%29+-%3E+i32+%7B%0A++++num+*+2%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+returns_twice_of_positive_numbers%28%29+%7B%0A++++++++assert_eq%21%284%2C+4%29%3B%0A++++%7D%0A%7D%0A)

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

Note that they may look similar but they are subtly different :)

- ["move_semantics1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Apub+fn+main%28%29+%7B%0A++++let+vec0+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++let+vec1+%3D+fill_vec%28vec0%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A++++vec1.push%2888%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A%7D%0A%0Afn+fill_vec%28vec%3A+Vec%3Ci32%3E%29+-%3E+Vec%3Ci32%3E+%7B%0A++++let+mut+vec+%3D+vec%3B%0A%0A++++vec.push%2822%29%3B%0A++++vec.push%2844%29%3B%0A++++vec.push%2866%29%3B%0A%0A++++vec%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+So+you%27ve+got+the+%22cannot+borrow+immutable+local+variable+%60vec1%60+as+mutable%22+error+on+line+8%2C%0A%2F%2F+right%3F+The+fix+for+this+is+going+to+be+adding+one+keyword%2C+and+the+addition+is+NOT+on+line+8%0A%2F%2F+where+the+error+is.%0A)
- ["move_semantics2.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile+without+changing+line+9%21+Scroll+down+for+hints+%3A%29%0A%0Apub+fn+main%28%29+%7B%0A++++let+vec0+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++let+mut+vec1+%3D+fill_vec%28vec0%29%3B%0A%0A++++%2F%2F+Do+not+change+the+following+line%21%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec0%22%2C+vec0.len%28%29%2C+vec0%29%3B%0A%0A++++vec1.push%2888%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A%7D%0A%0Afn+fill_vec%28vec%3A+Vec%3Ci32%3E%29+-%3E+Vec%3Ci32%3E+%7B%0A++++let+mut+vec+%3D+vec%3B%0A%0A++++vec.push%2822%29%3B%0A++++vec.push%2844%29%3B%0A++++vec.push%2866%29%3B%0A%0A++++vec%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+So+%60vec0%60+is+being+*moved*+into+the+function+%60fill_vec%60+when+we+call+it+on%0A%2F%2F+line+6%2C+which+means+it+gets+dropped+at+the+end+of+%60fill_vec%60%2C+which+means+we%0A%2F%2F+can%27t+use+%60vec0%60+again+on+line+9+%28or+anywhere+else+in+%60main%60+after+the%0A%2F%2F+%60fill_vec%60+call+for+that+matter%29.+We+could+fix+this+in+a+few+ways%2C+try+them%0A%2F%2F+all%21%0A%2F%2F+1.+Make+another%2C+separate+version+of+the+data+that%27s+in+%60vec0%60+and+pass+that%0A%2F%2F+to+%60fill_vec%60+instead.%0A%2F%2F+2.+Make+%60fill_vec%60+borrow+its+argument+instead+of+taking+ownership+of+it%2C%0A%2F%2F+and+then+copy+the+data+within+the+function+in+order+to+return+an+owned%0A%2F%2F+%60Vec%3Ci32%3E%60%0A%2F%2F+3.+Make+%60fill_vec%60+*mutably*+borrow+its+argument+%28which+will+need+to+be%0A%2F%2F+mutable%29%2C+modify+it+directly%2C+then+not+return+anything.+Then+you+can+get+rid%0A%2F%2F+of+%60vec1%60+entirely+--+note+that+this+will+change+what+gets+printed+by+the%0A%2F%2F+first+%60println%21%60%0A)
- ["move_semantics3.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile+without+adding+new+lines--+just+changing+existing+lines%21%0A%2F%2F+%28no+lines+with+multiple+semicolons+necessary%21%29%0A%2F%2F+Scroll+down+for+hints+%3A%29%0A%0Apub+fn+main%28%29+%7B%0A++++let+vec0+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++let+mut+vec1+%3D+fill_vec%28vec0%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A++++vec1.push%2888%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A%7D%0A%0Afn+fill_vec%28vec%3A+Vec%3Ci32%3E%29+-%3E+Vec%3Ci32%3E+%7B%0A++++vec.push%2822%29%3B%0A++++vec.push%2844%29%3B%0A++++vec.push%2866%29%3B%0A%0A++++vec%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+The+difference+between+this+one+and+the+previous+ones+is+that+the+first+line%0A%2F%2F+of+%60fn+fill_vec%60+that+had+%60let+mut+vec+%3D+vec%3B%60+is+no+longer+there.+You+can%2C%0A%2F%2F+instead+of+adding+that+line+back%2C+add+%60mut%60+in+one+place+that+will+change%0A%2F%2F+an+existing+binding+to+be+a+mutable+binding+instead+of+an+immutable+one+%3A%29%0A)
- ["move_semantics4.rs"](http://play.rust-lang.org/?code=%2F%2F+Refactor+this+code+so+that+instead+of+having+%60vec0%60+and+creating+the+vector%0A%2F%2F+in+%60fn+main%60%2C+we+instead+create+it+within+%60fn+fill_vec%60+and+transfer+the%0A%2F%2F+freshly+created+vector+from+fill_vec+to+its+caller.+Scroll+for+hints%21%0A%0Apub+fn+main%28%29+%7B%0A++++let+vec0+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++let+mut+vec1+%3D+fill_vec%28vec0%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A++++vec1.push%2888%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A%7D%0A%0Afn+fill_vec%28vec%3A+Vec%3Ci32%3E%29+-%3E+Vec%3Ci32%3E+%7B%0A++++let+mut+vec+%3D+vec%3B%0A%0A++++vec.push%2822%29%3B%0A++++vec.push%2844%29%3B%0A++++vec.push%2866%29%3B%0A%0A++++vec%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Stop+reading+whenever+you+feel+like+you+have+enough+direction+%3A%29+Or+try%0A%2F%2F+doing+one+step+and+then+fixing+the+compiler+errors+that+result%21%0A%2F%2F+So+the+end+goal+is+to%3A%0A%2F%2F+-+get+rid+of+the+first+line+in+main+that+creates+the+new+vector%0A%2F%2F+-+so+then+%60vec0%60+doesn%27t+exist%2C+so+we+can%27t+pass+it+to+%60fill_vec%60%0A%2F%2F+-+we+don%27t+want+to+pass+anything+to+%60fill_vec%60%2C+so+its+signature+should%0A%2F%2F+++reflect+that+it+does+not+take+any+arguments%0A%2F%2F+-+since+we%27re+not+creating+a+new+vec+in+%60main%60+anymore%2C+we+need+to+create%0A%2F%2F+++a+new+vec+in+%60fill_vec%60%2C+similarly+to+the+way+we+did+in+%60main%60%0A)

### Modules

[Relevant chapter in The Rust Programming Language](https://doc.rust-lang.org/stable/book/crates-and-modules.html)

- ["modules1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Amod+sausage_factory+%7B%0A++++fn+make_sausage%28%29+%7B%0A++++++++println%21%28%22sausage%21%22%29%3B%0A++++%7D%0A%7D%0A%0Afn+main%28%29+%7B%0A++++sausage_factory%3A%3Amake_sausage%28%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Everything+is+private+in+Rust+by+default--+but+there%27s+a+keyword+we+can+use%0A%2F%2F+to+make+something+public%21+The+compiler+error+should+point+to+the+thing+that%0A%2F%2F+needs+to+be+public.%0A)
- ["modules2.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+me+compile%21+Scroll+down+for+hints+%3A%29%0A%0Amod+us_presidential_frontrunners+%7B%0A++++use+self%3A%3Ademocrats%3A%3AHILLARY_CLINTON+as+democrat%3B%0A++++use+self%3A%3Arepublicans%3A%3ADONALD_TRUMP+as+republican%3B%0A%0A++++mod+democrats+%7B%0A++++++++pub+const+HILLARY_CLINTON%3A+%26%27static+str+%3D+%22Hillary+Clinton%22%3B%0A++++++++pub+const+BERNIE_SANDERS%3A+%26%27static+str+%3D+%22Bernie+Sanders%22%3B%0A++++%7D%0A%0A++++mod+republicans+%7B%0A++++++++pub+const+DONALD_TRUMP%3A+%26%27static+str+%3D+%22Donald+Trump%22%3B%0A++++++++pub+const+JEB_BUSH%3A+%26%27static+str+%3D+%22Jeb+Bush%22%3B%0A++++%7D%0A%7D%0A%0Afn+main%28%29+%7B%0A++++println%21%28%22candidates%3A+%7B%7D+and+%7B%7D%22%2C%0A+++++++++++++us_presidential_frontrunners%3A%3Ademocrat%2C%0A+++++++++++++us_presidential_frontrunners%3A%3Arepublican%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+The+us_presidential_frontrunners+module+is+trying+to+present+an+external%0A%2F%2F+interface+%28the+%60democrat%60+and+%60republican%60+constants%29+that+is+different+than%0A%2F%2F+its+internal+structure+%28the+%60democrats%60+and+%60republicans%60+modules+and%0A%2F%2F+associated+constants%29.+It%27s+almost+there+except+for+one+keyword+missing+for%0A%2F%2F+each+constant.%0A%2F%2F+One+more+hint%3A+I+wish+the+compiler+error%2C+instead+of+saying+%22unresolved+name%0A%2F%2F+%60us_presidential_frontrunners%3A%3Ademocrat%60%22%2C+could+say++%22constant%0A%2F%2F+%60us_presidential_frontrunners%3A%3Ademocrat%60+is+private%22%21)

### Standard library types

#### `Result`

The [Error Handling](https://doc.rust-lang.org/stable/book/error-handling.html) and [Generics](https://doc.rust-lang.org/stable/book/generics.html) sections are relevant.

- ["result1.rs"](http://play.rust-lang.org/?code=%2F%2F+Make+this+test+pass%21+Scroll+down+for+hints+%3A%29%0A%0A%23%5Bderive%28PartialEq%2CDebug%29%5D%0Astruct+PositiveNonzeroInteger%28u64%29%3B%0A%0A%23%5Bderive%28PartialEq%2CDebug%29%5D%0Aenum+CreationError+%7B%0A++++Negative%2C%0A++++Zero%2C%0A%7D%0A%0Aimpl+PositiveNonzeroInteger+%7B%0A++++fn+new%28value%3A+i64%29+-%3E+Result%3CPositiveNonzeroInteger%2C+CreationError%3E+%7B%0A++++++++Ok%28PositiveNonzeroInteger%28value+as+u64%29%29%0A++++%7D%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_creation%28%29+%7B%0A++++assert%21%28PositiveNonzeroInteger%3A%3Anew%2810%29.is_ok%28%29%29%3B%0A++++assert_eq%21%28Err%28CreationError%3A%3ANegative%29%2C+PositiveNonzeroInteger%3A%3Anew%28-10%29%29%3B%0A++++assert_eq%21%28Err%28CreationError%3A%3AZero%29%2C+PositiveNonzeroInteger%3A%3Anew%280%29%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+%60PositiveNonzeroInteger%3A%3Anew%60+is+always+creating+a+new+instance+and+returning+an+%60Ok%60+result.%0A%2F%2F+It+should+be+doing+some+checking%2C+returning+an+%60Err%60+result+if+those+checks+fail%2C+and+only%0A%2F%2F+returning+an+%60Ok%60+result+if+those+checks+determine+that+everything+is...+okay+%3A%29%0A)

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
* How could the process of doing these exercises work better? This is an open-ended question :) Are the playground links good enough? Are there ways that we could make going to the next exercise easier without forking the playgound??
