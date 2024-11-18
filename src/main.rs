fn main() {
    let a = 7; // coefficient a
    let b = 1; // coefficient b
    let p = 313; // prime modulus

    let point = (310, 61); // Point on the curve

    // Find the order of the point
    let order = find_order(point, a, b, p);
    println!("Point {:?} modulo {} is of order {}", point, p, order);
}

// Function for determining the order of a point on the elliptic curve modulo p.
fn find_order(g: (i64, i64), a: i64, _b: i64, p: i64) -> usize {
    let mut current_point = g;
    let mut order = 1;

    loop {
        current_point = add_points(current_point, g, a, p).unwrap_or((-1, -1));
        order += 1;

        // Checks whether we've reached the point of infinity.
        if current_point == (-1, -1) {
            return order;
        }
    }
}

// Function that adds two points on the elliptic curve modulo p
fn add_points(p1: (i64, i64), p2: (i64, i64), a: i64, p: i64) -> Option<(i64, i64)> {
    if p1 == (-1, -1) {
        return Some(p2);
    }
    if p2 == (-1, -1) {
        return Some(p1);
    }

    let (x1, y1) = p1;
    let (x2, y2) = p2;

    if x1 == x2 && y1 != y2 {
        return Some((-1, -1)); // Points are additive inverses, return point at infinity
    }

    let lambda: i64;
    if p1 == p2 {
        // Point doubling
        let num = (3 * x1 * x1 + a) % p;
        let den = (2 * y1).rem_euclid(p); // Use rem_euclid to handle negative modulus
        lambda = (num * mod_inverse(den, p)?) % p;
    } else {
        // Point addition
        let num = (y2 - y1).rem_euclid(p); // Use rem_euclid to handle negative modulus
        let den = (x2 - x1).rem_euclid(p); // Use rem_euclid to handle negative modulus
        lambda = (num * mod_inverse(den, p)?) % p;
    }

    let x3 = (lambda * lambda - x1 - x2).rem_euclid(p); // Use rem_euclid to handle negative modulus
    let y3 = (lambda * (x1 - x3) - y1).rem_euclid(p); // Use rem_euclid to handle negative modulus

    Some((x3, y3))
}

// Function to compute modular inverse using extended Euclidean algorithm
fn mod_inverse(a: i64, p: i64) -> Option<i64> {
    let (mut t, mut new_t) = (0, 1);
    let (mut r, mut new_r) = (p, a);

    while new_r != 0 {
        let quotient = r / new_r;
        let temp_t = t;
        t = new_t;
        new_t = temp_t - quotient * new_t;

        let temp_r = r;
        r = new_r;
        new_r = temp_r - quotient * new_r;
    }

    if r > 1 {
        return None; // No modular inverse exists
    }
    if t < 0 {
        t += p;
    }
    Some(t)
}

   