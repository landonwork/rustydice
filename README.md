# rustydice
## A server for doing absurdly complicated things with dice

Welcome, stranger!
This is a personal project to help me learn to do backend stuff
in Rust. More specifically, asynchronous programming and using
tokio. Not that you care.

To use the site, you can type the URLs straight into you browser
or you can send a get request if you're a `curl` kind of person.
Add a path and a query to the domain in order to do different
things.

- rustydice.tk/roll/?dice=2d6+1d4
This will generate outcomes from fair dice. If there is no
dice query parameter, it will default to a d6. Sides of
dice must be between 1 and 255. Add as many dice as you
like with a `+`!

- rustydice.tk/probability/?dice=2d6+1d4
This will give you the number of possible combinations for every
possible sum of the dice provided. However, there is a maximum
limit of 50 dice.
