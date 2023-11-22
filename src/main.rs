use std::env::{self, args};
use std::collections::HashMap;
use std::io::Error;
use std::fs::write;
use std::io::Read;
use std::str::FromStr;
struct Todo{
    map:HashMap<String,bool>

}
impl Todo{
    fn to_insert(&mut self, key: String){

        self.map.insert(key, true);
    }


    fn save (self)-> Result<(), Error>{
        let mut content = String::new();

        for (k,v) in self.map{

            let record = format!("{}\t{}\n", k,v);

            content.push_str(&record);
    
        }
        write("db.txt", content)
        
    }

    fn new()-> Result<Todo, Error>{
       let mut f = std::fs::OpenOptions::new()
       .write(true)
       .create(true)
       .read(true)
       .open("db.txt")?;

        let mut content = String::new();

        f.read_to_string(&mut content)?;
        let map:HashMap<String,bool> = content.lines()
        .map(|x|x.splitn(2, '\t').collect::<Vec<&str>>())
        .map(|v|(v[0],v[1]))
        .map(|(k,v)| (String::from(k), bool::from_str(v).unwrap()))
        .collect();

            Ok(Todo { map})

    }

    fn complete(&mut self,key: &String)-> Option<()>{
        match self.map.get_mut(key) {
            Some(v)=> Some(*v =false),
            None => None
        }
    }
}




fn main(){

    let action = args().nth(1).expect("failed to put the action");

    let item = args().nth(2).expect("failed to put the item");
     
     println!("{} {}", action, item );

        let mut todolist = Todo::new().expect("failure to initilize db.txt");

        if action =="add"{
            todolist.to_insert(item);
            match todolist.save(){
                Ok(_)=> println!("it has been saved "),
                Err(e)=> println!("An error occured {}" , e)
            }
                
            }else if action == "complete" {
                match todolist.complete(&item) {
                    None => println!("{} is not present in the item", item),
                    Some(_) => match todolist.save(){
                        Ok(_) => println!("it has been saved"),
                        Err(e) => println!("An error has occured {}", e),
                    }
                }
            }

        }



