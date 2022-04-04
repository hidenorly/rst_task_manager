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

    pub fn new() -> Self {
        Self {
            is_running : false,
            is_stopping : false,
        }
    }
}


fn main() {
    let mut handles = Vec::new();

    for x in 0..10 {
        handles.push(thread::spawn(move || {
            let mut task = Task::new();
            println!("task:{}", x);
            task.execute();
            thread::sleep(Duration::from_millis(1));
        }))
    }

    thread::sleep(Duration::from_millis(1));

    for handle in handles{
        let _ = handle.join();
    }
}



