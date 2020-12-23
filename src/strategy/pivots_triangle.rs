use crate::{
    model::open_close::OpenClose,
    technicals::pivots::{Pivot, PivotType},
};

pub fn pivots_triangle<'a>(pivots: &[&Pivot<'a>], minutes: &u32) -> Vec<OpenClose> {
    let mut triangles = Vec::new();
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
            triangles.push(OpenClose::from_date_close(p[5].close_time, minutes));
        };
    }
    triangles
}

#[cfg(test)]
mod test {

    #[test]
    fn pivots_triangle_test() {}
}
