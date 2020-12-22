use crate::technicals::pivots::{Pivot, PivotType};

pub fn pivots_triangle<'a>(pivots: &Vec<Pivot<'a>>) {
    for i in 0..pivots.len() - 6 {
        let p = [
            pivots.get(i).unwrap(),
            pivots.get(i + 1).unwrap(),
            pivots.get(i + 2).unwrap(),
            pivots.get(i + 3).unwrap(),
            pivots.get(i + 4).unwrap(),
            pivots.get(i + 5).unwrap(),
        ];
        if p[0].type_p == PivotType::High && p[0].price > p[2].price && p[2].price > p[4].price && p[1].price < p[3].price && p[3].price < p[5].price {
            println!("{}", p[5].close_time);
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pivots_triangle_test() {}
}
