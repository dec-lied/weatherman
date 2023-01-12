pub mod ui;
pub mod weather;

use tui::
{
    backend::CrosstermBackend,
    widgets::Paragraph,
    layout::Rect,
    Terminal
};
use std::
{
    time::{Instant, Duration},
    sync::mpsc, thread
};
use ui::{ForecastPosition, Screen};
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
            let loading_panel: Rect = ui::get_loading_panel(rect.size());

            let loading_message: Paragraph = ui::get_loading_message();

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
    let mut screen: Screen = Screen::HOME;

    loop
    {
        // checking for screen
        match screen
        {
            Screen::HOME =>
            {
                // drawing home screen
                terminal.draw
                (
                    |rect|
                    {
                        // drawing simple elements
                        let main_panel: Vec<Rect> = ui::get_home_panel(rect.size());

                        let title: Paragraph = ui::get_title();
                        let tabs: Paragraph = ui::get_tabs();
                        let controls: Paragraph = ui::get_controls();

                        rect.render_widget(title, main_panel[0]);
                        rect.render_widget(tabs, main_panel[1]);
                        rect.render_widget(controls, main_panel[3]);

                        // drawing forecast
                        let forecast_panel: Vec<Rect> = ui::get_forecast_panel(main_panel[2]);
                        let forecast_slots: Vec< Vec<Rect> > = weekly_forecast.days.iter()
                            .enumerate()
                            .map(|(i, _)| ui::get_forecast_slot_layout(forecast_panel[i]))
                            .collect();

                        let daily_paragraph_sets: Vec< Vec<Paragraph> > = weekly_forecast.days.iter().enumerate()
                            .map(|(index, day)| ui::get_forecast_paragraphs(day, ForecastPosition::from_index(&index)))
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
                        KeyCode::Char('q') =>
                        {
                            break;
                        },
                        KeyCode::Char('m') =>
                        {
                            screen = Screen::MENU;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    terminal.clear().expect("failed to clear terminal");
}
