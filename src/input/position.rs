use {crate::Position, dialoguer::Input};
impl Position {
    pub fn input(&mut self) {
        let x: String = Input::new()
            .with_prompt("x")
            .default("0".into())
            .show_default(false)
            .interact_text()
            .unwrap();
        let y: String = Input::new()
            .with_prompt("y")
            .default("0".into())
            .show_default(false)
            .interact_text()
            .unwrap();
        let x: u64 = x.parse().unwrap();
        let y: u64 = y.parse().unwrap();

        self.x = x;
        self.y = y;
    }
}
