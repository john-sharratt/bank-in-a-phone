pub fn pretty_print_cents(total_cents: u64) -> String {
    let dollars = total_cents / 100;
    let cents = total_cents % 100;
    format!("${}.{:02}", pretty_print_u64(dollars), cents)
}

fn pretty_print_u64(i: u64) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}
