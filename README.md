# Elliptic Curve Point Order Finder in Finite Fields

This Rust program calculates the order of a given point on an elliptic curve over a finite field. The elliptic curve is defined by the equation:

$y^2 = x^3 + ax + b \mod p$

## Description
The program determines the order of a point on the elliptic curve modulo a prime `p`. It uses point addition and point doubling on the elliptic curve and leverages modular arithmetic. The code includes functions for adding two points on the curve and finding the modular inverse using the extended Euclidean algorithm.

### Features
- Calculate the order of a point on an elliptic curve.
- Supports point addition and point doubling on elliptic curves.
- Handles modular arithmetic and inverse calculations efficiently.

## Code Explanation
The main function initializes the curve parameters:
- Coefficient `a = 7`
- Coefficient `b = 1`
- Prime modulus `p = 313`
- A point `(x, y)` on the elliptic curve: `(310, 61)`

It then calculates the order of this point using the `find_order` function.

### Example Output
Point (310, 61) modulo 313 is of order 303

## Functions

- `find_order(g: (i64, i64), a: i64, b: i64, p: i64) -> usize`
Determines the order of a given point on the elliptic curve.

- `add_points(p1: (i64, i64), p2: (i64, i64), a: i64, p: i64) -> Option<(i64, i64)>`
Adds two points on the elliptic curve modulo `p`.

- `mod_inverse(a: i64, p: i64) -> Option<i64>`
Computes the modular inverse using the extended Euclidean algorithm.

## Requirements
- To get started, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine. You can then clone the repository and build the project.

## Installation

1. **Clone the repository**: Clone this repository or copy the code into a `.rs` file.
    ```bash
    git clone https://github.com/cypriansakwa/Elliptic_Curve_Point_Order_Finder_in_Finite_Fields.git
    cd Elliptic_Curve_Point_Order_Finder_in_Finite_Fields
    ```

2. **Build the project**:
    ```bash
    cargo build
    ```

3. **Run the program**:
    ```bash
    cargo run
    ```


## Dependencies
No external dependencies are required. The program uses only Rust's standard library.


