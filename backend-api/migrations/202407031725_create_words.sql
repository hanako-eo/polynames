CREATE SEQUENCE IF NOT EXISTS word_id_sequence START 1;
CREATE TABLE IF NOT EXISTS words (
    id INT PRIMARY KEY DEFAULT nextval('word_id_sequence'),
    word STRING UNIQUE CHECK (NOT contains(word, ' ')),
);

COPY words(word) FROM 'migrations/words.csv';
