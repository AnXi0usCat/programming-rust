
// learning about lifetimes :)
fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s: &i32  = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn main() {
    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0)
    }
}
