
fn fib(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn print_ingredients() {
    for ingredient in ["flour", "sugar", "eggs", "butter", "milk"] {
        println!("{}", ingredient);
    }
}

fn cumaltive_sum(n: i32) -> i32 {
    let mut i = 1;
    let mut sum = 0;
    while i <= n {
        sum += i;
        i += 1;
    }

    return sum;
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn collatz_length(mut n: i32) -> i32 {
    let mut length = 1;
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }
    return length;
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0i32; 3]; 3];

    for x in 0..3 {
        for y in 0..3 {

            println!("{}", matrix[x][y]);
            result[y][x] = matrix[x][y];
        }
    }

    return result;
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
    // println!("Hello, world!");
    // let mut x: i32 = 5;
    // x = 2;
    // println!("x = {}", x);
    // println!("sum = {}", sum(100, 200));
    // println!("fib = {}", fib(5));
    // println!("cumaltive_sum = {}", cumaltive_sum(5));
    // print_ingredients();
    // println!("collatz_length = {}", collatz_length(11));

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}

