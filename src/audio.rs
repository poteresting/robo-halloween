use std::{collections::HashMap, io::Cursor};

use rodio::{source::Buffered, Decoder, OutputStream, Sink, Source};

pub struct Audio {
    audios: HashMap<String, Buffered<Decoder<Cursor<Vec<u8>>>>>,
    sinks: HashMap<String, Sink>,
    handle: rodio::OutputStreamHandle,
    _stream: rodio::OutputStream,
}

impl Audio {
    pub fn new() -> Self {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        Self {
            audios: HashMap::new(),
            sinks: HashMap::new(),
            handle,
            _stream,
        }
    }

    fn add(&mut self, identifer: &str, file: &[u8]) {
        let sink = Sink::try_new(&self.handle).unwrap();
        self.sinks.insert(identifer.to_owned(), sink);

        let decoder = rodio::Decoder::new(Cursor::new(file.to_vec())).unwrap();
        let source = decoder.buffered();
        self.audios.insert(identifer.to_owned(), source);
    }

    pub fn play(&self, identifer: &str) {
        if self.audios.contains_key(identifer) && self.sinks.contains_key(identifer) {
            if let Some(source) = self.audios.get(identifer) {
                if let Some(sink) = self.sinks.get(identifer) {
                    sink.append(source.clone());
                }
            }
        }
    }

    pub fn get_sink(&self, identifer: &str) -> Option<&Sink> {
        self.sinks.get(identifer)
    }

    fn set_music_volume(&self, identifer: &str, value: f32) {
        if let Some(sink) = self.get_sink(identifer) {
            sink.set_volume(value);
        }
    }

    pub fn upload_audios(&mut self) {
        let game_music = include_bytes!("../assets/sounds/game_music.mp3");
        self.add("game_music", game_music);

        let shoot = include_bytes!("../assets/sounds/shoot.mp3");
        self.add("shoot", shoot);

        let hit_pumpkin = include_bytes!("../assets/sounds/hit_pumpkin.mp3");
        self.add("hit_pumpkin", hit_pumpkin);

        let hit_zombie = include_bytes!("../assets/sounds/hit_zombie.mp3");
        self.add("hit_zombie", hit_zombie);
        self.set_music_volume("hit_zombie", 0.4);

        let player_dead = include_bytes!("../assets/sounds/player_dead.mp3");
        self.add("player_dead", player_dead);
        self.set_music_volume("player_dead", 0.2);

        let finished_game = include_bytes!("../assets/sounds/finished_game.mp3");
        self.add("finished_game", finished_game);
        self.set_music_volume("finished_game", 0.5);
    }
}
