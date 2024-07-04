CREATE SEQUENCE IF NOT EXISTS word_id_sequence START 1;
CREATE TABLE IF NOT EXISTS words (
    id INTEGER PRIMARY KEY DEFAULT nextval('word_id_sequence'),
    word TEXT UNIQUE CHECK (NOT contains(word, ' '))
);

COPY words(word) FROM 'migrations/words.csv';
