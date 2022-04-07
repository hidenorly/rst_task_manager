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

trait ITask
{
    fn set_id( &mut self, id : String );
    fn get_id( &self ) -> String;
    fn set_running_state( &mut self, is_running : bool );
    fn is_running( &self ) -> bool;
    fn set_stopping( &mut self, is_stopping : bool );
    fn is_stopping( &self ) -> bool;

    fn on_execute( &self );
    fn new( id : String ) -> Self;

    fn execute( &mut self ){
        self.set_running_state( true );
        self.set_stopping( false );
        self.on_execute();
        self.set_running_state( false );
    }
    fn cancel( &mut self ) {
        if let true = self.is_running() {
            self.set_stopping( true );
        }
    }
}


#[derive(Clone)]
pub struct Task
{
    id : String,
    is_running : bool,
    is_stopping : bool,
}

impl ITask for Task
{
    fn set_id( &mut self, id : String )
    {
        self.id = id;
    }

    fn get_id( &self ) -> String
    {
        self.id.clone()
    }

    fn set_running_state( &mut self, is_running : bool )
    {
        self.is_running = is_running;
    }

    fn is_running( &self ) -> bool {
        self.is_running
    }

    fn set_stopping( &mut self, is_stopping : bool )
    {
        self.is_stopping = is_stopping;
    }

    fn is_stopping( &self ) -> bool
    {
        self.is_stopping
    }

    fn on_execute( &self ){
        if let false = self.is_stopping()  {
            println!("on_execute:{}", &self.id);
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn new( id : String ) -> Self {
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
    max_concurrency : i32,
}

impl TaskManager
{
    pub fn new( max_concurrency : i32 ) -> Self {
        Self {
            tasks : Vec::new(),
            max_concurrency : max_concurrency,
        }
    }

    pub fn add_task( &mut self, task : Task){
        self.tasks.push( task );
    }

    pub fn execute( &mut self ){
        let mut handles = Vec::new();
        let mut handled_tasks = Vec::new();

        let mut count = 0;
        for _task in &self.tasks {
            if !_task.is_running() {
                handled_tasks.push( _task.id.clone() );
                let mut task = _task.clone();
                handles.push( thread::spawn( move || {
                    task.execute();
                } ) );
                count = count + 1;
                if count >= self.max_concurrency {
                    break;
                }
            }
        }

        for handle in handles{
            let _ = handle.join();
        }

        for _i in handled_tasks {
            self.remove_task( _i, false );
        }
    }

    pub fn remove_task( &mut self, _id : String, is_stop_task : bool ){
        let not_found_index = -1;
        let mut found_index : i32 = not_found_index;
        let mut index : i32 = 0;
        for _task in &self.tasks {
            if _id == _task.id {
                found_index = index;
                break;
            }
            index = index + 1;
        }
        if not_found_index != found_index {
            let found_index : usize = found_index as usize;
            if is_stop_task && self.tasks[ found_index ].is_running() {
                // TODO: join the task's thread
            }
            let _ = self.tasks.remove( found_index );
        }
    }

    pub fn cancel_task( &mut self, _id : String ){
        self.remove_task( _id, true );
    }

}

fn main() {
    // test case for TaskManager
    let mut task_manager = TaskManager::new( 4 );

    for _x in 0..10 {
        let x : i32 = _x;
        let task = Task::new( x.to_string() );
        task_manager.add_task( task );
    }

    task_manager.execute();

    for _x in 11..20 {
        let x : i32 = _x;
        let task = Task::new( x.to_string() );
        task_manager.add_task( task );
    }

    task_manager.execute();

    for _x in 0..20 {
        let x : i32 = _x;
        task_manager.cancel_task( x.to_string() );
    }

}