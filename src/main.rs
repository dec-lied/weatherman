pub mod ui;
pub mod weather;

use tui::
{
    backend::CrosstermBackend,
    widgets::{Paragraph, ListState, List},
    layout::Rect,
    Terminal
};
use std::
{
    time::{Instant, Duration},
    sync::mpsc, thread
};
use ui::{forecast_screen::ForecastPosition, Screen};
use crossterm::event::KeyCode;
use weather::WeeklyForecast;
use std::io;

#[tokio::main]
async fn main()
{
    // initializing terminal
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(CrosstermBackend::new(io::stdout())).expect("failed to instantiate crossterm terminal");
    terminal.clear().expect("failed to clear the terminal");

    // drawing loading screen while waiting for forecast api to respond
    terminal.draw
    (
        move |rect|
        {
            let loading_panel: Rect = ui::loading_screen::get_loading_panel(rect.size());

            let loading_message: Paragraph = ui::loading_screen::get_loading_message();

            rect.render_widget(loading_message, loading_panel);
        }
    ).expect("failed to draw on terminal");

    // getting api response
    let weekly_forecast: WeeklyForecast = WeeklyForecast::from(weather::generate_request().await.expect("failed to get api response"));

    // clearing terminal once api has responded
    terminal.clear().expect("failed to clear the terminal");

    // message passing for input
    let (tx, rx) = mpsc::channel::<crossterm::event::Event>();
    thread::spawn
    (
        move ||
        {
            const TICK_RATE: Duration = Duration::from_millis(200);
            let mut last_poll: Instant = Instant::now();

            loop
            {
                if last_poll.elapsed() >= TICK_RATE
                {
                    last_poll = Instant::now();
                }

                if crossterm::event::poll(TICK_RATE).expect("how is this even possible")
                {
                    if let crossterm::event::Event::Key(key) = crossterm::event::read().expect("error encountered in reading from crossterm")
                    {
                        tx.send(crossterm::event::Event::Key(key)).expect("failed to send key event through mpsc");
                    }
                }
            }
        }
    );

    // setting up main loop
    let mut screen: Screen = Screen::FORECAST;
    
    // initializing menu state
    let mut menu_state: ListState = ListState::default();
    menu_state.select(Some(0));

    loop
    {
        // checking for screen
        match screen
        {
            Screen::FORECAST =>
            {
                // drawing home screen
                terminal.draw
                (
                    |rect|
                    {
                        // drawing simple elements
                        let main_panel: Vec<Rect> = ui::forecast_screen::get_forecast_panel(rect.size());

                        let title: Paragraph = ui::forecast_screen::get_forecast_title();
                        let controls: Paragraph = ui::forecast_screen::get_forecast_controls();

                        rect.render_widget(title, main_panel[0]);
                        rect.render_widget(controls, main_panel[2]);

                        // drawing forecast
                        let forecast_panel: Vec<Rect> = ui::forecast_screen::get_forecast_slot_panel(main_panel[1]);
                        let forecast_slots: Vec< Vec<Rect> > = weekly_forecast.days.iter()
                            .enumerate()
                            .map(|(i, _)| ui::forecast_screen::get_forecast_slot_layout(forecast_panel[i]))
                            .collect();

                        let daily_paragraph_sets: Vec< Vec<Paragraph> > = weekly_forecast.days.iter().enumerate()
                            .map(|(index, day)| ui::forecast_screen::get_forecast_paragraphs(day, ForecastPosition::from_index(&index)))
                            .collect();

                        for paragraph_set in daily_paragraph_sets.into_iter().zip(forecast_slots.into_iter())
                        {
                            for i in 0..paragraph_set.0.len()
                            {
                                rect.render_widget(paragraph_set.0[i].clone(), paragraph_set.1[i]);
                            }
                        }
                    }
                ).expect("error encountered in drawing on the terminal");

                // handling input for home screen
                match rx.recv().expect("faild to read from mpsc")
                {
                    crossterm::event::Event::Key(key) => match key.code
                    {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('m') => screen = Screen::MENU,
                        _ => {}
                    },
                    _ => {}
                }
            },
            Screen::MENU =>
            {
                terminal.draw
                (
                    |rect|
                    {
                        let menu_panel: Vec<Rect> = ui::menu_screen::get_menu_panel(rect.size());

                        let menu_list: List = ui::menu_screen::get_menu_list();

                        rect.render_stateful_widget(menu_list, menu_panel[1], &mut menu_state);
                    }
                ).expect("failed to draw on terminal");

                match rx.recv().expect("failed to read from mpsc")
                {
                    crossterm::event::Event::Key(key) => match key.code
                    {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') =>
                        {
                            if let Some(sel_index) = menu_state.selected()
                            {
                                menu_state.select(Some(((sel_index) + 1) % 3));
                            }
                            else
                            {
                                menu_state.select(Some(0));
                            }
                        },
                        KeyCode::Char('k') =>
                        {
                            if let Some(sel_index) = menu_state.selected()
                            {
                                if sel_index == 0
                                {
                                    menu_state.select(Some(2));
                                }
                                else
                                {
                                    menu_state.select(Some(sel_index - 1));
                                }
                            }
                            else
                            {
                                menu_state.select(Some(2));
                            }
                        },
                        KeyCode::Enter =>
                        {
                            if let Some(sel_index) = menu_state.selected()
                            {
                                screen = match sel_index
                                {
                                    0 => Screen::FORECAST,
                                    1 => Screen::HOURLY,
                                    2 => Screen::OPTIONS,
                                    _ => Screen::FORECAST
                                };
                            }
                        },
                        _ => {}
                    },
                    _ => {}
                }
            },
            Screen::HOURLY =>
            {
                terminal.draw
                (
                    |rect|
                    {
                        
                    }
                ).expect("failed to draw on terminal");

                match rx.recv().expect("failed to read from mpsc")
                {
                    crossterm::event::Event::Key(key) => match key.code
                    {
                        KeyCode::Char('q') => break,
                        _ => {}
                    },
                    _ => {}
                }
            },
            Screen::OPTIONS =>
            {
                terminal.draw
                (
                    |rect|
                    {

                    }
                ).expect("failed to draw on terminal");

                match rx.recv().expect("failed to read from mpsc")
                {
                    crossterm::event::Event::Key(key) => match key.code
                    {
                        KeyCode::Char('q') => break,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    terminal.clear().expect("failed to clear terminal");
}
