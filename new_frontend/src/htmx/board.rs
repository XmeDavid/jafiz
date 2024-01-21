use chrono::{NaiveDate, Datelike};
use rocket::{response::content::RawHtml, http::Status};
use tokio::{io::AsyncReadExt, fs::File};

use crate::models::board::TableDTO;


pub async fn board_page(table: TableDTO, is_authenticated: bool) -> Result<RawHtml<String>, (Status, String)>{
    let mut file = File::open("./public/board.html").await.map_err(|_| (Status::NotFound, "   >> File './public/board.html' not found".to_string()))?;

    let mut template = String::new();
    file.read_to_string(&mut template).await.map_err(|_| (Status::InternalServerError, "   >> Failed to read from './public/board.html'".to_string()))?;

    let description = table.description.as_str();
    let id = table.id.as_str();

    template = template.replace("{{ name }}", table.name.as_str());
    template = template.replace("{{ description }}", description);
    template = template.replace("{{ months }}", months(table.to_owned()).as_str());

    match is_authenticated {
        true => {
            template = template.replace("{{ checkInButton }}", check_in_button(id).as_str());
            template = template.replace("{{ descriptionHidden }}", "");
            template = template.replace("{{ input }}","");// input(description).as_str());
        },
        false => {
            template = template.replace("{{ checkInButton }}", "");
            template = template.replace("{{ descriptionHidden }}", "");
            template = template.replace("{{ input }}", "");
        }
        
    }

    Ok(RawHtml(template))
}

fn get_empty_days(month_index: u32) -> String{
    let year = chrono::Utc::now().year();
    let month_index = month_index + 1;

    let date = NaiveDate::from_ymd_opt(year, month_index, 1);
    
    match date {
        Some(date) => {
            let day_index = date.weekday().num_days_from_sunday();
            let mut empty_days = String::new();
            for _ in 0..day_index {
                empty_days += "<div class=\"w-4 h-4\"></div>"
            }
            return empty_days;
        },
        None => {
            return String::new();
        }
    }

    
}

fn input(value: &str) -> String {
format!("
    <input type=\"text\" value=\"{}\"
        style=\"min-width: 0px;\"
        class=\"mr-2 mt-1 block w-fit
            text-2xl text-gray-900
            border-0 border-b-2 border-gray-300
            bg-transparent appearance-none dark:text-white dark:border-gray-600
            dark:focus:border-green-500focus:outline-none focus:ring-0 focus:border-green-600
        \"
        hidden
    >
", value)
}


fn check_in_button(id: &str) -> String {
format!("
    <button id=\"gradientButton\" hx-post=\"/check-in/{}\" hx-target=\"#monthsContainer\" class=\"w-48 h-16 relative flex justify-center place-items-center items-center mb-10 bg-green-400 rounded-lg group\">
    <span id=\"gradientBackground\" class=\"absolute inset-0 bg-gradient-to-r opacity-0 group-hover:opacity-100 transition-opacity duration-300\" style=\"background-size: 200% 200%;\"></span>
    <p class=\"z-50 text-xl font-bold text-white text-nowrap w-min h-min\"> Check in</p>
    </button>

    <script>
        document.body.addEventListener('htmx:load', function() {{
            var gradientButton = document.getElementById('gradientButton');
            if (gradientButton) {{
                var gradientBackground = document.getElementById('gradientBackground');

                gradientButton.addEventListener('mousemove', function(e) {{
                    var rect = gradientButton.getBoundingClientRect();
                    var x = (e.clientX - rect.left) / gradientButton.offsetWidth * 100;
                    var y = (e.clientY - rect.top) / gradientButton.offsetHeight * 100;
                    gradientBackground.style.backgroundPosition = x + '% ' + y + '%';
                }});
            }}
        }});
    </script>
", id)
}

pub fn months(table: TableDTO) -> String{
    [
        month(table.january, "January", 0),
        month(table.february, "February", 1),
        month(table.march, "March", 2),
        month(table.april, "April", 3),
        month(table.may, "May", 4),
        month(table.june, "June", 5),
        month(table.july, "July", 6),
        month(table.august, "August", 7),
        month(table.september, "September", 8),
        month(table.october, "October", 9),
        month(table.november, "November", 10),
        month(table.december, "December", 11),
    ].join("")
}


fn month(month: Vec<bool>, name: &str, month_index: u32) -> String{
    let mut string = format!("
    <div>
    <span class=\"text-gray-600 dark:text-gray-400 p-1\">
        {}
    </span>
    <div class=\"grid grid-rows-7 grid-flow-col gap-1\">", name);
    
        string += &get_empty_days(month_index);

        for is_completed in month {
            string += day(is_completed).to_owned().as_str();
        }

    string += "</div></div>";
    return string;
}

fn day(is_completed: bool) -> String {
format!("
    <div class=\"w-4 h-4 rounded-sm {} \"> </div>
", if is_completed { "bg-green-500" } else { "bg-gray-300 dark:bg-gray-600 shadow-inner" })
}

