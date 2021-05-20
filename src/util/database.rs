use sqlite::{Connection, Value, open};


const DB: &str = "kaati_ako.sqlite";


#[derive(Debug)]
pub struct Language {
    pub code: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Translation {
    pub language: Language,
    pub text: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Card {
    pub id: i64,
    pub category: String,
    pub translations: Vec<Translation>,
}


pub fn connect_database() -> Connection {
    open(DB).unwrap()
}

pub fn create_database(conn: &Connection) {
    conn
        .execute("
            DROP TABLE IF EXISTS category;
            CREATE TABLE category (
                id INTEGER NOT NULL PRIMARY KEY,
                name TEXT
            );
            INSERT INTO category (name) VALUES ('default');

            DROP TABLE IF EXISTS card;
            CREATE TABLE card (
                id INTEGER NOT NULL PRIMARY KEY,
                category_id INTEGER,
                FOREIGN KEY (category_id) REFERENCES category (id)
            );
            INSERT INTO card (category_id) VALUES (1);
            INSERT INTO card (category_id) VALUES (1);
            INSERT INTO card (category_id) VALUES (1);

            DROP TABLE IF EXISTS language;
            CREATE TABLE language (
                id INTEGER NOT NULL PRIMARY KEY,
                code TEXT,
                name TEXT
            );
            INSERT INTO language (code, name) VALUES ('to', 'Tongan');
            INSERT INTO language (code, name) VALUES ('en', 'English');
            INSERT INTO language (code, name) VALUES ('de', 'German');

            DROP TABLE IF EXISTS translation;
            CREATE TABLE translation (
                id INTEGER NOT NULL PRIMARY KEY,
                card_id INTEGER,
                language_id INTEGER,
                text TEXT,
                description TEXT,
                FOREIGN KEY (card_id) REFERENCES card (id),
                FOREIGN KEY (language_id) REFERENCES language (id)
            );
            INSERT INTO translation (card_id, language_id, text, description) VALUES (1, 1, 'kaati', '');
            INSERT INTO translation (card_id, language_id, text, description) VALUES (1, 2, 'card', 'A card as in flash card or birthday card');
            INSERT INTO translation (card_id, language_id, text, description) VALUES (1, 3, 'Karte', 'Eine Karte wie in Karteikarte oder Geburtstagskarte');
            INSERT INTO translation (card_id, language_id, text, description) VALUES (2, 1, 'ako', '');
            INSERT INTO translation (card_id, language_id, text, description) VALUES (2, 2, 'learn', 'Learn a language');
            INSERT INTO translation (card_id, language_id, text, description) VALUES (2, 3, 'lernen', 'Eine Sprache lernen');
            INSERT INTO translation (card_id, language_id, text, description) VALUES (3, 1, 'lea faka', '');
            INSERT INTO translation (card_id, language_id, text, description) VALUES (3, 2, 'language', 'Learn a language');
            INSERT INTO translation (card_id, language_id, text, description) VALUES (3, 3, 'Sprache', 'Eine Sprache lernen');
        ")
        .unwrap();
    //let cards = get_cards(&conn);
    //println!("cards: {:?}", cards);
}


fn get_translations(conn: &Connection, card_id: i64) -> Vec<Translation> {
    let mut cursor = conn
        .prepare("
            SELECT language.code, language.name, text, description
            FROM translation
            LEFT JOIN language ON translation.language_id = language.id
            WHERE card_id = ?
        ")
        .unwrap()
        .cursor();
    cursor.bind(&[Value::Integer(card_id)]).unwrap();
    let mut translations = Vec::new();
    while let Some(row) = cursor.next().unwrap() {
        let language = Language {
            code: row[0].as_string().unwrap().to_string(),
            name: row[1].as_string().unwrap().to_string(),
        };
        translations.push(Translation {
            language,
            text: row[2].as_string().unwrap().to_string(),
            description: row[3].as_string().unwrap().to_string(),
        })
    }
    translations
}


pub fn get_random_card(conn: &Connection) -> Card {
    let mut cursor = conn
        .prepare("
            SELECT card.id, category.name
            FROM card
            LEFT JOIN category ON card.category_id = category.id
            ORDER BY RANDOM()
            LIMIT 1
        ")
        .unwrap()
        .cursor();
    while let Some(row) = cursor.next().unwrap() {
        let card_id = row[0].as_integer().unwrap();
        return Card {
            id: card_id,
            category: row[1].as_string().unwrap().to_string(),
            translations: get_translations(conn, card_id),
        };
    }
    Card { id: 0, category: "".to_string(), translations: Vec::new()}
}


#[allow(dead_code)]
pub fn get_cards(conn: &Connection) -> Vec<Card> {
    let mut cursor = conn
        .prepare("
            SELECT card.id, category.name
            FROM card
            LEFT JOIN category ON card.category_id = category.id
            ORDER BY card.id
        ")
        .unwrap()
        .cursor();
    let mut cards = Vec::new();
    while let Some(row) = cursor.next().unwrap() {
        let card_id = row[0].as_integer().unwrap();
        cards.push(Card {
            id: card_id,
            category: row[1].as_string().unwrap().to_string(),
            translations: get_translations(conn, card_id),
        })
    }
    cards
}
