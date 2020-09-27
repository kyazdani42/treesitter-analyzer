# Treesitter based project analyzing tool

## Notice

- This is very very much early stage

## Why

[lsp](https://microsoft.github.io/language-server-protocol/) has become a standard in modern text editing.
But it's a hassle to manage lsp's for each language, sometimes they are broken, or very hard to configure.
When you use a good lsp, all the features are working, and you get used to it.
But when you switch language, suddenly, the lsp for the other language is not working properly, and you get frustrated because of it. 
It just kills your workflow and productivity. Also each lsp don't support the same toolset.

Treesitter is a parsing library. A very fast one.
It provides query based matching mecanism, which is a big plus for structural editing and highlighting.
It does not depend on huge project to run, but only small parsers, which can be written by basically anybody.
The goal of this project is to discover how far we can go with it.
I want features like go to definition and get references to be language agnostic.
I want to see if features like project based completion and diagnostics can be made with this.

Treesitter makes interpreting code in a language agnostic way very easy. We'll see how far it goes!
