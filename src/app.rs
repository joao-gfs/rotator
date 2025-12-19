use rot_manipulator::Ciphertext;

pub enum CurrentScreen {
    Main,
    Exiting
}

pub enum CurrentSection {
    Input,
    Result
}

pub struct App {
    pub ciphertext: Ciphertext,
    pub current_screen: CurrentScreen,
    pub current_section: CurrentSection,
}

impl App {
    pub fn new() -> App {
        App {
            ciphertext: Ciphertext::new(String::new()),
            current_screen: CurrentScreen::Main,
            current_section: CurrentSection::Input,
        }
    }

    pub fn toggle_section(&mut self) {
        match self.current_section {
            CurrentSection::Input => self.current_section = CurrentSection::Result,
            CurrentSection::Result => self.current_section = CurrentSection::Input
        }
    }

    pub fn print_cypher(&self) -> std::io::Result<()> {
        println!("ROT: {}", self.ciphertext.rot);
        println!("OUTPUT:\n{}", self.ciphertext.current_text);
        Ok(())
    }
}