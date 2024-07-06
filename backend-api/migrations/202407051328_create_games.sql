CREATE SEQUENCE IF NOT EXISTS card_id_sequence START 1;
CREATE TABLE IF NOT EXISTS cards (
    id INT PRIMARY KEY DEFAULT nextval('card_id_sequence'),
    color COLOR,
    id_word INT,
    looked BOOL,

    FOREIGN KEY (id_word) REFERENCES words (id),
);

CREATE TABLE IF NOT EXISTS games (
    id UUID PRIMARY KEY DEFAULT uuid(),
    players PLAYER[],
    score INT DEFAULT 0,
);