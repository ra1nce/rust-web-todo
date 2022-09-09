use serde::{Serialize, Deserialize};
use std::collections::{HashMap};
use rand::Rng;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    id: i32,
    text: String,
    state: String
}


pub fn get_tasks(con: &mut redis::Connection) -> Vec<Task>{
    let mut tasks: Vec<Task> = Vec::new();
    
    for i in get_task_ids(con){
        let task: HashMap<String, String> = redis::cmd("HGETALL").arg(format!("task:{}", i)).query(con).unwrap();
        
        tasks.push(
            Task{
                id: i,
                text: task.get("text").unwrap().to_string(),
                state: task.get("state").unwrap().to_string()
            }
        )
    }

    tasks
}


pub fn add_task(con: &mut redis::Connection, text: String){
    let mut rng = rand::thread_rng();
    let id: i32 = rng.gen_range(0..1000000);

    let _ : () = redis::cmd("LPUSH").arg("ids").arg(id).query(con).unwrap();
    let _ : () = redis::cmd("HSET")
        .arg(format!("task:{}", id))
        .arg("text")
        .arg(text)
        .arg("state")
        .arg("false")
        .query(con).unwrap();
}


pub fn mark_ok(con: &mut redis::Connection, id: i32) {
    let _ : () = redis::cmd("HSET")
        .arg(format!("task:{}", id))
        .arg("state")
        .arg("true")
        .query(con).unwrap();
}


pub fn mark_undo(con: &mut redis::Connection, id: i32) {
    let _ : () = redis::cmd("HSET")
        .arg(format!("task:{}", id))
        .arg("state")
        .arg("false")
        .query(con).unwrap();
}


pub fn del_task(con: &mut redis::Connection, id: i32) {
    let _ : () = redis::cmd("LREM").arg("ids").arg(1).arg(id).query(con).unwrap();
}


fn get_task_ids(con: &mut redis::Connection) -> Vec<i32>{
    let ids: Vec<i32> = redis::cmd("LRANGE").arg("ids").arg("0").arg("-1").query(con).unwrap();

    ids
}