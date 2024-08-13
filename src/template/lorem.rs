use fake::Fake;
use clap::ArgMatches;
use fake::faker::lorem::raw::{Paragraphs, Sentences, Words};
use fake::locales::EN;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Lorem {
    pub words: Vec<String>,
    pub sentences: Vec<String>,
    pub paragraphs: Vec<String>,
}

impl Lorem {
    pub fn create(
        _arg: &ArgMatches,
        lorems_words_nb: u32,
        lorems_sentences_nb: u32,
        lorems_paragraphs_nb: u32,
    ) -> Lorem {
        let mut words = vec![];
        if lorems_words_nb > 0 {
            words = Words(EN, lorems_words_nb as usize..lorems_words_nb as usize + 1).fake();
        }
        let mut sentences = vec![];
        if lorems_sentences_nb > 0 {
            sentences = Sentences(
                EN,
                lorems_sentences_nb as usize..lorems_sentences_nb as usize + 1,
            )
            .fake();
        }
        let mut paragraphs = vec![];
        if lorems_paragraphs_nb > 0 {
            paragraphs = Paragraphs(
                EN,
                lorems_paragraphs_nb as usize..lorems_paragraphs_nb as usize + 1,
            )
            .fake();
        }
        Lorem {
            words,
            sentences,
            paragraphs,
        }
    }
}
