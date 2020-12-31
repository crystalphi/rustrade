use crate::{
    model::open_close::OpenClose,
    technicals::topbottom::{TopBottom, TopBottomType},
};

pub fn topbottom_triangle<'a>(topbottoms: &[&TopBottom<'a>], minutes: &u32) -> Vec<OpenClose> {
    let mut triangles = Vec::new();
    for i in 0..topbottoms.len() - 6 {
        let p = [
            topbottoms.get(i).unwrap(),
            topbottoms.get(i + 1).unwrap(),
            topbottoms.get(i + 2).unwrap(),
            topbottoms.get(i + 3).unwrap(),
            topbottoms.get(i + 4).unwrap(),
            topbottoms.get(i + 5).unwrap(),
        ];
        if p[0].type_p == TopBottomType::Bottom && p[0].price > p[2].price && p[2].price > p[4].price && p[1].price < p[3].price && p[3].price < p[5].price {
            println!("{}", p[5].close_time);
            triangles.push(OpenClose::from_date_close(p[5].close_time, minutes));
        };
    }
    triangles
}

#[cfg(test)]
mod test {}
