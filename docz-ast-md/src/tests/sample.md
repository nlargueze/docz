---
title: My title
tags:
  - tag1
  - tag2
---

# H1 header

This is paragraph 1.

This is paragraph 2, with some text in _italic_ some in **bold**, some `code`, some ~~deleted text~~, and some ==highlighted text==, some ~subscript~, some ^superscript^.

## Latex 

An inline equation $\int_{-\infty}^\infty g(x) dx$

A block equation $$\int_{-\infty}^\infty g(x) dx$$

## List 

### Unordered list

- item 1
- item 2
- item 3

* item 1
* item 2
* item 3

### Ordered list

1. item 1
2. item 2
5. item 3

### Nested list

1. First item
2. Second item
3. Third item
    - Indented item
    - Indented item
4. Fourth item

## Task list

- [ ] Task 1
- [x] Task 2

## Codeblock

```rust
fn main() {
    println!("Hello, world!");
}
```

## Horizontal rule

---

## Link 

[My link](https://duckduckgo.com "My link title")

[My Link with ref][link_1]

[link_1]: https://en.wikipedia.org/wiki/Hobbit#Lifestyle "My link with ref"

## Image

![Tux, the Linux mascot](/assets/images/tux.png "My image title")

## HTML

<div>
    <p>HTML code</p>
</div>

## Emoji

:joy:

## Definition list

First Term
: This is the definition of the first term.

Second Term
: This is one definition of the second term.
: This is another definition of the second term.

## Footnote

Here's a simple footnote[^1] and here's a longer one[^bignote].

[^1]: This is the first footnote.

[^bignote]: Here's one with multiple paragraphs and code.

    Indent paragraphs to include them in the footnote.

    `{ my code }`

    Add as many paragraphs as you like.

## Blockquote

> Block line 1
> 
> Block line 2
>
>> Nested text
> 
> - item 1
> - item 2

## Table

| Name      | Description |
| ----------- | ----------- |
| Row 1      | Title       |
| Row 2   | Text        |