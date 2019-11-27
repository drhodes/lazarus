
struct Env {    
    table: HashMap<String, Id>,
    outer: Option<Box<Env>>,
}


impl Env {
    fn new(mut parms: Vec<String>, mut args: Vec<Id>, outer: Option<Box<Env>>) -> Env {
        assert_eq!(parms.len() == args.len(), true);
        let mut table = HashMap::new();
        while parms.len() > 0 {
            table.insert(parms.remove(0), args.remove(0));
        }
        Env{table, outer}
    }
   
    fn find(&mut self, var: &String) -> Option<&mut Id> {
        if self.table.contains_key(var) {
            return self.table.get_mut(var);
        } else if self.outer.is_some() {
            return self.find(var);
        } else {
            return None;
        }
    }

    fn standard_env() -> Env {
        let mut env = Env::new(vec!(), vec!(), None);
        env.table.insert(String::from("abs"), 0);
        return env;
    }
}
