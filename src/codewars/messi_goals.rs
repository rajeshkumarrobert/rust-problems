use std::ops::Add;

pub fn goals(la_liga_goals: i32, champions_league_goals: i32, copa_del_rey_goals: i32) -> i32 {
    let mut result:i32 =0;
    result = result.add(la_liga_goals).add(champions_league_goals).add(copa_del_rey_goals);
    result
 }