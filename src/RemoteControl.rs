pub struct RemoteControl {
    pub current_channel: usize,
    pub channels: Vec<String>,
}

impl RemoteControl {
    pub fn up(&mut self) {
        if self.current_channel < 30 {
            self.current_channel = self.current_channel + 1;
        }
        else {
            self.current_channel = 0;
        }
    }

    pub fn down(&mut self) {
        if self.current_channel > 0 {
            self.current_channel = self.current_channel - 1;
        }
        else {
            self.current_channel = 30;
        }
    }

    pub fn get_channel_name(&self) -> &String {
        &self.channels[self.current_channel]
    }

    pub fn save_channel_name(&mut self, name: String) {
        self.channels.insert(0, name);
    }
}