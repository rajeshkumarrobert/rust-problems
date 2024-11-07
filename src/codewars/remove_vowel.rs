pub fn disemvowel(s: &str) -> String {
   let result:String =  s.chars().into_iter().filter(|f| *f!='a'&&*f!='e'&&*f!='i'&&*f!='o'&&*f!='u'&&*f!='A'&&*f!='E'&&*f!='I'&&*f!='O'&&*f!='U').collect();
   result
}

//s.chars()
// .filter(|&c| !"aeiou".contains(c.to_ascii_lowercase()))
// .collect()