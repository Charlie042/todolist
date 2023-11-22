use std::env::{self, args};
use std::collections::HashMap;
use std::io::Error;
use std::fs::write;
use std::io::Read;
use std::str::FromStr;

use serde_json::map;
struct Todo{
    map:HashMap<String,bool>

}
impl Todo{
    fn to_insert(&mut self, key: String){

        self.map.insert(key, true);
    }


        fn save(self) -> Result<(), Box<dyn std::error::Error>> {
            // open db.json
            let f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open("db.json")?;
            // write to file with serde
            serde_json::to_writer_pretty(f, &self.map)?;
            Ok(())
    }

    fn new()-> Result<Todo, Error>{
       let mut f = std::fs::OpenOptions::new()
       .write(true)
       .create(true)
       .read(true)
       .open("db.json")?;

       
        let mut content = String::new();

        f.read_to_string(&mut content)?;
        let map:HashMap<String,bool> = content.lines()
        .map(|x|x.splitn(2, '\t').collect::<Vec<&str>>())
        .map(|v|(v[0],v[1]))
        .map(|(k,v)| (String::from(k), bool::from_str(v).unwrap()))
        .collect();

          
            match serde_json::from_reader(f) {
                Ok(map) => Ok(Todo { map }),
                Err(e) if e.is_eof() => Ok(Todo {
                    map: HashMap::new(),
                }),
                Err(e) => panic!("An error occurred: {}", e),
        

    }
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
    


