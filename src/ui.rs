use tui::
{
    widgets::{Block, Borders, BorderType, Paragraph},
    layout::{Layout, Direction, Constraint, Rect, Alignment},
    text::{Spans, Span}, style::{Style, Modifier, Color}
};

use crate::weather::DailyWeather;

// keeping track of which screen the user is on
pub enum Screen
{
    HOME,
    MENU
}

// how to let each forecast slot know where it is in the set
#[derive(PartialEq)]
pub enum ForecastPosition
{
    LEFT,
    MIDDLE,
    RIGHT
}

// ease of use for converting index to a position
impl ForecastPosition
{
    pub fn from_index(index: &usize) -> ForecastPosition
    {
        return match index
        {
            0..=2 => ForecastPosition::LEFT,
            3 => ForecastPosition::MIDDLE,
            _ => ForecastPosition::RIGHT
        };
    }
}

// given its anchor area, returns a rect in the middle of the screen for other elements to go in
pub fn get_loading_panel(area: Rect) -> Rect
{
    let columns: Vec<Rect> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints
        ([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20)
        ])
        .split(area);

    let rows:  Vec<Rect> = Layout::default()
        .direction(Direction::Vertical)
        .constraints
        ([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20)
        ])
        .split(columns[2]);

    return rows[2];
}

// returns the loading paragraph
pub fn get_loading_message<'a>() -> Paragraph<'a>
{
    return Paragraph::new("Loading...")
        .block
        (
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .alignment(Alignment::Center)
}

// given the area to place it, returns the layout for the home screen
pub fn get_home_panel(area: Rect) -> Vec<Rect>
{
    return Layout::default()
        .direction(Direction::Vertical)
        .constraints
        ([
            Constraint::Percentage(9),      // title
            Constraint::Percentage(9),      // tab
            Constraint::Percentage(73),     // body
            Constraint::Percentage(9)       // controls
        ])
        .split(area);
}

// returns the title paragraph
pub fn get_title<'a>() -> Paragraph<'a>
{
    return Paragraph::new("weatherman 🌩️")
        .block
        (
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .style
        (
            Style::default()
                .fg(Color::LightBlue)
        )
        .alignment(Alignment::Center);
}

// returns the tabs (for now not much)
pub fn get_tabs<'a>() -> Paragraph<'a>
{
    return Paragraph::new("7 day forecast")
        .block
        (
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .style
        (
            Style::default()
                .fg(Color::White)
        )
        .alignment(Alignment::Center);
}

// returns the layout for the 7 forecast colummns
pub fn get_forecast_panel(area: Rect) -> Vec<Rect>
{
    return Layout::default()
        .direction(Direction::Horizontal)
        .constraints
        ([
            Constraint::Percentage(15),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(15),
        ])
        .split(area);
}

// returns the layout for each individual forecast column
pub fn get_forecast_slot_layout(area: Rect) -> Vec<Rect>
{
    return Layout::default()
        .direction(Direction::Vertical)
        .constraints
        ([
            Constraint::Percentage(14),
            Constraint::Percentage(6),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(13),
            Constraint::Percentage(13),
            Constraint::Percentage(13),
            Constraint::Percentage(13)
        ])
        .split(area);
}

// given the forecast day and its position relative to the other 6, returns a vec of paragraphs with all the notable information
pub fn get_forecast_paragraphs<'a>(day: &DailyWeather, pos: ForecastPosition) -> Vec< Paragraph<'a> >
{
    let mut paragraphs: Vec< Paragraph<'a> > = Vec::with_capacity(7);

    let border: Borders = match pos
    {
        ForecastPosition::LEFT => Borders::LEFT,
        ForecastPosition::MIDDLE => Borders::LEFT.union(Borders::RIGHT),
        ForecastPosition::RIGHT => Borders::RIGHT
    };

    let date_clone: String = day.date.clone();
    let split_date: Vec<&str> = date_clone.split("-").collect();
    paragraphs.push(Paragraph::new(Span::styled(format!("{}/{}/{}", split_date[1], split_date[2], split_date[0]), Style::default().add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
        .style(Style::default().fg(Color::Magenta))
        .alignment(Alignment::Center));

    paragraphs.push(Paragraph::new(""));

    paragraphs.push(Paragraph::new(Span::styled(format!("high: {}°F", day.max_temp), Style::default().fg(Color::LightRed)))
        .block(Block::default().borders(border.clone().union(Borders::TOP)).border_type(BorderType::Rounded))
        .alignment(Alignment::Center));

    paragraphs.push(Paragraph::new(Span::styled(format!("low: {}°F", day.min_temp), Style::default().fg(Color::Cyan)))
        .block(Block::default().borders(border.clone()).border_type(BorderType::Rounded))
        .alignment(Alignment::Center));

    paragraphs.push(Paragraph::new(Span::styled(format!("sunrise: {}", day.sunrise), Style::default().fg(Color::Yellow)))
        .block(Block::default().borders(border.clone()).border_type(BorderType::Rounded))
        .alignment(Alignment::Center));

    paragraphs.push(Paragraph::new(Span::styled(format!("sunset: {}", day.sunset), Style::default().fg(Color::DarkGray)))
        .block(Block::default().borders(border.clone()).border_type(BorderType::Rounded))
        .alignment(Alignment::Center));

    paragraphs.push(Paragraph::new(Span::styled(format!("precip: {}in", day.precipitation), Style::default().fg(Color::Blue)))
        .block(Block::default().borders(border.clone()).border_type(BorderType::Rounded))
        .alignment(Alignment::Center));

    paragraphs.push(Paragraph::new(Span::styled(format!("winds: {}mph", day.max_windspeed), Style::default().fg(Color::White)))
        .block(Block::default().borders(border.clone().union(Borders::BOTTOM)).border_type(BorderType::Rounded))
        .alignment(Alignment::Center));

    return paragraphs;
}

// returns a paragraph stating the controls
pub fn get_controls<'a>() -> Paragraph<'a>
{
    return Paragraph::new
        (
            Spans::from
            (vec![
                Span::styled("Q", Style::default().add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED)),
                Span::raw(": quit | "),
                Span::styled("M", Style::default().add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED)),
                Span::raw(": menu ")
            ])
        )
        .block
        (
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .style
        (
            Style::default()
                .fg(Color::White)
        )
        .alignment(Alignment::Center);
}
