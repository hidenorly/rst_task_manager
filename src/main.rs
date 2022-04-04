/*
  Copyright (C) 2022 hidenorly

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::thread;
use std::time::Duration;

pub struct Task
{
    id : String,
    is_running : bool,
    is_stopping : bool,
}

impl Task
{
    fn on_execute( &self ){
        if let false = self.is_stopping  {
            println!("on_execute");
        }
    }

    pub fn execute( &mut self ){
        self.is_running = true;
        self.is_stopping = false;
        self.on_execute();
        self.is_running = false;
    }

    pub fn is_running( &self ) -> bool {
        self.is_running
    }

    pub fn cancel( &mut self ) {
        if let true = self.is_running {
            self.is_stopping = true
        }
    }

    pub fn new( id : String ) -> Self {
        Self {
            id : id,
            is_running : false,
            is_stopping : false,
        }
    }
}


pub struct TaskManager
{
    tasks : Vec<Task>,
}

impl TaskManager
{
    pub fn new() -> Self {
        Self {
            tasks : Vec::new(),
        }
    }

    pub fn add_task( &mut self, task : Task){
        self.tasks.push( task );
    }

    pub fn execute( &mut self ){
        // TODO: execute the task in thread::spawn
        for _task in &self.tasks {
            println!( "task:{}", _task.id )
        }
    }

    pub fn cancel_task( &mut self, _id : String ){
        let not_found_index = -1;
        let mut found_index : i32 = not_found_index;
        let mut index : i32 = 0;
        for _task in &self.tasks {
            if _id == _task.id {
                found_index = index;
            }
            index = index + 1;
        }
        if not_found_index != found_index {
            let found_index : usize = found_index as usize;
            self.tasks.remove( found_index );
        }
    }
}

fn main() {
    // test case for Task
    let mut handles = Vec::new();

    for x in 0..10 {
        let x : i32 = x;
        let mut task = Task::new( x.to_string() );
        println!("task:{}", x);
        handles.push( thread::spawn( move || {
            task.execute();
            thread::sleep(Duration::from_millis(1));
        } ) );
    }

    thread::sleep(Duration::from_millis(1));

    for handle in handles{
        let _ = handle.join();
    }


    // test case for TaskManager
    let mut task_manager = TaskManager::new();

    for _x in 0..10 {
        let x : i32 = _x;
        let task = Task::new( x.to_string() );
        task_manager.add_task( task );
    }

    task_manager.execute();

    for _x in 0..10 {
        let x : i32 = _x;
        task_manager.cancel_task( x.to_string() );
    }

}