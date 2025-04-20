/*

JSON CON: 
    /*  
    {
        "dia": {"hora": "evento", "hora": "evento"}, 
        "dia": {"hora": "evento", "hora": "evento"}
    }
    */
Opciones: 
    - Tareas pendientes para hoy. 
    - Agregar tarea.
    - Eliminar tarea. 


*/

// Implementar los hashmaps
use chrono::{DateTime, Utc, TimeZone};
use serde_json::{json, Value};
use std::{collections::HashMap, fs::{self, File}, io::prelude::*, os::unix::fs::chroot, path::Path, time::SystemTime};

mod utils;

fn main() {

    let mut calendar: HashMap<String, Value> = HashMap::new();

    // Importación del archivo. 

    if Path::new("calendar.json").exists() 
    {
        println!("Welcome back! Charging database..."); 
        calendar = loadCalendar().unwrap();
    }
    else {
        println!("Welcome to the calendar.rust, a little exercise."); 
    }

    // Seleccionar opción en el menú

    let optionMenu = utils::Menu::display_menu(&["See the pending tasks for today","See the tasks for the selected date", "Insert task", "Remove task"]);
    
    match optionMenu {
        0 => {
            print!("-------------------\nTasks for today");
            let dateOfToday = chrono::Utc::now().date().and_hms(0, 0, 0);
            println!("\n------------------->  {}", dateOfToday.format("%d-%b-%Y"));

            let dayOfTodayData = calendar.get(&dateOfToday.timestamp().to_string());

            if let Some(result) = dayOfTodayData {
                for (key, value) in result.as_object().unwrap() {
                    let stringHour = Utc.timestamp_opt(key.parse::<i64>().unwrap(), 0).unwrap();
                    println! ("Id: ({})\nTask: {}\nHour: {}", key, value.to_string().replace("\\n", ""), stringHour.format("%H:%M") );
                }
            } else {
                println!("There is not any task for today");
            }
        },
        1 => {
            let cinDate = cinDateYearMonthDay();
            let year = cinDate.0;
            let month = cinDate.1;
            let day = cinDate.2;

            let dateEntered: DateTime<Utc> = chrono::Utc.ymd(year, month, day).and_hms(0, 0, 0);
            let dayEnteredData  = calendar.get(&dateEntered.timestamp().to_string());

            if let Some(result) = dayEnteredData {
                for (key, value) in result.as_object().unwrap() {
                    let stringHour = Utc.timestamp_opt(key.parse::<i64>().unwrap(), 0).unwrap();
                    println! ("Id: ({})\nTask: {}\nHour: {}", key, value.to_string().replace("\\n", ""), stringHour.format("%H:%M") );
                }
            } else {
                println!("There is not any task for the entered day");
            }            
        },
        2 => {

            print!("-------------------\nInsert task\n-------------------\n");
            let cinDate = cinDateYearMonthDayHourMinuteTaskName();
            let year = cinDate.0;
            let month = cinDate.1;
            let day = cinDate.2;
            let hour = cinDate.3;
            let minute = cinDate.4;
            let taskName = cinDate.5;

            // Creación de identificador para día
            let dayIdentifier: DateTime<Utc> = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();
            
            let eventIdentifier: DateTime<Utc> = Utc.with_ymd_and_hms(year, month, day, hour, minute, 0).unwrap();

            if calendar.contains_key(&dayIdentifier.timestamp().to_string()) {
                let mut newValue = calendar.get(&dayIdentifier.timestamp().to_string()).unwrap().clone();
                newValue.as_object_mut().unwrap().insert(eventIdentifier.timestamp().to_string(), json!(taskName) );
                calendar.insert(dayIdentifier.timestamp().to_string(), newValue);
            }
            else {
                calendar.insert(
                    dayIdentifier.timestamp().to_string(),
                    json!(
                        {
                            eventIdentifier.timestamp().to_string(): taskName
                        }
                    )
                );
            }

            saveCalendar(&calendar);
        },
        3 => {
            print!("-------------------\nRemove task\n-------------------\n");
            println! ("Enter the task id:");

            let mut taskId = utils::Cin.string();
            taskId = taskId.trim().to_string(); // Estrictamente necesario si se quieren eliminar los saltos de línea. 

            println! ("{}", taskId);

            // ¡Recuerda! Utilizar .iter.mut te permitirá modificar hashmaps iterados sin que se pierda su ownership
            // Esto ha sido útil, porque nos permitirá guardarlo próximamente sin que desaparezca. 

            for (key, mut value) in calendar.iter_mut() {
                for task in value.as_object_mut() {
                    if let Some(result) = task.get(&taskId) {
                        task.remove(&taskId);
                        println! ("The selected task has been removed.");
                    }
                }
            };

            saveCalendar(&calendar);
            
        },
        _ => {
            println!("Fatal error");
        }
    };
    
    //Exportación del archivo. 

    fn saveCalendar (calendar: &HashMap<String, Value>) ->  std::io::Result<()> {
        let mut file = File::create("calendar.json")?; // En vez de hacer un try/catch, se pone un "?" al final para prevenir el error
        let json_string = serde_json::to_string_pretty(calendar).unwrap();
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }

    fn loadCalendar ()  ->  std::io::Result<HashMap<String, Value>>  {
        let json_str: String = fs::read_to_string("calendar.json")?;
        Ok((serde_json::from_str(&json_str).expect("ERROR in JSON format")))
    }
    
    fn cinDateYearMonthDayHourMinuteTaskName () -> (i32, u32, u32, u32, u32, String) {
            println! ("Year: ");
            let year: i32 = utils::Cin.i32();
            println! ("Month: ");
            let mut month = utils::Cin.u32();
            loop {
                if month > 12 || month <=0 {
                    println! ("---------\n[!]Value out of range[!]\n---------\n");
                    println! ("Month: ");
                    month = utils::Cin.u32();
                } else {
                    break
                };
            };
            println! ("Day: ");
            let mut day = utils::Cin.u32();
            loop {
                if day > 31 || day <=0 {
                    println! ("---------\n[!]Value out of range[!]\n---------\n");
                    println! ("Day: ");
                    day = utils::Cin.u32();
                } else {
                    break
                };
            };
            println! ("Hour: ");
            let mut hour = utils::Cin.u32();
            loop {
                if hour > 24 || hour <=0 {
                    println! ("---------\n[!]Value out of range[!]\n---------\n");
                    println! ("Hour: ");
                    hour = utils::Cin.u32();
                } else {
                    break
                };
            };
            println! ("Minute: ");
            let mut minute = utils::Cin.u32();
            loop {
                if minute > 60 || minute <=0 {
                    println! ("---------\n[!]Value out of range[!]\n---------\n");
                    println! ("Minute: ");
                    minute = utils::Cin.u32();
                } else {
                    break
                };
            };
            println! ("Name of the task: ");
            let taskName = utils::Cin.string();



        (year,month,day,hour,minute,taskName)
    }

    fn cinDateYearMonthDay () -> (i32, u32, u32) {
        println! ("Year: ");
            let year: i32 = utils::Cin.i32();
            println! ("Month: ");
            let mut month = utils::Cin.u32();
            loop {
                if month > 12 || month <=0 {
                    println! ("---------\n[!]Value out of range[!]\n---------\n");
                    println! ("Month: ");
                    month = utils::Cin.u32();
                } else {
                    break
                };
            };
            println! ("Day: ");
            let mut day = utils::Cin.u32();
            loop {
                if day > 31 || day <=0 {
                    println! ("---------\n[!]Value out of range[!]\n---------\n");
                    println! ("Day: ");
                    day = utils::Cin.u32();
                } else {
                    break
                };
            };
        (year,month,day)
    }
}
