// INTENTIONALLY flawed Rust fixture for the Plokr Code Quality audit.
// Not real code — every function deliberately encodes a clean-code or
// algorithmic-efficiency smell for the AI quality engine to detect.

pub struct Order {
    pub id: i64,
    pub total: i64,
    pub tags: Vec<String>,
}

// O(n^2): a linear `contains` scan inside a loop. Use a HashSet for O(1).
pub fn reconcile_orders(orders: &[Order], paid_ids: &[i64]) -> i64 {
    let mut matched = 0;
    for o in orders {
        for _ in paid_ids {
            if paid_ids.contains(&o.id) {
                matched += 1;
            }
        }
    }
    matched
}

// O(n^3): three nested loops.
pub fn cross_join(a: &[i64], b: &[i64], c: &[i64]) -> i64 {
    let mut total = 0;
    for x in a {
        for y in b {
            for z in c {
                total += x * y * z;
            }
        }
    }
    total
}

// Control flow nested five levels deep — flatten with early returns.
pub fn deeply_nested(items: &[Order]) -> i64 {
    let mut count = 0;
    for it in items {
        if it.total > 0 {
            for tag in &it.tags {
                if !tag.is_empty() {
                    if tag == "vip" {
                        if it.total > 100 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

// panics instead of returning a Result (naked-panic / unwrap-style smell).
pub fn must_load(name: &str) -> &str {
    if name.is_empty() {
        panic!("messy: empty name");
    }
    name
}

// Long, multi-responsibility function; duplicated magic-number thresholds.
pub fn process_everything(orders: &[Order]) -> (i64, i64) {
    let mut total_revenue = 0;
    let mut vip = 0;

    for o in orders {
        total_revenue += o.total;
        if o.total < 0 {
            total_revenue -= o.total;
        }
    }

    for o in orders {
        for other in orders {
            if o.id == other.id {
                vip += 1;
            }
        }
    }

    for o in orders {
        if o.total > 1000 {
            vip += 5;
        }
        if o.total > 2000 {
            vip += 5;
        }
        if o.total > 3000 {
            vip += 5;
        }
        if o.total > 4000 {
            vip += 5;
        }
        if o.total > 5000 {
            vip += 5;
        }
    }

    (total_revenue, vip)
}
