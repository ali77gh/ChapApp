pub fn get_default() -> &'static str {
    HELLO_WORLD
}

pub fn get_by_name(name: &str) -> &'static str {
    match name {
        "hello_world" => HELLO_WORLD,
        "count_down" => COUNT_DOWN,
        "is_prime" => IS_PRIME,
        "christmas_tree" => CHRISTMAS_TREE,
        "better_tree" => CHRISTMAS_TREE_WITH_TRUNK,
        "say_my_name" => SAY_MY_NAME,
        _ => "Example not found",
    }
}

pub const HELLO_WORLD: &str = r#"// Editable

"Hello World!"
"#;

pub const COUNT_DOWN: &str = r#"// Editable

10 -> $counter
@l
    $counter -> decrease
    $counter
@l, $counter, 0 -> jump_if_not_equal
"#;

pub const IS_PRIME: &str = r#"// Editable

100 -> $limit // change this

1 -> $i
0 -> $c
@loop

    @if1, ($i, 2 -> lt) -> jump_if_not
        @is -> jump
    @if1

    $i -> sqrt -> to int -> $end
    $end -> increase

    2 -> $inner_i
    @inner_loop
        @is_not, ($i, $inner_i -> mod), 0 -> jeq
        $inner_i -> increase
    @inner_loop, $inner_i, $end -> jneq

    @is
    $c -> increase
    $i
    @is_not
    $i -> increase
@loop, $i, $limit -> jneq

""
"We have ", $c," prime numbers from 1 to ", $limit -> cat

"#;

pub const CHRISTMAS_TREE: &str = r#"// Editable

0 -> $counter
@loop
    $counter -> increase

    (" ", (10, $counter -> minus) -> repeat),
    ("*", ($counter, 2 -> multiply) -> repeat) -> 
    cat
@loop, $counter, 10 -> jump if not equal

"#;

pub const SAY_MY_NAME: &str = r#"// Editable

input-> $name

"Hello ", $name -> cat
"#;

pub const CHRISTMAS_TREE_WITH_TRUNK: &str = r#"// Editable

0 -> $counter
@loop
    $counter -> increase

    $counter, 1 -> multiply -> $stars_size
    19, $counter -> minus -> $space_size

    " * ", $stars_size -> repeat -> $stars
    " ", $space_size -> repeat -> $spaces

    $spaces, $stars -> cat

    "`*-", $stars_size -> repeat -> $stars
    " ", $space_size -> repeat -> $spaces

    $spaces, $stars -> cat
@loop, $counter, 10 -> jump if not equal

3 -> $c
@loop
    $c-> increase

    $c, 2 -> multiply -> $stars_size
    22, $c-> minus -> $space_size


    "*", $stars_size -> repeat -> $stars
    " ", $space_size -> repeat -> $spaces

    $spaces, $stars -> cat

@loop, $c, 7 -> jump if not equal
"#;
